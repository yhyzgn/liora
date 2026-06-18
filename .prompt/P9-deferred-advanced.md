# P9 Deferred Advanced — 延后高级组件补全

> 上游: `.prompt/P8-engineering.md`
> 来源: P5 Advanced 中用户明确要求跳过/延后的组件；本阶段是最新阶段，作为后续需要时补充的 backlog。

## 目标

在 P5 当前请求范围完成后，集中补全被跳过或延后的高级组件。此阶段暂不自动执行；只有用户明确要求回到这些组件时才启动。

## 启动条件

- 用户明确要求实现本阶段任一组件；或
- P6/P7/P8 工程化、Demo 自包含、文档/API 梳理后，需要补齐剩余高级组件；或
- 产品需求重新确认这些组件进入当前交付范围。

## 组件清单

| # | Component | 中文名 | 来源/状态 | 后续补充方向 |
|---|-----------|--------|-----------|--------------|
| 1 | Carousel | 走马灯 | P5 用户要求跳过，已识别 | 自动播放、手动切换、指示器、方向、暂停/恢复、Demo |
| 2 | Calendar | 日历 | P5 deferred | 月视图、日期单元格、事件/日程标记、范围/禁用日期、Demo |
| 3 | TreeSelect | 树形选择 | P5 deferred | 弹出树、单选/多选、搜索、禁用节点、默认值、Demo |
| 4 | InputTag | 标签输入 | P5 deferred | 输入生成标签、删除、限制数量、重复校验、禁用、Demo |
| 5 | Mention | @提及 | P5 deferred | 触发符、建议列表、键盘选择、插入文本、Demo |
| 6 | Watermark | 水印 | P5 deferred | 文字/图片水印、密度、旋转、透明度、覆盖区域、Demo |
| 7 | Tour | 漫游引导 | P5 deferred | 步骤、目标定位、高亮遮罩、下一步/上一步、关闭、Demo |
| 8 | VirtualizedTable | 虚拟表格 | P5 deferred | 大数据行虚拟化、固定表头、滚动性能、与 Table API 对齐 |
| 9 | VirtualizedTree | 虚拟树 | P5 deferred | 大树虚拟化、展开/折叠、滚动性能、与 Tree API 对齐 |

## 实施规则

1. **不要把 P9 当作当前自动执行阶段。** 它是最新的 deferred backlog，等待后续用户明确要求。
2. 每个组件仍遵守 Liora 组件流程：
   - `crates/liora-components/src/<name>.rs`
   - `crates/liora-components/src/lib.rs` 注册 `pub mod` / `pub use`
   - `apps/liora-gallery/src/demos/<name>_demo.rs`
   - `apps/liora-gallery/src/demos/mod.rs` 注册 DemoEntry
   - 测试覆盖核心状态/过滤/边界逻辑
3. 优先复用现有组件能力：Input、Popover/Portal、Tree、Table、Scrollbar、Button、Icon。
4. 对复杂组件先做最小可用版本，再扩展高级 API；避免一次性引入不可验证的大实现。
5. 每完成一个 P9 组件，更新：
   - `.memory/inventory.md`
   - `.memory/state.md`
   - `.memory/sessions.md`
   - 如有设计取舍，更新 `.memory/decisions.md`

## 当前状态

- P9 created on 2026-05-10.
- 所有条目均为 deferred / identified for later。
- P5 requested subset 已完成；P9 等待后续补充。
