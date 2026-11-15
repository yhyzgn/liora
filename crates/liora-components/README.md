# Liora Components

Native GPUI component library for desktop Rust applications.

## What it provides

It provides reusable controls such as Button, Input, Select, Table, Grid, Shell, Sidebar, TitleBar, charts, overlays, menus, text primitives, and virtualized data views. Components are native Rust + GPUI, not WebView/Tauri/HTML runtime widgets.

## Quick example

```rust
use liora_components::{Button, Input, init_liora};

fn setup(cx: &mut gpui::App) {
    init_liora(cx);
    let _button = Button::new("Save").primary();
    let _input = Input::new().placeholder("Search");
}
```

## Recommended usage

- Prefer the top-level `liora` crate for ordinary applications.
- Depend on `liora-components` directly when you need fine-grained dependency control or are building another Liora crate.
- Keep application UI pure Rust + native GPUI; Liora crates do not require WebView, Tauri, HTML, CSS, or DOM runtime paths.

## PDF reader and reusable input controls

- `PdfDocument::from_bytes(title, bytes: Vec<u8>) -> Result<PdfDocument, PdfDecodeError>` parses and rasterizes real PDFs through Hayro, a pure-Rust CPU backend. Call it on a background worker, then create `PdfReader` or replace its document on the UI thread. Input is limited to 64 MiB, retained page/thumbnail rasters to 128 MiB, and the longest page raster side to 2048 pixels. This is eager rendering, not a total-process memory cap or an on-demand page cache.
- `PdfReader` provides native page navigation, layouts, zoom, rotation, thumbnails and search/tool events. The host owns file I/O, downloads, search indexing, text selection and annotation persistence. Tools are state/events only, not a drawing or PDF-writeback engine. Hosts may also supply their own `PdfPage` images.
- `FindBar` owns native query input and `QueryChanged`/`Previous`/`Next`/`Closed` events; the host computes matches and supplies match status. Focused Enter/Shift-Enter/Escape request navigation/close while active IME composition and modified shortcuts remain with the input.
- `CandidateBar` displays host-supplied candidates (default 8 per page, configurable from 1 to 9). Explicit focus enables arrows, PageUp/PageDown, 1–9, Space for candidate commit, Enter for literal commit, and Escape for cancel. Selection/page events use zero-based indices; the host performs text mutation and clears composition. No input-method engine, dictionary, language model or platform IME integration is bundled.

Create the reader only after a successful decode; surface errors in the host UI and retain the previous document on failure. For integration examples, see the repository documentation linked below.

## GPUI dependency note

Liora is developed against the official `zed-industries/zed` GPUI source. When building an application, follow the repository README for the exact GPUI git revision and patching guidance. Do not use unofficial GPUI forks unless the application owner explicitly accepts that risk.

## Documentation

- Repository: <https://github.com/yhyzgn/liora>
- Main README: <https://github.com/yhyzgn/liora#readme>
- Chinese README: <https://github.com/yhyzgn/liora/blob/main/README.zh-CN.md>

## License

This crate uses the repository license declared in `LICENSE.md`.

