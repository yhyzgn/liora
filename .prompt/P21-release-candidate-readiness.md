# P21 — Release Candidate Readiness

## Status

✅ Complete — 2026-06-18

## Goal

Close the repository-owned release-candidate gap for Liora `0.1.0` without changing Liora's runtime architecture. This phase does not ship a public release; it makes the repository ready for an owner-controlled protected release path.

## Non-negotiable boundaries

- Liora remains pure Rust + GPUI native.
- Do not introduce Tauri, WebView, HTML/CSS/DOM, browser runtime, or web chart/runtime shells.
- Do not re-add standalone `examples/minimal-app` or `examples/dashboard-app`; Gallery and Docs remain the canonical adoption/dogfooding apps.
- Keep sample/business screens and mock dashboard models out of `liora-components`.
- Keep `.omx/**` out of commits.

## Delivered scope

- Added `docs/release-candidate-checklist.md` as the RC source of truth.
- Refreshed README/CHANGELOG/prompt/memory so P12–P21 describe current reality instead of old sample-app or license TODOs.
- Added explicit package metadata to workspace package manifests: SDK crates are crates.io publishable with `license-file = "../../LICENSE.md"`; app/automation crates remain `publish = false` with `LicenseRef-Liora` metadata.
- Locked the RC boundary with docs regression tests covering commands, metadata, app boundaries, package workflow roles, SDK publishing, and absence of removed sample apps.

## Required local verification

```bash
cargo fmt --all --check
cargo check --workspace --all-targets
cargo test --workspace
cargo check -p liora-docs --bin check_snippets
cargo doc --workspace --no-deps
cargo run -p xtask -- package validate
cargo run -p xtask -- package release-readiness
cargo run -p xtask -- package ci --app gallery --format platform-defaults --dry-run --skip-build
cargo run -p xtask -- package install-smoke --app gallery --format platform-defaults --dry-run
git diff --check -- . ':(exclude).omx'
timeout 10s cargo run -p liora-gallery
timeout 10s cargo run -p liora-docs
```

`timeout 10s` GUI smoke exits with status `124` are expected after the native window starts.

## Protected follow-up

Only the owner/protected release environment should run the real `v0.1.0` tag release, crates.io SDK publication through `release-sdk.yml`, macOS notarization, Windows signing, destructive system install/uninstall smoke, and GitHub Release publication.
