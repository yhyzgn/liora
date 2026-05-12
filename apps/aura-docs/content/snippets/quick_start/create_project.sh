#!/usr/bin/env bash
set -euo pipefail

cargo new aura-native-demo --bin
cd aura-native-demo

# Optional but recommended for app projects with multiple crates later.
mkdir -p .cargo
cat > .cargo/config.toml <<'CONFIG'
[build]
# Put project-specific build config here when needed.
CONFIG
