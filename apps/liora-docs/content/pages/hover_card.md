# HoverCard

`HoverCard` 是基于 `Popover` 的轻量预览 facade，适合 profile、链接预览和上下文摘要。

悬停触发，无需点击。指针移入预览卡片后保持显示，离开目标和卡片后关闭；仍可点击外部或按 Escape 关闭。

下例使用可聚焦的 `Button`，Gallery 与 Docs 都执行这里展示的源码。Tab 聚焦触发按钮后，可用 Enter/Space 打开、Escape 关闭；纯指针悬停不会抢走当前编辑框焦点。悬停预览的 Escape 仅作用于打开它的窗口，不响应 Ctrl/Alt/Shift 等修饰键；输入法或控件已消费的原始按键仍由输入控件处理。

目标与卡片之间保留可穿过的安全区域；离开该区域即关闭，不创建延迟开关任务。触发节点卸载时释放预览，即使宿主仍持有预览实体。

## Basic

### 效果

::Demo{component="HoverCardBasic"}::

### 代码

```rust src="hover_card/basic.rs"
```
