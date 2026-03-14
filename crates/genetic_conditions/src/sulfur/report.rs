//! Check variants against sulfur metabolism genes. Reports normal/change and region type (coding vs non-coding) when present.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::variant_input::{is_pathogenic_or_likely_pathogenic, RegionType, VariantInput};
use super::conditions::list_sulfur_metabolism_conditions;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SulfurMetabolismFinding {
    pub condition_name: String,
    pub gene: String,
    pub variant: VariantInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_allele: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_allele: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_type: Option<RegionType>,
    pub note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SulfurMetabolismReport {
    pub condition_name: String,
    pub genes_checked: Vec<String>,
    pub findings: Vec<SulfurMetabolismFinding>,
    pub references: Vec<String>,
    pub disclaimer: String,
}

const DISCLAIMER: &str = "For research and educational use only. Not for clinical diagnosis. \
    Gene associations from OMIM/GeneReviews; presence of variants does not establish diagnosis.";

pub fn check_variants_against_sulfur_metabolism(variants: &[VariantInput]) -> Vec<SulfurMetabolismReport> {
    let conditions = list_sulfur_metabolism_conditions();
    let mut reports = Vec::with_capacity(conditions.len());
    for c in &conditions {
        let gene_set: HashSet<String> = c.genes.iter().map(|g| g.to_uppercase()).collect();
        let mut seen = HashSet::new();
        let mut findings = Vec::with_capacity(variants.len());
        for v in variants {
            let gene = match &v.gene {
                Some(g) => g.to_uppercase(),
                None => continue,
            };
            if !gene_set.contains(&gene) {
                continue;
            }
            if !is_pathogenic_or_likely_pathogenic(v) {
                continue;
            }
            let key = v.dedup_key();
            if seen.contains(&key) {
                continue;
            }
            seen.insert(key);
            let region_note = v.region_type.map(|r| format!(" Region: {}.", r.as_str())).unwrap_or_default();
            findings.push(SulfurMetabolismFinding {
                condition_name: c.name.clone(),
                gene: gene.clone(),
                variant: v.clone(),
                reference_allele: v.ref_allele.clone(),
                alternate_allele: v.alt_allele.clone(),
                region_type: v.region_type,
                note: format!("Variant in {} (sulfur metabolism); may relate to {}.{}", gene, c.name, region_note),
            });
        }
        reports.push(SulfurMetabolismReport {
            condition_name: c.name.clone(),
            genes_checked: c.genes.clone(),
            findings,
            references: c.references.clone(),
            disclaimer: DISCLAIMER.to_string(),
        });
    }
    reports
}
