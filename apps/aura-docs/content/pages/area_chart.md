# AreaChart

`AreaChart` 使用 GPUI 原生 `PathBuilder` 填充路径绘制面积趋势图，适合展示趋势规模、累计流量和多渠道构成。它不是 Web 图表封装，所有绘制都发生在 GPUI 的原生绘制管线中。

## 基础面积

### 效果

::AuraDemo{component="AreaChartBasic"}::

### 代码

```rust src="area_chart/basic.rs"
```

## 多序列叠加

### 效果

::AuraDemo{component="AreaChartOverlay"}::

### 代码

```rust src="area_chart/overlay.rs"
```

## 堆叠面积

### 效果

::AuraDemo{component="AreaChartStacked"}::

### 代码

```rust src="area_chart/stacked.rs"
```

## 平滑面积与标签配置

### 效果

::AuraDemo{component="AreaChartCustom"}::

### 代码

```rust src="area_chart/custom.rs"
```


## 大数据降采样

长序列默认使用 min/max bucket 采样保留首尾与局部峰谷；同时可以分别限制渲染点、x 轴标签和数值标签数量，避免 dense 页面因全量文本/点位准备产生卡顿。

### 效果

::AuraDemo{component="AreaChartDownsample"}::

### 代码

```rust src="area_chart/downsample.rs"
```
