# gpui-component → Liora 控件采集清单

> 目的：调研 `longbridge/gpui-component` 中对 Liora 有增量价值的组件/能力；该清单已经在 P22 收口，现仅作为历史研究记录。
> 原始范围：只做采集清单与优先级判断，不包含代码实现。
> 调研日期：2026-06-25

> **Closure status (2026-06-29)**: this collection list is fully processed. It is kept as historical research, not an active backlog. Items were closed by adding standalone Liora components, enhancing existing Liora controls in place, or explicitly declining boundary-violating/duplicate paths. Do not reopen this file as a task list unless the owner creates a new phase.


## 调研依据

- 上游仓库：<https://github.com/longbridge/gpui-component>
- 上游 README：<https://github.com/longbridge/gpui-component/blob/main/README.md>
- 上游组件索引：<https://github.com/longbridge/gpui-component/blob/main/docs/docs/components/index.md>
- 上游中文组件索引：<https://github.com/longbridge/gpui-component/blob/main/docs/zh-CN/docs/components/index.md>
- 上游源码树：<https://github.com/longbridge/gpui-component/tree/main/crates/ui/src>
- 本地对照：`crates/liora-components/src`、`.memory/inventory.md`

## 总体结论

`gpui-component` 对 Liora 最有增量价值的不是基础控件，而是三类能力：

1. **应用壳 / 桌面工作台能力**：`Sidebar`、`StatusBar`、`Dock Layout`、`Settings`、`Sheet`。
2. **复合输入与操作控件**：`Combobox`、`OtpInput`、`DropdownButton`、`Toggle`。
3. **内容展示与高级数据能力**：`TextView`、`DataTable` 增强、`CandlestickChart`、`SearchableList`、`CodeEditor` 高级能力。

Liora 已经覆盖了大多数基础组件，所以本清单重点筛选“增量价值”，不重复列入 Button、Dialog、Popover、Tree、Table、VirtualList、Progress、Switch 等已有同类能力，除非它们值得作为增强项吸收。

---

## 一、强烈推荐采集

| 优先级 | 控件/能力 | 上游名称 | Liora 状态 | 推荐指数 | 介绍 | 推荐原因 |
|---:|---|---|---|---:|---|---|
| P0 | Combobox | `combobox` | 仅有 `Autocomplete` / `Select` / `TreeSelect` | ⭐⭐⭐⭐⭐ | 可搜索单选/多选下拉，支持 grouped items、自定义 trigger/item/footer | 企业表单非常常见，明显补足 Select 与 Autocomplete 之间的空档 |
| P0 | Sidebar | `sidebar` | 无独立组件 | ⭐⭐⭐⭐⭐ | 桌面应用侧栏导航系统，通常含 header/group/menu/footer | Gallery/Docs 目前需要更稳定的 app shell 抽象 |
| P0 | StatusBar | `status_bar` | 无独立组件 | ⭐⭐⭐⭐⭐ | 底部状态栏，左/中/右区域布局 | 对原生桌面应用非常实用，可显示状态、连接、任务、版本、快捷提示 |
| P0 | Dock Layout | `dock` | 无 | ⭐⭐⭐⭐⭐ | 面板停靠、tab panel、split panel、tiles layout | 最大增量能力，适合 IDE/后台/复杂工具型桌面应用；但实现成本最高 |
| P1 | Settings UI | `setting` | 无系统化设置页组件 | ⭐⭐⭐⭐ | setting page/group/item/field，含 bool/string/number/dropdown 字段 | Liora 企业级组件库很需要“设置页模式组件” |
| P1 | DataTable 增强 | `table::data_table` | 有 `Table` + `VirtualizedTable` | ⭐⭐⭐⭐ | 高性能表格，支持列 resize/move/fixed、行列单元格选择、键盘导航、上下文菜单、load more | 不一定新建 DataTable，但值得增强 Liora VirtualizedTable |
| P1 | TextView / Document View | `text` / `text_view` | 有 `Paragraph`、`CodeBlock`、Docs Markdown renderer | ⭐⭐⭐⭐ | Markdown + 简单 HTML 文本渲染、选择、文档模型 | Liora Docs 已有渲染器，但可沉淀为通用 `TextView` 组件 |
| P1 | OtpInput | `input::otp_input` | 无 | ⭐⭐⭐⭐ | 一次性验证码 / PIN 输入格 | 低成本高实用，登录、2FA、设备配对常用 |
| P1 | Sheet | `sheet` | 有 `Drawer` | ⭐⭐⭐⭐ | 边缘滑入面板，常用于轻量 overlay 流程 | 与 Drawer 类似，但可以做更轻、更 app-shell 化的交互 |
| P1 | DropdownButton | `button::dropdown_button` | 有 Button/Dropdown，未组合 | ⭐⭐⭐⭐ | 左侧主按钮 + 右侧下拉触发器 | 工具栏、批处理、保存并继续等常见企业场景 |
| P1 | Accordion | `accordion` | 有 `Collapse` | ⭐⭐⭐⭐ | 多 section 折叠面板，支持单开/多开、禁用、边框、尺寸 | 可以作为 Collapse 的上层组合，文档/设置/FAQ 常用 |
| P1 | Spinner | `spinner` | 有 `Loading`，无独立 spinner | ⭐⭐⭐⭐ | 独立旋转加载图标，可嵌入按钮、列表、状态栏 | Liora 的 Loading 偏遮罩，Spinner 是细粒度状态件 |

