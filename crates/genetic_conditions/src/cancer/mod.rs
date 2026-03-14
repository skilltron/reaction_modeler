//! Genetic cancer screening: hereditary cancer syndrome genes only.
//! For research/educational use; not for clinical diagnosis.

mod report;
mod syndromes;

pub use report::{
    check_variants_against_cancer_syndromes,
    CancerScreeningFinding,
    CancerScreeningReport,
};
pub use syndromes::{list_hereditary_cancer_syndromes, CancerSyndromeRef};
