# Sparkline

`Sparkline` 是紧凑型迷你趋势图，适合放在指标卡片、表格单元格、列表项和 Dashboard 摘要中。它复用 P10 图表线型能力，支持趋势色、正负色、区域填充、0 基线、平滑曲线、虚线/点线和最后一个点标记。

## 基础趋势

### 效果

::LioraDemo{component="SparklineBasic"}::

### 代码

```rust src="sparkline/basic.rs"
```

## 指标卡片

### 效果

::LioraDemo{component="SparklineCards"}::

### 代码

```rust src="sparkline/cards.rs"
```

## 区域填充与基线

### 效果

::LioraDemo{component="SparklineArea"}::

### 代码

```rust src="sparkline/area.rs"
```

## 线型

### 效果

::LioraDemo{component="SparklineStyles"}::

### 代码

```rust src="sparkline/styles.rs"
```


## 长趋势降采样

### 效果

::LioraDemo{component="SparklineDownsample"}::

### 代码

```rust src="sparkline/downsample.rs"
```
