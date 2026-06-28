# Typography

Liora Typography 可以把多个不同样式的文本片段合成为同一个 `StyledText` 流。

这意味着 **strong**、*emphasis*、~~strike~~ 和 `inline code` 可以在同一段落内自动折行，而不是拆成多个独立块。

`Text`、`Title`、`Paragraph` 以及 `Text::document(...)` 里的标题、段落、引用和列表文字默认都可以用鼠标拖拽自由选择，并支持 `Ctrl/Cmd + A`、`Ctrl/Cmd + C`。只有装饰性标签、状态短文案等不希望被选中的场景，才需要显式调用 `.selectable(false)`。

## Text

`Text` 用于描述一段文字及其样式：颜色、背景、字号、字重、斜体、下划线、删除线和等宽字体。它默认启用原生鼠标选择；如果只是按钮内装饰性文案或状态标签，可以调用 `.selectable(false)` 关闭。默认情况下，普通文本使用 GPUI 的系统 UI 字体；`code_style` 只表达 inline code 语义，不会强制品牌字体。应用如需固定字体，应通过 `FontConfig::system().with_ui_families([...]).with_code_families([...])` 指定有序 family 兜底列表，或先用 `load_app_fonts` / `load_fonts_from_dir` / `load_font_assets` / `load_embedded_fonts` 注册私有字体资源后再配置 fallback 列表。建议原生应用优先发布 TTF/OTF/TTC/OTC，并通过 `FontLoadOptions::require_family` 验证目标 family 确实被当前 GPUI 后端识别。

## Paragraph

`Paragraph` 接收一个或多个 `Text` 片段，并把它们拼接为单个 GPUI `StyledText`。默认情况下整段文字作为连续文本流可被鼠标拖拽选择，即使不同片段使用了不同颜色、字重或 inline code 样式。

### 效果

::LioraDemo{component="TypographyParagraph"}::

### 代码

```rust src="typography/paragraph.rs"
```

## 自举意义

文档渲染不实现独立排版引擎，而是依赖 Liora 自己的 Typography 组件。这样文档能力和组件库能力会同步成长。


## Text 结构化文档块

`Text::document(...)` 覆盖原先 TextView 的结构化文档能力，用同一个 Text 入口渲染 About、Help、Release notes、设置说明等轻量原生文档。它复用 Liora 的 `Title`、`Paragraph`、`CodeBlock`、`Card`、`Divider`，不引入 WebView/HTML/DOM runtime。

### 效果

::LioraDemo{component="TextDocumentBlocks"}::

### 代码

```rust src="typography/document_blocks.rs"
```

## Text Markdown 子集

`Text::markdown(...)` 适合应用内 help/about，不支持完整 CommonMark，但能覆盖标题、段落、引用、列表、分隔线和 fenced code。

### 效果

::LioraDemo{component="TextMarkdown"}::

### 代码

```rust src="typography/markdown.rs"
```

## Text 无边框嵌入

需要把说明嵌入设置页、侧栏详情或空状态时，可以关闭外框并控制 selectable。下面示例使用 `.selectable(false)` 演示装饰性说明的 opt-out；常规正文不要关闭选择能力。

### 效果

::LioraDemo{component="TextDocumentInline"}::

### 代码

```rust src="typography/document_inline.rs"
```
