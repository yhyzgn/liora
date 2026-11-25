# Popover

点击触发后在目标附近渲染原生卡片内容，适合承载少量说明、二级操作或轻量表单。Popover 基于 Liora Portal 与 GPUI 原生元素实现，内容不是 Web 浮层。

## 基础卡片

用于在按钮附近展示标题、说明和操作按钮。推荐为每个 Popover 指定稳定 `id`，便于手动关闭和测试定位。

使用 Liora `Button` 作为触发器时，Tab 聚焦后按 Enter/Space 打开，ESC 可从触发按钮或卡片内的交互控件关闭。内容中的 Confirm 通过 `clear_popover` 关闭，并将焦点返回原触发按钮；如果焦点已移到其他控件，关闭不会抢回焦点。Popover 包装层不添加额外 Tab 停靠点，也不拦截可编辑触发器的文字输入。

### 效果

::Demo{component="PopoverBasic"}::

### 代码

```rust src="popover/basic.rs"
```

## 十二方向定位

通过 `Placement` 控制弹出方向和起始/结束对齐。文档中的所有方向都由同一个原生定位管线计算，不需要额外布局容器。

### 效果

::Demo{component="PopoverPlacements"}::

### 代码

```rust src="popover/placements.rs"
```

## 关闭策略与偏移

默认点击外部和按下 ESC 会关闭。对于必须显式确认的浮层，可关闭这些行为，并通过 `clear_popover` 在内容内部手动关闭。

### 效果

::Demo{component="PopoverCloseStrategy"}::

### 代码

```rust src="popover/close_strategy.rs"
```

## 锚点移动与卸载

点击 **Move in 2 s** 后立即打开 **Lifecycle trigger**，观察卡片是否跟随移动后的触发器。点击 **Remove in 2 s** 后立即打开触发器，卸载时卡片及外部点击遮罩应一起清除。**Reset trigger** 取消待执行操作并恢复触发器，不应重开旧卡片；重新安排操作会替换前一次待执行操作。

::Demo{component="PopoverLifecycle"}::
