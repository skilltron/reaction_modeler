//! Read an MDNG file (from the FASTQ → pipeline → .mdng path) and emit VariantInput JSON.
//! Lets you compare your own extraction (shards/FASTQ pipeline) vs VCF in the genetic conditions report.
//!
//! Usage: mdng-to-variants <path.mdng>
//!   Reads MDNG JSON (must have "variants" array with chromosome, position, reference_allele, alternate_alleles, variant_id).
//!   Writes VariantInput JSON array to stdout (one record per ALT if alternate_alleles is comma-separated).

use genetic_conditions::{ClinvarSummary, VariantInput};
use serde_json::Value;
use std::env;
use std::fs;
use std::io;
use std::process;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: mdng-to-variants <path.mdng>");
        eprintln!("  Reads MDNG from pipeline (FASTQ → shards → .mdng), writes VariantInput JSON to stdout.");
        process::exit(1);
    }
    let path = &args[1];
    let raw = fs::read_to_string(path)?;
    let data: Value = serde_json::from_str(&raw)?;
    let variants_array = data
        .get("variants")
        .and_then(|v| v.as_array())
        .ok_or("MDNG must have a 'variants' array")?;

    let mut out = Vec::with_capacity(variants_array.len().min(256 * 1024));
    for v in variants_array {
        let chrom = v
            .get("chromosome")
            .and_then(|c| c.as_str())
            .unwrap_or("")
            .to_string();
        let pos = v.get("position").and_then(|p| p.as_u64()).unwrap_or(0);
        let ref_allele = v
            .get("reference_allele")
            .and_then(|r| r.as_str())
            .map(String::from);
        let alts_str = v
            .get("alternate_alleles")
            .and_then(|a| a.as_str())
            .unwrap_or("");
        let rsid = v
            .get("variant_id")
            .and_then(|id| id.as_str())
            .filter(|s| !s.is_empty() && *s != ".")
            .map(String::from);
        let gene = v
            .get("metadata")
            .and_then(|m| m.get("gene"))
            .and_then(|g| g.as_str())
            .filter(|s| !s.is_empty())
            .map(String::from);
        let genotype = v.get("genotype").and_then(|g| g.as_str()).map(String::from);
        let clinvar: Option<ClinvarSummary> = v
            .get("metadata")
            .and_then(|m| m.get("clinvar"))
            .and_then(|c| serde_json::from_value(c.clone()).ok());

        for alt in alts_str.split(',') {
            let alt = alt.trim();
            if alt.is_empty() || alt == "." {
                continue;
            }
            out.push(VariantInput {
                chromosome: chrom.clone(),
                position: pos,
                gene: gene.clone(),
                rsid: rsid.clone(),
                ref_allele: ref_allele.clone(),
                alt_allele: Some(alt.to_string()),
                region_type: None,
                genotype: genotype.clone(),
                clinvar: clinvar.clone(),
                confidence: None,
            });
        }
    }

    serde_json::to_writer(io::stdout(), &out)?;
    Ok(())
}
