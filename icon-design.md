# Liora UI - 图标系统架构与代码生成规范 (Icon System Specification)

> Lucide 图标库官网： https://lucide.dev/icons/
> Lucide 图标库仓库：https://github.com/lucide-icons/lucide.git

## 1. 架构背景与目标
**Liora UI** 是一套基于 Rust 和 GPUI 开发的原生高性能企业级 UI 组件库。为了兼顾开箱即用的开发体验与企业级真实业务的定制化需求，Liora 的图标系统采用 **“无边界基础容器 + 独立代码生成扩展包”** 的混合架构。

当前目标：
1. 实现一个纯粹的 SVG Path 渲染容器 `Icon`。
2. 开发一个基于 `build.rs` 的代码生成工作流，将 **Lucide** 图标库批量编译为 Rust 枚举，并封装为独立的 Crate。

## 2. 核心设计一：无边界图标容器 (Icon)
`Icon` 不应该与任何特定的图标库强绑定。它本质上是一个高度封装的 GPUI 视图容器，只负责处理以下职责：
* **尺寸 (Size):** 统一处理宽度和高度。
* **色彩 (Color):** 响应 Liora 全局主题，支持自定义颜色。
* **交互 (Interaction):** 处理 Hover、Active 等状态。
* **路径渲染 (Path Rendering):** 接收并渲染合法的 SVG `<path>` 字符串。

### 期望的 API 调用范式：
```rust
// 范式 1：直接传入从 Figma 或 Iconfont 导出的原生 SVG Path 字符串（满足定制业务需求）
Icon::new()
    .size(px(24.0))
    .color(theme.primary)
    .path("M12 2L2 22h20L12 2zm0 3.5l7.5 14.5h-15L12 5.5z")
```

## 3. 核心设计二：代码生成模式 (Codegen for Lucide)
为了提供丝滑的内置图标开发体验，我们需要将 **Lucide** 图标库剥离为一个独立的 Crate（例如命名为 `liora-icons-lucide`）。

该 Crate 的核心是通过 `build.rs` 在编译期自动完成以下工作：
1. **读取：** 遍历本地或作为 submodule 引入的 Lucide `.svg` 文件目录。
2. **解析：** 提取 SVG 文件中 `<path>` 标签的 `d` 属性（即路径字符串）。对于包含多个 `<path>` 的复杂图标，需进行字符串合并处理。
3. **生成：** 自动生成一个包含所有图标名称的巨大 Rust `enum`，并为其实现获取对应路径字符串的方法。

### 期望的 API 调用范式（结合 Codegen）：
```rust
// 范式 2：使用独立扩展包提供的强类型 Enum
use liora_icons_lucide::IconName;

// 推荐的丝滑调用方式
Icon::new(IconName::Home)
    .size(px(20.0))
    .color(theme.text_main)

// 或者采用 Builder 模式的变体
Icon::new().icon(IconName::ShoppingCart)
```

---

## 4. 具体的开发任务（Prompt for AI）

请基于以上架构规范，使用 Rust 为我实现以下代码：

### 任务一：实现 `build.rs` 解析与生成逻辑
请在 `liora-icons-lucide` Crate 中编写一个 `build.rs` 脚本。
* 脚本需要能够读取指定的 `assets/svgs` 目录。
* 使用正则表达式或轻量级 XML 解析库（如 `quick-xml` 或 `roxmltree`），提取每个 SVG 文件的 `<path d="...">` 内容。
* 生成一个 `generated.rs` 文件，内容包含一个 `pub enum IconName`，以及一个返回静态字符串的 `impl IconName { pub fn path(&self) -> &'static str }`。
* 请注意处理 Rust 标识符的命名规范（如将 `shopping-cart.svg` 转换为 `IconName::ShoppingCart`）。

### 任务二：实现 `Icon` 核心容器组件
请使用 GPUI 的 API 实现 `Icon` 组件。
* 它需要实现 `gpui::IntoElement` 或 `gpui::RenderOnce`。
* 内部使用 GPUI 的 `svg()` 原生组件来承载最终的路径。
* 请实现 Builder 模式的方法，如 `.size()`, `.color()`, `.path()` 等。

### 任务三：优化 API 的丝滑度（Trait 抽象）
为了让 `Icon::new()` 既能接受字符串也能接受 `IconName` 枚举，请巧妙利用 Rust 的 Traits（例如定义一个 `IntoIconPath` trait，让 `&str` 和 `IconName` 都实现它），从而达成最精简、最优雅的 API 调用形态。请给出完整的组件结构和 Trait 实现代码。
```