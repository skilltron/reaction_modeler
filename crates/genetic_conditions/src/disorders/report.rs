//! Check variants against disorder susceptibility genes. Reports variants in genes associated with psychiatric, autoimmune, neurological, and metabolic disorders.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::variant_input::{is_pathogenic_or_likely_pathogenic, RegionType, VariantInput};
use super::diseases::list_disorders;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisorderFinding {
    pub disorder_name: String,
    pub category: String,
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
pub struct DisorderReport {
    pub disorder_name: String,
    pub category: String,
    pub genes_checked: Vec<String>,
    pub findings: Vec<DisorderFinding>,
    pub references: Vec<String>,
    pub disclaimer: String,
}

const DISCLAIMER: &str = "For research and educational use only. Not for clinical diagnosis. \
    These are susceptibility/association genes from GWAS and literature; variants do not establish diagnosis.";

pub fn check_variants_against_disorders(variants: &[VariantInput]) -> Vec<DisorderReport> {
    let disorders = list_disorders();
    let mut reports = Vec::with_capacity(disorders.len());
    for d in &disorders {
        let gene_set: HashSet<String> = d.genes.iter().map(|g| g.to_uppercase()).collect();
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
            findings.push(DisorderFinding {
                disorder_name: d.name.clone(),
                category: d.category.clone(),
                gene: gene.clone(),
                variant: v.clone(),
                reference_allele: v.ref_allele.clone(),
                alternate_allele: v.alt_allele.clone(),
                region_type: v.region_type,
                note: format!("Variant in {} ({});{}", gene, d.name, region_note),
            });
        }
        reports.push(DisorderReport {
            disorder_name: d.name.clone(),
            category: d.category.clone(),
            genes_checked: d.genes.clone(),
            findings,
            references: d.references.clone(),
            disclaimer: DISCLAIMER.to_string(),
        });
    }
    reports
}
