# Liora Carbon Icons

Bundled IBM Carbon SVG icon paths for Liora.

## What it provides

This crate exposes a typed `IconName` enum for the IBM Carbon icon pack. Icons render through `liora-icons::Icon` and GPUI's asset system. Application build scripts can use `liora-icons-optimizer` to package only the icons actually used.

## Quick example

```rust
use liora_icons::Icon;
use liora_icons_carbon::IconName;

let icon_name = IconName::CheckmarkFilled;
let _icon = Icon::new(icon_name);
```

## Recommended usage

- Prefer the top-level `liora` crate for ordinary applications.
- Depend on `liora-icons-carbon` directly when you need fine-grained dependency control or are building another Liora crate.
- Keep application UI pure Rust + native GPUI; Liora crates do not require WebView, Tauri, HTML, CSS, or DOM runtime paths.

## GPUI dependency note

Liora is developed against the official `zed-industries/zed` GPUI source. When building an application, follow the repository README for the exact GPUI git revision and patching guidance. Do not use unofficial GPUI forks unless the application owner explicitly accepts that risk.

## Documentation

- Repository: <https://github.com/yhyzgn/liora>
- Main README: <https://github.com/yhyzgn/liora#readme>
- Chinese README: <https://github.com/yhyzgn/liora/blob/main/README.zh-CN.md>

## License

This crate uses the repository license declared in `LICENSE.md`.

