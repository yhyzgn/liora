# Liora Icons Optimizer

Build-time optimizer for Liora icon bundles.

## What it provides

Add this crate as a build dependency in applications that use bundled `IconName` enums. It scans Rust/content sources, copies the referenced SVG files into the app icon bundle, and produces a report for packaging diagnostics.
An explicit `IconName::all()` request intentionally includes the full icon pack.

## Quick example

```rust
fn main() {
    liora_icons_optimizer::Optimizer::new()
        .scan_dir("src")
        .bundle_auto()
        .run();
}
```

## Recommended usage

- Prefer the top-level `liora` crate for ordinary applications.
- Depend on `liora-icons-optimizer` directly when you need fine-grained dependency control or are building another Liora crate.
- Keep application UI pure Rust + native GPUI; Liora crates do not require WebView, Tauri, HTML, CSS, or DOM runtime paths.

## GPUI dependency note

Liora uses the repository-maintained GPUI derivative described in [the source maintenance boundary](../../third_party/gpui/README.md), based on official `zed-industries/zed`. Follow the root README for the exact immutable Liora Git revision and application-root patch. Do not mix it with official GPUI or other forks; SDK manifests must not contain developer-local paths.

## Documentation

- Repository: <https://github.com/yhyzgn/liora>
- Main README: <https://github.com/yhyzgn/liora#readme>
- Chinese README: <https://github.com/yhyzgn/liora/blob/main/README.zh-CN.md>

## License

This crate uses the repository license declared in `LICENSE.md`.

