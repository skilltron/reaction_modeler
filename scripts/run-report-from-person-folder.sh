#!/usr/bin/env bash
# Generate a genetic report for one person using only variant data from their folder.
# Secure: run-unique variants file, GENETIC_REPORT_INPUT_FILE, no shared target/ files, unique view copy.
#
# Usage: run-report-from-person-folder.sh <folder_path> <person_name>
#   Example: run-report-from-person-folder.sh "/Volumes/Data/Phil Genome" "Phil"
#   Looks in folder for: ULTIMATE-COMPATIBILITY*.txt (Sequencing.com) or genome_*.txt (23andMe).
#   Output: <Person>_report.html in that folder; opens a unique view copy in browser.

set -e
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

if [[ $# -lt 2 ]]; then
  echo "Usage: $0 <folder_path> <person_name>"
  echo "  Example: $0 \"/path/to/Phil folder\" \"Phil\""
  echo "  Secure: uses only that folder's variant file; no shared target/; unique view copy."
  exit 1
fi

PERSON_FOLDER="$1"
PERSON_NAME="$2"
# Safe filename: spaces -> underscore, strip unsafe chars
SAFE_NAME=$(echo "$PERSON_NAME" | sed 's/[[:space:]]/_/g' | tr -dc 'A-Za-z0-9_-' | sed 's/__*/_/g; s/^_\|_$//g')
[[ -z "$SAFE_NAME" ]] && SAFE_NAME="report"

OUT="$PERSON_FOLDER/${SAFE_NAME}_report.html"
VARIANTS_JSON="$PERSON_FOLDER/.variants_${SAFE_NAME}_$$.json"

# Find one variant text file in folder (Sequencing.com or 23andMe style)
SEQ_TXT=$(find "$PERSON_FOLDER" -maxdepth 1 -name 'ULTIMATE-COMPATIBILITY*.txt' 2>/dev/null | head -1)
TWENTY3_TXT=$(find "$PERSON_FOLDER" -maxdepth 1 -name 'genome_*.txt' 2>/dev/null | head -1)

if [[ -n "$SEQ_TXT" && -f "$SEQ_TXT" ]]; then
  INPUT_TXT="$SEQ_TXT"
  echo "Using Sequencing.com export: $INPUT_TXT"
elif [[ -n "$TWENTY3_TXT" && -f "$TWENTY3_TXT" ]]; then
  INPUT_TXT="$TWENTY3_TXT"
  echo "Using 23andMe export: $INPUT_TXT"
else
  echo "Error: No variant text file found in $PERSON_FOLDER"
  echo "  Look for: ULTIMATE-COMPATIBILITY*.txt or genome_*.txt"
  exit 1
fi

# Secure: wipe this person's prior run temp files and all shared target/ variant files; remove old view copies
rm -f "$PERSON_FOLDER/.variants_${SAFE_NAME}_"*.json 2>/dev/null || true
rm -f "${TMPDIR:-/tmp}"/secure_report_view_*.html 2>/dev/null || true
rm -f "$REPO_ROOT/target/variants_from_mdng.json" "$REPO_ROOT/target/variants_mdng"*.json \
      "$REPO_ROOT/target/variants_from_vcf.json" "$REPO_ROOT/target/variants_batch"*.json 2>/dev/null || true

echo "Converting text export to variants JSON (run-unique file)..."
cargo run --release -p genetic_conditions --bin txt-to-variants -- "$INPUT_TXT" > "$VARIANTS_JSON"
if [[ ! -s "$VARIANTS_JSON" ]]; then
  echo "Error: No variants produced. Aborting."
  rm -f "$VARIANTS_JSON" 2>/dev/null || true
  exit 1
fi

echo "Generating report for $PERSON_NAME only (secure: single input file, no reuse)..."
export GENETIC_REPORT_INPUT_FILE="$VARIANTS_JSON"
cargo run --release -p genetic_conditions --bin genetic-report-html -- \
  "$VARIANTS_JSON" "$OUT" "$PERSON_NAME"

rm -f "$VARIANTS_JSON" 2>/dev/null || true

echo "Report written: $OUT"
if [[ "${BATCH_PRIVACY}" != "1" && "${NO_BROWSER_OPEN}" != "1" ]]; then
  VIEW_COPY="${TMPDIR:-/tmp}/secure_report_view_$$.html"
  cp "$OUT" "$VIEW_COPY"
  [[ "$(uname)" == Darwin ]] && open "$VIEW_COPY" || xdg-open "$VIEW_COPY" 2>/dev/null || true
  echo "  → Opened in browser (unique copy for this run only)."
  echo "  → Report also saved at: $OUT"
fi
echo "Done."
