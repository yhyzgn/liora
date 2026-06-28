# Text

`Text` 是 Liora 的基础文字组件，也是轻量应用文档入口。默认情况下，`Text::new(...)` 可以用鼠标拖拽自由选择文字，并支持 `Ctrl/Cmd + A`、`Ctrl/Cmd + C`；只有按钮装饰文字、状态标签等不希望复制的界面文案才需要调用 `.selectable(false)`。

## 基础文本

`Text` 支持颜色、背景、字号、字重、斜体、下划线、删除线和 inline code 样式。已选中文字会保留在稳定的组件 id 上，不会因为父组件重渲染而丢失。

### 效果

::LioraDemo{component="TextBasic"}::

### 代码

```rust src="typography/text.rs"
```

## 轻量文档

`Text::document(...)`、`TextBlock` 和 `Text::markdown(...)` 用于 About、Help、Release notes、设置说明等原生文档场景。它们复用 Liora 的 `Title`、`Paragraph`、`CodeBlock`、`Card`、`Divider`，不引入 WebView/HTML/DOM runtime。

### 效果

::LioraDemo{component="TextDocumentBlocks"}::

### 代码

```rust src="typography/document_blocks.rs"
```

## Markdown 子集

`Text::markdown(...)` 适合应用内 help/about，不支持完整 CommonMark，但覆盖标题、段落、引用、列表、分隔线和 fenced code。

### 效果

::LioraDemo{component="TextMarkdown"}::

### 代码

```rust src="typography/markdown.rs"
```
