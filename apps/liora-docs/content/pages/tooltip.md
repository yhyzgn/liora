# Tooltip

`Tooltip` 用于在鼠标悬停时展示简短提示，帮助解释按钮、图标或其他紧凑元素的含义。

## 基础方位

支持上、下、左、右四个基础方向。

### 效果

::LioraDemo{component="TooltipBasic"}::

### 代码

```rust src="tooltip/basic.rs"
```

## 更多方位

通过 `Placement::*Start` 与 `Placement::*End` 可以控制提示内容与触发器的起止对齐方式。

### 效果

::LioraDemo{component="TooltipMore"}::

### 代码

```rust src="tooltip/more.rs"
```
