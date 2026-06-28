<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/liora-logo.svg">
    <img src="assets/liora-logo.svg" alt="Liora — 纯 Rust + GPUI 原生 UI 组件库" width="220">
  </picture>

  <p><strong>面向 Rust 桌面应用的企业级 GPUI 原生组件库。</strong></p>
  <p>纯 Rust。GPUI Native。API 体系参考 Element Plus。无 Tauri、无 WebView、无浏览器运行时。</p>

  <p>
    <a href="README.md">English</a>
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

## 目录

- [Liora 是什么？](#liora-是什么)
- [你可以用 Liora 构建什么？](#你可以用-liora-构建什么)
- [环境要求](#环境要求)
- [如何选择依赖方式](#如何选择依赖方式)
- [GPUI 依赖与本地 patch 策略](#gpui-依赖与本地-patch-策略)
- [快速开始：创建一个 Liora 应用](#快速开始创建一个-liora-应用)
- [应用初始化](#应用初始化)
- [窗口启动、系统主题与图标](#窗口启动系统主题与图标)
- [liora 下各个模块怎么用](#liora-下各个模块怎么用)
- [组件代码示例](#组件代码示例)
- [高级用法](#高级用法)
- [组件清单](#组件清单)
- [原生打包](#原生打包)
- [常见问题与排查](#常见问题与排查)
- [质量门禁](#质量门禁)
- [README 同步规范](#readme-同步规范)
- [License](#license)

## Liora 是什么？

**Liora** 是一套用于构建精致桌面应用的 **Rust + GPUI 原生组件 SDK**。它提供顶层 `liora` facade crate，同时把核心运行时、主题 token、组件、图标、系统托盘、打包元数据和 GitHub Release 更新流程拆成独立模块，方便不同规模的应用接入。

Liora 明确不是 Web 外壳：

- 不使用 Tauri runtime；
- 不引入 WebView、HTML/CSS/DOM 或浏览器应用壳；
- 不使用 Web 图表运行时或前端 bundler；
- Gallery 和 Docs 都是真实 GPUI 原生应用，并使用和下游应用相同的公开 SDK 表面。

## 你可以用 Liora 构建什么？

当你想用 Rust 构建桌面应用，并且需要以下能力时，适合使用 Liora：

| 需求 | Liora 的方案 |
|---|---|
| 原生桌面 UI | GPUI 窗口、GPUI 元素树、原生文本/布局/绘制路径。 |
| 企业级组件覆盖 | 参考 Element Plus 的组件体系，覆盖布局、表单、浮层、导航、数据展示、图表和高级输入。 |
| 一行初始化应用 | `liora::init_liora(cx)` 初始化核心状态、组件服务和 key bindings。 |
| Light/Dark/System 主题 | `ThemeMode`、语义 token、运行时切换和系统外观跟随。 |
| 系统托盘应用 | `liora-tray` 基于 `tray-icon` 和 `muda` 封装稳定命令模型。 |
| 原生发布产物 | `liora-packager` + `xtask package` 校验并生成 Linux、macOS、Windows 的打包计划。 |
| 更新能力 | `liora-updater` 检查 GitHub Releases、选择 asset、校验 SHA-256，并返回明确安装计划。 |

## 环境要求

| 项 | 要求 |
|---|---|
| Rust | `rustc 1.95+`，Rust edition 2024。 |
| UI 后端 | 官方 Zed GPUI git 依赖，并 pin 到 Liora 验证过的 revision。 |
| Linux 原生依赖 | GTK3、Wayland/X11、xkbcommon、fontconfig/freetype、Vulkan、ALSA、`pkg-config`；Fedora 可参考 `scripts/install-fedora-deps.sh`。 |
| macOS | Release workflow 覆盖 Apple Silicon；需要 Xcode Command Line Tools。 |
| Windows | MSVC toolchain；GPUI Windows backend 会通过 `windows-manifest` 提供应用 manifest。 |

## 如何选择依赖方式

大多数应用应该直接依赖顶层 facade：

```toml
[dependencies]
liora = "0.1"
```

只有当你明确需要更窄依赖面时，才直接依赖底层 focused crates：

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

`liora` facade 重新导出了稳定模块名：

```rust
use liora::{components, core, icons, icons_lucide, theme, tray};
use liora::prelude::*;

#[cfg(feature = "updater")]
use liora::updater;

#[cfg(feature = "packager")]
use liora::packager;
```

如果应用不需要 packaging 或 updater helper，可以关闭默认 feature：

```toml
[dependencies]
liora = { version = "0.1", default-features = false }

# 或者只保留 updater：
liora = { version = "0.1", default-features = false, features = ["updater"] }
```

## GPUI 依赖与本地 patch 策略

Liora **只能使用官方 Zed GPUI**。禁止使用 `open-gpui` 等重命名或社区 fork。

为什么需要额外配置 GPUI：

1. `liora` 各个 crate 已发布到 crates.io。
2. 当前 Liora 面向比 registry 里旧的 `gpui 0.2.2` 更新的官方 Zed GPUI git revision 开发。
3. Cargo 不允许 crates.io 包强制所有下游应用使用 git-only transitive dependency。
4. 因此 Liora 发布包使用 Cargo 支持的 multiple-location dependency：发布时保留 registry fallback，本地开发时使用官方 Zed git rev。
5. 最终应用必须在根 `Cargo.toml` 添加 `[patch.crates-io]`，把所有传递依赖中的 `gpui` 统一解析到官方 Zed commit。

应用推荐使用以下 manifest 模式：

```toml
[package]
name = "acme-notes"
version = "0.1.0"
edition = "2024"
publish = false

[dependencies]
liora = "0.1"

# 什么时候需要手动添加 gpui？
# 当你的 crate 直接写到这些内容时：
# - gpui::App / Window / Context / Render / RenderOnce
# - gpui::div(), px(), size(), Entity<T>
# - 函数签名中出现 fn render(..., cx: &mut gpui::Context<Self>)
gpui = { version = "0.2.2", default-features = false }

# 什么时候需要手动添加 gpui_platform？
# 最终会打开窗口的 binary crate 使用 gpui_platform::application().run(...) 时需要。
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

什么时候必须手动添加 `gpui`？看这个例子：

```rust
// 因为这个文件直接使用 gpui 类型和宏，所以需要直接依赖 gpui。
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

什么时候可以不直接使用 `gpui`？如果只是做数据模型、主题值、更新请求或打包元数据 helper，不打开窗口、不写 GPUI 类型签名，就可以只依赖 focused Liora crate。

本仓库保留 `third_party/zed` 仅作为早期 Linux 启动窗口 patch 工作和 upstream PR 对照的未发布源码材料。当前开发应使用上面的官方 `zed-industries/zed` git 依赖。如确需临时本地 patch 做 app-only 验证，必须保持在可发布 SDK manifest 之外，并明确记录边界。

## 快速开始：创建一个 Liora 应用

### 1. 创建应用

```bash
cargo new acme-notes
cd acme-notes
```

### 2. 添加依赖

可以直接复制 [GPUI 依赖与本地 patch 策略](#gpui-依赖与本地-patch-策略) 中的 manifest，也可以先使用这个覆盖 Linux/macOS/Windows 的紧凑版本：

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

### 3. 写入 `src/main.rs`

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
        // 一次调用完成 theme/config、overlay/message 服务，以及交互控件 key bindings。
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

### 4. 运行

```bash
cargo run
```

### 5. 对照完整应用参考

在本仓库中运行：

```bash
cargo run -p liora-gallery
cargo run -p liora-docs
```

`liora-gallery` 是组件展示和应用壳参考；`liora-docs` 是原生文档应用和 Markdown renderer。

## 应用初始化

普通应用推荐使用 facade 入口：

```rust
use gpui::App;
use liora::{ThemeMode, init_liora, init_liora_with_mode, init_liora_with_options};
use liora::{FontConfig, LioraOptions};

fn init_default(cx: &mut App) {
    // 推荐默认：跟随系统主题。
    init_liora(cx);
}

fn init_dark(cx: &mut App) {
    // 显式指定启动主题。
    init_liora_with_mode(cx, ThemeMode::Dark);
}

fn init_with_system_font_names(cx: &mut App) {
    // 这里不加载任何字体文件，GPUI 会从操作系统字体中解析这些 family 名称。
    let fonts = FontConfig::system()
        .with_ui_families(["Segoe UI", "PingFang SC", "Arial"])
        .with_code_families(["JetBrains Mono", "SF Mono", "Monospace"]);

    init_liora_with_options(cx, LioraOptions::system().with_fonts(fonts));
}
```

如果你只依赖 focused components crate，也可以使用对应初始化入口：

```rust
use gpui::App;
use liora_components::{ThemeMode, init_liora, init_liora_with_mode};

fn init_components_only(cx: &mut App) {
    init_liora(cx);
    init_liora_with_mode(cx, ThemeMode::System);
}
```

注意区分：

```rust
// 高层应用初始化：core theme + portals + MessageManager + component key bindings。
liora::init_liora(cx);
liora_components::init_liora(cx);

// 低层 core-only 初始化：适合自定义组件 crate 或自己替换服务时使用。
liora_core::init_liora_with_mode(cx, liora_core::ThemeMode::System);
```

## 窗口启动、系统主题与图标

正式应用建议先用隐藏窗口创建，创建 root view 前附加系统主题观察器，`open_window` 返回后再激活窗口。这个模式可以避免首帧主题闪烁，也与 Gallery/Docs 的原生应用写法一致。

Windows release 构建建议使用与 Zed 相同的 subsystem 设置，这样双击 GUI `.exe` 时不会额外弹出空白命令行窗口：

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
```

`WindowFrameMode::Custom` 遵循 Zed 的 GPUI 兼容模型：Windows/macOS 在创建窗口时通过 `TitlebarOptions::appears_transparent` 隐藏系统标题栏；Linux/FreeBSD 通过 `WindowDecorations::Client` 使用客户端装饰。因此 Windows/macOS 的 frame mode 变更需要重开窗口生效，而 Linux 可以通过 `request_window_frame_mode` 实时请求 decorations 切换。

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

Linux/Wayland 下，任务栏图标由 desktop identity 解析，而不是由应用直接设置 window icon。应用需要让 `WindowOptions.app_id`、`.desktop` 文件和 icon theme 名称保持一致：

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

## liora 下各个模块怎么用

### `liora`

推荐应用依赖。常用导出：

```rust
use liora::{init_liora, init_liora_with_mode, init_liora_with_options};
use liora::{FontConfig, LioraOptions, ThemeMode};
use liora::{components, core, icons, icons_lucide, theme, tray};
```

### `liora-core`

核心运行时、主题配置、窗口 helper、Linux desktop identity、popper/portal 状态、唯一 ID 和主题切换：

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

语义 token 和共享组件枚举：

```rust
use liora::theme::{ButtonSize, ButtonVariant, Theme};

let light = Theme::light();
let dark = Theme::dark();
let primary_variant = ButtonVariant::Primary;
let large = ButtonSize::Large;
let surface = light.neutral.card;
```

### `liora-components`

可复用原生控件。大多数无状态组件可以 inline 构造：

```rust
use liora::components::{Button, Progress, Space, Tag, Text, Title};

let header = Space::new()
    .vertical()
    .child(Title::new("Deployments").h3())
    .child(Text::new("Production rollout status"))
    .child(Progress::new(72.0).primary().show_text(true))
    .child(Tag::new("Healthy").success());
```

有内部状态的控件应放在 `gpui::Entity<T>` 字段里，这样焦点、选区、弹窗状态、文本值才能跨 render 保持稳定：

```rust
use gpui::{AppContext, Context, Entity, Render, Window};
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

### `liora-icons` 与 `liora-icons-lucide`

图标 primitive 与内置 Lucide icon 名称：

```rust
use liora::core::Config;
use liora::icons::Icon;
use liora::icons_lucide::IconName;
use liora::components::Button;

let save = Button::new("Save").primary().icon_prefix(IconName::Save);
let icon = Icon::new(IconName::Settings).size(18.0);
```

如果应用使用 `gpui_platform::application()` 并渲染内置 Lucide SVG payload，建议安装 Liora icon asset source：

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

系统托盘 facade：

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

    // 在应用事件循环中用 tray.command_for_event(&event) 映射平台菜单事件。
    // Linux/FreeBSD 下需要定期调用 liora::tray::pump_platform_events()。

    drop(tray);
    Ok(())
}
```

### `liora-updater`

适合你自己应用的 GitHub Release 更新流程：

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
        // 只有用户明确点击安装时才执行安装动作。
    }

    Ok(())
}
```

### `liora-packager`

可复用的打包元数据与校验 helper。大多数应用可以复制本仓库 `xtask` 模式，也可以直接使用已发布的库：

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

## 组件代码示例

### 布局与卡片

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

### Button、Tag、Progress 与 toast

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

### 表单与状态控件

```rust
use gpui::{AppContext, Context, Entity, Render, Window};
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

`Mention` 也是输入类有状态组件，应作为 `Entity<Mention>` 保存在父 View 中。候选项的 `value` 是真正写回输入框的机器可读值；用户点击候选或按 `Enter` 时，Liora 会把当前触发符查询替换为 `trigger + item.value + 空格`，然后触发 `on_select`。

```rust
use gpui::{Context, Entity, Render, Window};
use liora::components::{Card, Mention, MentionItem, Space, Text, toast_success};

struct AssigneeField {
    people: Entity<Mention>,
    issue: Entity<Mention>,
}

impl AssigneeField {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            people: cx.new(|cx| {
                Mention::new(
                    vec![
                        MentionItem::new("alice", "Alice Chen").description("Design systems"),
                        MentionItem::new("bob", "Bob Smith").description("Release engineering"),
                    ],
                    cx,
                )
                .placeholder("Type @ to mention a teammate")
                .on_select(|item, _window, cx| {
                    toast_success(format!("Selected @{}", item.value), cx);
                })
            }),
            issue: cx.new(|cx| {
                Mention::new(vec![MentionItem::new("128", "#128 Improve chart hover")], cx)
                    .trigger('#')
                    .placeholder("Type # to reference an issue")
            }),
        }
    }
}

impl Render for AssigneeField {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        Space::new()
            .vertical()
            .child(Text::new("Mention selection writes back to the input value."))
            .child(Card::new(self.people.clone()))
            .child(Card::new(self.issue.clone()))
    }
}
```

例如输入 `hello @al` 后选择 `value = "alice"` 的候选项，会得到 `hello @alice `；设置 `trigger('#')` 后，输入 `fix #1` 并选择 `value = "128"` 会得到 `fix #128 `。

### 导航菜单

```rust
use liora::components::NavigationMenu;
use liora::icons_lucide::IconName;

let menu = NavigationMenu::new()
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

### `Shell`、`TitleBar` 和 `Sidebar` 应用框架

大多数应用窗口优先使用 `Shell`。它是 Liora 高层应用框架控件，统一管理可选自定义 `TitleBar`、header、左侧 sidebar、右侧 sidebar / inspector、可滚动 main、footer 和 overlay 等区域。只有在需要更底层组合时，才直接使用 `TitleBar`、`Sidebar`、`Container` 和 `AppWindowFrame`。

`NavigationMenu`、`Input` 等有状态组件仍应保存在父 View 的 `Entity<T>` 字段中。下面示例的布局全部由 Liora SDK 组件完成；应用入口仍可以使用 `Context`、`Entity`、`Render`、`Window` 等 GPUI 运行时类型。

```rust
use gpui::{AppContext, Context, Entity, Render, Window};
use liora::components::{
    Button, Card, NavigationMenu, NavigationMenuMode, Shell, ShellOverlayPosition, Sidebar, Space, Text, Title, TitleBar, WindowFrameMode,
};
use liora::core::Config;
use liora::icons::Icon;
use liora::icons_lucide::IconName;

struct AppShell {
    menu: Entity<NavigationMenu>,
}

impl AppShell {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            menu: cx.new(|_| {
                NavigationMenu::new()
                    .id("main-nav")
                    .mode(NavigationMenuMode::Vertical)
                    .default_active("dashboard")
                    .item("dashboard", "Dashboard", Some(IconName::LayoutDashboard))
                    .item("settings", "Settings", Some(IconName::Settings))
            }),
        }
    }
}

impl Render for AppShell {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        Shell::new(
            Card::new(
                Space::new()
                    .vertical()
                    .gap_sm()
                    .child(Title::new("Dashboard").h3())
                    .child(Text::new("Main content goes here.")),
            )
            .no_shadow(),
        )
        .id("acme-shell")
        .mode(WindowFrameMode::Custom)
        .titlebar(
            TitleBar::new()
                .title("Acme Notes")
                .subtitle("Native GPUI app")
                .height_units(52.0)
                .padding_x_units(18.0)
                .gap_units(10.0)
                .actions_gap_units(6.0)
                .window_controls(true)
                .action(Button::new("New").small()),
        )
        .sidebar(
            Sidebar::new()
                .id("app-sidebar")
                .brand("Acme Workspace")
                .brand_subtitle("Native GPUI")
                .logo(Icon::new(IconName::Sparkles).size_units(20.0))
                .expanded_width_units(280.0)
                .header_padding_units(14.0)
                .content_padding_units(8.0)
                .footer_padding_units(12.0)
                .gap_units(8.0)
                .rounded_units(16.0)
                .scrollable()
                .child(self.menu.clone())
                .footer(Text::new("v1.0").sm()),
        )
        .footer(Text::new("Ready").xs())
        .footer_height_units(40.0)
        .header_background(theme.neutral.card)
        .footer_background(theme.neutral.card)
        .body_background(theme.neutral.body)
        .main_background(theme.neutral.card)
        .main_rounded_units(18.0)
        .overlay(Text::new("Saved").xs())
        .overlay_position(ShellOverlayPosition::TopRight)
        .overlay_inset_units(16.0)
        .main_scroll()
        .main_padding_units(24.0)
    }
}
```

### 图表与指标

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

### 代码展示与编辑器

```rust
use gpui::{AppContext, Context, Entity, Render, Window};
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

### 二维码、上传、图片与预览

```rust
use liora::components::{Button, Image, Preview, QrCode, Space, Upload};

let utilities = Space::new()
    .vertical()
    .child(QrCode::new("https://github.com/yhyzgn/liora").show_text(true))
    .child(Image::new("file:///tmp/screenshot.png").width(gpui::px(240.0)))
    .child(Preview::new("file:///tmp/screenshot.png").child(Button::new("Preview image")))
    .child(Upload::new().width_lg());
```

### 虚拟化数据

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

## 高级用法

### 运行时主题切换

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

### 自定义字体且保留系统默认策略

Liora 把 **字体资源加载** 和 **字体族选择** 拆成两步：

1. 如果字体已经安装在用户系统里，不需要加载文件，直接用 `FontConfig` 指定有序 family 兜底列表。
2. 如果应用自带私有字体，先用 `load_app_fonts`、`load_fonts_from_dir`、`load_font_assets`、`load_embedded_fonts`，或底层兼容函数 `load_custom_fonts` 注册字体字节。
3. 然后在启动时通过 `LioraOptions::with_fonts(...)`，或运行时通过 `set_font_config(...)` 选择 UI/code 字体族。

支持的文件扩展名包括 `ttf`、`otf`、`ttc`、`otc`、`woff`、`woff2`，但真正解析能力由各平台官方 GPUI 后端决定。原生桌面应用优先使用 `ttf` / `otf` / `ttc` / `otc`。在 Linux/WGPU 上，当前 GPUI 的 `fontdb` 路径可能会忽略 WOFF/WOFF2 字节且不返回错误，所以只要某个 family 必须生效，就应该使用 `FontLoadOptions::require_family(...)` 并检查 `FontLoadReport::missing_required_families`。

#### 只使用系统已安装字体

```rust
use liora::{FontConfig, LioraOptions, init_liora_with_options, set_font_config};

fn init_with_system_fonts(cx: &mut gpui::App) {
    init_liora_with_options(
        cx,
        LioraOptions::system().with_fonts(
            FontConfig::system()
                .with_ui_families(["Segoe UI", "PingFang SC", "Arial"]) // 按顺序自动降级。
                .with_code_families(["JetBrains Mono", "SF Mono", "Monospace"]),
        ),
    );
}

fn switch_to_system_ui_and_monospace_code(cx: &mut gpui::App) {
    set_font_config(
        cx,
        FontConfig::system()
            .with_ui_families(["PingFang SC", "Segoe UI", "Arial"])
            .with_code_families(["JetBrains Mono", "SF Mono", "Monospace"]),
    );
}
```

#### 为裸可执行程序内嵌一个小 fallback 字体

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
        LioraOptions::system().with_fonts(
            FontConfig::system().with_ui_families(["Inter", "Segoe UI", "Arial"]),
        ),
    );
}
```

#### 安装包优先外部挂载，裸可执行回退内嵌字体

完整字体族通常很大。推荐把一个 Regular 字重内嵌进裸可执行程序，把完整字体族放到安装包或 portable archive 的 `assets/fonts` 目录中。

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
            dirs.push(exe_dir.join("assets/fonts"));                       // Windows 安装根目录或 portable 根目录。
            dirs.push(exe_dir.join("..").join("Resources").join("assets/fonts")); // macOS .app。
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

    // 混合来源示例：UI 使用随应用分发的 PingFang，代码字体使用系统族名。
    init_liora_with_options(
        cx,
        LioraOptions::system().with_fonts(
            FontConfig::system()
                .with_ui_families(["PingFang SC", "Segoe UI", "Arial"])
                .with_code_families(["JetBrains Mono", "SF Mono", "Monospace"]),
        ),
    );
}
```

#### 从 GPUI assets 或显式文件加载

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

Liora 自带的 Gallery 和 Docs 把完整 PingFangSC TTF 字体族分别放在各自应用的 `assets/fonts/PingFangSC/` 下。应用二进制只内嵌 `PingFangSC-Regular.ttf` 作为裸可执行 fallback；打包流水线会把完整 `assets/fonts` 目录作为外部资源挂载到安装包和 portable archive 中。

### 平台菜单与窗口内可见菜单栏

`Menu` 是共享的命令 descriptor。它既可以注册到 GPUI 官方平台菜单，也可以渲染成窗口内 fallback 菜单栏，还可以复用到命令面板。两者不是同一层：

| 目标 / 环境 | 使用方式 | 说明 |
|---|---|---|
| 接入 OS / 平台菜单语义 | `Menu::register(cx, menus)` | 委托给 GPUI `App::set_menus`。macOS 通常显示在屏幕顶部全局菜单栏；Linux/Wayland/KDE/GNOME 和 Windows 是否可见取决于平台后端/桌面环境，不会自动插入 GPUI 元素树。 |
| 必须在应用窗口内稳定可见 | `MenuBar::new(menus)` | `MenuBar` 是 Liora 可视组件，可放在 `Container` header、`Shell` 区域或自定义 `TitleBar`。 |
| System frame 且接受平台原生行为 | 只调用 `Menu::register(...)` | 适合 macOS 原生体验；部分 Linux/Windows 环境可能看不到窗口菜单。 |
| System frame 但窗口内必须显示菜单 | `Menu::register(...)` + header 中放 `MenuBar` | Gallery 就是这种模式：保留平台菜单注册，同时 header fallback 跨环境稳定可见。 |
| Custom frame / client-side decorations | `Menu::register(...)` + chrome/header 中放可见 `MenuBar` | 自定义标题栏不会让 GPUI 自动把平台菜单注入元素树。 |
| 文档、设置页、预览用例 | `MenuBar` 或单个 `Menu`，并设置 `.perform_builtin_actions(false)` | 避免示例点击时真的退出应用、打开 URL 或写剪贴板。 |

```rust
use gpui::App;
use liora::components::{Menu, MenuBar, MenuItem};

fn app_menus() -> [Menu; 2] {
    [
        Menu::new("File")
            .item(MenuItem::open_file())
            .item(MenuItem::open_folder())
            .item(MenuItem::separator())
            .item(MenuItem::quit()),
        Menu::new("Edit")
            .item(MenuItem::undo())
            .item(MenuItem::redo())
            .item(MenuItem::separator())
            .item(MenuItem::copy())
            .item(MenuItem::paste()),
    ]
}

fn register_platform_menu(cx: &mut App) {
    Menu::register(cx, app_menus());
}

fn in_window_menu_bar() -> MenuBar {
    MenuBar::new(app_menus()).perform_builtin_actions(false)
}
```

窗口内菜单栏直接用代码放进根布局，并渲染它的下拉气泡所需的 popover portal：

```rust
use gpui::{App, IntoElement, ParentElement, Styled, Window, div, px};
use liora::components::{AppWindowFrame, Container, MenuBar};

fn render_root(window: &mut Window, cx: &mut App) -> impl IntoElement {
    let menu_bar: MenuBar = in_window_menu_bar();

    // MenuBar 下拉气泡，以及所有基于 popover 的 Liora 组件都需要这一层。
    liora::core::render_active_popover_in_window(window, cx);

    AppWindowFrame::new(
        "My App",
        Container::new()
            .header(div().w_full().child(menu_bar))
            .header_height(px(40.0))
            .child("Window body"),
    )
}
```

`Menu::register(...)` 只调用 GPUI 官方 `App::set_menus`；窗口里可见的菜单行来自 `MenuBar::new(...)`。

### 浮层与 portal 渲染

大多数应用只需要 `liora::init_liora(cx)`。如果你自己实现根 shell 并手动管理 overlay layer，应在窗口根部渲染 portal：

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

普通 `Popover` 内容默认带 16 px 内边距，适合简单文本或卡片气泡，避免内容贴边。如果你的弹层主体本身已经是完整 surface（例如菜单、命令面板、自定义确认框），应移除共享 padding，让主体自己控制宽度、留白、滚动和条目间距：

```rust
use liora::components::{Button, Popover};

let popup = Popover::new(Button::new("Actions"))
    .flush_content()
    .content(|_window, _cx| {
        // 菜单/面板根节点自己控制 min width、padding、scrolling 和 item spacing。
        liora::components::Space::new()
            .padding_md()
            .child(Button::new("Archive"))
            .child(Button::new("Delete").danger())
    });

let roomy_popup = Popover::new(Button::new("Details"))
    .content_padding(gpui::px(20.0))
    .content(|_window, _cx| "Padded plain content");
```

`Dropdown`、`DropdownButton`、`Menu` 子菜单和 `Popconfirm` 已经在内部使用 flush 模式，因此菜单和确认面板在不同 placement 下会保持一致的尺寸和布局节奏。

### 应用状态留在应用层

不要把产品数据模型放进 `liora-components`。应用状态应存在自己的 GPUI view/entity 中，只把展示值和回调传给组件：

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

## 组件清单

| 分类 | 组件 |
|---|---|
| Basic / Layout 基础布局 | `Button`, `ButtonGroup`, `Icon`, `Link`, `Text`, `Title`, `Paragraph`, `SelectableTextGroup`, `Space`, `Divider`, `Row`, `Col`, `Container`, `Shell`, `Sidebar`, `TitleBar`, `Flex`, `Scrollbar`, `ScrollableMask`, `Splitter`, `DockLayout`, `Affix`, `Backtop` |
| Form 表单 | `Input`, `InputNumber`, `Textarea`, `Checkbox`, `CheckboxGroup`, `Radio`, `RadioGroup`, `Switch`, `Select`, `Slider`, `Form`, `FormItem`, `Rate`, `DatePicker`, `TimePicker`, `DateTimePicker`, `Upload`, `Cascader`, `Transfer`, `ColorPicker`, `Autocomplete`, `InputTag`, `Mention`, `TreeSelect`, `SearchableList`, `OtpInput`, `Toggle`, `ToggleGroup` |
| Feedback / Overlay 反馈浮层 | `Alert`, `Tooltip`, `Popover`, `Popconfirm`, `Dialog`, `Drawer`, `Message`, `Notification`, `MessageBox`, `Loading`, `Dropdown`, `DropdownButton`, `Preview`, `Tour`, `HoverCard`, `FocusTrap` |
| Navigation 导航 | `NavigationMenu`, `Tabs`, `Breadcrumb`, `Steps`, `PageHeader`, `Anchor`, `Accordion` |
| Data 数据展示 | `Table`, `List`, `VirtualizedTable`, `VirtualizedTree`, `VirtualizedList`, `Progress`, `Skeleton`, `Empty`, `Result`, `Descriptions`, `Timeline`, `Tree`, `Pagination`, `Statistic`, `Segmented`, `Tag`, `Avatar`, `Badge`, `Calendar`, `Carousel`, `Image`, `Watermark`, `Kbd`, `GroupBox`, `StatusBar`, `SettingsPage`, `SettingsGroup`, `SettingsItem` |
| Charts / Metrics 图表指标 | `LineChart`, `AreaChart`, `BarChart`, `PieChart`, `RingChart`, `Sparkline`, `SignalMeter`, `HeatBar`, `SegmentRatioBar`, `CandlestickChart` |
| Editing / Utility 编辑工具 | `CodeBlock`, `CodeEditor`, `QrCode`, `Timer`, `Label`, `Operation`, `Clipboard`, draggable list helpers |
| App shell / Platform 平台 | `Shell`, `AppWindowFrame`, `TitleBar`, `Sidebar`, `WindowFrameMode`, `StatusBar`, `DockLayout`, `Menu` / `MenuBar`, `liora-tray`, Linux desktop identity helpers, package metadata helpers, updater helpers |

### 控件收敛说明

Liora 避免为同一类功能提供重复控件：

- `Drawer` 同时覆盖完整抽屉和轻量面板场景。`Drawer::sheet()` 提供轻量默认尺寸，不再单独暴露 Sheet 控件。
- `Select` 同时覆盖固定选项和可搜索选择。`Select::searchable(...)`、`.multiple()`、分组、禁用项和 footer slot 覆盖原 Combobox 类工作流。
- `Text` 同时覆盖内联文本和轻量应用文档。`Text::document(...)`、`TextBlock`、`Text::markdown(...)` 覆盖 About/Help/Release notes 等原 TextView 类内容。`Text`、`Title`、`Paragraph` 默认可用鼠标自由选中文字；只有装饰性标签或不希望复制的界面文案才调用 `.selectable(false)` 关闭。如果需要跨多个 `Text` 与 `Paragraph` 块连续选择，例如 Release notes、帮助页和文档正文，请使用 `SelectableTextGroup`。

## 原生打包

仓库内打包能力由已发布的 `liora-packager` 库和本地 `xtask` 命令包装提供：

```bash
cargo run -p xtask -- package validate
cargo run -p xtask -- package release-readiness
cargo run -p xtask -- package build --all-apps
cargo run -p xtask -- package ci --app gallery --format platform-defaults --skip-build
cargo run -p xtask -- package smoke --app gallery --format platform-defaults
cargo run -p xtask -- package install-smoke --app gallery --format platform-defaults --dry-run
```

当前发布产物覆盖：

| 平台 | 原始可执行程序 | Gallery 安装器/包 |
|---|---|---|
| Linux x64 | `liora-docs`, `liora-gallery` | AppImage、`.deb`、`.rpm`、portable `.tar.gz` |
| macOS arm64 | `liora-docs`, `liora-gallery` | `.dmg` |
| Windows x64 | `liora-docs.exe`, `liora-gallery.exe` | NSIS setup `.exe`、MSI |

打包规则：

- 应用保持纯 Rust + GPUI 原生；
- app 图标、托盘图标、状态图标放在各自 app-owned asset 目录中；
- 使用 `liora-packager`/`xtask` 管理 package metadata，不引入 Web runtime；
- Windows app build script 只嵌入 icon/file metadata；GPUI Windows backend 已经提供 application manifest。

## 常见问题与排查

### Cargo 选中了 `gpui 0.2.2`，Liora 编译失败

你的应用缺少根级 patch：

```toml
[patch.crates-io]
gpui = { git = "https://github.com/zed-industries/zed", rev = "2c346f60a76fe3f0367ef924927f50a6efdf5718" }
```

用以下命令确认：

```bash
cargo tree -i gpui
cargo tree -p gpui
```

### `use of unresolved crate gpui` 或 `gpui_platform`

当你的 binary crate 命名 GPUI 类型或启动 app runtime 时，需要直接加依赖：

```toml
[dependencies]
gpui = { version = "0.2.2", default-features = false }
gpui_platform = { git = "https://github.com/zed-industries/zed", rev = "2c346f60a76fe3f0367ef924927f50a6efdf5718", default-features = false }
```

### Linux 缺 GTK、Wayland、X11、Vulkan、ALSA 或字体包导致构建失败

安装平台原生依赖。Fedora-like 系统可参考并调整：

```bash
scripts/install-fedora-deps.sh
```

Debian/Ubuntu-like 系统需要安装等价的 `libgtk-3-dev`、Wayland/X11/xkbcommon、fontconfig/freetype、Vulkan、ALSA 和 `pkg-config` 包。

### 标题栏图标正常，但 Linux 任务栏图标空白

Wayland 下需要注册 desktop identity，并让 `WindowOptions.app_id` 与 desktop entry/icon name 一致。Compositor 会通过 `.desktop` 和 icon theme 解析任务栏图标。

```rust
WindowOptions {
    app_id: Some("acme-notes".into()),
    ..Default::default()
}
```

### Dark/System 主题启动时闪一下

窗口使用 `show: false` 创建，在 `open_window` 回调一开始调用 `attach_system_theme_observer(window, cx)`，并在 `open_window` 返回 handle 后调用 `window.activate_window()`。

### Input 文本或焦点每次 render 后重置

把有状态控件放在 `gpui::Entity<T>` 字段中，不要在每次 render 中重新构造：

```rust
struct ViewState {
    input: gpui::Entity<liora::components::Input>,
}
```

### toast macro panic，提示缺少 `MessageManager`

使用 `liora::init_liora(cx)` 或 `liora_components::init_liora(cx)`。如果你刻意只使用 `liora_core`，需要自己初始化组件服务后再调用 toast helpers。

### Windows 链接失败，提示重复 `MANIFEST` resource

使用 GPUI Windows backend 时，不要在 app `build.rs` 中再嵌入 Windows Common Controls manifest。只嵌入 icon 和 file metadata；Windows 下 `gpui_platform` 会启用 GPUI 的 `windows-manifest` feature。

### Release package 里文件太多

GitHub Release assets 只上传面向用户的 raw binaries、installers/packages 和 `SHA256SUMS.txt`。生成的 notes/config 文件应放在 release body 或 CI artifacts 中，不作为用户下载资产。

### 下一步应该读哪些文档？

- `apps/liora-docs/content/pages/quick_start.md`：接入说明。
- `apps/liora-docs/content/pages/theme_system.md`：启动主题和窗口行为。
- `apps/liora-docs/content/pages/packaging_workflow.md`：发布打包流程。
- `apps/liora-gallery/src/demos/`：逐组件使用示例。

## 质量门禁

发布或提交变更前运行：

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

Release 构建还应运行：

```bash
cargo build --workspace --release
cargo run --release -p xtask -- package validate
cargo run --release -p xtask -- package release-readiness
```

## 设计原则

- **Native first**：所有组件走 GPUI 元素树、原生文本、原生输入和原生绘制路径。
- **应用级默认值**：主题、浮层、消息、键盘和选择行为通过一次初始化完成。
- **组合优先**：组件暴露 builder 风格 API；产品数据和页面组合留在应用层。
- **Token 驱动视觉**：Light/Dark/System 主题使用语义 token 管理表面、文本、边框、遮罩和交互状态。
- **性能感知数据 UI**：图表和虚拟化视图包含降采样、hit testing、缓存上限和可视区域渲染模式。

## 运行时模型

`liora::init_liora(cx)` 是使用 facade crate 时推荐的应用入口。它会初始化 Liora core/theme 状态、全局组件服务，以及交互控件 key bindings。

当产品需要显式选择启动主题时，使用 `liora::init_liora_with_mode(cx, ThemeMode::Light | ThemeMode::Dark | ThemeMode::System)`。运行时主题切换使用 `liora_core` 或 facade `core` 模块中的 `apply_theme_mode(window, cx, mode)`。

字体默认保持系统原生：Liora 不默认加载品牌字体，也不会把整个 UI 映射到 Zed 专用字体别名。自定义字体通过有序 `FontConfig` 兜底列表、`LioraOptions`、`load_app_fonts`、`load_fonts_from_dir`、`load_font_assets`、`load_embedded_fonts`、底层 `load_custom_fonts` 和 `set_font_config` 显式启用。

`Input`、`Switch`、`Select`、`TreeSelect`、`CodeEditor` 和虚拟化视图等有状态控件应放在 `gpui::Entity<T>` 字段中，以便焦点、展开状态、选区、滚动状态和文本值在重渲染后保持稳定。

## 技术创新点

- **一条依赖完成接入**：`liora` facade 重新导出常用 SDK 模块，应用 manifest 不需要零散添加多个 Liora crate。
- **一次性应用初始化**：`init_liora(cx)` 集中处理 core 配置、组件服务和键盘绑定。
- **原生 Markdown 文档**：Docs app 把 Markdown 内容渲染为 Liora/GPUI 节点，并检查外部 Rust 片段。
- **无浏览器层的原生图表**：图表使用 Rust 数据结构、GPUI paint path、hit testing 和降采样。
- **应用壳能力覆盖**：托盘常驻、toast、主题切换、可搜索组件导航和真实布局都在原生应用中验证。
- **打包意识内建**：安装器信息、manifest、checksum、backend config 和 dry-run 安装计划与代码一起验证。

## README 同步规范

以后任何代码改动都必须先问：**README 是否需要同步修改？**

当你修改以下内容时，必须在同一个变更中更新 `README.md` 和 `README.zh-CN.md`：

- public crate 名称、feature 或依赖说明；
- GPUI revision、patch 策略或平台 feature flags；
- 初始化 API、主题行为、字体、图标、窗口启动或托盘行为；
- 组件名称、主要组件 API、示例或应用壳模式；
- 打包、updater、release assets、CI 命令、MSRV 或排错说明。

如果 README 不需要变化，也要在最终变更说明中明确写出“不需要同步 README”。

## 贡献

提交 PR 前请阅读 `CONTRIBUTING.md`。关键边界：

- 保持 Liora 为纯 Rust + GPUI 原生；
- 不引入 Tauri、WebView、HTML/CSS/DOM、browser runtime 或 web chart shell；
- 不把产品数据模型或页面级 helper 放进 `liora-components`；
- 保持 Gallery、Docs、snippets、tests 和中英文 README 与公开行为同步。

## License

Liora 当前使用 `LicenseRef-Liora`；见 `LICENSE.md`。在项目维护者明确替换为 OSS 或商业 license 条款前，不要假设本项目为开源 license。
