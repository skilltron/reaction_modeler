//! Convert Sequencing.com / 23andMe tab-separated export to VariantInput JSON.
//! Format: # header lines, then "rsid\tchromosome\tposition\tgenotype" (one SNP per line).
//! Used for reports when the only variant source in a folder is a text export (e.g. ULTIMATE-COMPATIBILITY or 23andMe).
//!
//! Usage: txt-to-variants <path.txt>
//!   Reads tab-separated rsid, chromosome, position, genotype. Writes JSON array to stdout.

use genetic_conditions::VariantInput;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process;

fn run(path: &str) -> io::Result<Vec<VariantInput>> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let mut out = Vec::with_capacity(512 * 1024);

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 4 {
            continue;
        }
        let rsid_str = cols[0].trim();
        let chrom = cols[1].trim().to_string();
        let pos: u64 = match cols[2].trim().parse() {
            Ok(p) => p,
            Err(_) => continue,
        };
        let genotype = cols[3].trim();
        if chrom.is_empty() || rsid_str.is_empty() {
            continue;
        }
        let rsid = if rsid_str.starts_with("rs") && rsid_str.len() > 2 {
            Some(rsid_str.to_string())
        } else {
            Some(rsid_str.to_string())
        };
        // Text export has no ref/alt; derive a convention for star alleles: genotype "AB" -> ref first, alt second when different
        let (ref_allele, alt_allele) = if genotype.len() >= 2 {
            let a = genotype.chars().next().unwrap().to_string();
            let b = genotype.chars().nth(1).unwrap().to_string();
            if a == b {
                (Some(a.clone()), Some(a))
            } else {
                (Some(a), Some(b))
            }
        } else {
            (None, None)
        };

        out.push(VariantInput {
            chromosome: chrom,
            position: pos,
            gene: None,
            rsid,
            ref_allele,
            alt_allele,
            region_type: None,
            genotype: Some(genotype.to_string()),
            clinvar: None,
            confidence: None,
        });
    }

    Ok(out)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: txt-to-variants <path.txt>");
        eprintln!("  Input: tab-separated rsid, chromosome, position, genotype (Sequencing.com / 23andMe style).");
        eprintln!("  Output: VariantInput JSON array to stdout.");
        process::exit(1);
    }

    let path = &args[1];
    match run(path) {
        Ok(variants) => {
            if variants.is_empty() {
                eprintln!("Error: no variants parsed from {}", path);
                process::exit(1);
            }
            serde_json::to_writer(io::stdout(), &variants).expect("write JSON");
            let _ = io::stdout().flush();
        }
        Err(e) => {
            eprintln!("Error reading {}: {}", path, e);
            process::exit(1);
        }
    }
}
