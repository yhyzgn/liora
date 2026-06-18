#!/usr/bin/env bash
set -euo pipefail

cargo fmt
cargo check
cargo run
