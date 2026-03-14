//! Load multiple VCF or MDNG files, merge by variant key, write one MDNG with all data preserved.
//!
//! Usage: combine-to-mdng <output.mdng> <input1> [input2 ...]
//!   Each input: .mdng (MDNG JSON) or .vcf / .vcf.gz (VCF). Duplicates merged by (chr, pos, ref, alt); first occurrence kept.
//!   All processing is local; no data sent to the network.

use shard_db::ShardDb;
use std::env;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 2 {
        eprintln!("Usage: combine-to-mdng <output.mdng> <input1> [input2 ...]");
        eprintln!("  Each input: .mdng or .vcf / .vcf.gz");
        eprintln!("  Output: single MDNG with all variants merged (duplicates by chr/pos/ref/alt kept once).");
        std::process::exit(1);
    }

    let output_path = &args[0];
    let inputs = &args[1..];

    let temp_root = env::temp_dir().join(format!("combine_mdng_{}", std::process::id()));
    std::fs::create_dir_all(&temp_root)?;

    let mut db = ShardDb::open(&temp_root);

    for path in inputs {
        let path = Path::new(path);
        if !path.exists() {
            eprintln!("Skip (not found): {}", path.display());
            continue;
        }
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        if name.ends_with(".mdng") {
            eprintln!("Loading MDNG: {}", path.display());
            match db.ingest_mdng(path) {
                Ok(n) => eprintln!("  -> {} variants added", n),
                Err(e) => eprintln!("  Error: {}", e),
            }
            if let Ok(total) = db.total_count() {
                eprintln!("  Total in DB: {}", total);
            }
            continue;
        }

        let is_gz = name.ends_with(".gz");
        let vcf_path: std::path::PathBuf = if is_gz {
            eprintln!("Decompressing VCF (BGZF-safe): {}", path.display());
            let dec = Command::new("gzip").args(["-dc", path.to_str().unwrap()]).output();
            match dec {
                Ok(out) if out.status.success() => {
                    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("in");
                    let temp_vcf = temp_root.join(format!("{}.vcf", stem));
                    std::fs::write(&temp_vcf, &out.stdout)?;
                    temp_vcf
                }
                _ => {
                    eprintln!("  gzip -dc failed or not available; trying direct read (may be partial for BGZF)...");
                    path.to_path_buf()
                }
            }
        } else {
            path.to_path_buf()
        };

        eprintln!("Loading VCF: {}", vcf_path.display());
        match db.ingest_vcf(&vcf_path) {
            Ok(n) => eprintln!("  -> {} variants added", n),
            Err(e) => eprintln!("  Error: {}", e),
        }
        if let Ok(total) = db.total_count() {
            eprintln!("  Total in DB: {}", total);
        }
    }

    eprintln!("Writing combined MDNG: {}", output_path);
    let written = db.write_mdng(output_path)?;
    let _ = std::fs::remove_dir_all(&temp_root);
    eprintln!("Done. {} variants written to {}", written, output_path);

    Ok(())
}
