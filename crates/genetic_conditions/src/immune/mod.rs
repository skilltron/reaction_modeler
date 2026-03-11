//! Immune diseases with established genetic cause or susceptibility.

mod diseases;
mod report;

pub use diseases::{EvidenceLevel, ImmuneDiseaseRef, list_immune_diseases};
pub use report::{
    check_variants_against_immune_diseases, ImmuneDiseaseFinding, ImmuneDiseaseReport,
};
