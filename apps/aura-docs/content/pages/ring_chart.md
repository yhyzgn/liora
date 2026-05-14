# RingChart

`RingChart` 是带中心留空的饼图版本，适合 KPI 占比、进度型统计和更紧凑的分布展示。数值标注默认显示为 `值 / 总数 (百分比)`，支持 `show_percentage_labels(false)`、`percentage_decimals(n)` 与 `outside_label_threshold_degrees(deg)`，小扇区会自动切换为外部折线标注以避免文字挤压。

## 基础圆环

### 效果

::AuraDemo{component="RingChart"}::

### 代码

```rust src="ring_chart/basic.rs"
```