---

## 二、建议采集为现有组件增强

| 优先级 | 能力 | 上游对应 | Liora 当前 | 推荐指数 | 建议落点 |
|---:|---|---|---|---:|---|
| P1 | CodeEditor 高性能能力增强 | `input/lsp/*`, `highlighter/*`, `Editor` | 已有 `CodeEditor`，基于 syntect，诊断 provider 扩展点 | ⭐⭐⭐⭐ | 不整体照搬；研究 rope/tree-sitter/LSP 弹层/搜索/hover/completion 思路 |
| P1 | CandlestickChart | `chart/candlestick_chart.rs` | 有 Line/Area/Bar/Pie/Ring/Sparkline，无 K 线 | ⭐⭐⭐⭐ | 金融/监控类 Dashboard 很有价值，Liora 可新增原生 `CandlestickChart` |
| P1 | SearchableList | `searchable_list` | 无通用 searchable list 抽象 | ⭐⭐⭐⭐ | 可作为 Combobox、Select、Command Palette 的共享基础设施 |
| P1 | Native Menu | `native_menu` | Liora 有 `liora-tray` 菜单，组件层无 native app menu | ⭐⭐⭐ | 可做 app menu facade，但需注意平台差异 |
| P1 | FocusTrap | `focus_trap` | Dialog/Popover/Tour 有各自行为 | ⭐⭐⭐ | 抽成底层交互基础设施，增强 overlay 可访问性 |
| P1 | WindowExt / TitleBar / WindowBorder | `window_ext`, `title_bar`, `window_border` | 有 `window_frame` | ⭐⭐⭐ | 不新建一堆平行控件；增强 `WindowFrame` / app shell |
| P2 | Toggle / ToggleGroup | `button::toggle` | 有 Switch/Segmented/Radio button-style | ⭐⭐⭐ | 适合 toolbar binary 状态；可增强 ButtonGroup 或独立 `Toggle` |
| P2 | Kbd | `kbd` | 无 | ⭐⭐⭐ | 可作为 `Tag`/`Label` 的轻量变体，显示快捷键 |
| P2 | HoverCard | `hover_card` | 有 Tooltip/Popover | ⭐⭐⭐ | 可增强 Popover：头像/链接 hover preview |
| P2 | GroupBox | `group_box` | 可用 Card/Form 模拟 | ⭐⭐⭐ | 设置页/表单分组很实用，但可先作为 Card variant |
| P2 | Clipboard helper | `clipboard` | CodeBlock 有 copy，公共剪贴板 helper 未独立 | ⭐⭐ | 基础设施级，不一定暴露组件 |
| P2 | Resizable Panels | `resizable` | 有 `Splitter` | ⭐⭐ | Splitter 已覆盖大部分；可吸收 panel/handle API |
| P2 | Scrollable Mask | `scroll/scrollable_mask` | 有 Scrollbar | ⭐⭐ | 用于滚动边缘渐隐提示，视觉增强 |
| P2 | Root / GlobalState 模式 | `root`, `global_state` | Liora 有 Config/ContextExt | ⭐⭐ | 架构参考，不建议直接采集 |

---

## 三、低优先或暂不建议采集

