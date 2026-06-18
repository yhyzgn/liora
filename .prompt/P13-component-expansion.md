# P13 Component Expansion — Advanced Widgets & Customization

> 上游: `.prompt/P10-charts.md` / `.prompt/P12-packaging.md`
> 状态: Implemented / 收尾维护
> 目标: 补齐用户提出的新一批业务控件与既有控件增强，形成 Dashboard、低代码配置面板、数据监控、操作面板、内容编辑等场景的完整组件能力。

## 目标

P13 已完成主体实现，聚焦两类工作：

1. **新增控件**：只为当前组件库中不存在的能力新增文件/API，例如二维码、代码编辑器、信号图、热力图、分段比例条、水平可拖动列表、计时器、Label、Operation 等；“独立柱状图”按用户截图理解为 BarChart 的无坐标迷你柱样式，不新增平行控件。
2. **既有控件增强**：凡是已经存在的控件，必须直接在原组件、原 Demo、原 Docs 页面上增强；例如 RingChart、LineChart、BarChart、Progress/RingProgress、Button、Tag、Radio、Checkbox、Vertical/List 类控件，禁止为了单个增强点另建平行新控件。

所有组件必须保持 **Rust + GPUI 原生渲染**，遵守 Liora 组件 API 范式，不能引入 WebView/HTML/CSS/DOM/browser runtime。

## 绝对边界

- 禁止引入 Web 编辑器运行时（Monaco/CodeMirror WebView 等）。
- 禁止用网页二维码、SVG DOM、Canvas DOM 方案替代 GPUI 原生节点。
- QR 生成/识别可使用纯 Rust 算法库；识别需要作为可选能力或清晰隔离依赖，避免把图像解码链污染到基础控件路径。
- 代码编辑器第一阶段允许采用 `syntect` 高亮；语法检查必须设计成可插拔 diagnostics provider，不在 P13 MVP 中硬绑定 LSP。
- 图表/进度增强必须复用 P10 chart primitive，避免每个控件重复绘制基础设施。
- 交互组件拖动必须明确数据模型、拖动状态、drop 回调、无障碍 fallback，不能只做视觉移动。
- **已有控件增强优先原则**：如果能力属于已有控件（如 `Tag` flow、`Progress` 环形渐变、`LineChart` 线型、`BarChart` 区间色/独立迷你柱样式、`RingChart` 外置文本、`Button` 自定义颜色、`Radio`/`Checkbox` option 自定义），必须修改已有组件文件、已有 demo 和已有 docs/snippets；不得新增 `TagFlow`、`RingProgress2`、`AdvancedButton`、`FlatBarMeter` 等替代控件。

## 组件清单与需求拆解

