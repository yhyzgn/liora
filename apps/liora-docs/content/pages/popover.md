# Popover

点击触发后在目标附近渲染原生卡片内容，适合承载少量说明、二级操作或轻量表单。Popover 基于 Liora Portal 与 GPUI 原生元素实现，内容不是 Web 浮层。

## 基础卡片

用于在按钮附近展示标题、说明和操作按钮。推荐为每个 Popover 指定稳定 `id`，便于手动关闭和测试定位。

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