| 控件/能力 | 上游名称 | 原因 |
|---|---|---|
| WebView | `crates/webview`, `examples/webview` | **不采集**。Liora 红线是纯 Rust + GPUI native，禁止 WebView/browser runtime |
| WASM Web Gallery | `story-web` | **不采集**。Liora 当前边界是 native Gallery/Docs |
| Basic Button/Checkbox/Radio/Switch/Slider/Progress/Tag/Badge/Avatar | 同名 | Liora 已有，且 API 风格已定 |
| Calendar/DatePicker/ColorPicker/Image/Pagination/Tooltip/Popover/Dialog/Notification | 同名 | Liora 已有，除非做具体能力差异增强 |
| Table 普通版 | `table/table.rs` | Liora 已有 `Table`，重点应放 DataTable/VirtualizedTable 增强 |
| VirtualList 基础版 | `virtual_list` | Liora 已有 `virtualized_list`，可参考但不需重复 |
| Menu 基础版 | `menu` | Liora 已有 Menu/Dropdown；只采 ContextMenu/AppMenu 细分能力 |
| DescriptionList | `description_list` | Liora 已有 `Descriptions` |
| Rating | `rating` | Liora 已有 `Rate` |
| Stepper | `stepper` | Liora 已有 `Steps` |
| Collapsible | `collapsible` | Liora 已有 `Collapse`，更建议采 Accordion |
| AlertDialog | `dialog/alert_dialog` | Liora 有 MessageBox/Popconfirm/Dialog，可局部吸收 API |
| Separator | `separator` | Liora 有 Divider |
| Label | `label` | Liora 已有 Label |
| ListItem/SeparatorItem | `list/*` | 除非做 List 组件，否则单独采价值不大 |

---

## 四、推荐做成 Liora P22 的采集路线

### P22-A：低风险高收益组件

优先实现，几乎不破坏现有架构。

1. `OtpInput`
2. `Spinner`
3. `Kbd`
4. `DropdownButton`
5. `Accordion`
6. `Combobox`

### P22-B：App Shell 组件

服务 Gallery/Docs，也能作为外部应用搭壳能力。

1. `Sidebar`
2. `StatusBar`
3. `Sheet`
4. `Settings`
5. `TitleBar / WindowFrame enhancement`

### P22-C：高级数据与内容能力

更复杂，但能显著提升 Liora 的企业级深度。

1. `DataTable` 能力增强到 `VirtualizedTable`
2. `TextView` 通用化
3. `CandlestickChart`
4. `SearchableList` 基础设施
5. `CodeEditor` LSP/search/hover/completion 增强调研

### P22-D：大工程，单独阶段

建议不要混进普通组件采集。

1. `Dock Layout`
2. `Native App Menu`
3. `FocusTrap` 全 overlay 体系重构

---

## 五、最终建议采集名单

如果只挑最值得的，建议定这 15 个：

1. **Combobox** ⭐⭐⭐⭐⭐
2. **Sidebar** ⭐⭐⭐⭐⭐
3. **StatusBar** ⭐⭐⭐⭐⭐
4. **Dock Layout** ⭐⭐⭐⭐⭐
5. **Settings UI** ⭐⭐⭐⭐
6. **OtpInput** ⭐⭐⭐⭐
7. **Spinner** ⭐⭐⭐⭐
8. **DropdownButton** ⭐⭐⭐⭐
9. **Accordion** ⭐⭐⭐⭐
10. **Sheet** ⭐⭐⭐⭐
11. **DataTable 增强** ⭐⭐⭐⭐
12. **TextView** ⭐⭐⭐⭐
13. **CandlestickChart** ⭐⭐⭐⭐
14. **SearchableList** ⭐⭐⭐⭐
15. **CodeEditor 高级能力增强** ⭐⭐⭐⭐

其中最推荐先做：

> **OtpInput / Spinner / Kbd / DropdownButton / Accordion / Combobox**

这批成本低、边界清楚、很适合快速变成 Liora 新阶段的第一波。

---

## 六、实施边界提醒

- 不直接复制 `gpui-component` API；应保持 Liora 现有 Element-Plus 风格、builder 风格、主题 token 风格。
- 不采集 WebView、WASM Gallery、HTML/CSS/DOM/browser runtime 路线。
- 涉及 Markdown/HTML 的 `TextView` 只能作为 GPUI 原生元素树渲染能力，不引入浏览器渲染路径。
- 涉及 `Dock Layout`、`Native Menu`、`FocusTrap` 的工作建议单独拆阶段，不要和普通控件混做。
- 对已有组件优先做 in-place enhancement，避免重复组件和平行 API。
