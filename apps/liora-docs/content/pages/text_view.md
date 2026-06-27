# TextView

`TextView` 是 SDK 级轻量文档视图，用于 About、Help、Release notes、设置说明、产品内帮助等场景。它复用 Liora 的 `Title`、`Paragraph`、`CodeBlock`、`Card`、`Divider` 等原生 GPUI 组件，不依赖 Docs 应用内部的完整 Markdown renderer，也不会引入 WebView/HTML/DOM runtime。

## 结构化文档块

### 效果

::LioraDemo{component="TextViewBlocks"}::

### 代码

```rust src="text_view/blocks.rs"
```

## 快速 Markdown 子集

### 效果

::LioraDemo{component="TextViewMarkdown"}::

### 代码

```rust src="text_view/markdown.rs"
```

## 无边框嵌入模式

### 效果

::LioraDemo{component="TextViewInline"}::

### 代码

```rust src="text_view/inline.rs"
```