| # | Component / Enhancement | 类型 | 核心需求 | 优先级 |
|---|---|---|---|---|
| 1 | `QrCode` | 新增 | 二维码生成、识别、尺寸/纠错等级/颜色/Logo/复制/导出 | P0 |
| 2 | `CodeEditor` | 新增 | 行号、缩进、高亮、选择/复制、编辑、diagnostics 扩展点 | P0 |
| 3 | `SignalMeter` | 新增 | 手机信号/WiFi 风格、等级、每级颜色、禁用/空状态 | P0 |
| 4 | `HeatBar` / `HeatmapBar` | 新增 | 按截图实现时间轴柱状热力图：细圆角竖柱、按 severity/category 或 value range 渐变映射颜色、顶部 legend 汇总、可选 y 轴刻度/时间 x 轴 label、tooltip | P0 |
| 5 | `BarChart` standalone mini mode | 增强 | 按截图实现无坐标/无网格/无 legend 的独立迷你柱状样式：窄圆角竖柱、淡入/渐变配色、紧凑高度、可嵌入卡片；直接扩展现有 `BarChart`，不新增 `FlatBarMeter` | P0 |
| 6 | `SegmentRatioBar` | 新增 | 按截图实现一条横向分段比例条 + 可配置位置的 legend/value 文本行：文本可在上方、下方、上下同时或隐藏；每段颜色、圆点、label、比例值 pattern 可自定义，支持 label 与比例值在每个 legend item 两端对齐 | P0 |
| 7 | `HorizontalList` | 新增 | 横向滚动、item 完全自定义、divider 自定义、item 拖动 | P1 |
| 8 | Vertical list drag | 增强 | 既有列表/VirtualizedList 增加垂直 item 拖动 | P1 |
| 9 | `RingChart` external labels | 增强 | 图例 + 比例值完全外置，垂直/水平排列，不需要折线引导 | P1 |
| 10 | `LineChart` line style | 增强 | 每条线自定义虚线/实线/点线、颜色、粗细、点样式 | P1 |
| 11 | `BarChart` range colors | 增强 | 按指标值区间映射颜色，支持默认和 per-series override | P1 |
| 12 | `RingProgress` gradient | 增强 | 进度色渐变、完成色自定义、默认取渐变末色 | P1 |
| 13 | `Timer` | 新增 | 时间单位、正计时/倒计时、开始/暂停/重置、按单位获取结果 | P1 |
| 14 | `Button` gradient/custom color | 增强 | 渐变色、完全自定义颜色、自动推导 hover/pressed/disabled | P1 |
| 15 | `Tag` flow layout | 增强 | 标签流式布局、自动换行、gap、max rows/collapse | P2 |
| 16 | `Label` | 新增 | Icon + Text，间距、位置、尺寸、颜色可配 | P2 |
| 17 | `Operation` | 新增 | 左侧 Label + 右侧操作区域，两端对齐，操作区自定义 | P2 |
| 18 | `Radio` / `Checkbox` option customization | 增强 | option 布局/样式完全自定义，选中态布局/样式可自定义 | P2 |

## 推荐实施分批

### Wave 1 — 基础视觉与轻交互控件

目标：先交付可快速复用、风险低的新增控件，并把已有 `Tag` 的 flow 能力合并进原 Tag 组件文档/Demo。

- `SignalMeter`
- `BarChart` standalone mini mode（按截图：无坐标紧凑圆角竖柱，直接增强现有 BarChart）
- `HeatBar`
- `SegmentRatioBar`
- `Label`
- `Operation`
- `Tag` flow layout

验收：

- 新增控件导出 API、Gallery demo、Docs 页面、snippet；已有控件增强直接补充原 demo/docs/snippet。
- 样式参数覆盖颜色、尺寸、间距、label pattern。
- `cargo check -p liora-components -p liora-gallery -p liora-docs` 通过。

### Wave 2 — 图表与进度增强

目标：复用 P10/P4 图表进度能力，补齐用户明确指出的高度自定义。

- `RingChart` 外置 legend/value pattern
- `LineChart` per-series stroke style
- `BarChart` standalone mini mode + value range color rules
- `RingProgress` gradient + completion color

验收：

- 共享 chart style model，不在各图表中重复定义不兼容配置。
- Docs/Demo 在原控件页面展示每个新增配置项，不另起平行控件页。
- 单元测试覆盖 style resolution、range color matching、legend layout 数据结构。

### Wave 3 — 拖动列表与布局容器

目标：提供横向/纵向可拖动列表能力，服务配置项、步骤流、标签流、看板排序等场景。

- `HorizontalList`
- 既有垂直列表 / `VirtualizedList` item drag enhancement
- divider 自定义：默认垂直线段，也支持 icon/arrow/自定义 element

验收：

- 拖动排序回调提供 `from_index` / `to_index` / item key。
- 支持禁用拖动、固定项、拖动占位样式。
- 对虚拟列表避免保存跨 frame 的 `AnyElement` / `ArenaRef`，item 必须由闭包重新渲染。

### Wave 4 — QR 与 CodeEditor

目标：处理依赖和复杂交互风险最高的两个控件。

- `QrCode`：生成与识别。
- `CodeEditor`：编辑器基础能力。

验收：

- QR 生成支持纯数据结构测试；识别能力先以静态图片/bytes API 测试，不要求摄像头。
- CodeEditor 支持：行号、缩进、Tab/Shift+Tab、语法高亮、选择复制、基础 diagnostics 渲染。
- 语法检查只定义 provider trait 与 diagnostics 数据模型，不硬编码 Rust analyzer/LSP。

