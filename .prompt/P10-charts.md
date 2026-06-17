# P10 Charts — 原生统计图组件

> 上游: `.prompt/P8-engineering.md` / 当前阶段  
> 参考: GPUI 官方源码优先；`https://github.com/vicanso/zedis` 仅作为 GPUI Metrics 图表案例参考

## 目标

为 Aura UI 新增一组企业级统计图控件，用于 Dashboard、监控、报表和数据分析页面。所有图表必须是 **Rust + GPUI 原生绘制**，并作为 `aura-components` 的一等组件交付。

## 绝对边界

- 禁止 WebView、HTML、CSS、DOM、SVG DOM、WASM、ECharts、Vega、Plotly 等 Web 图表运行时。
- 禁止把图表作为图片远程渲染后嵌入；图表必须可由 GPUI paint pipeline 直接绘制。
- 外部案例仓库只能启发结构和 GPUI 用法，不能照搬 API、命名或依赖。
- 如 GPUI API 不确定，优先查本地 `gpui` 源码和当前仓库既有用法。

## 技术路线

### GPUI 官方能力优先

当前本地 GPUI 源码提供的关键能力：

- `gpui::canvas(prepaint, paint)`：短期自定义绘制入口。
- `gpui::PathBuilder`：构建 stroke/fill path，支持 line、curve、arc、polygon、dash array。
- `Window::paint_path(path, color)`：绘制矢量 path。
- `Window::paint_quad(...)` / `fill(...)` / `quad(...)`：绘制柱体、背景、网格辅助块。
- GPUI TextSystem / Aura `Text`、`Paragraph`：绘制标题、坐标轴标签、legend、tooltip 文案。

### zedis 案例参考结论

`vicanso/zedis` 的 Metrics 页面采用 GPUI 原生方案：

- 用 `canvas(...)` 建立图表绘制区域。
- 将图表拆为 `scale`、`axis/grid`、`shape` 层。
- Area/Line/Bar 都在 paint 回调中根据 bounds 计算 scale，再绘制 path/quad。

Aura P10 采用同类分层思想，但实现自己的组件 API、主题 token、测试、Gallery demo 和 Docs 文档。

## 架构分层

建议文件结构：

```text
crates/aura-components/src/
├── chart.rs            # 公共数据模型、ChartTheme、Legend、Tooltip、ChartFrame
├── chart_scale.rs      # ScaleLinear / ScaleBand / ScalePoint
├── chart_axis.rs       # Axis、Grid、Tick、Label 布局
├── chart_shape.rs      # Line/Area/Bar/Pie/Ring/Sparkline 绘制 primitive
├── line_chart.rs       # LineChart 组件
├── area_chart.rs       # AreaChart 组件
├── bar_chart.rs        # BarChart 组件
├── pie_chart.rs        # PieChart / RingChart 组件
└── sparkline.rs        # Sparkline 组件
```

如实现初期文件过多影响 review，可先在 `chart.rs` 内部模块化，稳定后再拆文件。

## 数据模型建议

```rust
ChartSeries::new("CPU")
    .points(vec![ChartPoint::new("10:00", 12.0), ...])
    .color(theme.primary.base)

LineChart::new(series)
    .height(px(280.0))
    .show_grid(true)
    .show_legend(true)
    .y_format(|v| format!("{v:.0}%"))
```

原则：

- X 轴第一版支持分类值（`SharedString`）和后续可扩展数值/时间值。
- Y 轴第一版支持 `f64`，自动计算 min/max，可手动覆盖 domain。
- 空数据必须渲染 Aura `Empty` 或轻量占位，不 panic。
- 所有组件提供 `.id(...)` 覆盖，默认使用内置唯一 ID。

## 首批组件范围

| 组件 | 必备能力 | 后续扩展 |
|------|----------|----------|
| `LineChart` | 单/多 series、直线、点标记、grid、axis、legend | smooth curve、step line、hover tooltip |
| `AreaChart` | 单/多 series、透明填充、axis/grid | stacked area、gradient fill |
| `BarChart` | 竖向柱、分类 x 轴、axis/grid、legend | grouped/stacked/horizontal bar |
| `PieChart` | 扇区绘制、百分比、legend | label line、selection offset |
| `RingChart` | donut inner radius、中心文本 | progress ring composition |
| `Sparkline` | tiny line/area，无 axis，适合卡片嵌入 | threshold color、last-point marker |

