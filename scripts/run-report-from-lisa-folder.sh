#!/usr/bin/env bash
# Generate Lisa's report using only variant data from her folder (Sequencing.com or 23andMe text export).
# Uses: ULTIMATE-COMPATIBILITY-*Lisa*.txt (Sequencing.com) or genome_lisa_haskell*.txt (23andMe) in the folder.
# Output: Lisa_Haskell_report.html in the same folder. No shared target/ files.
#
# Usage: run-report-from-lisa-folder.sh [folder_path]
#   Default folder: /Volumes/Crucial X10/Lisa New Genome Squencing feb 20 2026

set -e
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

LISA_FOLDER="${1:-/Volumes/Crucial X10/Lisa New Genome Squencing feb 20 2026}"
OUT="$LISA_FOLDER/Lisa_Haskell_report.html"
VARIANTS_JSON="$LISA_FOLDER/.variants_lisa_$$.json"

# Prefer Sequencing.com export (WGS); fallback to 23andMe
SEQ_TXT=$(find "$LISA_FOLDER" -maxdepth 1 \( -name 'ULTIMATE-COMPATIBILITY*Lisa*.txt' -o -name 'ULTIMATE-COMPATIBILITY*LisaHaskell*.txt' \) 2>/dev/null | head -1)
TWENTY3_TXT=$(find "$LISA_FOLDER" -maxdepth 1 -name 'genome_lisa_haskell*.txt' 2>/dev/null | head -1)

if [[ -n "$SEQ_TXT" && -f "$SEQ_TXT" ]]; then
  INPUT_TXT="$SEQ_TXT"
  echo "Using Sequencing.com export: $INPUT_TXT"
elif [[ -n "$TWENTY3_TXT" && -f "$TWENTY3_TXT" ]]; then
  INPUT_TXT="$TWENTY3_TXT"
  echo "Using 23andMe export: $INPUT_TXT"
else
  echo "Error: No Lisa variant text file found in $LISA_FOLDER"
  echo "  Look for: ULTIMATE-COMPATIBILITY*Lisa*.txt or genome_lisa_haskell*.txt"
  exit 1
fi

# Wipe any previous run temp and target variant files; remove old Lisa view copies so we open a fresh unique file
rm -f "$LISA_FOLDER/.variants_lisa_"*.json 2>/dev/null || true
rm -f "${TMPDIR:-/tmp}"/lisa_report_view_*.html 2>/dev/null || true
rm -f "$REPO_ROOT/target/variants_from_mdng.json" "$REPO_ROOT/target/variants_mdng"*.json \
      "$REPO_ROOT/target/variants_from_vcf.json" "$REPO_ROOT/target/variants_batch"*.json 2>/dev/null || true

echo "Converting text export to variants JSON..."
cargo run --release -p genetic_conditions --bin txt-to-variants -- "$INPUT_TXT" > "$VARIANTS_JSON"
if [[ ! -s "$VARIANTS_JSON" ]]; then
  echo "Error: No variants produced. Aborting."
  rm -f "$VARIANTS_JSON" 2>/dev/null || true
  exit 1
fi

echo "Generating report from Lisa's data only..."
# Force the report to read ONLY from this file (ignores any other path that might be passed)
export GENETIC_REPORT_INPUT_FILE="$VARIANTS_JSON"
# By default open this report so you see Lisa's data; set BATCH_PRIVACY=1 or NO_BROWSER_OPEN=1 to skip
cargo run --release -p genetic_conditions --bin genetic-report-html -- \
  "$VARIANTS_JSON" "$OUT" "Lisa Haskell"

rm -f "$VARIANTS_JSON" 2>/dev/null || true

echo "Report written: $OUT"
if [[ "${BATCH_PRIVACY}" != "1" && "${NO_BROWSER_OPEN}" != "1" ]]; then
  # Open a unique copy so the browser always loads this run's content (avoids cache from genetic_report.html or wrong file)
  VIEW_COPY="${TMPDIR:-/tmp}/lisa_report_view_$$.html"
  cp "$OUT" "$VIEW_COPY"
  [[ "$(uname)" == Darwin ]] && open "$VIEW_COPY" || xdg-open "$VIEW_COPY" 2>/dev/null || true
  echo "  → Opened in browser (unique URL so you see this run's data, not a cached or wrong report)."
  echo "  → Report also saved at: $OUT"
fi
echo "Done."
