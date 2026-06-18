# Liora 组件性能审查清单

日期：2026-05-10
范围：`crates/liora-components/src` 下所有组件实现，以及 Gallery 中与全局渲染相关的入口。
性质：只读分析记录；本文件只总结潜在算法/数据结构风险，不代表已经修改实现。
约束：任何后续优化都必须保持控件现有 API、功能和视觉效果不变。

## 总体结论

| 优先级 | 结论 | 置信度 | 依据 |
|---|---|---:|---|
| 1 | 大多数基础控件是常量级或小规模线性渲染，当前性能风险较低。 | 高 | 多数控件只渲染少量静态 children 或简单状态。 |
| 2 | 主要潜在热点集中在 `Table`、`Transfer`、`Input/Textarea`、`Image/Preview`、`Select`、`Autocomplete`、`Cascader`、`Tree`。 | 高 | 这些控件存在全量渲染、重复查找、重复 clone、重复 lowercase、无界缓存或长文本布局成本。 |
| 3 | 可优先采用内部索引、缓存、虚拟化、容量限制等方式优化，不需要改变外部 API 或视觉效果。 | 高 | 当前问题多发生在内部数据结构选择和 render-time 计算策略。 |

## 控件清单

| 控件 | 风险 | 结论 | 可选优化方向，不影响功能/效果 |
|---|---:|---|---|
| Affix | 低 | 布局/滚动位置类控件，当前实现可接受。 | 保持。 |
| Alert | 低 | 静态展示为主。 | 保持。 |
| Anchor | 中低 | 使用 `HashMap` 记录 target bounds，结构合理。 | 若锚点很多，可增量更新 bounds。 |
| Autocomplete | 中 | 每次匹配会对 item/value/label 做 `to_lowercase` 并 clone 结果。 | 预存 lowercase 字段或缓存 query 结果；长列表可虚拟化。 |
| Avatar | 低 | 简单图片/文本展示。 | 保持。 |
| Backtop | 低 | 状态和点击逻辑简单。 | 保持。 |
| Badge | 低 | 常量级渲染。 | 保持。 |
| Breadcrumb | 低 | 小规模线性渲染。 | 保持。 |
| Button | 低 | 状态/样式较多但数据规模固定。 | 保持。 |
| ButtonGroup | 低 | 小列表组合。 | 保持。 |
| Card | 低 | 容器型控件。 | 保持。 |
| Cascader | 中 | active columns 会 clone 子树列；搜索递归遍历叶子。 | 大选项树可引入路径索引、按需借用/`Arc`、搜索结果缓存。 |
| Checkbox | 低 | 单控件状态简单。 | 保持。 |
| CheckboxGroup | 中低 | `Vec<usize>` selected 使用 `contains/sort/retain`。 | 选项很多时可改 `BTreeSet`/`HashSet`，输出仍保持 Vec 顺序。 |
| Col | 低 | 布局容器。 | 保持。 |
| Collapse | 低 | `HashSet` 管 active names，合理。 | 保持。 |
| ColorPicker | 低 | 代码多但色板/计算规模固定。 | 保持；如频繁打开可缓存固定 palette。 |
| Container | 低 | 布局容器。 | 保持。 |
| DatePicker | 低 | 日历固定 42 cells，年份面板固定 12 年。 | 保持。 |
| DateTimePicker | 低中 | 日期+时间组合，规模固定。 | 可复用 DatePicker/TimePicker 的固定数组缓存。 |
| Descriptions | 低 | 行列拆分线性，通常字段少。 | 保持。 |
| Dialog | 低 | Portal/Modal 控制，单实例为主。 | 保持。 |
| Divider | 低 | 常量级。 | 保持。 |
| Drawer | 低 | Portal/Modal 控制，单实例为主。 | 保持。 |
| Dropdown | 低中 | item 列表全量渲染。 | item 很多时可虚拟化；普通菜单保持。 |
| Empty | 低 | 静态展示。 | 保持。 |
| Form | 低 | 容器型。 | 保持。 |
| Image | 中 | 远程图和圆形裁剪有全局缓存，但无容量/过期策略；本地图每次读取。 | 加 LRU/容量上限；本地图按 path 缓存；保持渲染效果。 |
| Input | 中 | prepaint 中多次 split/nth、每帧 shape line；长文本会敏感。 | 缓存行切分、byte offset、shape layout；文本未变时复用。 |
| InputNumber | 低 | 基于 Input，数字过滤规模小。 | 跟随 Input 优化即可。 |
| Link | 低 | 点击启动系统 open；渲染简单。 | 保持。 |
| Loading | 低 | 静态/动画展示。 | 保持。 |
| Menu | 中低 | popover 子项每次 render collect/clone；树规模大时有成本。 | 缓存扁平子项或借用渲染，保持交互一致。 |
| Message | 低中 | 每条消息一个 timer，过期 `retain` O(n)。 | 大量消息时可用队列/小顶堆；普通 toast 场景可接受。 |
| MessageBox | 低 | Dialog 包装为主。 | 保持。 |
| Notification | 低中 | 与 Message 类似，timer + retain。 | 大量通知时可队列化。 |
| PageHeader | 低 | 静态布局。 | 保持。 |
| Pagination | 低 | pager 数量固定最多约 7 个，算法较好。 | 保持。 |
| Paragraph | 低 | 文本组合。 | 保持。 |
| Popconfirm | 低 | Popover + 按钮组合。 | 保持。 |
| Popover | 低 | Portal 定位，单浮层为主。 | 保持。 |
| Preview | 低中 | 复用 Image 加载/全局预览状态。 | 受 Image 缓存策略影响。 |
| Progress | 低 | 渐变按颜色数生成 N-1 段；通常颜色数很少。 | 保持；极多颜色可限制/合并但一般无需。 |
| Radio | 低 | 单控件状态简单。 | 保持。 |
| RadioGroup | 低 | selected index，线性渲染。 | 保持。 |
| Rate | 低 | 固定星级数量。 | 保持。 |
| Result | 低 | 静态展示。 | 保持。 |
| Row | 低 | 布局容器。 | 保持。 |
| Scrollbar | 低 | 原生滚动容器包装。 | 保持。 |
| Segmented | 低 | 小选项列表。 | 保持。 |
| Select | 中 | 下拉打开时全量 clone/render options。 | 长 options 可虚拟化；减少 `options.clone()`。 |
| Skeleton | 低 | 固定 rows 占位。 | 保持。 |
| Slider | 低 | 常量级交互计算。 | 保持。 |
| Space | 低 | 布局容器。 | 保持。 |
| Splitter | 低 | 简单布局。 | 保持。 |
| Statistic | 低 | 静态数值展示。 | 保持。 |
| Steps | 低 | 小列表线性渲染。 | 保持。 |
| Switch | 低 | 单状态控件。 | 保持。 |
| Table | 中 | 行列全量渲染；每个 cell 用 `position + remove` 查找，接近 O(rows * cols * cells)。 | 行内 cell 预索引为 `HashMap`/`BTreeMap`；大表引入虚拟滚动。 |
| Tabs | 低中 | panes 全量渲染 header；通常数量小。 | tab 很多时可延迟/虚拟 header。 |
| Tag | 低 | 单控件/小列表。 | 保持。 |
| Text | 低 | 文本样式包装。 | 保持。 |
| Textarea | 中 | 基于 Input，长文本风险同 Input。 | 跟随 Input 行布局缓存优化。 |
| TimePicker | 低 | hour/min/sec 列表规模固定 24/60。 | 可静态缓存 stepped values，但收益小。 |
| Timeline | 低 | 小列表线性渲染。 | 保持。 |
| Title | 低 | 文本样式包装。 | 保持。 |
| Tooltip | 低 | 当前 passive overlay 合理。 | 保持。 |
| Transfer | 中 | 多处 `Vec::contains/find` 形成 O(n*m)，过滤重复 lowercase。 | target/checked 改 `HashSet` 索引，item 按 key 建 map；输出顺序保持 Vec。 |
| Tree | 低中 | `HashSet` 管 expanded/selected，递归渲染全树。 | 大树可懒渲染/虚拟化。 |
| Upload | 低中 | 文件列表 remove 用 position；批量校验顺序执行。 | 大量文件可 id map；普通上传列表可接受。 |

