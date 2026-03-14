//! One shard = one chromosome. In-memory: HashMap for O(1) lookup + sorted vec for iteration.

use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::record::ShardRecord;

/// Per-chromosome shard: variants in genomic order, O(1) lookup by (pos, ref, alt).
#[derive(Debug, Clone, Default)]
pub struct Shard {
    /// Chromosome (normalized, e.g. "1").
    pub chromosome: String,
    /// Index for O(1) lookup. Key = (position, ref_allele, alt_allele) since chr is fixed in this shard.
    index: HashMap<(u64, String, String), ShardRecord>,
    /// Sorted by position for iteration and for writing to disk in genomic order.
    sorted: Vec<ShardRecord>,
}

#[derive(Serialize, Deserialize)]
struct ShardFile {
    chromosome: String,
    variants: Vec<ShardRecord>,
}

impl Shard {
    pub fn new(chromosome: String) -> Self {
        Self {
            chromosome,
            index: HashMap::new(),
            sorted: Vec::new(),
        }
    }

    /// O(1) lookup by (position, ref, alt). Returns reference to stored record.
    pub fn get(&self, position: u64, ref_allele: &str, alt_allele: &str) -> Option<&ShardRecord> {
        self.index.get(&(
            position,
            ref_allele.to_string(),
            alt_allele.to_string(),
        ))
    }

    /// Merge one record. If key exists, existing is kept (first write wins). Returns true if added.
    pub fn merge_one(&mut self, record: ShardRecord) -> bool {
        let (pos, ref_a, alt_a) = (
            record.position,
            record.ref_allele.clone(),
            record.alt_alleles.clone(),
        );
        if self.index.contains_key(&(pos, ref_a.clone(), alt_a.clone())) {
            return false;
        }
        self.index.insert((pos, ref_a, alt_a), record.clone());
        self.sorted.push(record);
        self.sorted.sort_by_key(|r| (r.position, r.ref_allele.clone(), r.alt_alleles.clone()));
        true
    }

    /// Merge many records; dedup by key. Returns number of newly added.
    pub fn merge(&mut self, records: impl IntoIterator<Item = ShardRecord>) -> usize {
        let mut n = 0;
        for r in records {
            if self.merge_one(r) {
                n += 1;
            }
        }
        n
    }

    /// Iterate all variants in genomic order.
    pub fn iter(&self) -> impl Iterator<Item = &ShardRecord> {
        self.sorted.iter()
    }

    pub fn len(&self) -> usize {
        self.sorted.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sorted.is_empty()
    }

    /// Load shard from a JSON file. File format: { "chromosome": "1", "variants": [ ... ] }.
    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let raw = std::fs::read_to_string(path)?;
        let file: ShardFile = serde_json::from_str(&raw)?;
        let mut shard = Shard::new(file.chromosome);
        shard.merge(file.variants);
        Ok(shard)
    }

    /// Save shard to JSON. Variants written in genomic order.
    pub fn save(&self, path: &Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let file = ShardFile {
            chromosome: self.chromosome.clone(),
            variants: self.sorted.clone(),
        };
        let raw = serde_json::to_string_pretty(&file)?;
        std::fs::write(path, raw)?;
        Ok(())
    }
}
