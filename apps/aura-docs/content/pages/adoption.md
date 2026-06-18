# Adoption Guide

This page is for a new Rust/GPUI project that wants to adopt Aura without reading the whole repository first.

## 10-minute path

1. Install Rust stable and the GPUI platform dependencies from Quick Start.
2. Run the native examples:

```bash
cargo run -p aura-gallery
cargo run -p aura-docs
cargo run -p aura-minimal-app
cargo run -p aura-dashboard-app
```

3. Copy the app shape from `examples/minimal-app/src/main.rs`.
4. Initialize Aura once with `init_aura(cx, Theme::light())` or `Theme::dark()`.
5. Initialize global services such as `MessageManager::init(cx)` when using messages/toasts.
6. Register key bindings for components used by the app.
7. Keep stateful controls as `Entity<T>` fields.

## Minimal app

The repository includes a compile-checked external-style app:

```bash
cargo check -p aura-minimal-app
cargo run -p aura-minimal-app
```

It demonstrates:

- native GPUI window creation;
- Aura theme initialization;
- global message/toast initialization;
- key binding registration;
- `Entity<Input>` and `Entity<Switch>` state preservation;
- basic `Card`, `Space`, `Title`, `Text`, and `Button` composition.

## Dashboard dogfooding app

Use the dashboard app when you want to inspect real composition friction instead of isolated component behavior:

```bash
cargo check -p aura-dashboard-app
cargo run -p aura-dashboard-app
```

`examples/dashboard-app/src/main.rs` combines filters, statistic cards, charts, progress panels, a table, a runbook `CodeBlock`, toasts, and key binding setup in one native GPUI window. If a component API change makes this app difficult to maintain, treat that as adoption feedback.

## Dependency shape

For a workspace project, use path dependencies while developing locally:

```toml
[dependencies]
aura-components = { path = "../aura/crates/aura-components" }
aura-core = { path = "../aura/crates/aura-core" }
aura-theme = { path = "../aura/crates/aura-theme" }
gpui = { git = "https://github.com/zed-industries/zed", default-features = false }
gpui_platform = { git = "https://github.com/zed-industries/zed", default-features = false }
```

Keep GPUI sourced consistently across the app and Aura to avoid duplicate framework versions.

## Component adoption checklist

Before using a component in production, check:

- Docs page for effect + code examples.
- Gallery page for native visual behavior.
- Whether it needs app-level key binding registration.
- Whether it should live as `Entity<T>` instead of being rebuilt every render.
- Overlay close policy: ESC and outside-click behavior for popups, drawers, dialogs, Preview, and Tour.
- Performance knobs for large data components such as CodeBlock, charts, and virtualized controls.

## Packaging downstream apps

Aura's own Gallery and Docs apps use `aura-packager` through `xtask`. A downstream product can copy that structure if it wants native installers, but component-library users do not need to publish Aura's raw binaries.

The package release gate is:

```bash
cargo run -p xtask -- package release-readiness
```

Formal releases remain owner-controlled because macOS notarization, Windows signing, and destructive system-level installer smoke tests require protected credentials or dedicated runners.
