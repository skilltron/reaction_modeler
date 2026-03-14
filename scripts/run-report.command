#!/usr/bin/env bash
# Mac: double-click this file to open Terminal and run the report.
# You will be prompted to drag your VCF or variants.json file, or run with: ./run-report.command path/to/file.vcf.gz

cd "$(dirname "$0")"
if [[ $# -ge 1 ]]; then
  exec ./run-report.sh "$@"
fi
echo "Drag your VCF or variants.json file into this window, then press Enter."
echo "Or close this window and run from Terminal: ./run-report.sh path/to/file.vcf.gz"
read -r DRAG
./run-report.sh $DRAG
echo "Press Enter to close."
read -r
