#!/usr/bin/env bash
# Run genetic report from a VCF (e.g. snp-indel.genome.vcf) for one person. Secure: run-unique variants file, GENETIC_REPORT_INPUT_FILE, no shared target/, unique view copy.
#
# Usage: run-report-from-vcf-secure.sh <vcf_path> <person_name> [output.html]
#   If output.html omitted, writes to same dir as VCF: <person_name>_report.html
#   Example: run-report-from-vcf-secure.sh "/path/to/Phil.snp-indel.genome.vcf" "Phil"

set -e
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

if [[ $# -lt 2 ]]; then
  echo "Usage: $0 <vcf_path> <person_name> [output.html]"
  echo "  Example: $0 \"/Volumes/X10/Phil Genome/Phil.snp-indel.genome.vcf\" \"Phil\""
  exit 1
fi

VCF="$1"
PERSON_NAME="$2"
OUT="${3:-}"

if [[ ! -f "$VCF" ]]; then
  echo "Error: VCF not found: $VCF"
  exit 1
fi

VCF_DIR="$(dirname "$VCF")"
SAFE_NAME=$(echo "$PERSON_NAME" | sed 's/[[:space:]]/_/g' | tr -dc 'A-Za-z0-9_-' | sed 's/__*/_/g; s/^_\|_$//g')
[[ -z "$SAFE_NAME" ]] && SAFE_NAME="report"

if [[ -z "$OUT" ]]; then
  OUT="$VCF_DIR/${SAFE_NAME}_report.html"
fi

# Run-unique variants file next to output (not in target/)
OUT_DIR="$(dirname "$OUT")"
VARIANTS_JSON="$OUT_DIR/.variants_${SAFE_NAME}_$$.json"

# Secure: wipe shared target/ and old view copies
rm -f "$REPO_ROOT/target/variants_from_vcf.json" "$REPO_ROOT/target/variants_from_mdng.json" \
      "$REPO_ROOT/target/variants_batch"*.json 2>/dev/null || true
rm -f "${TMPDIR:-/tmp}"/secure_report_view_*.html 2>/dev/null || true
rm -f "$OUT_DIR/.variants_${SAFE_NAME}_"*.json 2>/dev/null || true

mkdir -p "$OUT_DIR"

echo "Converting VCF to variants (run-unique file)..."
if [[ "$VCF" == *.gz ]]; then
  gzip -dc "$VCF" 2>/dev/null | cargo run --release -p genetic_conditions --bin vcf-to-variants 2>/dev/null > "$VARIANTS_JSON"
else
  cargo run --release -p genetic_conditions --bin vcf-to-variants -- "$VCF" 2>/dev/null > "$VARIANTS_JSON"
fi

if [[ ! -s "$VARIANTS_JSON" ]] || ! grep -q '"chromosome"' "$VARIANTS_JSON" 2>/dev/null; then
  echo "Error: No variants produced from VCF. Aborting."
  rm -f "$VARIANTS_JSON" 2>/dev/null || true
  exit 1
fi

echo "Generating report for $PERSON_NAME only (secure)..."
export GENETIC_REPORT_INPUT_FILE="$VARIANTS_JSON"
cargo run --release -p genetic_conditions --bin genetic-report-html -- \
  "$VARIANTS_JSON" "$OUT" "$PERSON_NAME"

rm -f "$VARIANTS_JSON" 2>/dev/null || true

echo "Report written: $OUT"
if [[ "${BATCH_PRIVACY}" != "1" && "${NO_BROWSER_OPEN}" != "1" ]]; then
  VIEW_COPY="${TMPDIR:-/tmp}/secure_report_view_$$.html"
  cp "$OUT" "$VIEW_COPY"
  [[ "$(uname)" == Darwin ]] && open "$VIEW_COPY" || xdg-open "$VIEW_COPY" 2>/dev/null || true
  echo "  → Opened in browser (unique copy)."
fi
echo "Done."