### Wave 5 — 表单控件深度自定义

目标：增强 Button/Radio/Checkbox 的企业级主题与低代码可配置能力。

- `Button` gradient/custom palette + derived states。
- `Radio` / `Checkbox` option render customization。
- 与 `Label` / `Operation` 组合展示复杂 option 内容。

验收：

- 自定义颜色模式下 hover/pressed/disabled 由统一 color resolver 推导。
- Radio/Checkbox 可通过 builder 设置 option render/style hooks，同时保留普通用法。
- Demo 展示默认、卡片式、图标式、左右结构、选中态替换布局。

## API 设计草案

### QrCode

```rust
QrCode::new("https://github.com/yhyzgn/liora")
    .size(px(180.0))
    .error_correction(QrErrorCorrection::Medium)
    .foreground(theme.primary.base)
    .background(theme.background)
    .logo(Icon::new(IconName::Sparkles))
    .copyable(true)
```

识别 API 建议拆为非 UI helper：

```rust
let result = QrDecoder::decode_image_bytes(bytes)?;
```

### CodeEditor

```rust
CodeEditor::new(source)
    .language(CodeLanguage::Rust)
    .line_numbers(true)
    .tab_size(4)
    .soft_tabs(true)
    .diagnostics(diagnostics)
    .on_change(|text, window, cx| { /* ... */ })
```

### SignalMeter

```rust
SignalMeter::new(3, 5)
    .kind(SignalKind::Wifi)
    .level_colors(vec![danger, warning, success])
    .bar_gap(px(3.0))
```

### BarChart standalone mini mode

```rust
BarChart::new(series)
    .standalone()
    .show_axis(false)
    .show_grid(false)
    .show_legend(false)
    .bar_radius(px(4.0))
    .bar_width(px(5.0))
    .bar_gap(px(8.0))
    .value_color_ranges(vec![
        BarValueColorRange::up_to(20.0, theme.success.soft),
        BarValueColorRange::above(20.0, theme.success.base),
    ])
```

用户截图语义：一组轻量迷你竖向圆角柱，没有横竖坐标、没有边框、没有 legend，视觉上可像信号/频谱，但本质仍是 BarChart 的一个展示模式。

### HeatBar / HeatmapBar

```rust
HeatBar::new(points)
    .x_labels(HeatAxisLabels::time())
    .y_ticks(vec![0.0, 5.0, 10.0])
    .legend(vec![
        HeatLegendItem::new("错误", 3).color(theme.danger.base),
        HeatLegendItem::new("警告", 24).color(theme.warning.base),
    ])
    .color_ranges(vec![
        HeatColorRange::new(0.0..=3.0, theme.warning.soft),
        HeatColorRange::new(3.0..=7.0, theme.warning.base),
        HeatColorRange::new(7.0..=10.0, theme.danger.base),
    ])
    .bar_width(px(4.0))
    .bar_radius(px(2.0))
```

用户截图语义：不是日历网格热力图，而是按时间分布的柱状热力图。顶部显示分类 legend 与数量汇总；主体是密集细柱，柱色按类别或数值区间从浅色到高亮色映射；可带轻量 y 轴刻度与 x 轴时间标签。

### SegmentRatioBar

```rust
SegmentRatioBar::new(vec![
    Segment::new("Direct", 42.0).color(blue).value_pattern("{percent:.0}%"),
    Segment::new("Proxy", 51.0).color(green).value_pattern("{percent:.0}%"),
    Segment::new("Reject", 7.0).color(red).value_pattern("{percent:.0}%"),
])
.bar_height(px(7.0))
.bar_radius(px(4.0))
.legend_layout(SegmentLegendLayout::Inline)
.legend_position(SegmentLegendPosition::Bottom)
.label_align(SegmentLabelAlign::SplitEnds)
```

