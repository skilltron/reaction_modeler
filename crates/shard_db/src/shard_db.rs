//! Multi-shard database: one directory, one shard file per chromosome. Add variants, lookup O(1), export to MDNG.

use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};

use crate::record::ShardRecord;
use crate::shard::Shard;

const SHARD_PREFIX: &str = "chr";
const SHARD_SUFFIX: &str = ".json";

/// Database of variant shards partitioned by chromosome. Add variants (merge by key), O(1) lookup, export to single MDNG.
pub struct ShardDb {
    root: PathBuf,
    /// Loaded shards by normalized chromosome (e.g. "1", "2", "X"). Lazy-loaded on first access.
    shards: BTreeMap<String, Shard>,
}

impl ShardDb {
    /// Open or create a shard database at the given directory.
    pub fn open(root: impl AsRef<Path>) -> Self {
        let root = root.as_ref().to_path_buf();
        if root.exists() && !root.is_dir() {
            panic!("ShardDb root must be a directory: {}", root.display());
        }
        Self {
            root,
            shards: BTreeMap::new(),
        }
    }

    /// Ensure root directory exists.
    pub fn ensure_root(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.root)
    }

    fn shard_path(&self, chr: &str) -> PathBuf {
        self.root.join(format!("{}{}{}", SHARD_PREFIX, chr, SHARD_SUFFIX))
    }

    /// Load a shard from disk if not already in memory. Returns reference to the shard.
    fn get_or_load_shard(&mut self, chr: &str) -> Result<&mut Shard, Box<dyn std::error::Error + Send + Sync>> {
        if !self.shards.contains_key(chr) {
            let path = self.shard_path(chr);
            let shard = if path.exists() {
                Shard::load(&path)?
            } else {
                Shard::new(chr.to_string())
            };
            self.shards.insert(chr.to_string(), shard);
        }
        Ok(self.shards.get_mut(chr).unwrap())
    }

    /// Add variants to the database. Merges by (chr, pos, ref, alt). Records with multiple ALTs (comma-separated) are expanded to one record per ALT.
    /// Returns total number of variants newly added across all chromosomes.
    pub fn add_variants(&mut self, records: impl IntoIterator<Item = ShardRecord>) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        self.ensure_root()?;
        let mut total_added = 0;
        for rec in records {
            let chr = rec.chr_normalized();
            if chr.is_empty() {
                continue;
            }
            let alts: Vec<&str> = rec.alt_alleles.split(',').map(|s| s.trim()).filter(|s| !s.is_empty() && *s != ".").collect();
            if alts.is_empty() {
                continue;
            }
            let shard = self.get_or_load_shard(&chr)?;
            for alt in alts {
                let single = ShardRecord {
                    chromosome: rec.chromosome.clone(),
                    position: rec.position,
                    ref_allele: rec.ref_allele.clone(),
                    alt_alleles: alt.to_string(),
                    rsid: rec.rsid.clone(),
                    metadata: rec.metadata.clone(),
                    genotype: rec.genotype.clone(),
                };
                if shard.merge_one(single) {
                    total_added += 1;
                }
            }
        }
        Ok(total_added)
    }

    /// O(1) lookup: get one variant by key. Loads the chromosome shard if needed.
    pub fn get(
        &mut self,
        chromosome: &str,
        position: u64,
        ref_allele: &str,
        alt_allele: &str,
    ) -> Result<Option<ShardRecord>, Box<dyn std::error::Error + Send + Sync>> {
        let chr = crate::key::normalize_chromosome(chromosome);
        if chr.is_empty() {
            return Ok(None);
        }
        let shard = self.get_or_load_shard(&chr)?;
        Ok(shard.get(position, ref_allele, alt_allele).cloned())
    }

    /// Collect all variants across all chromosomes in genomic order (chr 1, 2, ... 22, X, Y, MT).
    pub fn collect_all(&mut self) -> Result<Vec<ShardRecord>, Box<dyn std::error::Error + Send + Sync>> {
        let chrom_order = self.chromosome_order();
        for chr in &chrom_order {
            let _ = self.get_or_load_shard(chr)?;
        }
        let mut out = Vec::new();
        for chr in chrom_order {
            if let Some(shard) = self.shards.get(&chr) {
                out.extend(shard.iter().cloned());
            }
        }
        Ok(out)
    }

    fn chromosome_order(&self) -> Vec<String> {
        let mut chrs: Vec<String> = self.shards.keys().cloned().collect();
        chrs.sort_by(|a, b| {
            let ord = |c: &str| -> u32 {
                if c == "X" { 100 } else if c == "Y" { 101 } else if c == "MT" || c == "M" { 102 } else { c.parse().unwrap_or(0) }
            };
            ord(a).cmp(&ord(b))
        });
        chrs
    }

    /// Save all in-memory shards to disk.
    pub fn save_all(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.ensure_root()?;
        let chrs: Vec<String> = self.shards.keys().cloned().collect();
        for chr in &chrs {
            let path = self.shard_path(chr);
            if let Some(shard) = self.shards.get_mut(chr) {
                if !shard.is_empty() {
                    shard.save(&path)?;
                }
            }
        }
        Ok(())
    }

    /// Write a single combined MDNG file (same format as pipeline .mdng) so mdng-to-variants can read it.
    pub fn write_mdng(&mut self, path: impl AsRef<Path>) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        self.ensure_root()?;
        let chrom_order: Vec<String> = {
            let mut c: Vec<String> = self.shards.keys().cloned().collect();
            c.sort_by(|a, b| {
                let ord = |c: &str| -> u32 {
                    if c == "X" { 100 } else if c == "Y" { 101 } else if c == "MT" || c == "M" { 102 } else { c.parse().unwrap_or(0) }
                };
                ord(a).cmp(&ord(b))
            });
            c
        };
        let mut all_variants: Vec<&ShardRecord> = Vec::new();
        for chr in &chrom_order {
            if let Some(shard) = self.shards.get(chr) {
                all_variants.extend(shard.iter());
            }
        }
        let mdng = serde_json::json!({
            "variants": all_variants.iter().map(|r| {
                serde_json::json!({
                    "chromosome": r.chromosome,
                    "position": r.position,
                    "reference_allele": r.ref_allele,
                    "alternate_alleles": r.alt_alleles,
                    "variant_id": r.rsid,
                    "metadata": r.metadata,
                    "genotype": r.genotype
                })
            }).collect::<Vec<_>>()
        });
        let raw = serde_json::to_string_pretty(&mdng)?;
        std::fs::write(path.as_ref(), raw)?;
        Ok(all_variants.len())
    }

    /// Ingest variants from an existing MDNG file into the shard database (merge by key).
    pub fn ingest_mdng(&mut self, path: impl AsRef<Path>) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        let raw = std::fs::read_to_string(path.as_ref())?;
        let data: serde_json::Value = serde_json::from_str(&raw)?;
        let arr = data.get("variants").and_then(|v| v.as_array()).ok_or("MDNG must have 'variants' array")?;
        let mut records = Vec::with_capacity(arr.len());
        for v in arr {
            let chrom = v.get("chromosome").and_then(|c| c.as_str()).unwrap_or("").to_string();
            let pos = v.get("position").and_then(|p| p.as_u64()).unwrap_or(0);
            let ref_a = v.get("reference_allele").and_then(|r| r.as_str()).unwrap_or("").to_string();
            let alts = v.get("alternate_alleles").and_then(|a| a.as_str()).unwrap_or("");
            let rsid = v.get("variant_id").and_then(|id| id.as_str()).filter(|s| !s.is_empty() && *s != ".").map(String::from);
            let metadata = v.get("metadata").cloned();
            let genotype = v.get("genotype").and_then(|g| g.as_str()).map(String::from);
            records.push(ShardRecord {
                chromosome: chrom,
                position: pos,
                ref_allele: ref_a,
                alt_alleles: alts.to_string(),
                rsid,
                metadata,
                genotype,
            });
        }
        self.add_variants(records)
    }

    /// Ingest variants from a VCF (plain or .gz). One record per ALT; merges by key.
    /// Note: .gz must be standard gzip; for BGZF use `gzip -dc in.vcf.gz > in.vcf` then ingest the plain file.
    pub fn ingest_vcf(&mut self, path: impl AsRef<Path>) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        let path = path.as_ref();
        let reader: Box<dyn Read> = if path.to_string_lossy().ends_with(".gz") {
            Box::new(flate2::read::GzDecoder::new(std::fs::File::open(path)?))
        } else {
            Box::new(std::fs::File::open(path)?)
        };
        let mut records = Vec::new();
        let buf = BufReader::new(reader);
        for line in buf.lines() {
            let line = line?;
            if line.starts_with('#') {
                continue;
            }
            let cols: Vec<&str> = line.split('\t').collect();
            if cols.len() < 5 {
                continue;
            }
            let chrom = cols[0].to_string();
            let pos: u64 = cols[1].parse().unwrap_or(0);
            let id = cols[2];
            let rsid = if id.is_empty() || id == "." {
                None
            } else {
                Some(id.to_string())
            };
            let ref_a = cols[3].to_string();
            let alts = cols[4];
            for alt in alts.split(',') {
                let alt = alt.trim();
                if alt.is_empty() || alt == "." {
                    continue;
                }
                records.push(ShardRecord {
                    chromosome: chrom.clone(),
                    position: pos,
                    ref_allele: ref_a.clone(),
                    alt_alleles: alt.to_string(),
                    rsid: rsid.clone(),
                    metadata: None,
                    genotype: None,
                });
            }
        }
        self.add_variants(records)
    }

    /// Total variant count across all shards. If no shards are loaded, discovers and loads all chr*.json in root.
    pub fn total_count(&mut self) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        if self.shards.is_empty() && self.root.exists() {
            let entries = std::fs::read_dir(&self.root)?;
            for e in entries {
                let e = e?;
                let name = e.file_name().to_string_lossy().to_string();
                if name.starts_with(SHARD_PREFIX) && name.ends_with(SHARD_SUFFIX) {
                    let chr = name[SHARD_PREFIX.len()..name.len() - SHARD_SUFFIX.len()].to_string();
                    let _ = self.get_or_load_shard(&chr)?;
                }
            }
        }
        Ok(self.shards.values().map(|s| s.len()).sum())
    }
}
