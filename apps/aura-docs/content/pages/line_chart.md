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

## 自定义样式与标签

### 效果

::AuraDemo{component="LineChartCustom"}::

### 代码

```rust src="line_chart/custom.rs"
```

## 每条线独立样式

### 效果

::AuraDemo{component="LineChartLineStyles"}::

### 代码

```rust src="line_chart/line_styles.rs"
```


## 大数据降采样

### 效果

::AuraDemo{component="LineChartDownsample"}::

### 代码

```rust src="line_chart/downsample.rs"
```
