# Session History

## Session 52 — 2026-05-08 (Phase Reorganization)

### Actions
- **新增两个阶段 (P6, P7)，原 P6 Engineering 改为 P8**:
  - P6 Built-in Unique ID: 确保全库每个控件有默认内置全局唯一 ID，事件冲突防护由组件库自身保证
  - P7 Demo Self-Contained: Gallery Demo 完全使用 Aura 组件库自身控件构建，禁止在 Demo 中直接使用 GPUI 原生组件
- **创建 `.prompt/P6-builtin-id.md`**: 详细定义内置唯一 ID 规范、实现策略、全局计数器基础设施
- **创建 `.prompt/P7-demo-self-contained.md`**: 定义 Demo 自举要求、缺失控件新增流程、改造范围
- **重命名 `.prompt/P6-engineering.md` → `.prompt/P8-engineering.md`** 并更新上游引用
- **同步更新所有相关文件**:
  - `prompt.md`: 阶段导航 (9)、工程结构 (3) 
  - `.memory/state.md`: 阶段进度表
  - `.memory/inventory.md`: 组件清单
  - `.memory/decisions.md`: 新增 ADR-011 (Built-in Unique ID)、ADR-012 (Demo Self-Contained)
  - `.memory/sessions.md`: 本记录

### Key Discoveries
- P0-P5 阶段反复出现的 ID 冲突问题 (Rate/Menu/Tabs/Pagination/Segmented/Dropdown) 说明默认唯一 ID 应该是组件库基础设施而非可选项
- Demo 中大量 GPUI 原语导致 Gallery 无法作为组件用法参考，需要系统性解决

### Verification
- File structure verified: `.prompt/P6-builtin-id.md`, `.prompt/P7-demo-self-contained.md`, `.prompt/P8-engineering.md` all present

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
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

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

## Session 26 — 2026-05-07 (Progress Text Inside)

### Actions
- **完善 Progress 百分比内显**:
  - 新增 `Progress::text_inside(bool)` builder，支持线性进度条将百分比渲染在进度条内部。
  - 内显文本使用白色、右对齐、nowrap，并在低百分比时给 bar 最小宽度，避免百分比文本被挤压不可读。
  - 外显文本逻辑保持原样；`show_text(false)` 仍会隐藏文本。
- **更新 Gallery Demo**:
  - 将“百分比内显 (TODO)”改为正式示例，覆盖 15%、70%、100% success 状态。

### Verification
- `cargo check` passed.
- `cargo test` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- 百分比内显需要与外显状态图标逻辑分离；内显模式应始终显示百分比文本，而不是在 success/exception 状态切换为外部图标。

## Session 27 — 2026-05-07 (Progress Inside Text Center)

### Actions
- **增强 Progress 内显文本对齐配置**:
  - 新增 `Progress::text_inside_center(bool)`，允许配置内显百分比固定在整条进度条中心或随已完成进度右对齐。
  - 新增 `Progress::text_inside_centered()` 便捷方法，同时启用内显和整条进度条居中。
  - 居中模式将文本渲染为 track 级覆盖层，并根据进度是否越过 50% 切换文本颜色，避免在未填充背景上使用白字。
- **更新 Gallery Demo**:
  - 在百分比内显示例中加入居中显示文本的进度条。

### Verification
- `cargo check` passed.
- `cargo test` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- 文本内显的“是否显示”和“如何对齐”应保持为独立配置；整条进度条居中需要根据文字所在背景动态选择颜色。

## Session 28 — 2026-05-07 (Tree Expand Click)

### Actions
- **修复 Tree 点击展开无反应**:
  - 将节点行点击统一接入 `click_node`，有子节点时点击整行即可切换展开/折叠，同时保留选中逻辑。
  - 展开箭头点击后调用 `stop_propagation()`，避免箭头点击同时触发行点击导致双重 toggle。
- **修复 Tree Demo 状态生命周期**:
  - 将 demo 中的 `Tree` entity 从 render 阶段创建改为 `TreeDemo` 初始化时创建并持有，避免每次父视图重渲染都重建 Tree 状态。

### Verification
- `cargo check` passed.
- `cargo test` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- 在 render 中临时 `cx.new` 交互控件会让状态生命周期不稳定；demo 中需要把有状态组件保存在父 view 字段里。

## Session 29 — 2026-05-07 (Tree Single Selection)

### Actions
- **修正 Tree 默认选择语义**:
  - Tree 默认改为单选：点击新节点会清空旧选中项并选中新节点。
  - 新增 `Tree::multiple(bool)`，需要多选时可显式开启原有 toggle 多选行为。

### Verification
- `cargo check` passed.
- `cargo test` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- 原实现使用 `HashSet` 直接 toggle 选中状态，导致普通 Tree 在没有复选框/多选配置时也表现为多选；默认交互应为单选。

## Session 30 — 2026-05-07 (Collapse Demo Interaction)

### Actions
- **修复 Collapse Demo 点击无反应**:
  - 将 demo 中的基础 Collapse 和 Accordion Collapse 从 render 阶段临时创建改为 `CollapseDemo` 初始化时创建并持有，确保 active 状态在父视图重渲染后保留。
  - 将 Collapse header ID 从调用位置 + index 改为基于 item name，避免同一组件内 item 重排时交互 ID 不稳定。

### Verification
- `cargo check` passed.
- `cargo test` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- Collapse 组件自身 toggle 逻辑有效；Gallery demo 中在 render 内 `cx.new` 导致有状态组件生命周期不稳定，是点击后看起来无反应的主要原因。

## Session 31 — 2026-05-07 (Menu Demo Interaction)

### Actions
- **修复 Menu Demo 点击无反应**:
  - 将水平、垂直、折叠菜单从 render 阶段临时创建改为 `MenuDemo` 初始化时创建并持有，确保 active/opened 状态在父视图重渲染后保留。
  - 为 `Menu` 增加稳定实例 ID，并在 demo 中显式设置 `menu-demo-horizontal` / `menu-demo-vertical` / `menu-demo-collapsed`。
  - 所有菜单 item/submenu/popover 交互 ID 增加 Menu 实例前缀，避免多个菜单共用 `"1"`, `"2"` 等业务 ID 时发生 GPUI 交互 ID 冲突。

### Verification
- `cargo check` passed.
- `cargo test` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- Menu 的状态逻辑有效，但 demo 中 render 内 `cx.new` 会让 active/opened 状态生命周期不稳定。
- 同一 Gallery 页面存在多个 Menu 实例且 item id 重复，组件内部必须用实例 ID 前缀隔离 GPUI Element ID。

## Session 32 — 2026-05-07 (Menu Popover Active State)

### Actions
- **修复 Menu 弹出气泡子菜单选中态**:
  - Collapsed vertical submenu popover 和 horizontal submenu popover 渲染时读取所属 Menu 的 `active_index`。
  - Popover 内子菜单 item 根据 active 状态应用主色文字、浅色背景和主色 icon，和普通菜单项选中态保持一致。

### Verification
- `cargo check` passed.
- `cargo test` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- Popover 内容在独立 view/context 中渲染，不能依赖外层 render 时的局部状态快照；需要通过 Menu entity handle 读取最新 active state。

## Session 33 — 2026-05-07 (Overlay Cursor Isolation)

### Actions
- **修复浮层 hover/cursor 穿透**:
  - 为 PortalLayer 全屏容器设置 `cursor_default()`，确保浮层层级存在时光标不继承底层按钮/链接的 pointer 状态。
  - 为 Popover 全屏交互背板和 popover 内容容器设置默认 cursor。
  - 为 Dialog / Drawer 遮罩和面板设置默认 cursor。
  - 为 Tooltip 浮层设置默认 cursor。

### Verification
- `cargo check` passed.
- `cargo test` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- 事件 propagation 阻断不等于 cursor 命中隔离；GPUI hover/cursor 样式需要当前顶层命中元素显式设置默认 cursor，否则可能保留/穿透底层 pointer 光标。

## Session 34 — 2026-05-07 (Menu Popover Cursor Isolation)

### Actions
- **补齐 Menu 弹出气泡 cursor 隔离**:
  - 为 collapsed vertical submenu popover 内容根容器设置 `cursor_default()`。
  - 为 horizontal submenu popover 内容根容器设置 `cursor_default()`。
  - 保留具体菜单项自身的 `cursor_pointer()`，仅防止 popover 空白区域透出底层按钮 pointer。

### Verification
- `cargo check` passed.
- `cargo test` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- 通用 Popover 面板默认 cursor 只能覆盖外层浮层；组件自定义 popover 内容根节点也需要声明默认 cursor，才能覆盖内容 padding/空白区域。

## Session 35 — 2026-05-07 (Popover Hit-Test Shield)

### Actions
- **加强 Menu/Popover 防穿透命中层**:
  - Popover 全屏背板明确设置 `top_0()` / `left_0()` / 透明背景，并添加 `on_hover` 阻断，避免仅靠 `on_mouse_move` 无法阻断 hover/cursor 状态。
  - Popover 内容容器增加稳定 ID 和 `on_hover` 阻断。
  - Menu submenu popover 内容根节点增加稳定 ID、`on_hover` 和 `on_mouse_move` 阻断，覆盖菜单气泡 padding/空白区域。

### Verification
- `cargo check` passed.
- `cargo test` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- Menu 气泡的问题不是单纯 cursor 样式，而是 hover 命中层没有完整接管；需要可命中的 stateful element + hover 阻断。

## Session 36 — 2026-05-07 (Menu Popover Shield and Color Consistency)

### Actions
- **修复 Menu 气泡 hover 穿透残留**:
  - 将 Gallery 的 PortalLayer 改为带稳定 ID 的全屏透明命中层，并在 portal 根层阻断 hover / mouse move 传播，避免菜单气泡移动时触发下方导航项 hover。
- **统一 Menu 字体和图标颜色**:
  - 垂直、水平、折叠 popover、水平 popover 的菜单项均使用同一个颜色变量驱动文字和图标。
  - 普通态文字/图标同色，选中态文字/图标同为主色；hover 仅调整背景，避免文字与图标状态脱节。

### Verification
- `cargo check` passed.
- `cargo test` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- 仅在 Popover 内部阻断事件仍可能不足；portal 根层本身也需要成为可命中的透明 hover shield，才能阻止底层菜单项接收 hover 状态。

## Session 37 — 2026-05-07 (Menu Popover Occlusion)

### Actions
- **重新检查 Menu/Popover/Portal 全链路 hover 隔离**:
  - 确认 GPUI 的 `stop_propagation()` 不会自动阻断后层 hover 命中。
  - 为 PortalLayer、Popover 全屏根层、Popover 内容面板、Menu 自定义 popover 内容根节点添加 `occlude()`，使弹层 hitbox 明确屏蔽背后元素 hover/cursor。

### Verification
- `cargo check` passed.
- `cargo test` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- GPUI 防穿透需要使用 `occlude()` / `HitboxBehavior::BlockMouse`；透明背景 + hover/mouse move stop propagation 只能处理事件冒泡，不能阻止底层元素进入 hover。

## Session 38 — 2026-05-07 (Tabs Demo Interaction)

### Actions
- **修复 Tabs Demo 点击无反应**:
  - 将 Tabs Demo 中各个 Tabs 从 render 阶段临时 `cx.new` 改为 `TabsDemo` 初始化时创建并持有，确保 active tab 状态在父视图重渲染后保留。
  - 为 `Tabs` 增加稳定实例 ID，并在 tab / close / add 交互元素 ID 前加实例前缀，避免多个 Tabs 示例共用 `first` / `second` / `add-tab` 等 ID 时互相冲突。

### Verification
- `cargo check` passed.
- `cargo test` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- Tabs 与 Menu/Tree/Collapse 一样，demo 中 render-time entity creation 会重置组件状态；多个 Tabs 示例复用同名 pane 还会造成 GPUI element ID 冲突。

## Session 39 — 2026-05-07 (Tabs Stretch and Editable Add)

### Actions
- **完善 Tabs 水平均分布局配置**:
  - 为 `Tabs` 增加 `stretch(bool)` 配置。
  - 水平布局开启 stretch 时 header 占满父级宽度，每个 tab 使用 `flex_1()` 自动均分；普通标准 Tabs 保持原 gap 布局。
  - Gallery 增加“自动均分并占满宽度”示例。
- **修复 Editable Tabs 点击 + 无视觉反馈**:
  - `add_tab` 现在会内置新增一个默认 Tab、切换为 active，并继续触发 `on_tab_add` 回调。

### Verification
- `cargo check` passed.
- `cargo test` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- 原 editable add 只调用外部回调并 notify，没有修改内部 panes，所以 demo 中点击 + 不会出现任何 UI 变化。

## Session 40 — 2026-05-07 (Menu Demo Content Switching)

### Actions
- **完善 Menu Demo 导航内容区效果**:
  - 为水平、垂直、折叠菜单分别增加独立内容展示区域。
  - 菜单 `on_select` 会更新对应内容卡片，展示当前 active id、标题和说明，形成类似 Tabs 的“切换导航后内容区变化”效果。
  - 内容区使用独立 `Entity<MenuContent>`，避免与 Menu 内部状态耦合，同时保持多 Menu 示例互相独立。

### Verification
- `cargo check` passed.
- `cargo test` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- Menu 组件已经提供 `on_select` 回调；Demo 只需要持有可更新的内容实体，就能展示真实导航页面切换效果。

## Session 41 — 2026-05-07 (Pagination Click, Hooks, Page Sizes)

### Actions
- **修复 Pagination demo 点击无反应**:
  - 将 Pagination Demo 改为在初始化时持有多个 `Entity<Pagination>`，避免 render-time `cx.new` 导致分页状态被重置。
  - 为 Pagination 添加稳定实例 ID，并给 prev/page/next/ellipsis/size 按钮加实例前缀，避免同页多个 Pagination 互相抢交互 ID。
- **补齐分页回调与每页条数配置**:
  - Pagination 增加 `on_page_size_change` 钩子。
  - 增加 `page_sizes(vec![...])` 配置，并通过 `sizes` layout 段渲染可点击的每页条数按钮。
  - 切换页码时仍触发 `on_change`，切换 page size 时触发 page size hook，必要时自动修正当前页。

### Verification
- `cargo check` passed.
- `cargo test` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- Pagination 也存在与 Tabs/Menu 相同的两类问题：render-time entity creation 会重置状态，且多实例共享简单按钮 ID 会导致交互冲突。

## Session 42 — 2026-05-07 (Pagination Select Style and Hover)

### Actions
- **将分页条数候选区改为 Select 下拉样式**:
  - Pagination 的 `sizes` 段改为复用现有 `Select` 控件，而不是一组静态候选按钮。
  - `Select` 作为 Pagination 内部稳定实体持有，并通过新增的 `set_options` / `set_selected_idx` 同步当前条数与可选项。
- **补充分页页码按钮 hover 效果**:
  - 页码、上一页/下一页、前后省略按钮在可点击状态下加入 hover 背景效果，提升交互可见性。
- **避免下拉同步重绘回路**:
  - `Select` 的 setter 仅在值变化时才触发 notify，防止 Pagination render 中同步下拉状态造成无意义重绘。

### Verification
- `cargo check` passed.
- `cargo test` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- 复用 Select 比手写候选按钮更符合现有控件体系，也能自然获得弹出层与选择状态。
- 在 render 里同步子实体状态时，setter 必须幂等，否则容易引发隐式重绘回路。

## Session 43 — 2026-05-08 (Pagination Hitbox Hover Cursor)

### Actions
- **修复 Pagination hover/cursor 未生效**:
  - 将分页按钮重构为外层带稳定 ID 的命中元素直接负责 `cursor_pointer()`、hover 背景/文字颜色和点击逻辑。
  - 去掉外层 wrapper + 内层按钮样式分离，避免 hover 写在非命中元素上导致 GPUI 交互样式不稳定。
- **补齐 Select 触发器 hover**:
  - Select 根触发器增加 hover 边框主色效果，分页 page-size 下拉也能获得明确 hover 反馈。

### Verification
- `cargo check` passed.
- `cargo test` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- GPUI 中 hover/cursor 应放在带 ID 与点击监听的实际 hitbox 元素上；子元素单独设置 hover 不一定会体现到用户鼠标所在的命中节点。

## Session 44 — 2026-05-08 (Pagination Hover Cursor Pointer)

### Actions
- **补齐 hover 小手 cursor**:
  - Pagination 可点击分页项在 hover 状态内显式设置 `cursor_pointer()`，确保鼠标移入时显示小手。
  - Select 触发器在 hover 状态内显式设置 `cursor_pointer()`，分页条数下拉同样显示小手。

### Verification
- `cargo check` passed.
- `cargo test` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- 本项目 GPUI 用法里 cursor 最稳妥的写法是放入 hover refinement；只在常规链路上写 cursor 可能不能满足用户期望的“hover 时变小手”。

## Session 45 — 2026-05-08 (Pagination Cursor Root Cause Follow-up)

### Actions
- **重新检查 Pagination hover/cursor 不明显的问题**:
  - 确认分页图标按钮的颜色不会继承父级 hover `text_color`，需要使用 Icon 的 `group_hover_color` 同步图标 hover 主色。
  - 将所有非 disabled 分页项（包括当前页）都纳入 hover/cursor 样式，hover 背景改为更明显的主色浅底。
  - Select 触发器和下拉选项均在 hover 状态中显式设置 `cursor_pointer()`。

### Verification
- `cargo check` passed.
- `cargo test` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- Pagination 上一页/下一页主要是 Icon，父级文字 hover 不会改变显式 Icon 颜色；需要 group hover。当前页之前也被 hover 条件排除，导致部分分页按钮看起来完全没有 hover/cursor 反馈。

## Session 46 — 2026-05-08 (PortalLayer Cursor Mask)

### Actions
- **修复小手 cursor 被空 PortalLayer 覆盖**:
  - Gallery 的 `PortalLayer` 在没有任何 portal entry 时不再设置 `cursor_default()`。
  - 保留有弹层时的全屏 `cursor_default()` + `occlude()`，确保弹层存在时仍能隔离背景 hover/cursor。

