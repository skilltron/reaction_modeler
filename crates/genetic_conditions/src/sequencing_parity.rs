//! Parity check vs Sequencing.com report targets (HENRY_QUALITY_CROSSREF, LISA_QUALITY_CROSSREF).
//! Ensures we find at least what they found; the rest of the report adds MCAS, cascade, survival, star alleles, ClinVar, etc.

use crate::VariantInput;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParityTarget {
    pub rsid: String,
    pub gene: String,
    pub condition: String,
    pub priority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequencingParityResult {
    pub found: Vec<ParityTarget>,
    pub missing: Vec<ParityTarget>,
    pub total: usize,
    pub found_count: usize,
}

/// Normalize rsID for lookup (lowercase, strip whitespace). Used by parity check and gene annotation.
pub fn canonical_rsid(s: &str) -> String {
    let s = s.trim();
    if s.is_empty() || s == "." {
        return String::new();
    }
    let lower = s.to_lowercase();
    if lower.starts_with("rs") && s.len() > 2 && s[2..].chars().all(|c| c.is_ascii_digit()) {
        lower
    } else {
        s.to_string()
    }
}

/// Combined list of rsIDs that Sequencing.com reports (Henry + Lisa targets). Deduped by rsid. Used to verify we're at least on par.
fn sequencing_report_targets() -> Vec<ParityTarget> {
    let mut list = vec![
        // Henry (HENRY_QUALITY_CROSSREF)
        ParityTarget { rsid: "rs6025".to_string(), gene: "F5".to_string(), condition: "Factor V Leiden".to_string(), priority: "CRITICAL".to_string() },
        ParityTarget { rsid: "rs1799945".to_string(), gene: "HFE".to_string(), condition: "Hemochromatosis H63D".to_string(), priority: "HIGH".to_string() },
        ParityTarget { rsid: "rs4124874".to_string(), gene: "UGT1A".to_string(), condition: "Gilbert Syndrome".to_string(), priority: "HIGH".to_string() },
        ParityTarget { rsid: "rs1801133".to_string(), gene: "MTHFR".to_string(), condition: "C677T folate".to_string(), priority: "HIGH".to_string() },
        ParityTarget { rsid: "rs1801131".to_string(), gene: "MTHFR".to_string(), condition: "A1298C folate".to_string(), priority: "HIGH".to_string() },
        ParityTarget { rsid: "rs429358".to_string(), gene: "APOE".to_string(), condition: "ε4 Alzheimer".to_string(), priority: "HIGH".to_string() },
        ParityTarget { rsid: "rs7412".to_string(), gene: "APOE".to_string(), condition: "ε2 Alzheimer".to_string(), priority: "HIGH".to_string() },
        ParityTarget { rsid: "rs376694515".to_string(), gene: "KIT".to_string(), condition: "GIST".to_string(), priority: "MEDIUM".to_string() },
        ParityTarget { rsid: "rs769635594".to_string(), gene: "PALLD".to_string(), condition: "Pancreatic cancer".to_string(), priority: "MEDIUM".to_string() },
        ParityTarget { rsid: "rs202003805".to_string(), gene: "PRSS1".to_string(), condition: "Hereditary pancreatitis".to_string(), priority: "MEDIUM".to_string() },
        ParityTarget { rsid: "rs1232891794".to_string(), gene: "PRSS1".to_string(), condition: "Hereditary pancreatitis".to_string(), priority: "MEDIUM".to_string() },
        ParityTarget { rsid: "rs757111793".to_string(), gene: "PRSS1".to_string(), condition: "Hereditary pancreatitis".to_string(), priority: "MEDIUM".to_string() },
        // Lisa (LISA_QUALITY_CROSSREF)
        ParityTarget { rsid: "rs63750885".to_string(), gene: "MSH2".to_string(), condition: "Lynch Syndrome".to_string(), priority: "HIGH".to_string() },
        ParityTarget { rsid: "rs1801265".to_string(), gene: "DPYD".to_string(), condition: "Fluorouracil toxicity".to_string(), priority: "HIGH".to_string() },
        ParityTarget { rsid: "rs7294".to_string(), gene: "VKORC1".to_string(), condition: "Warfarin dosage".to_string(), priority: "HIGH".to_string() },
        ParityTarget { rsid: "rs20455".to_string(), gene: "KIF6".to_string(), condition: "Pravastatin".to_string(), priority: "HIGH".to_string() },
        ParityTarget { rsid: "rs2236379".to_string(), gene: "PRKCQ".to_string(), condition: "IBD".to_string(), priority: "MEDIUM".to_string() },
        ParityTarget { rsid: "rs11209026".to_string(), gene: "IL23R".to_string(), condition: "Crohn".to_string(), priority: "MEDIUM".to_string() },
        ParityTarget { rsid: "rs689".to_string(), gene: "INS".to_string(), condition: "Type 1 Diabetes".to_string(), priority: "MEDIUM".to_string() },
        ParityTarget { rsid: "rs34536443".to_string(), gene: "TYK2".to_string(), condition: "Psoriasis".to_string(), priority: "MEDIUM".to_string() },
        ParityTarget { rsid: "rs1393350".to_string(), gene: "TYR".to_string(), condition: "Melanoma".to_string(), priority: "MEDIUM".to_string() },
        ParityTarget { rsid: "rs2294008".to_string(), gene: "PSCA".to_string(), condition: "Gastric cancer".to_string(), priority: "MEDIUM".to_string() },
        ParityTarget { rsid: "rs4673993".to_string(), gene: "ATIC".to_string(), condition: "Methotrexate".to_string(), priority: "MEDIUM".to_string() },
        ParityTarget { rsid: "rs1051266".to_string(), gene: "SLC19A1".to_string(), condition: "Methotrexate".to_string(), priority: "MEDIUM".to_string() },
    ];
    list.sort_by(|a, b| a.rsid.cmp(&b.rsid));
    list.dedup_by(|a, b| a.rsid == b.rsid);
    list
}

/// Build rsID → gene map from parity targets (and any other known rsIDs). Used to annotate VCF-derived variants so condition checks and cascade scores can match.
pub fn rsid_to_gene_map() -> HashMap<String, String> {
    let mut map: HashMap<String, String> = sequencing_report_targets()
        .into_iter()
        .map(|t| (canonical_rsid(&t.rsid), t.gene))
        .collect();
    // Cascade-relevant genes not in parity: sulfur (CBS), OI/survival (COL1A1, COL1A2), cancer (BRCA1, BRCA2)
    let extra = [
        ("rs5742905", "CBS"),   // CBS 699C>T, homocystinuria
        ("rs234715", "CBS"),
        ("rs1047891", "CBS"),
        ("rs1801133", "MTHFR"),
        ("rs1801131", "MTHFR"),
        ("rs1800012", "COL1A1"), // COL1A1 Sp1 binding site; bone density / OI spectrum
        ("rs412777", "COL1A2"),  // COL1A2; osteogenesis imperfecta
        // BRCA1 (chr17) – HBOC
        ("rs799917", "BRCA1"),
        ("rs1799950", "BRCA1"),
        ("rs16941", "BRCA1"),
        ("rs16942", "BRCA1"),
        ("rs1799966", "BRCA1"),
        ("rs799908", "BRCA1"),
        ("rs8176318", "BRCA1"),
        // BRCA2 (chr13) – HBOC
        ("rs144848", "BRCA2"),
        ("rs1799943", "BRCA2"),
        ("rs1799955", "BRCA2"),
        ("rs1799954", "BRCA2"),
        ("rs15869", "BRCA2"),
        ("rs4987117", "BRCA2"),
        ("rs206075", "BRCA2"),
        ("rs1799967", "BRCA2"),
        // MCAS / mast cell instability integrated analysis
        ("rs1805087", "MTR"),
        ("rs1801394", "MTRR"),
        ("rs1532268", "MTRR"),
        ("rs4680", "COMT"),
        ("rs11558538", "HNMT"),
        ("rs10156191", "AOC1"),
        ("rs1049793", "AOC1"),
        ("rs1800629", "TNF"),
        ("rs1800795", "IL6"),
        ("rs1695", "GSTP1"),
        ("rs4880", "SOD2"),
        ("rs1050450", "GPX1"),
        ("rs2071746", "HMOX1"),
        ("rs1801133", "MTHFR"),
        ("rs1801131", "MTHFR"),
        // Exercise ammonia / nitrogen waste handling: urea cycle, AMP deamination, mitochondrial
        ("rs4148323", "UGT1A1"), // liver
        ("rs17602729", "AMPD1"), // AMPD1 C34T; exercise-related
        ("rs603601", "AMPD1"),
        ("rs3786525", "GLUL"),
        ("rs10911021", "GLUL"),
        ("rs4924", "CPT1A"),
        ("rs2229291", "CPT1A"),
        ("rs1799821", "CPT2"),
        ("rs1799822", "CPT2"),
    ];
    for (rsid, gene) in extra {
        map.insert(canonical_rsid(rsid), gene.to_string());
    }
    map
}

/// Check variant set against Sequencing.com report targets. Returns found/missing so report can show parity and gaps.
pub fn check_sequencing_parity(variants: &[VariantInput]) -> SequencingParityResult {
    let found_rsids: HashSet<String> = variants
        .iter()
        .filter_map(|v| v.rsid.as_ref())
        .map(|r| canonical_rsid(r))
        .filter(|s| !s.is_empty())
        .collect();

    let targets = sequencing_report_targets();
    let total = targets.len();
    let mut found = Vec::new();
    let mut missing = Vec::new();
    for t in targets {
        if found_rsids.contains(&canonical_rsid(&t.rsid)) {
            found.push(t);
        } else {
            missing.push(t);
        }
    }
    SequencingParityResult {
        found_count: found.len(),
        found,
        missing,
        total,
    }
}
