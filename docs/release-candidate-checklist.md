# Liora 0.6.x Release Candidate Checklist

This checklist defines the repository-owned readiness gate for the Liora 0.6.x release-candidate path. It is intentionally stricter than a normal feature-phase checklist because RC work must prove that docs, package metadata, workflows, and canonical apps still agree.

## Scope

- Target version: `0.6.1`, matching every workspace package manifest and `crates/liora-packager/Cargo.toml`.
- Runtime boundary: Gallery/Docs and default SDK components remain pure Rust + native GPUI; the standalone `liora-webview` crate is an explicit opt-in exception, not a default dependency or app runtime. Linux supports only the verified X11/XWayland path; macOS/Windows native behavior is unverified.
- Canonical apps: `apps/liora-gallery` and `apps/liora-docs`.
- Removed sample-app boundary: do not re-add `examples/minimal-app`, `examples/dashboard-app`, `liora-minimal-app`, or `liora-dashboard-app`; their useful adoption and dogfooding behavior lives in Gallery and Docs.
- Package policy: SDK crates (`liora`, `liora-theme`, `liora-icons-optimizer`, `liora-core`, `liora-icons`, `liora-icons-lucide`, `liora-icons-antd`, `liora-icons-ionic`, `liora-icons-tabler`, `liora-icons-carbon`, `liora-icons-material`, `liora-components`, `liora-tray`, `liora-packager`, `liora-updater`, `liora-webview`) use the repository license file and publish to crates.io. App packages and the repository-local `xtask` wrapper remain private workspace packages. `LicenseRef-Liora` remains the explicit package/install metadata until the owner replaces it with formal OSS or commercial terms.
- GPUI source policy: public SDK manifests inherit the exact immutable `https://github.com/yhyzgn/liora` source/revision from the root workspace, with Cargo's multiple-location `gpui = 0.2.2` registry fallback. This explicitly authorized derivative retains official Zed provenance; it is not an official Zed revision. Downstream apps must patch registry `gpui` to that same Liora source SHA and use matching `gpui_platform` (use the pin belonging to the SDK version being consumed). No renamed forks, local GPUI paths or extra root source overrides are allowed. `third_party/gpui` is the maintained nine-crate source closure; `third_party/zed` remains inert historical material. See [maintenance boundary](../third_party/gpui/README.md).
- Explicit leaf exception to the former blanket root patch ban: only official `accesskit_atspi_common 0.19.1` plus the two upstream Busy/Disabled backports at `third_party/accesskit_atspi_common` may be selected by the workspace root. All other crate/source overrides remain forbidden. The leaf is excluded from workspace members and SDK publication targets; root patches are not inherited by downstream SDK hosts. Run `python scripts/audit_sdk_policy.py` and `python scripts/test_audit_sdk_policy.py -v` before integration; ordinary CI and the SDK release audit invoke the same policy. [Source provenance, consumer setup and limits](accesskit-atspi-backport.md) remain required; no tag/signing/publication is authorized by this exception.

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
cargo package -p liora-webview --no-verify
cargo run -p xtask -- package ci --app gallery --format platform-defaults --dry-run --skip-build
cargo run -p xtask -- package install-smoke --app gallery --format platform-defaults --dry-run
git diff --check -- . ':(exclude).omx'
timeout 10s cargo run -p liora-gallery
timeout 10s cargo run -p liora-docs
```

The GUI smoke commands are expected to exit with status `124` under `timeout` after a window starts successfully. Treat early build/runtime failures as RC blockers.

## Metadata and workflow audit

Before a tag release, verify these files agree:

- `Cargo.toml` and every workspace package manifest include repository-owned metadata. SDK crates (`liora`, `liora-theme`, `liora-icons-optimizer`, `liora-core`, `liora-icons`, `liora-icons-lucide`, `liora-icons-antd`, `liora-icons-ionic`, `liora-icons-tabler`, `liora-icons-carbon`, `liora-icons-material`, `liora-components`, `liora-tray`, `liora-packager`, `liora-updater`, `liora-webview`) use `license-file = "../../LICENSE.md"` and `publish = true`; apps and `xtask` keep `license = "LicenseRef-Liora"` and `publish = false`.
- `README.md`, `README.zh-CN.md`, `CHANGELOG.md`, `crates/liora-webview/README.md`, `AGENTS.md`, `.context/README.md`, `.context/plans/010_gallery_performance_webview_release_20261116.md`, and `.context/tasks/010_gallery_performance_webview_release_20261116.md` describe the same release boundary.
- `docs/packaging-installer-technical-plan.md` and `apps/liora-docs/content/pages/packaging_workflow.md` keep packaging as a pure native installer pipeline.
- `.github/workflows/ci.yml` remains validation-only and must not publish installers or mutate GitHub Releases.
- `.github/workflows/package.yml` owns native app preview/release artifacts: cross-platform raw binaries for Docs/Gallery, Gallery installer packages, grouped changelog generation, and `v*` GitHub Release publication.
- macOS packaging and runtime verification use `macos-26`; packaging selects Xcode 26.0.1 and `MACOSX_DEPLOYMENT_TARGET=12.0`. Compatibility on macOS 12 still requires dedicated runtime verification.
- `.github/workflows/release-sdk.yml` packages and publishes the SDK crate set, including standalone `liora-webview`, in dependency order through the protected `crates-io` environment.

## Protected release-only items

These items are intentionally outside local developer machines and ordinary CI dry-runs. They must be executed only in owner-controlled protected environments:

1. Create and push a real `vX.Y.Z` tag only after the local RC gates pass and the tag matches `crates/liora-packager/Cargo.toml`.
2. Always configure the protected `LIORA_RPM_SIGNING_PRIVATE_KEY` secret and independently provisioned `LIORA_RPM_SIGNING_FINGERPRINT` repository variable (full OpenPGP v4 fingerprint) for RPM releases. The workflow binds both font variants to this fingerprint, verifies signatures against the isolated publisher keyring, and publishes the public key/fingerprint; no unsigned-RPM override is allowed on release tags. Owners must also independently distribute the full fingerprint before users import the key. macOS signing/notarization (`codesign`, `notarytool`, `stapler`) and Windows signing (`signtool`, timestamp server, certificate secrets) retain their opt-in `LIORA_REQUIRE_SIGNING=true` gate.
3. Run real system-level install/uninstall smoke tests for `.deb`, `.rpm`, AppImage, macOS app/dmg, NSIS, and MSI on dedicated runners or test machines.
4. Publish GitHub Release app assets only through the protected `package.yml` release path; publish crates.io SDK crates only through `release-sdk.yml` with `CRATES_IO_TOKEN`.
5. Change license metadata only when the owner formally chooses the replacement license.

## Completion definition

The RC gate is complete when the repository contains this checklist, AGENTS.md and `.context/` entries are updated, package metadata is explicit, regression tests lock the RC boundaries, the local RC gates pass, and the resulting commit is pushed to `main`.
