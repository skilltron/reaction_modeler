//! Integrated CGRP / mast cell / calcium / mitochondrial cascade.
//! Uses existing inflammation, sulfur, and related modules; produces scores, ranking, waste/clearance.
//! For research and educational use only; not for clinical diagnosis.

mod ranking;
mod scores;
mod waste;

pub use ranking::{BuildupTableRow, MediatorPatternRow, PathwayRanking, PathwayTableRow};
pub use scores::{IntegratedScores, ScoreBand, score_band};
pub use waste::{ClearanceNeed, RoleLevel, SuspectedBuildup};

use crate::AllConditionsReport;
use std::cmp::min;

/// Integrated cascade report: scores, ranking, suspected buildup, clearance needs.
#[derive(Debug, Clone)]
pub struct IntegratedCascadeReport {
    pub scores: IntegratedScores,
    pub ranking: PathwayRanking,
    pub suspected_buildups: Vec<SuspectedBuildup>,
    pub clearance_needs: Vec<ClearanceNeed>,
}

/// Compute integrated cascade from existing condition reports (genotype-driven; phenotype can refine later).
pub fn compute_cascade_from_report(report: &AllConditionsReport) -> IntegratedCascadeReport {
    let inflammation_finding_count = report
        .inflammation
        .iter()
        .map(|r| r.findings.len())
        .sum::<usize>();
    let sulfur_finding_count = report
        .sulfur
        .iter()
        .map(|r| r.findings.len())
        .sum::<usize>();

    let mast: u8 = min(100, inflammation_finding_count * 25) as u8;
    let sulf: u8 = min(100, sulfur_finding_count * 20) as u8;
    let composite: u8 = min(100, (mast as u16 + sulf as u16) / 2) as u8;

    let scores = IntegratedScores {
        calcium_mast_cell_sensitivity: mast,
        trigeminal_calcium_excitability: mast.saturating_sub(10).min(100),
        mitochondrial_stress_amplification: 30,
        histamine_mediator_burden_likelihood: mast,
        prostaglandin_mediator_burden_likelihood: mast.saturating_sub(15).min(100),
        sulfur_burden_likelihood: sulf,
        ammonia_burden_likelihood: 25,
        nitric_oxide_amplification: 25,
        waste_clearance_strain: min(100, (mast as u16 + sulf as u16) / 2) as u8,
        composite_cgrp_runaway_cascade: composite,
    };

    let ranking = PathwayRanking {
        primary_drivers: vec![
            "Mast cell / histamine / prostaglandin activation (if KIT/TPSAB1 or phenotype)".to_string(),
            "Mitochondrial energy stress lowering calcium threshold (if supported by variants)".to_string(),
        ],
        secondary_amplifiers: vec![
            "Nitric oxide dysregulation".to_string(),
            "Sulfur / sulfite burden (CBS, SUOX, MOCS, CTH)".to_string(),
            "Ammonia buildup during exertion (urea cycle)".to_string(),
            "Impaired histamine clearance (DAO/HNMT)".to_string(),
        ],
        downstream_manifestations: vec![
            "CGRP migraine".to_string(),
            "Gut pain".to_string(),
            "Joint inflammatory pain".to_string(),
            "Flushing / facial burning".to_string(),
            "Exercise intolerance".to_string(),
        ],
    };

    let suspected_buildups = vec![
        SuspectedBuildup {
            category: "Histamine".to_string(),
            why_may_accumulate: "Mast cell degranulation or reduced DAO/HNMT clearance.".to_string(),
            broken_or_overloaded_process: "Histamine breakdown or mast cell stabilisation.".to_string(),
            possible_symptoms: "Flushing, itch, headache, gut pain, nasal congestion.".to_string(),
            may_worsen: "Mast cell activation, migraine, gut pain, skin.".to_string(),
            clearance_category: "Histamine breakdown support".to_string(),
        },
        SuspectedBuildup {
            category: "Sulfite / H2S".to_string(),
            why_may_accumulate: "CBS/SUOX/CTH variants or flux imbalance.".to_string(),
            broken_or_overloaded_process: "Transsulfuration or sulfite oxidase.".to_string(),
            possible_symptoms: "Reactions to wine, dried fruit, sulfur foods; flushing, headache.".to_string(),
            may_worsen: "Mast cell irritation, redox stress, headache.".to_string(),
            clearance_category: "Sulfur / sulfite handling support".to_string(),
        },
    ];

    let clearance_needs = vec![
        ClearanceNeed {
            process_support: "Mast cell stabilisation".to_string(),
            why_needed: "Reduce degranulation and mediator release.".to_string(),
            symptoms_it_may_reduce: "Flushing, gut pain, headache, skin reactivity.".to_string(),
            impact_1_to_10: 8,
            role: waste::RoleLevel::Primary,
        },
        ClearanceNeed {
            process_support: "Antioxidant / ROS cleanup".to_string(),
            why_needed: "Mitochondrial or redox stress.".to_string(),
            symptoms_it_may_reduce: "Fatigue, post-exertional malaise, headache.".to_string(),
            impact_1_to_10: 6,
            role: waste::RoleLevel::Secondary,
        },
    ];

    IntegratedCascadeReport {
        scores,
        ranking,
        suspected_buildups,
        clearance_needs,
    }
}
