//! Copy number assay input for the report.
//!
//! Copy number is detected by dedicated assays (e.g. MLPA, qPCR, array CGH), not by SNV/indel
//! variant calls. When provided, the report can interpret conditions that depend on copy number,
//! such as Hereditary alpha-tryptasemia (HαT), which is defined by TPSAB1 copy number gain.

use serde::{Deserialize, Serialize};

/// One gene's copy number result from an assay.
/// Normal diploid = 2; gain (e.g. duplication) = 3 or more; loss = 0 or 1.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyNumberResult {
    /// Gene symbol (e.g. TPSAB1).
    pub gene: String,
    /// Reported copy number (integer).
    pub copy_number: u32,
    /// Optional source/lab or assay name for display.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// Gene symbol used for HαT (Hereditary alpha-tryptasemia).
pub const TPSAB1: &str = "TPSAB1";

/// Returns TPSAB1 copy number from the assay results if present.
#[inline]
pub fn tpsab1_copy_number(results: &[CopyNumberResult]) -> Option<u32> {
    results
        .iter()
        .find(|r| r.gene.eq_ignore_ascii_case(TPSAB1))
        .map(|r| r.copy_number)
}

/// Whether TPSAB1 copy number indicates gain (≥3), when assay data is present.
#[inline]
pub fn tpsab1_gain_detected(results: &[CopyNumberResult]) -> bool {
    tpsab1_copy_number(results).map(|n| n >= 3).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tpsab1_copy_number_found() {
        let r = vec![
            CopyNumberResult {
                gene: "OTHER".to_string(),
                copy_number: 2,
                source: None,
            },
            CopyNumberResult {
                gene: "TPSAB1".to_string(),
                copy_number: 3,
                source: Some("Lab X".to_string()),
            },
        ];
        assert_eq!(tpsab1_copy_number(&r), Some(3));
        assert!(tpsab1_gain_detected(&r));
    }

    #[test]
    fn tpsab1_no_gain() {
        let r = vec![CopyNumberResult {
            gene: "TPSAB1".to_string(),
            copy_number: 2,
            source: None,
        }];
        assert_eq!(tpsab1_copy_number(&r), Some(2));
        assert!(!tpsab1_gain_detected(&r));
    }

    #[test]
    fn tpsab1_missing() {
        let r: Vec<CopyNumberResult> = vec![];
        assert_eq!(tpsab1_copy_number(&r), None);
        assert!(!tpsab1_gain_detected(&r));
    }
}
