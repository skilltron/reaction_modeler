//! Convert a VCF (plain or .gz) to VariantInput JSON for genetic-report-html.
//! Standard conversion path for verification vs MDNG. One VariantInput per ALT (multi-ALT rows expanded).
//!
//! Usage: vcf_to_variants [path.vcf | path.vcf.gz]
//!   If no path: read VCF from stdin. Writes JSON array to stdout.

use flate2::read::GzDecoder;
use genetic_conditions::VariantInput;
use std::env;
use std::io::{self, BufRead, BufReader, Read};
use std::process;

const VCF_CHROM: usize = 0;
const VCF_POS: usize = 1;
const VCF_ID: usize = 2;
const VCF_REF: usize = 3;
const VCF_ALT: usize = 4;

fn read_vcf<R: Read>(reader: R) -> io::Result<Vec<VariantInput>> {
    let mut out = Vec::with_capacity(256 * 1024);
    let buf = BufReader::new(reader);
    for line in buf.lines() {
        let line = line?;
        if line.starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() <= VCF_ALT {
            continue;
        }
        let chrom = cols[VCF_CHROM].to_string();
        let pos: u64 = match cols[VCF_POS].parse() {
            Ok(p) => p,
            Err(_) => continue,
        };
        let id = cols[VCF_ID];
        let rsid = if id == "." || id.is_empty() {
            None
        } else {
            Some(id.to_string())
        };
        let ref_allele = Some(cols[VCF_REF].to_string());
        let alts = cols[VCF_ALT];
        for alt in alts.split(',') {
            if alt == "." || alt.is_empty() {
                continue;
            }
            out.push(VariantInput {
                chromosome: chrom.clone(),
                position: pos,
                gene: None,
                rsid: rsid.clone(),
                ref_allele: ref_allele.clone(),
                alt_allele: Some(alt.to_string()),
                region_type: None,
            });
        }
    }
    Ok(out)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let reader: Box<dyn Read> = match args.len() {
        1 => Box::new(io::stdin()),
        2 => {
            let path = &args[1];
            let f = std::fs::File::open(path)?;
            if path.ends_with(".gz") {
                Box::new(GzDecoder::new(f))
            } else {
                Box::new(f)
            }
        }
        _ => {
            eprintln!("Usage: vcf_to_variants [path.vcf | path.vcf.gz]");
            eprintln!("  No arg: read VCF from stdin. Writes VariantInput JSON to stdout.");
            process::exit(1);
        }
    };

    let variants = read_vcf(reader)?;
    serde_json::to_writer(io::stdout(), &variants)?;
    Ok(())
}
