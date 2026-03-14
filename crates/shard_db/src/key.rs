//! Canonical key for dedup and O(1) lookup: (chromosome, position, ref_allele, alt_allele).

use std::fmt;

/// Normalize chromosome so "1" and "chr1" map to the same key.
#[inline]
pub fn normalize_chromosome(s: &str) -> String {
    let s = s.trim();
    if s.is_empty() {
        return s.to_string();
    }
    if s.starts_with("chr") || s.starts_with("CHR") {
        return s[3..].trim().to_string();
    }
    s.to_string()
}

/// Key for a single variant: (chromosome normalized, position, ref, alt). Used for dedup and lookup.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct VariantKey {
    pub chromosome: String,
    pub position: u64,
    pub ref_allele: String,
    pub alt_allele: String,
}

impl VariantKey {
    pub fn new(chromosome: &str, position: u64, ref_allele: &str, alt_allele: &str) -> Self {
        Self {
            chromosome: normalize_chromosome(chromosome),
            position,
            ref_allele: ref_allele.to_string(),
            alt_allele: alt_allele.to_string(),
        }
    }
}

impl fmt::Debug for VariantKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{} {}>{}",
            self.chromosome, self.position, self.ref_allele, self.alt_allele
        )
    }
}
