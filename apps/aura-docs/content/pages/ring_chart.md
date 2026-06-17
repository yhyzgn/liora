# RingChart

`RingChart` 是带中心留空的饼图版本，适合 KPI 占比、进度型统计和更紧凑的分布展示。数值标注默认显示为 `值 / 总数 (百分比)`，支持 `show_percentage_labels(false)`、`percentage_decimals(n)`、`value_label_content(...)`、`value_label_placement(...)` 与 `outside_label_threshold_degrees(deg)`，小扇区会自动切换为外部折线标注以避免文字挤压。默认开启原生 hover tooltip，命中逻辑按圆环分段计算并排除内圆空洞，可通过 `tooltip_hit_radius(px(...))` 或 `show_tooltip(false)` 配置。

## 基础圆环

### 效果

::AuraDemo{component="RingChart"}::

### 代码

```rust src="ring_chart/basic.rs"
```

## 两侧对齐外部标注

### 效果

::AuraDemo{component="RingChartCustom"}::

### 代码

```rust src="ring_chart/custom.rs"
```

## 图例与比例值完全外置

圆环内部不绘制文字，也不绘制折线引导；标签统一放在图例区域。图例支持水平排列，也支持垂直放在图形左侧或右侧；可通过 `max_items` 只展示前 N 项，并可通过 `ChartValueLabelContent` 选择显示数量、百分比或数量/总数/百分比组合。

### 效果

::AuraDemo{component="RingChartExternal"}::

### 代码

```rust src="ring_chart/external.rs"
```
