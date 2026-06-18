# LineChart

`LineChart` 是 P10 统计图阶段的第一个原生图表控件，使用 GPUI `canvas`、`PathBuilder` 与 `paint_path` 绘制趋势线，并复用 Liora 的主题、图例、空状态和原生 hover tooltip。

## 基础趋势 + hover tooltip

### 效果

::LioraDemo{component="LineChartBasic"}::

### 代码

```rust src="line_chart/basic.rs"
```

## 多序列

### 效果

::LioraDemo{component="LineChartMulti"}::

### 代码

```rust src="line_chart/multi.rs"
```

## 空数据

### 效果

::LioraDemo{component="LineChartEmpty"}::

### 代码

```rust src="line_chart/empty.rs"
```

## 自定义样式与标签

### 效果

::LioraDemo{component="LineChartCustom"}::

### 代码

```rust src="line_chart/custom.rs"
```

## 每条线独立样式（关闭 tooltip）

### 效果

::LioraDemo{component="LineChartLineStyles"}::

### 代码

```rust src="line_chart/line_styles.rs"
```


## 大数据降采样

长序列默认使用 min/max bucket 采样保留首尾与局部峰谷；同时可以分别限制渲染点、x 轴标签和数值标签数量，避免 dense 页面因全量文本/点位准备产生卡顿。

### 效果

::LioraDemo{component="LineChartDownsample"}::

### 代码

```rust src="line_chart/downsample.rs"
```
