# Liora 0.1.x Release Candidate Checklist

This checklist defines the repository-owned readiness gate for the Liora 0.1.x release-candidate path. It is intentionally stricter than a normal feature-phase checklist because RC work must prove that docs, package metadata, workflows, and canonical apps still agree.

## Scope

- Target version: the next `0.1.x` tag matching `crates/liora-packager/Cargo.toml`.
- Runtime boundary: pure Rust + GPUI native apps only. Do not introduce Tauri, WebView, HTML/CSS/DOM, WASM chart runtimes, or browser shells.
- Canonical apps: `apps/liora-gallery` and `apps/liora-docs`.
- Removed sample-app boundary: do not re-add `examples/minimal-app`, `examples/dashboard-app`, `liora-minimal-app`, or `liora-dashboard-app`; their useful adoption and dogfooding behavior lives in Gallery and Docs.
- Package policy: SDK crates (`liora`, `liora-theme`, `liora-locales-codegen`, `liora-core`, `liora-icons`, `liora-icons-lucide`, `liora-icons-antd`, `liora-icons-ionic`, `liora-icons-tabler`, `liora-icons-carbon`, `liora-icons-material`, `liora-components`, `liora-tray`, `liora-packager`, `liora-updater`) use the repository license file and publish to crates.io. App packages and the repository-local `xtask` wrapper remain private workspace packages. `LicenseRef-Liora` remains the explicit package/install metadata until the owner replaces it with formal OSS or commercial terms.
- GPUI source policy: public SDK manifests must use official `https://github.com/zed-industries/zed` for local development and Cargo's multiple-location `gpui = 0.2.2` registry fallback for crates.io publishing. Downstream applications must use `[patch.crates-io]` to resolve `gpui` to the matching official Zed revision (currently `2c346f60a76fe3f0367ef924927f50a6efdf5718`). Manifests must not contain renamed GPUI fork dependencies, `[patch."https://github.com/zed-industries/zed"]`, `third_party/zed`, or local path GPUI overrides. The `third_party/zed` snapshot is app-root-only historical verification/reference material and is not part of Liora crates.

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
cargo run -p xtask -- package ci --app gallery --format platform-defaults --dry-run --skip-build
cargo run -p xtask -- package install-smoke --app gallery --format platform-defaults --dry-run
git diff --check -- . ':(exclude).omx'
timeout 10s cargo run -p liora-gallery
timeout 10s cargo run -p liora-docs
```

The GUI smoke commands are expected to exit with status `124` under `timeout` after a window starts successfully. Treat early build/runtime failures as RC blockers.

## Metadata and workflow audit

Before a tag release, verify these files agree:

- `Cargo.toml` and every workspace package manifest include repository-owned metadata. SDK crates (`liora`, `liora-theme`, `liora-locales-codegen`, `liora-core`, `liora-icons`, `liora-icons-lucide`, `liora-icons-antd`, `liora-icons-ionic`, `liora-icons-tabler`, `liora-icons-carbon`, `liora-icons-material`, `liora-components`, `liora-tray`, `liora-packager`, `liora-updater`) use `license-file = "../../LICENSE.md"` and `publish = true`; apps and `xtask` keep `license = "LicenseRef-Liora"` and `publish = false`.
- `README.md`, `CHANGELOG.md`, `prompt.md`, `.prompt/P21-release-candidate-readiness.md`, and `.memory/state.md` all describe the same RC boundary.
- `docs/packaging-installer-technical-plan.md` and `apps/liora-docs/content/pages/packaging_workflow.md` keep packaging as a pure native installer pipeline.
- `.github/workflows/ci.yml` remains validation-only and must not publish installers or mutate GitHub Releases.
- `.github/workflows/package.yml` owns native app preview/release artifacts: cross-platform raw binaries for Docs/Gallery, Gallery installer packages, grouped changelog generation, and `v*` GitHub Release publication.

## Protected release-only items

These items are intentionally outside local developer machines and ordinary CI dry-runs. They must be executed only in owner-controlled protected environments:

1. Create and push a real `vX.Y.Z` tag only after the local RC gates pass and the tag matches `crates/liora-packager/Cargo.toml`.
2. For signed releases, configure macOS signing/notarization inputs (`codesign`, `notarytool`, `stapler`) and Windows signing inputs (`signtool`, timestamp server, certificate secrets), then set `LIORA_REQUIRE_SIGNING=true`; unsigned first-release builds are allowed when that variable is absent.
3. Run real system-level install/uninstall smoke tests for `.deb`, `.rpm`, AppImage, macOS app/dmg, NSIS, and MSI on dedicated runners or test machines.
4. Publish GitHub Release app assets only through the protected `package.yml` release path; publish crates.io SDK crates only through `release-sdk.yml` with `CRATES_IO_TOKEN`.
5. Change license metadata only when the owner formally chooses the replacement license.

## Completion definition

The RC gate is complete when the repository contains this checklist, the phase prompt and memory entries are updated, package metadata is explicit, regression tests lock the RC boundaries, the local RC gates pass, and the resulting commit is pushed to `main`.
