# Liora Packaging Resources

This directory contains static resources consumed by `cargo xtask package` and
`crates/liora-packager`.

Liora remains a pure Rust + GPUI native application. These files describe package
metadata and platform integration; they do not introduce Tauri or a WebView
runtime.

## Icon set

The icon set is generated from deterministic SVG sources in `packaging/icons/`:

- `liora.*` — main Liora component-library brand logo.
- `liora-gallery.*` — application icon for `liora-gallery` packages; component-grid mark.
- `liora-docs.*` — application icon for `liora-docs` packages; document/fold mark.

Each icon currently ships as:

- `.svg` source, committed for future edits.
- `.png` 1024x1024 RGBA app icon.
- `.ico` multi-size Windows icon.
- `.icns` multi-size macOS icon.

Regenerate these assets from the SVG sources when the brand mark changes. The
packaging validator checks that all required icon files exist and have the
expected file headers. Runtime tray icons are app-owned assets under
`apps/liora-gallery/assets/tray-icons/` and `apps/liora-docs/assets/tray-icons/`.
Window header marks live under each app's `assets/status-icons/` directory, and
bottom status-bar state icons live under each app's `assets/status-bar-icons/`
directory. These assets intentionally do not live inside the reusable
`liora-tray` SDK crate.