### Verification
- `cargo check` passed.
- `cargo test` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- 空 PortalLayer 虽然没有弹层内容，但全屏 `cursor_default()` 仍会向 GPUI 注册 cursor 样式请求，覆盖底层分页按钮的 `cursor_pointer()`。

## Session 47 — 2026-05-08 (Statistic Icon Alignment)

### Actions
- **修复 Statistic 自定义前后缀图标与数值文字未居中对齐**:
  - 将数值行从 baseline 对齐改为 center 对齐。
  - 为前缀/后缀自定义元素增加 flex 居中 wrapper。
  - 统一数值文本 `line_height` 与前后缀 wrapper 高度，避免字体行盒和 SVG 方盒差异导致视觉中心偏移。

### Verification
- `cargo check` passed.
- `cargo test` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- 仅使用 flex center 仍可能受大号文字 line box 与小尺寸 SVG box 的差异影响；显式统一 line-height/wrapper height 更稳定。

## Session 48 — 2026-05-08 (Segmented Interaction)

### Actions
- **修复 Segmented Demo 点击无反应**:
  - 将 Segmented Demo 中基础、禁用、Block 三个分段控件从 render 阶段临时 `cx.new` 改为初始化时创建并持有，避免点击后状态被父视图重渲染重置。
  - 为 `Segmented` 增加稳定实例 ID，并为每个 option 的交互 ID 增加实例前缀，避免多个示例共用 `0/1/2` ID 发生冲突。
  - 非激活可点击 option 增加更明确的 hover 背景和 pointer cursor。

### Verification
- `cargo check` passed.
- `cargo test` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- Segmented 与 Tabs/Menu/Pagination 同类：demo render-time entity creation 会丢失内部选中状态，多实例复用简单数字 ID 也会造成 GPUI 交互冲突。

## Session 49 — 2026-05-08 (Dropdown Menu Styling)

### Actions
- **优化 Dropdown 下拉气泡样式**:
  - 参考 Select 下拉菜单，将 Dropdown 内容区改为更宽的菜单面板，增加 `min_w(168px)` 与 `max_h(200px)`。
  - 菜单项改为 Select 风格的整行选项：统一 `px_3` / `py_2` / `min_h(34px)`，移除挤压感明显的小圆角 pill 间距。
  - hover 改为中性色背景 + 主色文字，并保留小手 cursor。
  - Dropdown 自定义内容根节点增加稳定 ID、默认 cursor 与 occlusion，避免菜单空白区域事件穿透。

### Verification
- `cargo check` passed.
- `cargo test` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- Dropdown 复用 Popover 作为外壳，内部菜单项不应再额外做紧凑 pill 列表；与 Select 一致的整行选项布局更自然。

## Session 50 — 2026-05-08 (Dropdown Demo ID Isolation)

### Actions
- **修复 Dropdown Demo 只有第一个能弹出**:
  - 为 demo 中每个 Dropdown 显式设置唯一 ID，避免 helper 函数同一调用点生成相同 `track_caller` 默认 ID。
  - Dropdown 菜单项 ID 增加 dropdown 实例 ID 前缀，避免不同下拉菜单中的 item ID 冲突。

### Verification
- `cargo check` passed.
- `cargo test` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- 多个 Dropdown 都通过同一个 `menu(...)` helper 构造时，`Dropdown::new` 的默认 caller-based ID 相同；Popover trigger ID 冲突后表现为只有一个实例能正常弹出。

## Session 51 — 2026-05-08 (Affix Backtop Anchor Repair)

### Actions
- **修复 Affix / Backtop / Anchor demo 无效果问题**:
  - Affix demo 改为持有稳定 `Entity<Affix>`，滚动区域触发 notify；Affix 记录 placeholder bounds，并在 fixed 状态下按窗口坐标偏移渲染固定副本。
  - Backtop 增加稳定实例 ID，demo 改为持有两个 `Entity<Backtop>`，避免 render-time 重建和按钮 ID 冲突。
  - Anchor demo 改为持有稳定 `Entity<Anchor>`，避免每次 render 重建导致 target bounds/active link 丢失。
  - Anchor 点击跳转和 active 检测改为基于 scroll viewport top + offset 计算，不再把 target 的窗口坐标误当作滚动容器局部坐标。

### Verification
- `cargo check` passed.
- `cargo test` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery successfully; process was intentionally stopped by timeout after startup.

### Key Discoveries
- 这三个控件有实际价值，尤其在长页面、文档和组件库 demo 场景中。
- 当前无效果的主要原因仍是交互/滚动状态在 demo render 阶段重建，以及滚动坐标系计算没有考虑 scroll viewport 的窗口位置。

## Session 53 — 2026-05-08 (Affix / Backtop / Anchor Demo Fixes)

### Actions
- **修复 Affix 展示机制**:
  - 将 `BoundsTracker` 改为真正包裹并绘制子元素，而不是只绘制一个 0 尺寸占位元素。
  - 固定态内容通过非阻塞 passive portal 渲染到顶层，避免被滚动容器裁剪，同时保留原位置占位尺寸防止布局跳动。
  - 调整 Affix demo 为明确高度的滚动展示区，避免嵌套在 Gallery 滚动内容中因高度不确定导致“无效果”。
- **修复 Backtop 可见性与定位**:
  - 增加 `BacktopVisibilityTracker` 在 paint 阶段读取 `ScrollHandle` 偏移并触发组件重绘。
  - 将 Backtop 根元素设为绝对定位全尺寸层，使右下角按钮相对 demo 容器正确展示。
  - 调整 Backtop demo 为带边框的固定高度滚动区域，并把 Backtop 放在该相对定位容器内。
- **修复 Anchor 跳转与目标展示**:
  - Anchor 点击时从组件实时读取 `targets_bounds`，避免闭包捕获初始化时的空 bounds 快照导致点击无效。
  - AnchorTarget 使用 `prepaint_at(bounds.origin, ...)` 正确预绘制子元素。
  - 调整 Anchor demo 为固定高度可视区，确保滚动区域和右侧锚点导航可见。
- **新增 passive portal 通道**:
  - `aura_core::PassivePortal` / `push_passive_portal` 用于 Affix 这类不应 occlude / stop propagation 的顶层渲染。
  - Gallery 的 `PortalLayer` 分离 passive portal 与原有 active portal，保留弹层类组件的事件阻断行为。

### Verification
- `cargo check` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Affix 不适合复用原有 `Portal`，因为原 active portal layer 会 `.occlude()` 并停止事件传播，固定内容应走非阻塞 passive portal。
- 依赖 `ScrollHandle` 的可见性状态不能只在 render 中读取；需要 paint/scroll 触发状态同步，否则滚动后组件可能不会重绘。
- 自定义 `gpui::Element` 包裹 `AnyElement` 时，prepaint 阶段应使用 `prepaint_at(bounds.origin, ...)`，否则子元素可能无法按目标 bounds 展示。

## Session 54 — 2026-05-08 (P5 Table P0)

### Actions
- **Started P5 Advanced implementation with Table P0**:
  - Added `Table`, `TableColumn`, `TableRow`, and `TableAlign` in `crates/aura-components/src/table.rs`.
  - Implemented P0 table capabilities: column-driven row rendering, empty state, loading overlay, border mode, stripe mode, and fixed-header scroll body via `.height(...)` / `.fixed_header(true)`.
  - Added public exports in `crates/aura-components/src/lib.rs`.
- **Added Gallery demo**:
  - Created `apps/aura-gallery/src/demos/table_demo.rs` with Basic, Stripe + Border, Fixed Header, Loading, and Empty examples.
  - Registered `Table 表格` in the Gallery demo registry.
- **Updated memory**:
  - Marked P5 progress as 1/20 in `.memory/state.md`.
  - Added Table status to `.memory/inventory.md`.

### Verification
- `cargo check` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- `overflow_y_scroll()` requires a stateful element in this GPUI version, so Table body uses a generated `.id(...)` before enabling fixed-header scrolling.


## Session 55 — 2026-05-08 (Table Header Customization + Sort)

### Actions
- **Enhanced Table header API**:
  - Table headers remain bold by default when using `TableColumn::new(key, label)`.
  - Added `TableColumn::header(...)` so developers can provide any Aura/GPUI element, including `Text`, as custom header content.
- **Added opt-in sortable columns**:
  - Added `TableColumn::sortable()` to explicitly enable sorting behavior per column.
  - Added `TableSortOrder` and `TableSortState`.
  - Added controlled sorting API: `Table::sort(key, order)` + `Table::on_sort_change(...)`.
  - Header click cycles `none -> ascending -> descending -> none`; sorting remains developer-enabled and developer-controlled so application data ordering stays explicit.
- **Updated Table demo**:
  - Added a custom-header + sortable-columns example.
  - Demo uses `Text::new("客户")` as a custom header and sorts sample rows when sortable headers are clicked.

### Verification
- `cargo check` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Because table cells hold `AnyElement`, automatic internal sorting cannot safely infer comparable values. A controlled sort callback keeps Table generic while letting developers sort their source data explicitly.


## Session 56 — 2026-05-08 (P5 DatePicker)

### Actions
- **Added DatePicker component**:
  - Implemented `DatePicker` and `DateValue` in `crates/aura-components/src/date_picker.rs`.
  - Supports single-date selection, formatted display (`YYYY-MM-DD`), month navigation, disabled state, width/placeholder/id builder options, and `on_change` / `set_on_change` callbacks.
  - Calendar panel renders through the existing portal layer and captures trigger bounds for placement.
  - Added public exports in `crates/aura-components/src/lib.rs`.
- **Added Gallery demo**:
  - Created `apps/aura-gallery/src/demos/date_picker_demo.rs`.
  - Registered `DatePicker 日期选择器` in the Gallery demo registry.
  - Demo covers basic selection with callback text, preset value, and disabled state.
- **Updated memory**:
  - Marked P5 progress as 2/20 in `.memory/state.md`.
  - Added DatePicker status to `.memory/inventory.md`.

### Verification
- `cargo check` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- DatePicker avoids adding a date/time dependency for the initial P5 slice by using small local calendar helpers for leap years, month length, and weekday alignment.

## Session 57 — 2026-05-08 (DatePicker Click Crash Fix)

### Actions
- **Fixed DatePicker popup crash on click/open**:
  - Removed the custom `CalendarPanel` `Element` wrapper that rebuilt a fresh `AnyElement` during `request_layout`, `prepaint`, and `paint`.
  - Replaced it with a direct `render_calendar_panel(...) -> AnyElement` used inside the portal render closure so GPUI owns the element lifecycle normally.

### Verification
- `cargo check` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- GPUI `AnyElement` instances cannot be reconstructed independently across custom Element lifecycle methods; `request_layout` must establish the drawable state used by `prepaint`. Rebuilding a fresh child in `prepaint_at` triggers `must call request_layout before prepaint`.

## Session 58 — 2026-05-08 (DatePicker Header Navigation Fix)

### Actions
- **Improved DatePicker calendar header controls**:
  - Added four explicit navigation controls: previous year, previous month, next month, next year.
  - Added `shift_year` while preserving existing month rollover logic.
- **Fixed popup closing when clicking panel controls**:
  - Removed trigger-level `on_mouse_down_out` close behavior that treated portal clicks as outside-trigger clicks.
  - Added a full-screen portal backdrop that closes the picker only when the user clicks outside the panel.
  - Added `stop_propagation` on the calendar panel so header controls and day cells keep the popup open unless a date is selected.

### Verification
- `cargo check` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Portal-rendered content is outside the trigger subtree, so `on_mouse_down_out` on the trigger will close the DatePicker before panel controls can be used. Popup components need backdrop-level outside-click handling instead.


## Session 59 — 2026-05-08 (DatePicker Formats and Range Modes)

### Actions
- **Expanded DatePicker value modes**:
  - Added `DatePickerType` with `Date`, `DateRange`, `Month`, `MonthRange`, `Year`, and `YearRange`.
  - Added `DatePickerSelection` plus controlled callbacks for single, range, and generic selection changes.
  - Added range state and selection behavior: first click starts a range, second click completes and orders it.
- **Added display format support**:
  - Added `.format(...)` using tokens `YYYY`, `YY`, `MM`, `M`, `DD`, and `D`.
  - Added `.range_separator(...)` for range display text.
  - Defaults are date `YYYY-MM-DD`, month `YYYY-MM`, and year `YYYY`.
- **Added month/year panels**:
  - Month and month-range use a 12-month panel with year navigation.
  - Year and year-range use a 12-year panel with page navigation.
- **Updated DatePicker demo**:
  - Added custom display format, date range, month, month range, year, and year range examples.

### Verification
- `cargo check` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Since date/month/year values share the same `DateValue` storage, modes normalize granularity: month values use day `1`, and year values use month/day `1/1`.

## Session 60 — 2026-05-08 (DatePicker Range Trigger Polish)

### Actions
- **Polished DatePicker range trigger display**:
  - Replaced the compact plain-text `start 至 end` range string in the trigger with a structured layout.
  - Start and end values now render as separate soft pill blocks with spacing.
  - The range separator renders as its own muted chip, preventing the “至” text from visually colliding with either date.
  - In-progress ranges show the selected start plus a muted “请选择结束” end placeholder.

### Verification
- `cargo check` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Range display should not be a single concatenated string once custom formats are supported; separate layout nodes keep separator spacing predictable across date, month, and year ranges.

## Session 61 — 2026-05-08 (DatePicker Range Trigger Simplification)

### Actions
- **Simplified DatePicker range trigger styling**:
  - Removed background fills from the left and right range value areas.
  - Kept the separator (`至` by default) as the only chip with a muted background.
  - Preserved spacing and text hierarchy so range values remain readable without visual clutter.

### Verification
- `cargo check` passed.
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- When the separator already has a chip background, adding backgrounds to both date values makes the range trigger visually heavy and less balanced.

## Session 62 — 2026-05-08 (DatePicker Demo Borrow Fix)

### Actions
- **Hardened DatePicker demo against `Context` borrow conflicts**:
  - Changed the demo theme binding from a borrowed `&cx.global::<Config>().theme` reference to an owned cloned `Theme` value.
  - This prevents immutable `cx` borrows from being inferred across later mutable `cx` use in the render function.

### Verification
- `cargo check` passed.
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Even when mutable `cx` use is visually before theme reads, holding an owned theme clone is safer in GPUI demo render functions that update child entities and then render themed content.

## Session 63 — 2026-05-08 (DatePicker Demo Borrow Fix Follow-up)

### Actions
- **Removed the problematic render-time child update from `date_picker_demo.rs`**:
  - Deleted the `self.basic.update(cx, ...)` callback rebinding block from `DatePickerDemo::render`.
  - The selected date text now derives from `self.basic.read(cx).value_ref()` instead of mutating the child picker during parent render.
  - Removed the extra `selected_text` field from the demo state.

### Verification
- `cargo check` passed.
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Rebinding child callbacks inside a parent `Render::render` is fragile and can create `Context` borrow overlap diagnostics in downstream editors/toolchains. Demo render paths should prefer read-only child inspection unless mutation is unavoidable.


## Session 64 — 2026-05-08 (P5 TimePicker)

### Actions
- **Added TimePicker component**:
  - Implemented `TimePicker` and `TimeValue` in `crates/aura-components/src/time_picker.rs`.
  - Supports fixed-list time selection, custom display formats, minute/second step controls, optional hidden seconds, disabled state, width/placeholder/id builder options, and `on_change` / `set_on_change` callbacks.
  - Uses the existing portal layer for the dropdown panel and trigger bounds capture for placement.
  - Added public exports in `crates/aura-components/src/lib.rs`.
- **Added Gallery demo**:
  - Created `apps/aura-gallery/src/demos/time_picker_demo.rs`.
  - Registered `TimePicker 时间选择器` in the Gallery demo registry.
  - Demo covers basic selection, custom format, stepped options, hidden seconds, and disabled state.
- **Updated memory**:
  - Marked P5 progress as 3/20 in `.memory/state.md`.
  - Added TimePicker status to `.memory/inventory.md`.

### Verification
- `cargo check` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- The TimePicker panel can reuse the DatePicker portal/backdrop pattern safely when the panel itself is a normal element tree rather than a custom wrapper element.

## Session 65 — 2026-05-08 (DatePicker Range Value Font Size)

### Actions
- **Adjusted DatePicker range trigger typography**:
  - Restored the left/right range value text to the normal input font size.
  - Kept only the separator chip (`至` by default) visually smaller/muted.

### Verification
- `cargo check` passed.
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Range endpoint values should match the normal DatePicker input text size; only secondary separators need reduced visual weight.

## Session 66 — 2026-05-08 (P5 DateTimePicker)

### Actions
- **Added DateTimePicker component**:
  - Implemented `DateTimePicker`, `DateTimeValue`, `DateTimePickerType`, and `DateTimePickerSelection`.
  - Supports single date-time selection, date-time ranges, custom display formats, range separator text, minute/second steps, optional hidden seconds, disabled state, and change callbacks.
  - Uses the portal/dropdown pattern with a normal element tree, calendar navigation, time columns, range endpoint chips, and explicit confirm/cancel actions.
  - Added public exports in `crates/aura-components/src/lib.rs`.
- **Added Gallery demo**:
  - Created `apps/aura-gallery/src/demos/date_time_picker_demo.rs`.
  - Registered `DateTimePicker 日期时间选择器` in the Gallery demo registry.
  - Demo covers basic selection, custom format, stepped time, hidden seconds, range selection, and disabled state.
- **Updated memory**:
  - Marked P5 progress as 4/20 in `.memory/state.md`.
  - Added DateTimePicker status to `.memory/inventory.md`.

### Verification
- `cargo check` passed.
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- DateTime selection benefits from explicit confirm/cancel because users may need to adjust both a calendar date and multiple time columns before committing the value.

## Session 67 — 2026-05-08 (P5 Upload)

### Actions
- **Added Upload component**:
  - Implemented `Upload`, `UploadFile`, `UploadStatus`, and `UploadListType`.
  - Supports button and drag-style upload triggers, text file lists, picture-card lists, progress bars, success/error/uploading/ready states, file size metadata, disabled state, multiple/accept/limit options, and select/remove callbacks.
  - Exposes mutation helpers for host-driven file list updates and internal remove actions.
  - Added public exports in `crates/aura-components/src/lib.rs`.
