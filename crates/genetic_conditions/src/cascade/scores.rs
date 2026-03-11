//! Integrated CGRP / mast cell / calcium / mitochondrial cascade scores (0–100).
//! For research and educational use only; not for clinical diagnosis.

use serde::{Deserialize, Serialize};

/// Interpretation band for a 0–100 score.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScoreBand {
    LowSupport,    // 0–24
    MildSupport,   // 25–49
    ModerateSupport, // 50–74
    StrongSupport, // 75–100
}

pub fn score_band(score: u8) -> ScoreBand {
    match score {
        0..=24 => ScoreBand::LowSupport,
        25..=49 => ScoreBand::MildSupport,
        50..=74 => ScoreBand::ModerateSupport,
        _ => ScoreBand::StrongSupport,
    }
}

/// All 10 integrated cascade scores (0–100 each).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegratedScores {
    pub calcium_mast_cell_sensitivity: u8,
    pub trigeminal_calcium_excitability: u8,
    pub mitochondrial_stress_amplification: u8,
    pub histamine_mediator_burden_likelihood: u8,
    pub prostaglandin_mediator_burden_likelihood: u8,
    pub sulfur_burden_likelihood: u8,
    pub ammonia_burden_likelihood: u8,
    pub nitric_oxide_amplification: u8,
    pub waste_clearance_strain: u8,
    pub composite_cgrp_runaway_cascade: u8,
}

impl IntegratedScores {
    pub fn band(&self, name: &str) -> ScoreBand {
        let s = match name {
            "calcium_mast_cell_sensitivity" => self.calcium_mast_cell_sensitivity,
            "trigeminal_calcium_excitability" => self.trigeminal_calcium_excitability,
            "mitochondrial_stress_amplification" => self.mitochondrial_stress_amplification,
            "histamine_mediator_burden_likelihood" => self.histamine_mediator_burden_likelihood,
            "prostaglandin_mediator_burden_likelihood" => self.prostaglandin_mediator_burden_likelihood,
            "sulfur_burden_likelihood" => self.sulfur_burden_likelihood,
            "ammonia_burden_likelihood" => self.ammonia_burden_likelihood,
            "nitric_oxide_amplification" => self.nitric_oxide_amplification,
            "waste_clearance_strain" => self.waste_clearance_strain,
            "composite_cgrp_runaway_cascade" => self.composite_cgrp_runaway_cascade,
            _ => 0,
        };
        score_band(s)
    }
}
