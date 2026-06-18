# Preview

图片预览弹层。Preview 可以包裹缩略图、卡片或按钮等任意原生触发器，打开后使用 Liora Portal 展示居中大图。

## 图片触发

最常见用法是包裹 `Image` 缩略图。缩略图自身关闭 preview，外层 Preview 负责打开大图。

### 效果

::LioraDemo{component="PreviewImageTrigger"}::

### 代码

```rust src="preview/image_trigger.rs"
```

## 自定义触发器

Preview 的 child 可以是任意 Liora/GPUI 元素，例如卡片、按钮或自定义行项目。

### 效果

::LioraDemo{component="PreviewCustomTrigger"}::

### 代码

```rust src="preview/custom_trigger.rs"
```

## 关闭策略

默认点击图片外阴影区域或按 ESC 会关闭。对于受控流程，可通过 `close_on_escape(false)` 禁用 ESC 关闭，也可以通过 `close_on_click_outside(false)` 禁用点击外部关闭。

### 效果

::LioraDemo{component="PreviewEscape"}::

### 代码

```rust src="preview/escape.rs"
```