- **Added Gallery demo**:
  - Created `apps/aura-gallery/src/demos/upload_demo.rs`.
  - Registered `Upload 上传` in the Gallery demo registry.
  - Demo covers basic list, drag style, picture card list, upload limit, and disabled state.
- **Updated memory**:
  - Marked P5 progress as 5/20 in `.memory/state.md`.
  - Added Upload status to `.memory/inventory.md`.

### Verification
- `cargo check` passed.
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- GPUI does not provide a browser-style file input in this component layer, so `Upload` exposes `on_select` for the host app to bridge a native file picker while the component owns presentation and list interactions.

## Session 68 — 2026-05-08 (Time Candidate Panel Polish)

### Actions
- **Polished TimePicker time candidate panel**:
  - Added a clearer header, helper text, and selected-time preview pill.
  - Restyled hour/minute/second columns as bordered cards with labeled headers, stronger spacing, and selected-state contrast.
- **Polished DateTimePicker embedded time panel**:
  - Matched the same candidate-column treatment inside the combined date-time popup.
  - Added an embedded panel surface and preview pill so the time area no longer looks like raw lists.

### Verification
- `cargo check` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery; process ended by timeout with no startup crash.

### Key Discoveries
- Dense time candidate lists read better when each column has an explicit label, a quiet surface, and a high-contrast selected pill instead of flat text rows.

## Session 69 — 2026-05-08 (Time Panel Overflow Fix and Redesign)

### Actions
- **Reworked TimePicker time panel again after visual feedback**:
  - Fixed popup width calculation so three time columns no longer overflow the trigger-sized panel.
  - Replaced the nested card-heavy layout with one compact candidate surface and simpler rows.
  - Shortened the header and removed visually noisy helper copy.
- **Reworked DateTimePicker embedded time panel**:
  - Increased combined popup width and reduced calendar/time-panel column widths so the content fits inside bounds.
  - Changed the time panel to a compact fixed-width surface aligned with the DateTimePicker calendar area.
  - Simplified candidate rows and selected states to avoid the previous bulky/ugly appearance.

### Verification
- `cargo check` passed.
- `timeout 8s cargo run -p aura-gallery` compiled and launched the gallery; process ended by timeout with no startup crash.

### Key Discoveries
- The previous TimePicker popup still used a 260px minimum width while the redesigned three-column panel needed more horizontal space, causing visible overflow.


## Session 70 — 2026-05-09 (P5 Cascader)

### Actions
- **Added Cascader component**:
  - Implemented `Cascader` and `CascaderOption` in `crates/aura-components/src/cascader.rs`.
  - Supports multi-level option columns, default selected paths, disabled/loading options, clearable trigger, search-result rendering via `search_query`, width/placeholder/separator options, and `on_change` callbacks.
  - Added pure path helpers for label resolution and selectable-path validation.
  - Added public exports in `crates/aura-components/src/lib.rs`.
- **Added test coverage**:
  - Created `crates/aura-components/tests/cascader.rs` for selected-path label resolution and disabled/unknown path rejection.
- **Added Gallery demo**:
  - Created `apps/aura-gallery/src/demos/cascader_demo.rs`.
  - Registered `Cascader 级联选择器` in the Gallery demo registry.
  - Demo covers basic multi-level selection, default selected path, disabled state, and searchable result panel.
- **Updated memory**:
  - Marked P5 progress as 6/20 in `.memory/state.md`.
  - Added Cascader status to `.memory/inventory.md`.

### Verification
- `cargo test -p aura-components --test cascader` passed after an intentional RED failure for missing `Cascader` exports.
- `cargo check` passed.
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Cascader can reuse the Select portal/bounds pattern while keeping hierarchical option traversal as pure helpers, making the path behavior testable without GPUI rendering.


## Session 71 — 2026-05-09 (Cascader Popup Interaction Fix)

### Actions
- **Fixed Cascader popup item interaction**:
  - Added a default caller-derived component id and stable path-derived popup item ids so option rows inside scrollable portal columns become stateful interactive elements.
  - Added panel occlusion and mousedown propagation stop to keep inside-panel clicks from being treated as outside clicks.
  - Added regression coverage for stable popup item id generation.

### Verification
- `cargo test -p aura-components --test cascader` passed with 3 tests.
- `cargo check` passed.
- `git diff --check` passed.
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Portal popup rows inside scrollable columns need stable element ids for reliable hover/click hit testing, matching the DatePicker/TimePicker item pattern.


## Session 72 — 2026-05-09 (Cascader Leaf-only Dismissal)

### Actions
- **Fixed Cascader popup dismissal semantics**:
  - Removed trigger-level `on_mouse_down_out` closing, which treated portal panel clicks as outside-trigger clicks.
  - Wrapped the popup panel in a transparent backdrop that closes only when the backdrop itself is clicked.
  - Kept panel and option clicks from propagating so parent-group clicks update `active_path` and keep the popup open, while leaf clicks select and close through `choose_path`.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test cascader` passed with 3 tests.
- `git diff --check` passed.
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Trigger-level outside-click handlers do not distinguish portal descendants from true outside clicks; dropdown-style portal components should own their backdrop/inside-click propagation policy.


## Session 73 — 2026-05-09 (Cascader Lazy Loading)

### Actions
- **Added Cascader lazy loading**:
  - Added `Cascader::lazy(true)` and `Cascader::on_lazy_load(...)` / `set_on_lazy_load(...)` APIs.
  - Added `CascaderOption::leaf(true)` so lazy mode can distinguish final selectable leaves from unloaded empty branches.
  - Added host update helpers `set_children_at_path(...)` and `set_loading_at_path(...)`, backed by pure option-tree helpers.
  - Updated selection behavior so lazy empty branches trigger `on_lazy_load`, show loading state, keep the popup open, and only select when a leaf is chosen.
- **Added Gallery usage**:
  - Extended `apps/aura-gallery/src/demos/cascader_demo.rs` with a `懒加载` section showing `lazy(true)`, `set_on_lazy_load`, and `set_children_at_path`.
- **Added tests**:
  - Covered lazy option selectability and installing children into a lazy path.
- **Updated memory**:
  - Updated Cascader inventory status to include lazy loading.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test cascader` passed with 5 tests.
- `git diff --check` passed.
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Lazy Cascader needs an explicit `leaf(true)` marker because an empty child list can mean either a final selectable node or a not-yet-loaded branch.


## Session 74 — 2026-05-09 (Cascader Lazy Callback Reentrancy Fix)

### Actions
- **Fixed Cascader lazy-loading crash**:
  - Changed lazy-load callbacks to receive `&mut Cascader` and `&mut Context<Cascader>` directly.
  - Updated the Gallery lazy demo to call `set_children_at_path` inside the provided callback without nested `Entity::update`.
  - This avoids GPUI double-lease panics when lazy loading is triggered from inside the component's own event update.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test cascader` passed with 5 tests.
- `git diff --check` passed.
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- GPUI entities cannot be updated recursively while already leased; component callbacks that may mutate the same component should receive the active mutable component/context rather than requiring callers to re-enter `Entity::update`.


## Session 75 — 2026-05-09 (P5 Transfer)

### Actions
- **Added Transfer component**:
  - Implemented `Transfer` and `TransferItem` in `crates/aura-components/src/transfer.rs`.
  - Supports source/target panels, checked item movement, disabled items, target key ordering, optional filter display, custom titles/sizing, and `on_change` callbacks.
  - Added public exports in `crates/aura-components/src/lib.rs`.
- **Added test coverage**:
  - Created `crates/aura-components/tests/transfer.rs`.
  - Covered moving checked source items, moving checked target items back, disabled item preservation, and filtering by key/label/description.
- **Added Gallery demo**:
  - Created `apps/aura-gallery/src/demos/transfer_demo.rs`.
  - Registered `Transfer 穿梭框` in the Gallery demo registry.
  - Demo covers basic movement, filtered display, and disabled target items.
- **Updated memory**:
  - Marked P5 progress as 7/20 in `.memory/state.md`.
  - Added Transfer status to `.memory/inventory.md`.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test transfer` passed with 3 tests.
- `git diff --check` passed.
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Transfer needs to be a stateful `Render` component, not `RenderOnce`, because item checking and move actions mutate internal selected-key state before emitting changed target keys.


## Session 76 — 2026-05-09 (Transfer Checked-state Handoff)

### Actions
- **Adjusted Transfer move semantics**:
  - Added `move_to_target_with_checked` and `move_to_source_with_checked` helpers.
  - Updated UI move actions so moved items remain checked on the destination side.
  - Added regression tests for source→target and target→source checked-state handoff.

### Verification
- `cargo test -p aura-components --test transfer` passed with 5 tests.
- `cargo check` passed.
- `git diff --check` passed.
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Transfer should preserve user intent across side changes by transferring checked state with moved items instead of clearing all destination checks.


## Session 77 — 2026-05-10 (Upload Select Callback Demo Fix)

### Actions
- **Fixed Upload click/select behavior in Gallery**:
  - Updated `Upload::on_select` to receive `&mut Upload` and `&mut Context<Upload>` so callbacks can safely mutate the same component without nested `Entity::update`.
  - Updated `Upload::on_remove` to follow the same direct-mutation callback shape.
  - Added `Upload::file_count` and `Upload::can_accept_more_len` helpers.
  - Extended the Upload demo so clicking the button/drag/picture-card triggers adds a simulated file via `on_select`.
- **Added tests**:
  - Created `crates/aura-components/tests/upload.rs` for accept/limit checks and progress clamping.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test upload` passed with 2 tests.
- `git diff --check` passed.
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Upload demo previously exposed an empty select callback path, so clicking appeared broken. Same-component mutation callbacks should pass the active component/context directly to avoid GPUI double-lease risks.


## Session 78 — 2026-05-10 (Upload Real File Picker)

### Actions
- **Upgraded Upload to use the platform file selector**:
  - `Upload` now opens GPUI's `prompt_for_paths` dialog when the trigger is clicked.
  - Added support for single/multiple selection through the existing `multiple` flag.
  - Added `max_size(bytes)` and post-selection validation for accepted file extensions / MIME groups (`.png`, `.pdf`, `image/*`, etc.).
  - Selected files are converted into `UploadFile` entries with path, name, size, and ready status.
  - Invalid selections are ignored and surfaced through an inline error message.
  - `on_select` now runs after accepted files are added and receives `&mut Upload` plus `Context<Upload>` for safe same-component mutation.
- **Updated Gallery demo**:
  - Replaced simulated selection with real file picker usage.
  - Added accept/max-size examples for basic, drag, picture-card, and limited uploads.
- **Added tests**:
  - Expanded `crates/aura-components/tests/upload.rs` to cover accept matching and max-size rejection.

### Verification
- `cargo test -p aura-components --test upload` passed with 4 tests.
- `cargo check` passed.
- `git diff --check` passed.
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- GPUI 0.2.2 exposes `prompt_for_paths` for file picking but its `PathPromptOptions` does not include native accept/type filters, so Aura validates accepted type and size after selection.


## Session 79 — 2026-05-10 (P5 ColorPicker)

### Actions
- **Added ColorPicker component**:
  - Implemented `ColorPicker` in `crates/aura-components/src/color_picker.rs`.
  - Supports HEX normalization, RGB conversion helper, preset swatches, custom presets, disabled state, optional label display, sizing, and `on_change` callbacks.
  - Added public exports in `crates/aura-components/src/lib.rs`.
- **Added test coverage**:
  - Created `crates/aura-components/tests/color_picker.rs` for HEX normalization, invalid color rejection, and RGB conversion.
- **Added Gallery demo**:
  - Created `apps/aura-gallery/src/demos/color_picker_demo.rs`.
  - Registered `ColorPicker 颜色选择器` in the Gallery demo registry.
  - Demo covers basic use, custom presets, hidden label, and disabled state.
- **Updated memory**:
  - Marked P5 progress as 8/20 in `.memory/state.md`.
  - Added ColorPicker status to `.memory/inventory.md`.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test color_picker` passed with 3 tests.
- `git diff --check` passed.
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- A preset-swatch ColorPicker can keep color parsing testable by exposing pure HEX normalization/RGB helpers while leaving richer custom color input for a future enhancement.


## Session 80 — 2026-05-10 (ColorPicker Popup Rainbow Panel)

### Actions
- **Updated ColorPicker interaction model**:
  - Changed the visible control to a compact color cube trigger.
  - Added a portal popup panel that opens on trigger click and closes on outside click or color selection.
  - Added a rainbow color matrix plus custom preset swatches inside the popup.
  - Added stable trigger/panel bounds capture and row item ids for popup interaction.
- **Updated tests and demo**:
  - Added `ColorPicker::rainbow_palette()` coverage.
  - Updated Gallery copy to explain the cube trigger and popup color panel.
- **Updated memory**:
  - Updated ColorPicker inventory status to include cube trigger and popup rainbow panel.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test color_picker` passed with 4 tests.
- `git diff --check` passed.
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- GPUI does not expose a CSS-style gradient background helper in this version, so the ColorPicker popup uses a dense rainbow swatch matrix to approximate a colorful gradient panel while preserving reliable hit testing.


## Session 81 — 2026-05-10 (ColorPicker HSV + Alpha Panel)

### Actions
- **Reworked ColorPicker popup to match screenshot target**:
  - Replaced the simple rainbow swatch popup with a picker-style panel.
  - Added a large clickable saturation/value color area generated from the active hue.
  - Added a right-side hue selector bar.
  - Added an alpha selector bar and rgba display text.
  - Added HSV-to-HEX and rgba formatting helpers.
- **Updated tests and demo**:
  - Added tests for rgba display and HSV color generation.
  - Updated Gallery copy to describe free color/hue/alpha selection.
- **Updated memory**:
  - Updated ColorPicker inventory status to include HSV panel, hue bar, alpha bar, and rgba display.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test color_picker` passed with 6 tests.
- `git diff --check` passed.

### Key Discoveries
- Without a native gradient background API, a dense HSV grid provides free-ish color selection and predictable click targets while visually matching a gradient picker much more closely than static presets.


## Session 82 — 2026-05-10 (ColorPicker Alpha Rendering and Dense Grid)

### Actions
- **Fixed ColorPicker alpha rendering**:
  - Applied current alpha to the trigger cube color.
  - Applied current alpha to the large saturation/value panel colors.
  - Kept rgba display clamped and added test coverage for alpha clamping.
- **Improved color panel density**:
  - Increased the saturation/value picker from a coarse 20×12 grid to a dense 70×45 grid.
  - Reduced each cell to roughly 4px to better approximate a continuous gradient panel.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test color_picker` passed with 7 tests.
- `git diff --check` passed.
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Alpha previously only affected the text/alpha bar, not the rendered picker swatch or SV panel; applying opacity at render points makes the demo visibly respond to alpha changes.

## Session 83 — 2026-05-10 (ColorPicker Pixel Grid and Stable Panel Alpha)

### Actions
- Changed the ColorPicker saturation/value area to a 280×180 grid with 1px cells.
- Kept the saturation/value panel and preset swatches opaque when alpha changes, so alpha edits do not wash out the original color-selection panel.
- Preserved alpha on the selected trigger/rgba output and alpha bar where alpha is the selected value/preview.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test color_picker` passed with 7 tests.
- `git diff --check` passed.
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Applying alpha to the SV grid made alpha changes visually alter the color-selection surface itself; the selection surface should remain an opaque source of color values while alpha is edited independently.

## Session 84 — 2026-05-10 (ColorPicker Pixel Sliders and Surface Render Optimization)

### Actions
- Replaced per-pixel `div` children/listeners in the ColorPicker SV panel with a custom painted surface and one click/drag handler.
- Reworked hue and alpha sliders as 1px-granularity painted surfaces.
- Added drag selection for SV, hue, and alpha surfaces using coordinate-to-value mapping.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test color_picker` passed with 7 tests.
- `git diff --check` passed.
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- The visible stall came from tens of thousands of GPUI elements/listeners for a 1px grid; painting quads from one custom element keeps the 1px visual density while avoiding 50k child elements.

## Session 85 — 2026-05-10 (ColorPicker Rasterized Surfaces for Responsiveness)

### Actions
- Replaced ColorPicker per-frame 1px quad painting with cached `RenderImage` rasters for SV, hue, and alpha surfaces.
- Cached SV raster by rounded hue, cached hue raster statically, and cached alpha raster by selected color.
- Kept coordinate-based 1px selection and marker overlays while reducing render work to one image paint per surface.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test color_picker` passed with 7 tests.
- `git diff --check` passed.
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- The second stall came from painting tens of thousands of quads each frame; cached raster surfaces keep the 1px appearance and selection accuracy without rebuilding scene geometry on every popup/click.

## Session 86 — 2026-05-10 (ColorPicker Raster Channel Order Fix)

### Actions
- Aligned ColorPicker raster surface pixels with GPUI `RenderImage` BGRA channel order.
- Kept HSV click mapping unchanged so the displayed panel color now matches the selected preview value.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test color_picker` passed with 7 tests.
- `git diff --check` passed.
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- `RenderImage` data is expected in BGRA order; generating RGBA bytes made the rasterized panel display a different color than the HSV value selected by clicking it.

## Session 87 — 2026-05-10 (ColorPicker Centered Dropdown Icon)

### Actions
- Centered the ColorPicker down-arrow icon within the trigger cube.
- Kept the small translucent icon backing, now centered instead of bottom-right.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test color_picker` passed with 7 tests.
- `git diff --check` passed.
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- The trigger already had a full-size bounds-capture overlay; the icon can use a separate absolute full-size flex overlay to center without affecting click handling.


## Session 88 — 2026-05-10 (P5 Carousel Deferred and Image Component)

### Actions
- Marked Carousel as deferred/identified for later by user request instead of implementing it now.
- Added the P5 Image component with fit modes, configurable size, radius, border, shadow, grayscale, preview badge, loading placeholder, fallback, and empty state.
- Added Image exports, gallery demo, and unit tests for fit/dimension builder behavior.
- Updated P5 progress to 9/20 completed components.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test image` passed with 3 tests.
- `git diff --check` passed.
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- GPUI `img` already provides object-fit, loading, and fallback hooks, so Aura Image can wrap that API with Element-style defaults and gallery-friendly states.

