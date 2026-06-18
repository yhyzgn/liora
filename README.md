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
    <a href="docs/release-candidate-checklist.md">Release Candidate Checklist</a>
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
    <img alt="Release candidate" src="https://img.shields.io/badge/0.1.0-RC%20ready-0ea5e9">
  </p>
</div>

---

## What is Liora?

**Liora** is a monorepo for building polished, enterprise-style **Rust native desktop UI** with [GPUI](https://github.com/zed-industries/zed). It brings a broad Element Plus-inspired component taxonomy to native Rust applications: basic elements, forms, overlays, navigation, data display, advanced inputs, native charts, virtualized data views, code display/editing, tray integration, and installer packaging infrastructure.

Liora is intentionally not a web shell. Applications built with Liora stay on the **pure Rust + GPUI native** path:

- no Tauri runtime;
- no WebView, HTML/CSS/DOM, or browser application shell;
- no web chart runtime, SVG DOM charting layer, or frontend build pipeline;
- Gallery and Docs are maintained native GPUI apps, not standalone sample products.

## Why Liora?

Rust desktop teams often need more than a handful of primitives. Liora focuses on the missing middle between low-level GPUI layout code and real product screens:

| Need | Liora answer |
|---|---|
| Native desktop UI | GPUI element trees, native windows, native text/layout/paint paths. |
| Enterprise component coverage | Element Plus-style categories and APIs across forms, feedback, data, navigation, charts, and advanced controls. |
| Product dogfooding | `liora-gallery` and `liora-docs` exercise real composition, theme switching, search/filtering, tray behavior, docs rendering, and dashboard-style patterns. |
| Theming | Light, Dark, and System theme modes with semantic tokens and component-level variants. |
| Release readiness | `liora-packager` + `xtask package` validate native installer metadata, manifests, checksums, signing policy, and release gates. |
| Clear architecture boundary | Reusable components stay in `liora-components`; app-specific sample models and dashboard glue stay in Gallery/Docs. |

## Feature highlights

- **70+ native UI components** across Basic, Form, Feedback, Data, Navigation, and Others categories.
- **Element Plus-inspired API style** adapted for Rust builders and GPUI rendering.
- **Native charting**: Line, Area, Bar, Pie, Ring, Sparkline, scales, grids, legends, downsampling, and hover hit testing.
- **Advanced controls**: CodeEditor, CodeBlock, QR code, Timer, SignalMeter, HeatBar, SegmentRatioBar, draggable lists, Tour, TreeSelect, Mention, InputTag, Watermark, virtualized table/tree.
- **Overlay and interaction systems**: Tooltip, Popover, Popconfirm, Dialog, Drawer, Dropdown, Message, Notification, MessageBox, Loading, Preview, Tour.
- **Native docs renderer**: Markdown is parsed as input and rendered into Liora/GPUI native nodes; code snippets live outside Markdown and are compile-checked.
- **System tray facade**: `liora-tray` wraps `tray-icon` + `muda` for dynamic icons, nested menus, checkbox items, stable commands, and process-resident GPUI apps.
- **Installer pipeline**: metadata validation, `cargo-packager` config generation, RPM supplemental config, portable `.tar.gz`, manifests, checksums, release notes, and CI dry-run gates.
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
| Platform / App shell | `liora-tray`, custom window-frame polish, Gallery shell patterns, Docs adoption pages, packaging helpers |

## Repository layout

```text
liora/
├── crates/
│   ├── liora-core/            # global config, theme setup, popper/portal state, unique IDs
│   ├── liora-theme/           # semantic tokens, light/dark/system theme support
│   ├── liora-components/      # reusable GPUI components
│   ├── liora-icons/           # native icon trait and helpers
│   ├── liora-icons-lucide/    # generated Lucide icon names and paths
│   ├── liora-tray/            # tray-icon + muda facade for GPUI apps
│   └── liora-packager/        # package metadata, manifests, checksums, backend config
├── apps/
│   ├── liora-gallery/         # native component gallery and dogfooding shell
│   └── liora-docs/            # native documentation app and Markdown renderer
├── xtask/                     # cargo run -p xtask -- package ...
├── packaging/                 # icons, desktop/metainfo, macOS/Windows/Linux package resources
├── docs/                      # technical plans, RC checklist, repository metadata guidance
├── .prompt/                   # phase prompts and maintenance contracts
├── .memory/                   # project state, decisions, inventory, session history
└── Cargo.toml                 # workspace root
```

## Quick start

### 1. Install prerequisites

Install Rust stable and the native dependencies required by GPUI on your platform. On Linux, common development packages include GTK3, Wayland/X11, xkbcommon, fontconfig/freetype, Vulkan, ALSA, and `pkg-config`. The repository also includes `scripts/install-fedora-deps.sh` for Fedora-oriented setup.

### 2. Run the native Gallery

```bash
cargo run -p liora-gallery
```

The Gallery is the canonical visual surface for component demos, theme switching, search/filtering, tray controls, toasts, and product-style composition.

### 3. Run the native Docs app

```bash
cargo run -p liora-docs
```

The Docs app is the canonical adoption surface. It renders Markdown content into native Liora/GPUI elements and shows compile-checked snippets from `apps/liora-docs/content/snippets/`.

### 4. Check the workspace

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
use liora_core::{init_liora_with_mode, ThemeMode};

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

Prefer `init_liora_with_mode(cx, ThemeMode::System)` so applications follow the operating system by default. Use `Entity<T>` for stateful controls such as `Input`, `Switch`, `Select`, and `CodeEditor` so focus and internal state survive re-rendering. Gallery and Docs are the maintained compile-checked references for app shell setup, key binding registration, theme switching, tray behavior, toasts, and composition patterns.

## Component API example

Liora components follow a builder style and render through GPUI-native elements:

```rust
use gpui::{div, IntoElement, RenderOnce};
use liora_components::{Button, Space, Tag, Text, Title};

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

## Documentation map

| Resource | Purpose |
|---|---|
| `apps/liora-docs` | Native documentation app, adoption guide, component pages, and release docs. |
| `apps/liora-gallery` | Native component Gallery and app-shell dogfooding surface. |
| `apps/liora-docs/content/pages/` | Markdown source pages rendered by the native Docs app. |
| `apps/liora-docs/content/snippets/` | External code snippets referenced by Markdown and checked by `check_snippets`. |
| `docs/release-candidate-checklist.md` | Local `0.1.0` RC source of truth. |
| `docs/packaging-installer-technical-plan.md` | Packaging architecture and platform release notes. |
| `assets/github-repository-metadata.md` | Recommended GitHub About description and SEO topics. |
| `.prompt/` and `.memory/` | Phase contracts, current state, decisions, inventory, and session history. |

## Native packaging and release readiness

Repository-owned packaging readiness is implemented through `liora-packager` and `xtask`:

```bash
cargo run -p xtask -- package validate
cargo run -p xtask -- package release-readiness
cargo run -p xtask -- package ci --all-apps --format platform-defaults --dry-run --skip-build
cargo run -p xtask -- package install-smoke --all-apps --format platform-defaults --dry-run
```

The packaging pipeline can generate backend config for AppImage, `.deb`, `.rpm`, macOS `.app` / `.dmg`, Windows NSIS / MSI, and Liora portable `.tar.gz` archives. Formal release publication remains protected-environment work: macOS notarization, Windows signing, destructive system-level install/uninstall smoke, and real `vX.Y.Z` GitHub Release publication must be performed only by the owner-controlled release path.

See `docs/release-candidate-checklist.md` for the current `0.1.0` RC gate.

## Development workflow

For component or feature work, keep changes small and verifiable:

1. Implement or update the component in `crates/liora-components/src/`.
2. Export the API from `crates/liora-components/src/lib.rs` when needed.
3. Add or update a Gallery demo in `apps/liora-gallery/src/demos/`.
4. Add or update native Docs content and external snippets.
5. Add focused tests for behavior, calculations, parsing, or release boundaries.
6. Run the smallest relevant check first, then the workspace gate.
7. Update `.memory/` when code behavior, component inventory, or phase state changes.

## GitHub SEO metadata

Recommended repository description:

> Pure Rust + GPUI native enterprise UI component library for desktop apps — Element Plus-inspired components, charts, docs, tray integration, and installer packaging.

Recommended topics are maintained in `assets/github-repository-metadata.md`. GitHub topics help discovery and should use lowercase letters, numbers, and hyphens with no more than 20 topics, per GitHub Docs.

## Current status

Local implementation phases are complete through **P21 Release Candidate Readiness**. The repository is ready for owner-controlled `0.1.0` release-candidate validation. Protected release-only items remain outside ordinary local development: real signing credentials, notarization, destructive installer smoke tests, formal license replacement, and public release publication.

## Contributing

Read `CONTRIBUTING.md` before opening a pull request. Important boundaries:

- Keep Liora pure Rust + GPUI native.
- Do not introduce Tauri, WebView, HTML/CSS/DOM, browser runtime, or web chart shells.
- Do not re-add standalone `examples/minimal-app` or `examples/dashboard-app`.
- Keep business sample models and dashboard-only helper code out of `liora-components`.
- Keep `.omx/**` local and out of commits.

## License

Liora currently uses `LicenseRef-Liora`; see `LICENSE.md`. Do not assume an OSS license until the project owner replaces that policy with explicit OSS or commercial terms.
