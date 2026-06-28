# Quick Start

本页从一个空目录开始，演示如何创建 Rust 项目、接入 GPUI、接入 Liora，并打开一个包含 Liora 组件的原生窗口。示例以 Linux/Fedora 为主，同时列出 macOS、Ubuntu/Debian 和 Windows 的准备事项。

## 0. 先理解 GPUI 的核心模型

在写代码前先记住几个 GPUI 概念：

- **Application**：`gpui_platform::application().run(...)` 启动原生事件循环。所有窗口和全局状态都在这个循环里创建。
- **App**：应用级上下文，常用于注册全局状态、初始化主题、打开窗口、创建顶层 `Entity`。
- **Window**：单个原生窗口上下文，负责焦点、输入事件、布局、绘制和窗口级 overlay。
- **Entity<View>**：持久化的、有状态 View。输入框、开关、选择器这类组件必须放在 Entity 中，才能保留焦点、输入值、选中状态和动画状态。
- **Render / RenderOnce**：`Render` 用于有状态 View，`RenderOnce` 更适合一次性元素。交互组件通常实现 `Render`。
- **IntoElement / AnyElement**：把组件组合成 GPUI 可布局、可绘制的元素树。
- **Context<Self>**：View 内部上下文，用来 `cx.new(...)` 创建子 Entity、`cx.notify()` 触发重绘、注册 listener。

Liora 在这些概念之上提供组件、主题、Portal、全局消息层和常用布局组件。

## 1. 安装 Rust

