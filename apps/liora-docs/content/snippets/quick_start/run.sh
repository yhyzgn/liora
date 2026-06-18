#!/usr/bin/env bash
# Run Liora's native component gallery.
cargo run -p liora-gallery

# Run the standalone native documentation application.
cargo run -p liora-docs

# Check both main applications without launching windows.
cargo check -p liora-gallery -p liora-docs
