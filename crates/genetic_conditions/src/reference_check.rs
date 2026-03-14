//! Cross-check variant data against well-known marker positions to detect reference-build drift
//! and alignment issues. Uses markers in well-known genes (MTHFR, APOE, F5, BRCA1, BRCA2) so we
//! can infer GRCh37 vs GRCh38 and flag inconsistent or unexpected positions.

use crate::variant_input::VariantInput;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Expected position for one reference build. Chromosome as string (e.g. "1", "17").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpectedPosition {
    pub chromosome: String,
    pub position: u64,
}

/// One marker variant we use for cross-check (rsID + expected positions per build).
#[derive(Debug, Clone)]
pub struct MarkerRef {
    pub rsid: String,
    pub gene: String,
    pub grch37: ExpectedPosition,
    pub grch38: ExpectedPosition,
}

/// Result of checking one marker in the user's data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkerCheckResult {
    pub rsid: String,
    pub gene: String,
    /// Chromosome as seen in the data (may be "1" or "chr1").
    pub observed_chromosome: String,
    pub observed_position: u64,
    /// Matches GRCh37 expected position (within tolerance).
    pub matches_grch37: bool,
    /// Matches GRCh38 expected position (within tolerance).
    pub matches_grch38: bool,
    /// No variant with this rsID found in the data.
    pub not_found: bool,
}

/// Outcome of the reference cross-check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceCheckResult {
    /// "GRCh37" | "GRCh38" | "Unknown" | "Inconsistent"
    pub inferred_build: String,
    /// Human-readable summary.
    pub summary: String,
    /// Per-marker results (only for markers that were checked, i.e. present in data or explicitly missing).
    pub markers: Vec<MarkerCheckResult>,
    /// Recommendation (e.g. "Data consistent with GRCh38" or "Position drift detected; confirm reference build.").
    pub recommendation: String,
}