建议使用 rustup：

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
rustup component add rustfmt clippy
```

Windows 请在 PowerShell 中使用 rustup 官网给出的安装器，并确保 Git 与 C++ 构建工具可用。

## 2. 安装系统依赖

GPUI 是原生 GPU UI 框架，Linux 下需要字体、窗口系统、XKB/X11/Wayland、Vulkan 和 C/C++ 构建工具。Liora 仓库已提供 Fedora 脚本：

```shell src="quick_start/deps_fedora.sh"
```

Ubuntu / Debian 可按下面安装常见依赖：

```shell src="quick_start/deps_ubuntu.sh"
```

macOS 需要 Xcode、命令行工具和 CMake：

```shell src="quick_start/deps_macos.sh"
```

Windows 需要 Rust MSVC 工具链、Visual Studio Build Tools、Git 长路径和可用 GPU 驱动：

```powershell src="quick_start/deps_windows.ps1"
```

注意：Linux 运行窗口时还需要可用的 Vulkan 驱动。如果窗口无法打开或报 `NoSupportedDeviceFound`，先用 `vulkaninfo` 或 `vkcube` 检查显卡驱动。

## 3. 创建一个新的 Rust 项目

```shell src="quick_start/create_project.sh"
```

这里使用 binary crate，因为我们马上要打开一个原生窗口。对于真实产品，推荐使用 workspace：把应用放在 `apps/your-app`，共享组件放在 `crates/*`。

## 4. 配置 Cargo 依赖

Liora SDK 从 crates.io 安装；GPUI 通过 Cargo `[patch.crates-io]` 解析到 Liora 验证过的 Zed 官方 git revision，并通过 `gpui_platform::application()` 启动原生事件循环。外部项目可以按下面方式配置：

```toml src="quick_start/app_cargo.toml"
```

几个关键点：

- `gpui` 在 `[dependencies]` 中保留 `version = "0.2.2"` 作为 crates.io 发布 fallback，然后通过 `[patch.crates-io]` 覆盖到 Zed 官方 `https://github.com/zed-industries/zed` 指定 `rev`。不要使用非官方 fork。
- Linux / FreeBSD app 需要在 target-specific dependencies 中同时为 `gpui` 与 `gpui_platform` 显式启用 `wayland`、`x11`、`font-kit`。
- macOS app 需要同时为 `gpui` 与 `gpui_platform` 启用 `font-kit`。
- Windows app 使用官方 GPUI Windows 后端即可。
- 组件库 crate 不应该开启平台 feature；平台 feature 应放在最终 app crate 的 `cfg(...)` target dependency 中。
- 如果你在 Liora 仓库内开发，优先使用 workspace 中已有的 path dependency；外部应用优先使用 crates.io `liora = "0.1"`。

## 5. 初始化 Liora 并打开窗口

最小可运行窗口包括三步：

1. 启动 `gpui_platform::application()`。
2. 调用 `liora::init_liora(cx)`，一次性注册 Liora core/theme 状态、组件全局服务和 key bindings；它默认使用 `ThemeMode::System` 跟随系统主题。
3. 通过 `cx.open_window(...)` 创建窗口，并返回一个 `cx.new(...)` 的根 View。

```rust src="quick_start/main_window.rs"
```

## 6. 注册系统平台菜单

平台菜单和窗口内菜单栏是两个层级：`Menu::register(cx, menus)` 注册 GPUI 官方平台菜单；如果你还希望在窗口内容里稳定显示 `File / Edit` 这一行，就把同一份 descriptor 渲染成 `MenuBar` 放进 header / Shell / TitleBar。

```rust src="quick_start/platform_menu.rs"
```

## 7. 在 View 中组合 Liora 组件

GPUI 的推荐写法是：状态放在 View 字段里，渲染时把这些字段转换为元素。下面示例演示了按钮、标题、文本、输入框和开关：

```rust src="quick_start/component_view.rs"
```

注意事项：

- `Input`、`Switch`、`Select` 等有状态组件要作为 `Entity<T>` 字段保存，不要在每次 `render` 中临时创建后丢弃。
- 事件回调用 `cx.listener(...)` 或组件自己的 `on_click` / `on_change` API。
- 修改 View 自身状态后调用 `cx.notify()`，让 GPUI 安排下一帧重绘。
- 需要弹层或全局提示的应用，要在根布局末尾渲染对应 Portal / Message 层。
- 字体默认走系统；如果要像 Gallery/Docs 一样指定应用级字体，请先注册字体资源，再在 `LioraOptions::system().with_fonts(...)` 中指定有序 fallback family。详见下一节“应用级字体自定义”。

## 8. 应用级字体自定义

Liora 把字体分成两个步骤：**加载资源** 和 **选择 family**。系统已安装字体不需要加载文件，直接用 `FontConfig::system().with_ui_families([...]).with_code_families([...])` 指定有序兜底列表即可；随应用分发的私有字体要先通过 `load_app_fonts` / `load_fonts_from_dir` / `load_font_assets` / `load_embedded_fonts` 注册，再用 `init_liora_with_options(...)` 或运行时 `set_font_config(...)` 生效。

Gallery 和 Docs 当前采用同一策略：源码运行时扫描 `apps/<app>/assets/fonts`；安装包或 portable archive 优先使用可执行程序旁边的外部 `assets/fonts`；裸可执行程序则内嵌一个较小的 `PingFangSC-Regular.ttf` 作为 fallback。这样既能保证 `cargo run -p liora-gallery` / `cargo run -p liora-docs` 可直接看到 PingFang SC，也避免把完整字体族全部塞进二进制。

```rust src="quick_start/fonts.rs"
```

注意事项：

- 支持的输入扩展名包括 `ttf`、`otf`、`ttc`、`otc`、`woff`、`woff2`，但实际解析能力取决于官方 GPUI 在当前平台的字体后端。
- 原生桌面发布优先使用 `ttf` / `otf` / `ttc` / `otc`；如果使用 `woff` / `woff2`，务必配合 `FontLoadOptions::require_family(...)` 检查目标 family 是否真的可见。
- `with_ui_families([...])` 和 `with_code_families([...])` 都是有序 fallback 列表，不是单个 family。建议把品牌字体放前面，把跨平台系统字体放后面。
- 字体资源属于应用级资产。SDK 只提供加载和配置 API，不会强制把某套字体耦合进所有下游应用。

## 9. 运行和验证

```shell src="quick_start/verify.sh"
```

在 Liora 仓库内还可以直接运行官方应用：

```shell src="quick_start/run.sh"
```

## 常见问题

- **Linux 编译找不到 `fontconfig.pc` 或 `freetype2.pc`**：安装 `fontconfig-devel` / `freetype-devel` 或 Ubuntu 对应的 `libfontconfig-dev` / `libfreetype-dev`。
- **找不到 `c++`**：安装 `gcc-c++`、`g++` 或完整 build essentials。
- **窗口打不开 / GPU 报错**：优先检查 Vulkan 驱动和桌面会话。Linux 需要 Vulkan 1.3 级别的驱动能力；Windows 需要最新显卡驱动。
- **组件状态每次刷新都丢失**：不要在 `render` 中临时 `cx.new`；把 Entity 存在 View struct 字段里。
- **弹层位置不对**：弹层类组件通常要捕获 trigger bounds，并通过 Liora Portal 在窗口 overlay 层渲染。
