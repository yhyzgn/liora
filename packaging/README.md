# Liora Packaging Resources

This directory contains static resources consumed by `cargo xtask package` and
`crates/liora-packager`.

Liora remains a pure Rust + GPUI native application. These files describe package
metadata and platform integration; they do not introduce Tauri or a WebView
runtime.

## Icon set

The icon set is generated from deterministic SVG sources in `packaging/icons/`:

- `liora.*` — main Liora component-library brand logo.
- `liora-gallery.*` — application icon for `liora-gallery` packages.
- `liora-docs.*` — application icon for `liora-docs` packages.

Each icon currently ships as:

- `.svg` source, committed for future edits.
- `.png` 1024x1024 RGBA app icon.
- `.ico` multi-size Windows icon.
- `.icns` multi-size macOS icon.

Regenerate these assets from the SVG sources when the brand mark changes. The
packaging validator checks that all required icon files exist and have the
expected file headers.
