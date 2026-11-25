# Tooltip

`Tooltip` 在鼠标悬停或内部原控件获得焦点时展示简短提示。包装器不增加 Tab 停靠点，不接管按钮的 Enter/Space、点击或 Input 的文本输入；非焦点控件仍只支持悬停。

## 基础方位

支持上、下、左、右四个基础方向。悬停或用 Tab 聚焦 **Focus or hover**，按 Escape 关闭：焦点保留在原控件，持续悬停/聚焦期间重绘不会重开；焦点和指针都离开后，重新进入才再次显示。可用 **After tooltip** 离开，再用 Shift+Tab 逐步返回提示触发器。

指针可跨过默认 8 px 间距移入提示并停留；离开触发器与提示的连接区域后关闭。保持原有即时显示，不新增延时选项；淡入使用 GPUI 的 reduced-motion 策略。

Escape 按真实激活来源优先关闭提示：原触发控件仍持有焦点，或指针实际命中触发器、提示或 portal 连接区时，首次 Escape 只关闭提示，后续 Escape 再交给 Dialog/Popover。局部遮挡提示中心不会否定这些来源；原控件失焦且所有真实悬停来源都离开或被遮挡截断时，提示退休，不再拦截 Escape。

固定 GPUI 只提供单点 hit-test，不提供任意覆盖层下的剩余可见区域查询。若自定义覆盖层完全盖住提示却刻意不移走触发控件焦点，仍采用保守的 focus-first 行为，可能需第二次 Escape 才关闭容器；不通过中心或有限采样推断整个提示是否可见。

原生提示暴露 Tooltip 角色及内容名称；此固定 GPUI API 不提供触发器到提示的 `described_by` builder 关系，因此不承诺屏幕阅读器会自动朗读提示。必要信息仍应放在可见标签或原控件的辅助说明中。提示跟随已渲染的锚点边界，并在触发器卸载后退休；此行为依赖根层继续渲染提示与 `PortalLayer`，不新增轮询或空闲重绘循环。

### 效果

::Demo{component="TooltipBasic"}::

### 代码

```rust src="tooltip/basic.rs"
```

## 更多方位

通过 `Placement::*Start` 与 `Placement::*End` 可以控制提示内容与触发器的起止对齐方式。

### 效果

::Demo{component="TooltipMore"}::

### 代码

```rust src="tooltip/more.rs"
```

## 锚点移动与卸载

先点击 **Move in 2 s**，再立即 Tab 聚焦 **Lifecycle trigger**：锚点移动后，提示应跟随仍有焦点的触发器。若仅悬停且移动使触发器离开静止指针，提示应关闭。点击 **Remove in 2 s** 后重新悬停或聚焦触发器，卸载时提示应退出；**Reset trigger** 取消待执行操作并恢复触发器，不应恢复旧提示。每次重新安排操作会替换前一次待执行操作。

::Demo{component="TooltipLifecycle"}::
