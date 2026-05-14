# PieChart

`PieChart` 使用 GPUI 原生扇区路径绘制分类占比，适合展示份额分布、构成比例和分类统计。数值标注默认显示为 `值 / 总数 (百分比)`，可通过 `show_percentage_labels(false)` 关闭百分比，通过 `percentage_decimals(n)` 控制百分比小数位；当扇区过小时会自动使用折线引导到外部标注，阈值可通过 `outside_label_threshold_degrees(deg)` 调整。

## 基础饼图

### 效果

::AuraDemo{component="PieChart"}::

### 代码

```rust src="pie_chart/basic.rs"
```
