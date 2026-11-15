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
4. Use Docs to copy the setup shape: `liora::init_liora(cx)` as the one-line default entry point. It initializes core/theme state, global component services, and component key bindings. Use `liora::init_liora_with_mode(cx, ThemeMode::Light | ThemeMode::Dark | ThemeMode::System)` when a product wants an explicit startup mode.
5. Keep stateful controls as `Entity<T>` fields in your own app views.

## Canonical app surfaces

Liora no longer maintains separate `minimal-app` or `dashboard-app` binaries. That functionality belongs in the maintained native apps:

- Gallery is the visual dogfooding surface for components, filters, theme switching, feedback, tray residency, and shell-level interactions.
- Docs is the adoption/reference surface for setup, architecture, packaging, and component usage.

This avoids sample-only drift. If Gallery or Docs needs raw GPUI glue for a common product behavior, treat that as a signal to improve Liora components or shell helpers instead of creating another standalone sample app.

## Minimal app shape

A downstream application still follows the same minimal setup:

```rust
use gpui::App;
use liora::init_liora;

fn main() {
    gpui_platform::application().run(|cx: &mut App| {
        init_liora(cx);
        // open your product window here
    });
}
```

## Dependency shape

For a downstream project after the v0.6.1 crates.io release, use Liora from crates.io and patch Cargo's `gpui` registry fallback to Liora's matching official Zed revision:

This 0.6.1 release targets official Zed GPUI revision `f0d896f2c18a543335eb155f729d36b624695591`. The example below becomes registry-installable after publication completes; until then, use the local workspace paths described below. macOS 0.6 builds require the macOS 26 SDK and run on macOS 12+.

```toml
[dependencies]
liora = "0.6.1"
gpui = { version = "0.2.2", default-features = false }
gpui_platform = { git = "https://github.com/zed-industries/zed", rev = "f0d896f2c18a543335eb155f729d36b624695591", default-features = false }

[patch.crates-io]
gpui = { git = "https://github.com/zed-industries/zed", rev = "f0d896f2c18a543335eb155f729d36b624695591" }
```

For local monorepo development, use `liora = { path = "../liora/crates/liora" }` and use the exact GPUI revision from that checkout's workspace manifest for both `gpui` and `gpui_platform`.

## Opt-in WebView

Applications that intentionally host web content can add `liora-webview = "0.6.1"` after its crates.io publication completes. Before publication, use `liora-webview = { path = "../../crates/liora-webview" }` from an app manifest under `apps/<app>`. The `liora` facade and `liora-components` do not include it; Gallery, Docs, and other native controls do not use it. See the [crate README](https://github.com/yhyzgn/liora/blob/main/crates/liora-webview/README.md) for entity lifetime, navigation, messaging, permissions, and platform limits. Linux requires X11/XWayland, GTK3, and WebKitGTK 4.1; macOS/Windows runtime behavior is not verified, and system-engine compatibility is not browser equivalence.

## Component adoption checklist

Before using a component in production, check:

- Docs page for effect + code examples.
- Gallery page for native visual behavior.
- Whether it has state that should live in an `Entity<T>`; app-level key bindings are covered by `liora::init_liora(cx)`.
- Whether it should live as `Entity<T>` instead of being rebuilt every render.
- Overlay close policy: ESC and outside-click behavior for popups, drawers, dialogs, Preview, and Tour.
- Performance knobs for large data components such as CodeBlock, charts, and virtualized controls.

## Packaging downstream apps

Liora's own Gallery and Docs apps use the published `liora-packager` library through the repository-local `xtask` wrapper. A downstream product can reuse `liora-packager` APIs or copy the wrapper structure if it wants native installers, but component-library users do not need to publish Liora's raw binaries.

The package release gate is:

```bash
cargo run -p xtask -- package release-readiness
```

Formal releases remain owner-controlled because macOS notarization, Windows signing, and destructive system-level installer smoke tests require protected credentials or dedicated runners.
