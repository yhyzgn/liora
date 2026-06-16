# Progress

`Progress` 用于展示操作进度，可通过状态、内显文本、颜色、环状形态与动画反馈任务阶段。
条状与环状进度均为 GPUI 原生元素/Path 渲染，不依赖 Web 排版或 Canvas 兼容层。

## 基础用法

### 效果

::AuraDemo{component="ProgressBasic"}::

### 代码

```rust src="progress/basic.rs"
```

## 百分比内显

### 效果

::AuraDemo{component="ProgressInside"}::

### 代码

```rust src="progress/inside.rs"
```

## 不同状态

### 效果

::AuraDemo{component="ProgressStatus"}::

### 代码

```rust src="progress/status.rs"
```

## 自定义颜色

### 效果

::AuraDemo{component="ProgressColor"}::

### 代码

```rust src="progress/color.rs"
```

## 完成态渐变

`gradient` 可用于条状和环状进度；当进度达到 100% 时，`complete_color` 会覆盖最终颜色。未显式设置时默认使用渐变色数组的最后一种颜色。

### 效果

::AuraDemo{component="ProgressGradientComplete"}::

### 代码

```rust src="progress/gradient_complete.rs"
```

## 环状进度条

### 效果

::AuraDemo{component="ProgressCircle"}::

### 代码

```rust src="progress/circle.rs"
```

## 动画与环形样式自定义

### 效果

::AuraDemo{component="ProgressCustom"}::

### 代码

```rust src="progress/custom.rs"
```

## 环形渐变与完成色

### 效果

::AuraDemo{component="ProgressCircleGradient"}::

### 代码

```rust src="progress/circle_gradient.rs"
```
