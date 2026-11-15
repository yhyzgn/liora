# HighlightedText

`HighlightedText` 将宿主搜索得到的 Unicode 字符位置渲染成原生 GPUI 字重或主题强调色。文本仍是一整段可选择、可复制的内容；组件不负责搜索或分词。参考 [openidea HighlightedText](https://github.com/openideatw/openidea/blob/main/src/ui/HighlightedText.tsx) 的交互形态，代码为独立 Rust 实现。

## 匹配文本

### 效果

::Demo{component="HighlightedTextBasic"}::

### 代码

```rust src="highlighted_text/basic.rs"
```

`indices` 使用从 0 开始的 Unicode scalar（`char`）下标，不是 UTF-8 字节偏移或 UTF-16 code unit。重复位置和越界位置会被忽略；匹配样式仅用于视觉强调，不改变复制结果。重复文本出现在多行时，调用 `.id(行的稳定标识)` 保持选区独立。
