# PieChart

`PieChart` 使用 GPUI 原生扇区路径绘制分类占比，适合展示份额分布、构成比例和分类统计。数值标注默认显示为 `值 / 总数 (百分比)`，可通过 `show_percentage_labels(false)` 关闭百分比，通过 `percentage_decimals(n)` 控制百分比小数位，也可以用 `value_label_content(...)` 自由选择数量、百分比或数量/总数；`value_label_placement(...)` 支持内部、外部自由排列和外部两侧对齐，扇区过小时默认会用折线引导到外部标注，阈值可通过 `outside_label_threshold_degrees(deg)` 调整。默认开启原生 hover tooltip，命中逻辑按极坐标扇区计算，可通过 `tooltip_hit_radius(px(...))` 放宽边界命中，或用 `show_tooltip(false)` 关闭。

## 基础饼图

### 效果

::AuraDemo{component="PieChart"}::

### 代码

```rust src="pie_chart/basic.rs"
```

## 外部自由标注与百分比

### 效果

::AuraDemo{component="PieChartCustom"}::

### 代码

```rust src="pie_chart/custom.rs"
```
