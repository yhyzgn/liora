# Tour

`Tour` 用顶层浮层引导用户理解界面关键区域。它不是普通页面组件，不能作为子元素嵌入文档流；正确用法是通过 `Tour::show(cx)` 进入 Liora active modal layer，在当前窗口上方显示遮罩、步骤卡片和前进/后退/完成操作。

## 基础引导

### 效果

::Demo{component="Tour"}::

### 代码

```rust src="tour/basic.rs"
```

## 从中间步骤开始

`active_index` 设置初始步骤，后续前进/后退由 Tour 浮层自身推进，同时触发 `on_change`。

### 效果

::Demo{component="TourMiddle"}::

### 代码

```rust src="tour/middle.rs"
```

## 透明遮罩模式

`show_mask(false)` 不显示半透明遮罩，但仍是窗口顶层浮层；`close_on_click_outside(true)` 可允许点击浮层外部关闭。

### 效果

::Demo{component="TourNoMask"}::

### 代码

```rust src="tour/no_mask.rs"
```

## 受控关闭策略

默认 Tour 支持 ESC 关闭，外部点击关闭默认关闭以避免误触中断引导。需要强制用户点击关闭或完成按钮时，可同时设置 `close_on_escape(false)` 与 `close_on_click_outside(false)`。

### 效果

::Demo{component="TourClosePolicy"}::

### 代码

```rust src="tour/close_policy.rs"
```