## 重点问题拆解

### 1. Table：行内 cell 查找成本偏高

- 证据：`crates/liora-components/src/table.rs` 中 `TableRow::take_cell` 使用 `position` 查找 key 后 `remove`；渲染时对每一列调用一次。
- 影响：小表无明显问题；大表或列很多时，复杂度接近 `rows * cols * cells`。
- 可选方案：渲染前把每行 cells 转成 key-index map，或在 `TableRow` 内部保存有序 map。输出顺序仍按 columns 渲染，视觉不变。

### 2. Transfer：多处 Vec membership 导致放大成本

- 证据：`move_to_target`、`move_to_source`、`source_items`、`target_items`、`toggle_key` 等路径依赖 `Vec::contains`、`iter().find`、`retain`。
- 影响：少量穿梭项可接受；大量 items/target_keys/checked_keys 时容易形成 O(n*m)。
- 可选方案：内部维护 `HashSet<SharedString>` 用于 membership，另保留 Vec 维护展示顺序和对外回调顺序。

### 3. Input/Textarea：长文本布局和行处理成本较高

- 证据：prepaint 阶段会 split lines、shape line，并多次按行计算 offset；长文本下每帧重算成本明显。
- 影响：普通单行输入无问题；Textarea 或长文本编辑会敏感。
- 可选方案：缓存行切分、原始 byte offset、display offset、shaped line；仅在 value/style/width 变化时失效。

