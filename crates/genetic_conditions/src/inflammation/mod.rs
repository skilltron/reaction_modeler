//! Inflammation: MCAS/mastocytosis (KIT, TPSAB1) and SLK/Theodore's syndrome reference.

mod mcas;
mod report;
mod slk;

pub use mcas::{
    MastCellDisorderKind, MastCellDisorderRef, McasStabilizer,
    mcas_mastocytosis_ref, mcas_recommended_combo_with_cromolyn, mcas_stabilizers_ref,
};
pub use report::{check_variants_against_inflammation, InflammationFinding, InflammationReport};
pub use slk::{SlkRef, slk_theodores_ref};
