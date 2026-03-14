# Shard database of variants: scaling and adding data

The variant shard system (MDNG in the pipeline) holds a large number of variants and must support **keep adding** to it as new runs, panels, or reference data are added. This doc covers how to grow the shard database without losing O(1) lookup or breaking the pipeline.

---

## Where the shard system lives

- **Code:** `reaction_modeler_convert` (e.g. on Crucial X10: `/Volumes/Crucial X10/reaction_modeler_convert/`), in Gene_Forager and related crates (mdng, genome-pipeline-test).
- **Output:** Per-sample `.mdng` files (e.g. `Gene_Forager/test_genomes/Henry_Haskell/Henry_Haskell.mdng`) contain a variant store written in genomic order; the pipeline writes them from in-memory deduped variants.
- **Consumption:** `reaction_modeler` uses `mdng-to-variants` to read an MDNG and emit VariantInput JSON for the genetic conditions report and comparison.

See **MDNG_AND_VERIFICATION.md** for O(1) requirement and verification.

---

## Goals when adding to the shard database

1. **Richer, more complete representation** — MDNG is designed to combine info from multiple sources (VCF, arrays, FASTQ, other MDNGs) into one representation so analysis sees the full picture.
2. **Preserve O(1) lookup** — New variants must be stored in structures that allow constant-time access by (chromosome, position) or full key (chr, pos, ref, alt).
3. **Support a large number of variants** — Design for millions of variants per sample and/or a shared reference shard set that grows over time.
4. **Additive growth** — Prefer appending or merging new data into shards rather than full rewrites where possible.
5. **Genomic order** — Variants in shards should remain sorted by (chromosome, position) for consistent export and future range queries.

---

## Strategies for adding variants to the shard system

### 1. Per-run MDNG (current pattern)

Each pipeline run produces **one .mdng per person** with that run’s variants (VCF + 23andMe + optional FASTQ). Adding more variants means:

- **More runs:** New samples or new runs add new .mdng files; the “database” grows by adding more files, not by growing one file.
- **Richer inputs per run:** Add more VCFs, more arrays, or FASTQ-derived variants to the same run so that run’s .mdng contains more variants.

No change to the shard format is required; the pipeline already merges all sources, dedups, and writes one MDNG per person.

### 2. Shared / reference shard database (future)

If you want a **single growing database** of variants (e.g. a reference panel or multi-sample store):

- **Partition by chromosome (and optionally by position range):** e.g. shard `chr1.0-50M.json`, `chr1.50M-100M.json`, … so each shard stays bounded in size and new variants are written into the correct shard.
- **Append or merge within a shard:** For each partition, maintain an O(1) structure (e.g. `HashMap<(u64, String, String), Variant>` by (pos, ref, alt) within that chromosome range). New variants either:
  - **Append** to a log and periodically merge into the index, or
  - **Merge** in place: load shard, add new variants, dedup by key, write back.
- **Deduplication key:** Use (chromosome, position, ref_allele, alt_allele) so the same variant from multiple sources is stored once; when “adding” new data, merge on this key and keep or combine metadata (e.g. ClinVar, gene).

### 3. Adding new sources into the pipeline

To **keep adding** variant sources without changing the shard format:

- **New VCF/array inputs:** Add another entry in the pipeline config (e.g. another `vcf_files` or `twentythreeandme_files` path); the pipeline already extends `all_variants` and deduplicates before writing the MDNG.
- **FASTQ-derived variants:** Once the FASTQ path returns real variant records (not just counts), merge them into `all_variants` before dedup so they are written into the same MDNG and thus into the same shard output.
- **Reference or panel VCF:** If you have a “reference” or “panel” VCF to merge into every run, add it as an additional VCF input or a dedicated merge step before writing the MDNG so the shard database (per-run or shared) includes those variants.

---

## Implementation checklist for growth

When implementing or extending the shard system in **reaction_modeler_convert**:

- [ ] **Key format:** Use a single canonical key for dedup and lookup: (chromosome, position, ref_allele, alt_allele). Normalize chromosome (e.g. "1" vs "chr1") and alleles so the same variant always maps to the same key.
- [ ] **Write path:** When writing MDNG, sort variants by (chr, position) before writing so shards stay in genomic order; preserve O(1) lookup in the in-memory structure used at read time (or in the on-disk format if it is indexed).
- [ ] **Merge path:** For “add variants to existing shard”, load existing shard, merge new variants by key (keep or update metadata), re-sort if needed, write back. Avoid full scan for every lookup.
- [ ] **Partitioning (optional):** If a single MDNG or single shard file gets too large, split by chromosome (and optionally by position windows); have a small index or manifest that maps (chr, pos) to shard file so readers can open the right shard for O(1) or O(log n) access per variant.
- [ ] **ClinVar / metadata:** When adding variants, look up ClinVar (and gene/disease) for rsIDs and store in variant metadata so the shard database stays annotated as it grows.

---

## Summary

- **Current:** Each run writes one MDNG per person; “adding” = more runs or more inputs per run. Shard content is whatever the pipeline writes for that run.
- **Scaling:** For a single large “shard database” that keeps growing, partition by chr (and optionally position), use a fixed key (chr, pos, ref, alt), and support merge/append into those partitions while keeping O(1) lookup and genomic order.
- **Code location:** Shard/MDNG read and write logic lives in **reaction_modeler_convert** (Gene_Forager); **reaction_modeler** only consumes MDNG via `mdng-to-variants`. Changes to how variants are stored and added happen in the convert repo.
