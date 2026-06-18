# Liora 0.1.0 Release Candidate Checklist

This checklist defines the repository-owned readiness gate for the Liora `0.1.0` release-candidate path. It is intentionally stricter than a normal feature-phase checklist because RC work must prove that docs, package metadata, workflows, and canonical apps still agree.

## Scope

- Target version: `0.1.0`.
- Runtime boundary: pure Rust + GPUI native apps only. Do not introduce Tauri, WebView, HTML/CSS/DOM, WASM chart runtimes, or browser shells.
- Canonical apps: `apps/liora-gallery` and `apps/liora-docs`.
- Removed sample-app boundary: do not re-add `examples/minimal-app`, `examples/dashboard-app`, `liora-minimal-app`, or `liora-dashboard-app`; their useful adoption and dogfooding behavior lives in Gallery and Docs.
- Package policy: `LicenseRef-Liora` remains the explicit package/license metadata until the owner replaces it with formal OSS or commercial terms.

## Local RC gates

Run these commands from the repository root before marking an RC phase complete:

```bash
cargo fmt --all --check
cargo check --workspace --all-targets
cargo test --workspace
cargo check -p liora-docs --bin check_snippets
cargo doc --workspace --no-deps
cargo run -p xtask -- package validate
cargo run -p xtask -- package release-readiness
cargo run -p xtask -- package ci --all-apps --format platform-defaults --dry-run --skip-build
cargo run -p xtask -- package install-smoke --all-apps --format platform-defaults --dry-run
git diff --check -- . ':(exclude).omx'
timeout 10s cargo run -p liora-gallery
timeout 10s cargo run -p liora-docs
```

The GUI smoke commands are expected to exit with status `124` under `timeout` after a window starts successfully. Treat early build/runtime failures as RC blockers.

## Metadata and workflow audit

Before a tag release, verify these files agree:

- `Cargo.toml` and every workspace package manifest include repository-owned metadata: `license = "LicenseRef-Liora"`, `repository = "https://github.com/yhyzgn/liora"`, and `publish = false` unless the owner explicitly approves publishing a crate.
- `README.md`, `CHANGELOG.md`, `prompt.md`, `.prompt/P21-release-candidate-readiness.md`, and `.memory/state.md` all describe the same RC boundary.
- `docs/packaging-installer-technical-plan.md` and `apps/liora-docs/content/pages/packaging_workflow.md` keep packaging as a pure native installer pipeline.
- `.github/workflows/ci.yml` remains validation-only and must not publish installers or mutate GitHub Releases.
- `.github/workflows/package.yml` owns preview/release artifacts, grouped changelog generation, raw binary upload for this repository, and `v*` release publication.

## Protected release-only items

These items are intentionally outside local developer machines and ordinary CI dry-runs. They must be executed only in owner-controlled protected environments:

1. Create and push a real `v0.1.0` tag only after the local RC gates pass.
2. Configure macOS signing/notarization inputs (`codesign`, `notarytool`, `stapler`) and Windows signing inputs (`signtool`, timestamp server, certificate secrets).
3. Run real system-level install/uninstall smoke tests for `.deb`, `.rpm`, AppImage, macOS app/dmg, NSIS, and MSI on dedicated runners or test machines.
4. Publish GitHub Release assets only through the protected `package.yml` release path.
5. Change license metadata only when the owner formally chooses the replacement license.

## Completion definition

P21 is complete when the repository contains this checklist, the phase prompt and memory entries are updated, package metadata is explicit, regression tests lock the RC boundaries, the local RC gates pass, and the resulting commit is pushed to `main`.
