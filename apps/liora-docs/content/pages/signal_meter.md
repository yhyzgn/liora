# SignalMeter

`SignalMeter` 用于展示移动信号或 Wi-Fi 风格的等级状态，支持自定义等级数量、激活色、未激活色、柱宽、间距和整体高度。

## 移动信号

### 效果

::Demo{component="SignalMeterMobile"}::

### 代码

```rust src="signal_meter/mobile.rs"
```

## Wi-Fi 风格

### 效果

::Demo{component="SignalMeterWifi"}::

### 代码

```rust src="signal_meter/wifi.rs"
```

## 总信号数与每级颜色

### 效果

::Demo{component="SignalMeterLevels"}::

### 代码

```rust src="signal_meter/levels.rs"
```

## 按当前等级统一着色

### 效果

::Demo{component="SignalMeterThresholdColors"}::

### 代码

```rust src="signal_meter/threshold_colors.rs"
```
