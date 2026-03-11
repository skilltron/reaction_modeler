# MDNG O(1) requirement and conversion verification

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

## Where MDNG lives

The MDNG format and genome-collector pipeline live in the **reaction_modeler_convert** workspace (e.g. on Crucial X10: `/Volumes/Crucial X10/reaction_modeler_convert/`), in Gene_Forager and related crates (mdng/vcf/genome-collector). This repo (`reaction_modeler`) holds the genetic conditions report and the **verification tool** that compares two VariantInput JSON files.
