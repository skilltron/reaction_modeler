//! Disorders: psychiatric, autoimmune, neurological, metabolic (susceptibility genes).
//! Separate from rare genetic diseases; these are polygenic/complex disorders.

mod diseases;
mod report;

pub use diseases::list_disorders;
pub use report::{check_variants_against_disorders, DisorderFinding, DisorderReport};