fn canonical_rsid(s: &str) -> String {
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

fn chrom_normalize(chrom: &str) -> String {
    chrom.trim_start_matches("chr").to_string()
}

/// Position tolerance (bp): allow small differences for patch builds.
const POS_TOLERANCE: u64 = 500;

fn position_matches(observed_pos: u64, expected_pos: u64) -> bool {
    observed_pos.abs_diff(expected_pos) <= POS_TOLERANCE
}

/// Well-known markers: rsID, gene, GRCh37 (chr, pos), GRCh38 (chr, pos). Positions from dbSNP/NCBI/Ensembl.
fn marker_refs() -> Vec<MarkerRef> {
    vec![
        MarkerRef {
            rsid: "rs1801133".to_string(),
            gene: "MTHFR".to_string(),
            grch37: ExpectedPosition { chromosome: "1".to_string(), position: 11_796_321 },
            grch38: ExpectedPosition { chromosome: "1".to_string(), position: 11_796_321 },
        },
        MarkerRef {
            rsid: "rs429358".to_string(),
            gene: "APOE".to_string(),
            grch37: ExpectedPosition { chromosome: "19".to_string(), position: 45_411_941 },
            grch38: ExpectedPosition { chromosome: "19".to_string(), position: 44_908_684 },
        },
        MarkerRef {
            rsid: "rs6025".to_string(),
            gene: "F5".to_string(),
            grch37: ExpectedPosition { chromosome: "1".to_string(), position: 169_519_049 },
            grch38: ExpectedPosition { chromosome: "1".to_string(), position: 169_549_811 },
        },
        MarkerRef {
            rsid: "rs1799950".to_string(),
            gene: "BRCA1".to_string(),
            grch37: ExpectedPosition { chromosome: "17".to_string(), position: 41_197_694 },
            grch38: ExpectedPosition { chromosome: "17".to_string(), position: 43_044_295 },
        },
        MarkerRef {
            rsid: "rs144848".to_string(),
            gene: "BRCA2".to_string(),
            grch37: ExpectedPosition { chromosome: "13".to_string(), position: 32_315_479 },
            grch38: ExpectedPosition { chromosome: "13".to_string(), position: 32_315_479 },
        },
    ]
}

/// Build a map from canonical rsID to (first) variant's chromosome and position in the data.
fn rsid_to_observed(variants: &[VariantInput]) -> HashMap<String, (String, u64)> {
    let mut map = HashMap::new();
    for v in variants {
        if let Some(ref rsid) = v.rsid {
            let key = canonical_rsid(rsid);
            if !key.is_empty() && !map.contains_key(&key) {
                map.insert(key, (v.chromosome.clone(), v.position));
            }
        }
    }
    map
}

/// Run reference cross-check on the variant set. Detects build (GRCh37 vs GRCh38) and flags drift.
pub fn run_reference_check(variants: &[VariantInput]) -> ReferenceCheckResult {
    let refs = marker_refs();
    let observed = rsid_to_observed(variants);
    let mut markers = Vec::with_capacity(refs.len());

    for m in &refs {
        let key = canonical_rsid(&m.rsid);
        let (matches_37, matches_38, not_found, observed_chr, observed_pos) = match observed.get(&key) {
            None => (false, false, true, String::new(), 0),
            Some((chr, pos)) => {
                let chr_norm = chrom_normalize(chr);
                let matches_37 = chr_norm == chrom_normalize(&m.grch37.chromosome)
                    && position_matches(*pos, m.grch37.position);
                let matches_38 = chr_norm == chrom_normalize(&m.grch38.chromosome)
                    && position_matches(*pos, m.grch38.position);
                (matches_37, matches_38, false, chr.clone(), *pos)
            }
        };
        markers.push(MarkerCheckResult {
            rsid: m.rsid.clone(),
            gene: m.gene.clone(),
            observed_chromosome: observed_chr,
            observed_position: observed_pos,
            matches_grch37: matches_37,
            matches_grch38: matches_38,
            not_found,
        });
    }

    let found_markers: Vec<_> = markers.iter().filter(|x| !x.not_found).collect();
    let any_match_37 = found_markers.iter().any(|x| x.matches_grch37);
    let any_match_38 = found_markers.iter().any(|x| x.matches_grch38);
    let any_mismatch = found_markers.iter().any(|x| !x.matches_grch37 && !x.matches_grch38);

    let (inferred_build, summary, recommendation) = if found_markers.is_empty() {
        (
            "Unknown".to_string(),
            "No cross-check markers found in your variant set; cannot infer reference build.".to_string(),
            "Add data that includes common markers (e.g. MTHFR rs1801133, APOE rs429358, F5 rs6025, BRCA1/BRCA2) to enable reference check.".to_string(),
        )
    } else if any_mismatch && !any_match_37 && !any_match_38 {
        (
            "Inconsistent".to_string(),
            format!(
                "{} marker(s) found but positions do not match GRCh37 or GRCh38; possible alignment or build mismatch.",
                found_markers.len()
            ),
            "Confirm the reference genome used to produce your VCF (GRCh37/hg19 vs GRCh38/hg38). If the build is correct, positions may have shifted; consider re-alignment or re-calling.".to_string(),
        )
    } else if any_match_38 && !any_match_37 {
        (
            "GRCh38".to_string(),
            format!(
                "{} marker(s) checked; positions consistent with GRCh38.",
                found_markers.len()
            ),
            "Data appear consistent with GRCh38. No correction needed.".to_string(),
        )
    } else if any_match_37 && !any_match_38 {
        (
            "GRCh37".to_string(),
            format!(
                "{} marker(s) checked; positions consistent with GRCh37 (hg19).",
                found_markers.len()
            ),
            "Data appear consistent with GRCh37/hg19. Gene position-based annotation in this report uses GRCh38 ranges; BRCA1/BRCA2 position check covers both builds.".to_string(),
        )
    } else if any_match_38 {
        (
            "GRCh38".to_string(),
            format!(
                "{} marker(s) checked; positions consistent with GRCh38 (some markers same in both builds).",
                found_markers.len()
            ),
            "Data appear consistent with GRCh38. No correction needed.".to_string(),
        )
    } else {
        (
            "Unknown".to_string(),
            "Could not infer reference build from marker positions.".to_string(),
            "Confirm reference genome (GRCh37 vs GRCh38) with your data provider.".to_string(),
        )
    };

    ReferenceCheckResult {
        inferred_build,
        summary,
        markers,
        recommendation,
    }
}
