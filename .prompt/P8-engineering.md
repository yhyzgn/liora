# P8 Engineering — Aura Gallery 原生文档大屏

> 上游: `.prompt/P7-demo-self-contained.md`
> 下游: `.prompt/P9-deferred-advanced.md`（deferred backlog，非自动执行）

## 目标

将 Aura Gallery 从“组件看板”升级为 Aura UI 的**官方原生文档与组件展示大屏**。

P8 不再建设 VitePress/Web 文档站。新的技术路线是：所有文档、示例、导航、Markdown 渲染和活体组件 Demo 都运行在 **GPUI 原生视窗** 内。

## 角色与上下文

你是一位 Rust 与 GPUI 原生框架架构师，负责从 0 到 1 建设 Aura Gallery。Aura Gallery 是 Aura UI（基于 GPUI 的原生企业级组件库）的官方文档与组件展示应用。

### 绝对禁令

- 本项目 100% 运行在 GPUI 原生窗口中。
- 严禁引入 Web 文档站、跨端转译、浏览器运行时或网页渲染路径。
- 文档渲染必须基于 Rust 数据结构、GPUI 元素树、Aura 组件、Flex 布局和原生渲染能力。
- 如遇 GPUI API 差异或废弃，优先查阅本地 `gpui` 源码和当前 repo 的既有用法，不凭记忆猜测。

## 技术栈

| 层级 | 选择 | 说明 |
|------|------|------|
| UI Framework | `gpui`（沿用 workspace 当前版本/特性） | Aura Gallery 唯一渲染运行时 |
| Markdown AST | `pulldown-cmark` | 只负责 Markdown 解析与事件/AST 生成 |
| 文档渲染 | Aura Typography + Layout 组件 | 排版、折行、样式由 Aura 自举组件负责 |
| Live Demo 注入 | GPUI/Aura 真实 View Node | 特殊语法直接插入可交互组件 |
| Language | Rust（最低语义基线 2021；当前 workspace 保持现有 edition 2024） | 不因 P8 文档方案回退 Cargo edition |

## Architecture Core: Bootstrapping（自举）

文档渲染的核心挑战是富文本折行（Word Wrap）。P8 采用“自举”策略：

1. `pulldown-cmark` 只负责 Markdown 解析，输出事件/AST。
2. Markdown renderer 不实现复杂外部排版引擎。
3. 所有排版、折行、颜色、粗细、行内样式和块级布局，必须交给 Aura 自己封装的 Typography/Layout 组件处理。
4. `AuraText` / `AuraParagraph` 是最核心的基础设施：多个不同样式的文本片段必须能在同一个段落容器中流式拼接、自动换行，且不可截断。

> 命名说明：现有代码已经有 `Text` / `Paragraph`。P8 实施时应优先评估是扩展现有组件，还是新增富文本专用类型；若新增，公共命名仍遵循 ADR-009（不加 `Aura` 前缀），可使用 `RichText` / `RichParagraph` 等不冲突名称。

## 执行阶段

必须按以下 4 个子阶段顺序执行。每个子阶段完成后，必须确保 `cargo check` 无错误，并运行相关测试后再进入下一个子阶段。

### Phase 1: 基础设施搭建 & Typography 组件（核心）

- [x] 确认项目结构包含并继续使用：
  - `crates/aura-components`
  - `apps/aura-gallery`
- [x] 在 `aura-components` 中实现/扩展富文本文本片段组件：
  - 封装 GPUI 文本样式：字体粗细、颜色、背景色、等宽字体、行内代码样式等。
  - 可作为段落内的 style run，而不是只能作为独立块元素。
- [x] 在 `aura-components` 中实现/扩展富文本段落组件：
  - 接收一个或多个文本片段作为子节点。
  - 底层必须使用 GPUI `StyledText` 或当前版本等价机制。
  - 不同样式文本片段拼接后，必须在同一容器内正确流式布局并自动换行。
  - 长文本不可被截断；如 GPUI API 限制存在，必须记录限制并提供最小可验证替代方案。
- [x] 为 Typography 行为添加最小回归测试：样式片段拼接、粗体/斜体/行内代码状态、长文本换行容器不截断。

### Phase 2: Markdown 解析引擎与状态机

- [x] 在 `aura-gallery` 中引入 `pulldown-cmark`。
- [x] 新建 `apps/aura-gallery/src/markdown.rs`。
- [x] 实现：`render_markdown(md_text: &str) -> gpui::AnyElement`。
- [x] 使用 `Vec` 作为栈管理层级：
  - 遇到块级元素开始（Heading、Paragraph、List、BlockQuote 等）时，将对应 Aura 容器压入栈。
  - 遇到内联元素开始/结束（Strong、Emphasis、Code 等）时，更新当前文本样式上下文。
  - 遇到纯文本时，根据当前上下文生成文本片段，并添加到栈顶容器。
  - 遇到块级元素结束时，将栈顶容器弹出，并作为子节点添加到新的栈顶容器。
- [x] 添加 Markdown renderer 回归测试：标题、段落、粗体、斜体、列表、嵌套块级结构。

### Phase 3: 代码块与样式打磨

- [ ] 完善 `Start(Tag::CodeBlock)` 映射：
  - 使用 Aura 容器提供浅/深主题兼容的灰色背景。
  - 使用等宽字体。
  - 使用 `Scrollbar` 或现有 Aura 滚动容器提供水平滚动。
- [ ] 完善行内代码：
  - 等宽字体。
  - 浅色背景/圆角/内边距。
  - 不破坏段落流式换行。
- [ ] 构建 Aura Gallery 双栏文档窗口：
  - 左侧：文档导航树/组件目录。
  - 右侧：Markdown 渲染结果区。
  - 右侧支持垂直滚动。
  - 整体仍使用 Aura `Container` / `Menu` / `Flex` / `Scrollbar` 等原生组件。

### Phase 4: 活体组件注入（Live Demo）

- [ ] 在 Markdown 事件处理过程中识别特殊语法：
  - `::AuraDemo{component="Button"}::`
- [ ] 识别到该语法时，不渲染为普通文字。
- [ ] 根据 `component` 实例化真实 Aura 组件，例如 `Button::new("Button").primary()`。
- [ ] 将真实 GPUI/Aura view node 直接插入文档流。
- [ ] Live Demo 必须保留 hover/click 等真实交互能力。
- [ ] 添加回归测试：特殊语法不会出现在最终文本中，并能映射到对应 demo 节点。

## 保留的工程化任务

原 P8 的工程化事项仍保留，但顺序调整到原生文档大屏基础完成之后：

- [ ] Gallery 主题切换按钮（light/dark 一键切换）
- [ ] 组件搜索（输入过滤）
- [ ] 窗口标题：`Aura Gallery — Native Component Library`
- [ ] 测试体系：单元测试、组件测试、`cargo test`
- [ ] CI/CD：`cargo check` / `cargo clippy` / `cargo test` / `cargo doc`
- [ ] 发布：CHANGELOG、SemVer、LICENSE、crates.io 发布策略
- [ ] 社区文档：CONTRIBUTING、CODE_OF_CONDUCT、README

## 明确废弃的旧方案

- [x] 不再搭建 `apps/docs/` VitePress 文档站。
- [x] 不再把官方文档作为网页产物维护。
- [x] 不再将 Markdown 渲染外包给网页/浏览器/跨端运行时。
