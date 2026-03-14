# FASTQ → variants: alignment + variant calling

To get **FASTQ-derived variants** into the pipeline (and into the MDNG / shard DB), you need three steps.

---

## 1. Alignment

Map FASTQ reads to a reference genome → BAM/SAM.

- **Tools:** BWA-MEM, bowtie2, or similar.
- **Input:** Paired FASTQ (e.g. `sample.1.fq.gz`, `sample.2.fq.gz`).
- **Reference:** Same as your pipeline (e.g. GRCh38); must match the reference used for the lab VCF if you want to compare.
- **Output:** Aligned BAM (sorted, indexed for the caller).

Example (conceptual):

```bash
bwa mem -t 8 ref.fa sample.1.fq.gz sample.2.fq.gz | samtools sort -o sample.bam -
samtools index sample.bam
```

---

## 2. Variant calling

Run a caller on the BAM → VCF (or equivalent).

- **Tools:** bcftools mpileup + call, GATK HaplotypeCaller, or similar.
- **Input:** BAM from step 1 (+ reference, optional known sites).
- **Output:** VCF (e.g. `sample.vcf.gz`).

Example (conceptual):

```bash
bcftools mpileup -Ou -f ref.fa sample.bam | bcftools call -mv -Oz -o sample.vcf.gz
bcftools index sample.vcf.gz
```

---

## 3. Merge into the pipeline

Feed the resulting VCF into the same pipeline that builds the MDNG, or into the shard DB.

**Option A – Pipeline (reaction_modeler_convert)**  
- Add the called VCF as an extra input in the pipeline config (e.g. another `vcf_files` entry for that sample).
- The pipeline merges all VCFs, deduplicates, and writes the MDNG. FASTQ-derived variants then appear in the same MDNG as the lab VCF / 23andMe data.

**Option B – Shard DB (reaction_modeler)**  
- Convert the VCF to variants (e.g. `vcf-to-variants` or a VCF→ShardRecord step).
- Merge those variants into the shard DB with `ShardDb::add_variants` (or ingest a single-sample MDNG built from that VCF).

**Option C – Report comparison**  
- Run `vcf-to-variants` on the called VCF → `variants_fastq_called.json`.
- Compare to lab VCF variants with `compare-variant-sources` and `assess-run` to see overlap and differences.

---

## Summary

| Step | Input | Output |
|------|--------|--------|
| 1. Alignment | FASTQ + reference | BAM |
| 2. Variant calling | BAM + reference | VCF |
| 3. Merge | VCF | Pipeline MDNG / shard DB / report comparison |

The current FASTQ path in the pipeline does **not** run alignment or variant calling; it only reports read and variant counts. To get real FASTQ-derived variants, run steps 1–2 (externally or via scripts), then use step 3 to merge into the pipeline or shard DB.

---

## Concrete usage in this repo

### Script only (align + call → VCF)

From the repo root, with `bwa`, `samtools`, and `bcftools` on `PATH`:

```bash
./scripts/fastq-to-vcf.sh <reference.fa> <sample_id> <out_dir> <fastq1.fq.gz> [fastq2.fq.gz]
```

- **reference.fa** – Reference genome (e.g. GRCh38); must be BWA-indexed (`bwa index reference.fa`).
- **sample_id** – Name for outputs (e.g. `Henry_Haskell`).
- **out_dir** – Directory for BAM and VCF (created if missing).
- **fastq1** – First pair or single-end FASTQ.
- **fastq2** – Optional second pair.

Outputs: `out_dir/<sample_id>.bam`, `out_dir/<sample_id>.bam.bai`, `out_dir/<sample_id>.vcf.gz`, `out_dir/<sample_id>.vcf.gz.csi`.

Optional: `THREADS=16 ./scripts/fastq-to-vcf.sh ...` (default 8).

### One-shot: FASTQ → VCF and optionally into shard DB / MDNG

The **fastq-to-variants** binary runs the script above, then optionally ingests the VCF into the shard DB and/or writes a single MDNG file:

```bash
cargo run --release -p genetic_conditions --bin fastq-to-variants -- \
  <reference.fa> <sample_id> <out_dir> <fastq1> [fastq2] \
  [--into-shard-db <dir>] [--write-mdng <path.mdng>]
```

- Without `--into-shard-db`: only runs the script and prints the VCF path.
- With `--into-shard-db <dir>`: after calling, decompresses the VCF (BGZF) and ingests into the shard DB at `<dir>`.
- With `--write-mdng <path.mdng>`: requires `--into-shard-db`; after ingest, writes one combined MDNG file to `<path.mdng>` (suitable for `mdng-to-variants` or pipeline comparison).

Example (ingest and write MDNG):

```bash
cargo run --release -p genetic_conditions --bin fastq-to-variants -- \
  /path/to/GRCh38.fa my_sample ./fastq_out my_1.fq.gz my_2.fq.gz \
  --into-shard-db ./shard_fastq --write-mdng ./my_sample_fastq.mdng
```

Then run the report on the MDNG, or compare to lab VCF:

```bash
cargo run --release -p genetic_conditions --bin mdng-to-variants -- ./my_sample_fastq.mdng 2>/dev/null > variants_fastq.json
# compare with variants_from_vcf.json via compare-variant-sources / assess-run
```

### VCF → shard DB from Rust

If you already have a VCF (e.g. from the script or another caller), you can ingest it without running FASTQ steps:

- **ShardDb::ingest_vcf(path)** in `crates/shard_db`: reads plain or `.gz` VCF, one record per ALT; merges by key. For BGZF, decompress first (e.g. `gzip -dc in.vcf.gz > in.vcf`) or use the fastq-to-variants path (it uses `gzip -dc` before ingest).
