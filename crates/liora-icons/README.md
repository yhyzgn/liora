# Liora Icons

Native GPUI icon primitive and shared icon asset loader.

## What it provides

This crate renders SVG icon paths through GPUI assets and provides the `IntoIconPath` trait used by bundled icon-pack crates. It also supports inline SVG strings and caller-owned SVG files.

## Quick example

```rust
use liora_icons::{Icon, IconAssetSource};

fn main() {
    gpui_platform::application()
        .with_assets(IconAssetSource)
        .run(|_cx| {
            let _icon = Icon::new("assets/icons/product.svg").size_lg();
        });
}
```

## Recommended usage

- Prefer the top-level `liora` crate for ordinary applications.
- Depend on `liora-icons` directly when you need fine-grained dependency control or are building another Liora crate.
- Keep application UI pure Rust + native GPUI; Liora crates do not require WebView, Tauri, HTML, CSS, or DOM runtime paths.

## GPUI dependency note

Liora uses the repository-maintained GPUI derivative described in [the source maintenance boundary](../../third_party/gpui/README.md), based on official `zed-industries/zed`. Follow the root README for the exact immutable Liora Git revision and application-root patch. Do not mix it with official GPUI or other forks; SDK manifests must not contain developer-local paths.

## Documentation

- Repository: <https://github.com/yhyzgn/liora>
- Main README: <https://github.com/yhyzgn/liora#readme>
- Chinese README: <https://github.com/yhyzgn/liora/blob/main/README.zh-CN.md>

## License

This crate uses the repository license declared in `LICENSE.md`.

