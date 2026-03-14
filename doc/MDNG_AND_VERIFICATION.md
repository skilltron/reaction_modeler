# MDNG O(1) requirement and conversion verification

## Why MDNG: richer, more complete representation

A central point of the MDNG format is **combining** variant data from multiple sources (VCFs, arrays, FASTQ-derived calls, other MDNGs) into a **single, richer representation**. Merging by variant key (chr, pos, ref, alt) preserves every unique variant and allows metadata (e.g. ClinVar, gene, source) to live in one place. Downstream analysis then runs on this more complete picture rather than on a single VCF or run. The **combine-to-mdng** tool supports this workflow: load multiple files, merge, write one MDNG with all data preserved.

## MDNG must afford O(1) access

The MDNG (multi-dimensional / indexed genome) format used in the FASTQ → MDNG → analysis pipeline **must provide O(1) lookup** for variant access. That means:

- **Lookup by key**: Access to a variant by a unique key (e.g. `(chromosome, position)` or `(chromosome, position, ref_allele, alt_allele)`) must be **constant time**, not a linear scan.
- **Implementation**: Use indexed structures such as:
  - `HashMap<(String, u64), Variant>` or `HashMap<(String, u64, String, String), Variant>` for O(1) by (chr, pos) or full key, or
  - `BTreeMap` for ordered O(log n) if sorted access is needed; for strict O(1) use `HashMap`.
- **Iteration**: Producing a stream or list of all variants for export to VariantInput JSON can be O(n) in the number of variants; the requirement is that **single-variant lookup** is O(1).

This keeps the pipeline fast when the genetic conditions report (or downstream tools) query specific positions or regions.

## Verification: MDNG vs standard conversion

To verify that the MDNG path does not drop data, run **both** conversion paths and compare:

1. **MDNG path**: FASTQ → MDNG (O(1) structure) → export to VariantInput JSON → `variants_mdng.json`
2. **Standard path**: FASTQ → alignment → variant calling → VCF → convert VCF to VariantInput JSON → `variants_standard.json`

**Standard path (VCF → JSON):** Use the `vcf-to-variants` binary on a VCF (or gzipped VCF):

```bash
# From gzipped VCF (e.g. Henry 30x WGS)
cargo run --release -p genetic_conditions --bin vcf-to-variants -- path/to/snp-indel.genome.vcf.gz > variants_standard.json
# Or from stdin (e.g. gzip -dc file.vcf.gz | vcf-to-variants > variants_standard.json)
```

Then run the comparison tool:

```bash
cargo run --release -p genetic_conditions --bin compare-variant-sources -- variants_mdng.json variants_standard.json
```

The tool reports:

- Total variant count from each file
- Unique variant count (by chromosome + position + ref + alt) for each
- Overlap: how many variants appear in both
- Which conversion shows **more data** (higher count)
- Optional: list of keys only in A or only in B (for debugging)

**Interpretation**: If the MDNG path shows **fewer** variants than the standard path, investigate filters or dedup logic in the MDNG pipeline. If MDNG shows **more**, the standard pipeline may be dropping calls (e.g. low QUAL, filters). If counts are close and overlap is high, both paths are consistent.

## Report-level comparison (assess-run)

To compare **reports** (condition findings, cascade scores, survival genes), not just raw variant counts, run the full pipeline on one or two variant JSONs:

```bash
# Single run: full assessment (conditions, cascade, survival)
cargo run --release -p genetic_conditions --bin assess-run -- variants.json

# Two runs: assess both and compare (e.g. MDNG vs VCF)
cargo run --release -p genetic_conditions --bin assess-run -- variants_mdng.json variants_standard.json
```

The tool uses all modules: `check_variants_against_all` (immune, exposure, inflammation, sulfur, rare), `cascade::compute_cascade_from_report`, and `survival::analyze_survival`. For two paths it prints side-by-side counts and which run has more variants, more findings per category, higher cascade composite score, and more survival genes. Use this to assess which conversion path yields the richer report.

## Variants in shards with ClinVar

In the pipeline (reaction_modeler_convert), variants are written to MDNG **in their proper place**: sorted by (chromosome, position) so they are stored in genomic order. For each variant with an rsID, the pipeline looks up **ClinVar** from `Gene_Forager/resources/clinvar_index_by_rsid.json` (and disease/gene hits from the disease analyzer) and stores them in the variant metadata so the shards carry full annotation.

## Where MDNG lives

The MDNG format and genome-collector pipeline live in the **reaction_modeler_convert** workspace (e.g. on Crucial X10: `/Volumes/Crucial X10/reaction_modeler_convert/`), in Gene_Forager and related crates (mdng/vcf/genome-collector). This repo (`reaction_modeler`) holds the genetic conditions report and the **verification tool** that compares two VariantInput JSON files.

## Growing the shard database

The variant shard system must support a large number of variants and keep adding to it. For design and strategies (per-run vs shared DB, partitioning, merge/append, O(1) preservation), see **SHARD_DATABASE_GROWTH.md**.

## Combining multiple files into one MDNG

To load several VCF and/or MDNG files and write a single MDNG with all variants (duplicates merged by chr/pos/ref/alt; first occurrence kept):

```bash
cargo run --release -p shard_db --bin combine-to-mdng -- combined.mdng file1.mdng file2.vcf file3.vcf.gz
```

Inputs: `.mdng` (MDNG JSON) or `.vcf` / `.vcf.gz` (VCF). On macOS/Linux, `.vcf.gz` is decompressed with `gzip -dc` before ingest. Output: one high-value MDNG with all data preserved. All processing is local.

## FASTQ-derived variants

The pipeline’s FASTQ path does not run alignment or variant calling. To get variants from FASTQ: **(1) alignment** (FASTQ → BAM), **(2) variant calling** (BAM → VCF), **(3) merge** that VCF into the pipeline or shard DB. See **FASTQ_TO_VARIANTS.md**.
