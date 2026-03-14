//! ClinVar annotation from a local index (rsID → ClinvarSummary).
//! Wire into the report pipeline so VCF-derived variants get ClinVar when the index is available.
//! Set CLINVAR_INDEX_PATH to a JSON file: { "rs123": { "classification": "...", "review_status": "...", "conditions": [], ... }, ... }.

use crate::sequencing_parity;
use crate::variant_input::{ClinvarSummary, VariantInput};
use std::collections::HashMap;
use std::path::Path;

/// Load ClinVar index from JSON. Keys: canonical rsID (e.g. "rs123"); values: ClinvarSummary.
pub fn load_clinvar_index(path: &Path) -> Result<HashMap<String, ClinvarSummary>, Box<dyn std::error::Error + Send + Sync>> {
    let raw = std::fs::read_to_string(path)?;
    let map: HashMap<String, ClinvarSummary> = serde_json::from_str(&raw)?;
    Ok(map)
}

/// Annotate variants with ClinVar from the index. Only fills in variants that have an rsID and no clinvar yet.
pub fn annotate_variants_with_clinvar(
    variants: Vec<VariantInput>,
    index: &HashMap<String, ClinvarSummary>,
) -> Vec<VariantInput> {
    if index.is_empty() {
        return variants;
    }
    variants
        .into_iter()
        .map(|mut v| {
            if v.clinvar.is_none() {
                if let Some(ref rsid) = v.rsid {
                    let key = sequencing_parity::canonical_rsid(rsid);
                    if !key.is_empty() {
                        if let Some(clinvar) = index.get(&key) {
                            v.clinvar = Some(clinvar.clone());
                        }
                    }
                }
            }
            v
        })
        .collect()
}

/// Load index from path if file exists and is valid; return None on missing or error.
pub fn load_clinvar_index_optional(path: &str) -> Option<HashMap<String, ClinvarSummary>> {
    let p = Path::new(path);
    if !p.is_file() {
        return None;
    }
    load_clinvar_index(p).ok()
}
