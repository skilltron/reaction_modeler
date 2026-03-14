//! Check variants against immune-disease genes. Reports normal (reference) and change (alternate) alleles and region type (coding vs non-coding) when present.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::variant_input::{is_pathogenic_or_likely_pathogenic, RegionType, VariantInput};
use super::diseases::{list_immune_diseases, EvidenceLevel, ImmuneDiseaseRef};

/// One variant finding: includes normal (reference) and change (alternate), and region type (coding vs non-coding) when available.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmuneDiseaseFinding {
    pub disease_name: String,
    pub genetic_cause_indicated: bool,
    pub evidence_level: EvidenceLevel,
    pub gene: String,
    pub variant: VariantInput,
    /// Reference (normal) allele when available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_allele: Option<String>,
    /// Alternate (change) allele when available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_allele: Option<String>,
    /// Coding vs non-coding; shown so non-coding changes are reported when no coding variants present for that gene.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_type: Option<RegionType>,
    pub note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmuneDiseaseReport {
    pub disease_name: String,
    pub genetic_cause_indicated: bool,
    pub evidence_level: EvidenceLevel,
    pub genes_checked: Vec<String>,
    pub findings: Vec<ImmuneDiseaseFinding>,
    pub references: Vec<String>,
    pub disclaimer: String,
}

const DISCLAIMER: &str = "For research and educational use only. Not for clinical diagnosis. \
    Gene associations are from published literature; presence of variants does not \
    establish diagnosis or indicate causation.";

pub fn check_variants_against_immune_diseases(variants: &[VariantInput]) -> Vec<ImmuneDiseaseReport> {
    let diseases = list_immune_diseases();
    let mut reports = Vec::with_capacity(diseases.len());
    for d in &diseases {
        reports.push(build_report_for_disease(d, variants));
    }
    reports
}

fn build_report_for_disease(disease: &ImmuneDiseaseRef, variants: &[VariantInput]) -> ImmuneDiseaseReport {
    let gene_set: HashSet<String> = disease.genes.iter().map(|g| g.to_uppercase()).collect();
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
        findings.push(ImmuneDiseaseFinding {
            disease_name: disease.name.clone(),
            genetic_cause_indicated: disease.genetic_cause_indicated,
            evidence_level: disease.evidence_level,
            gene: gene.clone(),
            variant: v.clone(),
            reference_allele: v.ref_allele.clone(),
            alternate_allele: v.alt_allele.clone(),
            region_type: v.region_type,
            note: format!(
                "Variant in {} ({}); genetic cause/susceptibility well-established. {}",
                gene,
                disease.name,
                v.region_type.map(|r| format!("Region: {}.", r.as_str())).unwrap_or_else(|| "Region: unknown.".to_string())
            ),
        });
    }
    ImmuneDiseaseReport {
        disease_name: disease.name.clone(),
        genetic_cause_indicated: disease.genetic_cause_indicated,
        evidence_level: disease.evidence_level,
        genes_checked: disease.genes.clone(),
        findings,
        references: disease.references.clone(),
        disclaimer: DISCLAIMER.to_string(),
    }
}
