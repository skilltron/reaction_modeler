//! Pharmacogene star allele inference from variant list (CPIC/PharmVar defining variants).
//! For cross-checking: compare with PharmCAT, StellarPGx, or other official star allele finders when available.

use crate::VariantInput;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// One defining variant for a star allele.
struct DefiningVariant {
    rsid: &'static str,
    ref_allele: &'static str,
    alt_allele: &'static str,
}

/// Gene, allele name, defining variant, effect label.
struct AlleleDef {
    gene: &'static str,
    allele: &'static str,
    variant: DefiningVariant,
    effect: &'static str,
}

fn allele_definitions() -> Vec<AlleleDef> {
    vec![
        AlleleDef {
            gene: "CYP2C19",
            allele: "*2",
            variant: DefiningVariant { rsid: "rs4244285", ref_allele: "G", alt_allele: "A" },
            effect: "loss of function",
        },
        AlleleDef {
            gene: "CYP2C19",
            allele: "*3",
            variant: DefiningVariant { rsid: "rs4986893", ref_allele: "G", alt_allele: "A" },
            effect: "loss of function",
        },
        AlleleDef {
            gene: "CYP2C19",
            allele: "*17",
            variant: DefiningVariant { rsid: "rs12248560", ref_allele: "C", alt_allele: "T" },
            effect: "increased function",
        },
        AlleleDef {
            gene: "CYP2D6",
            allele: "*3",
            variant: DefiningVariant { rsid: "rs35742686", ref_allele: "A", alt_allele: "del" },
            effect: "loss of function",
        },
        AlleleDef {
            gene: "CYP2D6",
            allele: "*4",
            variant: DefiningVariant { rsid: "rs3892097", ref_allele: "G", alt_allele: "A" },
            effect: "loss of function",
        },
        AlleleDef {
            gene: "CYP2D6",
            allele: "*10",
            variant: DefiningVariant { rsid: "rs1065852", ref_allele: "G", alt_allele: "A" },
            effect: "reduced function",
        },
        AlleleDef {
            gene: "CYP2D6",
            allele: "*41",
            variant: DefiningVariant { rsid: "rs28371725", ref_allele: "G", alt_allele: "A" },
            effect: "reduced function",
        },
        AlleleDef {
            gene: "CYP2C9",
            allele: "*2",
            variant: DefiningVariant { rsid: "rs1799853", ref_allele: "C", alt_allele: "T" },
            effect: "reduced function",
        },
        AlleleDef {
            gene: "CYP2C9",
            allele: "*3",
            variant: DefiningVariant { rsid: "rs1057910", ref_allele: "A", alt_allele: "C" },
            effect: "reduced function",
        },
        AlleleDef {
            gene: "CYP3A4",
            allele: "*22",
            variant: DefiningVariant { rsid: "rs35599367", ref_allele: "C", alt_allele: "T" },
            effect: "reduced function",
        },
    ]
}

fn normalize_rsid(rsid: &str) -> String {
    let s = rsid.trim();
    if s.is_empty() {
        return String::new();
    }
    let lower = s.to_lowercase();
    if lower.starts_with("rs") && s.len() > 2 && s[2..].chars().all(|c| c.is_ascii_digit()) {
        lower
    } else {
        s.to_string()
    }
}

fn variant_matches(ref_a: &str, alt_a: &str, def_ref: &str, def_alt: &str) -> bool {
    let r = ref_a.trim().to_uppercase();
    let a = alt_a.trim().to_uppercase();
    let dr = def_ref.trim().to_uppercase();
    let da = def_alt.trim().to_uppercase();
    if dr != "?" && r != dr {
        return false;
    }
    if da == "?" {
        return true;
    }
    if da == "DEL" || da.is_empty() {
        return a.is_empty() || a == "." || a == "DEL" || a.len() < r.len();
    }
    a == da
}

/// True if genotype string indicates alternate allele present (0/1, 1/0, 1/1, etc).
fn genotype_has_alt(genotype: Option<&str>) -> bool {
    let gt = match genotype {
        Some(g) if !g.is_empty() => g.trim(),
        _ => return false,
    };
    let parts: Vec<&str> = gt.split(|c| c == '/' || c == '|').map(|s| s.trim()).collect();
    if parts.len() < 2 {
        return false;
    }
    parts.iter().any(|p| *p == "1")
}

/// Build rsid -> (ref_allele, alt_allele, genotype) from variant list. When genotype is missing, variant in list is treated as present (0/1).
fn build_rsid_lookup(variants: &[VariantInput]) -> HashMap<String, (String, String, Option<String>)> {
    let mut out = HashMap::new();
    for v in variants {
        let rsid = match &v.rsid {
            Some(r) if !r.is_empty() && r != "." => r.clone(),
            _ => continue,
        };
        let ref_a = v.ref_allele.clone().unwrap_or_default();
        let alt_a = v.alt_allele.clone().unwrap_or_default();
        let gt = v.genotype.clone();
        let key_norm = normalize_rsid(&rsid);
        if key_norm.is_empty() {
            continue;
        }
        if !out.contains_key(&key_norm) {
            out.insert(key_norm.clone(), (ref_a.clone(), alt_a.clone(), gt.clone()));
        }
        out.insert(rsid, (ref_a, alt_a, gt));
    }
    out
}

/// One row for the star allele legend (gene, allele, defining rsID, effect).
#[derive(Debug, Clone)]
pub struct StarAlleleLegendEntry {
    pub gene: String,
    pub allele: String,
    pub rsid: String,
    pub effect: String,
}

