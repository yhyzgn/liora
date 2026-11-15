# Markdown Renderer

本页描述 Docs 应用私有 renderer 的栈式状态机、snippet 加载和 live demo 注入。通用动态文档 SDK 请见 MarkdownView；两者都不执行 HTML，也不使用 WebView。

## SDK Markdown extensions

`MarkdownDocument` enables tables, strikethrough, task lists, footnotes, definition lists, superscript/subscript, GFM callouts, math event parsing and bounded YAML/plus-delimited metadata blocks through the pinned `pulldown-cmark` parser. Headings expose unique document-local IDs and TOC data; `MarkdownView::set_show_toc(true, cx)` enables native Contents buttons. Safe `#fragment` links and numbered footnote/back buttons emit host navigation events; focused references show native previews. The host owns actual scrolling. HTML remains inert text; image destinations are denied unless the host explicitly approves a local path or decoded `RenderImage` through `MarkdownView::set_image_resolver`.

`MarkdownEditor` keeps Markdown as its durable source, updates the rendered projection after native input changes, and toggles between `MarkdownEditMode::Source` and `MarkdownEditMode::Rich`. Source mode uses native GPUI input for IME, selection, clipboard and undo. Rich mode provides native inline Bold/Italic/Strike/Code/Highlight actions plus Paragraph/Heading/Quote/List/Split operations for supported paragraph or heading blocks; ambiguous style edits are rejected and source remains authoritative. Native table editing exposes per-cell inputs and insert/delete/reorder row/column actions with visual column resize. Cell replacement intentionally writes escaped literal plain text and drops prior inline formatting. Composing cells resist synchronization; one undo/redo records each accepted source update unless the host opens an explicit undo group. This is bounded editing, not full WYSIWYG parity.

Inline/display math uses bounded `latex-rust` native rasterization with surrounding text RGB as its default; explicit LaTeX colors keep their scope. Mermaid flowcharts and sequence diagrams use `mermaid-svg` and native SVG rasterization; unresolved generic font aliases fall back to an installed font, and the default light diagram palette keeps a light canvas on dark surfaces. Errors render visibly with source rather than silently substituting output. Metadata access returns flat source-backed fields and diagnostics; it does not evaluate YAML/TOML. Math, diagrams, metadata and footnotes remain source-preserving at the Markdown model boundary. The full capability matrix is `docs/markdown-capability-matrix.md`.

## Typora / MarkText 能力对标边界

Native subscript/superscript runs use a smaller size and baseline shift; Linux/Xwayland light/dark captures verify visible scripts beside prose and formulas. The table editor has cell inputs, insert/delete/reorder controls and visual column resizing; Rich editing offers bounded inline formatting and block conversion/split actions. Full Typora/MarkText WYSIWYG parity and macOS/Windows native IME remain unclaimed. Linux/Xwayland synthetic composition smoke verifies preservation/commit and clipboard copy/paste; it does not by itself prove a real OS input method. Formula and ordinary prose wrapping use bounded native layout heuristics rather than complete Unicode line-breaking parity.

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
