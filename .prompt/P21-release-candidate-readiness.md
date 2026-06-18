# P21 — Release Candidate Readiness

## Status

✅ Complete — 2026-06-18

## Goal

Close the repository-owned release-candidate gap for Aura `0.1.0` without changing Aura's runtime architecture. This phase does not ship a public release; it makes the repository ready for an owner-controlled protected release path.

## Non-negotiable boundaries

- Aura remains pure Rust + GPUI native.
- Do not introduce Tauri, WebView, HTML/CSS/DOM, browser runtime, or web chart/runtime shells.
- Do not re-add standalone `examples/minimal-app` or `examples/dashboard-app`; Gallery and Docs remain the canonical adoption/dogfooding apps.
- Keep sample/business screens and mock dashboard models out of `aura-components`.
- Keep `.omx/**` out of commits.

## Delivered scope

- Added `docs/release-candidate-checklist.md` as the RC source of truth.
- Refreshed README/CHANGELOG/prompt/memory so P12–P21 describe current reality instead of old sample-app or license TODOs.
- Added explicit package metadata to workspace package manifests: `LicenseRef-Aura`, repository URL, descriptions, and `publish = false`.
- Locked the RC boundary with docs regression tests covering commands, metadata, app boundaries, package workflow roles, and absence of removed sample apps.

## Required local verification

```bash
cargo fmt --all --check
cargo check --workspace --all-targets
cargo test --workspace
cargo check -p aura-docs --bin check_snippets
cargo doc --workspace --no-deps
cargo run -p xtask -- package validate
cargo run -p xtask -- package release-readiness
cargo run -p xtask -- package ci --all-apps --format platform-defaults --dry-run --skip-build
cargo run -p xtask -- package install-smoke --all-apps --format platform-defaults --dry-run
git diff --check -- . ':(exclude).omx'
timeout 10s cargo run -p aura-gallery
timeout 10s cargo run -p aura-docs
```

`timeout 10s` GUI smoke exits with status `124` are expected after the native window starts.

## Protected follow-up

Only the owner/protected release environment should run the real `v0.1.0` tag release, macOS notarization, Windows signing, destructive system install/uninstall smoke, and GitHub Release publication.