用户截图语义：默认可表现为上方一条横向分段比例条、下方 legend/value 信息行；但文本位置必须可配置，支持 `Top`、`Bottom`、`Both`、`Hidden`。每段宽度按占比计算、颜色独立；legend/value 通常为彩色圆点 + label + 百分比。每个 legend item 内 label 与比例值需要可分开两端对齐，也要支持自定义 pattern，如 `{label}`、`{percent:.1}%`、`{value}/{total}`。


### HorizontalList

```rust
HorizontalList::new(items)
    .item(|item| item.render())
    .divider(|| Icon::new(IconName::ChevronRight))
    .draggable(true)
    .on_reorder(|from, to, cx| { /* update model */ })
```

### Timer

```rust
Timer::count_up()
    .unit(TimeUnit::Second)
    .precision(2)
    .on_tick(|elapsed, cx| {})

Timer::count_down(Duration::from_secs(300))
    .on_finish(|cx| {})
```

## 依赖调研清单

实现过程中按需完成轻量 dependency review，后续新增依赖仍需遵守：

| 能力 | 候选 | 关注点 |
|---|---|---|
| QR 生成 | `qrcode`, `fast_qr` | 纯 Rust、许可、SVG 依赖可关闭、image 输出能力 |
| QR 识别 | `rqrr`, `quircs` | 纯 Rust/FFI、image crate 兼容、识别率、维护状态 |
| 高亮 | 已有 `syntect` / `two-face` | 复用 CodeBlock 高亮资源，避免重复依赖 |
| 语法检查 | provider trait / 后续 LSP | P13 只做扩展点，不强绑定外部进程 |
| 拖拽 | GPUI mouse event + state | 不引入平台 DnD 作为 item reorder 前提 |

如需引入新依赖，必须先在 phase 执行记录中说明理由、替代方案和许可证风险。

## Demo / Docs 要求

每个新增或增强项都必须同步；其中已有控件增强必须改原文件/原页面：

- 新增控件：`crates/liora-components/src/<component>.rs`，并在 `lib.rs` 导出。
- 已有控件增强：直接修改现有文件，例如 `tag.rs`、`progress.rs`、`line_chart.rs`、`bar_chart.rs`、`button.rs`、`radio*.rs`、`checkbox*.rs`。
- 新增控件：创建 `apps/liora-gallery/src/demos/<component>_demo.rs` 并注册。
- 已有控件增强：补充现有 `<component>_demo.rs`，不新增平行 demo。
- 新增控件：创建 `apps/liora-docs/content/pages/<component>.md` 与 snippets。
- 已有控件增强：补充现有 `content/pages/<component>.md` 与原 snippets 目录。

Docs 页面继续保持：**一种效果 → 对应代码 → 下一种效果 → 对应代码**。已有控件增强必须追加到原控件页面中的对应效果段落。

## 测试要求

- Builder 状态测试：所有新增配置项都可被断言。
- 纯计算逻辑测试：比例、颜色区间、渐变 stop、timer 单位换算、drag reorder 结果。
- 边界测试：空数据、单 item、NaN/Infinity、负值、总数为 0、超出等级范围。
- 文档 snippet 必须能被 `cargo check`/现有 snippet checker 覆盖。
- 复杂交互至少用可测试的 reducer/helper 把状态转换与 GPUI event 分离。

## 完成标准

- [x] Wave 1 完成并通过 check/test。
- [x] Wave 2 完成并通过 check/test。
- [x] Wave 3 完成并通过 check/test。
- [x] Wave 4 完成并通过 check/test。
- [x] Wave 5 完成并通过 check/test。
- [x] Gallery demos 完整、布局不拥挤。
- [x] 新增控件有独立页面；已有控件增强只补充原页面章节。
- [x] `.memory/inventory.md`、`.memory/state.md`、`prompt.md` 更新。
- [x] 如新增依赖，`Cargo.lock`、许可证说明、依赖取舍记录完成。


## 2026-06-16 实现状态快照

P13 主体功能已按当前计划落地：新增控件和既有控件增强均已接入 `liora-components`、Gallery、Docs/snippets，并同步 `.memory/inventory.md`。后续如继续扩展，应按本文件的边界和测试要求作为维护规约执行，而不是重新创建平行控件。

已验证的收尾命令包括：

```bash
cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets
```
