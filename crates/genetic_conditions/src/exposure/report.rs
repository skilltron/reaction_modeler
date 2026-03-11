//! Check variants against chemical effect patterns. Reports normal (reference) and change (alternate) and region type (coding vs non-coding) when present.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::variant_input::{RegionType, VariantInput};
use super::chemicals::{chemical_effect_pattern, ChemicalName};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneVariantFinding {
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
pub struct ChemicalDamageReport {
    pub chemical_name: String,
    pub chemical_description: String,
    pub pathways: Vec<String>,
    pub genes_checked: Vec<String>,
    pub findings: Vec<GeneVariantFinding>,
    pub references: Vec<String>,
    pub disclaimer: String,
}

const DISCLAIMER: &str = "For research and educational use only. Not for clinical diagnosis. \
    Gene associations are from published literature; presence of variants does not \
    prove exposure or damage.";

pub fn check_variants_against_chemical(chemical: ChemicalName, variants: &[VariantInput]) -> ChemicalDamageReport {
    let pattern = chemical_effect_pattern(chemical);
    let gene_set: HashSet<String> = pattern.genes.iter().map(|g| g.to_uppercase()).collect();
    let mut findings = Vec::with_capacity(variants.len());
    for v in variants {
        let gene = match &v.gene {
            Some(g) => g.to_uppercase(),
            None => continue,
        };
        if gene_set.contains(&gene) {
            let region_note = v.region_type.map(|r| format!(" Region: {}.", r.as_str())).unwrap_or_default();
            findings.push(GeneVariantFinding {
                gene: gene.clone(),
                variant: v.clone(),
                reference_allele: v.ref_allele.clone(),
                alternate_allele: v.alt_allele.clone(),
                region_type: v.region_type,
                note: format!(
                    "Variant in {} ({}); may relate to susceptibility or exposure-related pathways.{}",
                    gene, pattern.name, region_note
                ),
            });
        }
    }
    ChemicalDamageReport {
        chemical_name: pattern.name.clone(),
        chemical_description: pattern.description,
        pathways: pattern.pathways,
        genes_checked: pattern.genes,
        findings,
        references: pattern.references,
        disclaimer: DISCLAIMER.to_string(),
    }
}
