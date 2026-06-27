<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/liora-logo.svg">
    <img src="assets/liora-logo.svg" alt="Liora — pure Rust + GPUI native UI component library" width="220">
  </picture>

  <p><strong>Enterprise-grade native UI components for Rust desktop applications.</strong></p>
  <p>Pure Rust. GPUI native. Element Plus-inspired APIs. No Tauri, no WebView, no browser runtime.</p>

  <p>
    <a href="README.zh-CN.md">简体中文</a>
    ·
    <a href="CONTRIBUTING.md">Contributing</a>
    ·
    <a href="CHANGELOG.md">Changelog</a>
  </p>

  <p>
    <a href="https://crates.io/crates/liora"><img alt="crates.io liora" src="https://img.shields.io/crates/v/liora.svg?label=liora"></a>
    <img alt="MSRV 1.95" src="https://img.shields.io/badge/rustc-1.95%2B-dea584?logo=rust&logoColor=white">
    <img alt="Rust 2024" src="https://img.shields.io/badge/edition-2024-dea584?logo=rust&logoColor=white">
    <img alt="GPUI native" src="https://img.shields.io/badge/GPUI-official%20Zed%20git-7c3aed">
    <img alt="Pure Rust" src="https://img.shields.io/badge/runtime-pure%20Rust%20native-10b981">
    <img alt="Native packaging" src="https://img.shields.io/badge/packaging-native-0ea5e9">
    <img alt="LicenseRef-Liora" src="https://img.shields.io/badge/license-LicenseRef--Liora-64748b">
  </p>
</div>

---

## Table of contents

