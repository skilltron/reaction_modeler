//! **Genetic conditions:** consolidated module for checking variants against
//! disease and exposure-related genes.
//!
//! Submodules: **immune**, **exposure**, **inflammation**, **sulfur**, **rare**.
//! For research/educational use only; not for clinical diagnosis.

mod variant_input;
pub use variant_input::{ClinvarSummary, RegionType, VariantInput};

pub mod cancer;
pub mod cascade;
pub mod immune;
pub mod exposure;
pub mod inflammation;
pub mod html_report;
pub mod report_plain_text;
pub mod sulfur;
pub mod rare;
pub mod disorders;
pub mod supplements;
pub mod survival;
pub mod star_alleles;
pub mod gene_annotation;
pub mod pharmacopoeia;
pub mod sequencing_parity;
pub mod reference_check;
pub mod clinvar_lookup;
pub mod mcas_integrated;
pub mod exercise_ammonia;
pub mod copy_number;

use serde::{Deserialize, Serialize};

/// Aggregated report from all condition categories.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllConditionsReport {
    pub immune: Vec<immune::ImmuneDiseaseReport>,
    pub exposure: Vec<exposure::ChemicalDamageReport>,
    pub inflammation: Vec<inflammation::InflammationReport>,
    pub sulfur: Vec<sulfur::SulfurMetabolismReport>,
    pub rare: Vec<rare::RareDiseaseReport>,
    pub cancer: Vec<cancer::CancerScreeningReport>,
    pub disorders: Vec<disorders::DisorderReport>,
    /// KIT D816V mutation detected (minor criterion for systemic mastocytosis). This report checks genetics only; other minor criteria (tryptase, CD25, mast cell count) require lab/pathology.
    pub kit_d816v_detected: bool,
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
        cancer: cancer::check_variants_against_cancer_syndromes(variants),
        disorders: disorders::check_variants_against_disorders(variants),
        kit_d816v_detected: inflammation::has_kit_d816v(variants),
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
        assert!(!report.cancer.is_empty());
        assert!(report.cancer.len() >= 8);
        assert!(!report.disorders.is_empty());
        assert!(report.disorders.len() >= 10);
        assert!(!report.kit_d816v_detected);
    }

    #[test]
    fn kit_d816v_detected_when_present() {
        use crate::inflammation;
        let empty: Vec<VariantInput> = vec![];
        assert!(!inflammation::has_kit_d816v(&empty));
        let with_rsid = vec![VariantInput {
            chromosome: "4".to_string(),
            position: 55_599_352,
            gene: None,
            rsid: Some("rs121913529".to_string()),
            ref_allele: Some("A".to_string()),
            alt_allele: Some("T".to_string()),
            region_type: None,
            genotype: None,
            clinvar: None,
            confidence: None,
        }];
        assert!(inflammation::has_kit_d816v(&with_rsid));
        let report = check_variants_against_all(&with_rsid);
        assert!(report.kit_d816v_detected);
    }

    #[test]
    fn check_variants_kit_finding() {
        // KIT D816V (pathogenic) is counted as inflammation finding; other KIT variants only if ClinVar pathogenic.
        let variants = vec![VariantInput {
            chromosome: "4".to_string(),
            position: 55_599_352,
            gene: Some("KIT".to_string()),
            rsid: Some("rs121913529".to_string()),
            ref_allele: Some("A".to_string()),
            alt_allele: Some("T".to_string()),
            region_type: Some(RegionType::Coding),
            genotype: None,
            clinvar: None,
            confidence: None,
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
    fn annotation_fills_gene_from_rsid_and_cascade_scores_nonzero() {
        use crate::gene_annotation;
        use crate::cascade;
        use crate::variant_input::ClinvarSummary;
        let variants = vec![
            VariantInput {
                chromosome: "1".to_string(),
                position: 11_790_695,
                gene: None,
                rsid: Some("rs1801133".to_string()),
                ref_allele: Some("C".to_string()),
                alt_allele: Some("T".to_string()),
                region_type: None,
                genotype: None,
                clinvar: Some(ClinvarSummary {
                    classification: "Likely pathogenic".to_string(),
                    review_status: "".to_string(),
                    conditions: vec![],
                    accession: None,
                    last_evaluated: None,
                }),
                confidence: None,
            },
        ];
        let annotated = gene_annotation::annotate_variants_with_genes(variants);
        assert_eq!(annotated[0].gene.as_deref(), Some("MTHFR"));
        let report = check_variants_against_all(&annotated);
        let sulfur_findings: usize = report.sulfur.iter().map(|r| r.findings.len()).sum();
        assert!(sulfur_findings >= 1, "MTHFR variant with pathogenic ClinVar should produce sulfur finding after annotation");
        let cascade_report = cascade::compute_cascade_from_report(&report);
        assert!(cascade_report.scores.sulfur_burden_likelihood > 0);
        assert!(cascade_report.scores.composite_cgrp_runaway_cascade > 0);
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
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
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
        assert!(html.contains("Minor criteria"));
        assert!(html.contains("KIT D816V"));
        assert!(html.contains("tryptase"));
        assert!(html.contains("CD25"));
        assert!(html.contains("No matching variants"));
        assert!(html.contains("no-matching"));
        assert!(html.contains("<!DOCTYPE html>"));
    }

    #[test]
    fn supplements_count_at_most_15() {
        let supps = supplements::supplements_for_weak_areas();
        assert!(supps.len() <= 15);
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
                genotype: None,
                clinvar: None,
                confidence: None,
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
    fn star_allele_inference_cyp2c19() {
        let variants = vec![
            VariantInput {
                chromosome: "10".to_string(),
                position: 94_722_627,
                gene: Some("CYP2C19".to_string()),
                rsid: Some("rs4244285".to_string()),
                ref_allele: Some("G".to_string()),
                alt_allele: Some("A".to_string()),
                region_type: None,
                genotype: Some("0/1".to_string()),
                clinvar: None,
                confidence: None,
            },
        ];
        let results = star_alleles::infer_star_alleles(&variants);
        let cyp2c19 = results.iter().find(|r| r.gene == "CYP2C19").expect("CYP2C19 result");
        assert!(cyp2c19.alleles.contains(&"*2".to_string()));
        assert!(cyp2c19.diplotype.contains("*2"));
    }

    #[test]
    fn html_with_cascade_and_survival() {
        let variants: Vec<VariantInput> = vec![];
        let report = check_variants_against_all(&variants);
        let cr = cascade::compute_cascade_from_report(&report);
        let sa = survival::analyze_survival(&variants);
        let html = html_report::all_conditions_to_html(&report, "Test", "2025-01-01", None, Some(&cr), Some(&sa), None, None, None, None, None, None, None, None, None, None, None);
        assert!(html.contains("Cascade"));
        assert!(html.contains("Survival"));
        assert!(html.contains("Scores (0–100)"));
    }

    #[test]
    fn mcas_integrated_section_in_html() {
        let variants: Vec<VariantInput> = vec![];
        let report = check_variants_against_all(&variants);
        let mi = mcas_integrated::run_mcas_integrated_analysis(&variants);
        let html = html_report::all_conditions_to_html(&report, "Test", "2025-01-01", None, None, None, Some(&mi), None, None, None, None, None, None, None, None, None, None);
        assert!(html.contains("MCAS / Mast Cell Instability Integrated Analysis"));
        assert!(html.contains("mcas-integrated"));
        assert!(html.contains("Pathway breakdown"));
        assert!(html.contains("Cascade analysis"));
        assert!(html.contains("Interventions"));
        assert!(html.contains("User-relevant pattern"));
        assert_eq!(mi.pathway_reports.len(), 5);
    }

    #[test]
    fn exercise_ammonia_section_in_html() {
        let variants: Vec<VariantInput> = vec![];
        let report = check_variants_against_all(&variants);
        let inflammation_count: usize = report.inflammation.iter().map(|r| r.findings.len()).sum();
        let ea = exercise_ammonia::run_exercise_ammonia_analysis(&variants, inflammation_count);
        let html = html_report::all_conditions_to_html(&report, "Test", "2025-01-01", None, None, None, None, Some(&ea), None, None, None, None, None, None, None, None, None);
        assert!(html.contains("Exercise Ammonia / Nitrogen Waste Handling Integrated Analysis"));
        assert!(html.contains("exercise-ammonia"));
        assert!(html.contains("Pathway breakdown"));
        assert!(html.contains("Cascade analysis"));
        assert!(html.contains("User-facing interpretation"));
        assert!(html.contains("CPS1"));
        assert!(html.contains("User-relevant context"));
        assert_eq!(ea.pathway_reports.len(), 6);
    }
}
