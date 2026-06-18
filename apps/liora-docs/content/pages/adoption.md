# Adoption Guide

This page is for a Rust/GPUI project that wants to adopt Liora without treating sample apps as separate products.

## 10-minute path

1. Install Rust stable and the GPUI platform dependencies from Quick Start.
2. Run the maintained native surfaces:

```bash
cargo run -p liora-gallery
cargo run -p liora-docs
```

3. Use Gallery to inspect component behavior and app-shell interactions such as menu search, theme switching, tray controls, toasts, and close-to-tray flow.
4. Use Docs to copy the setup shape: `init_liora_with_mode(cx, ThemeMode::System)`, global service initialization, and component key binding registration. `init_liora(cx, Theme::light())` remains available when a product wants an explicit fixed theme.
5. Keep stateful controls as `Entity<T>` fields in your own app views.

## Canonical app surfaces

Liora no longer maintains separate `minimal-app` or `dashboard-app` binaries. That functionality belongs in the maintained native apps:

- Gallery is the visual dogfooding surface for components, filters, theme switching, feedback, tray residency, and shell-level interactions.
- Docs is the adoption/reference surface for setup, architecture, packaging, and component usage.

This avoids sample-only drift. If Gallery or Docs needs raw GPUI glue for a common product behavior, treat that as a signal to improve Liora components or shell helpers instead of creating another standalone sample app.

## Minimal app shape

A downstream application still follows the same minimal setup:

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
        // open your product window here
    });
}
```

## Dependency shape

For a workspace project, use path dependencies while developing locally:

```toml
[dependencies]
liora-components = { path = "../liora/crates/liora-components" }
liora-core = { path = "../liora/crates/liora-core" }
liora-theme = { path = "../liora/crates/liora-theme" }
gpui = { git = "https://github.com/zed-industries/zed", default-features = false }
gpui_platform = { git = "https://github.com/zed-industries/zed", default-features = false }
```

Keep GPUI sourced consistently across the app and Liora to avoid duplicate framework versions.

## Component adoption checklist

Before using a component in production, check:

- Docs page for effect + code examples.
- Gallery page for native visual behavior.
- Whether it needs app-level key binding registration.
- Whether it should live as `Entity<T>` instead of being rebuilt every render.
- Overlay close policy: ESC and outside-click behavior for popups, drawers, dialogs, Preview, and Tour.
- Performance knobs for large data components such as CodeBlock, charts, and virtualized controls.

## Packaging downstream apps

Liora's own Gallery and Docs apps use `liora-packager` through `xtask`. A downstream product can copy that structure if it wants native installers, but component-library users do not need to publish Liora's raw binaries.

The package release gate is:

```bash
cargo run -p xtask -- package release-readiness
```

Formal releases remain owner-controlled because macOS notarization, Windows signing, and destructive system-level installer smoke tests require protected credentials or dedicated runners.
