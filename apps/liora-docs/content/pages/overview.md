# Liora Docs

Liora Docs 是 Liora UI 的官方原生文档主程序。它不是网页文档站、不是 WebView，也不是把 Web 技术转译到桌面；它本身就是一个 GPUI 原生 Rust 应用，用真实窗口、真实元素树和真实组件来解释 Liora。

## Liora 是什么

Liora UI 是一套基于 GPUI 的原生企业级组件库。它把常见后台 / 工具类产品需要的按钮、输入、选择器、表格、弹层、反馈提示、代码块、图片预览等能力封装成可复用 Rust 组件。

这个仓库同时包含三类内容：

- `crates/liora-core`：主题注册、Portal、Tooltip、Popover、Modal、Drawer 等跨组件运行时基础设施。
- `crates/liora-theme`：颜色、字号、圆角、按钮类型等设计 token。
- `crates/liora-components`：所有可复用组件，例如 `Button`、`Input`、`Select`、`CodeBlock`、`Preview`。
- `apps/liora-gallery`：组件看板，按 Gallery 顺序展示交互效果。
- `apps/liora-docs`：独立文档主程序，展示架构说明、使用步骤、组件效果和对应代码。

## 为什么文档也必须是原生应用

Liora 的目标不是输出网页组件，而是验证一套 GPUI 原生 UI 体系是否足够支撑真实产品。因此文档站本身也必须使用 Liora 组件完成：

1. **组件自举**：文档中的标题、段落、代码块、滚动容器和弹层都由组件库自己渲染。
2. **真实 Live Demo**：`::Demo{component="ButtonTypes"}::` 会插入真实组件节点，不是截图或 iframe。
3. **反馈闭环**：如果文档排版、滚动、代码选择或弹层不好用，说明组件库也需要修。
4. **一致运行时**：Gallery 和 Docs 都使用相同的 `liora_components::init_liora`、主题、Portal 和 key binding 初始化方式。

> 项目边界：严禁引入 HTML、CSS、DOM、WebAssembly、WebView 或跨端转译思路。Markdown 只是输入文本格式，最终必须变成 GPUI/Liora 原生元素树。

## 文档目前覆盖什么

- Quick Start：从新建 Rust 项目到集成 GPUI 与 Liora 的完整流程。
- Architecture：解释 workspace 分层、GPUI 核心模型、Markdown 渲染流水线和 Live Demo 自举策略。
- Component Pages：每个组件按“一个效果 + 对应代码”拆分展示。
- CodeBlock：语法高亮、主题切换、复制和鼠标选择能力。
- Message / Toast：全局消息层和 `toast_info!`、`toast_success!` 等宏。

## 推荐阅读顺序

1. 先看 **Quick Start**，确认系统依赖、Cargo 配置、窗口创建和组件注册方式。
2. 再看 **Architecture**，理解 Liora 为什么这样分层，以及 GPUI 的 View / Element / Context 模型。
3. 然后按 Gallery 顺序阅读组件文档；每个效果下方都有对应的独立代码片段。
4. 最后看 **Authoring** 和 **Live Demo**，学习如何继续给 Docs 补充页面和活体示例。

## 当前开发状态

Liora 仍处于快速迭代阶段。公开 API、组件能力和文档结构会持续调整，但以下方向是稳定的：

- 保持纯原生 GPUI。
- 组件代码位于 `crates/liora-components`。
- 文档和示例代码分离维护。
- Docs 示例优先使用可被 `cargo check -p liora-docs --bin check_snippets` 编译检查的 `.rs` snippet。
