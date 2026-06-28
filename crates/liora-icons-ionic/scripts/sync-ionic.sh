#!/usr/bin/env bash
# Sync Ionicons SVG icons from the official upstream repository.
#
#   ./scripts/sync-ionic.sh         # incremental cache refresh
#   ./scripts/sync-ionic.sh --full  # force overwriting copied SVGs

set -euo pipefail

UPSTREAM_REPO='https://github.com/ionic-team/ionicons.git'
UPSTREAM_PATH='src/svg'
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
CRATE_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
PROJECT_DIR="$(cd "$CRATE_DIR/../.." && pwd)"
CACHE_DIR="$PROJECT_DIR/target/ionic-icons-cache"
SVG_DIR="$CRATE_DIR/assets/svgs"
FULL="${1:-}"
CACHE_STAMP="$CACHE_DIR/.stamp"
CACHE_TTL=$((24 * 3600))

mkdir -p "$SVG_DIR" "$CACHE_DIR"
need_fetch=true
if [ -d "$CACHE_DIR/.git" ] && [ "$FULL" != "--full" ] && [ -f "$CACHE_STAMP" ]; then
  now=$(date +%s)
  last=$(cat "$CACHE_STAMP")
  if [ $((now - last)) -lt $CACHE_TTL ]; then need_fetch=false; fi
fi

if $need_fetch; then
  if [ -d "$CACHE_DIR/.git" ]; then
    git -C "$CACHE_DIR" fetch --depth 1 origin HEAD
  else
    git clone --depth 1 --filter=blob:none --sparse "$UPSTREAM_REPO" "$CACHE_DIR"
  fi
  git -C "$CACHE_DIR" sparse-checkout set "$UPSTREAM_PATH"
  git -C "$CACHE_DIR" reset --hard FETCH_HEAD >/dev/null 2>&1 || git -C "$CACHE_DIR" reset --hard HEAD >/dev/null
  date +%s > "$CACHE_STAMP"
fi

python3 "$SCRIPT_DIR/sync_ionic.py" "$CACHE_DIR/$UPSTREAM_PATH" "$SVG_DIR" "$FULL"
