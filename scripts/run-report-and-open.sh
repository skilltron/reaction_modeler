#!/usr/bin/env bash
# CD to repo root, run VCF → variants → HTML report, then open the report.
# If extraction JSON is given, compare VCF vs extraction (variant counts + report-level).
#
# Usage: run-report-and-open.sh <vcf> [output.html | extraction.json] [extraction.json]
#   One arg:   VCF only → report to genetic_report.html, open.
#   Two args:  If 2nd ends in .json → VCF + extraction (compare and report to genetic_report.html). If 2nd ends in .html → report to that path.
#   Three args: VCF, output.html, extraction.json → compare and write report to output.html.

set -e
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

if [[ $# -lt 1 ]]; then
  echo "Usage: $0 <vcf> [output.html | extraction.json] [extraction.json]"
  echo "  VCF only:        $0 path/to/file.vcf.gz"
  echo "                   Report written to: \$HOME/Downloads/genetic_report.html"
  echo "  VCF + extraction (compare): $0 path/to/file.vcf.gz path/to/variants_mdng.json"
  echo "  VCF + output:    $0 path/to/file.vcf.gz report.html"
  echo "  All three:       $0 path/to/file.vcf.gz report.html path/to/variants_mdng.json"
  exit 1
fi

VCF="$1"
# Default report output: OS Downloads folder (override with 2nd arg when not extraction.json)
DOWNLOADS="${HOME}/Downloads"
DEFAULT_OUT="${DOWNLOADS}/genetic_report.html"

# Parse optional args: 2nd can be output.html OR extraction.json; 3rd is extraction when 2nd is output.
if [[ $# -ge 2 && "${2}" == *.json ]]; then
  OUT="${DEFAULT_OUT}"
  EXTRACTION_JSON="$2"
elif [[ $# -ge 3 ]]; then
  OUT="$2"
  EXTRACTION_JSON="$3"
else
  OUT="${2:-${DEFAULT_OUT}}"
  EXTRACTION_JSON="${3:-}"
fi
VARIANTS_FROM_VCF="$REPO_ROOT/target/variants_from_vcf.json"
# Clear old variant temp data so this run never uses another person's data
rm -f "$VARIANTS_FROM_VCF" "$REPO_ROOT/target/variants_batch_tmp.json" "$REPO_ROOT/target/variants_from_mdng.json" 2>/dev/null || true
find "$REPO_ROOT/target" -maxdepth 1 -name 'variants_batch_*.json' -delete 2>/dev/null || true
# Append one CSV line per run for easy extraction/averaging (e.g. last day).
TIMINGS_LOG="${TIMINGS_LOG:-$REPO_ROOT/report_timings.log}"
_run_start=$(date +%s)

if [[ ! -f "$VCF" ]]; then
  echo "VCF not found: $VCF"
  exit 1
fi

_timer() { echo "  → ${1}s"; }

echo "Converting VCF to variants JSON..."
_start=$(date +%s)
if [[ "$VCF" == *.gz ]]; then
  # BGZF (e.g. Sequencing.com) requires full decompression; flate2 GzDecoder reads only first block.
  gzip -dc "$VCF" 2>/dev/null | cargo run --release -p genetic_conditions --bin vcf-to-variants 2>/dev/null > "$VARIANTS_FROM_VCF"
else
  cargo run --release -p genetic_conditions --bin vcf-to-variants -- "$VCF" 2>/dev/null > "$VARIANTS_FROM_VCF"
fi
_vcf_sec=$(($(date +%s) - _start))
_timer "$_vcf_sec"

_compare_sec=""
_assess_sec=""
if [[ -n "$EXTRACTION_JSON" ]]; then
  # Resolve to absolute path so we check and report one clear location
  if [[ "$EXTRACTION_JSON" != /* ]]; then
    EXTRACTION_JSON="$REPO_ROOT/$EXTRACTION_JSON"
  fi
  if [[ ! -f "$EXTRACTION_JSON" ]]; then
    echo "Extraction JSON not found: $EXTRACTION_JSON"
    echo "Skipping comparison. Use the full path to your extraction file (e.g. /Volumes/Crucial X10/.../variants_mdng.json or ~/path/to/variants_mdng.json)."
    EXTRACTION_JSON=""
  fi
fi
if [[ -n "$EXTRACTION_JSON" ]]; then
  echo ""
  echo "========== VCF vs your extraction (variant-level) =========="
  _start=$(date +%s)
  cargo run --release -p genetic_conditions --bin compare-variant-sources -- "$VARIANTS_FROM_VCF" "$EXTRACTION_JSON" 2>/dev/null
  _compare_sec=$(($(date +%s) - _start))
  _timer "$_compare_sec"
  echo ""
  echo "========== VCF vs your extraction (report-level: conditions, cascade, survival) =========="
  _start=$(date +%s)
  cargo run --release -p genetic_conditions --bin assess-run -- "$VARIANTS_FROM_VCF" "$EXTRACTION_JSON" 2>/dev/null
  _assess_sec=$(($(date +%s) - _start))
  _timer "$_assess_sec"
  echo ""
fi

echo "Building report from VCF and opening..."
_start=$(date +%s)
export GENETIC_REPORT_INPUT_FILE="$VARIANTS_FROM_VCF"
cargo run --release -p genetic_conditions --bin genetic-report-html -- "$VARIANTS_FROM_VCF" "$OUT" 2>/dev/null
_report_sec=$(($(date +%s) - _start))
_timer "$_report_sec"

_total_sec=$(($(date +%s) - _run_start))
echo "Done. Report written to: $OUT (opened in browser)"

# Append one CSV line for extraction/averaging (date_iso,vcf_sec,compare_sec,assess_sec,report_sec,total_sec)
if [[ ! -f "$TIMINGS_LOG" ]]; then
  echo "date_iso,vcf_sec,compare_sec,assess_sec,report_sec,total_sec" >> "$TIMINGS_LOG"
fi
_date_iso=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
_compare_val="${_compare_sec:--}"
_assess_val="${_assess_sec:--}"
echo "$_date_iso,$_vcf_sec,$_compare_val,$_assess_val,$_report_sec,$_total_sec" >> "$TIMINGS_LOG"
