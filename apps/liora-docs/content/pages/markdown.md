# Markdown Renderer

本页描述 Docs 应用私有 renderer 的栈式状态机、snippet 加载和 live demo 注入。通用动态文档 SDK 请见 MarkdownView；两者都不执行 HTML，也不使用 WebView。

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
