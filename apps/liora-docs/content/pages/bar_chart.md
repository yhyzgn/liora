# BarChart

`BarChart` 使用 GPUI 原生 `canvas` 与 `paint_quad` 绘制分类柱状图，适合展示分类对比、分组对比和构成堆叠。它复用 P10 图表基础设施中的 `ChartSeries`、`ChartPoint`、比例尺、坐标轴、主题色板和原生 hover tooltip。

## 基础分组 + hover tooltip

### 效果

::Demo{component="BarChartBasic"}::

### 代码

```rust src="bar_chart/basic.rs"
```

## 多序列分组（矩形 hit testing）

### 效果

::Demo{component="BarChartGrouped"}::

### 代码

```rust src="bar_chart/grouped.rs"
```

## 渐变柱体

### 效果

::Demo{component="BarChartGradient"}::

### 代码

```rust src="bar_chart/gradient.rs"
```

## 逐根柱渐变

### 效果

::Demo{component="BarChartPerBarGradient"}::

### 代码

```rust src="bar_chart/per_bar_gradient.rs"
```

## 堆叠柱状图（分段 hover）

### 效果

::Demo{component="BarChartStacked"}::

### 代码

```rust src="bar_chart/stacked.rs"
```

## 颜色、间距与标签内容

### 效果

::Demo{component="BarChartCustom"}::

### 代码

```rust src="bar_chart/custom.rs"
```

## 独立柱状图 / 迷你指标

固定柱宽/间距时会按内容宽度紧凑展示，不再把少量柱子强行铺满整行。

### 效果

::Demo{component="BarChartStandalone"}::

### 代码

```rust src="bar_chart/standalone.rs"
```

## 迷你指标多风格（可关闭 tooltip）

### 效果

::Demo{component="BarChartStandaloneStyles"}::

### 代码

```rust src="bar_chart/standalone_styles.rs"
```
