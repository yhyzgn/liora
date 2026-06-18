# Markdown Renderer

Liora Docs 的 Markdown renderer 是一个栈式状态机。

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

## 当前边界

Markdown 表格、图片、链接跳转等能力还未作为交互控件完整实现。它们应该继续以 Liora 原生组件方式补齐，而不是引入浏览器能力。
