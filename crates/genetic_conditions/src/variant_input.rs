//! Shared variant input and region type for all condition checks.

use serde::{Deserialize, Serialize};

/// ClinVar summary for display (from pipeline/MDNG or lookup). Informational only.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinvarSummary {
    #[serde(default)]
    pub classification: String,
    #[serde(default)]
    pub review_status: String,
    #[serde(default)]
    pub conditions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accession: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated: Option<String>,
}

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
    /// Genotype when available (e.g. 0/1, 1/1). Used for star allele inference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genotype: Option<String>,
    /// ClinVar annotation when available (from MDNG or lookup). Shown in ClinVar report section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clinvar: Option<ClinvarSummary>,
    /// Confidence when available from pipeline (e.g. "High", "Medium", "Low", "Constitutive").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<String>,
}

/// True if ClinVar classification is Pathogenic or Likely pathogenic (case-insensitive).
/// Used so only pathogenic/likely pathogenic variants count as "findings" in each report section.
pub fn is_pathogenic_or_likely_pathogenic(v: &VariantInput) -> bool {
    let c = match &v.clinvar {
        Some(cv) => cv.classification.trim().to_lowercase(),
        None => return false,
    };
    if c.is_empty() {
        return false;
    }
    for part in c.split('/').map(|s| s.trim()) {
        if part == "pathogenic" || part == "likely pathogenic" {
            return true;
        }
    }
    false
}

impl VariantInput {
    /// Key for deduplicating findings: same chr:pos:ref:alt = one finding per condition/section.
    pub fn dedup_key(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.chromosome,
            self.position,
            self.ref_allele.as_deref().unwrap_or(""),
            self.alt_allele.as_deref().unwrap_or("")
        )
    }
}
