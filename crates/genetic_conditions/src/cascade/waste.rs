//! Waste / byproduct / intermediate buildup and clearance needs.
//! For research and educational use only; not for clinical diagnosis.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspectedBuildup {
    pub category: String,
    pub why_may_accumulate: String,
    pub broken_or_overloaded_process: String,
    pub possible_symptoms: String,
    pub may_worsen: String,
    pub clearance_category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearanceNeed {
    pub process_support: String,
    pub why_needed: String,
    pub symptoms_it_may_reduce: String,
    pub impact_1_to_10: u8,
    pub role: RoleLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoleLevel {
    Primary,
    Secondary,
    Compensatory,
}
