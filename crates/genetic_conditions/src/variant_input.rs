//! Shared variant input and region type for all condition checks.

use serde::{Deserialize, Serialize};

/// Region type for variant annotation when available from pipeline.
/// Used to show coding vs non-coding changes; report non-coding when no coding variants present.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegionType {
    Coding,
    Exon,
    Intron,
    Utr5,
    Utr3,
    Promoter,
    Regulatory,
    NonCoding,
    Unknown,
}

impl RegionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            RegionType::Coding => "coding",
            RegionType::Exon => "exon",
            RegionType::Intron => "intron",
            RegionType::Utr5 => "5'_UTR",
            RegionType::Utr3 => "3'_UTR",
            RegionType::Promoter => "promoter",
            RegionType::Regulatory => "regulatory",
            RegionType::NonCoding => "non_coding",
            RegionType::Unknown => "unknown",
        }
    }
}

/// Minimal variant input (e.g. from VCF pipeline). Used by all submodules.
/// ref_allele = normal/reference; alt_allele = change. region_type indicates coding vs non-coding when available.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantInput {
    pub chromosome: String,
    pub position: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gene: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsid: Option<String>,
    /// Reference (normal) allele.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_allele: Option<String>,
    /// Alternate (change) allele.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_allele: Option<String>,
    /// Coding vs non-coding; when present, reports show normal/changes and region. If only non-coding variants present for a gene, they are still reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_type: Option<RegionType>,
}