- [What is Liora?](#what-is-liora)
- [What you can build](#what-you-can-build)
- [Requirements](#requirements)
- [Choose the right dependency setup](#choose-the-right-dependency-setup)
- [GPUI dependency and local patch policy](#gpui-dependency-and-local-patch-policy)
- [Quick start: create a Liora app](#quick-start-create-a-liora-app)
- [Application initialization](#application-initialization)
- [Window startup, system theme, and icons](#window-startup-system-theme-and-icons)
- [Using the Liora modules](#using-the-liora-modules)
- [Component examples](#component-examples)
- [Advanced usage](#advanced-usage)
- [Component catalog](#component-catalog)
- [Native packaging](#native-packaging)
- [Troubleshooting](#troubleshooting)
- [Quality gates](#quality-gates)
- [Technical differentiators](#technical-differentiators)
- [Documentation maintenance rule](#documentation-maintenance-rule)
- [License](#license)

## What is Liora?

**Liora** is a native Rust + GPUI component SDK for building polished desktop applications. It provides a one-stop `liora` facade crate plus focused modules for core runtime setup, theme tokens, components, icons, tray integration, package metadata, and GitHub Release update flows.

Liora is intentionally not a web application shell:

- no Tauri runtime;
- no WebView, HTML/CSS/DOM, or browser application shell;
- no web chart runtime or frontend bundler;
- Gallery and Docs are real native GPUI applications that use the same public SDK surface as downstream apps.

## What you can build

Use Liora when you want a Rust desktop app with:

| Need | Liora answer |
|---|---|
| Native desktop UI | GPUI windows, GPUI element trees, native text/layout/paint paths. |
| Enterprise component coverage | Element Plus-inspired components across layout, forms, overlays, navigation, data display, charts, and advanced inputs. |
| One-line app initialization | `liora::init_liora(cx)` initializes core state, component services, and key bindings. |
| Light/Dark/System theming | `ThemeMode`, semantic tokens, runtime switching, and system appearance tracking. |
| System tray apps | `liora-tray` wraps `tray-icon` and `muda` with stable app commands. |
| Native release artifacts | `liora-packager` + `xtask package` validate and generate package plans for Linux, macOS, and Windows. |
| Updater integration | `liora-updater` checks GitHub Releases, selects assets, verifies SHA-256, and returns explicit install plans. |

## Requirements

| Item | Requirement |
|---|---|
| Rust | `rustc 1.95+`, Rust edition 2024. |
| UI backend | Official Zed GPUI git dependency pinned to Liora's verified revision. |
| Linux native deps | GTK3, Wayland/X11, xkbcommon, fontconfig/freetype, Vulkan, ALSA, `pkg-config`; see `scripts/install-fedora-deps.sh` for a Fedora-oriented baseline. |
| macOS | Apple Silicon is covered by the release workflow; install Xcode Command Line Tools. |
| Windows | MSVC toolchain; GPUI's Windows backend provides the application manifest through `windows-manifest`. |

## Choose the right dependency setup

Most applications should depend on the facade crate:

```toml
[dependencies]
liora = "0.1"
```

Use focused crates only when you deliberately want a narrower surface:

```toml
[dependencies]
liora-components = "0.1"
liora-core = "0.1"
liora-theme = "0.1"
liora-icons = "0.1"
liora-icons-lucide = "0.1"
liora-tray = "0.1"
liora-updater = "0.1"
liora-packager = "0.1"
```

The facade re-exports stable module names:

```rust
use liora::{components, core, icons, icons_lucide, theme, tray};
use liora::prelude::*;

#[cfg(feature = "updater")]
use liora::updater;

#[cfg(feature = "packager")]
use liora::packager;
```

If you do not need packaging or updater helpers in your app dependency graph, turn off facade defaults and re-enable only what you need:

```toml
[dependencies]
liora = { version = "0.1", default-features = false }

# Or keep only updater helpers:
liora = { version = "0.1", default-features = false, features = ["updater"] }
```

## GPUI dependency and local patch policy

Liora uses **official Zed GPUI only** from the official Zed upstream repository. Do not use renamed or community forks such as `open-gpui`.

Why the extra GPUI setup exists:

1. `liora` crates are published on crates.io.
2. Current Liora development targets a newer official Zed GPUI git revision than the old registry `gpui 0.2.2` fallback.
3. Cargo does not let a crates.io package force a git-only transitive dependency on every downstream app.
4. Therefore published Liora crates use Cargo's multiple-location dependency form: registry fallback for publication, official Zed git rev for local development.
5. Final applications must add a root-level `[patch.crates-io]` entry so every transitive `gpui` dependency resolves to the official Zed commit.

Use this application manifest pattern:

```toml
[package]
name = "acme-notes"
version = "0.1.0"
edition = "2024"
publish = false

[dependencies]
liora = "0.1"

# Add gpui manually when your crate mentions gpui types directly:
# - gpui::App / Window / Context / Render / RenderOnce
# - gpui::div(), px(), size(), Entity<T>
# - function signatures such as fn render(..., cx: &mut gpui::Context<Self>)
gpui = { version = "0.2.2", default-features = false }

# Add gpui_platform manually in final binary crates that create native windows
# with gpui_platform::application().run(...).
gpui_platform = { git = "https://github.com/zed-industries/zed", rev = "2c346f60a76fe3f0367ef924927f50a6efdf5718", default-features = false }

[target.'cfg(any(target_os = "linux", target_os = "freebsd"))'.dependencies]
gpui = { version = "0.2.2", default-features = false, features = ["wayland", "x11", "font-kit"] }
gpui_platform = { git = "https://github.com/zed-industries/zed", rev = "2c346f60a76fe3f0367ef924927f50a6efdf5718", default-features = false, features = ["wayland", "x11", "font-kit"] }

[target.'cfg(target_os = "macos")'.dependencies]
gpui = { version = "0.2.2", default-features = false, features = ["font-kit"] }
gpui_platform = { git = "https://github.com/zed-industries/zed", rev = "2c346f60a76fe3f0367ef924927f50a6efdf5718", default-features = false, features = ["font-kit"] }

[target.'cfg(target_os = "windows")'.dependencies]
gpui = { version = "0.2.2", default-features = false }
gpui_platform = { git = "https://github.com/zed-industries/zed", rev = "2c346f60a76fe3f0367ef924927f50a6efdf5718", default-features = false }

[patch.crates-io]
gpui = { git = "https://github.com/zed-industries/zed", rev = "2c346f60a76fe3f0367ef924927f50a6efdf5718" }
```

When do you manually add `gpui`?

```rust
// You need a direct gpui dependency because this file names gpui types and macros.
use gpui::{App, Context, IntoElement, Render, Window, div, px};
use liora::components::{Button, Title};

struct RootView;

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .p(px(24.0))
            .child(Title::new("Acme Notes").h2())
            .child(Button::new("Create note").primary())
    }
}

fn needs_gpui_app_type(_cx: &mut App) {}
```

When can you avoid direct `gpui` usage? Very small helper crates that only build data models, theme values, update requests, or packaging metadata can depend on focused Liora crates without opening windows or naming `gpui` types.

The repository keeps `third_party/zed` only as non-published upstream-source reference material for prior Linux startup-window patch work and PR comparison. Current development should use the official `zed-industries/zed` git dependency above. If a temporary local patch is needed for app-only verification, keep it outside publishable SDK manifests and document the boundary.

## Quick start: create a Liora app

### 1. Create the app

```bash
cargo new acme-notes
cd acme-notes
```

### 2. Add dependencies

Paste the manifest from [GPUI dependency and local patch policy](#gpui-dependency-and-local-patch-policy), or start with this compact Linux/macOS/Windows manifest:

```toml
[package]
name = "acme-notes"
version = "0.1.0"
edition = "2024"
publish = false

[dependencies]
liora = "0.1"
gpui = { version = "0.2.2", default-features = false }
gpui_platform = { git = "https://github.com/zed-industries/zed", rev = "2c346f60a76fe3f0367ef924927f50a6efdf5718", default-features = false }

[target.'cfg(any(target_os = "linux", target_os = "freebsd"))'.dependencies]
gpui = { version = "0.2.2", default-features = false, features = ["wayland", "x11", "font-kit"] }
gpui_platform = { git = "https://github.com/zed-industries/zed", rev = "2c346f60a76fe3f0367ef924927f50a6efdf5718", default-features = false, features = ["wayland", "x11", "font-kit"] }

[target.'cfg(target_os = "macos")'.dependencies]
gpui = { version = "0.2.2", default-features = false, features = ["font-kit"] }
gpui_platform = { git = "https://github.com/zed-industries/zed", rev = "2c346f60a76fe3f0367ef924927f50a6efdf5718", default-features = false, features = ["font-kit"] }

[target.'cfg(target_os = "windows")'.dependencies]
gpui = { version = "0.2.2", default-features = false }
gpui_platform = { git = "https://github.com/zed-industries/zed", rev = "2c346f60a76fe3f0367ef924927f50a6efdf5718", default-features = false }

[patch.crates-io]
gpui = { git = "https://github.com/zed-industries/zed", rev = "2c346f60a76fe3f0367ef924927f50a6efdf5718" }
```

### 3. Write a minimal `src/main.rs`

```rust
use gpui::{App, AppContext, Context, IntoElement, Render, Window, WindowOptions, div, px};
use liora::components::{Button, Card, Space, Tag, Text, Title};
use liora::init_liora;

struct RootView;

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().p(px(24.0)).child(
            Card::new(
                Space::new()
                    .vertical()
                    .child(Title::new("Acme Notes").h2())
                    .child(Text::new("A native Rust desktop app powered by GPUI and Liora."))
                    .child(
                        Space::new()
                            .child(Button::new("New note").primary())
                            .child(Button::new("Import"))
                            .child(Tag::new("Pure Rust").success()),
                    ),
            )
            .title("Welcome")
            .width_lg(),
        )
    }
}

fn main() {
    gpui_platform::application().run(|cx: &mut App| {
        // One call initializes theme/config state, overlay/message services,
        // and key bindings for interactive Liora controls.
        init_liora(cx);

        let _ = cx.open_window(
            WindowOptions {
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("Acme Notes".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| RootView),
        );
    });
}
```

### 4. Run it

```bash
cargo run
```

### 5. Compare with full app references

Inside this repository:

```bash
cargo run -p liora-gallery
cargo run -p liora-docs
```

`liora-gallery` is the component showcase and app-shell reference. `liora-docs` is the native documentation app and Markdown renderer.

## Application initialization

Use the facade entry points for normal app binaries:

```rust
use gpui::App;
use liora::{ThemeMode, init_liora, init_liora_with_mode, init_liora_with_options};
use liora::{FontConfig, LioraOptions};

fn init_default(cx: &mut App) {
    // Recommended default: follow the operating system theme.
    init_liora(cx);
}

fn init_dark(cx: &mut App) {
    // Explicit startup mode.
    init_liora_with_mode(cx, ThemeMode::Dark);
}

fn init_with_system_font_names(cx: &mut App) {
    // No font files are loaded here. GPUI resolves these names from the OS.
    let fonts = FontConfig::system()
        .with_ui_family("Segoe UI")
        .with_code_family("JetBrains Mono");

    init_liora_with_options(cx, LioraOptions::system().with_fonts(fonts));
}
```

If you depend on focused crates instead of the facade, use the matching component initializer:

```rust
use gpui::App;
use liora_components::{ThemeMode, init_liora, init_liora_with_mode};

fn init_components_only(cx: &mut App) {
    init_liora(cx);
    init_liora_with_mode(cx, ThemeMode::System);
}
```

Important distinction:

```rust
// High-level app setup: core theme + portals + MessageManager + component key bindings.
liora::init_liora(cx);
liora_components::init_liora(cx);

// Lower-level core setup only: use when building a custom component crate or replacing services yourself.
liora_core::init_liora_with_mode(cx, liora_core::ThemeMode::System);
```

## Window startup, system theme, and icons

A production app should create the window hidden, attach system theme tracking before creating the root view, then activate the window after `open_window` returns. This avoids first-frame theme flicker and mirrors the pattern used by the native Gallery and Docs apps.

```rust
use gpui::{App, AppContext, Context, Render, Window, WindowOptions, px, size};
use liora::components::{Title, apply_window_frame_mode, WindowFrameMode};
use liora::{attach_system_theme_observer, init_liora, startup_maximized_window_bounds};

struct RootView;

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        Title::new("Maximized Liora window").h2()
    }
}

fn main() {
    gpui_platform::application().run(|cx: &mut App| {
        init_liora(cx);

        let options = apply_window_frame_mode(
            WindowOptions {
                show: false,
                app_id: Some("acme-notes".into()),
                window_bounds: Some(startup_maximized_window_bounds(
                    cx,
                    size(px(1440.0), px(900.0)),
                )),
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("Acme Notes".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            WindowFrameMode::System,
        );

        if let Ok(handle) = cx.open_window(options, |window, cx| {
            attach_system_theme_observer(window, cx);
            cx.new(|_| RootView)
        }) {
            let any_handle: gpui::AnyWindowHandle = handle.into();
            let _ = any_handle.update(cx, |_, window, _| window.activate_window());
        }
    });
}
```

On Linux/Wayland, taskbar icons are resolved by desktop identity (`app_id` + `.desktop` + icon theme), not by setting a window icon directly. Liora exposes helpers used by Gallery/Docs; downstream apps can use the same pattern with app-owned icon assets:

```rust
use liora::core::{
    LinuxDesktopIdentity, LinuxDesktopPngIcon, ensure_linux_desktop_identity,
    linux_desktop_entry, linux_desktop_png_icon_path,
};

fn register_linux_identity() {
    let icon_name = "acme-notes";
    let desktop_entry = linux_desktop_entry(
        icon_name,
        "Acme Notes",
        "Native notes app built with Liora",
        icon_name,
    );

    let _ = ensure_linux_desktop_identity(LinuxDesktopIdentity {
        app_id: icon_name,
        desktop_entry: &desktop_entry,
        png_icons: &[LinuxDesktopPngIcon {
            size: 512,
            bytes: include_bytes!("../assets/acme-notes-512.png"),
        }],
    });

    let _icon_path = linux_desktop_png_icon_path(icon_name, 512);
}
```

## Using the Liora modules

### `liora`

The recommended app dependency. It re-exports:

```rust
use liora::{init_liora, init_liora_with_mode, init_liora_with_options};
use liora::{FontConfig, LioraOptions, ThemeMode};
use liora::{components, core, icons, icons_lucide, theme, tray};
```

### `liora-core`

Core runtime, theme config, window helpers, Linux desktop identity, popper/portal state, unique IDs, and theme switching:

```rust
use liora::core::{apply_theme_mode, sync_system_theme, ThemeMode};

fn set_dark(window: &mut gpui::Window, cx: &mut gpui::App) {
    apply_theme_mode(window, cx, ThemeMode::Dark);
}

fn follow_system_again(window: &mut gpui::Window, cx: &mut gpui::App) {
    apply_theme_mode(window, cx, ThemeMode::System);
    sync_system_theme(window, cx);
}
```

### `liora-theme`

Semantic tokens and shared component enums:

```rust
use liora::theme::{ButtonSize, ButtonVariant, Theme};

let light = Theme::light();
let dark = Theme::dark();
let primary_variant = ButtonVariant::Primary;
let large = ButtonSize::Large;
let surface = light.neutral.card;
```

### `liora-components`

Reusable native controls. Most stateless components can be built inline:

```rust
use liora::components::{Button, Progress, Space, Tag, Text, Title};

let header = Space::new()
    .vertical()
    .child(Title::new("Deployments").h3())
    .child(Text::new("Production rollout status"))
    .child(Progress::new(72.0).primary().show_text(true))
    .child(Tag::new("Healthy").success());
```

Stateful controls should live in `gpui::Entity<T>` fields so focus, selection, popup state, and text values survive renders:

```rust
use gpui::{Context, Entity, Render, Window};
use liora::components::{Input, Switch};

struct SettingsView {
    search: Entity<Input>,
    notifications: Entity<Switch>,
}

impl SettingsView {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            search: cx.new(|cx| Input::new("", cx).placeholder("Search settings")),
            notifications: cx.new(|cx| Switch::new(true, cx)),
        }
    }
}

impl Render for SettingsView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        gpui::div()
            .child(self.search.clone())
            .child(self.notifications.clone())
    }
}
```

### `liora-icons` and `liora-icons-lucide`

Icon primitives plus bundled Lucide icon names:

```rust
use liora::icons::Icon;
use liora::icons_lucide::IconName;
use liora::components::Button;

let save = Button::new("Save").primary().icon_prefix(IconName::Save);
let icon = Icon::new(IconName::Settings).size(18.0);
```

When using raw `gpui_platform::application()`, install the Liora icon asset source if your app uses bundled Lucide SVG payloads:

```rust
fn main() {
    gpui_platform::application()
        .with_assets(liora_icons::IconAssetSource)
        .run(|cx| {
            liora::init_liora(cx);
            // open windows...
        });
}
```

### `liora-tray`

System tray facade:

```rust
use liora::tray::{
    LioraTray, TrayCommand, TrayConfig, TrayMenuItemSpec, icon_from_png_bytes,
};
fn install_tray() -> liora::tray::Result<()> {
    let icon = icon_from_png_bytes(include_bytes!("../assets/tray-default.png"))?;
    let config = TrayConfig::new("acme-notes")
        .tooltip("Acme Notes")
        .icon(icon)
        .menu(vec![
            TrayMenuItemSpec::action("Show", TrayCommand::Show),
            TrayMenuItemSpec::check("Start at login", TrayCommand::Custom("login".into()), false),
            TrayMenuItemSpec::separator(),
            TrayMenuItemSpec::submenu(
                "Status",
                vec![
                    TrayMenuItemSpec::action("Online", TrayCommand::SetIcon("online".into())),
                    TrayMenuItemSpec::action("Busy", TrayCommand::SetIcon("busy".into())),
                ],
            ),
            TrayMenuItemSpec::separator(),
            TrayMenuItemSpec::action("Quit", TrayCommand::Quit),
        ]);

    let tray = LioraTray::install(config)?;

    // In your app event loop, map platform menu events with:
    // if let Some(command) = tray.command_for_event(&event) { ... }
    // On Linux/FreeBSD, periodically call liora::tray::pump_platform_events().

    drop(tray);
    Ok(())
}
```

### `liora-updater`

Reusable GitHub Release update flow for your own app:

```rust
use liora::updater::{AssetKind, AssetSelector, Platform, UpdateRequest, Updater};

fn check_for_update() -> Result<(), liora::updater::UpdaterError> {
    let platform = Platform::current().expect("supported desktop platform");
    let request = UpdateRequest::new(
        "acme-notes",
        "v0.3.0",
        platform,
        std::env::temp_dir().join("acme-notes-updates"),
    )
    .selector(
        AssetSelector::for_platform(platform)
            .matching_prefix("acme-notes")
            .kind_priority([AssetKind::Installer, AssetKind::RawExecutable]),
    );

    if let Some(update) = Updater::new("acme", "acme-notes")
        .with_checksum_asset_name("SHA256SUMS.txt")
        .prepare_update(&request)?
    {
        println!("new version: {}", update.release.tag);
        println!("asset: {}", update.asset.name);
        println!("install plan: {:?}", update.install_plan);
        // Run installation only after a visible user action.
    }

    Ok(())
}
```

### `liora-packager`

Reusable packaging metadata and validation helpers. Most applications will copy the repository's `xtask` pattern, but the library is publishable for custom release tools:

```rust
use liora::packager::validate_packaging_layout;

fn validate_release_inputs() {
    let report = validate_packaging_layout(std::env::current_dir().unwrap());
    if !report.is_ok() {
        for error in report.errors {
            eprintln!("{error}");
        }
    }
}
```

## Component examples

### Layout and cards

```rust
use gpui::{div, px};
use liora::components::{Button, Card, Flex, Space, Statistic, Tag, Text, Title};

let dashboard = Flex::new()
    .gap(px(16.0))
    .child(
        Card::new(
            Space::new()
                .vertical()
                .child(Title::new("Revenue").h3())
                .child(Statistic::new("MRR", "$42,800"))
                .child(Tag::new("+12.4%").success()),
        )
        .width_lg(),
    )
    .child(
        Card::new(
            div()
                .child(Text::new("Ship a native desktop dashboard without a WebView."))
                .child(Button::new("Open report").primary()),
        )
        .title("Summary")
        .hoverable(),
    );
```

### Buttons, tags, progress, and feedback

```rust
use liora::components::{
    Button, Progress, Space, Tag, toast_error, toast_success,
};

let actions = Space::new()
    .child(Button::new("Save").primary().on_click(|_, _window, cx| {
        toast_success("Saved", cx);
    }))
    .child(Button::new("Delete").danger().on_click(|_, _window, cx| {
        toast_error("Deletion failed in this demo", cx);
    }))
    .child(Tag::new("Draft").warning())
    .child(Progress::new(48.0).show_text(true));
```

### Forms and stateful controls

```rust
use gpui::{Context, Entity, Render, Window};
use liora::components::{Button, Checkbox, Form, FormItem, Input, Space};

struct LoginForm {
    email: Entity<Input>,
    remember: Entity<Checkbox>,
}

impl LoginForm {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            email: cx.new(|cx| Input::new("", cx).placeholder("name@example.com").clearable(true)),
            remember: cx.new(|cx| Checkbox::new(true, cx)),
        }
    }
}

impl Render for LoginForm {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        Form::new()
            .child(FormItem::new().label("Email").child(self.email.clone()))
            .child(FormItem::new().label("Remember me").child(self.remember.clone()))
            .child(Space::new().child(Button::new("Sign in").primary()))
    }
}
```

### Navigation menu

```rust
use liora::components::Menu;
use liora::icons_lucide::IconName;

let menu = Menu::new()
    .id("main-nav")
    .item("dashboard", "Dashboard", Some(IconName::LayoutDashboard))
    .submenu("settings", "Settings", Some(IconName::Settings), |menu| {
        menu.item("profile", "Profile", None)
            .item("security", "Security", None)
    })
    .on_select(|id, _window, _cx| {
        eprintln!("selected menu item: {id}");
    });
```

### App shell with TitleBar and Sidebar

`TitleBar` owns native custom titlebar chrome and window-control areas. `Sidebar` owns app navigation panel layout, width, fixed header/footer slots, and scrolling. Keep stateful controls such as `Menu` in the parent view as `Entity<T>` fields. When `Sidebar` is placed in `Container::aside(...)`, add `.aside_passthrough()` so the sidebar owns its own width instead of being wrapped by the container's default aside panel.

```rust
use gpui::{Context, Entity, Render, Window};
use liora::components::{
    AppWindowFrame, Button, Card, Container, Flex, Menu, MenuMode, Sidebar, Space, Text, Title,
    TitleBar, WindowFrameMode,
};
use liora::icons_lucide::IconName;

struct AppShell {
    menu: Entity<Menu>,
}

impl AppShell {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            menu: cx.new(|_| {
                Menu::new()
                    .id("main-nav")
                    .mode(MenuMode::Vertical)
                    .default_active("dashboard")
                    .item("dashboard", "Dashboard", Some(IconName::LayoutDashboard))
                    .item("settings", "Settings", Some(IconName::Settings))
            }),
        }
    }
}

impl Render for AppShell {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        AppWindowFrame::new(
            "Acme Notes",
            Container::new()
                .aside(
                    Sidebar::new()
                        .id("app-sidebar")
                        .header(Flex::new().padding_md().child(Text::new("Workspace")))
                        .child(self.menu.clone())
                        .footer(Flex::new().padding_md().child(Text::new("v1.0"))),
                )
                .aside_passthrough()
                .child(
                    Flex::new().padding_lg().child(
                        Card::new(
                            Space::new()
                                .vertical()
                                .gap_sm()
                                .child(Title::new("Dashboard").h3())
                                .child(Text::new("Main content goes here.")),
                        )
                        .no_shadow(),
                    ),
                ),
        )
        .mode(WindowFrameMode::Custom)
        .titlebar(
            TitleBar::new()
                .title("Acme Notes")
                .subtitle("Native GPUI app")
                .action(Button::new("New").small()),
        )
    }
}
```

### Charts and metrics

```rust
use gpui::rgb;
use liora::components::{
    AreaChart, BarChart, ChartPoint, ChartSeries, HeatBar, HeatBarItem, LineChart, PieChart, Sparkline,
};

let revenue = ChartSeries::new("Revenue", [
    ChartPoint::new("Mon", 12.0),
    ChartPoint::new("Tue", 18.0),
    ChartPoint::new("Wed", 16.0),
    ChartPoint::new("Thu", 24.0),
    ChartPoint::new("Fri", 32.0),
]);
let costs = ChartSeries::new("Costs", [
    ChartPoint::new("Mon", 8.0),
    ChartPoint::new("Tue", 9.0),
    ChartPoint::new("Wed", 11.0),
    ChartPoint::new("Thu", 13.0),
    ChartPoint::new("Fri", 15.0),
]);

let line = LineChart::new([revenue.clone(), costs.clone()])
    .show_grid(true)
    .show_axis(true)
    .show_legend(true)
    .show_tooltip(true);

let area = AreaChart::new([revenue.clone()]).show_tooltip(true);
let bars = BarChart::new([revenue.clone(), costs.clone()]).grouped();
let pie = PieChart::new([revenue.clone(), costs.clone()]).show_percentage_labels(true);
let spark = Sparkline::new([3.0, 4.0, 8.0, 6.0, 12.0]).show_last_point(true);

let heat = HeatBar::new([
    HeatBarItem::new("Low", 18, rgb(0x22, 0xc5, 0x5e).into()),
    HeatBarItem::new("Medium", 42, rgb(0xf5, 0x9e, 0x0b).into()),
    HeatBarItem::new("High", 9, rgb(0xef, 0x44, 0x44).into()),
]);
```

### Code display and editor

```rust
use gpui::{Context, Entity, Render, Window};
use liora::components::{CodeBlock, CodeDiagnostic, CodeEditor};

let snippet = CodeBlock::new("cargo run -p liora-gallery")
    .language("bash")
    .copyable(true);

struct EditorView {
    editor: Entity<CodeEditor>,
}

impl EditorView {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            editor: cx.new(|cx| {
                CodeEditor::new("fn main() { println!(\"hello\"); }", cx)
                    .language("rust")
                    .diagnostics(vec![CodeDiagnostic::info(1, 1, "Example diagnostic")])
            }),
        }
    }
}

impl Render for EditorView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        self.editor.clone()
    }
}
```

### QR code, upload, image, and preview

```rust
use liora::components::{Button, Image, Preview, QrCode, Space, Upload};

let utilities = Space::new()
    .vertical()
    .child(QrCode::new("https://github.com/yhyzgn/liora").show_text(true))
    .child(Image::new("file:///tmp/screenshot.png").width(gpui::px(240.0)))
    .child(Preview::new("file:///tmp/screenshot.png").child(Button::new("Preview image")))
    .child(Upload::new().width_lg());
```

### Virtualized data

```rust
use gpui::{Context, IntoElement};
use liora::components::{TableColumn, TreeNode, VirtualizedTable, VirtualizedTree};

fn build_table(cx: &mut Context<MyView>) -> gpui::Entity<VirtualizedTable> {
    let rows = vec![
        ("Liora".to_string(), "Ready".to_string()),
        ("GPUI".to_string(), "Native".to_string()),
    ];

    cx.new(|_| {
        VirtualizedTable::new(
            vec![TableColumn::new("name", "Name"), TableColumn::new("status", "Status")],
            rows.len(),
            move |row, key, _window, _cx| {
                let value = match key.as_ref() {
                    "name" => rows[row].0.clone(),
                    "status" => rows[row].1.clone(),
                    _ => String::new(),
                };
                liora::components::Text::new(value).into_any_element()
            },
        )
    })
}

fn build_tree(cx: &mut Context<MyView>) -> gpui::Entity<VirtualizedTree> {
    cx.new(|cx| {
        VirtualizedTree::new(
            vec![TreeNode::new("root", "Workspace").child(TreeNode::new("src", "src"))],
            cx,
        )
        .show_checkbox(true)
    })
}

struct MyView;
```

## Advanced usage

### Runtime theme switcher

```rust
use liora::components::{Segmented, SegmentedOption};
use liora::core::{ThemeMode, apply_theme_mode};

fn theme_switcher(current: ThemeMode) -> Segmented {
    Segmented::new(vec![
        SegmentedOption::new("System", "system"),
        SegmentedOption::new("Light", "light"),
        SegmentedOption::new("Dark", "dark"),
    ])
    .value(current.value())
    .on_change(|value, window, cx| {
        if let Some(mode) = ThemeMode::from_value(value.as_ref()) {
            apply_theme_mode(window, cx, mode);
        }
    })
}
```

### Custom fonts without losing system defaults

Liora separates **font resource loading** from **font family selection**:

1. If the family is already installed on the user's system, do not load any file. Set the family name with `FontConfig`.
2. If the app ships private fonts, register bytes first with `load_app_fonts`, `load_fonts_from_dir`, `load_font_assets`, `load_embedded_fonts`, or the low-level `load_custom_fonts` compatibility helper.
3. Then choose the UI and code family names with `LioraOptions::with_fonts(...)` at startup or `set_font_config(...)` at runtime.

Supported file extensions are `ttf`, `otf`, `ttc`, `otc`, `woff`, and `woff2`, but actual parsing is delegated to the official GPUI backend for each platform. Prefer `ttf`/`otf`/`ttc`/`otc` for native desktop apps. On Linux/WGPU, the current GPUI `fontdb` path can ignore WOFF/WOFF2 bytes without returning an error, so use `FontLoadOptions::require_family(...)` and check `FontLoadReport::missing_required_families` whenever a specific family must be active.

#### Use only system-installed fonts

```rust
use liora::{FontConfig, LioraOptions, init_liora_with_options, set_font_config};

fn init_with_system_fonts(cx: &mut gpui::App) {
    init_liora_with_options(
        cx,
        LioraOptions::system().with_fonts(
            FontConfig::system()
                .with_ui_family("Segoe UI")       // Windows example; use any installed family.
                .with_code_family("JetBrains Mono"), // Can also be an installed system font.
        ),
    );
}

fn switch_to_system_ui_and_monospace_code(cx: &mut gpui::App) {
    set_font_config(
        cx,
        FontConfig::system()
            .with_ui_family("PingFang SC")
            .with_code_family("Monospace"),
    );
}
```

#### Embed a small fallback font into a bare executable

```rust
use std::borrow::Cow;
use liora::{
    FontConfig, FontLoadMode, FontLoadOptions, LioraOptions,
    init_liora_with_options, load_app_fonts,
};

fn init_with_embedded_font(cx: &mut gpui::App) {
    let report = load_app_fonts(
        cx,
        FontLoadOptions::new(FontLoadMode::Embedded).embedded(
            "Inter-Regular.ttf",
            Cow::Borrowed(include_bytes!("../assets/fonts/Inter-Regular.ttf").as_slice()),
        ),
    );
    if !report.failures.is_empty() || !report.required_families_available() {
        eprintln!("font load failures: {report:?}");
    }

    init_liora_with_options(
        cx,
        LioraOptions::system().with_fonts(FontConfig::system().with_ui_family("Inter")),
    );
}
```

#### Prefer packaged external fonts, fall back to embedded bytes

This is the recommended pattern when full font families are large. Keep a small regular face embedded for raw executables, and ship the complete family under `assets/fonts` in installers or portable archives.

```rust
use std::{borrow::Cow, path::PathBuf};
use liora::{
    FontConfig, FontLoadMode, FontLoadOptions, LioraOptions,
    init_liora_with_options, load_app_fonts,
};

fn font_dirs(app_binary: &str) -> Vec<PathBuf> {
    let mut dirs = vec![PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/fonts")];

    if let Ok(exe) = std::env::current_exe() {
        if let Some(exe_dir) = exe.parent() {
            dirs.push(exe_dir.join("assets/fonts"));                       // Windows/install root or portable root.
            dirs.push(exe_dir.join("..").join("Resources").join("assets/fonts")); // macOS .app.
        }
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    dirs.push(PathBuf::from("/usr/lib").join(app_binary).join("assets/fonts"));

    dirs
}

fn init_with_external_then_embedded(cx: &mut gpui::App) {
    let mut options = FontLoadOptions::new(FontLoadMode::ExternalThenEmbedded).embedded(
        "PingFangSC-Regular.ttf",
        Cow::Borrowed(include_bytes!("../assets/fonts/PingFangSC-Regular.ttf").as_slice()),
    )
    .require_family("PingFang SC");

    for dir in font_dirs("my-gpui-app") {
        options = options.external_dir(dir);
    }

    let report = load_app_fonts(cx, options);
    if !report.failures.is_empty() || !report.required_families_available() {
        eprintln!("font load failures: {report:?}");
    }

    // Mixed source example: UI uses the shipped PingFang family, code uses a system family.
    init_liora_with_options(
        cx,
        LioraOptions::system().with_fonts(
            FontConfig::system()
                .with_ui_family("PingFang SC")
                .with_code_family("Monospace"),
        ),
    );
}
```

#### Load from GPUI assets or explicit files

```rust
use liora::{load_font_assets, load_font_files, load_fonts_from_dir};

fn register_more_fonts(cx: &mut gpui::App) {
    let asset_report = load_font_assets(cx, ["fonts/Brand-Regular.otf".into()]);
    let dir_report = load_fonts_from_dir(cx, "assets/fonts");
    let file_report = load_font_files(cx, [std::path::PathBuf::from("/opt/my-app/fonts/BrandCode.ttf")]);

    for report in [asset_report, dir_report, file_report] {
        if !report.failures.is_empty() {
            eprintln!("font load failures: {report:?}");
        }
    }
}
```

For Liora's own apps, Gallery and Docs keep the full PingFangSC TTF family under each app's `assets/fonts/PingFangSC/`. The app binaries embed only `PingFangSC-Regular.ttf` as a raw-executable fallback; the packaging pipeline mounts the full `assets/fonts` directory externally for installers and portable archives.

### Overlay and portal rendering

Most apps only need `liora::init_liora(cx)`. If you build a custom root shell that manually manages overlay layers, keep portal rendering near the window root:

```rust
use liora::core::{
    render_active_drawer_in_window, render_active_modal_in_window, render_active_popover_in_window,
};

fn render_overlays(window: &mut gpui::Window, cx: &mut gpui::App) {
    render_active_popover_in_window(window, cx);
    render_active_modal_in_window(window, cx);
    render_active_drawer_in_window(window, cx);
}
```

### Keep app state in app views

Do not put product data models in `liora-components`. Store app state in your GPUI view/entity and pass only display values/callbacks to components:

```rust
use gpui::{Context, IntoElement, Render, Window};
use liora::components::{Empty, Table, TableColumn, TableRow, Tag, Text};

struct OrdersView {
    rows: Vec<(String, String)>,
}

impl Render for OrdersView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        if self.rows.is_empty() {
            Empty::new().description("No orders yet").into_any_element()
        } else {
            let rows = self.rows.iter().map(|(id, status)| {
                TableRow::new()
                    .cell("id", Text::new(id.clone()))
                    .cell("status", Tag::new(status.clone()).success())
            });

            Table::new(vec![TableColumn::new("id", "Order"), TableColumn::new("status", "Status")])
                .rows(rows)
                .into_any_element()
        }
    }
}
```

## Component catalog

| Category | Components |
|---|---|
| Basic and layout | `Button`, `ButtonGroup`, `Icon`, `Link`, `Text`, `Title`, `Paragraph`, `Space`, `Divider`, `Row`, `Col`, `Container`, `Sidebar`, `TitleBar`, `Flex`, `Scrollbar`, `Splitter`, `Affix`, `Backtop` |
| Form controls | `Input`, `InputNumber`, `Textarea`, `Checkbox`, `CheckboxGroup`, `Radio`, `RadioGroup`, `Switch`, `Select`, `Slider`, `Form`, `FormItem`, `Rate`, `DatePicker`, `TimePicker`, `DateTimePicker`, `Upload`, `Cascader`, `Transfer`, `ColorPicker`, `Autocomplete`, `InputTag`, `Mention`, `TreeSelect`, `OtpInput` |
| Feedback and overlays | `Alert`, `Tooltip`, `Popover`, `Popconfirm`, `Dialog`, `Drawer`, `Message`, `Notification`, `MessageBox`, `Loading`, `Dropdown`, `DropdownButton`, `Preview`, `Tour` |
| Navigation | `Menu`, `Tabs`, `Breadcrumb`, `Steps`, `PageHeader`, `Anchor`, `Accordion` |
| Data display | `Table`, `VirtualizedTable`, `VirtualizedTree`, `VirtualizedList`, `Progress`, `Skeleton`, `Empty`, `Result`, `Descriptions`, `Timeline`, `Tree`, `Pagination`, `Statistic`, `Segmented`, `Tag`, `Avatar`, `Badge`, `Calendar`, `Carousel`, `Image`, `Watermark`, `Kbd` |
| Charts and metrics | `LineChart`, `AreaChart`, `BarChart`, `PieChart`, `RingChart`, `Sparkline`, `SignalMeter`, `HeatBar`, `SegmentRatioBar` |
| Editing and utility | `CodeBlock`, `CodeEditor`, `QrCode`, `Timer`, `Label`, `Operation`, draggable list helpers |
| App shell and platform | `AppWindowFrame`, `TitleBar`, `Sidebar`, `WindowFrameMode`, `liora-tray`, Linux desktop identity helpers, package metadata helpers, updater helpers |

## Native packaging

Repository-owned packaging readiness is implemented through the published `liora-packager` library plus the repository-local `xtask` command wrapper:

```bash
cargo run -p xtask -- package validate
cargo run -p xtask -- package release-readiness
cargo run -p xtask -- package build --all-apps
cargo run -p xtask -- package ci --app gallery --format platform-defaults --skip-build
cargo run -p xtask -- package smoke --app gallery --format platform-defaults
cargo run -p xtask -- package install-smoke --app gallery --format platform-defaults --dry-run
```

Supported release artifacts include:

| Platform | Raw apps | Gallery installers/packages |
|---|---|---|
| Linux x64 | `liora-docs`, `liora-gallery` | AppImage, `.deb`, `.rpm`, portable `.tar.gz` |
| macOS arm64 | `liora-docs`, `liora-gallery` | `.dmg` |
| Windows x64 | `liora-docs.exe`, `liora-gallery.exe` | NSIS setup `.exe`, MSI |

Packaging rules:

- keep apps pure Rust + GPUI native;
- keep app icons and tray/status icons in app-owned asset folders;
- use `liora-packager`/`xtask` for package metadata instead of adding a web runtime;
- Windows app build scripts should embed icon/file metadata only; GPUI's Windows backend already provides the application manifest.

## Troubleshooting

### `gpui 0.2.2` is selected and Liora does not compile

Your application is missing the root patch:

```toml
[patch.crates-io]
gpui = { git = "https://github.com/zed-industries/zed", rev = "2c346f60a76fe3f0367ef924927f50a6efdf5718" }
```

Verify with:

```bash
cargo tree -i gpui
cargo tree -p gpui
```

### `use of unresolved crate gpui` or `gpui_platform`

Add direct dependencies when your final binary names GPUI types or starts the app runtime:

```toml
[dependencies]
gpui = { version = "0.2.2", default-features = false }
gpui_platform = { git = "https://github.com/zed-industries/zed", rev = "2c346f60a76fe3f0367ef924927f50a6efdf5718", default-features = false }
```

### Linux build fails because GTK, Wayland, X11, Vulkan, ALSA, or font packages are missing

Install native build dependencies. On Fedora-like systems, inspect and adapt:

```bash
scripts/install-fedora-deps.sh
```

On Debian/Ubuntu-like systems, install the equivalent `libgtk-3-dev`, Wayland/X11/xkbcommon, fontconfig/freetype, Vulkan, ALSA, and `pkg-config` packages.

### Icons render in titlebar but not in the Linux taskbar

On Wayland, register a desktop identity and set `WindowOptions.app_id` to the same name. The compositor resolves taskbar icons from `.desktop` metadata and the icon theme.

```rust
WindowOptions {
    app_id: Some("acme-notes".into()),
    ..Default::default()
}
```

### Dark/System theme flashes on startup

Create windows with `show: false`, call `attach_system_theme_observer(window, cx)` at the beginning of the `open_window` callback, and activate the window after `open_window` returns.

### Input text or focus state resets during render

Store stateful controls in `gpui::Entity<T>` fields rather than constructing them inside every render pass.

```rust
struct ViewState {
    input: gpui::Entity<liora::components::Input>,
}
```

### Toast macros panic because `MessageManager` is missing

Use `liora::init_liora(cx)` or `liora_components::init_liora(cx)`. If you intentionally use only `liora_core`, initialize component services yourself before calling toast helpers.

### Windows link fails with duplicate `MANIFEST` resource

Do not embed your own Windows Common Controls manifest in app `build.rs` when using GPUI's Windows backend. Embed icons and file metadata only; `gpui_platform` enables GPUI's `windows-manifest` feature for Windows.

### Release package contains too many files

Only upload distributable raw binaries, installers/packages, and `SHA256SUMS.txt`. Keep generated notes/config files in the release body or CI artifacts, not as end-user release assets.

### Which docs should I read next?

- `apps/liora-docs/content/pages/quick_start.md` for adoption setup.
- `apps/liora-docs/content/pages/theme_system.md` for startup theme/window behavior.
- `apps/liora-docs/content/pages/packaging_workflow.md` for release packaging.
- `apps/liora-gallery/src/demos/` for component-by-component usage.

## Quality gates

Before publishing or submitting changes, run:

```bash
cargo fmt --all --check
cargo check --workspace --all-targets
cargo test --workspace
cargo check -p liora-docs --bin check_snippets
cargo doc --workspace --no-deps
cargo run -p xtask -- package validate
cargo run -p xtask -- package release-readiness
cargo run -p xtask -- package ci --app gallery --format platform-defaults --dry-run --skip-build
cargo run -p xtask -- package install-smoke --app gallery --format platform-defaults --dry-run
```

For release builds:

```bash
cargo build --workspace --release
cargo run --release -p xtask -- package validate
cargo run --release -p xtask -- package release-readiness
```

## Design principles

- **Native first**: all components render through GPUI element trees, native text, native input, and native paint paths.
- **Application-ready defaults**: theme, overlay, message, keyboard, and selection behavior work from one setup call.
- **Composable over prescriptive**: components expose builder-style APIs; product data and screen composition stay in applications.
- **Token-driven visuals**: light/dark/system themes use semantic tokens for surfaces, text, borders, masks, and interaction states.
- **Performance-aware data UI**: charts and virtualized views include downsampling, hit testing, cache limits, and visible-area rendering patterns.

## Runtime model

`liora::init_liora(cx)` is the recommended application entry point when using the facade crate. It initializes Liora core/theme state, global component services, and key bindings for interactive controls.

Use `liora::init_liora_with_mode(cx, ThemeMode::Light | ThemeMode::Dark | ThemeMode::System)` when the product needs to choose an explicit startup theme mode. Runtime theme switches use `apply_theme_mode(window, cx, mode)` from `liora_core` or the facade's `core` module.

Typography defaults are system-native: Liora does not load branded fonts by default and does not map the whole UI to Zed-specific font aliases. Custom fonts are opt-in via `FontConfig`, `LioraOptions`, `load_app_fonts`, `load_fonts_from_dir`, `load_font_assets`, `load_embedded_fonts`, the low-level `load_custom_fonts`, and `set_font_config`.

Stateful controls such as `Input`, `Switch`, `Select`, `TreeSelect`, `CodeEditor`, and virtualized views should live in `gpui::Entity<T>` fields so focus, open state, selections, scroll state, and text values survive re-rendering.

## Technical differentiators

Liora is more than a component catalog:

- **One-dependency adoption**: the crates.io `liora` facade re-exports the maintained public SDK modules so app manifests stay compact while focused utility crates remain independently usable.
- **One-call application setup**: `init_liora(cx)` centralizes core configuration, component services, and keyboard bindings so applications do not repeat per-widget setup.
- **Native Markdown documentation**: Markdown stays as authored content, while the running Docs app renders it into Liora/GPUI nodes and verifies external Rust snippets.
- **Native charts without a browser layer**: chart primitives use Rust data structures, GPUI paint paths, hit testing, and downsampling instead of a WebView chart runtime.
- **Application-shell coverage**: tray residency, toasts, theme switching, searchable component navigation, and real layout patterns are exercised in native apps.
- **Packaging-aware from the workspace**: installer information, manifests, checksums, backend configs, and dry-run install plans are validated alongside code.

## Documentation maintenance rule

Every future code change must ask: **does README need to change?**

Update `README.md` and `README.zh-CN.md` in the same change when you modify:

- public crate names, features, or dependency instructions;
- GPUI revision, patch strategy, or platform feature flags;
- initialization APIs, theme behavior, fonts, icons, window startup, or tray behavior;
- component names, major component APIs, examples, or app-shell patterns;
- packaging, updater, release assets, CI commands, MSRV, or troubleshooting guidance.

If README does not need a change, state that explicitly in the final change summary.

## Contributing

Read `CONTRIBUTING.md` before opening a pull request. Important boundaries:

- keep Liora pure Rust + GPUI native;
- do not introduce Tauri, WebView, HTML/CSS/DOM, browser runtime, or web chart shells;
- do not put product data models or page-only helpers into `liora-components`;
- keep Gallery, Docs, snippets, tests, and both READMEs in sync with public behavior.

## License

Liora currently uses `LicenseRef-Liora`; see `LICENSE.md`. Do not assume an OSS license until the project maintainer replaces that policy with explicit OSS or commercial terms.
