# Paragraph

`Paragraph` 用于正文段落和混合样式文本流。它接收一个或多个 `Text` 片段，并把这些片段渲染为同一个可选择文本区域；自动换行、inline code、颜色、字重和下划线都不应该把段落拆成多个无法连续选择的块。

## 自动换行连续选择

长段落会根据父容器宽度自动折行。用户从第一行拖到第二行、第三行时，应得到同一个连续选区；点击段落外空白区域会清除当前选区。

### 效果

::LioraDemo{component="ParagraphWrapped"}::

### 代码

```rust src="typography/paragraph.rs"
```


## 跨多个文本块选择

当一段内容需要拆成多个 `Text` 与 `Paragraph` 块，但用户仍然期望像文档一样从标题拖选到正文时，使用 `SelectableTextGroup`。它会把多个块合并到同一个可选择文本流里，并用 `separator` 控制块之间的复制文本分隔符。

### 效果

::LioraDemo{component="SelectableTextGroupBasic"}::

### 代码

```rust src="typography/selectable_text_group.rs"
```

## 混合样式片段

多个 `Text` 子片段会被合并成一组 `TextRun`，适合在同一段正文中混排强调、语义色、inline code 和链接式下划线。除非是装饰性文案，否则不建议关闭默认选择能力。

### 效果

::LioraDemo{component="TypographyParagraph"}::

### 代码

```rust src="typography/paragraph.rs"
```
