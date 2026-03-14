//! Orchestrate FASTQ → VCF (via scripts/fastq-to-vcf.sh), then optionally ingest into shard DB and/or write MDNG.
//!
//! Usage:
//!   fastq-to-variants <reference.fa> <sample_id> <out_dir> <fastq1> [fastq2]
//!     [--into-shard-db <dir>] [--write-mdng <path.mdng>]
//!
//! Runs alignment + variant calling; writes <out_dir>/<sample_id>.vcf.gz.
//! If --into-shard-db: ingests that VCF into the shard DB (decompresses BGZF via gzip -dc).
//! If --write-mdng: after ingest, writes a single MDNG file (requires --into-shard-db or uses a temp DB).

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut into_shard_db: Option<PathBuf> = None;
    let mut write_mdng: Option<PathBuf> = None;
    let mut positional = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--into-shard-db" {
            i += 1;
            if i < args.len() {
                into_shard_db = Some(PathBuf::from(&args[i]));
            }
            i += 1;
        } else if args[i] == "--write-mdng" {
            i += 1;
            if i < args.len() {
                write_mdng = Some(PathBuf::from(&args[i]));
            }
            i += 1;
        } else {
            positional.push(args[i].clone());
            i += 1;
        }
    }

    if positional.len() < 4 {
        eprintln!(
            "Usage: fastq-to-variants <reference.fa> <sample_id> <out_dir> <fastq1> [fastq2] \\
    [--into-shard-db <dir>] [--write-mdng <path.mdng>]"
        );
        eprintln!("  Runs scripts/fastq-to-vcf.sh then optionally ingests VCF into shard DB.");
        eprintln!("  --write-mdng requires --into-shard-db (ingests into that dir then writes one MDNG file).");
        std::process::exit(1);
    }

    let ref_fa = &positional[0];
    let sample_id = &positional[1];
    let out_dir = &positional[2];
    let fastq1 = &positional[3];
    let fastq2 = positional.get(4).map(String::as_str);

    // Script path: from crate dir, repo root is ../.., scripts at ../../scripts/fastq-to-vcf.sh
    let script_crate = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scripts/fastq-to-vcf.sh");
    let script = if script_crate.exists() {
        script_crate
    } else if Path::new("scripts/fastq-to-vcf.sh").exists() {
        PathBuf::from("scripts/fastq-to-vcf.sh")
    } else {
        eprintln!("Script not found: {} (or scripts/fastq-to-vcf.sh from cwd)", script_crate.display());
        std::process::exit(1);
    };

    let mut cmd = Command::new("bash");
    cmd.arg(&script)
        .arg(ref_fa)
        .arg(sample_id)
        .arg(out_dir)
        .arg(fastq1);
    if let Some(f2) = fastq2 {
        cmd.arg(f2);
    }
    eprintln!("Running: {:?}", cmd);
    let status = cmd.status()?;
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }

    let vcf_gz = Path::new(out_dir).join(format!("{}.vcf.gz", sample_id));
    if !vcf_gz.exists() {
        eprintln!("Expected VCF not found: {}", vcf_gz.display());
        std::process::exit(1);
    }
    eprintln!("VCF: {}", vcf_gz.display());

    if let Some(shard_root) = &into_shard_db {
        std::fs::create_dir_all(shard_root)?;
        // BGZF: decompress to temp file so ShardDb can read full VCF (GzDecoder may only read first block).
        let temp_vcf = env::temp_dir().join(format!("fastq_to_variants_{}.vcf", std::process::id()));
        let dec = Command::new("gzip")
            .args(["-dc", vcf_gz.to_str().unwrap()])
            .output()?;
        if !dec.status.success() {
            eprintln!("gzip -dc failed");
            std::process::exit(1);
        }
        std::fs::write(&temp_vcf, &dec.stdout)?;
        let mut db = shard_db::ShardDb::open(shard_root);
        let n = db.ingest_vcf(&temp_vcf)?;
        let _ = std::fs::remove_file(&temp_vcf);
        eprintln!("Ingested {} variants into shard DB: {}", n, shard_root.display());

        if let Some(mdng_path) = &write_mdng {
            let written = db.write_mdng(mdng_path)?;
            eprintln!("Wrote {} variants to MDNG: {}", written, mdng_path.display());
        }
    }

    Ok(())
}
