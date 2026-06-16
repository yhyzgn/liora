# BarChart

`BarChart` 使用 GPUI 原生 `canvas` 与 `paint_quad` 绘制分类柱状图，适合展示分类对比、分组对比和构成堆叠。它复用 P10 图表基础设施中的 `ChartSeries`、`ChartPoint`、比例尺、坐标轴和主题色板。

## 基础分组

### 效果

::AuraDemo{component="BarChartBasic"}::

### 代码

```rust src="bar_chart/basic.rs"
```

## 多序列分组

### 效果

::AuraDemo{component="BarChartGrouped"}::

### 代码

```rust src="bar_chart/grouped.rs"
```

## 渐变柱体

### 效果

::AuraDemo{component="BarChartGradient"}::

### 代码

```rust src="bar_chart/gradient.rs"
```

## 逐根柱渐变

### 效果

::AuraDemo{component="BarChartPerBarGradient"}::

### 代码

```rust src="bar_chart/per_bar_gradient.rs"
```

## 堆叠柱状图

### 效果

::AuraDemo{component="BarChartStacked"}::

### 代码

```rust src="bar_chart/stacked.rs"
```

## 颜色、间距与标签内容

### 效果

::AuraDemo{component="BarChartCustom"}::

### 代码

```rust src="bar_chart/custom.rs"
```

## 独立柱状图 / 迷你指标

固定柱宽/间距时会按内容宽度紧凑展示，不再把少量柱子强行铺满整行。

### 效果

::AuraDemo{component="BarChartStandalone"}::

### 代码

```rust src="bar_chart/standalone.rs"
```

## 迷你指标多风格

### 效果

::AuraDemo{component="BarChartStandaloneStyles"}::

### 代码

```rust src="bar_chart/standalone_styles.rs"
```
