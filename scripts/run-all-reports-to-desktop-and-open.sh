#!/usr/bin/env bash
# Generate all 6 reports as PLAIN TEXT into ~/Desktop/GeneticReports/ (no HTML/browser until fixed).
set -e
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"
OUT_DIR="${1:-$HOME/Desktop/GeneticReports}"
mkdir -p "$OUT_DIR"
HEALTH="$HOME/Library/Mobile Documents/com~apple~CloudDocs/Health"
DOWNLOADS="$HOME/Library/Mobile Documents/com~apple~CloudDocs/Downloads"
export NO_BROWSER_OPEN=1
export GENETIC_REPORT_PLAIN_TEXT=1

echo "Writing all reports as PLAIN TEXT to $OUT_DIR."

# Henry
echo "Henry..."
GENETIC_REPORT_INPUT_FILE="$REPO_ROOT/target/henry_small_variants.json" \
  cargo run --release -p genetic_conditions --bin genetic-report-html -- \
  target/henry_small_variants.json "$OUT_DIR/Henry_report.txt" "Henry" 2>&1 | grep -E "Reading|WILL WRITE|RENAMED|PLAIN TEXT" || true

# Siva
SIVA_JSON="$OUT_DIR/.variants_siva_$$.json"
cargo run --release -p genetic_conditions --bin txt-to-variants -- "$HEALTH/Siva/genome_Sivakumaran_Tharmarajah_v5_Full_20250705111518.txt" > "$SIVA_JSON" 2>/dev/null
GENETIC_REPORT_INPUT_FILE="$SIVA_JSON" cargo run --release -p genetic_conditions --bin genetic-report-html -- "$SIVA_JSON" "$OUT_DIR/Siva_report.txt" "Siva" 2>&1 | grep -E "Reading|WILL WRITE|RENAMED|PLAIN TEXT" || true
rm -f "$SIVA_JSON"

# Regina
REGINA_JSON="$OUT_DIR/.variants_regina_$$.json"
gzip -dc "$HEALTH/Regina/ReginaRich-SQ732UR7-30x-WGS-Sequencing_com-08-10-25.snp-indel.genome.vcf 3.gz" 2>/dev/null | cargo run --release -p genetic_conditions --bin vcf-to-variants 2>/dev/null > "$REGINA_JSON"
GENETIC_REPORT_INPUT_FILE="$REGINA_JSON" cargo run --release -p genetic_conditions --bin genetic-report-html -- "$REGINA_JSON" "$OUT_DIR/Regina_report.txt" "Regina" 2>&1 | grep -E "Reading|WILL WRITE|RENAMED|PLAIN TEXT" || true
rm -f "$REGINA_JSON"

# Michael
MICHAEL_JSON="$OUT_DIR/.variants_michael_$$.json"
cargo run --release -p genetic_conditions --bin txt-to-variants -- "$HEALTH/MichaelBalderree/ULTIMATE-COMPATIBILITY-MichaelBalderree.txt" > "$MICHAEL_JSON" 2>/dev/null
GENETIC_REPORT_INPUT_FILE="$MICHAEL_JSON" cargo run --release -p genetic_conditions --bin genetic-report-html -- "$MICHAEL_JSON" "$OUT_DIR/Michael_Balderree_report.txt" "Michael Balderree" 2>&1 | grep -E "Reading|WILL WRITE|RENAMED|PLAIN TEXT" || true
rm -f "$MICHAEL_JSON"

# Elisabeth
ELISABETH_JSON="$OUT_DIR/.variants_elisabeth_$$.json"
cargo run --release -p genetic_conditions --bin txt-to-variants -- "$HEALTH/ElisabethBalderree/ULTIMATE-COMPATIBILITY-ElisabethBalderree.txt" > "$ELISABETH_JSON" 2>/dev/null
GENETIC_REPORT_INPUT_FILE="$ELISABETH_JSON" cargo run --release -p genetic_conditions --bin genetic-report-html -- "$ELISABETH_JSON" "$OUT_DIR/Elisabeth_Balderree_report.txt" "Elisabeth Balderree" 2>&1 | grep -E "Reading|WILL WRITE|RENAMED|PLAIN TEXT" || true
rm -f "$ELISABETH_JSON"

# Lisa
LISA_JSON="$OUT_DIR/.variants_lisa_$$.json"
cargo run --release -p genetic_conditions --bin txt-to-variants -- "$DOWNLOADS/genome_lisa_haskell_v4_full_20241210213511.txt" > "$LISA_JSON" 2>/dev/null
GENETIC_REPORT_INPUT_FILE="$LISA_JSON" cargo run --release -p genetic_conditions --bin genetic-report-html -- "$LISA_JSON" "$OUT_DIR/Lisa_Haskell_report.txt" "Lisa Haskell" 2>&1 | grep -E "Reading|WILL WRITE|RENAMED|PLAIN TEXT" || true
rm -f "$LISA_JSON"

echo ""
echo "Report line from each file (should show different names and variant counts):"
for f in "$OUT_DIR"/Henry_report.txt "$OUT_DIR"/Siva_report.txt "$OUT_DIR"/Regina_report.txt "$OUT_DIR"/Michael_Balderree_report.txt "$OUT_DIR"/Elisabeth_Balderree_report.txt "$OUT_DIR"/Lisa_Haskell_report.txt; do
  [ -f "$f" ] && echo "  $(basename "$f"): $(grep '^Report:' "$f" | head -1)"
done

# Verify datasets differ: same Dataset line = same input data (e.g. duplicate source files)
echo ""
DATASET_CHECK=$(mktemp)
for f in "$OUT_DIR"/Henry_report.txt "$OUT_DIR"/Siva_report.txt "$OUT_DIR"/Regina_report.txt "$OUT_DIR"/Michael_Balderree_report.txt "$OUT_DIR"/Elisabeth_Balderree_report.txt "$OUT_DIR"/Lisa_Haskell_report.txt; do
  [ -f "$f" ] && echo "$(basename "$f")|$(grep '^Dataset:' "$f" | head -1)" >> "$DATASET_CHECK"
done
DUPLICATES=$(sort -t'|' -k2 "$DATASET_CHECK" | cut -d'|' -f2 | uniq -d)
if [ -n "$DUPLICATES" ]; then
  echo "WARNING: Some reports have IDENTICAL Dataset (same variant data):"
  while IFS= read -r line; do
    [ -z "$line" ] && continue
    echo "  Same data: $line"
    grep -F "$line" "$DATASET_CHECK" | cut -d'|' -f1 | sed 's/^/    -> /'
  done <<< "$DUPLICATES"
  echo "  Fix: ensure each person's source file (e.g. ULTIMATE-COMPATIBILITY-*.txt) is different."
else
  echo "OK: All 6 reports have different Dataset (independent data)."
fi
rm -f "$DATASET_CHECK"
echo ""
echo "Opening each .txt report..."
open "$OUT_DIR/Henry_report.txt"
open "$OUT_DIR/Siva_report.txt"
open "$OUT_DIR/Regina_report.txt"
open "$OUT_DIR/Michael_Balderree_report.txt"
open "$OUT_DIR/Elisabeth_Balderree_report.txt"
open "$OUT_DIR/Lisa_Haskell_report.txt"
echo "Done. Plain text reports are in $OUT_DIR. Each file shows Report/Dataset at the top — no browser cache."
