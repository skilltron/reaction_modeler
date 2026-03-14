# Ready for live runs

You can run live now. Use the flows below.

---

## 1. Report from lab VCF only (no pipeline, no comparison)

**Where:** reaction_modeler (this repo)  
**Needs:** Your lab VCF (e.g. Henry’s in `~/Downloads/...vcf.gz`)

```bash
cd /Users/henryhaskell/Projects/reaction_modeler
./scripts/run-report-and-open.sh ~/Downloads/HenryHaskelljr-SQ4VM673-30x-WGS-Sequencing_com-01-31-25.snp-indel.genome.vcf.gz
```

Report opens as `genetic_report.html`. Timings printed for VCF→variants and report build.

---

## 2. Pipeline (VCF + 23andMe) on X10 → report later

**Where:** reaction_modeler_convert on Crucial X10  
**Needs:** X10 mounted; VCFs and 23andMe paths in config (e.g. `config_no_fastq.json`)

```bash
cd "/Volumes/Crucial X10/reaction_modeler_convert"
cargo run -p genome-pipeline-test --release -- --config Gene_Forager/config_no_fastq.json
```

Produces per-person: `.mdng`, HTML report, JSON summary. Variants in MDNG are sorted by (chr, position) and ClinVar is looked up when `Gene_Forager/resources/clinvar_index_by_rsid.json` exists (optional; pipeline runs without it).

---

## 3. Report from pipeline MDNG (your extraction) and compare to lab VCF

**Needs:**  
- A `.mdng` from step 2 (or from a FASTQ run).  
- Lab VCF for the same sample.

```bash
# In reaction_modeler: turn .mdng into VariantInput JSON
cd /Users/henryhaskell/Projects/reaction_modeler
cargo run --release -p genetic_conditions --bin mdng-to-variants -- "/Volumes/Crucial X10/reaction_modeler_convert/Gene_Forager/test_genomes/Henry_Haskell/Henry_Haskell.mdng" > variants_mdng.json

# Report from VCF and compare VCF vs extraction (timings printed)
./scripts/run-report-and-open.sh ~/Downloads/HenryHaskelljr-SQ4VM673-30x-WGS-Sequencing_com-01-31-25.snp-indel.genome.vcf.gz variants_mdng.json
```

---

## 4. Pipeline with FASTQ (when you have FASTQ paths)

Create a config (e.g. `Gene_Forager/config_henry_fastq.json`) with `fastq_files` and `output_dir`. Run the pipeline; then use the resulting `.mdng` in step 3 as your extraction.

---

## Quick checklist

| Item | Status |
|------|--------|
| reaction_modeler: genetic-report-html, vcf-to-variants, mdng-to-variants, compare-variant-sources, assess-run | Built (release) |
| run-report-and-open.sh: VCF → report, optional comparison, timings | Ready |
| reaction_modeler_convert: genome-pipeline-test (pipeline-test) | Built (release) on X10 |
| Pipeline: variants sorted by (chr, pos); ClinVar lookup for rsIDs | In place |
| ClinVar index (optional) | `Gene_Forager/resources/clinvar_index_by_rsid.json` — add for annotation |

You’re ready for live runs. Start with **1** for a quick report from the lab VCF; use **2** and **3** when you want pipeline output and VCF-vs-extraction comparison.
