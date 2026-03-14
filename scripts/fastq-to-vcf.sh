#!/usr/bin/env bash
# FASTQ → alignment (BAM) → variant calling (VCF).
# Requires: bwa, samtools, bcftools on PATH; reference FASTA; paired FASTQ.
#
# Usage: fastq-to-vcf.sh <reference.fa> <sample_id> <out_dir> <fastq1.fq.gz> [fastq2.fq.gz]
#   If fastq2 is omitted, fastq1 is treated as single-end.
# Output: <out_dir>/<sample_id>.bam, <out_dir>/<sample_id>.bam.bai, <out_dir>/<sample_id>.vcf.gz, <out_dir>/<sample_id>.vcf.gz.csi

set -e
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
THREADS="${THREADS:-8}"

if [[ $# -lt 4 ]]; then
  echo "Usage: $0 <reference.fa> <sample_id> <out_dir> <fastq1.fq.gz> [fastq2.fq.gz]"
  echo "  reference.fa   Reference genome (e.g. GRCh38). Must be indexed: bwa index reference.fa"
  echo "  sample_id      Name for BAM and VCF (e.g. Henry_Haskell)"
  echo "  out_dir        Directory for BAM and VCF output"
  echo "  fastq1.fq.gz   First pair (or single-end reads)"
  echo "  fastq2.fq.gz   Second pair (optional; if omitted, single-end)"
  echo ""
  echo "Requires: bwa, samtools, bcftools on PATH."
  echo "Optional: THREADS=8 (default)"
  exit 1
fi

REF="$1"
SAMPLE="$2"
OUT_DIR="$3"
FQ1="$4"
FQ2="${5:-}"

mkdir -p "$OUT_DIR"
BAM="$OUT_DIR/$SAMPLE.bam"
VCF_GZ="$OUT_DIR/$SAMPLE.vcf.gz"

if [[ ! -f "$REF" ]]; then
  echo "Reference not found: $REF"
  exit 1
fi
if [[ ! -f "$FQ1" ]]; then
  echo "FASTQ not found: $FQ1"
  exit 1
fi

# 1. Alignment
echo "[1/3] Aligning (bwa mem)..."
if [[ -n "$FQ2" && -f "$FQ2" ]]; then
  bwa mem -t "$THREADS" "$REF" "$FQ1" "$FQ2" | samtools sort -@ "$THREADS" -o "$BAM" -
else
  bwa mem -t "$THREADS" "$REF" "$FQ1" | samtools sort -@ "$THREADS" -o "$BAM" -
fi
samtools index -@ "$THREADS" "$BAM"
echo "  BAM: $BAM"

# 2. Variant calling
echo "[2/3] Variant calling (bcftools mpileup + call)..."
bcftools mpileup -Ou -f "$REF" -@ "$THREADS" "$BAM" | bcftools call -mv -Oz -o "$VCF_GZ"
bcftools index -t -f "$VCF_GZ"
echo "  VCF: $VCF_GZ"

echo "[3/3] Done. Merge this VCF into the pipeline or shard DB (see doc/FASTQ_TO_VARIANTS.md)."