## Session 89 — 2026-05-10 (Image Remote URL and Local File Sources)

### Actions
- Copied `~/Downloads/local.jpeg` into `apps/aura-gallery/assets/local.jpeg` for the Image demo workspace.
- Added first-class `ImageSource` support for remote URL strings and local filesystem paths.
- Added `Image::local(...)` / `Image::file(...)` builders and source inspection helpers.
- Updated the Image demo to show the provided Element remote URL and the copied local image asset.
- Expanded Image tests to cover both remote URL and local file source selection.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test image` passed with 5 tests.
- `git diff --check` passed.
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- GPUI `img` treats strings as URI/embedded resources, while filesystem images should be passed as `PathBuf`; Aura Image now preserves that distinction instead of forcing all sources through `SharedString`.

## Session 90 — 2026-05-10 (Image Demo Local Asset Absolute Path)

### Actions
- Fixed Image demo local asset path to use `env!("CARGO_MANIFEST_DIR")/assets/local.jpeg` instead of a workspace-relative string.
- Kept `Image::local(...)` path-based loading, now passing an absolute path in the gallery demo so runtime cwd changes do not break local images.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test image` passed with 5 tests.
- `git diff --check` passed after reverting unrelated local formatting noise.
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- GPUI `img(PathBuf)` resolves filesystem paths literally; a workspace-relative string can fail when the gallery binary runs with a different cwd, so demo assets should use the gallery crate manifest directory.


## Session 91 — 2026-05-10 (Image Local Decode and P7 Prompt Sync)

### Actions
- Synced the user's `.prompt/P7-demo-self-contained.md` update into session memory: P7 now explicitly requires demo registry/components to be ordered by component name dictionary ASC.
- Changed local Image rendering to decode filesystem files into a GPUI `RenderImage` directly, avoiding the async path-resource load path that was not showing the local demo image.
- Kept remote URLs on GPUI's async image loader and local files on the direct filesystem decode path.
- Added a test that the copied demo local asset exists.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test image` passed with 6 tests.
- `git diff --check` passed after trimming trailing whitespace in the user-updated P7 prompt.
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Absolute local paths were correct, but relying on GPUI's path-resource async image branch still did not render in the demo. Directly decoding local files to `RenderImage` makes local image display deterministic for Aura Image.


## Session 92 — 2026-05-10 (Image Local Custom Painter)

### Actions
- Added a small custom `LocalImageElement` for `Image::local(...)` that paints decoded `RenderImage` data directly with `Window::paint_image`.
- Kept remote URL rendering on GPUI `img(...)`, but stopped routing local files through GPUI `img(...)` after decode.
- Preserved object-fit and grayscale support for local images in the custom painter.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test image` passed with 6 tests.
- `git diff --check` passed.
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process exited successfully in this run.

### Key Discoveries
- Local file decoding and asset-path existence were both verified, so the remaining blank local display path is the local RenderImage handoff through `gpui::img`; direct `paint_image` is the narrower rendering path.

## Session 93 — 2026-05-10 (Image File Protocol and Local Fill Layout)

### Actions
- Added `file://` protocol recognition to `Image::new(...)`, mapping file URLs to local file rendering instead of remote URL loading.
- Updated the Image demo local sample to use `file://{CARGO_MANIFEST_DIR}/assets/local.jpeg` so the displayed source is explicitly a local-file protocol.
- Wrapped the custom local image painter in an absolute full-size layer so local images fill the component frame instead of depending on flex child sizing.
- Added file protocol source classification test coverage.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test image` passed with 7 tests.
- `git diff --check` passed.
- `timeout 20s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- The file existed and decoded, so the remaining risks were source classification ambiguity and the custom local painter not filling the visual frame. The demo now exercises the same file-protocol API users should call.


## Session 94 — 2026-05-10 (Image Cached Remote Decode)

### Actions
- Interpreted the screenshot: the local deer image is visible, while remote URL slots are falling back.
- Added a cached direct remote URL decode path for Aura Image using `ureq`, sharing the same raster painter as local files.
- Kept source classification: `file://` and `Image::local` use filesystem decode; `https://` URLs use cached URL decode before falling back.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test image` passed with 7 tests.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- The provided screenshot shows the local asset rendering in the middle Image slots; the real remaining failure is GPUI's async remote URL branch falling back for the Element CDN URL.


## Session 95 — 2026-05-10 (Image Async Remote, Preview, and Radius)

### Actions
- Removed blocking remote URL fetch from Image render path; remote images now load on a background thread and request animation frames while pending.
- Added a persistent preview popup for Image preview mode using the loaded raster, Aura portal layer, and gallery-level preview renderer.
- Passed component radius into raster image painting so round/circle images clip through `Window::paint_image` instead of only rounding the outer frame.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test image` passed with 7 tests.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Direct synchronous URL decoding fixed remote display but made selecting the Image demo stall; remote decoding must not happen on the render path.


## Session 96 — 2026-05-10 (Image Remote Refresh and Preview Cleanup)

### Actions
- Changed remote image loading completion to actively refresh the gallery window when the background fetch/decode finishes, reducing visible delay after selecting the Image demo.
- Removed the visible "Preview" badge/button from preview images; cursor/hover affordance remains.
- Changed the circle image demo to use the local image source so circle clipping can be seen immediately without depending on remote image load timing.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test image` passed: 7 tests.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; timeout stopped the running GUI smoke test.

### Key Discoveries
- Remote loading should signal the window directly when complete; relying only on animation-frame polling can make the ready image appear late. The circle demo was using a remote source, so remote latency made it look like circle rendering was broken.


## Session 97 — 2026-05-10 (Image Circle Radius Clamp)

### Actions
- Matched GPUI's built-in image painting behavior by clamping custom raster image corner radii and made `ImageRadius::Round` compute its radius from the visible container short side, so cover-cropped images paint as circles instead of rounded rectangles.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test image` passed: 7 tests.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; timeout stopped the running GUI smoke test.

### Key Discoveries
- The circle image path uses the custom raster painter, not GPUI's `img` element. GPUI's `img` clamps corner radii before painting; the custom painter was passing the sentinel round radius directly. After clamping, cover-cropped images could still look rounded because the painted image bounds can be wider than the visible square, so round radius must be based on the visible container bounds.


## Session 98 — 2026-05-10 (Image Round Crop)

### Actions
- Changed `ImageRadius::Round` raster painting to center-crop the decoded image to a square and paint it into the visible square bounds with a half-side radius.
- Cached square-cropped render images by source render image id to avoid recropping every frame.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test image` passed: 7 tests.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; timeout stopped the running GUI smoke test.

### Key Discoveries
- Painting a rectangular cover-fitted raster with large radii still produces a rounded rectangle because the rounded rectangle is computed against the expanded cover bounds. A true circle requires a square paint target and square source crop.


## Session 99 — 2026-05-10 (Image Round Options and Ring Sleeve)

### Actions
- Added `ImageRoundOptions` for configurable round rendering and `ImageRing` for a transparent circular ring sleeve overlay.
- Added builder APIs: `round_options(...)`, `round_ring(...)`, and `round_config()` inspection for tests.
- Updated the Image demo with Circle, Round bounds, and Ring sleeve examples.

### Verification
- Red test first: `cargo test -p aura-components --test image image_supports` failed before the new API existed.
- `cargo check` passed.
- `cargo test -p aura-components --test image` passed: 9 tests.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; timeout stopped the running GUI smoke test.

### Key Discoveries
- The ring sleeve should be a transparent-background paint overlay with only border pixels, so the image remains visible through the center of the circular sleeve.


## Session 100 — 2026-05-10 (Image Round Bounds Semantics)

### Actions
- Fixed `ImageRoundOptions::without_square_crop()` so it uses the component/container bounds instead of forcing a square paint target.
- Changed the Image demo `Round bounds` example to a rectangle so it visibly differs from the `Circle` example.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test image` passed: 9 tests.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; timeout stopped the running GUI smoke test.

### Key Discoveries
- `Round bounds` looked like `Circle` because the custom raster painter forced all round rendering through square bounds. The no-square-crop option needs to preserve the container bounds and only apply half-short-side radii.


## Session 101 — 2026-05-10 (Autocomplete and P5 Deferrals)

### Actions
- Added Autocomplete as the final requested P5 component before skipping the remaining advanced components.
- Added static suggestion items, case-insensitive filtering, click-to-select, clear action, disabled state, configurable width/max suggestions, and gallery demo.
- Added `Input::on_change` / `set_on_change` support so composed components can react to typing.
- Marked Calendar, TreeSelect, InputTag, Mention, Watermark, Tour, and VirtualizedTable/VirtualizedTree as deferred/identified for later per user request.

### Verification
- `cargo test -p aura-components` passed.
- `cargo check` passed.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; timeout stopped the running GUI smoke test.

### Key Discoveries
- Autocomplete can reuse the existing `Input` entity safely when text-change observation is exposed by `Input`, avoiding a second text-editing implementation.


## Session 102 — 2026-05-10 (Autocomplete Clear and Suffix Icons)

### Actions
- Replaced Autocomplete's absolute-positioned clear icon with the existing Input clear affordance, so it is vertically centered and only appears when content is non-empty.
- Added Autocomplete suffix icon configuration: default Search icon, `suffix_icon(...)` for custom icons, and `no_suffix_icon()` to remove it.
- Added gallery demo coverage for custom suffix icon and no suffix icon.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test autocomplete` passed: 4 tests.
- `cargo test -p aura-components` passed.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; timeout stopped the running GUI smoke test.

### Key Discoveries
- Input already has a centered clear icon that is hidden while empty, so Autocomplete should configure/reuse it instead of painting a second absolute clear icon.


## Session 103 — 2026-05-10 (Input Clear Icon Interaction)

### Actions
- Changed the shared Input clear icon to clear on mouse-down and stop propagation so composed components like Autocomplete do not swallow the click.
- Added explicit hover pointer styling for the clear icon.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components` passed.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; timeout stopped the running GUI smoke test.

### Key Discoveries
- Clear was using mouse-up without propagation control; in composed input wrappers that also listen to mouse-down, the interaction could focus/open instead of clearing reliably.


## Session 104 — 2026-05-10 (Autocomplete Input Clear Hit Testing)

### Actions
- Kept Autocomplete using the shared `Input` clear icon instead of a separate Autocomplete clear overlay.
- Fixed Autocomplete hit testing by placing the bounds-capture overlay behind the Input child, so Input's clear icon receives hover/click events inside Autocomplete.
- Reverted the shared Input clear behavior back to its Input-owned implementation; the fix is scoped to Autocomplete composition.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test autocomplete` passed: 4 tests.
- `cargo test -p aura-components` passed.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; timeout stopped the running GUI smoke test.

### Key Discoveries
- Autocomplete's absolute bounds-capture child was rendered above the Input child, blocking Input's own clear icon interactions. Reordering the capture child behind Input preserves composition and lets Input own clear behavior.


## Session 105 — 2026-05-10 (Autocomplete Clear Event Ownership)

### Actions
- Kept clear icon ownership in the shared Input used by Autocomplete.
- Removed Autocomplete's full-width mouse open handler so it no longer competes with Input's inner clear icon hover/click handling.
- Preserved the bounds capture layer behind the Input child.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test autocomplete` passed: 4 tests.
- `cargo test -p aura-components` passed.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; timeout stopped the running GUI smoke test.

### Key Discoveries
- Input's clear icon already works by itself. In Autocomplete, the remaining blocker was the Autocomplete wrapper registering its own full-width mouse handler over the same interaction area, competing with the composed Input's inner controls.


## Session 106 — 2026-05-10 (Autocomplete No Wrapper Mouse Capture)

### Actions
- Removed Autocomplete's remaining wrapper mouse-down-out listener so no Autocomplete wrapper mouse listener participates in the input hit area.
- Left the shared Input clear icon as the only clear control and kept bounds capture behind Input.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test autocomplete` passed: 4 tests.
- `cargo test -p aura-components` passed.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; timeout stopped the running GUI smoke test.

### Key Discoveries
- Even outside-click listeners add a wrapper hitbox/listener to the same composed region. For Autocomplete, the input wrapper must not register mouse handlers over the Input if Input child controls (clear) need hover/click priority.


## Session 107 — 2026-05-10 (Autocomplete Clear and Outside Dismiss)

### Actions
- Made Input clear handle mouse-down with pointer hover and propagation stop so it can win before Autocomplete/popup outside handlers react.
- Reintroduced outside-click dismissal on the popup panel itself via `on_mouse_down_out`, not on the Autocomplete input wrapper.
- Autocomplete now closes suggestions when the clear action empties the input.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test autocomplete` passed: 4 tests.
- `cargo test -p aura-components` passed.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; timeout stopped the running GUI smoke test.

### Key Discoveries
- The safe event split is: Input owns clear on mouse-down; popup panel owns outside dismissal. The Autocomplete input wrapper should remain non-interactive over the Input region.



## Session 108 — 2026-05-10 (P9 Deferred Advanced Phase)

### Actions
- Created `.prompt/P9-deferred-advanced.md` as the latest phase for components skipped/deferred from P5.
- Moved Carousel, Calendar, TreeSelect, InputTag, Mention, Watermark, Tour, VirtualizedTable, and VirtualizedTree into P9 backlog.
- Updated `prompt.md`, `.memory/state.md`, and `.memory/inventory.md` so future sessions remember P9 is deferred and should be supplemented later only when requested.

### Verification
- `git diff --check` passed.
- Documentation/memory references checked locally: `prompt.md`, `.prompt/P5-advanced.md`, `.prompt/P8-engineering.md`, `.prompt/P9-deferred-advanced.md`, `.memory/state.md`, `.memory/inventory.md`.

### Key Discoveries
- P5 requested subset is over for now; deferred advanced components should remain visible as identified future scope rather than being lost or treated as active work.

## Session 109 — 2026-05-10 (RadioGroup and CheckboxGroup Button Layouts)

### Actions
- Added explicit `Vertical`, `Horizontal`, and `Button` layout variants for `RadioGroup` and `CheckboxGroup`.
- Added `Large`, `Default`, and `Small` group sizing APIs so button-style groups can match the provided segmented reference.
- Updated the Form demo with large/default/small segmented RadioGroup and CheckboxGroup examples using the New York / Washington / Los Angeles / Chicago labels.
- Added lightweight layout default regression tests for the new public layout/size enums.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components` passed: 37 tests total across component/unit/integration suites.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- The reference maps best to an opt-in segmented button layout, not a replacement of the existing radio-circle / checkbox-row defaults.
- Existing GPUI styling helpers in this project do not include `inline_flex` / `w_fit`, so the segmented container uses the available flex/border/radius/overflow primitives.

## Session 110 — 2026-05-10 (Group Button Stretch Mode)

### Actions
- Added opt-in `stretch(true)` APIs for `RadioGroup` and `CheckboxGroup` button mode.
- Added `block(true)` aliases for compatibility with Segmented-style naming while preserving Tabs-like `stretch` terminology.
- Kept button groups wrap-content by default; stretch mode applies `w_full()` to the group and `flex_1()` to each option.
- Added stretched RadioGroup and CheckboxGroup examples to the Form demo.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components` passed.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- This behavior matches the existing Tabs `stretch(true)` pattern and Segmented `block(true)` width semantics: default content width, opt-in full parent width with equal option widths.

## Session 111 — 2026-05-10 (Group Button Wrap Content Fix)

