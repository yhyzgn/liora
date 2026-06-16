# P14 Deferred Advanced — Advanced Component Backlog Completion

> 上游: `.prompt/P9-deferred-advanced.md` / `.prompt/P13-component-expansion.md`
> 状态: In Progress
> 目标: 将 P9 deferred backlog 正式转为可交付阶段，逐步补齐 Carousel、Calendar、TreeSelect、InputTag、Mention、Watermark、Tour、VirtualizedTable、VirtualizedTree。

## 背景

P9 原本是 P5 跳过的高级组件暂存 backlog。用户要求在 P13/P12 完成后自动进入下一个 P，因此本阶段将 P9 的 deferred 条目迁移为 P14 执行阶段，避免继续停留在“等待明确要求”的状态。

## 执行原则

- 继续保持 **纯 Rust + GPUI native**；禁止 WebView/HTML/CSS/DOM/browser runtime。
- 每个组件都必须包含：`aura-components` API、Gallery demo、Docs 页面、compile-checked snippets、关键测试。
- 优先复用现有组件：Input、TagFlow、Tree、Table、VirtualizedList、Popover/Portal、Button、Icon。
- 复杂交互先交付真实可用 MVP，再扩展高级配置；不能提交空壳/占位组件。

## 组件清单

| # | Component | 状态 | 交付要求 |
|---|-----------|------|----------|
| 1 | Carousel | ✅ Wave 1 | 轮播项、方向、指示器位置、箭头、自动播放配置、自定义内容 slot |
| 2 | Calendar | ✅ Wave 1 | 月视图、选中日期、范围、禁用日期、事件标记、选择回调 |
| 3 | InputTag | ✅ Wave 1 | 输入生成标签、删除、限制数量、重复策略、TagFlow 展示 |
| 4 | TreeSelect | ⬜ Pending | 弹出树选择、单选/多选、搜索、禁用节点、默认值 |
| 5 | Mention | ⬜ Pending | 触发符、建议列表、键盘选择、插入文本 |
| 6 | Watermark | ⬜ Pending | 文字/图片水印、密度、旋转、透明度、覆盖区域 |
| 7 | Tour | ⬜ Pending | 步骤、高亮遮罩、下一步/上一步、关闭 |
| 8 | VirtualizedTable | ⬜ Pending | 大数据行虚拟化、固定表头、Table API 对齐 |
| 9 | VirtualizedTree | ⬜ Pending | 大树虚拟化、展开/折叠、滚动性能、Tree API 对齐 |

## Wave 1 — 2026-06-16

已实现：

- `Carousel`: `CarouselItem` + `Carousel` builder API，支持方向、指示器位置、箭头开关、autoplay/interval/pause 配置、自定义内容 slot。
- `Calendar`: `CalendarDate`、`CalendarEvent`、月视图 42 单元格、选中/范围/禁用/事件标记、选择回调。
- `InputTag`: `Input` + `TagFlow` 组合控件，支持回车添加、删除、最大数量、重复项策略、on_change 回调。
- Gallery: 新增 `carousel_demo.rs`、`calendar_demo.rs`、`input_tag_demo.rs` 并注册。
- Docs: 新增 `carousel.md`、`calendar.md`、`input_tag.md` 与 snippets，并接入 `check_snippets`。

## 下一步

优先级建议：

1. `TreeSelect`：复用 `TreeNode` 数据模型与现有 Select/Popover 行为，是表单类高级控件最高价值项。
2. `Mention`：复用 Input/Textarea 输入能力与 Autocomplete 建议列表。
3. `Watermark`：偏展示层，风险较低，可快速补齐。
4. `Tour`：需要 overlay/position/highlight，依赖 Popover/Portal。
5. `VirtualizedTable` / `VirtualizedTree`：性能型组件，最后集中做。

## 验收命令

```bash
cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets
cargo test -p aura-components carousel && cargo test -p aura-components calendar && cargo test -p aura-components input_tag
cargo test -p aura-gallery registry_entries_are_sorted_with_charts_grouped_last
cargo run -p aura-gallery
cargo run -p aura-docs
```
