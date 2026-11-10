# Clipboard

`clipboard_text` / `write_text_to_clipboard` 是组件和应用共享的原生文本剪贴板 helper，无需创建额外的 Clipboard 控件。

## 复制与读取

### 效果

1. 编辑「待复制文本」，点击「复制到系统剪贴板」。该操作会替换系统剪贴板内容，包括空文本。
2. 点击「读取 / 粘贴剪贴板」，下方显示从系统剪贴板实际读到的文本，而不是复制源的本地副本。
3. 也可以在其他应用中复制文字，然后点击读取，验证跨应用粘贴。读取结果是最近一次操作的快照，不会自动刷新。

复制后的提示仅表示已调用原生写入 API；请通过读取按钮或其他应用粘贴验证结果。GPUI 将空文本也视为无可读取文本；剪贴板为空、文本为空或仅包含图片等非文本数据时，均显示无可读取文本的提示。

::Demo{component="Clipboard"}::

### 代码

将输入框作为 `Entity<Input>` 保存在视图中，让编辑状态跨渲染保留。复制使用现有 helper，读取直接使用 GPUI `read_from_clipboard` / `ClipboardItem::text`。

```rust src="clipboard/helper.rs"
```

## Helper API

- `write_text_to_clipboard(cx, text)`：立即向原生剪贴板写入纯文本，适合按钮、菜单和快捷键事件；写入 API 不返回系统级成功确认。
- `clipboard_text(text)`：构造 GPUI `ClipboardItem`，供需要自行调用 `cx.write_to_clipboard(item)` 的场景使用。
- `cx.read_from_clipboard().and_then(|item| item.text())`：返回当前可读取的非空文本；`None` 表示没有可读取文本（包括空文本）。

这些 helper 不会后台监听或主动读取剪贴板；示例仅在用户点击按钮时访问系统剪贴板。
