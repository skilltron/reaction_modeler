//! Convert a VCF (plain or .gz) to VariantInput JSON for genetic-report-html.
//! Standard conversion path for verification vs MDNG. One VariantInput per ALT (multi-ALT rows expanded).
//! Genotype (GT) is read from the first sample column when FORMAT contains GT.
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
const VCF_FORMAT: usize = 8;
/// First sample column in a standard VCF (after FORMAT).
const VCF_FIRST_SAMPLE: usize = 9;

/// Find the index of "GT" in FORMAT (colon-separated). Returns None if not found.
fn gt_index(format: &str) -> Option<usize> {
    format.split(':').position(|s| s.trim().eq_ignore_ascii_case("GT"))
}

/// Extract GT from a sample value using FORMAT. Sample and format are colon-separated; we take the same index as GT in format.
fn extract_gt(format: &str, sample: &str) -> Option<String> {
    let idx = gt_index(format)?;
    let gt = sample.split(':').nth(idx)?.trim();
    if gt.is_empty() || gt == "." || gt == "./." || gt == ".|." {
        return None;
    }
    Some(gt.to_string())
}

fn read_vcf<R: Read>(reader: R) -> io::Result<Vec<VariantInput>> {
    let mut out = Vec::with_capacity(256 * 1024);
    let buf = BufReader::new(reader);
    let mut lines = buf.lines();
    let mut sample_col = None::<usize>;
    let mut format_col = None::<usize>;

    // Find #CHROM line to get column layout (FORMAT = 8, first sample = 9 in standard VCF)
    while let Some(line) = lines.next() {
        let line = line?;
        if line.starts_with("##") {
            continue;
        }
        if line.starts_with('#') {
            let cols: Vec<&str> = line.split('\t').collect();
            if cols.len() > VCF_FIRST_SAMPLE {
                sample_col = Some(VCF_FIRST_SAMPLE);
                format_col = Some(VCF_FORMAT);
            }
            break;
        }
        // No header? We'll have no genotype
        break;
    }

    for line in lines {
        let line = line?;
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

        let genotype = sample_col.and_then(|sc| format_col.and_then(|fc| {
            if cols.len() > sc && cols.len() > fc {
                extract_gt(cols[fc], cols[sc])
            } else {
                None
            }
        }));

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
                genotype: genotype.clone(),
                clinvar: None,
                confidence: None,
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
