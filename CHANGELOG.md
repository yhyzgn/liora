# Changelog

Liora currently records detailed implementation history in `.memory/sessions.md` and phase-level plans under `.prompt/`.

## Unreleased

### Added

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
- Package/license metadata currently uses `LicenseRef-Liora`; do not assume OSS terms until the owner replaces the policy in `LICENSE.md` and package metadata.
- Formal public releases remain gated by owner-controlled signing/notarization credentials, protected release environments, real system-level install/uninstall smoke tests, and a matching `vX.Y.Z` GitHub Release tag.
