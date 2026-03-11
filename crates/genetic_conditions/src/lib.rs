//! **Genetic conditions:** consolidated module for checking variants against
//! disease and exposure-related genes.
//!
//! Submodules: **immune**, **exposure**, **inflammation**, **sulfur**, **rare**.
//! For research/educational use only; not for clinical diagnosis.

mod variant_input;
pub use variant_input::{RegionType, VariantInput};

pub mod cascade;
pub mod immune;
pub mod exposure;
pub mod inflammation;
pub mod html_report;
pub mod sulfur;
pub mod rare;
pub mod supplements;
pub mod survival;

use serde::{Deserialize, Serialize};

/// Aggregated report from all condition categories.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllConditionsReport {
    pub immune: Vec<immune::ImmuneDiseaseReport>,
    pub exposure: Vec<exposure::ChemicalDamageReport>,
    pub inflammation: Vec<inflammation::InflammationReport>,
    pub sulfur: Vec<sulfur::SulfurMetabolismReport>,
    pub rare: Vec<rare::RareDiseaseReport>,
}

/// Run variant checks against all categories and return one aggregated report.
#[inline(always)]
pub fn check_variants_against_all(variants: &[VariantInput]) -> AllConditionsReport {
    const CHEMICALS: [exposure::ChemicalName; 5] = [
        exposure::ChemicalName::Trichloroethylene,
        exposure::ChemicalName::Radon,
        exposure::ChemicalName::Benzene,
        exposure::ChemicalName::Arsenic,
        exposure::ChemicalName::IonizingRadiation,
    ];
    let mut exposure_reports = Vec::with_capacity(CHEMICALS.len());
    for &c in &CHEMICALS {
        exposure_reports.push(exposure::check_variants_against_chemical(c, variants));
    }
    AllConditionsReport {
        immune: immune::check_variants_against_immune_diseases(variants),
        exposure: exposure_reports,
        inflammation: inflammation::check_variants_against_inflammation(variants),
        sulfur: sulfur::check_variants_against_sulfur_metabolism(variants),
        rare: rare::check_variants_against_rare_diseases(variants),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::html_report;
    use crate::variant_input::RegionType;

    #[test]
    fn check_variants_empty() {
        let variants: Vec<VariantInput> = vec![];
        let report = check_variants_against_all(&variants);
        assert!(!report.immune.is_empty());
        assert!(report.inflammation.len() >= 4);
        assert!(!report.exposure.is_empty());
        assert!(!report.sulfur.is_empty());
        assert!(!report.rare.is_empty());
    }

    #[test]
    fn check_variants_kit_finding() {
        let variants = vec![VariantInput {
            chromosome: "4".to_string(),
            position: 55_593_605,
            gene: Some("KIT".to_string()),
            rsid: None,
            ref_allele: Some("A".to_string()),
            alt_allele: Some("G".to_string()),
            region_type: Some(RegionType::Coding),
        }];
        let report = check_variants_against_all(&variants);
        let inflammation_with_findings: Vec<_> = report
            .inflammation
            .iter()
            .filter(|r| !r.findings.is_empty())
            .collect();
        assert!(!inflammation_with_findings.is_empty());
        assert!(inflammation_with_findings
            .iter()
            .any(|r| r.genes_checked.contains(&"KIT".to_string())));
    }

    #[test]
    fn html_contains_mcas_and_supplements() {
        let variants: Vec<VariantInput> = vec![];
        let report = check_variants_against_all(&variants);
        let html = html_report::all_conditions_to_html(
            &report,
            "Test Report",
            "2025-01-01",
            None,
            None,
        );
        assert!(html.contains("MCAS"));
        assert!(html.contains("Cromolyn"));
        assert!(html.contains("supplements"));
        assert!(html.contains("Supplements"));
        assert!(html.contains("Methylfolate"));
        assert!(html.contains("Quercetin"));
        assert!(html.contains("Disclaimer"));
        assert!(html.contains("<!DOCTYPE html>"));
    }

    #[test]
    fn supplements_count_at_most_8() {
        let supps = supplements::supplements_for_weak_areas();
        assert!(supps.len() <= 8);
        assert!(supps.len() >= 7);
    }

    #[test]
    fn cascade_from_report() {
        let variants: Vec<VariantInput> = vec![];
        let report = check_variants_against_all(&variants);
        let cr = cascade::compute_cascade_from_report(&report);
        assert!(cr.scores.composite_cgrp_runaway_cascade <= 100);
        assert!(!cr.ranking.primary_drivers.is_empty());
        assert!(!cr.suspected_buildups.is_empty());
        assert!(!cr.clearance_needs.is_empty());
    }

    #[test]
    fn survival_analysis_empty() {
        let variants: Vec<VariantInput> = vec![];
        let sa = survival::analyze_survival(&variants);
        assert!(sa.genes_with_severe_phenotype.is_empty());
        assert!(sa.summary.contains("No variants"));
    }

    #[test]
    fn survival_analysis_col1a1() {
        let variants = vec![
            VariantInput {
                chromosome: "17".to_string(),
                position: 48_212_000,
                gene: Some("COL1A1".to_string()),
                rsid: None,
                ref_allele: None,
                alt_allele: None,
                region_type: None,
            },
        ];
        let sa = survival::analyze_survival(&variants);
        assert!(sa.genes_with_severe_phenotype.iter().any(|g| g == "COL1A1"));
        assert!(sa.by_gene.contains_key("COL1A1"));
        let r = sa.by_gene.get("COL1A1").unwrap();
        assert!(r.condition.contains("Osteogenesis"));
        assert!(r.reasons_for_mild_or_survival.iter().any(|x| x.reason.contains("Incomplete") || x.reason.contains("mild")));
    }

    #[test]
    fn html_with_cascade_and_survival() {
        let variants: Vec<VariantInput> = vec![];
        let report = check_variants_against_all(&variants);
        let cr = cascade::compute_cascade_from_report(&report);
        let sa = survival::analyze_survival(&variants);
        let html = html_report::all_conditions_to_html(&report, "Test", "2025-01-01", Some(&cr), Some(&sa));
        assert!(html.contains("Cascade"));
        assert!(html.contains("Survival"));
        assert!(html.contains("Scores (0–100)"));
    }
}
