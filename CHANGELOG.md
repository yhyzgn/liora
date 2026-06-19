# Changelog

## 0.1.5 - 2026-06-19

- Removed Gallery/Docs-specific bundled tray icon assets from the reusable `liora-tray` SDK crate.
- Moved runtime tray icons into app-owned `apps/liora-gallery/assets/tray-icons/` and `apps/liora-docs/assets/tray-icons/` assets.
- Redesigned Gallery and Docs desktop/taskbar package icons as distinct Liora-branded app marks.
- Registered Gallery and Docs user-scoped Linux desktop identities at startup so Wayland titlebar/taskbar icons resolve correctly during `cargo run`, not only after installing packages.
- Added separate app-owned window header icons under `assets/status-icons/` and status-bar state icons under `assets/status-bar-icons/`, then rendered them in Gallery/Docs shells.
- Updated tray documentation and compile-checked snippets to demonstrate app-owned icon loading with `icon_from_png_bytes`.

## 0.1.4 - 2026-06-19

- Refactored `liora-updater` into a reusable GitHub Release updater SDK for any native app, with generic `AssetSelector`, `UpdateRequest`, and `PreparedUpdate` APIs.
- Kept Liora Gallery/Docs release selection as small presets on top of the generic updater instead of hard-coding those apps into the core flow.
- Added configurable checksum asset names and documented custom updater integration for downstream applications.
- Updated Gallery and Docs About/update flows to use the generic updater request pipeline.

## 0.1.3 - 2026-06-19

- Added publishable `liora-updater` crate and default `liora::updater` facade export for GitHub Release checks, platform asset selection, cached downloads, SHA-256 verification, and explicit install plans.
- Added About / update panels to Liora Gallery and Liora Docs with startup auto-check/download behavior and user-visible install actions.
- Updated all workspace package versions and internal Liora dependency versions to `0.1.3`.
- Updated release/CI workflows to Node 24 action generations and included `liora-updater` in SDK crates publishing.
- Cleaned Rust warnings in component/app async update paths and form-control listeners.

Liora currently records detailed implementation history in `.memory/sessions.md` and phase-level plans under `.prompt/`.

## Unreleased

### Fixed

- 0.1.2 dependency correction: switched every published Liora crate and maintained app from the non-official `open-gpui` packages to the official Zed crates.io `gpui = "0.2.2"`, removed the separate `gpui_platform` dependency, and updated app bootstraps to use `gpui::Application::new()`.
- Release and documentation guardrails now assert that SDK packages use official Zed GPUI only, with no renamed fork dependency, no `[patch]` section, and no bundled `third_party/zed` path dependency.

### Added

- 0.1.1 release refinement: added the one-stop `liora` facade crate, made `liora-packager` publishable for reusable packaging helpers, kept `xtask` repository-local, and shortened GitHub Release assets to distributable files plus one `SHA256SUMS.txt`.
- Release pipeline split: dedicated `.github/workflows/release-sdk.yml` publishes SDK crates to crates.io using `CRATES_IO_TOKEN`; native app packaging now releases Docs as cross-platform raw executables and Gallery as raw executables plus planned installer formats.
- P21 release-candidate readiness: `docs/release-candidate-checklist.md`, explicit package metadata audit coverage, refreshed README/CHANGELOG/prompt/memory state, and regression tests for current release boundaries.
- P20 theme and interaction polish: System/Light/Dark theme mode, tokenized overlay/mask paths, custom window-frame controls, Theme Gallery page, and native Docs Theme page.
- P19 dashboard state/data-flow guidance: app-layer state, filter, refresh, empty/degraded/loading branches, and Dashboard State docs folded into Gallery/Docs.
- P18 dashboard polish/API ergonomics: Gallery shell dogfooding for search/filtering, theme switching, toast feedback, and Dashboard Patterns docs without exporting sample-only helpers.
- P17 dashboard dogfooding: realistic composition lessons were folded into Gallery and Docs; standalone dashboard app was intentionally removed.
- P16 adoption readiness: root README, CONTRIBUTING, crate-level Rustdoc entrypoints, native Docs Adoption Guide, and compile-checked adoption examples. Minimal app guidance is folded into Gallery/Docs instead of a standalone app.
- P15 quality hardening: workspace CI gates, docs/snippet checks, API consistency, theme/overlay cleanup, keyboard behavior, and CodeBlock/cache performance hardening.
- P14 deferred advanced controls: Carousel, Calendar, InputTag, Mention, Watermark, TreeSelect, Tour, VirtualizedTable, and VirtualizedTree.
- P13 component expansion: QR code, CodeEditor, SignalMeter, HeatBar, SegmentRatioBar, HorizontalList drag behavior, Timer, Label, Operation, and in-place enhancements for charts, Progress, Button, Tag, Radio, and Checkbox.
- P12 packaging/release readiness: `liora-packager`, `xtask package`, platform packaging resources, CI installer pipeline, grouped changelog generation, raw binary upload policy, signing policy, and release-readiness gate.

### Changed

- Preferred Liora app initialization now follows the operating system by default with `init_liora_with_mode(cx, ThemeMode::System)`. Fixed explicit themes such as `init_liora(cx, Theme::light())` remain supported for compatibility and product-specific policies.
- Gallery and Docs are the canonical maintained app surfaces. `examples/minimal-app`, `examples/dashboard-app`, `liora-minimal-app`, and `liora-dashboard-app` must not be reintroduced as separate products.

### Notes

- Liora remains pure Rust + GPUI native.
- Public SDK crates use the repository license file for crates.io publication; installer/app metadata continues to use `LicenseRef-Liora` until the owner replaces the policy in `LICENSE.md` and package metadata.
- Formal app releases remain gated by owner-controlled signing/notarization credentials, protected release environments, real system-level install/uninstall smoke tests, and a matching `vX.Y.Z` GitHub Release tag.
