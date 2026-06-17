# Aura UI 架构设计文档

> 基于 Element-Plus 设计规范的 Rust GPUI Native 组件库

---

## 一、项目概述

### 1.1 项目定位

Aura 是一套基于 [GPUI](https://github.com/zed-industries/zed)（Zed 编辑器底层 GPU 加速 UI 框架）的企业级 Native 组件库。参考 [Element-Plus](https://element-plus.org/) 的 API 设计规范和组件分类体系，将 Web 端成熟的设计语言映射到 Rust 原生 GUI 领域。

### 1.2 核心设计原则

参照 Element-Plus 的四项设计原则：

| 原则 | 含义 | Aura 实践 |
|------|------|----------|
| **一致 Consistency** | 与现实生活一致，界面元素统一 | 统一 Design Token、统一 API 风格、统一 Builder 模式 |
| **反馈 Feedback** | 清晰的交互动效和状态反馈 | Hover/Active/Disabled/Loading 四态，动画过渡 |
| **效率 Efficiency** | 简洁直观，减少认知负担 | 链式调用 API，开箱即用，语义化命名 |
| **可控 Controllability** | 用户决策，结果可逆 | 组件可控回调，不强制行为，提供取消机制 |

### 1.3 与 Element-Plus 的核心概念映射

| Element-Plus (Vue/Web) | Aura (Rust/GPUI) | 说明 |
|------------------------|------------------|------|
| `<el-button type="primary">` | `Button::new().primary()` | Builder Pattern |
| Props 属性传递 | 链式方法 `.size()` `.disabled()` | 类型安全的编译期检查 |
| Slots 插槽 | 闭包 `\|cx\| -> impl IntoElement` | 灵活的子元素渲染 |
| `ref` / `reactive` 响应式 | `Model<T>` + `cx.notify()` | GPUI 单向数据流 |
| `emit` 事件派发 | 闭包回调 `.on_click(\|event, window, cx\| {})` | 闭包捕获 |
| `<el-config-provider>` | `cx.set_global(Config{})` | GPUI Global 机制 |
| CSS 变量 / Theme | `Theme` + `cx.global::<Config>()` | Rust 类型系统 |
| Popper.js 弹出定位 | 自研 Anchor + Portal 定位系统 | 第四阶段攻克 |
| Async Validator | 自研 Form 校验模型 | 第三阶段 |

---

## 二、工程架构

### 2.1 Workspace 结构

```
aura/
├── Cargo.toml                    # [workspace] 根配置
├── crates/
│   ├── aura-core/                # 核心 Trait、全局配置、宏、工具函数
│   │   └── src/lib.rs
│   ├── aura-theme/               # 设计系统、Token、亮/暗色模式、色彩计算
│   │   └── src/lib.rs
│   ├── aura-components/          # 全部组件实现
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── button.rs
│   │       ├── input.rs
│   │       ├── chart*.rs         # P10 原生统计图基础设施
│   │       └── ...
│   ├── aura-tray/                # P11 系统托盘 facade
│   ├── aura-packager/            # P12 打包领域逻辑
│   └── aura-icons/               # 图标库
│       └── src/lib.rs
├── apps/
│   ├── aura-gallery/             # 组件看板（GPUI）
│   │   └── src/
│   │       ├── main.rs
│   │       ├── category.rs
│   │       └── demos/
│   └── aura-docs/                # 官方原生文档主程序（GPUI）
│       ├── content/
│       │   ├── pages/            # 每个文档/控件一份 Markdown
│       │   └── snippets/         # 外部 .rs 代码片段，按页面分目录
│       └── src/
│           ├── main.rs
│           └── markdown.rs       # Markdown AST → Aura 原生元素树
└── architecture-design.md
```

### 2.2 Crate 职责矩阵

| Crate | 依赖 | 职责 |
|-------|------|------|
| `aura-theme` | `gpui` | Theme(亮/暗)、Design Tokens、间距/圆角/字号规范、ButtonVariant/ButtonSize/Colors |
| `aura-icons` | `gpui`, `aura-core`, `aura-theme` | Icon 容器（RenderOnce + IntoElement）、IntoIconPath trait |
| `aura-icons-lucide` | `aura-icons` | build.rs 代码生成 → IconName 枚举（1,703 Lucide 图标），实现 IntoIconPath |
| `aura-core` | `gpui`, `aura-theme` | Config(Global)、init_aura()、ContextExt trait、ElementExt trait、Z-Index 管理器、工具函数 |
| `aura-components` | `gpui`, `aura-core`, `aura-theme`, `aura-icons` | 全部业务组件（Button/Input/Dialog/Table 等） |
| `aura-gallery` | `gpui`(default), `gpui_platform`, 全部 aura crates | 组件看板，展示已实现组件 Demo |
| `aura-docs` | `gpui`(default), `gpui_platform`, 全部 aura crates；P8 增加 `pulldown-cmark` | 官方原生文档主程序，包含 Markdown 渲染与 Live Demo 注入 |
| `aura-tray` | `tray-icon`, `muda`(via tray-icon re-export), `image` | P11 系统托盘/进程常驻 facade：动态图标、CheckBox、递归子菜单、稳定命令桥接 |
| `aura-components::chart*` | `gpui` 原生 `canvas`/`PathBuilder`/paint API | P10 统计图控件基础设施与 Line/Area/Bar/Pie/Ring/Sparkline，含降采样与 hover hit testing |
| `aura-packager` | `serde`, `sha2`, `toml` | P12 打包领域逻辑：app metadata、format model、checksum、manifest、backend config |
| `xtask` | `aura-packager` | P12 统一打包入口：validate/build/package/ci/smoke/install-smoke |

### 2.3 GPUI 依赖策略

```toml
# 根 workspace — 不启用平台特定 feature
[workspace.dependencies]
gpui = { git = "https://github.com/zed-industries/zed", default-features = false }

# 库 crate — 继承 workspace（仅需类型定义，无需平台后端）
aura-theme/Cargo.toml: gpui.workspace = true

# App crate — 显式启用所需平台 feature
aura-gallery/Cargo.toml:
  gpui = { workspace = true, features = ["wayland", "x11", "font-kit"] }
  gpui_platform = { workspace = true, features = ["wayland", "x11"] }
```

### 2.5 系统托盘依赖策略（P11）

`aura-tray` 是应用壳层 crate，用于封装 `tray-icon` 和 `muda`，避免 Gallery/Docs/业务 app 直接绑定三方 API。

- 公开 Aura 自有类型：`TrayConfig`、`TrayMenuItemSpec`、`TrayCommand`、`AuraTray`。
- 通过 `tray-icon::menu` re-export 使用 `muda` 菜单类型，不额外引入平行菜单依赖。
- 动态图标通过 `TrayIcon::set_icon` 封装为 `set_icon` / `set_icon_from_rgba` / `set_icon_from_path`。
- 菜单事件映射为稳定 command id，主程序负责将 `Show/Hide/Toggle/Quit/SetIcon/Custom` 应用到 GPUI 窗口和业务状态。
- 启用托盘的 GPUI app 必须使用 `QuitMode::Explicit` 并持有 `AuraTray` 全生命周期。
- Linux 需要 GTK/AppIndicator 系统库；macOS 创建要求主线程；普通文档/demo 仅展示配置预览，避免创建真实 OS 托盘副作用。

### 2.6 原生打包架构（P12）

`aura-packager` 与 `xtask package` 共同承担安装器/包产物流程，应用本体仍然是纯 Rust + GPUI native。

- `aura-packager` 负责 app metadata、package format、backend config、SHA-256 checksum、manifest/release-notes 生成。
- `xtask package` 是唯一公开工程入口，覆盖 `validate`、`build`、`package`、`ci`、`smoke`、`install-smoke`。
- `packaging/` 存放 Gallery/Docs 图标、Linux desktop/metainfo、macOS entitlements、Windows installer resource skeleton。
- GitHub Actions package workflow 覆盖 preview/release matrix，并上传原始 release binary 与 package artifacts。
- `install-smoke --dry-run` 是 runner-safe plan-only gate；真实系统安装、签名、公证、许可策略仍属于外部 policy / dedicated runner 事项。

### 2.4 Gallery 组件看板规约（P7 已自举）

**定位**：aura-gallery 是 Aura 的原生组件看板，功能对标 [Element-Plus 官网](https://element-plus.org/zh-CN/) 的组件总览页。在 GPUI 窗口中按分类层次展示所有已实现组件的交互效果。

**架构**：

```
apps/aura-gallery/src/
├── main.rs                  # App 入口，Gallery struct，分类渲染循环
├── category.rs              # Category 枚举（Basic/Form/Data/Navigation/Feedback/Others）
└── demos/
    ├── mod.rs               # Demo 注册表 registry() -> Vec<DemoEntry>
    ├── button_demo.rs       # 各组件的 Demo 页面渲染函数
    ├── input_demo.rs        # （增量添加）
    └── ...
```

**增量添加规约**：

```
新增一个组件 Demo 只需两步：

1. 创建 apps/aura-gallery/src/demos/<name>_demo.rs
   ── 实现 pub fn render(theme: &Theme) -> AnyElement
   ── 内部按小节（Variants/Sizes/States 等）展示组件多种形态

2. 在 apps/aura-gallery/src/demos/mod.rs 注册表添加一行
   ── DemoEntry { name, category, description, render }
```

**DemoEntry 注册表结构**：

```rust
pub struct DemoEntry {
    pub name: &'static str,              // 组件名称，如 "Button 按钮"
    pub category: Category,              // 所属分类
    pub description: &'static str,        // 一行描述
    pub render: fn(&Theme) -> AnyElement,  // Demo 渲染函数
}
```

**Gallery 渲染逻辑**：

1. `Gallery` struct 在 `render()` 中读取注册表
2. 按 `Category::ALL` 顺序遍历，过滤出每个分类的组件列表
3. 每个分类渲染为一个 `category_section()`：
   - 分类标题（图标 + 名称 + 组件数量）
   - 下方依次排列 `component_card()`
4. 每个卡片 `component_card()` 包含：
   - 组件名称 + 描述
   - 调用 `entry.render(theme)` 渲染 Demo 内容

**Demo 页面编写规范**：

```rust
// 每个 Demo 页面内部按「使用场景」分小节
pub fn render(theme: &Theme) -> AnyElement {
    div()
        .flex().flex_col().gap_3()
        .child(section_header(theme, "Variants 变体"))
        .child(demo_row(theme, vec![
            Button::new("Primary").primary().build(theme),
            Button::new("Success").success().build(theme),
            // ...
        ]))
        .child(section_header(theme, "Sizes 尺寸"))
        .child(demo_row(theme, vec![
            Button::new("Small").small().build(theme),
            // ...
        ]))
        .child(section_header(theme, "States 状态"))
        // ...
        .into_any_element()
}
```

**布局效果**：

```
┌──────────────────────────────────────────────────────┐
│  Aura UI                                             │
│  Native Component Library — 1 components             │
│──────────────────────────────────────────────────────│
│  ⊞ Basic 基础  · 1 components                       │
│  ┌──────────────────────────────────────────────────┐│
│  │ Button 按钮                                       ││
│  │ 常用的操作按钮                                     ││
│  │                                                    ││
│  │ Variants 按钮变体                                  ││
│  │ [Primary] [Success] [Warning] [Danger] [Info]     ││
│  │                                                    ││
│  │ Sizes 尺寸                                         ││
│  │ [Small] [Default] [Large]                         ││
│  │                                                    ││
│  │ States 状态                                        ││
│  │ [Disabled] [Loading]                              ││
│  └──────────────────────────────────────────────────┘│
│                                                      │
│  ☰ Form 表单  · 0 components (coming soon)          │
│  ...                                                 │
└──────────────────────────────────────────────────────┘
```

**设计原则**：

- **增量友好**：新增组件只需 2 个文件位置改动
- **自动编目**：Gallery 自动按分类组织，无需手动维护目录结构
- **层次分明**：分类 → 卡片 → 小节 → 组件实例，四级层次对应 Element-Plus 的信息架构
- **自文档化**：每个 Demo 本身即是组件的使用示例代码
- **即见即得**：`cargo run -p aura-gallery` 直接在原生窗口查看效果

---


### 2.6 P10 Native Charts 架构

**定位**：P10 已交付 Dashboard/监控/报表所需的统计图组件。它们属于 `aura-components`，由 `aura-gallery` 展示，由 `aura-docs` 文档化。

**绝对边界**：统计图必须 100% 使用 GPUI 原生渲染路径。严禁 ECharts、Vega、Plotly、WebView、HTML/CSS、DOM、SVG DOM、WASM 或远程图片渲染。

**参考优先级**：

1. GPUI 官方/本地源码：`canvas(...)`、`PathBuilder`、`Window::paint_path`、`Window::paint_quad`、TextSystem。
2. 当前 Aura 组件模式：`RenderOnce + IntoElement`、全局 Theme、内置唯一 ID、Gallery 自举、Docs 效果→代码。
3. `https://github.com/vicanso/zedis` 仅作为案例参考：其 Metrics 页面使用 GPUI `canvas`，并将图表拆为 scale、axis/grid、shape 层。Aura 可吸收这种分层思想，但必须实现自己的 API、主题与测试。

**建议模块分层**：

```text
crates/aura-components/src/
├── chart.rs            # ChartSeries/Point/Theme/Frame/Legend/Tooltip 公共模型
├── chart_scale.rs      # ScaleLinear / ScaleBand / ScalePoint
├── chart_axis.rs       # Axis/Grid/Tick/Label 布局
├── chart_shape.rs      # Line/Area/Bar/Pie/Ring/Sparkline 绘制 primitive
├── line_chart.rs
├── area_chart.rs
├── bar_chart.rs
├── pie_chart.rs
└── sparkline.rs
```

**已交付**：`LineChart`、`AreaChart`、`BarChart`、`PieChart`、`RingChart`、`Sparkline`，以及共享 `Scale`、`Axis`、`Grid`、`Legend`、`Tooltip/hover hit test` 基础设施。Line/Area/Sparkline 支持大数据降采样；Line/Area/Bar/Pie/Ring 支持原生 hover hit testing。

**渲染策略**：在 `canvas` 的 paint 回调中基于实际 `bounds` 计算 scale；使用 `PathBuilder` 生成折线/面积/扇区 path；使用 `paint_quad` 绘制柱体和必要背景；坐标轴文字、legend、tooltip 文案优先复用 Aura Typography 或 GPUI TextSystem。

### 2.5 P8 Aura Docs 主程序架构

**定位**：P8 将 `aura-gallery` 继续作为组件看板，`aura-docs` 作为官方原生文档主程序。官方文档不再作为 Web 文档站交付，而是在 GPUI 原生窗口中完成导航、Markdown 渲染、组件 API 展示和 Live Demo 注入。

**绝对边界**：

- 100% GPUI 原生视窗运行。
- 不建设 VitePress/Web 文档站；`apps/aura-docs` 是 GPUI 原生主程序。
- 不引入浏览器运行时、跨端转译运行时或网页排版模型。
- 文档渲染输出必须是 Rust 数据结构驱动的 Aura/GPUI 原生元素树。

**核心挑战：富文本折行（Word Wrap）**

P8 采用自举策略：`pulldown-cmark` 只负责 Markdown AST/Event 解析；排版、折行、颜色、粗细、行内代码、块级间距等全部交给 Aura Typography/Layout 组件。富文本段落必须能承载多个不同样式的文本片段，并在同一段落容器中流式拼接、自动换行且不截断。

**建议模块边界**：

```
apps/aura-docs/
├── content/
│   ├── pages/
│   │   ├── button.md        # 单控件文档页
│   │   ├── input.md
│   │   └── ...
│   └── snippets/
│       ├── button/
│       │   ├── types.rs     # 与 button.md 中的代码块 src 对应
│       │   └── sizes.rs
│       └── ...
└── src/
    ├── main.rs              # 独立 GPUI 主窗口
    └── markdown.rs          # render_markdown(md_text: &str) -> gpui::AnyElement

crates/aura-components/src/
├── text.rs / paragraph.rs   # 现有 Typography，可扩展
├── rich_text.rs             # 如现有 Text/Paragraph 不适合，可新增富文本片段（命名待实现时验证）
└── rich_paragraph.rs        # 基于 GPUI StyledText 或等价机制实现流式段落
```

**Markdown Renderer 状态机**：

- 使用 `Vec` 作为块级容器栈。
- `Start(Tag::Heading | Tag::Paragraph | Tag::List | ...)`：压入 Aura 块级容器。
- `Start(Tag::Strong | Tag::Emphasis | Tag::Code)`：更新文本样式上下文。
- `Event::Text`：按当前上下文生成富文本片段，追加到栈顶容器。
- `End(Tag::...)`：弹出栈顶容器并追加到新的栈顶容器。
- `Start(Tag::CodeBlock)`：生成带主题化背景、等宽字体和水平滚动能力的代码块容器。

**文档与代码片段约定**：

- 每个组件文档独立为 `apps/aura-docs/content/pages/<component>.md`，非组件页面使用 snake_case，例如 `quick_start.md`。
- 代码片段独立放在 `apps/aura-docs/content/snippets/<page>/<case>.rs`。
- Markdown 代码块通过 fenced info 引用外部片段：<code>```rust src="button/types.rs"</code>。
- `src` 路径相对于 `content/snippets/`，由 `markdown.rs` 映射为编译期 `include_str!`，避免运行时文件 IO。

**Live Demo 注入**：

Markdown 中的特殊语法：

```text
::AuraDemo{component="Button"}::
```

由 renderer 识别为真实 Aura 组件节点，而不是文本。示例：识别 `Button` 后插入 `Button::new("Button").primary()` 等真实 view node，保留 hover/click 等原生交互。

**Rust edition 说明**：新技术方案的最低语义基线可按 Rust 2021 理解；当前仓库 `Cargo.toml` 已使用 edition 2024，P8 不因文档方案调整而回退 edition。

## 三、组件总览

参照 Element-Plus 官网组件分类，共 81 个组件，分为 7 大类。标注 `GPUI原生` 表示 GPUI 已内置无需单独实现，`v2` 表示第二阶段迭代。

### 3.1 Basic 基础组件 (13)

| Element-Plus | Aura 组件 | 阶段 | 说明 |
|-------------|----------|------|------|
| Button | `Button` | P1 ✅ | 已完成基础实现 |
| Icon | `Icon` trait | P1 | 图标系统，集成 SVG |
| Link | `Link` | P1 | 链接按钮样式 |
| Text | `Text` | P1 | 文本组件（单行/多行截断） |
| Typography | `AuraTitle` / `AuraParagraph` | P1 | 排版组件 |
| Space | `AuraSpace` | P1 | 间距组件 |
| Divider | `AuraDivider` | P1 | 分割线 |
| CodeBlock | `CodeBlock` | P1 ✅ | 代码高亮显示，支持语言标签、复制、块级/行内格式 |
| Scrollbar | `AuraScrollbar` | P2 | 自定义滚动条 |
| Layout | `AuraRow` / `AuraCol` | P2 | 24 栅格布局（GPUI flexbox 已有基础，此为语义封装） |
| Container | `AuraContainer` | P2 | 布局容器（header/aside/main/footer） |
| Border | — GPUI 原生 | — | `.border_1()` `.border_color()` |
| Color | — Theme Token | — | 设计 Token 中已定义 |

### 3.2 Config 配置组件 (1)

| Element-Plus | Aura 组件 | 阶段 | 说明 |
|-------------|----------|------|------|
| Config Provider | `Config` + `init_aura()` | P0 ✅ | Global 注入，已完成 |

### 3.3 Form 表单组件 (24)

| Element-Plus | Aura 组件 | 阶段 | 说明 |
|-------------|----------|------|------|
| Input | `AuraInput` | P2 | 文本输入（含 prefix/suffix icon、clearable、password toggle） |
| InputNumber | `AuraInputNumber` | P2 | 数字输入（步进按钮、min/max 限制） |
| Textarea | `Textarea` | P2 | 多行文本（自动撑高、maxlength 计数） |
| Checkbox | `AuraCheckbox` / `AuraCheckboxGroup` | P2 | 多选 |
| Radio | `AuraRadio` / `AuraRadioGroup` | P2 | 单选 |
| Switch | `AuraSwitch` | P2 | 开关 |
| Select | `AuraSelect` | P3 | 下拉选择 ⚠️ Popper 定位 |
| Slider | `AuraSlider` | P3 | 滑块 |
| Form | `AuraForm` / `AuraFormItem` | P3 | 表单容器 + 校验 |
| Rate | `AuraRate` | P3 | 评分 |
| DatePicker | `AuraDatePicker` | P5 | 日期选择（自定义日历面板） |
| TimePicker | `AuraTimePicker` | P5 | 时间选择 |
| DateTimePicker | `AuraDateTimePicker` | P5 | 日期时间选择 |
| ColorPicker | `AuraColorPicker` | P5 | 颜色选择 |
| Cascader | `AuraCascader` | P5 | 级联选择 |
| Transfer | `AuraTransfer` | P5 | 穿梭框 |
| Upload | `AuraUpload` | P5 | 文件上传 |
| Autocomplete | `AuraAutocomplete` | P5 | 自动补全 |
| TreeSelect | `AuraTreeSelect` | P5 | 树形选择 |
| VirtualizedSelect | — v2 | P6 | 虚拟化选择器（GPUI 已有 UniformList） |
| InputTag | `AuraInputTag` | P5 | 标签输入 |
| Mention | `AuraMention` | P5 | @提及 |
| DatePickerPanel | — 子组件 | P5 | 日期面板（内部组件） |
| ColorPickerPanel | — 子组件 | P5 | 颜色面板（内部组件） |

### 3.4 Data 数据展示 (23)

| Element-Plus | Aura 组件 | 阶段 | 说明 |
|-------------|----------|------|------|
| Avatar | `AuraAvatar` | P2 | 头像 |
| Badge | `AuraBadge` | P2 | 徽章 |
| Tag | `AuraTag` | P2 | 标签 |
| Card | `AuraCard` | P3 | 卡片 |
| Collapse | `AuraCollapse` / `AuraCollapseItem` | P3 | 折叠面板 |
| Progress | `AuraProgress` | P3 | 进度条 |
| Skeleton | `AuraSkeleton` | P3 | 骨架屏 |
| Empty | `AuraEmpty` | P3 | 空状态 |
| Result | `AuraResult` | P4 | 结果页 |
| Descriptions | `AuraDescriptions` | P4 | 描述列表 |
| Timeline | `AuraTimeline` | P4 | 时间线 |
| Tree | `AuraTree` | P4 | 树形控件 |
| Pagination | `AuraPagination` | P4 | 分页 |
| Statistic | `AuraStatistic` | P4 | 统计数值 |
| Segmented | `AuraSegmented` | P4 | 分段控制器 |
| Table | `AuraTable` | P5 ⚠️ | 表格 ⚠️ 重难点 |
| Calendar | `AuraCalendar` | P5 | 日历 |
| Carousel | `AuraCarousel` | P5 | 走马灯 |
| Image | `AuraImage` | P5 | 图片（懒加载、预览） |
| Tour | `AuraTour` | P5 | 漫游引导 |
| VirtualizedTable | — v2 | P6 | 虚拟化表格 |
| VirtualizedTree | — v2 | P6 | 虚拟化树 |
| InfiniteScroll | — GPUI UniformList | — | GPUI 已有列表虚拟滚动 |

### 3.5 Navigation 导航 (9)

| Element-Plus | Aura 组件 | 阶段 | 说明 |
|-------------|----------|------|------|
| Dropdown | `AuraDropdown` | P3 | 下拉菜单 ⚠️ Popper 定位 |
| Menu | `AuraMenu` | P4 | 导航菜单 |
| Tabs | `AuraTabs` / `AuraTabPane` | P4 | 标签页（下划线动画） |
| Breadcrumb | `AuraBreadcrumb` | P4 | 面包屑 |
| Steps | `AuraSteps` | P4 | 步骤条 |
| PageHeader | `AuraPageHeader` | P4 | 页头 |
| Affix | `AuraAffix` | P4 | 固钉（滚动吸顶） |
| Backtop | `AuraBacktop` | P4 | 回到顶部 |
| Anchor | `AuraAnchor` | P4 | 锚点链接 |

### 3.6 Feedback 反馈组件 (10)

| Element-Plus | Aura 组件 | 阶段 | 说明 |
|-------------|----------|------|------|
| Tooltip | `AuraTooltip` | P3 | 文字提示 ⚠️ Popper 定位 |
| Popover | `AuraPopover` | P3 | 气泡卡片 ⚠️ Popper 定位 |
| Popconfirm | `AuraPopconfirm` | P3 | 气泡确认 ⚠️ Popper 定位 |
| Dialog | `AuraDialog` | P3 | 模态对话框（遮罩、焦点锁定） |
| Drawer | `AuraDrawer` | P3 | 抽屉面板 |
| Message | `AuraMessage` | P3 | 全局消息提示 |
| Notification | `AuraNotification` | P3 | 通知 |
| Alert | `AuraAlert` | P3 | 警示提示 |
| Loading | `AuraLoading` | P3 | 加载状态（全屏/局部） |
| MessageBox | `AuraMessageBox` | P4 | 消息弹窗（confirm/prompt） |

### 3.7 Charts 统计图 (6+)

| 中文 | Public API | 阶段 | 说明 |
|------|------------|------|------|
| 折线图 | `LineChart` | P10 | 单/多 series、axis/grid、legend、点标记、降采样、hover tooltip |
| 面积图 | `AreaChart` | P10 | overlay/stacked area、渐变填充、降采样、overlay hover tooltip |
| 柱状图 | `BarChart` | P10 | grouped/stacked、range color、standalone mini、hover tooltip |
| 饼图 | `PieChart` | P10 | 扇区、百分比、外部标注、极坐标 hover tooltip |
| 环图 | `RingChart` | P10 | donut、外置 legend/value、内圆排除 hover tooltip |
| 迷你趋势图 | `Sparkline` | P10 | 卡片/Statistic 内嵌微型趋势图、降采样、趋势样式 |

### 3.8 Others 其他 (2)

| Element-Plus | Aura 组件 | 阶段 | 说明 |
|-------------|----------|------|------|
| Divider | `AuraDivider` | P1 | 分割线（横向/纵向/带文字） |
| Watermark | `AuraWatermark` | P5 | 水印 |

---

## 四、开发阶段规划

### 阶段总览

```
P0 · Foundation       ██░░░░░░░░  已完成 — 工程骨架 + 主题 + Button 最小闭环
P1 · Basic Elements   ░░░░░░░░░░  基础组件 + CodeBlock 代码显示
P2 · Form Controls    ░░░░░░░░░░  表单控件 24 个（前 10 个核心）
P3 · Popper + Feedback ░░░░░░░░░░  弹出层基建 + 反馈组件 + Data 初步
P4 · Navigation + Data ░░░░░░░░░░  导航组件 + 数据展示扩容
P5 · Advanced          ░░░░░░░░░░  重型组件（Table/DatePicker/Upload 等）
P6 · Built-in Unique ID  ██░░░░░░░░  已完成 — 内建唯一 ID
P7 · Demo Self-Contained ██░░░░░░░░  已完成 — Gallery Demo 自举
P8 · Native Docs App ██░░░░░░░░  核心已完成 — 原生文档大屏 + Markdown + Live Demo
P9 · Deferred Advanced ✅  已迁移并由 P14 完成
P10 · Native Charts ✅  已完成 — GPUI 原生统计图组件 + tooltip/性能维护
P11 · Native Tray ✅  已完成 — 系统托盘 facade + Gallery/Docs 用例
P12 · Native Packaging 🔶  Readiness — 打包流水线已可用，签名/真实系统安装需外部策略
P13 · Component Expansion ✅  已完成 — 业务控件与既有控件增强
P14 · Deferred Advanced ✅  已完成 — P9 backlog 全部补齐
P15 · Quality Hardening 🔄  Active — CI 门禁、API/视觉/交互/性能/Docs 收口
```

### P0 · Foundation（已完成）

**目标**：跑通 "配置 → 渲染" 最小闭环

| 任务 | 状态 |
|------|------|
| Cargo Workspace 工程结构 + 4 crate + 2 app | ✅ |
| GPUI 依赖策略（workspace default-features=false，app 显式 features） | ✅ |
| `Theme` Design Tokens（色板/间距/圆角/字号） | ✅ |
| `Config` Global 注入 + `init_aura()` | ✅ |
| `ContextExt` trait（从任意 `Context<'_, V>` 读取主题） | ✅ |
| `Button` 首个组件（6 种 Variant、3 种 Size、Disabled/Loading） | ✅ |
| `aura-gallery` GPUI 窗口展示 Button 变体 | ✅ |
| `Icon` trait + `IconSize` + 占位图标函数 | ✅ |
| `ElementExt` 通用 trait 骨架 | ✅ |
| Z-Index 管理工具函数 | ✅ |

### P1 · Basic Elements（目标：完成基础物料层）

**组件清单（13 个）**：

```
Button(完善)  Icon(SVG集成)  Link  Text  Title  Paragraph
Space  Divider  Row  Col  Container  Scrollbar  Splitter
```

**关键任务**：

| 任务 | 说明 |
|------|------|
| **Button 完善** | 添加 icon 支持、圆角按钮、幽灵按钮、按钮组 ButtonGroup |
| **Icon 系统** | 集成 SVG 图标集（如 Lucide），支持动态变色和尺寸 |
| **Link** | 链接样式按钮（underline/hover/disabled） |
| **Text / Title / Paragraph** | 排版组件（截断、行数限制、渐变） |
| **Space** | 间距包裹组件（横向/纵向，自动 gap） |
| **Divider** | 分割线（横向/纵向/带文字/虚线） |
| **Row / Col** | 24 栅格系统（gutter、offset、响应式断点） |
| **Container** | 布局容器（header/aside/main/footer 经典布局） |
| **Scrollbar** | 自定义滚动条样式 |
| **Splitter** | 分隔面板（可拖拽调整大小） |

### P2 · Form Controls（目标：表单数据录入体系）

**组件清单（10 个核心）**：

```
Input  InputNumber  Textarea  Checkbox(Group)  Radio(Group)
Switch  Select  Slider  Form/FormItem  Rate
```

**关键任务**：

| 任务 | 说明 |
|------|------|
| **Input** | 文本输入（prefix/suffix icon、clearable、password toggle、maxlength 计数） |
| **InputNumber** | 数字输入（步进按钮、min/max/precision） |
| **Textarea** | 多行文本（auto-resize、maxlength） |
| **Checkbox / CheckboxGroup** | 多选（indeterminate 半选、min/max 限制） |
| **Radio / RadioGroup** | 单选（button 样式、border） |
| **Switch** | 开关（active/inactive 文字、loading） |
| **Select** | 下拉选择（⚠️ Popper 定位、可搜索、多选、分组） |
| **Slider** | 滑块（范围选择、刻度标记、input 联动） |
| **Form / FormItem** | 表单容器（label-width、必填标记、校验状态、error message、inline 模式） |
| **Rate** | 评分（半星、文字辅助、只读） |

### P3 · Popper + Feedback（目标：攻克悬浮弹出层基建）

**核心难点**：原生 GUI 中没有 HTML 的 `position: fixed` 和自动 Z-Index 层级。需要自研 Anchor → Portal 定位系统。

**弹出层基建**：

| 任务 | 说明 |
|------|------|
| **AnchorPosition** | 锚点定位计算引擎（top/bottom/left/right + 12 种对齐偏移） |
| **Portal** | 将元素渲染到根窗口最顶层（脱离正常布局流） |
| **ViewportBoundary** | 屏幕边缘溢出检测与自动翻转方向 |
| **ZIndexStack** | 全局 Z-Index 栈管理（弹窗/下拉/通知的分层） |
| **ClickOutside** | 点击外部区域关闭检测 |
| **FocusTrap** | 焦点锁定（Tab 循环在弹窗/下拉内） |

**组件清单（10 个反馈 + 2 个 Data + 1 个 Nav）**：

```
Tooltip  Popover  Popconfirm  Dialog  Drawer
Message  Notification  Alert  Loading  MessageBox
Dropdown  Card  Collapse
```

### P4 · Navigation + Data Expansion（目标：导航与数据展示全貌）

**组件清单（20 个）**：

```
Menu  Tabs  Breadcrumb  Steps  PageHeader  Affix  Backtop  Anchor
Progress  Skeleton  Empty  Result  Descriptions  Timeline
Tree  Pagination  Statistic  Segmented  Tag  Avatar  Badge
```

### P5 · Advanced Components（目标：重型组件攻克）

**组件清单（20 个）**：

```
Table  Calendar  Carousel  Image  DatePicker  TimePicker  DateTimePicker
ColorPicker  Cascader  Transfer  Upload  Autocomplete
TreeSelect  InputTag  Mention  Watermark  Tour  Scrollbar  Splitter
```

⚠️ **Table 组件特别说明**：

Table 是企业级组件库中最复杂、工作量最大的组件。Aura Table 需规划以下能力矩阵：

| 能力 | 优先级 | 说明 |
|------|--------|------|
| 基础渲染（列定义 + 行数据） | P0 | 核心 |
| 固定表头 | P0 | Sticky header |
| 固定列（左/右） | P1 | 横向滚动 + 固定列 |
| 排序（单列/多列） | P1 | 表头点击排序 |
| 筛选（列过滤） | P1 | 表头下拉筛选 |
| 选择（单选/多选） | P1 | Checkbox 列 |
| 展开行 | P2 | 可展开的嵌套内容 |
| 合并行/列 | P2 | rowspan / colspan |
| 树形数据 | P2 | 树形表格 |
| 虚拟滚动 | P2 | 万级数据性能 |
| 拖拽排序 | P3 | 行拖拽 / 列拖拽 |
| 编辑单元格 | P3 | 行内编辑 |
| 导出 CSV | P3 | 数据导出 |
| 汇总行 / 合计 | P3 | 表尾合计 |
| Loading 状态 | P0 | 数据加载中 |
| Empty 状态 | P0 | 空数据占位 |

### P8 · Native Gallery Documentation（原生文档大屏 + 工程化交付）

| 任务 | 说明 |
|------|------|
| **Aura Docs 官方文档主程序** | 在 GPUI 原生窗口中承载文档导航、Markdown 内容、组件 Demo 和组件 API 说明 |
| **Typography 自举基础设施** | 富文本片段 + 段落组件；基于 GPUI StyledText 或等价机制实现多样式片段流式拼接、自动换行、不截断 |
| **Markdown 解析引擎** | `pulldown-cmark` 生成 AST/Event；`apps/aura-docs/src/markdown.rs` 映射为 Aura/GPUI 原生元素树 |
| **代码块/行内代码渲染** | 等宽字体、主题化背景、代码块水平滚动、行内代码不破坏段落换行 |
| **双栏文档窗口** | 左侧文档/组件导航树，右侧 Markdown 渲染结果区，右侧支持垂直滚动 |
| **Live Demo 注入** | `::AuraDemo{component="Button"}::` 映射为真实 Aura 组件 view node，保留原生交互 |
| **组件 API 文档** | 每个组件：Builder 方法 / Events 回调 / Slots 子元素 / 示例代码 / Live Demo |
| **测试体系** | 单元测试（crate 级）、Markdown renderer 回归测试、组件测试、集成测试 |
| **CI/CD** | GitHub Actions：cargo check / clippy / test / doc build |
| **发布流程** | crates.io 发布策略、CHANGELOG 自动化 |

---


### P10 · Native Charts（统计图组件，已完成）

目标：补齐 Aura 在企业 Dashboard、监控、报表场景中的原生可视化能力。

| 子任务 | 优先级 | 说明 |
|--------|--------|------|
| Chart scale 基础设施 | ✅ | `ScaleLinear` / `ScaleBand` / `ScalePoint`，覆盖空值、单点、负值、手动 domain |
| Axis/Grid 基础设施 | ✅ | x/y axis、ticks、grid line、label format |
| LineChart | ✅ | 单/多 series、axis/grid、legend、点标记、降采样、hover tooltip |
| AreaChart | ✅ | overlay/stacked、透明/渐变填充、降采样、overlay hover tooltip |
| BarChart | ✅ | 分类轴、grouped/stacked、standalone mini、range color、hover tooltip |
| PieChart/RingChart | ✅ | 极坐标扇区/圆环、外部标注、polar hover tooltip |
| Sparkline | ✅ | 无坐标轴微型趋势图，可嵌入 Statistic/Card，支持降采样与线型 |
| Hover/Tooltip | ✅ | Line/Area/Bar/Pie/Ring 原生命中测试与 Aura Tooltip portal |
| 性能优化 | ✅ | Line/Area/Sparkline min/max bucket 降采样、稀疏 axis/value labels、避免全量中间 Vec |

验收：所有图表组件已进入 `aura-components` 导出；Gallery 有自举 Demo；Docs 有按“效果 → 代码”组织的页面和完整 `.rs` snippets；相关 scale/shape/builder/hit-test 测试通过。

## 五、组件 API 设计规范

### 5.1 组件 API 风格（codex 范式）

**强制规则**：所有组件必须实现 `RenderOnce` + `IntoElement`（通过 `Component`），
**禁止** `.build(theme)` 等显式传参模式。主题通过 `cx.global::<Config>().theme` 自动读取。

```rust
// ✅ 正确 — codex 范式：RenderOnce + IntoElement
pub struct MyComponent { /* config */ }

impl RenderOnce for MyComponent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        // ... render using theme
    }
}

impl IntoElement for MyComponent {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element { Component::new(self) }
}

// 使用：直接放入 vec![] 或 .child()
MyComponent::new().size(24.0).primary()

// ❌ 禁止 — 野路子
MyComponent::new().build(theme)
```

### 5.2 图标系统 API

```rust
// Lucide 内置图标 — 颜色自动从全局主题读取
Icon::new(IconName::House).size(24.0)                          // 默认图标色
Icon::new(IconName::Star).color(theme.warning.base)            // 自定义色

// 自定义路径
Icon::new("/path/to/custom.svg").size(32.0)
```

**架构**：
- `aura-icons`：`Icon` 容器（`RenderOnce` + `IntoElement`），`IntoIconPath` trait
- `aura-icons-lucide`：`build.rs` 扫描 `assets/svgs/*.svg` → 生成 `IconName` 枚举
- 1,703 个 Lucide 图标自动生成，首次编译通过 `scripts/sync-lucide.sh` 拉取

### 5.3 事件回调规范

组件通过 Builder 方法接受回调闭包，直接传入（不经过 `cx.listener`）：

```rust
Button::new("Click Me")
    .on_click(|event, window, cx| {
        println!("clicked!");
    })
```

类型签名：`impl Fn(&ClickEvent, &mut Window, &mut App) + 'static`

### 5.4 组件命名规范

接受 `impl IntoElement` 作为子内容：

```rust
AuraCard::new()
    .header(div().child("Card Title"))
    .body(div().child("Card Content"))
    .build(&theme)
```

### 5.5 组件命名规范

- 组件名：`Aura` + PascalCase → `Button`, `AuraInput`, `AuraDialog`
- Builder 方法：`pub fn new(...) -> Self` 构造函数，`.variant()` / `.size()` 配置方法
- 快捷方法：`.primary()` = `.variant(ButtonVariant::Primary)`，`.small()` = `.size(ButtonSize::Small)`
- 构建方法：`.build(&theme) -> impl IntoElement`

---

## 六、设计 Token 体系

> 色板方案: **NaiveUI Forest Green** — 参考 [NaiveUI](https://www.naiveui.com/) 色彩体系

### 6.1 语义色板

```
Primary (Green):  #18A058  | Hover: #36AD6A  | Active: #0C7A43
Info (Blue):      #2080F0  | Hover: #4098FC  | Active: #1060C9
Success (Green):  #18A058  | Hover: #36AD6A  | Active: #0C7A43
Warning (Gold):   #F0A020  | Hover: #FCB040  | Active: #C97C10
Danger (Red):     #D03050  | Hover: #DE576D  | Active: #AB1F3F
```

### 6.2 中性色 (Light)

```
Text Primary:    rgb(31, 34, 37)     Body:       #FFFFFF
Text Secondary:  rgb(51, 54, 57)     Card:       #FFFFFF
Text Tertiary:   rgb(118, 124, 130)  Modal:      #FFFFFF
Text Disabled:   rgba(0,0,0,0.38)    Popover:    #FFFFFF
Placeholder:     rgba(0,0,0,0.38)
Icon:            rgba(0,0,0,0.38)

Border:          rgb(224, 224, 230)  Divider:    rgb(239, 239, 245)
Hover:           rgb(243, 243, 245)  Pressed:    rgb(237, 237, 239)
Overlay:         rgba(0,0,0,0.50)    Mask:       rgba(255,255,255,0.90)
```

### 6.3 暗色模式 (Dark)

```
Primary (Green):  #63E2B7  | Hover: #7FE7C4  | Active: #5ACEA7
Info (Blue):      #70C0E8  | Hover: #8ACBEC  | Active: #66AFD3
Success (Green):  #63E2B7  | Hover: #7FE7C4  | Active: #5ACEA7
Warning (Gold):   #F2C97D  | Hover: #F5D599  | Active: #E6C260
Danger (Red):     #E88080  | Hover: #E98B8B  | Active: #E57272

Text 1:    rgba(255,255,255,0.90)    Body:       #101014
Text 2:    rgba(255,255,255,0.82)    Card:       #18181C
Text 3:    rgba(255,255,255,0.52)    Modal:      #2C2C32
Disabled:  rgba(255,255,255,0.38)    Popover:    #48484E

Border:    rgba(255,255,255,0.24)    Divider:    rgba(255,255,255,0.09)
Hover:     rgba(255,255,255,0.09)    Pressed:    rgba(255,255,255,0.05)
```

### 6.4 间距

```
xs:  4px    sm:  8px    md: 12px
lg: 20px    xl: 32px
```

### 6.3 圆角

```
sm:   2px    md:   4px
lg:   8px    full: 9999px (胶囊)
```

### 6.4 字号

```
xs: 10px    sm: 12px    md: 14px
lg: 16px    xl: 20px
```

### 6.5 组件尺寸

| Size | Height | Padding X | Font |
|------|--------|-----------|------|
| Small | 24px | 8px | xs(10px) |
| Default | 32px | 15px | md(14px) |
| Large | 40px | 19px | lg(16px) |

---

## 七、技术难点与解决方案

### 7.1 Popper/Popup 弹出定位

| 难点 | 方案 |
|------|------|
| 绝对定位（脱离布局流） | **Portal 机制**：将弹出内容渲染到窗口根节点而非父元素内 |
| 锚点坐标计算 | **Anchor Trait**：通过 `window.bounds_for_element(anchor_id)` 获取锚点相对窗口坐标 |
| 边缘溢出检测 | **ViewportBoundary**：计算弹出框与窗口四边的距离，自动翻转方向 |
| Z-Index 层级 | **ZIndexStack**：全局 `u32` 递增栈，popup=+100, modal=+200, notification=+300, tooltip=+400 |
| 点击外部关闭 | **ClickOutside**：全局 click 事件监听，判断目标是否在弹出元素内 |
| 焦点锁定 | **FocusTrap**：弹出层内 Tab 键循环（first/last focusable element） |

### 7.2 表单校验

参照 Element-Plus 的 `async-validator`：

```rust
pub struct FormRule {
    pub required: bool,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub pattern: Option<Regex>,
    pub validator: Option<Box<dyn Fn(&str) -> Result<(), String>>>,
    pub message: String,
    pub trigger: ValidationTrigger, // Change | Blur | Submit
}
```

### 7.3 Table 虚拟滚动

- GPUI 已有 `UniformList` 支持等高的虚拟列表
- Table 需要在此基础上支持**不等高行**和**固定列**
- 方案：按列拆分为多个 `UniformList`，横向滚动时同步位移

### 7.4 国际化 (i18n)

- `Config` 中扩展 `locale: AuraLocale`
- 组件内部文案从 `cx.global::<Config>().locale` 读取
- 预设 `zh-CN` / `en-US` 语言包

---

## 八、测试策略

| 层级 | 工具 | 覆盖目标 |
|------|------|---------|
| 单元测试 | `cargo test` | Theme 计算、工具函数、组件逻辑 |
| 组件测试 | GPUI TestApp | 组件渲染正确性、交互行为 |
| 视觉回归 | 截图对比 | 组件在不同主题/尺寸下的视觉一致性 |
| 集成测试 | `aura-gallery` | 手动验证全部组件交互 |

---

## 九、里程碑与交付计划

| 阶段 | 组件数 | 预计周期 | 交付物 |
|------|--------|---------|--------|
| P0 Foundation | — | ✅ 已完成 | Workspace + Theme + Config + Button |
| P1 Basic | 15 | 2-3 周 | Icon/Link/Text/Layout/Space/Divider/Row/Col/Container/Scrollbar/CodeBlock |
| P2 Form | 10 | 3-4 周 | Input/InputNumber/Switch/Checkbox/Radio/Select/Slider/Form/Rate |
| P3 Popper+Feedback | 13 | 4-5 周 ⚠️ | Popper 基建 + Tooltip/Dialog/Drawer/Message/Dropdown/Popover/Popconfirm |
| P4 Navigation+Data | 20 | 3-4 周 | Menu/Tabs/Steps/Progress/Skeleton/Tree/Pagination+ |
| P5 Advanced | 20 | 6-8 周 ⚠️ | Table/DatePicker/Upload/Carousel/Cascader+ |
| P6 Built-in Unique ID | — | ✅ 已完成 | 内建唯一 ID |
| P7 Demo Self-Contained | — | ✅ 已完成 | Gallery Demo 自举 |
| P8 Native Docs App | — | ✅ 核心完成 | 原生文档大屏/Markdown/Live Demo/Test/CI/Release |
| P9 Deferred Advanced | 9 | ✅ 已迁移 | 由 P14 完成 Carousel/Calendar/TreeSelect/InputTag/Mention/Watermark/Tour/Virtualized* |
| P10 Native Charts | 6+ | ✅ 已完成 | Line/Area/Bar/Pie/Ring/Sparkline + scale/axis/grid/legend/tooltip/降采样 |
| P11 Native Tray | — | ✅ 已完成 | aura-tray facade + Gallery/Docs tray control examples |
| P12 Native Packaging | — | 🔶 Readiness | aura-packager + xtask + CI；签名/真实安装需外部策略 |
| P13 Component Expansion | 18 | ✅ 已完成 | QR/CodeEditor/Signal/Heat/Segment/Timer/Label/Operation 等与既有控件增强 |
| P14 Deferred Advanced | 9 | ✅ 已完成 | P9 backlog 全部补齐 |
| P15 Quality Hardening | — | 🔄 Active | 通用 CI 门禁与 API/视觉/交互/性能/Docs 收口 |
| **合计** | **76+** | **约 6 个月** | 完整企业级组件库 |

---

## 十、附录

### A. 参考资源

- [Element-Plus 官网](https://element-plus.org/zh-CN/)
- [Element-Plus GitHub](https://github.com/element-plus/element-plus)
- [GPUI 官网](https://gpui.rs)
- [Zed Editor GitHub](https://github.com/zed-industries/zed)
- [GPUI Hello World 示例](https://github.com/zed-industries/zed/blob/main/crates/gpui/examples/hello_world.rs)

### B. 文件清单

| 文件 | 说明 |
|------|------|
| `Cargo.toml` | Workspace 根配置 |
| `crates/aura-core/src/lib.rs` | Config, Global, ContextExt, init_aura, Z-Index |
| `crates/aura-theme/src/lib.rs` | Theme, Design Tokens, ButtonVariant/Size |
| `crates/aura-components/src/button.rs` | Button 组件 |
| `crates/aura-icons/src/lib.rs` | Icon trait, IconSize |
| `apps/aura-gallery/src/main.rs` | Gallery 展示应用 |
| `architecture-design.md` | 本文档 |
| `structure.txt` | 目录结构速览 |
| `chat.txt` | 初始调研对话记录 |
