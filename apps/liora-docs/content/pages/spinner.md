# Spinner

`Spinner` 是独立的细粒度加载图标，适合嵌入按钮、状态栏、列表行、工具栏和卡片局部刷新。它只表达“小范围仍在进行中”，不替代 `Loading` 的遮罩式页面等待。

## 尺寸与图标

### 效果

::Demo{component="SpinnerSizes"}::

### 代码

```rust src="spinner/sizes.rs"
```

## 语义颜色

### 效果

::Demo{component="SpinnerColors"}::

### 代码

```rust src="spinner/colors.rs"
```

## 组合场景

### 效果

::Demo{component="SpinnerComposition"}::

### 代码

```rust src="spinner/composition.rs"
```

## Native spring 原生弹簧动效

下面与 Gallery 共用同一有状态示例。点击 `Toggle spring target` 改变间距，动画中再次点击可重定向；相同 element ID 保留当前位置和速度。`Remove spring` 卸载动画行，`Restore spring` 重新挂载；新挂载默认从当前目标开始，不继承已卸载元素的运动状态。

`MotionSpring::Gentle` 和 `Snappy` 为临界阻尼，`Bouncy` 允许过冲；不要将有过冲的弹簧直接用于必须保持在固定范围内的值。入口动画可显式 `.from(...)`；原生 spring 遵守宿主的 `App::set_reduce_motion`，示例不会改写全局减少动效设置。实际帧耗时、可见运动与卸载停止需要独立原生采样，不由静态画面证明。

### 效果

::Demo{component="SpinnerSpring"}::

### 代码

```rust src="spinner/spring.rs"
```