### 4. Image/Preview：图片缓存无容量策略

- 证据：远程图片和圆形裁剪使用全局 `HashMap` 缓存，未见容量上限或过期策略；本地图读取走同步 `std::fs::read`。
- 影响：图片来源较多时可能长期占内存；本地图频繁渲染可能重复读磁盘。
- 可选方案：远程/裁剪缓存加 LRU 或容量上限；本地图按 path+mtime 缓存；保持当前 `RenderImage` 输出效果。

### 5. Select / Autocomplete / Cascader：大列表场景缺少索引或虚拟化

- 证据：Select 打开时 clone options 并全量渲染；Autocomplete 每次过滤都 lowercase；Cascader 搜索递归遍历叶子且 active columns clone。
- 影响：小数据量正常；上百/上千选项会有明显 render/filter 成本。
- 可选方案：预处理 lowercase、缓存搜索结果、使用索引结构；大列表/大树引入虚拟列表或懒渲染。

## 建议的后续优化顺序

| 顺序 | 目标 | 原因 | 风险控制 |
|---:|---|---|---|
| 1 | `Table` cell 查找索引化 | 复杂度收益明确，功能边界清晰。 | 加表格渲染顺序/缺失 cell 回归测试。 |
| 2 | `Transfer` membership 索引化 | 数据结构收益明确，可保持 Vec 输出顺序。 | 加移动、过滤、checked 保持顺序测试。 |
| 3 | `Input/Textarea` 行布局缓存 | 对长文本收益最大，但改动更敏感。 | 先补多行、密码、中文、选择区测试。 |
| 4 | `Image/Preview` 缓存容量策略 | 防止长期内存增长。 | 不改变加载成功/失败/预览效果。 |
| 5 | `Select/Autocomplete/Cascader/Tree` 大列表优化 | 适合后续按真实数据量逐步做。 | 先保留现有 API，内部替换过滤/渲染策略。 |

## 当前无需优先处理的部分

- `Pagination`：页码渲染数量固定，算法已经是常量级。
- `DatePicker` / `TimePicker`：日历和时间候选规模固定，当前实现可接受。
- 基础展示类控件：如 `Alert`、`Badge`、`Card`、`Empty`、`Result`、`Statistic` 等，主要是静态布局，性能风险低。

## 分析边界

- 本文档基于源码静态审查，没有运行 profiler 或真实大数据量压测。
- 风险等级表示“在数据量变大时的潜在成本”，不表示当前 demo 一定存在可感知性能问题。
- 后续如要实施优化，应先补回归测试，再做内部实现替换，确保控件功能和视觉效果不变。
