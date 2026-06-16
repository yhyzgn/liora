# HeatBar

`HeatBar` 是高密度时间轴热力条，适合展示日志、告警、错误、请求峰值等按时间分布的数据。它不是日历网格热力图，而是由大量窄竖条组成的时间轴热力分布。每个 item 的 `value` 控制高度，`color_ranges` 可按数值区间自动映射严重等级颜色；item 自身颜色作为 fallback。

## 事件热力

### 效果

::AuraDemo{component="HeatBarEvents"}::

### 代码

```rust src="heat_bar/events.rs"
```
