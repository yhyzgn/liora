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

Liora 当前使用 workspace 里的 GPUI git 依赖。外部项目可以先按下面方式配置：

```toml src="quick_start/app_cargo.toml"
```

几个关键点：

- `gpui` / `gpui_platform` 来自 Zed 仓库，Liora 当前也是这样接入。
- Linux / FreeBSD app 需要在 target-specific dependencies 中显式启用 `wayland`、`x11`、`font-kit`。
- macOS app 需要启用 `font-kit`。
- Windows app 保持默认 GPUI/GPUI Platform 后端即可。
- 组件库 crate 不应该开启平台 feature；平台 feature 应放在最终 app crate 的 `cfg(...)` target dependency 中。
- 如果你在 Liora 仓库内开发，优先使用 workspace 中已有的 path dependency。

## 5. 初始化 Liora 并打开窗口

最小可运行窗口包括五步：

1. 启动 `gpui_platform::application()`。
2. 调用 `init_liora(cx, Theme::light())` 注册主题。
3. 初始化全局层，例如 `MessageManager::init(cx)`。
4. 注册需要键盘交互的组件 key bindings。
5. 通过 `cx.open_window(...)` 创建窗口，并返回一个 `cx.new(...)` 的根 View。

```rust src="quick_start/main_window.rs"
```

## 6. 在 View 中组合 Liora 组件

GPUI 的推荐写法是：状态放在 View 字段里，渲染时把这些字段转换为元素。下面示例演示了按钮、标题、文本、输入框和开关：

```rust src="quick_start/component_view.rs"
```

注意事项：

- `Input`、`Switch`、`Select` 等有状态组件要作为 `Entity<T>` 字段保存，不要在每次 `render` 中临时创建后丢弃。
- 事件回调用 `cx.listener(...)` 或组件自己的 `on_click` / `on_change` API。
- 修改 View 自身状态后调用 `cx.notify()`，让 GPUI 安排下一帧重绘。
- 需要弹层或全局提示的应用，要在根布局末尾渲染对应 Portal / Message 层。

## 7. 运行和验证

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
