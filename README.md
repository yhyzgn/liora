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
    <img alt="Rust 2024" src="https://img.shields.io/badge/Rust-2024-dea584?logo=rust&logoColor=white">
    <img alt="GPUI native" src="https://img.shields.io/badge/GPUI-native-7c3aed">
    <img alt="Pure Rust" src="https://img.shields.io/badge/runtime-pure%20rust%20native-10b981">
    <img alt="LicenseRef-Liora" src="https://img.shields.io/badge/license-LicenseRef--Liora-64748b">
    <img alt="Native packaging" src="https://img.shields.io/badge/packaging-native-0ea5e9">
  </p>
</div>

---

## What is Liora?

**Liora** is a monorepo for building polished, enterprise-style **Rust native desktop UI** with [GPUI](https://github.com/zed-industries/zed). It brings a broad Element Plus-inspired component taxonomy to native Rust applications: basic elements, forms, overlays, navigation, data display, advanced inputs, native charts, virtualized data views, code display/editing, tray integration, and installer packaging infrastructure.

Liora is intentionally not a web shell. Applications built with Liora stay on the **pure Rust + GPUI native** path:

- no Tauri runtime;
- no WebView, HTML/CSS/DOM, or browser application shell;
- no web chart runtime, SVG DOM charting layer, or frontend build pipeline;
- Gallery and Docs are native GPUI applications that demonstrate the library in real app shells.

## Why Liora?

Rust desktop teams often need more than a handful of primitives. Liora focuses on the missing middle between low-level GPUI layout code and real product screens:

| Need | Liora answer |
|---|---|
| Native desktop UI | GPUI element trees, native windows, native text/layout/paint paths. |
| Enterprise component coverage | Element Plus-style categories and APIs across forms, feedback, data, navigation, charts, and advanced controls. |
| Real app surfaces | `liora-gallery` and `liora-docs` show complete native application setup, theme switching, search/filtering, tray behavior, docs rendering, and dashboard-style composition. |
| Theming | Light, Dark, and System theme modes with semantic tokens and component-level variants. |
| One git dependency for apps | `liora` is consumed from this repository at a pinned commit and exposes the SDK facade: core, theme, components, icons, tray, packaging, and generic updater helpers. |
| Native distribution | `liora-packager` + `xtask package` validate installer information, manifests, checksums, signing policy, and package generation plans. |
| Clear architecture boundary | Reusable components stay in `liora-components`; product-specific data models and screen composition stay in applications. |

## Feature highlights

- **One-stop SDK facade**: depend on `liora` for application development, or use focused crates such as `liora-components`, `liora-theme`, and `liora-packager` when you need narrower surfaces.
- **70+ native UI components** across Basic, Form, Feedback, Data, Navigation, and Others categories.
- **Element Plus-inspired API style** adapted for Rust builders and GPUI rendering.
- **Native charting**: Line, Area, Bar, Pie, Ring, Sparkline, scales, grids, legends, downsampling, and hover hit testing.
- **Advanced controls**: CodeEditor, CodeBlock, QR code, Timer, SignalMeter, HeatBar, SegmentRatioBar, draggable lists, Tour, TreeSelect, Mention, InputTag, Watermark, virtualized table/tree.
- **Overlay and interaction systems**: Tooltip, Popover, Popconfirm, Dialog, Drawer, Dropdown, Message, Notification, MessageBox, Loading, Preview, Tour.
- **Native docs renderer**: Markdown is parsed as input and rendered into Liora/GPUI native nodes; code snippets live outside Markdown and are compile-checked.
- **System tray facade**: `liora-tray` wraps `tray-icon` + `muda` for dynamic icons, nested menus, checkbox items, stable commands, and process-resident GPUI apps.
- **Installer pipeline**: package information validation, `cargo-packager` config generation, RPM supplemental config, portable `.tar.gz`, manifests, checksums, release notes, and CI validation gates.
- **Reusable updater pipeline**: `liora-updater` checks GitHub Releases for any configured repository, selects assets with app/platform naming rules, downloads into a cache, verifies SHA-256 manifests, and returns an explicit install plan. Liora Gallery/Docs use small built-in presets on top of the generic API.
- **Quality gates**: workspace fmt/check/test, docs snippet checks, package validation, release-readiness checks, and GUI startup smoke commands.

## Component coverage

| Category | Components and capabilities |
|---|---|
| Basic | Button, ButtonGroup, Icon, Link, Text, Title, Paragraph, Space, Divider, Row, Col, Container, Scrollbar, Splitter, CodeBlock |
| Form | Input, InputNumber, Textarea, Checkbox, CheckboxGroup, Radio, RadioGroup, Switch, Select, Slider, Form, FormItem, Rate, DatePicker, TimePicker, DateTimePicker, Upload, Cascader, Transfer, ColorPicker, Autocomplete, InputTag, Mention, TreeSelect |
| Feedback / Overlay | Tooltip, Popover, Popconfirm, Dialog, Drawer, Message, Notification, Alert, Loading, MessageBox, Dropdown, Card, Collapse, Preview, Tour |
| Navigation | Menu, Tabs, Breadcrumb, Steps, PageHeader, Affix, Backtop, Anchor |
| Data display | Table, VirtualizedTable, VirtualizedTree, VirtualizedList, Progress, Skeleton, Empty, Result, Descriptions, Timeline, Tree, Pagination, Statistic, Segmented, Tag, Avatar, Badge, Calendar, Carousel, Image, Watermark |
| Charts / Metrics | LineChart, AreaChart, BarChart, PieChart, RingChart, Sparkline, SignalMeter, HeatBar, SegmentRatioBar |
| Editing / Utility | CodeEditor, QrCode, Timer, Label, Operation, draggable horizontal and vertical list patterns |
| Platform / App shell | `liora-tray`, custom window frame, Gallery shell composition, Docs adoption pages, packaging helpers |

## Repository layout

```text
liora/
├── crates/
│   ├── liora/                 # one-stop SDK facade for application dependencies
│   ├── liora-core/            # global config, theme setup, popper/portal state, unique IDs
│   ├── liora-theme/           # semantic tokens, light/dark/system theme support
│   ├── liora-components/      # reusable GPUI components
│   ├── liora-icons/           # native icon trait and helpers
│   ├── liora-icons-lucide/    # generated Lucide icon names and paths
│   ├── liora-tray/            # tray-icon + muda facade for GPUI apps
│   ├── liora-packager/        # package info, manifests, checksums, backend config
│   └── liora-updater/         # GitHub Release checks, downloads, checksums, install plans
├── apps/
│   ├── liora-gallery/         # native component gallery and showcase application
│   └── liora-docs/            # native documentation app and Markdown renderer
├── xtask/                     # cargo run -p xtask -- package ...
├── packaging/                 # icons, desktop/metainfo, macOS/Windows/Linux package resources
└── Cargo.toml                 # workspace root
```

## Quick start

### 1. Install prerequisites

Install Rust stable and the native dependencies required by GPUI on your platform. On Linux, common development packages include GTK3, Wayland/X11, xkbcommon, fontconfig/freetype, Vulkan, ALSA, and `pkg-config`. The repository also includes `scripts/install-fedora-deps.sh` for Fedora-oriented setup.

### 2. Add Liora to an app

Liora is published on crates.io, while GPUI should be resolved from the official Zed repository revision that Liora was developed and verified against. Add Liora normally, then patch Cargo's `gpui` registry fallback to the matching official Zed git revision:

```toml
[dependencies]
liora = "0.1"
gpui = { version = "0.2.2", default-features = false }

# Applications that open windows also depend on gpui_platform directly.
gpui_platform = { git = "https://github.com/zed-industries/zed", rev = "2c346f60a76fe3f0367ef924927f50a6efdf5718", default-features = false }

[patch.crates-io]
gpui = { git = "https://github.com/zed-industries/zed", rev = "2c346f60a76fe3f0367ef924927f50a6efdf5718" }
```

Use lower-level Liora crates from crates.io when a workspace needs a narrower dependency surface. The top-level `liora` crate remains the recommended one-dependency facade.

### Updater module

`liora-updater` is included in the default git `liora` facade and can also be used directly from crates.io:

```bash
cargo add liora-updater
```

It provides a reusable GitHub Release update layer for your own applications: configure the owner/repo, current tag, app name, platform selector, cache directory, checksum asset name, and installer preference. Gallery and Docs use Liora presets, but those presets are not required for other products.

```rust
use liora_updater::{
    AssetKind, AssetSelector, Platform, UpdateRequest, Updater,
};

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
    // Show update.release_tag(), update.asset.name, and update.install_plan in your UI.
    // Run update.install_plan.install() only from an explicit user action.
}
# Ok::<(), liora_updater::UpdaterError>(())
```

The updater can automatically check, download, and verify assets, while installation remains a visible user action because some installer formats require OS elevation or replacing a running executable.

### 3. Run the native Gallery

```bash
cargo run -p liora-gallery
```

The Gallery presents component demos, theme switching, search/filtering, tray controls, toasts, and product-style composition in a native GPUI window.

### 4. Run the native Docs app

```bash
cargo run -p liora-docs
```

The Docs app explains adoption and component usage. It renders Markdown content into native Liora/GPUI elements and shows compile-checked snippets from `apps/liora-docs/content/snippets/`.

### 5. Check the workspace

```bash
cargo fmt --all --check
cargo check --workspace --all-targets
cargo test --workspace
cargo check -p liora-docs --bin check_snippets
cargo doc --workspace --no-deps
```

## Minimal application shape

A Liora-powered GPUI app should initialize theme/config, initialize global services used by selected components, register component key bindings, and then open a GPUI window.

```rust
use gpui::App;
use liora::init_liora;

fn main() {
    gpui_platform::application().run(|cx: &mut App| {
        // Initializes Liora core/theme state, component services, and key bindings.
        init_liora(cx);

        // cx.open_window(...)
    });
}
```

`liora::init_liora(cx)` follows the operating system by default and also initializes component services plus key bindings. Use `liora::init_liora_with_mode(cx, ThemeMode::Light | ThemeMode::Dark | ThemeMode::System)` when a product wants to choose the startup mode explicitly. The lower-level `liora_components::init_liora(...)` entry point remains available for users who depend on individual crates instead of the facade. Use `Entity<T>` for stateful controls such as `Input`, `Switch`, `Select`, and `CodeEditor` so focus and internal state survive re-rendering. Gallery and Docs are compile-checked references for app shell setup, key binding registration, theme switching, tray behavior, toasts, and composition patterns.

Liora does not bundle or force a default UI font. Normal text uses GPUI's platform/system UI font, and code-oriented surfaces use GPUI's generic monospace family unless an application opts into custom typography. To use a bundled or user-selected font, first register the font bytes with `liora::load_custom_fonts(cx, ...)`, then pass family names through `liora::init_liora_with_options(cx, LioraOptions::system().with_fonts(...))` or update them later with `liora::set_font_config(cx, ...)`.

## Component API example

Liora components follow a builder style and render through GPUI-native elements:

```rust
use gpui::{div, IntoElement, RenderOnce};
use liora::components::{Button, Space, Tag, Text, Title};

struct WelcomePanel;

impl RenderOnce for WelcomePanel {
    fn render(self, _window: &mut gpui::Window, _cx: &mut gpui::App) -> impl IntoElement {
        div()
            .child(Title::new("Native Rust UI").level(2))
            .child(Text::new("Build enterprise desktop screens with GPUI and Liora."))
            .child(
                Space::new()
                    .child(Button::new("Open Gallery").primary())
                    .child(Tag::new("Pure Rust").success()),
            )
    }
}
```

Themes are read from Liora global config inside render paths. Avoid passing theme objects through `.build(theme)`-style APIs.

## Technical differentiators

Liora is more than a component catalog:

- **One-dependency adoption**: the crates.io `liora` facade re-exports the maintained public SDK modules so app manifests stay compact while focused utility crates remain independently usable.
- **One-call application setup**: `init_liora(cx)` centralizes core configuration, component services, and keyboard bindings so applications do not repeat per-widget setup.
- **Native Markdown documentation**: Markdown stays as authored content, while the running Docs app renders it into Liora/GPUI nodes and verifies external Rust snippets.
- **Native charts without a browser layer**: chart primitives use Rust data structures, GPUI paint paths, hit testing, and downsampling instead of a WebView chart runtime.
- **Application-shell coverage**: tray residency, toasts, theme switching, searchable component navigation, and dashboard-style layouts are exercised in real native apps.
- **Packaging-aware from the workspace**: installer information, manifests, checksums, backend configs, and dry-run install plans are validated alongside code.

## Documentation map

| Resource | Purpose |
|---|---|
| `apps/liora-docs` | Native documentation app, adoption guide, component pages, and compile-checked snippets. |
| `apps/liora-gallery` | Native component Gallery and app-shell reference surface. |
| `apps/liora-docs/content/pages/` | Markdown source pages rendered by the native Docs app. |
| `apps/liora-docs/content/snippets/` | External code snippets referenced by Markdown and checked by `check_snippets`. |

## GPUI dependency and local patch policy

Liora depends on the official Zed upstream repository, pinned by commit for reproducible local builds. Published crates use Cargo's supported multiple-location dependency form: the workspace resolves `gpui` from the official Zed git rev locally, while crates.io receives a registry fallback dependency on `gpui = 0.2.2`.

```toml
[dependencies]
liora = "0.1"
gpui = { version = "0.2.2", default-features = false }

[patch.crates-io]
gpui = { git = "https://github.com/zed-industries/zed", rev = "2c346f60a76fe3f0367ef924927f50a6efdf5718" }
```

The `[patch.crates-io]` entry is intentional: it makes every transitive `gpui` dependency, including Liora's, resolve to the selected official Zed commit. Without that patch, Cargo will try the stale registry `gpui 0.2.2` fallback and modern Liora components may not compile. Library crates keep `default-features = false`; final app crates enable platform features on `gpui` and `gpui_platform` in target-specific dependencies. Do not use renamed or community forks such as `open-gpui`.

The repository keeps `third_party/zed` only as non-published upstream-source reference material for prior Linux startup-window patch work and PR comparison. Current development should use the official `zed-industries/zed` git dependency above. If a temporary local patch is needed for app-only verification, keep it on a throwaway branch and never commit it to publishable SDK manifests.

## Native packaging

Repository-owned packaging readiness is implemented through the published `liora-packager` library plus the repository-local `xtask` command wrapper:

```bash
cargo run -p xtask -- package validate
cargo run -p xtask -- package release-readiness
cargo run -p xtask -- package ci --all-apps --format platform-defaults --dry-run --skip-build
cargo run -p xtask -- package install-smoke --all-apps --format platform-defaults --dry-run
```

The packaging pipeline can generate backend config for AppImage, `.deb`, `.rpm`, macOS `.app` / `.dmg`, Windows NSIS / MSI, and Liora portable `.tar.gz` archives. Signing, notarization, installer smoke tests, and publication are intentionally separated from everyday development commands so application code remains pure Rust + GPUI native.

## Quality gates

Before publishing or submitting changes, run the same local checks used by the project:

```bash
cargo fmt --all --check
cargo check --workspace --all-targets
cargo test --workspace
cargo check -p liora-docs --bin check_snippets
cargo doc --workspace --no-deps
cargo run -p xtask -- package validate
cargo run -p xtask -- package ci --all-apps --format platform-defaults --dry-run --skip-build
```

Component changes should include the reusable component API, Gallery coverage, native Docs content, external snippets, and focused tests for calculations or interaction behavior.

## Design principles

Liora is designed around a few product-facing rules:

- **Native first**: every component renders through GPUI element trees, native text, native input, and native paint paths.
- **Application-ready defaults**: theme, overlay, message, keyboard, and selection behavior should work from one setup call.
- **Composable over prescriptive**: components expose builder-style APIs and stay reusable; product screens and data models belong in applications.
- **Token-driven visuals**: light/dark/system themes use semantic tokens for surfaces, text, borders, masks, and interaction states.
- **Performance-aware data UI**: charts and virtualized views include downsampling, hit testing, cache limits, and visible-area rendering patterns.

## Runtime model

`liora::init_liora(cx)` is the recommended application entry point when using the facade crate. `liora_components::init_liora(cx)` provides the same setup for users of the focused components crate. It initializes Liora core/theme state, global component services, and key bindings for interactive controls.

Use `liora::init_liora_with_mode(cx, ThemeMode::Light | ThemeMode::Dark | ThemeMode::System)` when the product needs to choose an explicit startup theme mode. Runtime theme switches still use `apply_theme_mode(window, cx, mode)` from `liora_core`.

Typography defaults are intentionally system-native: Liora does not load branded fonts by default and does not map the whole UI to Zed-specific font aliases. Custom fonts are opt-in via `FontConfig`, `LioraOptions`, `load_custom_fonts`, and `set_font_config`.

Stateful controls such as `Input`, `Switch`, `Select`, and `CodeEditor` should live in `gpui::Entity<T>` fields so focus, open state, selections, and text values survive re-rendering.

## Contributing

Read `CONTRIBUTING.md` before opening a pull request. Important boundaries:

- Keep Liora pure Rust + GPUI native.
- Do not introduce Tauri, WebView, HTML/CSS/DOM, browser runtime, or web chart shells.
- Do not re-add standalone `examples/minimal-app` or `examples/dashboard-app`.
- Keep product-specific data models and screen-level helper code out of `liora-components`.

## License

Liora currently uses `LicenseRef-Liora`; see `LICENSE.md`. Do not assume an OSS license until the project maintainer replaces that policy with explicit OSS or commercial terms.
