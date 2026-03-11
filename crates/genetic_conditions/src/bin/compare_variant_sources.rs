//! Compare two VariantInput JSON files (e.g. MDNG conversion vs standard conversion).
//! Reports total counts, unique variant counts (by chr+pos+ref+alt), overlap, and which has more data.
//!
//! Usage: compare-variant-sources <path_a.json> <path_b.json>
//!   path_a typically = MDNG-derived variants, path_b = standard (e.g. VCF-derived) variants.

use genetic_conditions::VariantInput;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::process;

fn key(v: &VariantInput) -> (String, u64, String, String) {
    (
        v.chromosome.clone(),
        v.position,
        v.ref_allele.clone().unwrap_or_default(),
        v.alt_allele.clone().unwrap_or_default(),
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: compare-variant-sources <variants_a.json> <variants_b.json>");
        eprintln!("  Typically: A = MDNG conversion, B = standard (VCF) conversion.");
        process::exit(1);
    }
    let path_a = &args[1];
    let path_b = &args[2];

    let raw_a = fs::read_to_string(path_a)?;
    let raw_b = fs::read_to_string(path_b)?;
    let variants_a: Vec<VariantInput> = serde_json::from_str(&raw_a)?;
    let variants_b: Vec<VariantInput> = serde_json::from_str(&raw_b)?;

    let set_a: HashSet<_> = variants_a.iter().map(key).collect();
    let set_b: HashSet<_> = variants_b.iter().map(key).collect();
    let overlap = set_a.intersection(&set_b).count();
    let only_a = set_a.difference(&set_b).count();
    let only_b = set_b.difference(&set_a).count();

    println!("Source A: {}  (unique keys: {})", path_a, set_a.len());
    println!("  Total records: {}", variants_a.len());
    println!("Source B: {}  (unique keys: {})", path_b, set_b.len());
    println!("  Total records: {}", variants_b.len());
    println!();
    println!("Overlap (in both):     {}", overlap);
    println!("Only in A:             {}", only_a);
    println!("Only in B:             {}", only_b);
    println!();
    if set_a.len() > set_b.len() {
        println!("Result: A has MORE unique variants than B (by {}).", set_a.len() - set_b.len());
    } else if set_b.len() > set_a.len() {
        println!("Result: B has MORE unique variants than A (by {}).", set_b.len() - set_a.len());
    } else {
        println!("Result: A and B have the SAME number of unique variants.");
    }

    Ok(())
}
