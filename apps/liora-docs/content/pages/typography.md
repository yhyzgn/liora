# Typography

Liora Typography 可以把多个不同样式的文本片段合成为同一个 `StyledText` 流。

这意味着 **strong**、*emphasis*、~~strike~~ 和 `inline code` 可以在同一段落内自动折行，而不是拆成多个独立块。

## Text

`Text` 用于描述一段文字及其样式：颜色、背景、字号、字重、斜体、下划线、删除线和等宽字体。默认情况下，普通文本使用 GPUI 的系统 UI 字体；`code_style` 只表达 inline code 语义，不会强制品牌字体。应用如需固定字体，应通过 `FontConfig::system().with_ui_families([...]).with_code_families([...])` 指定有序 family 兜底列表，或先用 `load_app_fonts` / `load_fonts_from_dir` / `load_font_assets` / `load_embedded_fonts` 注册私有字体资源后再配置 fallback 列表。建议原生应用优先发布 TTF/OTF/TTC/OTC，并通过 `FontLoadOptions::require_family` 验证目标 family 确实被当前 GPUI 后端识别。

## Paragraph

`Paragraph` 接收一个或多个 `Text` 片段，并把它们拼接为单个 GPUI `StyledText`。

```rust src="typography/paragraph.rs"
```

## 自举意义

文档渲染不实现独立排版引擎，而是依赖 Liora 自己的 Typography 组件。这样文档能力和组件库能力会同步成长。
