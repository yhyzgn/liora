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
- Linux/FreeBSD GTK startup hints resolve theme name and dark preference independently of configuration line order: `true` requests a dark variant; `false` does not force an explicitly dark theme back to light.
- System appearance resolves from GPUI's explicit initialized snapshot, not whether an appearance callback happened. Pending Linux snapshots retain synchronous startup hints; initialized Light/Dark wins even before any observer was attached. Fixed modes remain fixed, and switching back to System uses the current initialized window snapshot.
- Locale directory loads commit only after every file has been read and parsed; a failed runtime reload preserves translations, locale version, and the remembered resource directory. Successful loads replace each loaded locale, not individual keys, and retain locales absent from the directory.
- `stable_unique_id` borrows its prefix during keyed-state initialization instead of copying it on every render; generated identities remain owned, with unchanged window/element namespace and consecutive-frame lifetimes.

## GPUI dependency note

Liora uses a repository-maintained GPUI derivative of official `zed-industries/zed` revision `f0d896f2c18a543335eb155f729d36b624695591` for explicit appearance readiness. Use the same immutable Liora Git source/revision for `gpui` and `gpui_platform`, as recorded in the root manifest and README. Registry consumers still need their own root `gpui` patch; local paths and unrelated forks are not accepted. Provenance and rollback: [maintained GPUI source](../../third_party/gpui/README.md).

## Documentation

- Repository: <https://github.com/yhyzgn/liora>
- Main README: <https://github.com/yhyzgn/liora#readme>
- Chinese README: <https://github.com/yhyzgn/liora/blob/main/README.zh-CN.md>

## License

This crate uses the repository license declared in `LICENSE.md`.

