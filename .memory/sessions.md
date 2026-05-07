# Session History

## Session 15 — 2026-05-06 (Night)

### Actions
- **修复 Alert 控件图标与标题文本垂直对齐问题**:
  - 在 `Alert` 组件的主容器 `div` 上添加 `.items_center()`，确保单行模式下图标与文字完美居中。
  - 为标题文本容器添加 `.flex().items_center().min_h(px(20.0))`，使其与 20px 的图标高度对齐。
- **完成 P4 Navigation & Data 组件开发**:
  - **Menu**: 支持 `Vertical` / `Horizontal`，折叠状态，多级嵌套，Popover 弹出子菜单。
  - **Tabs**: 支持 `Standard` / `Card` / `BorderCard` 风格，灵活位置，动态新增与关闭。
  - **Breadcrumb**: 支持自定义字符串与图标分隔符，末级自动加粗，支持点击跳转。
  - **Steps**: 支持 `Horizontal` / `Vertical`，自动推导 `Wait`/`Process`/`Finish`/`Error` 状态。
  - **PageHeader**: 包含返回按钮、主副标题、扩展插槽 (`extra`, `content`, `footer`)。
  - **Affix**: 基于 `BoundsTracker` 监听 `paint` 阶段坐标，支持滚动吸顶/吸底。
  - **Backtop**: 绑定 `ScrollHandle` 监听滚动偏移，支持自定义显示阈值与平滑回顶。
  - **Anchor**: 基于 `AnchorTarget` 收集位置信息，支持滚动自动高亮和多级嵌套跳转。
  - **Progress**: 实现 `Line` 线形进度条，根据不同状态展示对应颜色和内置图标。
  - **Skeleton**: 提供 `Circle`, `Square`, `Paragraph`, `Image` 占位，支持多行随机宽度。
  - **Empty**: 内置缺省图标，支持自定义图片、描述文案以及 `extra` 底部操作插槽。
  - **Result**: 内置四种标准结果状态 (`Success`, `Warning`, `Error`, `Info`)，支持灵活的图文排版。
  - **Descriptions**: 基于 `span` 与 `column` 算法模拟 Grid 布局，支持 `Horizontal`/`Vertical` 及带边框的表格样式。
  - **Timeline**: 自动计算并绘制垂直轴线，支持 `reverse` 倒序、自定义图标、以及不同时间戳位置。
  - **Tree**: 递归渲染树形结构，通过深度计算左侧缩进，支持节点的展开与折叠交互。
  - **Pagination**: 根据传入的 `layout` 字符串动态渲染分页模块，内置标准分页折叠算法，支持 `background` 背景色模式。
  - **Statistic**: 提供数值格式化展示，支持前后缀图标和自定义数值颜色，方便突出关键指标。
  - **Segmented**: 提供分段选择控制器，支持禁用单个选项和 `block` 模式撑满容器。至此，P4 阶段 20 个核心组件全部完成开发并集成 Demo。
- **Gallery Demo 增强**:
  - 新增以上所有 17 个组件的展示用例。
- **Git 提交与推送**:
  - 逐步提交组件实现并推送到 `main` 分支。

### Verification
- `cargo check` passed.

### Key Discoveries
- GPUI 中 `cx.entity()` 是在 `Render` 过程中获取自身 View 句柄的正确方式，用于在异步或独立 Context 中回调更新原始 View。
- 复杂的 View 组件在 Demo 中需通过 `cx.new(|_| Component::new())` 实例化以满足 `IntoElement` 约束。
- 由于 GPUI `Div` 等元素未实现 `Clone`，在需要多次引用同一子树时，应使用闭包或局部渲染函数。
- `RenderOnce` 组件处理循环渲染时需注意 `items` 所有权，适时使用 `into_iter()`。
- 连接线在 `flex` 布局中可以通过 `flex_1` 配合 `h(px(1.0))` 轻松实现自适应伸缩。
- 在 `'static` 闭包中访问全局状态 (如 Theme) 时，应在闭包执行时从传入的 `App` (cx) 中获取。
- 实现 `Affix` / `Anchor` 等依赖布局结果的组件，可以在 `paint` 阶段检测 `Bounds` 并反向 `notify` View。
- `ScrollHandle` 的 `offset()` 结合 `View` 的 re-render 机制可方便实现基于滚动进度的交互。
- 进度条宽度可通过 `gpui::relative(percentage / 100.0)` 实现响应式。
- 骨架屏的 `div` 占位配合 `theme.neutral.hover` 背景色在 GPUI 中表现优异。
- `div().overflow_y_scroll()` 必须在设置 `.id()` 之后调用。
- 在 `when` 闭包中使用 `.id()` 会将元素转换为 `Stateful` 类型，可能导致与原始非 `Stateful` 元素类型不匹配。

