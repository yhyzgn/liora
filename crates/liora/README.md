# Liora

One-stop SDK facade for native Rust + GPUI applications.

## What it provides

`liora` re-exports the maintained public Liora SDK surface so application crates can depend on one package instead of wiring every module separately. It includes core initialization, theme tokens, components, icon primitives, icon libraries, tray helpers, updater planning, and optional packaging helpers.

## Quick example

```rust
use liora::{ThemeMode, init_liora};
use liora::components::Button;

fn setup(cx: &mut gpui::App) {
    init_liora(cx);
    let _mode = ThemeMode::System;
    let _button = Button::new("Hello Liora");
}
```

## Recommended usage

- Prefer the top-level `liora` crate for ordinary applications.
- Depend on `liora` directly when you need fine-grained dependency control or are building another Liora crate.
- Keep application UI pure Rust + native GPUI; Liora crates do not require WebView, Tauri, HTML, CSS, or DOM runtime paths.
- Web content is available only through the separate opt-in `liora-webview` crate; it is not re-exported or included in the facade dependencies.

## GPUI dependency note

Liora is developed against the official `zed-industries/zed` GPUI source. When building an application, follow the repository README for the exact GPUI git revision and patching guidance. Do not use unofficial GPUI forks unless the application owner explicitly accepts that risk.

## Documentation

- Repository: <https://github.com/yhyzgn/liora>
- Main README: <https://github.com/yhyzgn/liora#readme>
- Chinese README: <https://github.com/yhyzgn/liora/blob/main/README.zh-CN.md>

## License

This crate uses the repository license declared in `LICENSE.md`.

