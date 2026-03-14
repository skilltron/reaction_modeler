#!/usr/bin/env bash
# Generate the report from an MDNG file and open it.
# Usage: run-report-from-mdng.sh <path.mdng> [output.html]
# Uses a run-unique variants file next to the output (never target/) so no shared state with other runs.

set -e
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

if [[ $# -lt 1 ]]; then
  echo "Usage: $0 <path.mdng> [output.html]"
  echo "  Report is written to \$HOME/Downloads/genetic_report.html unless you pass a second argument."
  exit 1
fi

MDNG="$1"
DOWNLOADS="${HOME}/Downloads"
OUT="${2:-${DOWNLOADS}/genetic_report.html}"
# Run-unique variants file in same dir as output so we never read from target/ or another person's run
OUT_DIR="$(dirname "$OUT")"
OUT_BASE="$(basename "$OUT" .html)"
VARIANTS_JSON="$OUT_DIR/.variants_${OUT_BASE}_$$.json"
REPORT_NAME=""   # optional: e.g. "Lisa Haskell" for report title

if [[ ! -f "$MDNG" ]]; then
  echo "MDNG not found: $MDNG"
  exit 1
fi

# Wipe all variant temp files so this run cannot read anyone else's data
rm -f "$REPO_ROOT/target/variants_from_mdng.json" "$REPO_ROOT/target/variants_mdng.json" \
      "$REPO_ROOT/target/variants_mdng_clean.json" "$REPO_ROOT/target/variants_from_vcf.json" \
      "$REPO_ROOT/target/variants_batch_tmp.json" 2>/dev/null || true
find "$REPO_ROOT/target" -maxdepth 1 -name 'variants_batch_*.json' -delete 2>/dev/null || true

mkdir -p "$OUT_DIR"
# Remove any stale run-unique file from same output base (safety)
rm -f "$OUT_DIR/.variants_${OUT_BASE}_"*.json 2>/dev/null || true

echo "Converting MDNG to variants JSON (run-unique file)..."
cargo run --release -p genetic_conditions --bin mdng-to-variants -- "$MDNG" 2>/dev/null > "$VARIANTS_JSON"
if [[ ! -s "$VARIANTS_JSON" ]]; then
  echo "Error: MDNG produced no variants. Aborting."
  rm -f "$VARIANTS_JSON" 2>/dev/null || true
  exit 1
fi

# Optional: set report title from output filename (e.g. Lisa_Haskell_report -> "Lisa Haskell")
if [[ "$OUT_BASE" == *"_report" ]]; then
  REPORT_NAME="${OUT_BASE%_report}"
  REPORT_NAME="${REPORT_NAME//_/ }"
fi

echo "Generating report from this run's variants only..."
# Force the report to read ONLY from this file (so main report cannot use another person's data)
export GENETIC_REPORT_INPUT_FILE="$VARIANTS_JSON"
if [[ -n "$REPORT_NAME" ]]; then
  BATCH_PRIVACY="${BATCH_PRIVACY:-1}" NO_BROWSER_OPEN="${NO_BROWSER_OPEN:-1}" \
    cargo run --release -p genetic_conditions --bin genetic-report-html -- "$VARIANTS_JSON" "$OUT" "$REPORT_NAME"
else
  BATCH_PRIVACY="${BATCH_PRIVACY:-1}" NO_BROWSER_OPEN="${NO_BROWSER_OPEN:-1}" \
    cargo run --release -p genetic_conditions --bin genetic-report-html -- "$VARIANTS_JSON" "$OUT"
fi
# Remove variants file immediately so no reuse
rm -f "$VARIANTS_JSON" 2>/dev/null || true

echo "Report written: $OUT"
if [[ "${BATCH_PRIVACY}" != "1" && "${NO_BROWSER_OPEN}" != "1" ]]; then
  if [[ "$(uname)" == Darwin ]]; then
    open "$OUT"
  else
    xdg-open "$OUT" 2>/dev/null || true
  fi
fi
echo "Done."
