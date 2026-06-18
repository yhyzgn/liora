# Liora

Liora is a pure Rust + GPUI native component library for desktop applications. It provides enterprise-style UI components, a native Gallery app, a native Docs app, tray support, charting widgets, code display/editing primitives, and an installer packaging pipeline for Liora's own GPUI apps.

Liora applications stay native: no Tauri runtime, no WebView, no HTML/CSS/DOM application shell, and no browser runtime architecture.

## What is included

- `liora-components` — buttons, forms, overlays, data display, navigation, charts, virtualized lists/tables/trees, code blocks, QR code, timer, tray-related UI demos, and more.
- `liora-core` — global Liora config, theme initialization, portal/popper state, unique id helpers.
- `liora-theme` — light/dark design tokens and semantic component sizes/variants.
- `liora-icons` / `liora-icons-lucide` — native GPUI icon rendering and bundled Lucide icon paths.
- `liora-tray` — cross-platform system tray facade for Liora GPUI applications.
- `liora-packager` + `xtask` — native installer/package metadata, validation, manifests, checksums, and release-readiness gates.
- `apps/liora-gallery` — native component demo app and dogfooding shell for search/filtering, theme switching, tray behavior, toasts, and component composition.
- `apps/liora-docs` — native documentation app and adoption guide.

## Quick start

Install Rust stable and the platform GPUI dependencies documented in the native Docs app Quick Start page. On Linux, the common development packages include GTK3, Wayland/X11, xkbcommon, fontconfig/freetype, Vulkan, ALSA, and pkg-config.

Run the Gallery:

```bash
cargo run -p liora-gallery
```

Run the Docs app:

```bash
cargo run -p liora-docs
```

Gallery and Docs are the canonical native adoption surfaces. Linux app crates enable GPUI `wayland`/`x11` features explicitly. If a native window does not appear, run from a graphical session and check `DISPLAY` or `WAYLAND_DISPLAY`.

## Minimal application shape

A GPUI app using Liora should initialize the Liora theme/config, initialize global services that the selected components need, register component key bindings, then open a GPUI window:

```rust
use liora_core::{ThemeMode, init_liora_with_mode};
use gpui::App;

fn main() {
    gpui_platform::application().run(|cx: &mut App| {
        init_liora_with_mode(cx, ThemeMode::System);
        liora_components::MessageManager::init(cx);
        liora_components::Input::register_key_bindings(cx);
        liora_components::CodeBlock::register_key_bindings(cx);
        liora_components::CodeEditor::register_key_bindings(cx);
        liora_components::Preview::register_key_bindings(cx);
        liora_components::Text::register_key_bindings(cx);
        liora_components::Paragraph::register_key_bindings(cx);
        liora_components::Title::register_key_bindings(cx);
        liora_components::Tour::register_key_bindings(cx);
        // cx.open_window(...)
    });
}
```

Prefer `init_liora_with_mode(cx, ThemeMode::System)` so the app follows the operating system by default. `init_liora(cx, Theme::light())` remains available for explicit compatibility/setup flows that need a fixed theme.

Use `Entity<T>` for stateful controls such as `Input`, `Switch`, `Select`, or `CodeEditor` so focus and internal state survive re-rendering. Gallery and Docs are the maintained compile-checked examples for app shell setup, key binding registration, theme switching, tray behavior, toasts, and composition patterns.

## Development checks

Run the same checks used by the local quality gate:

```bash
cargo fmt --all --check
cargo check --workspace --all-targets
cargo test --workspace
cargo check -p liora-docs --bin check_snippets
cargo doc --workspace --no-deps
```

Packaging readiness gates:

```bash
cargo run -p xtask -- package validate
cargo run -p xtask -- package release-readiness
cargo run -p xtask -- package ci --all-apps --format platform-defaults --dry-run --skip-build
cargo run -p xtask -- package install-smoke --all-apps --format platform-defaults --dry-run
```

## Documentation

- Native Docs app: `cargo run -p liora-docs`
- Dashboard Patterns page: composition guidance extracted from Gallery/Docs dogfooding
- Dashboard State page: data modeling, filters, refresh, and state branches for real Liora screens
- Component Gallery: `cargo run -p liora-gallery`
- Packaging plan: `docs/packaging-installer-technical-plan.md`
- Release candidate checklist: `docs/release-candidate-checklist.md`
- Phase prompts and status: `.prompt/` and `.memory/`

## Release candidate readiness

P21 records the repository-owned `0.1.0` release-candidate checklist in `docs/release-candidate-checklist.md`. The checklist keeps the local gates, package metadata audit, canonical app boundary, and protected release-only items in one place.

P12 repository-owned packaging readiness is complete. Formal public releases require owner-controlled credentials and protected release environments for macOS notarization, Windows signing, real system install/uninstall execution, and publishing a real `vX.Y.Z` GitHub Release. The repository includes gates so missing release credentials block formal release publishing instead of silently producing unsigned public artifacts.

## License

Liora currently uses `LicenseRef-Liora`; see `LICENSE.md`. Do not assume an OSS license until the project owner replaces that policy with explicit OSS or commercial terms.
