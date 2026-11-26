# Card

`Card` 用于把标题、正文和底部操作聚合到一个边界清晰的内容容器中。

## 基础卡片

常规卡片适合承载简短信息；`hoverable()` 会在鼠标悬停时增强阴影并使用主题主色边框。这是视觉反馈，不会让卡片整体变成按钮。下面两个示例使用 `width_md()`，目标宽度为 300 px；`width_lg()` 为 400 px，`width(px(...))` 可指定宽度。默认允许在 flex 布局中压缩，固定宽度不是最小宽度保证。

### 效果

::Demo{component="CardBasic"}::

### 代码

```rust src="card/basic.rs"
```

## 底部操作

通过 `footer(...)` 可以把按钮、链接或其他原生节点放到卡片底部，形成常见的确认/取消操作区。本例宽度为 400 px。点击 Cancel / Save 会通过宿主 Message 层显示 `Cancel requested (demo)` / `Save requested (demo)`；它们只报告演示操作请求，不会保存、撤销或持久化任何数据。独立使用此代码时，需要宿主初始化消息层并挂载 PortalLayer。

### 效果

::Demo{component="CardFooter"}::

### 代码

```rust src="card/footer.rs"
```

## 窄容器与自定义布局

下例在 320 px 宽、220 px 高的 Liora Flex 原生滚动容器中放置两张卡片。第一张使用自定义 `header(...)` 和 `footer(...)`；正文保留 150 px 高度。两张卡片都调用 `full_width().no_shrink().no_shadow()`：填满父容器宽度、不参与 flex 压缩、移除默认阴影。容器高度不足时通过滚动查看第二张卡片，而不是压扁正文。

`full_width()` 与 `width(...)`（包括两个宽度预设）以后调用的为准。`no_shadow()` 只关闭默认阴影；如果同时启用 `hoverable()`，悬停阴影仍会出现。

### 效果

::Demo{component="CardLayout"}::

### 代码

```rust src="card/layout.rs"
```

## 标题与稳定标识

`title(...)` 优先于 `header(...)`：两者同时设置时只显示标题，与调用顺序无关。需要自定义标题区域时只设置 `header(...)`。正文始终显示，设置了 `footer(...)` 才显示底部区域。

默认标识取自 `Card::new(...)` 的源码调用位置。不同调用位置可使用默认值；同一工厂函数或循环在同一父节点下生成多张卡片时，必须为每张卡片设置不同且跨重绘稳定的 `.id(...)`，不要使用每次渲染变化的值。自定义布局示例显式命名两张卡片，按钮保持各自的原生交互状态。

卡片的容器身份不会拆分同一个 `Text::document` 的共组选区：嵌套引文仍可跨块拖选、全选和复制；双击选词后松开不会截断整词。不同父容器中的独立文档可以使用相同的局部文档 ID，选区不会因此合并。

## Gallery 对照

Gallery 的“基础卡片”“底部操作”“窄容器与自定义布局”与本页直接执行相同的 `card/basic.rs`、`card/footer.rs`、`card/layout.rs` 源码；不是另写一套近似实现。可对照检查悬停反馈、Cancel / Save 消息，以及窄容器中的滚动和完整正文高度。
