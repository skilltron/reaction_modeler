# Henry’s run: lab VCF vs pipeline MDNG (and FASTQ)

## What was run

- **Lab VCF:** `HenryHaskelljr-SQ4VM673-30x-WGS-Sequencing_com-01-31-25.snp-indel.genome.vcf.gz` (Sequencing.com 30× WGS, BGZF).
- **Pipeline MDNG:** `Henry_Haskell.mdng` on X10, from pipeline run with **VCF + 23andMe** (no FASTQ). Config: `config_henry_only.json` with `fastq_files: []`.

Conversion:

- Lab VCF → `gzip -dc ... | vcf-to-variants` → `target/variants_from_vcf.json`
- MDNG → `mdng-to-variants` → `target/variants_mdng_clean.json` (use `2>/dev/null` so JSON is not mixed with cargo stderr)

## Variant-level comparison

| Source | Unique variants (chr+pos+ref+alt) |
|--------|-----------------------------------|
| Lab VCF (A) | 4,663,888 |
| Pipeline MDNG (B) | 5,304,013 |
| **Overlap** | 4,618,493 |
| **Only in VCF (A)** | 45,395 |
| **Only in MDNG (B)** | 685,520 |

- **B has more** overall: MDNG = pipeline VCF + 23andMe merged and deduped, so the extra ~685k “only in B” are largely 23andMe array SNPs not in the WGS VCF.
- **45,395 only in VCF:** variants in the lab WGS that are **not** in the pipeline MDNG. So the pipeline “missed” these relative to the lab VCF.

## Why did the pipeline “miss” those 45,395?

1. **Different VCF input**  
   The pipeline used the **X10 copy** of Henry’s VCF (`henry_snp_indel.vcf.gz` in `Gene_Forager/test_genomes/Henry_Haskell/`), not the lab file in Downloads. If the X10 copy was older, truncated, or different, the pipeline would have fewer (or different) variants than the lab VCF.

2. **Parsing / format**  
   Pipeline uses `vcf_parser` (reaction_modeler_convert); report uses `vcf-to-variants` (reaction_modeler). Different parsers or handling of FILTER/ALT/REF could drop some rows (e.g. ref/alt “.” or multi-allelic handling).

3. **Dedup**  
   Pipeline deduplicates by (chr, pos, ref, alt). If the same key appeared multiple times with different INFO/format, only one is kept. That can reduce count vs the raw lab VCF.

4. **FASTQ did not add variants**  
   The pipeline’s FASTQ path was **not** used for Henry (`fastq_files: []`). Even if it were, the current FASTQ processor only returns **(reads, variant count)**; it does **not** return variant records or merge them into the MDNG. So no variants are “from FASTQ” in this run. To get real FASTQ-derived variants you’d need alignment + variant calling (e.g. BAM → VCF) and then feed that VCF into the pipeline (or merge that VCF’s variant list into the pipeline output).

## Report-level (assess-run)

Both variant JSONs gave **0 findings** in the condition modules (immune, inflammation, exposure, sulfur, rare, disorders) and **0** cascade composite / survival genes in the assessment. So for this run the main difference is **variant count and overlap**; condition logic would need gene/rsID annotation (or different input) to show non-zero findings.

## Fixes applied for this run

- **BGZF:** Lab VCF is BGZF. `vcf-to-variants` with plain `GzDecoder` only reads the first block and gets no data. `run-report-and-open.sh` now uses `gzip -dc "$VCF"` when the path ends in `.gz` so the full VCF is passed to `vcf-to-variants`.
- **Clean extraction JSON:** When generating extraction from MDNG, run `mdng-to-variants ... 2>/dev/null > path.json` so cargo stderr does not end up at the start of the JSON file.

## Summary

- **Differences:** Lab VCF has 45,395 variants not in the pipeline MDNG; pipeline MDNG has 685,520 not in the lab VCF (mostly 23andMe).
- **Why “missed” in pipeline:** Different/older VCF on X10, possible parser/dedup differences, and **FASTQ not used and not producing variant records**.
- **Next steps (if you want FASTQ to contribute):** Implement alignment + variant calling from Henry’s FASTQ, then merge that VCF (or its variant list) into the pipeline; or extend the FASTQ processor to output variant records and merge them into the pipeline’s `all_variants`.
