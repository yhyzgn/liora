# CandlestickChart

`CandlestickChart` 使用 GPUI 原生 `canvas` 绘制 OHLC 蜡烛图。它适合金融行情、交易终端、价格区间监控、库存高低水位等需要同时表达 open/high/low/close 的场景。

## 基础 OHLC + hover tooltip

### 效果

::Demo{component="CandlestickChartBasic"}::

### 代码

```rust src="candlestick_chart/basic.rs"
```

## 自定义涨跌色与实体宽度

### 效果

::Demo{component="CandlestickChartCustom"}::

### 代码

```rust src="candlestick_chart/custom.rs"
```

## 密集数据降采样 + 收盘价标签

### 效果

::Demo{component="CandlestickChartDense"}::

### 代码

```rust src="candlestick_chart/dense.rs"
```
