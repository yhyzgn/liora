#!/usr/bin/env bash
# Sync Lucide SVG icons from GitHub — incremental, hash-based.
#
#   ./scripts/sync-lucide.sh         # incremental (hash compare)
#   ./scripts/sync-lucide.sh --full  # force full refresh
#
# Rules:
#   1. Never delete existing SVGs
#   2. Cache lucide repo at ../../target/lucide-cache (fetch if >24h old)
#   3. Only add new icons; only update if upstream hash differs
#   4. First run does full sync

set -euo pipefail

LUCIDE_REPO="https://github.com/lucide-icons/lucide.git"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
CRATE_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
PROJECT_DIR="$(cd "$CRATE_DIR/../.." && pwd)"
SVG_DIR="$CRATE_DIR/assets/svgs"
HASH_FILE="$SVG_DIR/.hashes"
CACHE_DIR="$PROJECT_DIR/target/lucide-cache"
CACHE_STAMP="$CACHE_DIR/.stamp"
FULL="${1:-}"
CACHE_TTL=$((24 * 3600))  # 24 hours

mkdir -p "$SVG_DIR" "$CACHE_DIR"

# ── Clone or update cache ──────────────────────────────────
need_fetch=true
if [ -d "$CACHE_DIR/.git" ] && [ "$FULL" != "--full" ]; then
    now=$(date +%s)
    if [ -f "$CACHE_STAMP" ]; then
        last=$(cat "$CACHE_STAMP")
        if [ $((now - last)) -lt $CACHE_TTL ]; then
            need_fetch=false
        fi
    fi
fi

if $need_fetch; then
    if [ -d "$CACHE_DIR/.git" ]; then
        echo "=== Updating lucide cache (git fetch) ==="
        git -C "$CACHE_DIR" fetch --depth 1 origin main 2>&1 | tail -1
        git -C "$CACHE_DIR" reset --hard origin/main 2>&1 | tail -1
    else
        echo "=== Cloning lucide-icons/lucide to cache ==="
        git clone --depth 1 --filter=blob:none "$LUCIDE_REPO" "$CACHE_DIR" 2>&1 | tail -1
    fi
    date +%s > "$CACHE_STAMP"
fi

# ── Compare hashes ─────────────────────────────────────────
declare -A known
if [ -f "$HASH_FILE" ] && [ "$FULL" != "--full" ]; then
    while IFS='  ' read -r hash name; do
        known["$name"]="$hash"
    done < "$HASH_FILE"
fi

added=0; updated=0; skipped=0

for svg in "$CACHE_DIR/icons"/*.svg; do
    [ -f "$svg" ] || continue
    name=$(basename "$svg")
    new_hash=$(sha256sum "$svg" | awk '{print $1}')

    if [ ! -f "$SVG_DIR/$name" ]; then
        cp "$svg" "$SVG_DIR/"
        added=$((added + 1))
    elif [ "$FULL" = "--full" ]; then
        cp "$svg" "$SVG_DIR/"
        updated=$((updated + 1))
    elif [ "${known[$name]:-}" != "$new_hash" ]; then
        cp "$svg" "$SVG_DIR/"
        updated=$((updated + 1))
    else
        skipped=$((skipped + 1))
    fi
    echo "$new_hash  $name" >> "$SVG_DIR/.hashes.tmp"
done

mv "$SVG_DIR/.hashes.tmp" "$HASH_FILE" 2>/dev/null || true

echo "=== Lucide sync: +$added added, ~$updated updated, =$skipped skipped (total: $(ls "$SVG_DIR"/*.svg 2>/dev/null | wc -l)) ==="
