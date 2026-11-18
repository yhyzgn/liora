# Slider

`Slider` 用于在连续或离散范围内选择数值。

步进网格以范围下界为起点，吸附后仍限制于范围内，两个端点均可到达。反向边界按大小排序，非有限边界和非正或非有限步长保留默认设置。

## 基础滑块

### 效果

::Demo{component="SliderBasic"}::

### 代码

```rust src="slider/basic.rs"
```

## 步进滑块

### 效果

::Demo{component="SliderStep"}::

### 代码

```rust src="slider/step.rs"
```
