# Liora Core

Core runtime state, initialization, themes, locales, fonts, and shared GPUI helpers.

## What it provides

Use this crate when you are building lower-level integrations or a custom component crate and do not want the full `liora` facade. Most applications should start with `liora` unless they need fine-grained dependencies.

## Quick example

```rust
use liora_core::{ThemeMode, init_liora_with_mode};

fn setup(cx: &mut gpui::App) {
    init_liora_with_mode(cx, ThemeMode::System);
}
```

## Recommended usage

- Prefer the top-level `liora` crate for ordinary applications.
- Depend on `liora-core` directly when you need fine-grained dependency control or are building another Liora crate.
- Keep application UI pure Rust + native GPUI; Liora crates do not require WebView, Tauri, HTML, CSS, or DOM runtime paths.
- `apply_theme_mode` schedules all open GPUI windows to redraw; System appearance synchronization does the same when the shared theme changes.
- Locale directory loads commit only after every file has been read and parsed; a failed runtime reload preserves translations, locale version, and the remembered resource directory. Successful loads replace each loaded locale, not individual keys, and retain locales absent from the directory.

## GPUI dependency note

Liora is developed against the official `zed-industries/zed` GPUI source. When building an application, follow the repository README for the exact GPUI git revision and patching guidance. Do not use unofficial GPUI forks unless the application owner explicitly accepts that risk.

## Documentation

- Repository: <https://github.com/yhyzgn/liora>
- Main README: <https://github.com/yhyzgn/liora#readme>
- Chinese README: <https://github.com/yhyzgn/liora/blob/main/README.zh-CN.md>

## License

This crate uses the repository license declared in `LICENSE.md`.

