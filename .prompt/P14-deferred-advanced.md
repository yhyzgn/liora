# P14 Deferred Advanced — Advanced Component Backlog Completion

> 上游: `.prompt/P9-deferred-advanced.md` / `.prompt/P13-component-expansion.md`
> 状态: Complete
> 目标: 将 P9 deferred backlog 正式转为可交付阶段，逐步补齐 Carousel、Calendar、TreeSelect、InputTag、Mention、Watermark、Tour、VirtualizedTable、VirtualizedTree。

## 背景

P9 原本是 P5 跳过的高级组件暂存 backlog。用户要求在 P13/P12 完成后自动进入下一个 P，因此本阶段将 P9 的 deferred 条目迁移为 P14 执行阶段，避免继续停留在“等待明确要求”的状态。

## 执行原则

- 继续保持 **纯 Rust + GPUI native**；禁止 WebView/HTML/CSS/DOM/browser runtime。
- 每个组件都必须包含：`liora-components` API、Gallery demo、Docs 页面、compile-checked snippets、关键测试。
- 优先复用现有组件：Input、TagFlow、Tree、Table、VirtualizedList、Popover/Portal、Button、Icon。
- 复杂交互先交付真实可用 MVP，再扩展高级配置；不能提交空壳/占位组件。

## 组件清单

| # | Component | 状态 | 交付要求 |
|---|-----------|------|----------|
| 1 | Carousel | ✅ Wave 1 | 轮播项、方向、指示器位置、箭头、自动播放配置、自定义内容 slot |
| 2 | Calendar | ✅ Wave 1 | 月视图、选中日期、范围、禁用日期、事件标记、选择回调 |
| 3 | InputTag | ✅ Wave 1 | 输入生成标签、删除、限制数量、重复策略、TagFlow 展示 |
| 4 | TreeSelect | ✅ Wave 3 | 树形选择、单选/多选、过滤、禁用节点、默认值 |
| 5 | Mention | ✅ Wave 2 | 触发符、建议列表、过滤、选择回调、禁用状态 |
| 6 | Watermark | ✅ Wave 2 | 文字水印、密度、间距、透明度、颜色、页眉/页脚局部位置 |
| 7 | Tour | ✅ Wave 4 | 步骤、target 描述、placement、进度、下一步/上一步、关闭/完成回调 |
| 8 | VirtualizedTable | ✅ Wave 5 | 大数据行虚拟化、固定表头、TableColumn API 对齐、排序状态回调 |
| 9 | VirtualizedTree | ✅ Wave 6 | 大树虚拟化、展开/折叠、单选/多选、checkbox、滚动性能 |

## Wave 1 — 2026-06-16

已实现：

- `Carousel`: `CarouselItem` + `Carousel` builder API，支持方向、指示器位置、箭头开关、autoplay/interval/pause 配置、自定义内容 slot。
- `Calendar`: `CalendarDate`、`CalendarEvent`、月视图 42 单元格、选中/范围/禁用/事件标记、选择回调。
- `InputTag`: `Input` + `TagFlow` 组合控件，支持回车添加、删除、最大数量、重复项策略、on_change 回调。
- Gallery: 新增 `carousel_demo.rs`、`calendar_demo.rs`、`input_tag_demo.rs` 并注册。
- Docs: 新增 `carousel.md`、`calendar.md`、`input_tag.md` 与 snippets，并接入 `check_snippets`。

## 后续维护

P14 已完成。后续只在出现新的用户需求或缺陷报告时维护这些控件；不要把 P9 backlog 重新视为未完成。

## 验收命令

```bash
cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets
cargo test -p liora-components carousel && cargo test -p liora-components calendar && cargo test -p liora-components input_tag && cargo test -p liora-components mention && cargo test -p liora-components watermark && cargo test -p liora-components tree_select && cargo test -p liora-components tour
cargo test -p liora-gallery registry_entries_are_sorted_with_charts_grouped_last
cargo run -p liora-gallery
cargo run -p liora-docs
```

## Wave 2 — 2026-06-16

已实现：

- `Mention`: 复用 `Input` 作为输入内核，支持触发符、候选过滤、最大候选数量、禁用状态和选择回调。
- `Watermark`: 支持覆盖/页眉/页脚位置、文字内容、密度、间距、透明度、颜色和旋转配置记录。
- Gallery: 新增 `mention_demo.rs`、`watermark_demo.rs` 并注册。
- Docs: 新增 `mention.md`、`watermark.md` 与 snippets，并接入 `check_snippets`。

剩余 P14：TreeSelect、Tour、VirtualizedTable、VirtualizedTree。

## Wave 3 — 2026-06-16

已实现：

- `TreeSelect`: 新增树形选择控件，支持单选、多选、默认选中、禁用节点、filterable 搜索过滤、选择回调。
- Gallery: 新增 `tree_select_demo.rs` 并注册。
- Docs: 新增 `tree_select.md` 与 single/multiple/filterable snippets，并接入 `check_snippets`。

剩余 P14：Tour、VirtualizedTable、VirtualizedTree。

## Wave 4 — 2026-06-17

已实现：

- `Tour`: 新增受控步骤引导组件，支持 step list、active_index、target 描述、placement、mask/progress 开关、上一页/下一页/完成/关闭回调。
- Gallery: 新增 `tour_demo.rs` 并注册。
- Docs: 新增 `tour.md` 与 basic/middle/no_mask snippets，并接入 `check_snippets`。

剩余 P14：无。


## Wave 5 — 2026-06-17

已实现：

- `VirtualizedTable`: 新增大数据虚拟表格，支持 `TableColumn` 列定义、固定表头、`ListState` 可见区行渲染、Liora `VirtualScrollbar`、高度/行高/overdraw、斑马纹/边框/加载/空状态、三态排序回调。
- Gallery: 新增 `virtualized_table_demo.rs` 并注册基础万行表格与排序状态用例。
- Docs: 新增 `virtualized_table.md` 与 basic/sortable snippets，并接入 `check_snippets`。

剩余 P14：无。


## Wave 6 — 2026-06-17

已实现：

- `VirtualizedTree`: 新增大型树虚拟化控件，维护原始 `TreeNode`、展开 key、选择 key 和轻量可见节点元数据，使用 `ListState` 只渲染可见行，支持默认展开/选中、展开/折叠、单选/多选、checkbox 风格、on_node_click 回调、高度/行高/缩进/overdraw 配置和 Liora 滚动条。
- Gallery: 新增 `virtualized_tree_demo.rs` 并注册大型组织树与多选回调用例。
- Docs: 新增 `virtualized_tree.md` 与 basic/checkable snippets，并接入 `check_snippets`。

P14 deferred advanced backlog 已全部完成：Carousel、Calendar、InputTag、Mention、Watermark、TreeSelect、Tour、VirtualizedTable、VirtualizedTree。
