# Tour

`Tour` 用顶层浮层引导用户理解界面关键区域。它不应该被嵌入页面文档流，而是通过 `Tour::show(cx)` 进入 Aura active modal layer，在当前窗口上方显示遮罩、步骤卡片和前进/后退/完成操作。

## 基础引导

### 效果

::AuraDemo{component="Tour"}::

### 代码

```rust src="tour/basic.rs"
```

## 从中间步骤开始

`active_index` 设置初始步骤，后续前进/后退由 Tour 浮层自身推进，同时触发 `on_change`。

### 代码

```rust src="tour/middle.rs"
```

## 透明遮罩模式

`show_mask(false)` 不显示半透明遮罩，但仍是窗口顶层浮层；`close_on_click_outside(true)` 可允许点击浮层外部关闭。

### 代码

```rust src="tour/no_mask.rs"
```
