#!/usr/bin/env bash
# Sync Lucide SVG icons from GitHub.
# Usage: ./scripts/sync-lucide.sh [--all]
#
#   --all   Download ALL ~1500 icons (default: only missing ones)
#
# Output: crates/aura-icons-lucide/assets/svgs/*.svg

set -euo pipefail

LUCIDE_REPO="https://github.com/lucide-icons/lucide.git"
SVG_DIR="$(dirname "$0")/../crates/aura-icons-lucide/assets/svgs"
TMP_DIR=$(mktemp -d)

cleanup() { rm -rf "$TMP_DIR"; }
trap cleanup EXIT

echo "=== Cloning lucide-icons/lucide (shallow) ==="
git clone --depth 1 --filter=blob:none --sparse "$LUCIDE_REPO" "$TMP_DIR/lucide" 2>&1 | tail -1

cd "$TMP_DIR/lucide"
git sparse-checkout set icons

echo "=== Copying SVG icons ==="
mkdir -p "$SVG_DIR"

if [ "${1:-}" = "--all" ]; then
    cp icons/*.svg "$SVG_DIR/"
    echo "Copied $(ls icons/*.svg | wc -l) icons."
else
    count=0
    for svg in icons/*.svg; do
        name=$(basename "$svg")
        if [ ! -f "$SVG_DIR/$name" ]; then
            cp "$svg" "$SVG_DIR/"
            count=$((count + 1))
        fi
    done
    echo "Copied $count new icons (total: $(ls "$SVG_DIR" | wc -l))."
fi

echo "=== Done ==="