## Theme / Token

优先复用现有语义色：

1. primary
2. info
3. success
4. warning
5. danger
6. neutral.text_3 / neutral.border / neutral.divider 用于 axis/grid

若视觉不足，再在 `aura-theme` 中新增 `ChartPalette`，包含 `series_1..series_8`、`axis`、`grid`、`tooltip_bg` 等 token。

## Gallery / Docs 要求

每个图表控件都必须同时完成：

- `apps/aura-gallery/src/demos/<chart>_demo.rs`
- Gallery registry 注册
- `apps/aura-docs/content/pages/<chart>.md`
- `apps/aura-docs/content/snippets/<chart>/*.rs`
- Docs 页面按“效果 → 对应代码”组织

## 测试要求

必须覆盖：

- scale 计算：linear/band/point domain-range 映射、空/单点/负值边界。
- path 数据生成：line/area/bar/pie 的核心点位或角度计算。
- API builder：show_grid/show_legend/height/y_format/id 等状态被正确记录。
- 空数据和异常值：NaN/Infinity 应过滤或降级，不得 panic。
- Gallery/Docs 注册完整性。

## 推荐执行顺序

1. 研究本地 GPUI `canvas` / `PathBuilder` / `paint_path` 示例，写最小 chart primitive spike。
2. 实现 `chart_scale.rs` + tests。
3. 实现 `chart_axis.rs` + grid/label 布局数据计算 tests。
4. 实现 `LineChart` MVP + demo/docs。
5. 抽象共享 shape 后实现 `AreaChart`、`BarChart`。
6. 实现极坐标基础后实现 `PieChart`、`RingChart`。
7. 实现 `Sparkline` 并集成 Statistic/Card 示例。
8. 性能审查：大 series 采样/降采样策略、缓存、hover hit test 边界。

## 完成标准

- [ ] 首批 6 类图表组件完成并导出。
- [ ] Gallery demos 完整且自举。
- [ ] Docs 每个控件按“效果 → 代码”展示。
- [ ] `cargo fmt`、`cargo check -p aura-components`、`cargo check -p aura-docs`、`cargo check -p aura-gallery`、`cargo test --workspace` 通过。
- [ ] `timeout 8s cargo run -p aura-docs`、`timeout 8s cargo run -p aura-gallery` 可启动无即时崩溃。
- [ ] `.memory/*` 与 `architecture-design.md` 更新。


## 2026-06-16 Performance maintenance update

- 首批 6 类图表组件已实现：`LineChart`、`AreaChart`、`BarChart`、`PieChart`、`RingChart`、`Sparkline`。
- 已完成第一轮大数据性能增强：`LineChart`、`AreaChart`、`Sparkline` 支持共享 min/max bucket 降采样，通过 `max_render_points(...)` 限制绘制点数，并可用 `disable_downsampling()` 关闭。
- 已完成第二轮大数据性能增强：`LineChart`/`AreaChart` 的 x 轴改为 index-only scale，轴标签通过默认 `max_axis_labels` 稀疏绘制；value label 通过默认 `max_value_labels` 限流。
- 已完成第三轮大数据性能修正：核心采样新增 `downsample_index_range`/`downsample_indexed_values`，LineChart/AreaChart/Sparkline 不再先构建全量 `(index,value)`/GPUI Point 中间 Vec 再采样；demo/snippet 不再靠显式稀疏标签参数掩盖卡顿。
- 降采样策略保留首尾点和局部峰谷，避免长序列在 GPUI native path 中产生过量绘制，同时不隐藏监控尖峰。
- 已完成 Cartesian hover tooltip / hit testing：`LineChart` 与 `AreaChart` Overlay 模式支持原生最近点 tooltip，底层提供可测试的 `nearest_cartesian_hit_point`。
- 已完成 BarChart 矩形 hover tooltip / hit testing：Grouped 命中单根柱，Stacked 命中具体堆叠分段。
- 剩余维护项：Pie/Ring 极坐标扇区 hit testing、进一步缓存策略。


## 2026-06-17 Cartesian tooltip maintenance

- Completed LineChart and AreaChart Overlay nearest-point hover tooltip support using shared pure hit-testing helpers.
- Completed BarChart rectangular hover hit testing for grouped bars and stacked bar segments.
- Remaining tooltip polish: Pie/Ring polar sector hit testing and any further cache policy.
