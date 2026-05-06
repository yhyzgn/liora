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
- **Gallery Demo 增强**:
  - 新增以上所有 14 个组件的展示用例。
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
