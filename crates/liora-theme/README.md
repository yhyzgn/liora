# Liora Theme

Design tokens and semantic theme data for native GPUI components.

## What it provides

This crate contains the theme primitives consumed by `liora-core`, `liora-components`, and application code that needs stable semantic colors, radii, shadows, and interaction states.

## Quick example

```rust
use liora_theme::Theme;

let light = Theme::light();
let dark = Theme::dark();
assert_ne!(light.neutral.body, dark.neutral.body);
```

## Recommended usage

- Prefer the top-level `liora` crate for ordinary applications.
- Depend on `liora-theme` directly when you need fine-grained dependency control or are building another Liora crate.
- Keep application UI pure Rust + native GPUI; Liora crates do not require WebView, Tauri, HTML, CSS, or DOM runtime paths.

## GPUI dependency note

Liora uses the repository-maintained GPUI derivative described in [the source maintenance boundary](../../third_party/gpui/README.md), based on official `zed-industries/zed`. Follow the root README for the exact immutable Liora Git revision and application-root patch. Do not mix it with official GPUI or other forks; SDK manifests must not contain developer-local paths.

## Documentation

- Repository: <https://github.com/yhyzgn/liora>
- Main README: <https://github.com/yhyzgn/liora#readme>
- Chinese README: <https://github.com/yhyzgn/liora/blob/main/README.zh-CN.md>

## License

This crate uses the repository license declared in `LICENSE.md`.

