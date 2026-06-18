# Native Architecture

Liora 的架构目标是：用 Rust + GPUI 构建一套真正原生的企业级组件库，并让官方 Docs 本身成为组件库能力的自举验证场。

## 架构边界

Liora 不包含 Web 渲染层。以下技术不属于本项目解法：HTML、CSS、DOM、WebView、WebAssembly、跨端转译、浏览器排版引擎。

我们只处理：

- Rust 内存模型和所有权。
- GPUI 原生窗口、元素树、布局、事件和绘制。
- GPU-backed 渲染管线。
- Liora 主题、组件、Portal、全局反馈层和文档渲染器。

## Workspace 分层

| 路径 | 职责 |
| --- | --- |
| `crates/liora-core` | 全局配置、主题注入、Portal、Tooltip、Popover、Modal、Drawer 等基础设施。 |
| `crates/liora-theme` | 设计 token：颜色、字号、圆角、按钮语义等。 |
| `crates/liora-icons` | 通用图标抽象。 |
| `crates/liora-icons-lucide` | Lucide 图标枚举与适配。 |
| `crates/liora-components` | 可复用 UI 组件与布局组件。 |
| `apps/liora-gallery` | 组件看板，按 Gallery registry 展示每个组件的交互效果。 |
| `apps/liora-docs` | 独立文档主程序，渲染 Markdown、展示 Live Demo、关联代码片段。 |

依赖方向应保持单向：app 依赖 components，components 依赖 core/theme/icons；组件库不能反向依赖 docs 或 gallery。

## GPUI 运行时模型

Liora 代码基本围绕 GPUI 的几个核心抽象组织：

1. **Application loop**：`gpui_platform::application().run` 启动原生事件循环。
2. **Global App state**：`App` 保存全局配置、主题和服务，例如 Message manager。
3. **Window**：每个窗口有自己的焦点、输入、布局和绘制上下文。
4. **View Entity**：`cx.new(|cx| MyView::new(cx))` 创建持久 View；组件状态应放在 Entity 中。
5. **Element tree**：`Render` 返回 `impl IntoElement`，GPUI 负责布局、命中测试和绘制。
6. **Notify loop**：交互事件修改状态后调用 `cx.notify()`，下一帧重新渲染受影响 View。

这个模型意味着 Liora 组件要避免隐藏的全局副作用。可持久化状态放在 View / Component struct 中，窗口级浮层通过 Portal 明确接入。

## Liora 初始化流程

应用入口一般按这个顺序：

1. `init_liora(cx, Theme::light())` 注册主题配置。
2. `MessageManager::init(cx)` 初始化全局提示。
3. 注册 `Input`、`CodeBlock`、`Checkbox`、`Radio`、`Switch`、`Dialog`、`Drawer`、`Preview` 等 key bindings。
4. `cx.open_window(WindowOptions { ... }, |_, cx| cx.new(...))` 打开窗口。
5. 根 View 渲染 `Container`、页面内容和 Portal / Message overlay。

Docs 和 Gallery 都遵循这条启动路径。

## 文档渲染流水线

Liora Docs 使用 Markdown 作为作者输入，但 Markdown 不负责最终排版。真实流程是：

1. `pulldown-cmark` 读取 Markdown，并产出 Event 流。
2. Docs parser 使用 `Vec` 作为栈管理块级结构，例如 Heading、Paragraph、List、BlockQuote、CodeBlock。
3. Inline 状态记录 strong、emphasis、strikethrough、code 等样式。
4. 普通文本被转换为 Liora `Text` / `Paragraph`，并交给 GPUI 文本布局处理折行。
5. 代码块交给 Liora `CodeBlock`，支持语法高亮、复制、主题和选择。
6. `::LioraDemo{component="..."}::` 被转换为 `LiveDemoHost`，插入真实 View Node。
7. 根 Docs shell 用 `Container`、`Menu` 和滚动区域组织导航与正文。

```rust src="architecture/render_pipeline.rs"
```

## 自举策略：用 Liora 渲染 Liora 文档

富文本折行、代码块选择、滚动性能、弹层遮罩、图片预览和全局 toast 都是组件库必须解决的问题。Docs 不绕开这些问题，而是直接使用 Liora 自己的组件：

- 段落用 `Paragraph`。
- 标题用 `Title`。
- 代码用 `CodeBlock`。
- 页面框架用 `Container`。
- 导航用 `Menu`。
- 示例效果用真实组件 Entity。
- 全局反馈用 `Message` / toast 宏。

因此 Docs 的体验问题通常也是组件库质量问题，需要回到 `crates/liora-components` 修复。

## Live Demo 与代码片段的关系

组件文档遵循固定结构：

1. 一个命名效果章节。
2. `### 效果` 下插入 `::LioraDemo{component="..."}::`。
3. `### 代码` 下引用 `content/snippets/<component>/<case>.rs`。
4. `.rs` snippet 必须能被 `cargo check -p liora-docs --bin check_snippets` 编译检查。

这样可以保证文档不是只展示效果，也不是只贴代码；每种配置都有可运行效果和可验证代码。

## 平台 feature 策略

Workspace 中的 `gpui` / `gpui_platform` 设置为 `default-features = false`。原因是：

- library crate 不应该擅自选择 Wayland、X11、字体后端等平台 feature。
- app crate 才知道自己要运行在哪个平台。
- `liora-docs` 和 `liora-gallery` 作为最终应用，通过 target-specific dependencies 启用平台 feature：Linux/FreeBSD 启用 `wayland`、`x11`、`font-kit`；macOS 启用 `font-kit`；Windows 保持 GPUI/GPUI Platform 默认 Windows 后端。

这也是外部项目接入 Liora 时应遵守的策略。

## 测试与验证层级

常用验证命令：

- `cargo check -p liora-components`：检查组件库 API。
- `cargo check -p liora-docs --bin check_snippets`：检查文档 `.rs` 代码片段。
- `cargo test -p liora-docs`：检查 Markdown 解析、Docs 页面注册、示例结构。
- `cargo run -p liora-gallery`：人工验证组件看板。
- `cargo run -p liora-docs`：人工验证文档主程序。

## 设计约束

- 组件 API 优先清晰、可组合、可在 Rust 中静态检查。
- 有状态组件必须保留 Entity，避免 render 期间重建导致焦点或输入丢失。
- 弹层、预览、消息和通知必须有明确 overlay 层。
- 文档 snippet 与 Markdown 分离维护，避免页面内长代码难以复用和检查。
- 任何新增组件都要同时考虑 Gallery demo、Docs 页面、snippet 和必要测试。
