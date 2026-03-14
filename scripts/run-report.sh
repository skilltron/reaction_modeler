#!/usr/bin/env bash
# Standalone run script for the genetic report package (Mac/Linux).
# Expects vcf-to-variants and genetic-report-html in the same directory as this script.
# Usage: ./run-report.sh <path-to.vcf|path-to.vcf.gz|path-to-variants.json>
# Report is written to: $HOME/Downloads/genetic_report.html

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOWNLOADS="${HOME}/Downloads"
OUT="${DOWNLOADS}/genetic_report.html"

if [[ $# -lt 1 ]]; then
  echo "Usage: $0 <path-to.vcf|path-to.vcf.gz|path-to-variants.json>"
  echo "  Report will be written to: $OUT"
  exit 1
fi

INPUT="$1"
VARIANTS_JSON="${SCRIPT_DIR}/variants_temp_$$.json"
cleanup() { rm -f "$VARIANTS_JSON"; }
trap cleanup EXIT

# Resolve to absolute path if relative
if [[ "$INPUT" != /* ]]; then
  if [[ -f "$SCRIPT_DIR/$INPUT" ]]; then
    INPUT="$SCRIPT_DIR/$INPUT"
  fi
fi

if [[ ! -f "$INPUT" ]]; then
  echo "File not found: $INPUT"
  exit 1
fi

VCF_TO_VARIANTS="${SCRIPT_DIR}/vcf-to-variants"
REPORT_HTML="${SCRIPT_DIR}/genetic-report-html"
if [[ ! -x "$VCF_TO_VARIANTS" ]]; then
  echo "Binary not found: $VCF_TO_VARIANTS (run from the package folder or build with cargo)"
  exit 1
fi
if [[ ! -x "$REPORT_HTML" ]]; then
  echo "Binary not found: $REPORT_HTML"
  exit 1
fi

if [[ "$INPUT" == *.json ]]; then
  echo "Building report from variants JSON..."
  "$REPORT_HTML" "$INPUT" "$OUT"
else
  echo "Converting VCF to variants..."
  if [[ "$INPUT" == *.gz ]]; then
    gzip -dc "$INPUT" 2>/dev/null | "$VCF_TO_VARIANTS" 2>/dev/null > "$VARIANTS_JSON" || "$VCF_TO_VARIANTS" "$INPUT" 2>/dev/null > "$VARIANTS_JSON"
  else
    "$VCF_TO_VARIANTS" "$INPUT" 2>/dev/null > "$VARIANTS_JSON"
  fi
  echo "Building report..."
  "$REPORT_HTML" "$VARIANTS_JSON" "$OUT"
fi

echo "Done. Report written to: $OUT"
