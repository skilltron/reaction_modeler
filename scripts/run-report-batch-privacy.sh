#!/usr/bin/env bash
# Run report for multiple VCFs with privacy in mind:
# - Outputs to a directory you choose. With --names: named files (e.g. Lisa_Haskell_report.html) and name in report title.
# - Does not open the report in a browser.
# - Clears temp data before the run and removes it after each report (no leftover variants JSON).
#
# Usage:
#   run-report-batch-privacy.sh <output_dir> <vcf1> [vcf2 ...]
#   run-report-batch-privacy.sh --names <output_dir> <vcf1> <name1> [<vcf2> <name2> ...]
#
# With --names: report title and filename use the given name (e.g. "Lisa Haskell" -> Lisa_Haskell_report.html).

set -e
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

USE_NAMES=0
if [[ "$1" == --names ]]; then
  USE_NAMES=1
  shift
fi

if [[ $# -lt 2 ]]; then
  echo "Usage: $0 [--names] <output_dir> <vcf1> [name1 | vcf2 ...]"
  echo "  Without --names: report_001.html, report_002.html, ..."
  echo "  With --names: <name>_report.html and name in report title (pairs: vcf1 name1 vcf2 name2 ...)"
  exit 1
fi

OUT_DIR="$1"
shift
mkdir -p "$OUT_DIR"
mkdir -p "$REPO_ROOT/target"

# Batch mode: no browser open; no reuse of prior run's data
export BATCH_PRIVACY=1
export NO_BROWSER_OPEN=1
export INCLUDE_SEQUENCING_PARITY=0

# Clear ALL batch/variant temp data before any run so no report ever sees another person's data
rm -f "$REPO_ROOT/target/variants_batch_tmp.json" \
      "$REPO_ROOT/target/variants_from_vcf.json" \
      "$REPO_ROOT/target/variants_from_mdng.json" 2>/dev/null || true
find "$REPO_ROOT/target" -maxdepth 1 -name 'variants_batch_*.json' -delete 2>/dev/null || true
find "$REPO_ROOT/target" -maxdepth 1 -name 'vcf_err_*.txt' -delete 2>/dev/null || true
# When using --names, remove old numbered reports so only fresh named reports remain
if [[ $USE_NAMES -eq 1 ]]; then
  find "$OUT_DIR" -maxdepth 1 -name 'report_[0-9][0-9][0-9].html' -delete 2>/dev/null || true
fi

# Sanitize name for filename: spaces -> underscore, remove unsafe chars
sanitize_name() {
  echo "$1" | sed 's/[*?[:space:]]/_/g' | tr -dc 'A-Za-z0-9_-' | sed 's/__*/_/g; s/^_\|_$//g'
}

n=1
if [[ $USE_NAMES -eq 1 ]]; then
  while [[ $# -ge 2 ]]; do
    VCF="$1"
    NAME="$2"
    shift 2
    if [[ ! -f "$VCF" ]]; then
      echo "Skip (not found): $VCF"
      continue
    fi
    # Unique temp file per person so no run can read another's data
    VARIANTS_TMP="$REPO_ROOT/target/variants_batch_$$_${n}.json"
    rm -f "$VARIANTS_TMP"
    safe_name=$(sanitize_name "$NAME")
    if [[ -z "$safe_name" ]]; then safe_name="report_$(printf '%03d' "$n")"; fi
    out_name="${safe_name}_report.html"
    out_path="$OUT_DIR/$out_name"
    echo "Run $n: $VCF (${NAME}) -> $out_path"
    if find "$OUT_DIR" -maxdepth 1 -name '*.pdf' 2>/dev/null | grep -q .; then
      export INCLUDE_SEQUENCING_PARITY=1
    else
      export INCLUDE_SEQUENCING_PARITY=0
    fi
    VCF_ERR="$REPO_ROOT/target/vcf_err_$$.txt"
    if [[ "$VCF" == *.gz ]]; then
      gzip -dc "$VCF" 2>"$VCF_ERR" | cargo run --release -p genetic_conditions --bin vcf-to-variants 2>>"$VCF_ERR" > "$VARIANTS_TMP"
    else
      cargo run --release -p genetic_conditions --bin vcf-to-variants -- "$VCF" 2>"$VCF_ERR" > "$VARIANTS_TMP"
    fi
    # Only run report if we have valid variant data (avoid wrong/empty report)
    if [[ ! -s "$VARIANTS_TMP" ]]; then
      echo "  ERROR: No variant data produced for ${NAME}. Skip." >&2
      [[ -s "$VCF_ERR" ]] && echo "  (vcf-to-variants stderr: $(head -3 "$VCF_ERR" | tr '\n' ' '))" >&2
      rm -f "$VARIANTS_TMP" "$VCF_ERR"
      n=$((n + 1))
      continue
    fi
    # Reject empty variant set (e.g. VCF with header but no data rows)
    if ! grep -q '"chromosome"' "$VARIANTS_TMP" 2>/dev/null; then
      echo "  ERROR: VCF produced 0 variants (header-only or unparseable?) for ${NAME}. Skip." >&2
      [[ -s "$VCF_ERR" ]] && echo "  (vcf-to-variants stderr: $(head -3 "$VCF_ERR" | tr '\n' ' '))" >&2
      rm -f "$VCF_ERR"
      rm -f "$VARIANTS_TMP"
      n=$((n + 1))
      continue
    fi
    rm -f "$VCF_ERR"
    export GENETIC_REPORT_INPUT_FILE="$VARIANTS_TMP"
    cargo run --release -p genetic_conditions --bin genetic-report-html -- "$VARIANTS_TMP" "$out_path" "$NAME"
    rm -f "$VARIANTS_TMP"
    echo "  -> $out_path"
    n=$((n + 1))
  done
else
  for VCF in "$@"; do
    if [[ ! -f "$VCF" ]]; then
      echo "Skip (not found): $VCF"
      continue
    fi
    VARIANTS_TMP="$REPO_ROOT/target/variants_batch_$$_${n}.json"
    rm -f "$VARIANTS_TMP"
    out_name=$(printf "report_%03d.html" "$n")
    out_path="$OUT_DIR/$out_name"
    echo "Run $n: $VCF -> $out_path"
    if find "$OUT_DIR" -maxdepth 1 -name '*.pdf' 2>/dev/null | grep -q .; then
      export INCLUDE_SEQUENCING_PARITY=1
    else
      export INCLUDE_SEQUENCING_PARITY=0
    fi
    VCF_ERR="$REPO_ROOT/target/vcf_err_$$.txt"
    if [[ "$VCF" == *.gz ]]; then
      gzip -dc "$VCF" 2>"$VCF_ERR" | cargo run --release -p genetic_conditions --bin vcf-to-variants 2>>"$VCF_ERR" > "$VARIANTS_TMP"
    else
      cargo run --release -p genetic_conditions --bin vcf-to-variants -- "$VCF" 2>"$VCF_ERR" > "$VARIANTS_TMP"
    fi
    if [[ ! -s "$VARIANTS_TMP" ]]; then
      echo "  ERROR: No variant data produced for VCF. Skip." >&2
      [[ -s "$VCF_ERR" ]] && echo "  (vcf-to-variants stderr: $(head -3 "$VCF_ERR" | tr '\n' ' '))" >&2
      rm -f "$VARIANTS_TMP" "$VCF_ERR"
      n=$((n + 1))
      continue
    fi
    if ! grep -q '"chromosome"' "$VARIANTS_TMP" 2>/dev/null; then
      echo "  ERROR: VCF produced 0 variants (header-only or unparseable?). Skip." >&2
      [[ -s "$VCF_ERR" ]] && echo "  (vcf-to-variants stderr: $(head -3 "$VCF_ERR" | tr '\n' ' '))" >&2
      rm -f "$VCF_ERR" "$VARIANTS_TMP"
      n=$((n + 1))
      continue
    fi
    rm -f "$VCF_ERR"
    export GENETIC_REPORT_INPUT_FILE="$VARIANTS_TMP"
    cargo run --release -p genetic_conditions --bin genetic-report-html -- "$VARIANTS_TMP" "$out_path"
    rm -f "$VARIANTS_TMP"
    echo "  -> $out_path"
    n=$((n + 1))
  done
fi

# Final cleanup: remove any batch temp files that might remain
find "$REPO_ROOT/target" -maxdepth 1 -name 'variants_batch_*.json' -delete 2>/dev/null || true
find "$REPO_ROOT/target" -maxdepth 1 -name 'vcf_err_*.txt' -delete 2>/dev/null || true

echo "Done. Reports in $OUT_DIR (no browser opened)."
