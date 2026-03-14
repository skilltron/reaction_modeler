#!/usr/bin/env bash
# Copy release binaries to Mendel's X-10 drive, or to Desktop if X-10 is not available.
# Run from repo root after: cargo build --release

set -e
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TARGET="$REPO_ROOT/target/release"
X10_VOLUME="/Volumes/Crucial X10"
X10_DEST="$X10_VOLUME/reaction_modeler_release"
DESKTOP_DEST="$HOME/Desktop/reaction_modeler_release"

BINARIES=(
  genetic-report-html
  vcf-to-variants
  mdng-to-variants
  compare-variant-sources
  assess-run
  combine-to-mdng
)
for b in "${BINARIES[@]}"; do
  if [[ ! -f "$TARGET/$b" ]]; then
    echo "Release binary not found: $b. Run: cargo build --release"
    exit 1
  fi
done

if [[ -d "$X10_VOLUME" ]]; then
  DEST="$X10_DEST"
else
  DEST="$DESKTOP_DEST"
  echo "X-10 drive not mounted. Copying to Desktop. Move to X-10 later if needed."
fi

mkdir -p "$DEST"
for b in "${BINARIES[@]}"; do
  cp -f "$TARGET/$b" "$DEST/$b"
  echo "Copied $b to: $DEST"
done
echo "Run: ./genetic-report-html [variants.json [output.html]]"
echo "     ./vcf-to-variants <vcf|vcf.gz> > variants.json"
echo "     ./mdng-to-variants <path.mdng> > variants_mdng.json"
echo "     ./compare-variant-sources variants_a.json variants_b.json"
echo "     ./assess-run variants.json [variants_b.json]"
echo "     ./combine-to-mdng output.mdng input1.mdng input2.vcf [input3.vcf.gz ...]"