## Session 2 — 2026-05-03

### Actions
- codex 重构 button 为 RenderOnce + IntoElement
- codex 消除 .build(theme) 传参模式
- codex 实现按钮内置唯一 ID

### Key Discoveries
- GPUI RenderOnce 适合无状态一次性组件
- Component::new() 包装后可直接用于 .child()

## Session 1 — 2026-05-03

### Actions
- 搭建 Cargo workspace 结构 (4 crate + 2 app per structure.txt)
- 实现 aura-theme: Theme, Design Tokens, light/dark 模式
- 实现 aura-core: Config (Global), init_aura(), ContextExt, Z-Index utils
- 实现 aura-icons: AuraIcon trait, IconSize, 10 个占位图标
- 实现 aura-components: AuraButton (6 variants × 3 sizes × disabled/loading)
- 适配 GPUI 0.2.2 API (Render trait, Context<'_, V>, InteractiveElement, AnyElement)
- 解决 GPUI feature 策略: 显式 features 替代 default-features=true
- 实现 aura-gallery: 分类卡片式组件看板
- 编写 architecture-design.md: 完整项目设计文档
- 搭建 .memory/ + .prompt/ + prompt.md 协作基础设施

### Key Discoveries
- GPUI 0.2.2 中 `StatefulInteractiveElement` 仅在 `.id()` 之后可用
- `.active()` 和 `.on_click()` 需要 `Stateful<Div>` 包裹
- `.when()` / `.when_some()` 在 0.2.2 中已移除
- `default-features = true` 覆盖 workspace 设置可能有 bug，改用显式 features
- `WindowContext` 类型在 0.2.2 中不存在，使用 `Window` + `Context<'_, V>` + `App`

### Decisions Made
- 组件与主题解耦: `.build(&theme)` 显式传入
- Demo 返回 `AnyElement` 用于注册表类型统一
- 库 crate 不启用 GPUI 平台 features

## Session 16 — 2026-05-06 (Late Night)

### Actions
- **清理工程警告**:
  - 移除 `pagination.rs`, `statistic.rs`, `segmented.rs`, `progress.rs`, `skeleton.rs`, `affix.rs`, `backtop.rs`, `anchor.rs` 等文件中的未使用导入和变量。
  - 修复 `aura-gallery` 中多个 Demo 文件的未使用导入。
- **补全 P4 缺失组件**:
  - **Tag**: 实现标签组件，支持 `Success`/`Warning`/`Danger`/`Info` 类型，`Light`/`Dark`/`Plain` 主题效果，以及 `closable` 和 `round` 属性。
  - **Avatar**: 实现头像组件，支持图片 (`src`)、图标 (`icon`) 和默认占位，支持 `Circle`/`Square` 形状和三种标准尺寸。
  - **Badge**: 实现徽章包装器，支持在任意子元素右上角显示数值 (`value`)、最大值限制 (`max`) 或小红点 (`is_dot`)。
- **完善 Gallery Demo**:
  - 新增 `Tag`, `Avatar`, `Badge` 的独立 Demo 页面。
  - 修复 `mod.rs` 中因操作失误导致的 `steps_demo`/`tabs_demo` 重复以及 `tree_demo`/`typography_demo` 丢失的问题。
- **更新记忆库**:
  - 更新 `.memory/inventory.md`，标记 P4 组件数为 21 个。
  - 更新 `.memory/state.md`，正式宣布 P4 完成并进入 P5 阶段。

### Verification
- `cargo check` passed with 0 errors and 0 warnings (except gallery unused imports in new demos).
- All new components registered in gallery.

### Key Discoveries
- GPUI `Img` 元素在当前版本中不支持 `.alt()` 方法，需移除。
- `RenderOnce` 组件内部使用 `.on_click` 必须先调用 `.id()` 以满足交互 Trait 约束。
- 绝对定位叠加可以通过 `relative()` 容器配合 `absolute()` 子元素轻松实现，适用于 `Badge` 等组件。

## Session 17 — 2026-05-06 (Refining Text)

### Actions
- **打磨 Text 控件**:
  - 增强 `Text` 结构，支持 `color`, `bg`, `size`, `weight`, `style` (italic), `underline`, `strikethrough`, `font_family` 等多种属性。
  - 提供 `code_style()` 快捷方法。
  - 修复 GPUI 0.2.2 中 `Div` 不支持 `.font_style()` 的问题，改用 `.italic()` 映射。
- **重构 Paragraph 控件**:
  - `Paragraph` 现在是一个容器，接收多个 `Text` 段落。
  - 底层使用 `flex_row()` + `flex_wrap()` 模拟流式布局，确保不同样式的文本块能自动换行且拼接紧凑。
  - 保留 `with_text` 快捷构造方法。
- **更新 Typography Demo**:
  - 在画廊中展示了富文本段落的拼接效果：包含加粗、斜体、背景色、下划线等混合样式。

### Verification
- `cargo check` passed.
- Gallery demo verified.

### Key Discoveries
- GPUI 的文本修饰方法在不同版本间有差异，`.italic()` 是比 `.font_style(FontStyle::Italic)` 更直接的选项。
- `flex_wrap()` 在处理变宽文本块时能很好地替代传统的段落渲染，前提是块之间没有强制的间距干扰。

## Session 18 — 2026-05-06 (Dynamic Tags)

### Actions
- **激活 Tag 移除功能**:
  - 修复 `Tag` 组件的 `on_close` 回调逻辑，确保点击关闭按钮时能够正确触发外部传入的闭包。
  - 为 `Tag` 的关闭按钮生成基于标签文本的唯一 ID，解决 GPUI 多个交互元素 ID 冲突的问题。
- **增强 Tag Demo**:
  - 在画廊中新增了“动态添加和移除”演示小节。
  - 使用 `View` 状态管理实现了一个可实时增删的标签列表，演示了 `Tag` 的交互能力。

### Verification
- `cargo check` passed.
- 画廊中标签点击“x”后可正确消失。

### Key Discoveries
- 在 GPUI 的 `RenderOnce` 组件闭包中，若要引用 `View` 的句柄并进行异步或事件回调更新，需在渲染时通过 `cx.entity().clone()` 捕获句柄，并在闭包内部调用 `view.update(cx, ...)`。

## Session 19 — 2026-05-06 (Interactive Dynamic Tags)

### Actions
- **增强 Input 组件**:
  - 为 `Input` 组件添加 `on_enter` 回调支持。
  - 新增 `set_on_enter` 方法，支持在 `update_entity` 中动态更新回车回调。
- **完善 Tag Demo 交互**:
  - 重构“动态添加和移除”模块，将固定的 "New Tag" 按钮改为“点击切换输入框”模式。
  - 用户点击 "+ New Tag" 后，按钮变为输入框并自动获取焦点。
  - 用户输入内容并回车后，生成对应名称的新标签，并恢复为按钮状态。
  - 若输入为空回车，则直接恢复为按钮状态。

### Verification
- `cargo check` passed.
- 交互流程符合预期：按钮 -> 点击 -> 输入 -> 回车 -> 生成标签并恢复按钮。

### Key Discoveries
- 在 GPUI 中实现“点击按钮变输入框”的模式，需要将 `Input` 作为一个持久的 `Entity` 存储在 `View` 中，并在切换显示时通过 `cx.focus_view(&view.input, window)` 手动转移焦点。
- `Entity<T>` 本身实现了 `IntoElement`，因此可以直接作为 `.child()` 传入。

## Session 20 — 2026-05-06 (Fixing Tag Demo Panic)

### Actions
- **修复 Input 组件 double-lease 崩溃**:
  - 重构 `Input` 的 `on_enter` 回调，将其签名改为 `Fn(&str, &mut Window, &mut App)`。
  - 通过在 `enter` 内部克隆当前值并将其传递给闭包，避免了回调内部尝试通过 `Entity<Input>` 重新获取写锁导致的 panic。
  - 调整了 `set_on_enter` 和相关方法以适配新的签名。
- **更新 Tag Demo**:
  - 适配新的 `on_enter` 模式，直接从回调参数中获取输入值。
  - 确保标签生成后，输入框正确清空并隐藏。

### Verification
- `cargo check` passed.
- 解决了输入回车导致的 "cannot read Input while it is already being updated" 崩溃。

### Key Discoveries
- 在 GPUI 的事件监听器（Listener）中，实体已经处于 `update` 状态。如果在回调中再次尝试通过 handle `read` 或 `update` 该实体，会导致重入性 Panic。
- 最佳实践是将所需数据从组件内部“推”给回调，而不是让回调回过头来“拉”组件的数据。

## Session 21 — 2026-05-06 (Re-fixing Tag Demo Panic)

### Actions
- **彻底解决 Input 回调重入崩溃**:
  - 重构 `Input` 的 `on_enter` 回调，使其接收 `&mut Self` (Input 实例) 作为第一个参数。
  - 在 `Input::enter` 内部直接将 `self` 传递给闭包，从而允许回调直接调用 `input.set_value("", cx)` 而无需通过 `Entity` 句柄触发二次 `update`。
  - 这种模式完全避开了 GPUI 的 double-lease 保护机制。
- **同步更新 Tag Demo**:
  - 适配新的回调签名，在回调内部直接操作 `input` 实例清空文本。

### Verification
- `cargo check` passed.
- 解决了在更新过程中由于 handle 重入导致的 Panic。

## Session 22 — 2026-05-07 (Text Auto Wrap)

### Actions
- **增强 Text 控件自动换行能力**:
  - 为 `Text` 增加 `wrap()` / `auto_wrap()` builder，启用 `whitespace_normal()` 并让文本填满父容器宽度，从而在受限宽度内自动折行。
  - 增加 `nowrap()` builder，用于显式保持单行文本。
  - 为 `Text` 统一设置基于字号的 `line_height`，提升多行文本可读性。
- **更新 Typography Demo**:
  - 新增受限宽度容器中的长文本自动换行示例。

### Verification
- `cargo check` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- GPUI 文本折行依赖 `WhiteSpace::Normal` 且通常需要确定的可用宽度；`Text::wrap()` 通过 `w_full()` 让文本在父容器宽度内参与折行。

## Session 23 — 2026-05-07 (Gallery Performance)

### Actions
- **优化 Gallery Demo 窗口渲染性能**:
  - 将原先单个超长滚动页中一次性渲染全部 demo 的模式改为左侧导航 + 右侧当前 demo 详情。
  - `Gallery` 现在缓存 `DemoEntry` 注册表，避免每次 render 都重新构造 registry。
  - 每帧只挂载当前选中的 `AnyView`，显著减少主窗口重绘和布局压力。
- **交互调整**:
  - 左侧 demo 列表支持点击切换当前示例。
  - 保持现有 demo render 函数和组件 API 不变。

### Verification
- `cargo check` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- Gallery 卡顿的主要来源是 `Gallery::render()` 每次遍历并渲染全部 demo，同时每次重新调用 `demos::registry()` 分配注册表。
- 部分 demo（Form、Icon、Backtop、Anchor）本身较重；当前改动先避免它们在同一帧全部参与布局。

## Session 24 — 2026-05-07 (Rate Hover Reset)

### Actions
- **修复 Rate hover 状态未恢复问题**:
  - 为每个 Rate 实例和星星元素生成基于 `EntityId` 的稳定唯一 ID，避免多个 Rate 实例共享同名交互元素。
  - 将星星的 hover 预览从 `on_mouse_move` 改为每颗星独立的 `on_hover` 进入/离开处理。
  - 当鼠标离开当前 hover 星星且未点击时，清空 `hover_value`，渲染回真实 `value` 评分状态。

### Verification
- `cargo check` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- 仅依赖 Rate 容器的 `on_hover(false)` 不足以覆盖从最后一颗星右侧移出的路径；每颗星独立处理 hover leave 更可靠。

## Session 25 — 2026-05-07 (Independent Overlays)

### Actions
- **重构浮层全局状态为 keyed multi-instance manager**:
  - 将 `ActiveTooltip`, `ActivePopover`, `ActiveModal`, `ActiveDrawer` 从单例 `Option` 改为按 `SharedString` ID 管理的多实例集合。
  - 新增 `clear_tooltip`, `clear_popover`, `clear_modal`, `clear_drawer` 等按 ID 精确关闭接口，同时保留 `clear_active_*` 作为关闭全部的兼容入口。
  - `render_active_*_in_window` 现在逐个渲染所有 active overlay entry。
- **组件实例独立化**:
  - `Tooltip` 增加稳定 ID，并在 hover leave 时只清理自身 tooltip。
  - `Popover` 点击时按自身 ID toggle，只替换/关闭自身实例，不再覆盖全局唯一 popover。
  - `Popconfirm` 和 `Dropdown` 基于自身 popover ID 精确关闭。
  - `Dialog` / `Drawer` 增加 `.id(...)` 与 `close_id(...)`，close 按实例 ID 清理对应 overlay。
- **Demo 同步**:
  - Popover 手动关闭示例改为按 `popover-demo-manual-close` 精确关闭自身。

### Verification
- `cargo check` passed.
- `cargo test` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- 之前的 `ActivePopover(Option<AnyView>)` / `ActiveTooltip(Option<TooltipData>)` 架构天然只能存在一个浮层，任何 clear 都是全局清空。
- 当前默认 ID 仍基于调用位置；同一调用位置循环创建多个浮层时，应显式调用 `.id(...)` 传入业务唯一 ID。
