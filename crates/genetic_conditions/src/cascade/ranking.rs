//! Pathway ranking and cascade summary.
//! For research and educational use only; not for clinical diagnosis.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathwayRanking {
    pub primary_drivers: Vec<String>,
    pub secondary_amplifiers: Vec<String>,
    pub downstream_manifestations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathwayTableRow {
    pub pathway_or_burden: String,
    pub score: u8,
    pub main_genes: String,
    pub phenotype_match: String,
    pub likely_role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildupTableRow {
    pub suspected_buildup: String,
    pub why_present: String,
    pub likely_symptoms: String,
    pub cleanup_need: String,
    pub impact_1_to_10: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediatorPatternRow {
    pub mediator_pattern: String,
    pub supporting_clues: String,
    pub confidence: String,
    pub main_downstream_risks: String,
}
