//! Independent MCAS verification: read variant JSON and compute KIT D816V + pathogenic KIT/TPSAB1 count.
//! Does NOT use the report or inflammation module — reimplements the same rules so we can compare.
//! Usage: verify-mcas <variants.json>
//! Output: KIT_D816V=0|1 and PATHOGENIC_KIT_TPSAB1_COUNT=N (one line each) for comparison with report.

use genetic_conditions::VariantInput;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::process;

const KIT_D816V_RSID: &str = "rs121913529";
const KIT_D816V_POSITION_GRCH38: u64 = 55_599_352;
const KIT_D816V_POSITION_GRCH37: u64 = 54_733_155;

fn is_kit_d816v(v: &VariantInput) -> bool {
    if v.rsid.as_deref().map(|r| r == KIT_D816V_RSID).unwrap_or(false) {
        return true;
    }
    let chr4 = v.chromosome.eq_ignore_ascii_case("4");
    let pos_ok = v.position == KIT_D816V_POSITION_GRCH38 || v.position == KIT_D816V_POSITION_GRCH37;
    let at = v.ref_allele.as_deref().map(|r| r == "A").unwrap_or(false)
        && v.alt_allele.as_deref().map(|a| a == "T").unwrap_or(false);
    chr4 && pos_ok && at
}

fn is_pathogenic(c: &str) -> bool {
    let c = c.trim().to_lowercase();
    if c.is_empty() {
        return false;
    }
    for part in c.split('/').map(|s| s.trim()) {
        if part == "pathogenic" || part == "likely pathogenic" {
            return true;
        }
    }
    false
}

fn gene_is_kit_or_tpsab1(v: &VariantInput) -> bool {
    match v.gene.as_deref() {
        Some(g) => g.eq_ignore_ascii_case("KIT") || g.eq_ignore_ascii_case("TPSAB1"),
        None => false,
    }
}

fn dedup_key(v: &VariantInput) -> String {
    format!(
        "{}:{}:{}:{}",
        v.chromosome,
        v.position,
        v.ref_allele.as_deref().unwrap_or(""),
        v.alt_allele.as_deref().unwrap_or("")
    )
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<String> = env::args().collect();
    let path = match args.get(1) {
        Some(p) => p.as_str(),
        None => {
            eprintln!("Usage: verify-mcas <variants.json>");
            process::exit(1);
        }
    };
    let raw = fs::read_to_string(path)?;
    let variants: Vec<VariantInput> = serde_json::from_str(&raw)?;
    eprintln!("[verify-mcas] Read {} variants from {}", variants.len(), path);

    let kit_d816v = variants.iter().any(is_kit_d816v);
    let mut seen = HashSet::new();
    let mut count = 0usize;
    for v in &variants {
        if !gene_is_kit_or_tpsab1(v) {
            continue;
        }
        let pathogenic = v
            .clinvar
            .as_ref()
            .map(|c| is_pathogenic(&c.classification))
            .unwrap_or(false);
        let is_kit = is_kit_d816v(v);
        if !pathogenic && !is_kit {
            continue;
        }
        let key = dedup_key(v);
        if seen.contains(&key) {
            continue;
        }
        seen.insert(key);
        count += 1;
    }

    println!("KIT_D816V={}", if kit_d816v { 1 } else { 0 });
    println!("PATHOGENIC_KIT_TPSAB1_COUNT={}", count);
    Ok(())
}
