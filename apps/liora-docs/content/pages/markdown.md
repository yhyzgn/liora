# Markdown Renderer

本页描述 Docs 应用私有 renderer 的栈式状态机、snippet 加载和 live demo 注入。通用动态文档 SDK 请见 MarkdownView；两者都不执行 HTML，也不使用 WebView。

## SDK Markdown extensions

`MarkdownDocument` enables tables, strikethrough, task lists, footnotes, definition lists, superscript/subscript, GFM callouts, math event parsing and bounded YAML/plus-delimited metadata blocks through the pinned `pulldown-cmark` parser. Headings expose unique document-local IDs and TOC data; `MarkdownView::set_show_toc(true, cx)` enables native Contents buttons. Safe `#fragment` links and numbered footnote/back buttons emit host navigation events; focused references show native previews. The host owns actual scrolling. HTML remains inert text; image destinations are denied unless the host explicitly approves a local path or decoded `RenderImage` through `MarkdownView::set_image_resolver`.

`MarkdownEditor` keeps Markdown as its durable source and switches between `MarkdownEditMode::Source` and `MarkdownEditMode::Rich`. Rich mode owns one native editor selection/input handler and virtual viewport; the host supplies a bounded height. Source-offset selection APIs replace paragraph Input access. Native table cells retain their inputs and insert/delete/reorder/visual-resize actions; cell replacement writes escaped literal plain text and intentionally drops prior inline formatting. Canonical change events exclude IME preedit. The current migration and exact acceptance evidence are recorded in `.context/tasks/011_quiet_markdown_native_20261117.md`.

Inline/display math uses bounded `latex-rust` native rasterization with surrounding text RGB as its default; explicit LaTeX colors keep their scope. Mermaid flowcharts and sequence diagrams use `mermaid-svg` and native SVG rasterization; unresolved generic font aliases fall back to an installed font, and the default light diagram palette keeps a light canvas on dark surfaces. Errors render visibly with source rather than silently substituting output. Metadata access returns flat source-backed fields and diagnostics; it does not evaluate YAML/TOML. Math, diagrams, metadata and footnotes remain source-preserving at the Markdown model boundary. The full capability matrix is `docs/markdown-capability-matrix.md`.

## Typora / MarkText 能力对标边界

Native subscript/superscript runs use a smaller size and baseline shift; Linux/Xwayland light/dark captures verify visible scripts beside prose and formulas. The table editor has cell inputs, insert/delete/reorder controls and visual column resizing. Full Typora/MarkText/Quiet.md parity and macOS/Windows native IME remain unclaimed. Existing Linux/Xwayland composition/clipboard evidence predates the unified editor and must not be substituted for its acceptance. Formula and ordinary prose wrapping use bounded native layout heuristics rather than complete Unicode line-breaking parity.

对标范围：CommonMark/GFM 块与行内样式、任务列表、表格工具、脚注跳转、数学/图表呈现、图片权限、前置元数据、目录/内部链接，以及 Source/Rich 编辑的源文档保真。HTML/iframe/video 执行、隐式网络资源和浏览器式 WYSIWYG 不属于 Liora 默认 Markdown 控件边界。

## 块级元素

- Heading
- Paragraph
- BlockQuote
- List / Item
- CodeBlock
- Rule

## 内联元素

- Strong
- Emphasis
- Strikethrough
- Inline code

## 状态机核心

```rust src="markdown/state_machine.rs"
```

## Docs renderer 的当前边界

Docs 私有 renderer 的链接、图片、表格交互由其自身实现决定，不能把 SDK 新增的行为视为 Docs 自动拥有。通用动态文档应使用 MarkdownView 页面介绍的 SDK。
