# LineChart

`LineChart` 是 P10 统计图阶段的第一个原生图表控件，使用 GPUI `canvas`、`PathBuilder` 与 `paint_path` 绘制趋势线，并复用 Aura 的主题、图例和空状态。

## 基础趋势

### 效果

::AuraDemo{component="LineChartBasic"}::

### 代码

```rust src="line_chart/basic.rs"
```

## 多序列

### 效果

::AuraDemo{component="LineChartMulti"}::

### 代码

```rust src="line_chart/multi.rs"
```

## 空数据

### 效果

::AuraDemo{component="LineChartEmpty"}::

### 代码

```rust src="line_chart/empty.rs"
```