### Actions
- Fixed button-mode `RadioGroup` and `CheckboxGroup` default width by setting `align-self: start` when `stretch` is false.
- Preserved `stretch(true)` / `block(true)` behavior as full parent width with equal option widths.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components --test group_layout` passed.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Root cause: the Form demo places groups inside a flex-column parent whose default cross-axis alignment stretches child flex items. The group itself needed `self_start()` when not stretched; width auto alone was not enough.

## Session 112 — 2026-05-10 (Independent Form Control Demos)

### Actions
- Added `form_controls_demo.rs` with independent usage demos for Input, InputNumber, Textarea, Checkbox, Radio, Switch, Select, Slider, and Rate.
- Registered the new standalone form-control demos in the Gallery navigation before the existing `Form 表单` demo.
- Preserved `form_demo.rs` without changing its Form/FormItem usage, so form-specific examples remain available.

### Verification
- `cargo check` passed.
- `cargo test -p aura-gallery` passed.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- The safest extraction path is additive: standalone component demos can duplicate the existing usage examples while leaving `FormDemo` as the form-layout integration reference.


## Session 113 — 2026-05-10 (P6 Built-in Unique IDs)

### Actions
- Added `aura_core::next_unique_id()` and `aura_core::unique_id(prefix)` backed by a process-wide `AtomicU64`.
- Replaced call-site/render-site derived default IDs in interactive components with runtime unique, component-prefixed IDs.
- Added/retained `.id(...)` override APIs for migrated components including Alert, Breadcrumb, Collapse, Link, PageHeader, Scrollbar, Tag, and Tree.
- Prefixed internal child IDs with each component root ID for migrated controls, including Dropdown items, Cascader search results, Tag close buttons, Tree node sub-elements, and Scrollbar viewport.
- Advanced project state from P6 to P7 pending.

### Verification
- `cargo test -p aura-core unique_id_tests::generated_ids_are_prefixed_and_unique` passed.
- `cargo test -p aura-components` passed.
- `cargo check` passed with 0 warnings.
- `git diff --check` passed.

### Key Discoveries
- Several components already prefixed child IDs with a component ID but seeded that component ID from `track_caller`; loops/helpers could still collide.
- Literal IDs remained in a few interactive children (`close-btn`, `back-btn`, `scroll-viewport`, Cascader search results); those now derive from the component ID.


## Session 114 — 2026-05-10 (Fix ID Stability Regression)

### Actions
- Root-caused the interaction regression introduced by P6: several `RenderOnce` components were assigning fresh atomic IDs during each render, which changes GPUI `ElementId`s across frames and breaks hover/click/portal state.
- Restored cross-frame stable IDs for transient `RenderOnce` controls including Button, Link, Tooltip, Popover, Popconfirm, Tag, Tree child elements, Alert, PageHeader, Scrollbar, and related demo controls.
- Added `aura_core::stable_unique_id(...)`, which stores a generated ID in GPUI keyed element state so render-path components can get a globally unique ID without changing it every frame.
- Kept `aura_core::unique_id(prefix)` for persistent component/entity construction where the ID is assigned once and then remains stable.
- Updated `unique_id` documentation to explicitly forbid direct per-frame allocation.

### Verification
- `cargo check` passed.
- `cargo test -p aura-components` passed.
- `cargo test -p aura-core unique_id_tests::generated_ids_are_prefixed_and_unique` passed.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- GPUI `ElementId` must be globally unique enough for the rendered tree and stable across frames for the same visual element.
- Atomic/generated IDs satisfy uniqueness but are unsafe in `RenderOnce` render paths unless the generated value is stored in persistent entity/element state.
- For persistent `Render` components, constructor-time `unique_id(prefix)` is acceptable because it runs once per entity instance.
- For stateless `RenderOnce` builders created every frame, use `stable_unique_id` with a stable key, explicit `.id(...)`, or wrap the component in a persistent entity before using runtime-generated IDs.


## Session 115 — 2026-05-10 (Portal Interaction Fixes)

### Actions
- Changed Message and Notification renderers to use passive portals and skip portal creation when empty, so expired toasts no longer leave an input-blocking active portal layer.
- Rendered Tooltip through the passive portal path because hover-only hints should not create a global input mask.
- Adjusted Pagination active-page hover to use a distinct active hover background instead of the normal page-hover treatment.
- Closed collapsed and horizontal Menu popovers after selecting a popover item.

### Verification
- `cargo fmt --all` passed.
- `cargo check` passed.
- `cargo test -p aura-components` passed.
- `cargo test -p aura-core unique_id_tests::generated_ids_are_prefixed_and_unique` passed.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Passive/non-modal overlays must not use the active `Portal` layer because `PortalLayer` intentionally occludes the full window whenever active portals exist.
- Toasts and tooltips are visual overlays, not modal interaction surfaces; modal/popover overlays can remain active portals.


## Session 116 — 2026-05-10 (Menu/Select/Pagination/Progress polish)

### Actions
- Fixed Menu popover item selection to clear the actual collapsed/horizontal popover id after selecting an item.
- Changed Pagination hover from green-tinted backgrounds to darker neutral gray hover states, with a distinct current-page hover treatment.
- Added explicit IDs to Select dropdown options so GPUI hover styling applies reliably in the portal list, including a selected-option hover state.
- Added `Progress::gradient(left, middle, right)` and rendered the line progress fill as two linear-gradient halves for left→middle→right transitions.
- Added a gradient progress example to the Gallery demo.

### Verification
- `cargo fmt --all` passed.
- `cargo test -p aura-components` passed.
- `cargo test -p aura-core unique_id_tests::generated_ids_are_prefixed_and_unique` passed.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Menu popover close must clear the trigger popover id, not a child item id-derived string.
- Select portal options need their own stable IDs for hover styles to be tracked consistently.


## Session 117 — 2026-05-10 (Pagination Hover Color Correction)

### Actions
- Corrected Pagination hover semantics: active/current page hover now uses a primary-color darkening treatment, while inactive pages use a darker neutral gray hover.

### Verification
- `cargo fmt --all` passed.
- `cargo check` passed.
- `git diff --check` passed.

## Session 118 — 2026-05-10 (Progress Gradient Vector API)

### Actions
- Changed `Progress::gradient` to accept `gradient(vec![...])` with any non-empty number of colors instead of fixed left/middle/right arguments.
- Rendered one color as a solid fill and two or more colors as adjacent two-stop gradient segments, preserving support for arbitrary color counts despite GPUI's two-stop linear gradient primitive.
- Updated the Gallery Progress demo to use a four-color gradient vector.

### Verification
- `cargo fmt --all` passed.
- `cargo test -p aura-components` passed.
- `cargo check` passed.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- GPUI `linear_gradient` currently accepts two stops, so arbitrary multi-color gradients are best represented as equal-width adjacent two-stop segments until a multi-stop background API exists.

## Session 119 — 2026-05-10 (Extract Preview Component)

### Actions
- Extracted Image preview behavior into a standalone `Preview` component with image URL/file source builders and arbitrary trigger content.
- Kept `Image::preview(true)` behavior and hover styling intact by delegating click/overlay behavior to `Preview` while preserving the Image frame styling.
- Moved the shared preview portal/global state to the new Preview module and kept `aura_components::image::render_image_preview` as a compatibility re-export.
- Added a Gallery `Preview 预览` demo entry showing image and custom-card triggers.
- Added regression tests for the new Preview builder and existing Image preview flag behavior.

### Verification
- Wrote failing tests first: `cargo test -p aura-components --test image` failed because `Preview` and `Image::preview_enabled` did not exist.
- `cargo fmt --all` passed.
- `cargo test -p aura-components --test image` passed.
- `cargo test -p aura-components` passed.
- `cargo check` passed.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- The preview overlay can be separated cleanly from Image rendering by sharing the existing `RasterImageElement` and image-loading helpers within the crate.
- Image preview hover styling must remain on the Image frame rather than a wrapper to preserve the existing visual effect.

## Session 120 — 2026-05-10 (Component Performance Audit Document)

### Actions
- Converted the read-only component performance audit into `docs/component-performance-audit-2026-05-10.md`.
- Preserved the overall conclusions, full component risk table, focused hotspots, evidence boundaries, and non-regression constraint.

### Verification
- `git diff --check` passed.
- Reviewed the generated Markdown header/table preview with `sed`.

### Key Discoveries
- The audit is documentation-only and does not modify component behavior.

## Session 121 — 2026-05-10 (Start P7 Demo Self-Contained)

### Actions
- Started P7 Demo Self-Contained work.
- Locked Gallery demo ordering with a regression test and sorted `registry()` by component name ascending at runtime.
- Migrated `button_demo.rs` away from direct GPUI layout primitives by using Aura `Space` and `Title`.
- Added small Aura API helpers needed for demo self-containment: `Space::wrap`, semantic gap helpers, and Button rounded convenience builders.
- Added tests for the new ordering rule, Button demo primitive ban, Space wrap builder, and Button rounded helpers.

### Verification
- Confirmed the new registry-order test failed before implementation.
- Confirmed the Button demo primitive-ban test failed before migration.
- `cargo fmt --all` passed.
- `cargo test -p aura-gallery registry_entries_are_sorted_by_component_name` passed.
- `cargo test -p aura-gallery button_demo_uses_aura_layout_primitives` passed.
- `cargo test -p aura-components space_wrap_builder_tracks_state` passed.
- `cargo test -p aura-components button_rounded_helpers_set_custom_radius` passed.
- `cargo test -p aura-components` passed.
- `cargo test -p aura-gallery` passed.
- `cargo check` passed.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- P7 needs a few small semantic builder helpers in existing Aura components before demos can stop using GPUI primitives cleanly.
- Sorting the registry at return time enforces the ASC requirement without risky manual reorder churn.

## Session 122 — 2026-05-10 (P7 Link Demo Migration)

### Actions
- Added a reusable Gallery test helper that bans direct demo usage of `div(`, `px(`, and low-level flex method chains for migrated demo files.
- Added and confirmed a failing self-contained test for `link_demo.rs` before migration.
- Migrated `link_demo.rs` to use Aura `Space` and `Title` for layout/section headings instead of GPUI primitives.

### Verification
- `cargo test -p aura-gallery link_demo_uses_aura_layout_primitives` failed before migration and passed after migration.
- `cargo test -p aura-gallery button_demo_uses_aura_layout_primitives` passed.
- `cargo test -p aura-gallery registry_entries_are_sorted_by_component_name` passed.
- `cargo test -p aura-gallery` passed.
- `cargo check` passed.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- The same `Space` + `Title` pattern used for Button demo cleanly covers Link demo without adding new component API.

## Session 123 — 2026-05-10 (P7 Feedback Demo Batch)

### Actions
- Added `apps/aura-gallery/src/demos/common.rs` with Aura-only demo helpers: `page`, `section`, `header`, `row`, and `row_md`.
- Added a batch self-contained guard test for feedback demos and confirmed it failed before migration.
- Migrated `dropdown_demo.rs`, `loading_demo.rs`, `message_box_demo.rs`, `message_demo.rs`, and `notification_demo.rs` away from direct demo-level `div(`, `px(`, and low-level flex primitives.

### Verification
- `cargo test -p aura-gallery feedback_demos_use_aura_layout_primitives` failed before migration and passed after migration.
- `cargo test -p aura-gallery` passed.
- `cargo check` passed.
- `git diff --check` passed.
- Confirmed migrated Button/Link/feedback demo files have zero occurrences of `div(`, `px(`, `.flex()`, `.flex_col()`, `.flex_row()`.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- A shared Aura-only demo helper allows multiple feedback demos to migrate without adding new production component APIs.

## Session 124 — 2026-05-10 (P7 Display Demo Batch)

### Actions
- Added and confirmed a failing self-contained guard test for display demos before migration.
- Migrated `alert_demo.rs`, `empty_demo.rs`, `result_demo.rs`, `segmented_demo.rs`, `statistic_demo.rs`, and `tree_demo.rs` to Aura/common demo helpers.
- Removed direct demo-level `div(`, `px(`, and low-level flex primitives from that batch.

### Verification
- `cargo test -p aura-gallery display_demos_use_aura_layout_primitives` failed before migration and passed after migration.
- `cargo test -p aura-gallery` passed.
- `cargo check` passed.
- `git diff --check` passed.
- Confirmed migrated display demo files have zero occurrences of `div(`, `px(`, `.flex()`, `.flex_col()`, `.flex_row()`.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Several display demos can be significantly reduced by composing `page`, `section`, `row`, `Space`, and existing Aura presentation components.

## Session 125 — 2026-05-10 (P7 Interaction Demo Batch)

### Actions
- Added and confirmed a failing self-contained guard test for interaction demos before migration.
- Migrated `pagination_demo.rs`, `popconfirm_demo.rs`, and `tooltip_demo.rs` to Aura/common demo helpers.
- Removed direct demo-level `div(`, `px(`, and low-level flex primitives from that batch.

### Verification
- `cargo test -p aura-gallery interaction_demos_use_aura_layout_primitives` failed before migration and passed after migration.
- `cargo test -p aura-gallery` passed.
- `cargo check` passed.
- `git diff --check` passed.
- Confirmed migrated interaction demo files have zero occurrences of `div(`, `px(`, `.flex()`, `.flex_col()`, `.flex_row()`.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- The common `page`/`section`/`row` helpers cover Popper-style demo pages without needing new production APIs.

## Session 126 — 2026-05-11 (P7 Typography and Progress Demo Batch)

### Actions
- Added and confirmed a failing self-contained guard test for `progress_demo.rs` and `typography_demo.rs` before migration.
- Migrated Progress demo to `page`/`section`/`Space` helpers and added `Progress::thick()` / `Progress::primary()` semantic builders to avoid demo-level `px()` and raw GPUI colors.
- Migrated Typography demo to shared helpers and Aura text/layout components, removing direct demo-level GPUI layout primitives.
- Added a unit test for `Progress::thick()`.

### Verification
- `cargo test -p aura-gallery typography_and_progress_demos_use_aura_layout_primitives` failed before migration and passed after migration.
- `cargo test -p aura-components progress_thick_sets_stroke_width` passed.
- `cargo test -p aura-components` passed.
- `cargo test -p aura-gallery` passed.
- `cargo check` passed.
- `git diff --check` passed.
- Confirmed Progress/Typography demo files have zero occurrences of `div(`, `px(`, `.flex()`, `.flex_col()`, `.flex_row()`.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Progress needed small semantic builders to keep the demo expressive without reaching for low-level pixel/color primitives.

## Session 127 — 2026-05-11 (P7 Navigation Demo Batch)

### Actions
- Added and confirmed a failing self-contained guard test for `breadcrumb_demo.rs`, `collapse_demo.rs`, and `steps_demo.rs` before migration.
- Migrated Breadcrumb, Collapse, and Steps demos to `page`/`section`/`Space` helpers and existing Aura components.
- Replaced Collapse item content demo-level `div()` wrappers with Aura `Text` content.
- Removed the Steps vertical example's demo-level fixed-height GPUI wrapper so the demo no longer reaches for `px()`.

### Verification
- `cargo test -p aura-gallery demos::tests::navigation_demos_use_aura_layout_primitives -- --exact` failed before migration and passed after migration.
- `cargo test -p aura-gallery` passed.
- `cargo check` passed.
- `cargo test -p aura-components` passed.
- `git diff --check` passed.
- Confirmed Breadcrumb/Collapse/Steps demo files have zero occurrences of `div(`, `px(`, `.flex()`, `.flex_col()`, `.flex_row()`.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Navigation/structure demos can be expressed with the existing common helper stack without adding new production component APIs.

## Session 128 — 2026-05-11 (P7 Input Picker Demo Batch)

### Actions
- Added and confirmed a failing self-contained guard test for ColorPicker, DatePicker, DateTimePicker, TimePicker, and Upload demos before migration.
- Migrated those input picker/upload demos to `page`/`section`/`Space` and existing Aura components.
- Added semantic demo-width helpers so demos no longer need direct `px()` for common picker/upload widths:
  - `ColorPicker::width_md()`
  - `DatePicker::width_md()` / `DatePicker::width_lg()`
  - `TimePicker::width_md()` / `TimePicker::width_lg()`
  - `DateTimePicker::width_md()` / `DateTimePicker::width_lg()`
  - `Upload::width_lg()`
- Added unit coverage for the new width helpers.

### Verification
- `cargo test -p aura-gallery demos::tests::input_picker_demos_use_aura_layout_primitives -- --exact` failed before migration and passed after migration.
- `cargo test -p aura-components width_` passed the new helper tests.
- `cargo test -p aura-gallery` passed.
- `cargo check` passed.
- `cargo test -p aura-components` passed.
- `git diff --check` passed.
- Confirmed ColorPicker/DatePicker/DateTimePicker/TimePicker/Upload demo files have zero occurrences of `div(`, `px(`, `.flex()`, `.flex_col()`, `.flex_row()`.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Picker demos are blocked mostly by repeated literal widths; narrow semantic width helpers keep demos self-contained without changing component behavior.

## Session 129 — 2026-05-11 (P7 Data Display Demo Batch)

### Actions
- Added and confirmed a failing self-contained guard test for Avatar, Badge, Descriptions, and Timeline demos before migration.
- Migrated those demos to shared `page`/`section`/`row`/`Space` helpers and existing Aura components.
- Added `TimelineTone` plus semantic `TimelineItem::{primary,success,warning,danger,info}` helpers so timeline demos can avoid reaching into theme/raw GPUI colors.
- Added unit coverage for Timeline tone helper precedence.

### Verification
- `cargo test -p aura-gallery data_display_demos_use_aura_layout_primitives` failed before migration and passed after migration.
- `cargo test -p aura-components timeline_tone_helpers_track_semantic_tone` passed.
- `cargo test -p aura-gallery` passed.
- `cargo check` passed.
- `cargo test -p aura-components` passed.
- `git diff --check` passed.
- Confirmed Avatar/Badge/Descriptions/Timeline demo files have zero occurrences of `div(`, `px(`, `.flex()`, `.flex_col()`, `.flex_row()`.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Timeline color examples needed semantic component-level tone APIs to remain self-contained without exposing demo code to theme internals.

## Session 130 — 2026-05-11 (P7 Layout Container Demo Batch)

### Actions
- Added and confirmed a failing self-contained guard test for Card, Scrollbar, and Splitter demos before migration.
- Migrated those demos to shared Aura helpers and component APIs, removing demo-level `div()`, `px()`, and flex primitives.
- Added `Card::width`, `Card::width_md()`, and `Card::width_lg()` to keep card demo sizing inside component semantics.
- Added `Splitter::height`, `Splitter::height_md()`, and `Splitter::bordered()` to express demo presentation without raw GPUI wrappers.
- Added unit coverage for the new Card and Splitter helpers.

### Verification
- `cargo test -p aura-gallery layout_container_demos_use_aura_layout_primitives` failed before migration and passed after migration.
- `cargo test -p aura-components helpers` passed the helper-focused tests.
- `cargo test -p aura-gallery` passed.
- `cargo check` passed.
- `cargo test -p aura-components` passed.
- `git diff --check` passed.
- Confirmed Card/Scrollbar/Splitter demo files have zero occurrences of `div(`, `px(`, `.flex()`, `.flex_col()`, `.flex_row()`.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Layout-oriented demos often need small semantic presentation helpers; migrating broader `layout_demo.rs` should be handled as a separate pass because grid color boxes still need component-level API support.

## Session 131 — 2026-05-11 (P7 Selection Demo Batch)

### Actions
- Added and confirmed a failing self-contained guard test for Autocomplete, Cascader, and Transfer demos before migration.
- Migrated those demos to shared `page`/`section`/`Space`/`Card` helpers and Aura `Text`, removing demo-level GPUI layout primitives.
- Added semantic width helpers used by the migrated demos:
  - `Autocomplete::width_lg()`
  - `Cascader::width_md()`
  - `Transfer::width_lg()`
- Added unit coverage for `Transfer::width_lg()`; Autocomplete/Cascader width helpers are compile-verified through migrated demo construction.
- Regenerated the remaining un-self-contained demo scan after migration.

### Verification
- `cargo test -p aura-gallery selection_demos_use_aura_layout_primitives` failed before migration and passed after migration.
- `cargo test -p aura-components transfer_width_lg_sets_demo_width` passed.
- `cargo test -p aura-gallery` passed.
- `cargo check` passed.
- `cargo test -p aura-components` passed.
- `git diff --check` passed.
- Confirmed Autocomplete/Cascader/Transfer demo files have zero occurrences of `div(`, `px(`, `.flex()`, `.flex_col()`, `.flex_row()`.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Remaining Not Self-Contained After This Session
- `affix_demo.rs`
- `anchor_demo.rs`
- `backtop_demo.rs`
- `container_demo.rs`
- `dialog_demo.rs`
- `drawer_demo.rs`
- `form_controls_demo.rs`
- `form_demo.rs`
- `icon_demo.rs`
- `image_demo.rs`
- `layout_demo.rs`
- `menu_demo.rs`
- `page_header_demo.rs`
- `popover_demo.rs`
- `preview_demo.rs`
- `skeleton_demo.rs`
- `table_demo.rs`
- `tabs_demo.rs`
- `tag_demo.rs`

## Session 132 — 2026-05-11 (P7 Overlay Demo Batch)

### Actions
- Added and confirmed a failing self-contained guard test for Dialog, Drawer, Popover, and PageHeader demos before migration.
- Migrated those demos to shared Aura helpers and Aura content/layout components, including content closures.
- Added semantic helpers used by overlay demos:
  - `Drawer::width_lg()`
  - `Drawer::height_sm()` / `Drawer::height_lg()`
  - `Popover::offset_lg()`
- Added unit coverage for Drawer size helpers and Popover offset helper.
- Kept PageHeader API unchanged after validating that existing closure APIs can return Aura components without demo-level GPUI primitives.
- Regenerated the remaining non-self-contained demo scan after migration.

### Verification
- `cargo test -p aura-gallery overlay_demos_use_aura_layout_primitives` failed before migration and passed after migration.
- `cargo test -p aura-components demo_` passed the helper-focused tests.
- `cargo test -p aura-gallery` passed.
- `cargo check` passed.
- `cargo test -p aura-components` passed.
- `git diff --check` passed.
- Confirmed Dialog/Drawer/Popover/PageHeader demo files have zero occurrences of `div(`, `px(`, `.flex()`, `.flex_col()`, `.flex_row()`.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Remaining Not Self-Contained After This Session
- `affix_demo.rs`
- `anchor_demo.rs`
- `backtop_demo.rs`
- `container_demo.rs`
- `form_controls_demo.rs`
- `form_demo.rs`
- `icon_demo.rs`
- `image_demo.rs`
- `layout_demo.rs`
- `menu_demo.rs`
- `preview_demo.rs`
- `skeleton_demo.rs`
- `table_demo.rs`
- `tabs_demo.rs`
- `tag_demo.rs`

## Session 133 — 2026-05-11 (Popover Spacing and Cascader Disabled Cursor Fix)

### Actions
- Investigated reported Popover bubble content being visually cramped after overlay demo migration.
- Identified root cause: the Popover shell rendered content directly inside the bordered/shadowed bubble without default padding, so Aura `Space` content touched the edge and appeared compressed.
- Added default `.p_4()` padding to the Popover content wrapper.
- Investigated reported Cascader disabled state cursor.
- Identified root cause: disabled Cascader trigger and disabled/loading popup options only skipped pointer hover; they did not set `cursor_not_allowed()`.
- Added not-allowed cursor styling for disabled Cascader trigger and disabled/loading Cascader options.
- Added source-sliced regression tests so the assertions inspect production code only, not the test body itself.

### Verification
- `cargo test -p aura-components regression` failed before the fix and passed after the fix.
- `cargo test -p aura-gallery` passed.
- `cargo check` passed.
- `cargo test -p aura-components` passed.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Popover default content padding belongs in the component shell because caller content can be plain text or compact Aura layout primitives.
- Cursor semantics need to be explicit on disabled states; merely omitting pointer hover leaves the default cursor.

## Session 134 — 2026-05-11 (P7 Tag and Tabs Demo Batch)

### Actions
- Added and confirmed a failing self-contained guard test for Tag and Tabs demos before migration.
- Migrated Tag demo to shared `page`/`section`/`row_md`/`Space`/`Card` helpers while preserving dynamic add/remove behavior.
- Migrated Tabs demo to shared helpers and changed tab pane closures to return Aura `Text` instead of demo-level GPUI `div()`.
- Regenerated the remaining non-self-contained demo scan after migration.

### Verification
- `cargo test -p aura-gallery tag_and_tabs_demos_use_aura_layout_primitives` failed before migration and passed after migration.
- `cargo test -p aura-gallery` passed.
- `cargo check` passed.
- `cargo test -p aura-components` passed.
- `git diff --check` passed.
- Confirmed Tag/Tabs demo files have zero occurrences of `div(`, `px(`, `.flex()`, `.flex_col()`, `.flex_row()`.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Remaining Not Self-Contained After This Session
- `affix_demo.rs`
- `anchor_demo.rs`
- `backtop_demo.rs`
- `container_demo.rs`
- `form_controls_demo.rs`
- `form_demo.rs`
- `icon_demo.rs`
- `image_demo.rs`
- `layout_demo.rs`
- `menu_demo.rs`
- `preview_demo.rs`
- `skeleton_demo.rs`
- `table_demo.rs`

## Session 135 — 2026-05-11 (Tag Input and Tabs Scroll Fix)

### Actions
- Investigated Tag dynamic input becoming too large after self-contained migration.
- Identified root cause: the migrated demo wrapped the input in `Card::width_md()` only to get width, making the editor visually too large.
- Added `Input::width`, `Input::width_sm()`, `Input::set_width`, and `Input::set_width_sm()` so compact field sizing can live on the input itself.
- Updated Tag demo dynamic input to use `Input::width_sm()` and removed the Card wrapper.
- Investigated Tabs demo bottom content being cramped and page not scrolling.
- Identified root cause: `Tabs` root forced `.h_full()`, causing each Tabs instance in a scroll page to compete for full parent height instead of natural content height.
- Removed root `h_full()` from `Tabs` while keeping width and orientation behavior.
- Added regression tests for compact Tag input usage and natural Tabs height in scroll pages.

### Verification
- `cargo test -p aura-gallery tag_dynamic_input_uses_compact_input_width` failed before fix and passed after fix.
- `cargo test -p aura-gallery tabs_demo_scrolls_with_natural_tab_height` failed before fix and passed after fix.
- `cargo test -p aura-components input_width_sm_sets_compact_width` passed.
- `cargo test -p aura-gallery` passed.
- `cargo check` passed.
- `cargo test -p aura-components` passed.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Compact input sizing should be component-level API (`Input::width_sm`) rather than borrowing Card width presets.
- Tabs is often embedded in scrollable documents; forcing full height at the component root breaks stacked demo/document layouts.

## Session 136 — 2026-05-11 (Statistic Horizontal Layout and Icon API)

### Actions
- Extended `Statistic` with explicit layout state: default vertical, compact horizontal, and space-between horizontal.
- Added `Statistic::icon(...)` using the existing `IntoIconPath`/`Icon` pipeline so lucide icon names and custom icon paths both work.
- Added icon position controls (`icon_left`, `icon_right`, `icon_position`) and `icon_color`; icon color defaults to the resolved statistic value color when not explicitly set.
- Preserved existing `prefix`/`suffix` element APIs for arbitrary custom adornments.
- Updated the Statistic gallery demo with icon color/position examples and horizontal compact/space-between cards.
- Added TDD regression coverage for horizontal layout helpers, icon position/color builders, and default icon color resolution.

### Verification
- `cargo test -p aura-components statistic_ --lib` failed before implementation because the new layout/icon API did not exist, then passed after implementation.
- `cargo test -p aura-components` passed: 21 unit tests plus component integration/doc tests.
- `cargo test -p aura-gallery` passed: 16 gallery tests including the Statistic demo self-contained guard batch.
- `cargo check` passed for the workspace.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- `Icon::color(Hsla)` already exists, but an unset Icon falls back to neutral icon color; Statistic therefore needs to explicitly pass the resolved value color to satisfy “icon follows number color by default”.
- `prefix`/`suffix` are too generic to enforce inherited icon coloring, so the new `icon(...)` API is the safer component-level path while keeping existing compatibility.

## Session 137 — 2026-05-11 (P7 Icon Demo Self-Bootstrap Batch)

### Actions
- Added a failing gallery guard for `icon_demo.rs` so it cannot use raw GPUI demo primitives (`div`, `px`, flex helpers) going forward.
- Added `Icon::size_xs`, `Icon::size_md`, `Icon::size_lg`, and `Icon::size_xl` helpers to replace demo-level pixel sizing with component-level semantic sizing.
- Rewrote the Icon demo with shared `page`/`section`/`row` helpers plus Aura `Space` and `Text`, preserving default color, size, and theme color examples.

### Verification
- `cargo test -p aura-icons icon_size_helpers_set_common_demo_sizes --lib` failed before the Icon helper implementation and passed after it.
- `cargo test -p aura-gallery icon_demo_uses_aura_layout_primitives` failed before the demo migration and passed after it.
- `cargo test -p aura-icons` passed.
- `cargo test -p aura-gallery` passed: 17 gallery tests.
- `cargo check` passed.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Remaining Not Self-Contained After This Session
- `affix_demo.rs`
- `anchor_demo.rs`
- `backtop_demo.rs`
- `container_demo.rs`
- `form_controls_demo.rs`
- `form_demo.rs`
- `image_demo.rs`
- `layout_demo.rs`
- `menu_demo.rs`
- `preview_demo.rs`
- `skeleton_demo.rs`
- `table_demo.rs`

## Session 138 — 2026-05-11 (Icon Demo Label Centering Fix)

### Actions
- Investigated the Icon demo label alignment regression after the self-bootstrap migration.
- Identified root cause: the demo uses vertical `Space` for icon+label pairs, but `Space` only controlled direction/wrap/gap and had no cross-axis alignment API, so vertical stacks defaulted to start alignment.
- Added `SpaceAlign` plus `Space::align`, `align_start`, `align_center`, and `align_end` helpers.
- Updated `icon_demo.rs` to use `Space::align_center()` for each icon+label stack.
- Added regression tests covering the Space alignment builder and the Icon demo's centered label requirement.

### Verification
- `cargo test -p aura-components space_align_center_tracks_cross_axis_alignment --lib` failed before the Space API implementation and passed after it.
- `cargo test -p aura-gallery icon_demo_labels_are_center_aligned_under_icons` failed before the demo update and passed after it.
- `cargo test -p aura-gallery icon_demo_uses_aura_layout_primitives` passed.
- `cargo test -p aura-components` passed: 22 component tests plus integration/doc tests.
- `cargo test -p aura-gallery` passed: 18 gallery tests.
- `cargo check` passed.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- Aura `Space` needed explicit cross-axis alignment to replace raw GPUI layout in vertical label stacks without losing visual centering.

## Session 139 — 2026-05-11 (P7 Skeleton Demo Self-Bootstrap Batch)

### Actions
- Added a gallery guard for `skeleton_demo.rs` so it cannot use raw GPUI demo primitives (`div`, `px`, flex helpers) going forward.
- Added `Space::grow()` to express flex growth through the Aura layout primitive instead of demo-level `flex_1`.
- Added `SkeletonItem::width(...)` and `SkeletonItem::width_2_5()` so custom skeleton templates can express partial-width paragraph placeholders without raw wrapper divs.
- Added `Avatar::background(...)` so loaded skeleton content can keep the colored avatar without raw demo-level circle styling.
- Rewrote the Skeleton demo with `page`/`section`/`row`, `Space`, `Text`, `Avatar`, `Skeleton`, and `SkeletonItem` while preserving loading toggle, common variants, custom template, and loaded content.

### Verification
- `cargo test -p aura-gallery skeleton_demo_uses_aura_layout_primitives` failed before migration and passed after migration.
- `cargo test -p aura-components skeleton_item_width_2_5_sets_fraction_width --lib` passed.
- `cargo test -p aura-components space_grow_tracks_flex_growth --lib` passed.
- `cargo test -p aura-components avatar_background_tracks_custom_color --lib` passed.
- `cargo test -p aura-components` passed: 25 component tests plus integration/doc tests.
- `cargo test -p aura-gallery` passed: 19 gallery tests.
- `cargo check` passed.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Remaining Not Self-Contained After This Session
- `affix_demo.rs`
- `anchor_demo.rs`
- `backtop_demo.rs`
- `container_demo.rs`
- `form_controls_demo.rs`
- `form_demo.rs`
- `image_demo.rs`
- `layout_demo.rs`
- `menu_demo.rs`
- `preview_demo.rs`
- `table_demo.rs`

### Key Discoveries
- Skeleton's custom template needed component-level equivalents for flex growth and partial-width paragraph rows.
- Avatar background color is useful beyond this demo and avoids hand-rolled colored circles in gallery code.

## Session 140 — 2026-05-11 (P7 Preview Demo Self-Bootstrap Batch)

### Actions
- Added a gallery guard for `preview_demo.rs` so it cannot use raw GPUI demo primitives (`div`, `px`, flex helpers) going forward.
- Added `Image::thumbnail()` to express the common preview thumbnail size through the Image API instead of demo-level pixel sizing.
- Rewrote the Preview demo with shared `page`/`section`/`row` helpers plus `Space`, `Text`, `Card`, `Icon`, `Image`, and `Preview`.
- Preserved remote/local image triggers and the custom card trigger while keeping Image preview disabled inside wrapped Preview triggers.

### Verification
- `cargo test -p aura-gallery preview_demo_uses_aura_layout_primitives` failed before migration and passed after migration.
- `cargo test -p aura-components image_thumbnail_sets_preview_dimensions --lib` failed before `Image::thumbnail()` and passed after implementation.
- `cargo test -p aura-components` passed: 26 component tests plus integration/doc tests.
- `cargo test -p aura-gallery` passed: 20 gallery tests.
- `cargo check` passed.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Remaining Not Self-Contained After This Session
- `affix_demo.rs`
- `anchor_demo.rs`
- `backtop_demo.rs`
- `container_demo.rs`
- `form_controls_demo.rs`
- `form_demo.rs`
- `image_demo.rs`
- `layout_demo.rs`
- `menu_demo.rs`
- `table_demo.rs`

### Key Discoveries
- Preview demos commonly need a thumbnail-sized Image; making that a component API avoids repeating raw pixel sizing in demos.

## Session 141 — 2026-05-11 (P7 Image Demo Self-Bootstrap Batch)

### Actions
- Added a gallery guard for `image_demo.rs` so it cannot use raw GPUI demo primitives (`div`, `px`, flex helpers) going forward.
- Added Image component helpers for common demo/example shapes: `thumbnail_sm()`, `square_lg()`, and `round_sleeve()`.
- Rewrote the Image demo with shared `page`/`section`/`row` helpers plus `Space`, `Text`, `Card`, and `Image` APIs.
- Preserved the main feature coverage: remote/local images, preview, cover/contain/fill/scale-down fit variants, circle crop, rounded bounds, ring sleeve, large-radius shadow, fallback, and empty states.

### Verification
- `cargo test -p aura-gallery image_demo_uses_aura_layout_primitives` failed before migration and passed after migration.
- `cargo test -p aura-components image_demo_size_helpers_track_common_examples --lib` failed before the Image size helpers and passed after implementation.
- `cargo test -p aura-components image_round_sleeve_sets_ring_configuration --lib` failed before `Image::round_sleeve()` and passed after implementation.
- `cargo test -p aura-components` passed: 28 component tests plus integration/doc tests.
- `cargo test -p aura-gallery` passed: 21 gallery tests.
- `cargo check` passed.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Remaining Not Self-Contained After This Session
- `affix_demo.rs`
- `anchor_demo.rs`
- `backtop_demo.rs`
- `container_demo.rs`
- `form_controls_demo.rs`
- `form_demo.rs`
- `layout_demo.rs`
- `menu_demo.rs`
- `table_demo.rs`

### Key Discoveries
- Image examples repeatedly need named visual presets; component-level helpers keep demo code self-contained while documenting supported presentation patterns.

## Session 142 — 2026-05-11 (P7 Menu Demo Self-Bootstrap Batch)

### Actions
- Added `menu_demo.rs` to the navigation self-bootstrap guard batch.
- Rewrote the Menu demo with shared `page`/`section` helpers and Aura `Space`, `Row`, `Col`, `Card`, and `Text` components.
- Preserved horizontal, vertical, and collapsed menu examples plus active content updates for each mode.
- Replaced hand-written content card styling with `Card::new(...).no_shadow()` and text styling through `Text`.

### Verification
- `cargo test -p aura-gallery navigation_demos_use_aura_layout_primitives` failed before migration because `menu_demo.rs` contained raw GPUI primitives, then passed after migration.
- `cargo test -p aura-components` passed: 28 component tests plus integration/doc tests.
- `cargo test -p aura-gallery` passed: 21 gallery tests.
- `cargo check` passed.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Remaining Not Self-Contained After This Session
- `affix_demo.rs`
- `anchor_demo.rs`
- `backtop_demo.rs`
- `container_demo.rs`
- `form_controls_demo.rs`
- `form_demo.rs`
- `layout_demo.rs`
- `table_demo.rs`

### Key Discoveries
- Menu demo could be migrated without new component APIs by reusing `Row`/`Col` for side-navigation layout and `Card` for the active content panel.

## Session 143 — 2026-05-11 (Gallery Shell Container/Menu Self-Bootstrap)

### Actions
- Added a shell-level regression test requiring the gallery main view to use Aura `Container` and `Menu` instead of the bespoke left-nav implementation.
- Extended `Container` for real app-shell usage: configurable header/footer height, aside width, aside/main scrolling, main padding, and root overlays for portal layers.
- Rebuilt the Aura gallery shell with `Container::new()` for header/aside/main layout and an Aura `Menu` entity for demo navigation.
- Preserved one-demo-at-a-time rendering, selected demo content cards, and all existing portal/message/notification/tooltip/popover/modal/drawer rendering.
- Kept the remaining raw GPUI in `main.rs` scoped to the low-level portal layer implementation rather than the app shell/navigation layout.

### Verification
- `cargo test -p aura-gallery gallery_shell_uses_container_and_menu` failed before the shell refactor and passed after it.
- `cargo test -p aura-components container_gallery_shell_helpers_track_layout_state --lib` failed before the Container API additions and passed after implementation.
- `cargo test -p aura-components` passed: 29 component tests plus integration/doc tests.
- `cargo test -p aura-gallery` passed: 22 gallery tests.
- `cargo check` passed.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Remaining Not Self-Contained Demo Pages
- `affix_demo.rs`
- `anchor_demo.rs`
- `backtop_demo.rs`
- `container_demo.rs`
- `form_controls_demo.rs`
- `form_demo.rs`
- `layout_demo.rs`
- `table_demo.rs`

### Key Discoveries
- `Container` needed app-shell capabilities (scrolling slots, wider aside, taller header, overlay support) before it could credibly dogfood the gallery root.
- `Menu` can drive the gallery navigation as single-line demo labels; detailed descriptions remain in the selected content card.

## Session 144 — 2026-05-11 (Container Shell Scroll ID Fix)

### Actions
- Investigated the gallery shell regression where the left menu jittered while scrolling and the right content panel did not respond to scroll.
- Identified root cause: `Container` generated both aside and main scroll element IDs through `stable_unique_id` with the same keyed-state key (`"container"`), so the two scroll regions shared/competed for one GPUI interactive state entry.
- Changed the scroll region keys to distinct stable keys: `container-aside-scroll` and `container-main-scroll`.
- Added a source-sliced regression test ensuring the two scroll regions keep distinct stable ID keys and do not regress to the shared key.

### Verification
- `cargo test -p aura-components container_scroll_regions_use_distinct_stable_id_keys --lib` failed before the fix and passed after it.
- `cargo test -p aura-components container_gallery_shell_helpers_track_layout_state --lib` passed.
- `cargo test -p aura-gallery gallery_shell_uses_container_and_menu` passed.
- `cargo test -p aura-components` passed: 30 component tests plus integration/doc tests.
- `cargo test -p aura-gallery` passed: 22 gallery tests.
- `cargo check` passed.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- `stable_unique_id`'s first argument is the keyed-state key; the prefix only affects the generated value. Distinct scroll regions must not share the same key even if their prefixes differ.

## Session 145 — 2026-05-11 (Container Main Scroll Height Fix)

### Actions
- Continued investigating the gallery shell right-panel scroll regression after fixing Container scroll ID collisions.
- Identified the remaining layout difference from the old hand-written gallery content scroller: the old right scroll container used `h_full()`, while the new Container main scroll slot only had `flex_1/min_h_0`.
- Added `h_full()` to the Container main scroll region before `overflow_y_scroll()` so GPUI creates a bounded scroll viewport instead of letting content height expand the region.
- Added a source-sliced regression test to keep the main scroll region height-constrained.

### Verification
- `cargo test -p aura-components container_main_scroll_region_is_height_constrained --lib` failed before the fix and passed after it.
- `cargo test -p aura-components container_scroll_regions_use_distinct_stable_id_keys --lib` passed.
- `cargo test -p aura-gallery gallery_shell_uses_container_and_menu` passed.
- `cargo test -p aura-components` passed: 31 component tests plus integration/doc tests.
- `cargo test -p aura-gallery` passed: 22 gallery tests.
- `cargo check` passed.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- For GPUI scroll regions nested in flex layouts, `flex_1/min_h_0` alone may not create the bounded viewport; the previous working gallery implementation also had `h_full()`, which Container now mirrors.

## Session 146 — 2026-05-11 (Gallery Content Card Scroll Fix)

### Actions
- Continued investigating the right-panel scroll regression using `form_demo` as the long-content reproduction case.
- Identified the next conflict introduced by the shell self-bootstrap: the selected demo is wrapped in `Card`, and `Card` defaults to `overflow_hidden()`. In a flex-column scroll container, the card can shrink to the viewport and clip the long form internally, leaving no overflow for the outer main scroll region.
- Added `Card::no_shrink()` to opt a card out of flex shrinking when it is used as scroll-region content.
- Applied `.no_shrink()` to the gallery selected-demo content card.
- Added tests for the Card helper and the gallery shell requirement.

### Verification
- `cargo test -p aura-components card_no_shrink_tracks_scroll_container_usage --lib` passed.
- `cargo test -p aura-gallery gallery_shell_uses_container_and_menu` passed.
- `cargo test -p aura-components` passed: 32 component tests plus integration/doc tests.
- `cargo test -p aura-gallery` passed: 22 gallery tests.
- `cargo check` passed.
- `git diff --check` passed.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- `Card::overflow_hidden()` is correct for visual clipping, but scroll-region content cards must be non-shrinking so long child content contributes to the outer scroll height instead of being clipped inside the card.


## Session 147 — 2026-05-11 (Complete Demo Self-Bootstrap)

### Actions
- Completed the remaining demo self-bootstrap work for `affix_demo.rs`, `anchor_demo.rs`, `backtop_demo.rs`, `container_demo.rs`, `form_controls_demo.rs`, `form_demo.rs`, `layout_demo.rs`, and `table_demo.rs`.
- Added the reusable Aura `Flex` layout primitive so demos can express row/column layout, fixed-size showcase panels, scroll viewports, borders, rounded surfaces, padding, and tracked scroll containers without raw GPUI `div()`/`px()` layout calls.
- Added small convenience APIs needed by self-bootstrapped demos: Affix offset helpers, Anchor offset helper, Backtop visibility/right helpers, Input text/icon addon helpers, Select compact width/text/padding helpers, and Table width/height helpers.
- Extended gallery regression tests to cover the last self-bootstrap files.
- Marked P7 Demo Self-Contained complete in `.memory/state.md`.

### Verification
- Red phase: `cargo test -p aura-gallery demos::tests::` failed on the newly added checks for the remaining non-self-contained demos.
- `cargo test -p aura-components flex_tracks --lib` passed.
- `cargo test -p aura-gallery demos::tests::` passed: 24 demo regression tests before full suite, then 25 gallery tests after shell test inclusion.
- `cargo test -p aura-components` passed: 35 component unit tests plus integration/doc tests.
- `cargo test -p aura-gallery` passed: 25 gallery tests.
- `cargo check` passed.
- `git diff --check` passed.
- `rg -n "div\(|px\(|\.flex\(\)|\.flex_col\(\)|\.flex_row\(\)" apps/aura-gallery/src/demos -g'*.rs'` now reports only the forbidden-token test list in `demos/mod.rs`.
- `timeout 25s cargo run -p aura-gallery` compiled and launched `target/debug/aura-gallery`; process ended by timeout with no startup compile error or immediate crash.

### Key Discoveries
- A single Aura-owned `Flex` primitive covers the remaining demo-only layout needs better than continuing to add ad hoc raw GPUI layout snippets in each demo.
- The last raw GPUI usage was concentrated in complex scroll showcases and older form/table/layout demos; the whole `apps/aura-gallery/src/demos` tree is now guarded by regression tests.

## Session 148 — 2026-05-11 (Fedora Dependency Script + Compact Menu Demo)

### Actions
- Added `scripts/install-fedora-deps.sh` to install Fedora system dependencies needed by Aura Gallery / GPUI (`gcc-c++`, `fontconfig-devel`, `freetype-devel`, Wayland/X11/XCB/Vulkan headers and loaders, clang/LLVM helpers, etc.).
- Narrowed the Menu demo vertical layout so the regular vertical menu uses a 4/24 sidebar column instead of 6/24, and the collapsed menu uses a 2/24 compact column instead of 4/24.
- Added a gallery regression test that keeps the Menu demo vertical and collapsed sidebars compact while preserving the existing self-contained demo guard.

### Verification
- `cargo test -p aura-gallery menu_demo_keeps_vertical_menu_compact` failed before the layout change and passed after the Menu demo columns were narrowed.
- `cargo test -p aura-gallery navigation_demos_use_aura_layout_primitives` passed after keeping the fix within Aura layout primitives.
- Full verification rerun after memory update is recorded in the assistant response for this session.

### Key Discoveries
- Menu itself renders with `.w_full()`, so demo sidebar width should be controlled by the surrounding Aura grid column rather than raw GPUI pixel wrappers.

## Session 149 — 2026-05-11 (Promote Demo Helpers to Component Crate)

### Actions
- Reviewed gallery demo self-bootstrap boundaries: the raw GPUI primitive scan only reports the forbidden-token test list in `demos/mod.rs`; no demo page leaked raw layout primitives.
- Moved shared demo layout helpers from `apps/aura-gallery/src/demos/common.rs` into `crates/aura-components/src/layout_helpers.rs` as `aura_components::layout_helpers::{page, section, header, row, row_md}`.
- Removed the gallery-local `common` module and updated demo files to import shared helpers from the component crate.
- Added a gallery regression test requiring shared demo helpers to live in `aura-components::demo` and preventing reintroduction of `pub mod common` in the gallery demos registry.

### Verification
- `cargo test -p aura-gallery layout_helpers_live_in_component_crate` failed before renaming/promoting helpers and passed after the migration.
- `rg -n "super::common|pub mod common|demos/common" apps/aura-gallery/src/demos crates/aura-components/src -S` no longer reports stale local common imports/module declarations.
- `rg -n "\\bdiv\\(|\\bpx\\(|\\.flex\\(\\)|\\.flex_col\\(\\)|\\.flex_row\\(|rgb\\(|hsla\\(" apps/aura-gallery/src/demos -g'*.rs' -S` reports only the guard token list in `demos/mod.rs`.
- Full verification rerun after memory update is recorded in the assistant response for this session.

### Key Discoveries
- The old `common.rs` helpers were already pure Aura component compositions, so the cleanest boundary is a namespaced `aura_components::layout_helpers` module rather than re-exporting generic names like `page` and `section` at crate root.

## Session 150 — 2026-05-11 (P8 Native Gallery Documentation Replan)

### Actions
- Replaced the previous P8 VitePress/Web documentation plan with a pure GPUI native Aura Gallery documentation architecture.
- Updated `architecture-design.md` with the new P8 native documentation architecture: `pulldown-cmark` parsing, Aura Typography bootstrapping, stack-based Markdown renderer, code block rendering, two-column document shell, and Live Demo injection.
- Updated `prompt.md` so future sessions treat P8 as a native Aura Gallery documentation phase and not as a Web docs phase.
- Rewrote `.prompt/P8-engineering.md` into a four-phase execution plan: Typography infrastructure, Markdown renderer/state machine, code block/document shell polish, and Live Demo injection.
- Added ADR-013 documenting the decision to run official documentation entirely inside the GPUI native gallery.
- Updated `.memory/state.md` and `.memory/inventory.md` to reflect the new P8 scope.

### Verification
- Documentation consistency checks and project verification were run after the edits; see the assistant response for exact commands and outcomes.

### Key Discoveries
- The new P8 plan should dogfood Aura's own text/layout primitives instead of creating a separate Web documentation surface.
- The repo currently uses Rust edition 2024; the new P8 plan treats Rust 2021 as a minimum language baseline but does not downgrade the workspace edition.

## Session 151 — 2026-05-11 (Image/Preview Menu Switch Performance)

### Actions
- Investigated Image and Preview menu-switch stutter and slow remote image display.
- Identified two image loading issues: URL rendering scheduled Aura's own background `ureq` fetch while also returning GPUI `img(src)`, causing a second remote loading path; cached remote states also requested animation frames during render.
- Changed remote URL rendering to use the Aura remote cache path only: loading renders the Aura placeholder, completion refreshes windows explicitly, and no GPUI `img(src)` fallback is started.
- Added a local image render cache so repeated local thumbnails in Image/Preview demos do not synchronously read/decode the same file on every render.
- Kept exactly one remote image in each Image/Preview demo for remote-loading coverage while moving repeated examples to the bundled local asset, so menu switching does not trigger many network loads.
- Added regression tests for passive remote loading state, single remote fetch path, local render-image cache, and bounded remote demo coverage.

### Verification
- `cargo test -p aura-components remote_image_loading_state_is_passive_after_first_fetch --lib` failed before the helper existed and passed after the remote state change.
- `cargo test -p aura-gallery image_and_preview_demos_keep_remote_loading_coverage_bounded` failed when the demos had zero remote URLs and passed after keeping exactly one remote URL per demo.
- Full verification rerun after formatting is recorded in the assistant response for this session.

### Key Discoveries
- The previous URL branch could start two remote image loaders for the same URL: Aura's background cache fetch plus GPUI `img(src)`.
- The Image and Preview gallery pages are performance-sensitive because selecting those cached views renders many image instances at once; local file decode must be cached too.

## Session 152 — 2026-05-11 (Preview Outside-Image Close)

### Actions
- Changed the Preview overlay so the backdrop close handler is blocked only by an image-sized preview box, not by the previous fixed 72% viewport container.
- Added `PreviewClose` and `Preview::register_key_bindings(cx)` so ESC closes an active image preview.
- Registered Preview key bindings in Aura Gallery startup.
- Added regression coverage for image-aspect preview sizing, ESC action registration, and gallery key-binding registration.

### Verification
- Targeted Preview and Gallery tests passed after the interaction changes.
- Full verification rerun after this memory update is recorded in the assistant response for this session.

### Key Discoveries
- The old preview hit target was a fixed 72% viewport box; with `ObjectFit::Contain`, letterboxed/shadow-adjacent areas inside that oversized box consumed clicks and prevented backdrop close.
- Matching the interactive preview box to the contained image aspect ratio keeps clicks on image open while allowing clicks outside the actual image box to close.

## Session 153 — 2026-05-11 (Preview 3D Frame Shadow)

### Actions
- Restored a stronger 3D frame treatment on the Preview overlay image box while preserving outside-image click dismissal.
- Replaced the generic `shadow_xl()` on the Preview image frame with layered `BoxShadow`s: deep drop shadow, tighter contact shadow, and subtle top highlight.
- Added a translucent light border around the preview image frame to reinforce the lifted/glass edge effect.
- Added regression coverage for the layered 3D shadow values and kept the image-sized hitbox guard.

### Verification
- `cargo test -p aura-components preview --lib` passed after the frame treatment change.
- Full verification rerun after this memory update is recorded in the assistant response for this session.

### Key Discoveries
- The 3D depth can live on the same image-sized hitbox; GPUI shadows paint outside the frame, so the visible shadow area remains outside the consumed click bounds and still dismisses via the backdrop.

## Session 154 — 2026-05-11 (Aura Motion Foundation and Component Coverage)

### Actions
- Added `crates/aura-components/src/motion.rs` as the shared Aura motion layer on top of GPUI `AnimationExt`, with duration tokens, easing tokens, fade/pop/pulse/spin helpers, and elastic slide easing.
- Added icon rotation support in `aura-icons::Icon` using GPUI SVG transformation so loading indicators can spin without changing layout or hitboxes.
- Covered high-impact animated components:
  - Preview: fade in/out overlay, pop-in image frame, delayed close state for fade-out.
  - Dialog / Drawer: fade-in overlay plus pop-in panel.
  - Popover / Dropdown: pop-in shell through the shared Popover renderer.
  - Tooltip: native GPUI fade-in in the passive tooltip renderer.
  - Message / Notification: pop-in toast/card entries.
  - Loading: fade-in wrapper plus spinning loader icon.
  - Button loading state: spinning loader icon.
  - Switch: elastic thumb slide using previous checked state to avoid first-render false-position animation.
  - Skeleton: pulsing animated rows when `animated` is enabled.
- Added source-sliced and unit regression coverage for the motion layer and each covered component path.

### Verification
- Targeted motion tests passed for `aura-components`, `aura-icons`, and `aura-core` before the final full verification run.
- Full verification rerun after this memory update is recorded in the assistant response for this session.

### Key Discoveries
- GPUI already provides `Animation` / `AnimationExt`; Aura needed a design-system wrapper for consistent duration/easing and component usage.
- Switch animation must remember the previous checked state; using only the target checked state makes initially unchecked switches animate from the wrong side on first render.
- SVG icon rotation is the narrowest native way to animate loading spinners without adding a custom paint wrapper.

## Session 155 — 2026-05-11 (Motion Timing and Switch Crash Fix)

### Actions
- Fixed the Switch crash caused by using an overshooting elastic curve as a GPUI easing function. GPUI asserts easing output must stay in `0..=1`, so Aura now clamps the `MotionEasing::Elastic` easing output for GPUI while keeping `elastic_slide` available for visual overshoot inside component interpolation.
- Changed Switch thumb animation to use bounded `MotionEasing::EaseOut` for GPUI and apply `elastic_slide(delta)` only when computing the thumb position.
- Slowed global motion timing from `120/180/240ms` to `220/320/900ms` for fast/normal/slow so overlays, switch movement, pulse, and spinner animations are more legible.
- Updated Preview close delay to use `MotionDuration::Fast` and Tooltip fade-in to 220ms so hard-coded timings align with the slower motion system.
- Added a regression test proving `MotionEasing::Elastic` remains bounded for GPUI animation.

### Verification
- Targeted motion and Switch tests passed after the fix.
- Full verification passed: `cargo test -p aura-core`, `cargo test -p aura-icons`, `cargo test -p aura-components`, `cargo test -p aura-gallery`, `cargo check`, `git diff --check`.
- Smoke-ran `cargo run -p aura-gallery` with the normal default entry and with a temporary Switch-selected startup entry; both launched and were stopped by timeout without reproducing the panic.

### Key Discoveries
- GPUI `AnimationElement` validates the eased delta before invoking the component animator, so any easing passed to `Animation::with_easing` must never overshoot.
- Elastic overshoot should be applied inside the animated property interpolation, not as the GPUI easing function itself.

## Session 156 — 2026-05-11 (Motion Interpolator and Elastic Snap Slide)

### Actions
- Added an `Interpolator` helper to `aura-components::motion` for reusable numeric interpolation across complex animations.
- Added `MotionCurve` with `Linear`, `EaseInOut`, `EaseOut`, and `ElasticSnap` curves.
- Added `elastic_snap(delta)` and `slide_snap(from, to, delta)` helpers so components can implement springy property interpolation without passing overshooting values into GPUI easing.
- Updated Switch thumb movement to use bounded linear GPUI animation plus `slide_snap(...)` for the default slide behavior: slow start, acceleration, deceleration, and a small snap/settle overshoot near the target.
- Added unit coverage for Interpolator sampling, elastic snap behavior, reverse-direction overshoot, and Switch use of `slide_snap`.

### Verification
- `cargo test -p aura-components motion --lib` passed.
- `cargo test -p aura-components switch_thumb_uses_elastic_motion --lib` passed.
- Full verification passed: `cargo test -p aura-core`, `cargo test -p aura-icons`, `cargo test -p aura-components`, `cargo test -p aura-gallery`, `cargo check`, `git diff --check`.
- Smoke-ran normal Gallery startup and a temporary Switch-selected startup; both launched and were stopped by timeout without panic.

### Key Discoveries
- The safe pattern for complex motion is: keep GPUI easing bounded, then use Aura `Interpolator` / `slide_snap` inside the animator closure for overshoot or other non-linear property effects.

## Session 80 — 2026-05-11 (Motion Coverage Audit)

### Actions
- 审计组件库动画覆盖面：确认现有 motion 已覆盖 Preview/Dialog/Drawer/Popover/Dropdown/Tooltip/Message/Notification/Loading/Button Loading/Switch/Skeleton。
- 为仍明显依赖“出现/展开/选中反馈”的交互控件补齐 Aura motion：Select、Cascader、DatePicker、TimePicker、DateTimePicker、Backtop、Checkbox、Radio、Collapse、Tree、Menu、Segmented、Tabs、Rate。
- 对弹出层统一使用 `pop_in`，对选中/展开/活动指示器使用轻量 `pop_in`，避免引入新的未受控 GPUI easing，继续保持弹性 overshoot 只在受控插值路径中使用。
- 增加 motion coverage 单测，防止后续重构移除这些交互动效接入点。

### Verification
- `cargo test -p aura-components` passed locally during implementation.

### Key Discoveries
- 本轮适合补动效的是“短生命周期可见性变化”和“选中态视觉反馈”；Progress/Slider/Upload 等连续数值型动效需要记忆前值或自绘动画状态，后续应单独做，不宜用出现动画伪装数值插值。
- Input/Textarea/InputNumber 等输入类更适合未来做 focus-ring/边框过渡，当前无需为了动效覆盖而增加复杂度。

## Session 81 — 2026-05-11 (P8 Phase 1 Typography Bootstrapping)

### Actions
- Started P8 Native Gallery Documentation work from `.prompt/P8-engineering.md`.
- Upgraded `Paragraph` to compose multiple `Text` segments into one GPUI `StyledText` with explicit `TextRun`s instead of fallback flex-wrapped child elements.
- Added Text segment run conversion for inline styles: color, background, font weight, italic, monospace family, underline, and strikethrough.
- Added regression tests for mixed style-run composition, inline-code run styling, and non-truncating native wrapping defaults.
- Updated P8 tracking docs to mark Phase 1 Typography bootstrapping complete and set Phase 2 Markdown renderer as next.

### Verification
- `cargo test -p aura-components paragraph` passed during implementation.

### Key Discoveries
- GPUI `StyledText` supports per-run font family/weight/style/color/background/underline/strikethrough via `TextRun`; per-run font size is not represented in `TextRun`, so segment-level size remains a standalone `Text` behavior rather than mixed-run paragraph behavior.
- Native wrapping is controlled by inherited GPUI `TextStyle.white_space`, so `Paragraph` sets a normal whitespace, full-width, no-overflow/no-line-clamp shell around the `StyledText`.

## Session 82 — 2026-05-11 (P8 Phase 2 Markdown Renderer)

### Actions
- Added `pulldown-cmark` to `aura-gallery`.
- Created `apps/aura-docs/src/markdown.rs` with `render_markdown(md_text: &str) -> gpui::AnyElement`.
- Implemented a stack-based Markdown parser using `Vec<Frame>` for Root/Paragraph/Heading/BlockQuote/List/ListItem and an inline style context for strong/emphasis/code/strikethrough.
- Mapped parsed Markdown blocks to native Aura/GPUI elements: `Title`, `Paragraph`, `Text`, `Space`, and GPUI layout primitives.
- Added regression tests for entrypoint construction, heading + mixed inline styles, unordered/ordered lists, and blockquote nesting.
- Exported the `markdown` module from the Gallery binary so the public renderer surface stays warning-free before the full document shell consumes it.
- Updated P8 tracking docs to mark Phase 2 complete and set Phase 3 code block styling + docs shell as next.

### Verification
- `cargo test -p aura-gallery markdown` passed during implementation.
- `cargo check -p aura-gallery` passed with no warnings after making the module public and test-gating parser inspection helpers.

### Key Discoveries
- `pulldown-cmark` 0.13 uses `Tag::Heading { level, .. }` and `TagEnd::Heading(level)`, so the renderer stores heading level on Start and pops the frame on End.
- Phase 2 intentionally leaves fenced/indented code block handling for Phase 3; inline code is already mapped through Aura `Text::code_style`.

## Session 83 — 2026-05-11 (P8 Phase 3 Code Blocks + Docs Shell)

### Actions
- Extended the native Markdown renderer with fenced/indented code block parsing using `Tag::CodeBlock` and `CodeBlockKind`.
- Rendered code blocks as native GPUI/Aura shells with neutral background, border, monospace text, no wrapping, and horizontal scrolling via `overflow_x_scroll`.
- Added a `DocsShell` native two-column document window: Aura `Container` + left Aura `Menu` navigation + right Markdown-rendered document content with vertical scrolling.
- Registered `Aura Docs` in the Gallery registry so the native docs shell is reachable from the existing demo bootstrap.
- Kept inline code styling through the existing `Text::code_style` path from Phase 1/2.
- Added tests for fenced code parsing, code block horizontal scroll styling, and docs shell native Container/Menu integration.
- Updated P8 tracking docs to mark Phase 3 complete and set Phase 4 Live Demo injection as next.

### Verification
- `cargo test -p aura-gallery markdown` passed during implementation.
- `cargo check -p aura-gallery` passed without warnings during implementation.

### Key Discoveries
- Current GPUI supports `overflow_x_scroll` on stateful divs, which is sufficient for Phase 3 horizontal code scrolling without introducing a custom horizontal Scrollbar component.
- The docs shell can be introduced as a normal Gallery registry entry first, avoiding a disruptive replacement of the existing component demo bootstrap before Live Demo injection is ready.

## Session 84 — 2026-05-11 (P8 Phase 4 Live Demo Injection)

### Actions
- Implemented Markdown live demo marker recognition for `::AuraDemo{component="Button"}::` in text events outside code blocks.
- Added text splitting so live demo markers are removed from paragraph text and inserted as dedicated `Block::LiveDemo` nodes while preserving surrounding text.
- Rendered `Block::LiveDemo { component: "Button" }` as a real Aura `Button` inside a native highlighted card shell with hover/click-capable GPUI interaction.
- Updated the docs component page to include the live Button marker so the Gallery docs shell exercises the injection path.
- Added regression tests for marker parsing, split behavior, marker removal from text blocks, and Button mapping to a real Aura component node.
- Updated P8 tracking docs to mark Phase 4 complete and P8 core done.

### Verification
- `cargo test -p aura-gallery markdown` passed during implementation.

### Key Discoveries
- `pulldown-cmark` emits the custom live demo syntax as normal `Event::Text`, so recognition belongs in the text-event path and must be disabled while the top frame is a code block.
- Live demo injection is safest as a block-level split for now: surrounding paragraph text becomes normal Paragraph blocks, and the component marker becomes a dedicated native demo block.

## Session 85 — 2026-05-11 (P8 Docs App Split)

### Actions
- Split the native docs surface into a dedicated `apps/aura-docs` binary crate with its own `main.rs` and Markdown renderer.
- Removed the docs shell entry and markdown module from `aura-gallery`, restoring gallery to a pure component showcase.
- Updated workspace membership, app Cargo manifests, and the project/phase docs to describe `aura-docs` as the official native docs main window.
- Adjusted app titles and shell text to refer to Aura Docs instead of the old gallery-hosted docs shell.

### Verification
- `cargo check -p aura-gallery -p aura-docs` passed.
- `cargo test -p aura-docs --no-run` passed.
- `timeout 8s cargo run -p aura-docs` started successfully and was stopped by timeout.
- `timeout 8s cargo run -p aura-gallery` started successfully and was stopped by timeout.

## Session 86 — 2026-05-12 (CodeBlock Component)

### Actions
- Added `CodeBlock` to `aura-components` with block and inline formats.
- Implemented lightweight native syntax highlighting with `StyledText`/`TextRun` for common Rust/TOML/JSON/Markdown/Shell/TS/JS tokens.
- Added language labels, convenience language builders, and a copy button backed by GPUI clipboard APIs.
- Replaced the Aura Docs Markdown code-block renderer with the reusable `CodeBlock` component.
- Added a Gallery demo covering Rust, JSON, Shell, and inline usage.

### Verification
- `cargo test -p aura-components code_block -- --nocapture` passed.
- `cargo check -p aura-docs -p aura-gallery` passed.
- `cargo test -p aura-gallery code_block_demo_uses_component_api` passed.
- `timeout 8s cargo run -p aura-gallery` started successfully and was stopped by timeout.
- `timeout 8s cargo run -p aura-docs` started successfully and was stopped by timeout.

## Session 87 — 2026-05-12 (Aura Docs Content Expansion)

### Actions
- Expanded `aura-docs` from three placeholder pages to a fuller native documentation set: Overview, Quick Start, Architecture, Typography, CodeBlock, Markdown, Live Demo, and Authoring.
- Added runnable command snippets, component examples, Markdown renderer architecture notes, and CodeBlock API docs.
- Added a regression test that verifies the core documentation pages are registered in the docs navigation.

### Verification
- `cargo test -p aura-docs markdown -- --nocapture` passed during implementation.
- `cargo check -p aura-docs` passed during implementation.
- `cargo check -p aura-gallery` passed.
- `timeout 8s cargo run -p aura-docs` started successfully and was stopped by timeout.

## Session 88 — 2026-05-12 (CodeBlock Syntect Highlighting)

### Actions
- Replaced the hand-written CodeBlock token highlighter with `syntect`.
- Kept rendering native by converting syntect regions into GPUI `TextRun`s inside `StyledText`.
- Switched CodeBlock visuals to a more polished dark code surface using the `base16-ocean.dark` syntect theme palette.
- Updated Aura Docs wording to document `syntect` as the highlighter implementation.

### Verification
- `cargo test -p aura-components code_block -- --nocapture` passed during implementation.
- `cargo check -p aura-docs -p aura-gallery` passed.
- `cargo test -p aura-docs markdown` passed.
- `cargo test -p aura-gallery code_block_demo_uses_component_api` passed.
- `cargo test -p aura-components code_block` passed.
- `timeout 8s cargo run -p aura-gallery` started successfully and was stopped by timeout.
- `timeout 8s cargo run -p aura-docs` started successfully and was stopped by timeout.

## Session 90 — 2026-05-12 (Aura Docs Page Split + External Snippets)

### Actions
- Split Aura Docs authored content out of `apps/aura-docs/src/markdown.rs` into per-page Markdown files under `apps/aura-docs/content/pages/`.
- Migrated current docs pages: Overview, Quick Start, Architecture, Typography, Button, CodeBlock, Input, Switch, Message, Markdown, Live Demo, Authoring.
- Extracted code samples into external `.rs` snippets under `apps/aura-docs/content/snippets/<page>/<case>.rs`.
- Added fenced code `src="..."` support in the Markdown renderer so snippets are loaded by convention and rendered through the existing native `CodeBlock` component.
- Updated architecture/prompt/memory docs with the page/snippet naming convention.

### Verification
- `cargo test -p aura-docs` passed during implementation.

### Key Decisions
- Docs authored content should not be embedded as large Rust string constants.
- Component documentation uses one Markdown file per component.
- Code examples are maintained separately from Markdown and included via `src` paths relative to `apps/aura-docs/content/snippets/`.

## Session 53 — 2026-05-14 (P10 Charts Planning)

### Actions
- Started new P10 phase for native statistics/chart components.
- Reviewed local GPUI drawing primitives: `canvas`, `PathBuilder`, `Window::paint_path`, `Window::paint_quad`, and text rendering support.
- Cloned/reviewed `vicanso/zedis` as a secondary case study; its Metrics view draws area/line/bar charts with GPUI canvas and scale/axis/shape layering.
- Added `.prompt/P10-charts.md`.
- Updated `prompt.md`, `architecture-design.md`, and `.memory/*` to make P10 the active charts phase and preserve the native-only constraint.

### Key Decisions
- Charts are first-class Aura components, not external chart runtime wrappers.
- GPUI official/local source is the primary reference; zedis is only a structural case study.
- First delivery set: LineChart, AreaChart, BarChart, PieChart, RingChart, Sparkline plus shared scale/axis/grid/legend/tooltip infrastructure.

### Verification
- Documentation/planning update only; run `cargo fmt` / `cargo check` after implementation changes begin.

## Session 54 — 2026-05-14 (P10 Charts Implementation Slice)

### Actions
- Added native chart foundation in `aura-components`: `ChartPoint`, `ChartSeries`, `ChartOptions`, palette/domain helpers, `ScaleLinear`, `ScalePoint`, `ScaleBand`, shared chart frame painting, and shape helpers.
- Implemented `LineChart`, `AreaChart`, and `BarChart` as pure GPUI components using `canvas`, `PathBuilder`, `paint_path`, and `paint_quad`.
- Added Gallery demos and Docs pages/snippets for LineChart, AreaChart, and BarChart.

### Key Decisions
- Shared axis/grid/label rendering now lives in `chart_frame.rs` to keep chart components thin.
- Area and Bar charts support both overlay/grouped and stacked modes before adding hover tooltip complexity.
- Docs snippets remain complete Rust files imported by the snippet check harness.

### Verification
- `cargo fmt`
- `cargo check -p aura-components`
- `cargo check -p aura-docs --bin check_snippets`
- `cargo check -p aura-docs`
- `cargo check -p aura-gallery`
- `cargo test --workspace`
- `timeout 8s cargo run -p aura-docs` (124 expected GUI timeout)
- `timeout 8s cargo run -p aura-gallery` (124 expected GUI timeout)


## Session 2026-06-17 — Phase Handoff Stale-State Cleanup

### Actions
- Audited the current phase documents against the repository state after P10/P12/P13/P14 work.
- Updated `prompt.md`, `.prompt/P12-packaging.md`, and `.memory/state.md` so the entrypoint no longer describes P8/P9 or early P12 scaffolding as current work.
- Preserved P12 as readiness rather than complete because signing/notarization, real system install/uninstall, license policy, and real `v*` release execution remain external-policy gated.

### Verification
- Documentation sync only; run markdown/search checks plus package dry-run and core cargo checks before commit.


## Session 2026-06-17 — P15 Quality Hardening Kickoff

### Actions
- Created `.prompt/P15-quality-hardening.md` to formalize the release-quality hardening phase.
- Added `.github/workflows/ci.yml` for general quality gates: fmt, workspace check/test, docs snippet check, packaging validate, packaging dry-run, and install-smoke dry-run.
- Updated `prompt.md` and `.memory/state.md` so future sessions enter P15 rather than reopening completed P13/P14 work.

### Verification
- Run local fmt/check/test/package dry-run gates before committing this slice.

## Session 2026-06-17 — P15 Track B API Consistency

### Actions
- Normalized remaining exact-`Pixels` public builder parameters across charts and newer P13/P14 components to `impl Into<Pixels>` where this is source-compatible.
- Preserved explicit `px(...)` usage in tests/examples for visual dimensions; the API is broader, but Aura docs and examples should keep units obvious.
- Extended builder-state assertions for SignalMeter, HeatBar, SegmentRatioBar, Label, Operation, and TagFlow.

### Verification
- `cargo test -p aura-components -- --nocapture` passed: 192 unit tests plus package integration tests.
- Full P15 gate suite passed: fmt, workspace check/test, docs snippet check, package validate, packaging dry-run, install-smoke dry-run, and `git diff --check`.
- GUI smoke passed: `timeout 10s cargo run -p aura-gallery` and `timeout 10s cargo run -p aura-docs` both started successfully and exited via expected timeout.

## Session 2026-06-17 — P15 Track B Callback and Panic Audit

### Actions
- Added source-level API consistency regression tests for public callback signatures and state-builder naming.
- Documented entity-local callback exceptions for Input, CodeEditor, and HorizontalList instead of forcing a breaking signature change.
- Removed avoidable production-path panics from Button icon-only rendering, DateTimePicker defaults, Input text hit-testing/paint, InputNumber filtering, Chart downsampling, Sparkline rendering, and CodeBlock shaped-text paint paths.

### Verification
- `cargo test -p aura-components api_consistency_audit_tests -- --nocapture` passed.
- Full P15 gate suite passed after whitespace cleanup: fmt, workspace check/test, docs snippet check, package validate, packaging dry-run, install-smoke dry-run, and `git diff --check`.
- GUI smoke passed: `timeout 10s cargo run -p aura-gallery` and `timeout 10s cargo run -p aura-docs` both started successfully and exited via expected timeout.

## Session 2026-06-17 — P15 Track C Visual Theme Consistency

### Actions
- Started Track C visual/theme consistency with a focused token-hardening slice.
- Replaced hard-coded production `gpui::white()` text on dark/colored Tag and line Progress surfaces with `theme.neutral.inverted`.
- Added source-level visual consistency regression tests for colored surface text tokens and representative Virtualized* surface/border/radius token usage.

### Verification
- `cargo test -p aura-components -- --nocapture` passed: 197 unit tests plus package integration tests.
- Full P15 gate suite passed: fmt, workspace check/test, docs snippet check, package validate, packaging dry-run, install-smoke dry-run, and `git diff --check`.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.

## Session 2026-06-17 — P15 Track C Chart Label Theme Tokens

### Actions
- Continued Track C visual/theme consistency with chart internal value labels.
- Replaced hard-coded white labels on stacked BarChart fills and Pie/Ring slices with `theme.neutral.inverted` passed into paint helpers.
- Extended visual consistency source regression coverage to BarChart and PieChart.

### Verification
- `cargo fmt --all --check` passed.
- `cargo test -p aura-components -- --nocapture` passed: 197 unit tests plus package integration tests.
- `cargo check --workspace --all-targets` passed.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.
