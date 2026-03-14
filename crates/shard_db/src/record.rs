//! Single variant record stored in a shard. MDNG-compatible (chromosome, position, reference_allele, alternate_alleles, variant_id, metadata, genotype).

use serde::{Deserialize, Serialize};

use crate::key::{normalize_chromosome, VariantKey};

/// One variant as stored in a shard. Matches MDNG JSON shape so export produces valid .mdng.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardRecord {
    pub chromosome: String,
    pub position: u64,
    #[serde(rename = "reference_allele")]
    pub ref_allele: String,
    /// Comma-separated if multi-ALT; we store one record per ALT in the index but can export as comma-separated for MDNG.
    #[serde(rename = "alternate_alleles")]
    pub alt_alleles: String,
    #[serde(rename = "variant_id", skip_serializing_if = "Option::is_none")]
    pub rsid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genotype: Option<String>,
}

impl ShardRecord {
    pub fn key(&self) -> VariantKey {
        VariantKey::new(
            &self.chromosome,
            self.position,
            &self.ref_allele,
            self.alt_alleles.split(',').next().unwrap_or(&self.alt_alleles).trim(),
        )
    }

    /// Chromosome for partitioning (normalized).
    pub fn chr_normalized(&self) -> String {
        normalize_chromosome(&self.chromosome)
    }
}
