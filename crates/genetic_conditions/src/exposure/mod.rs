//! Environmental exposure: gene susceptibility patterns for TCE, radon, benzene, arsenic, ionizing radiation.

mod chemicals;
mod report;

pub use chemicals::{chemical_effect_pattern, ChemicalEffectPattern, ChemicalName};
pub use report::{check_variants_against_chemical, ChemicalDamageReport, GeneVariantFinding};
