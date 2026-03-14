//! Check variants against hereditary cancer syndrome genes. Genetic-only screening; not clinical diagnosis.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::variant_input::{is_pathogenic_or_likely_pathogenic, RegionType, VariantInput};
use super::syndromes::list_hereditary_cancer_syndromes;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancerScreeningFinding {
    pub syndrome_name: String,
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
pub struct CancerScreeningReport {
    pub syndrome_name: String,
    pub genes_checked: Vec<String>,
    pub findings: Vec<CancerScreeningFinding>,
    pub screening_notes: Vec<String>,
    pub references: Vec<String>,
    pub disclaimer: String,
}

const DISCLAIMER: &str = "For research and educational use only. Not for clinical diagnosis. \
Genetic cancer screening here is gene-based only; pathogenicity and clinical action require accredited testing and genetic counseling.";

pub fn check_variants_against_cancer_syndromes(variants: &[VariantInput]) -> Vec<CancerScreeningReport> {
    let syndromes = list_hereditary_cancer_syndromes();
    let mut reports = Vec::with_capacity(syndromes.len());
    for s in &syndromes {
        let gene_set: HashSet<String> = s.genes.iter().map(|g| g.to_uppercase()).collect();
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
            findings.push(CancerScreeningFinding {
                syndrome_name: s.name.clone(),
                gene: gene.clone(),
                variant: v.clone(),
                reference_allele: v.ref_allele.clone(),
                alternate_allele: v.alt_allele.clone(),
                region_type: v.region_type,
                note: format!("Variant in {} ({});{}", gene, s.name, region_note),
            });
        }
        reports.push(CancerScreeningReport {
            syndrome_name: s.name.clone(),
            genes_checked: s.genes.clone(),
            findings,
            screening_notes: s.screening_notes.clone(),
            references: s.references.clone(),
            disclaimer: DISCLAIMER.to_string(),
        });
    }
    reports
}
