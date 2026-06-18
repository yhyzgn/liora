# Aura

Aura is a pure Rust + GPUI native component library for desktop applications. It provides enterprise-style UI components, a native Gallery app, a native Docs app, tray support, charting widgets, code display/editing primitives, and an installer packaging pipeline for Aura's own GPUI apps.

Aura applications stay native: no Tauri runtime, no WebView, no HTML/CSS/DOM application shell, and no browser runtime architecture.

## What is included

- `aura-components` — buttons, forms, overlays, data display, navigation, charts, virtualized lists/tables/trees, code blocks, QR code, timer, tray-related UI demos, and more.
- `aura-core` — global Aura config, theme initialization, portal/popper state, unique id helpers.
- `aura-theme` — light/dark design tokens and semantic component sizes/variants.
- `aura-icons` / `aura-icons-lucide` — native GPUI icon rendering and bundled Lucide icon paths.
- `aura-tray` — cross-platform system tray facade for Aura GPUI applications.
- `aura-packager` + `xtask` — native installer/package metadata, validation, manifests, checksums, and release-readiness gates.
- `apps/aura-gallery` — native component demo app.
- `apps/aura-docs` — native documentation app.
- `examples/minimal-app` — minimal external-style GPUI + Aura application.
- `examples/dashboard-app` — realistic dogfooding dashboard that composes Aura charts, forms, tables, progress, code blocks, and toasts.

## Quick start

Install Rust stable and the platform GPUI dependencies documented in the native Docs app Quick Start page. On Linux, the common development packages include GTK3, Wayland/X11, xkbcommon, fontconfig/freetype, Vulkan, ALSA, and pkg-config.

Run the Gallery:

```bash
cargo run -p aura-gallery
```

Run the Docs app:

```bash
cargo run -p aura-docs
```

Run the minimal adoption example:

```bash
cargo run -p aura-minimal-app
```

Run the dogfooding dashboard example:

```bash
cargo run -p aura-dashboard-app
```

## Minimal application shape

A GPUI app using Aura should initialize the Aura theme/config, initialize global services that the selected components need, register component key bindings, then open a GPUI window:

```rust
use aura_core::init_aura;
use aura_theme::Theme;
use gpui::App;

fn main() {
    gpui_platform::application().run(|cx: &mut App| {
        init_aura(cx, Theme::light());
        aura_components::MessageManager::init(cx);
        aura_components::Input::register_key_bindings(cx);
        aura_components::Text::register_key_bindings(cx);
        // cx.open_window(...)
    });
}
```

Use `Entity<T>` for stateful controls such as `Input`, `Switch`, `Select`, or `CodeEditor` so focus and internal state survive re-rendering. See `examples/minimal-app/src/main.rs` for a complete compile-checked starter and `examples/dashboard-app/src/main.rs` for a larger composition example using `DashboardGrid`, `dashboard_card`, `metric_card`, and theme switching.

## Development checks

Run the same checks used by the local quality gate:

```bash
cargo fmt --all --check
cargo check --workspace --all-targets
cargo test --workspace
cargo check -p aura-docs --bin check_snippets
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

- Native Docs app: `cargo run -p aura-docs`
- Dashboard Patterns page: composition guidance for real Aura product screens
- Component Gallery: `cargo run -p aura-gallery`
- Packaging plan: `docs/packaging-installer-technical-plan.md`
- Phase prompts and status: `.prompt/` and `.memory/`

## Release boundary

P12 repository-owned packaging readiness is complete. Formal public releases require owner-controlled credentials and protected release environments for macOS notarization, Windows signing, real system install/uninstall execution, and publishing a real `vX.Y.Z` GitHub Release. The repository includes gates so missing release credentials block formal release publishing instead of silently producing unsigned public artifacts.

## License

Aura currently uses `LicenseRef-Aura`; see `LICENSE.md`. Do not assume an OSS license until the project owner replaces that policy with explicit OSS or commercial terms.
