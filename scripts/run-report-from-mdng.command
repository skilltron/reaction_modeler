#!/usr/bin/env bash
# Mac: double-click to run report from an MDNG. Drag your .mdng file in when prompted.
cd "$(dirname "$0")"
if [[ $# -ge 1 ]]; then
  exec ./run-report-from-mdng.sh "$@"
fi
echo "Drag your .mdng file into this window, then press Enter."
echo "Or close and run: ./run-report-from-mdng.sh path/to/combined.mdng"
read -r DRAG
./run-report-from-mdng.sh $DRAG
echo "Press Enter to close."
read -r
