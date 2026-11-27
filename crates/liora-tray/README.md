# Liora Tray

Cross-platform system tray facade for Liora applications.

## What it provides

It wraps tray menu state, close-to-tray behavior, tray commands, icon events, and tray control-center integration while keeping application-specific icons owned by the app.

## Quick example

```rust
use liora_tray::TrayConfig;

let _config = TrayConfig::new("my-native-app")
    .tooltip("My Native App");
```

## Recommended usage

- Prefer the top-level `liora` crate for ordinary applications.
- Depend on `liora-tray` directly when you need fine-grained dependency control or are building another Liora crate.
- Keep application UI pure Rust + native GPUI; Liora crates do not require WebView, Tauri, HTML, CSS, or DOM runtime paths.
- `icon_from_rgba` reports exact `InvalidRgba` counts for representable length mismatches. Byte counts exceeding `usize` return `TrayError::BadIcon` wrapping `BadIcon::OsError` with `ErrorKind::InvalidInput`; native backend limits still apply.

## GPUI dependency note

Liora uses the repository-maintained GPUI derivative described in [the source maintenance boundary](../../third_party/gpui/README.md), based on official `zed-industries/zed`. Follow the root README for the exact immutable Liora Git revision and application-root patch. Do not mix it with official GPUI or other forks; SDK manifests must not contain developer-local paths.

## Documentation

- Repository: <https://github.com/yhyzgn/liora>
- Main README: <https://github.com/yhyzgn/liora#readme>
- Chinese README: <https://github.com/yhyzgn/liora/blob/main/README.zh-CN.md>

## License

This crate uses the repository license declared in `LICENSE.md`.