/// Return legend entries for all alleles used in inference (for display in report).
pub fn star_allele_legend() -> Vec<StarAlleleLegendEntry> {
    allele_definitions()
        .into_iter()
        .map(|d| StarAlleleLegendEntry {
            gene: d.gene.to_string(),
            allele: d.allele.to_string(),
            rsid: d.variant.rsid.to_string(),
            effect: d.effect.to_string(),
        })
        .collect()
}

/// Result for one gene: detected alleles (sorted) and diplotype string.
#[derive(Debug, Clone)]
pub struct StarAlleleGeneResult {
    pub gene: String,
    pub alleles: Vec<String>,
    pub diplotype: String,
    pub effect_labels: Vec<String>,
}

/// One gene's call from an official star allele finder (e.g. PharmCAT, StellarPGx).
#[derive(Debug, Clone, Deserialize)]
pub struct OfficialStarAlleleCall {
    pub diplotype: String,
    #[serde(default)]
    pub source: String,
}

/// Load official star allele results from JSON. Format: { "CYP2C19": { "diplotype": "*1/*2", "source": "PharmCAT" }, ... }.
pub fn load_official_star_alleles(path: &Path) -> Result<HashMap<String, OfficialStarAlleleCall>, Box<dyn std::error::Error + Send + Sync>> {
    let raw = std::fs::read_to_string(path)?;
    let map: HashMap<String, OfficialStarAlleleCall> = serde_json::from_str(&raw)?;
    Ok(map)
}

/// Optional load: returns None if file missing or invalid.
pub fn load_official_star_alleles_optional(path: &str) -> Option<HashMap<String, OfficialStarAlleleCall>> {
    let p = Path::new(path);
    if !p.is_file() {
        return None;
    }
    load_official_star_alleles(p).ok()
}

/// One row for verification table: our diplotype vs official, and whether they match.
#[derive(Debug, Clone)]
pub struct StarAlleleVerificationRow {
    pub gene: String,
    pub our_diplotype: String,
    pub official_diplotype: String,
    pub official_source: String,
    pub matches: bool,
}

/// Build verification rows for report when official results are available.
pub fn build_star_allele_verification(
    our_results: &[StarAlleleGeneResult],
    official: &HashMap<String, OfficialStarAlleleCall>,
) -> Vec<StarAlleleVerificationRow> {
    our_results
        .iter()
        .filter_map(|s| {
            let official_call = official.get(&s.gene)?;
            let our_dip = s.diplotype.trim();
            let off_dip = official_call.diplotype.trim();
            let matches = normalize_diplotype_for_compare(our_dip) == normalize_diplotype_for_compare(off_dip);
            Some(StarAlleleVerificationRow {
                gene: s.gene.clone(),
                our_diplotype: our_dip.to_string(),
                official_diplotype: off_dip.to_string(),
                official_source: if official_call.source.is_empty() { "Official finder".to_string() } else { official_call.source.clone() },
                matches,
            })
        })
        .collect()
}

fn normalize_diplotype_for_compare(d: &str) -> String {
    let d = d.trim();
    if d.is_empty() {
        return String::new();
    }
    let without_ref = d.replace(" (reference)", "").trim().to_string();
    let parts: Vec<&str> = without_ref.split('/').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
    if parts.len() >= 2 {
        let mut p = parts.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        p.sort();
        p.join("/")
    } else {
        without_ref
    }
}

/// Infer star alleles from variant list. Returns one result per gene (CYP2C19, CYP2D6, CYP2C9, CYP3A4).
/// Cross-check with StellarPGx or pipeline star allele output when available.
pub fn infer_star_alleles(variants: &[VariantInput]) -> Vec<StarAlleleGeneResult> {
    let defs = allele_definitions();
    let lookup = build_rsid_lookup(variants);
    let mut by_gene: HashMap<String, (HashSet<String>, Vec<String>)> = HashMap::new();

    for d in &defs {
        let key_norm = normalize_rsid(d.variant.rsid);
        if key_norm.is_empty() {
            continue;
        }
        let (ref_a, alt_a, genotype) = match lookup.get(d.variant.rsid).or_else(|| lookup.get(&key_norm)) {
            Some((r, a, g)) if !r.is_empty() => (r.as_str(), a.as_str(), g.as_deref()),
            _ => continue,
        };
        if !variant_matches(ref_a, alt_a, d.variant.ref_allele, d.variant.alt_allele) {
            continue;
        }
        // If genotype is present use it; otherwise variant in list implies at least one copy (0/1).
        let has_alt = genotype_has_alt(genotype) || (genotype.is_none() && !alt_a.is_empty() && alt_a != ".");
        if has_alt && d.allele.starts_with('*') {
            let entry = by_gene.entry(d.gene.to_string()).or_default();
            entry.0.insert(d.allele.to_string());
            if !entry.1.contains(&d.effect.to_string()) {
                entry.1.push(d.effect.to_string());
            }
        }
    }

    let mut genes_ordered: Vec<String> = Vec::new();
    for g in ["CYP2C19", "CYP2D6", "CYP2C9", "CYP3A4"] {
        if defs.iter().any(|d| d.gene == g) {
            genes_ordered.push(g.to_string());
        }
    }

    let mut result = Vec::new();
    for gene in genes_ordered {
        let (alleles_set, effects) = by_gene.remove(&gene).unwrap_or_default();
        let mut alleles: Vec<String> = alleles_set.into_iter().collect();
        alleles.sort();
        if alleles.len() > 2 {
            alleles.truncate(2);
        }
        let diplotype = if alleles.is_empty() {
            "*1/*1 (reference)".to_string()
        } else if alleles.len() == 1 {
            format!("*1/{}", alleles[0])
        } else {
            format!("{}/{}", alleles[0], alleles[1])
        };
        result.push(StarAlleleGeneResult {
            gene,
            alleles,
            diplotype,
            effect_labels: effects,
        });
    }
    result
}
