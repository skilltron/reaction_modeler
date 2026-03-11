#!/usr/bin/env bash
# Copy release binary to Mendel's X-10 drive, or to Desktop if X-10 is not available.
# Run from repo root after: cargo build --release

set -e
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BINARY="$REPO_ROOT/target/release/genetic-report-html"
X10_VOLUME="/Volumes/Crucial X10"
X10_DEST="$X10_VOLUME/reaction_modeler_release"
DESKTOP_DEST="$HOME/Desktop/reaction_modeler_release"

if [[ ! -f "$BINARY" ]]; then
  echo "Release binary not found. Run: cargo build --release"
  exit 1
fi

if [[ -d "$X10_VOLUME" ]]; then
  DEST="$X10_DEST"
else
  DEST="$DESKTOP_DEST"
  echo "X-10 drive not mounted. Copying to Desktop. Move to X-10 later if needed."
fi

mkdir -p "$DEST"
cp -f "$BINARY" "$DEST/genetic-report-html"
echo "Copied genetic-report-html to: $DEST"
echo "Run from there: ./genetic-report-html [variants.json [output.html]]"
