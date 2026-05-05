# Session History

## Session 5 — 2026-05-04 (Late Night)

### Actions
- **Input 组件深度增强**:
  - 实现光标闪烁逻辑 (500ms 周期，仅在 Focus 时激活)
  - 实现全选功能 (`SelectAll`, Cmd/Ctrl+A)
  - 实现剪贴板集成 (Copy/Paste/Cut, Cmd/Ctrl+C/V/X)
  - 实现鼠标选择逻辑 (双击选单词、三击全选、拖拽选择、Shift+点击选择)
  - 实现键盘选择逻辑 (Shift+方向键/Home/End)
  - 修正多行模式下的鼠标索引计算 (通过存储 `last_line_layouts`)
  - 修正多行模式下的选择区域渲染 (支持跨行选择背景)
- **表单组件通用增强**:
  - `Checkbox`, `Radio`, `Switch` 支持键盘操作 (Space/Enter 触发)
  - `Checkbox`, `Radio`, `Switch` 增加 Focus 视觉反馈 (Ring/Border 变色)
  - `RadioGroup` 支持方向键切换选项
  - 统一注册各组件键盘绑定到 Gallery
- **清理与优化**:
  - 修复 `checkbox` 与 `switch` 之间 `Toggle` 动作冲突 (重命名为专用名称)
  - 修复 `input` 与 `radio_group` 之间 `Up`/`Down` 动作冲突
  - 移除各文件中的冗余 import (`px`, `MouseUpEvent` 等)
  - 统一 `cx.spawn` 使用 `async move |this, mut cx|` 模式
- **状态同步**:
  - 更新 `.memory/state.md` 记录 P2 进度

### Key Discoveries
- GPUI 0.2.x 中 `cx.spawn` 在 `Context<T>` 上使用时，闭包签名需匹配 `async move |this, mut cx|` 以正确推导 `AsyncApp` 并处理生命周期
- 多行文本交互需要持久化所有行的 `ShapedLine` 布局信息，否则 `index_for_x` 无法跨行工作
- `this.update(cx, ...)` 在 `AsyncApp` 环境下直接传递 `cx` 即可 (无需 `&mut cx` 若 `cx` 已经是引用)

### Decisions Made
- 录入 P2 首个核心组件 `Input` 的完整交互逻辑，作为后续复杂组件的参考

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

## Session 2 — 2026-05-03

### Actions
- 调查 AuraButton hover/pressed 交互不生效问题。
- 移除 `AuraButton` 的双路径实现中过度复杂的 `Render` + 手写 `is_pressed` 状态路径，保留 `.build(&theme)` 的 theme-explicit builder 模式。
- 为 Button 添加可选 `.id(...)` builder；默认使用调用位置 + label 生成稳定 GPUI element id，供 `.hover()` / `.active()` 状态追踪使用。
- 修正按钮主题色：filled variant 的 hover 比 base 稍暗，active 比 hover 更深；Default 透明按钮现在有可见 hover/active overlay。
- 添加 aura-theme 单元测试锁定 hover/active 背景层级。

### Key Discoveries
- Gallery Demo 实际调用 `AuraButton::build(theme)`，之前 `impl Render for AuraButton` 的手写 pressed 状态是死路径。
- GPUI 0.2.2 的 `.active()` 是 `StatefulInteractiveElement` 样式能力，需要 `.id(...)` 后使用；不需要组件自己维护 `is_pressed`。
- 之前每次 build/render 使用全局递增 id 会让交互状态难以稳定追踪；稳定 id 更符合 GPUI 的 element state 模型。
- Primary hover 原本使用 theme `family.hover`，在当前 NaiveUI token 下比 base 更亮，不满足“hover 暗一点”的需求；Default 原本 hover/active 背景全透明。

### Verification
- `cargo test -p aura-theme` passed: 2 tests.
- `cargo check` passed; existing aura-gallery dead_code warnings remain.
- `timeout 5s cargo run -p aura-gallery` compiled successfully, then failed to open a window in tmux due to GPUI Linux `NoCompositor` (environment/display issue, not compile issue).

### Follow-up — Button id policy
- 明确 Button 不能依赖业务开发者手写 `.id(...)` 才获得基础 hover/active 交互。
- `AuraButton::new(...)` 现在通过 `#[track_caller]` 捕获组件创建位置，默认 id 由创建位置 + label + variant/size/状态参数生成。
- `.id(...)` 保留为高级覆盖项，用于同一调用点批量渲染同 label/variant 按钮等潜在冲突场景。
- Added aura-components tests for automatic id generation and explicit id override.

### Follow-up — Global theme API
- Replaced public `AuraButton::build(&theme)` usage with GPUI `IntoElement + RenderOnce`; Button now reads `Config.theme` from `App` during render.
- Gallery demo registry no longer passes theme through function pointers; button demo wraps content in a `RenderOnce` demo component and reads global theme internally.
- Updated prompt.md, P1 prompt, and decisions to supersede explicit `.build(&theme)` policy.

### Follow-up — Button icon hover color
- Re-read the icon library after adding `aura-icons-lucide` and Button icon support.
- Root cause: `AuraIcon` intentionally inherits parent `text_color` when no explicit color is set, but `AuraButton` passed `.color(c.text)` to every internal icon, fixing the SVG text color at the normal state.
- Fix: Button-created icons no longer set explicit icon color; they inherit the button container text color, including hover `text_color(c.text_hover)` and disabled text color.
- Verified `cargo check`, `cargo test -p aura-theme`, and `cargo test -p aura-components`.

### Correction — SVG color inheritance in GPUI
- Previous assumption was wrong: GPUI `Svg::paint` requires the SVG element's own final `style.text.color`; it does not render from an ancestor div's text color automatically.
- Removing explicit icon color caused Button icons to disappear because `style.text.color` on `svg()` was `None`.
- Correct fix: keep normal icon `.color(c.text)` and add `AuraIcon::group_hover_color(group, c.text_hover)`; Button assigns a hover group to the button container so child icons switch color via GPUI `group_hover`.
- Verified `cargo check`, `cargo test -p aura-icons`, and `cargo test -p aura-components`.

## Session 8 — 2026-05-05 (Evening)

### Actions
- **修复 Popover/Popconfirm 交互与定位问题**:
  - **恢复 `on_click`**: 在 `Popover::new` 和 `render` 中使用 `#[track_caller]` 捕获调用点，生成稳定的 `ElementId`。这使得 `div` 包装器可以安全使用 `.on_click()`，修复了鼠标从外部按下移入触发的问题。
  - **实现“安全轴心点”边界补偿 (Safe Pivot Clamping)**:
    - 在 `PopoverView` 中引入边界溢出检测。根据参考尺寸 (200x150) 计算气泡中心点的安全区间。
    - 当按钮靠近窗口边缘时，自动偏移 2000px 轴心容器的位置，确保气泡内容始终留在视口内 [0, viewport_size]。
    - 优化了 `Top` 分支的 `bottom` 坐标计算，彻底解决了 `TopCenter` 在某些临界坐标下不弹出的 bug。
    - 针对 `Start` / `End` 对齐变体，同样应用了中心点偏移补偿，确保边缘对齐且不溢出。
  - **验证**:
  - `cargo check` 通过。
  Applied fuzzy match at line 125-130.

## Session 9 — 2026-05-05 (Night)

### Actions
- **验证与提交修复**:
  - 确认 `Popover` 的边界补偿逻辑 (Safe Pivot Clamping) 已正确实现。
  - 确认 `#[track_caller]` 已用于生成稳定 `ElementId` 以支持 `on_click`。
  - 运行 `cargo check` 验证全量编译。
  - 分块提交 27 个变更文件，包括 `aura-core` 重构、`popover.rs` 修复、`tooltip.rs` 同步更新以及组件适配。
- **清理与验证**:
  - 运行 `aura-gallery` 确认无运行时崩溃。
  - 确认 `state.md` 与项目实际进度一致 (P3 完美收工)。

### Key Discoveries
- 将 `ActivePopover` 和 `ActiveTooltip` 提升至 `aura-core` 全局状态极大简化了 Portal 渲染逻辑，并解决了多层叠加时的层级冲突。
- 使用 `pivot_container` (2000px) 配合 `justify_center` / `items_center` 是在 GPUI 中实现完美居中定位且支持边界补偿的最灵活方案。

## Session 10 — 2026-05-06

### Actions
- **修复 Popover / Popconfirm 气泡水平中心对齐问题**:
  - 根因: `PopoverView` 使用 2000px `pivot_container` + flex 居中近似定位，并用固定 `400x300` reference size 估算边界；实际内容宽度变化时，气泡中心会偏离触发元素中心。
  - 改为使用 GPUI 原生 `anchored()` 元素，以触发元素的真实中心/边缘锚点定位气泡。
  - 对 `Top` / `Bottom` 使用 `TopCenter` / `BottomCenter` 锚点保证水平中心对齐；对 `Start` / `End` 和左右方位映射到相应角/中心锚点。
  - 使用 `snap_to_window_with_margin(px(4.0))` 保证气泡不超出窗体边界。
  - 给气泡内容设置 `max_w` 为视口减边距，避免超宽内容横向溢出；纵向位置交由 `anchored` 贴边处理。
- **补充 Popper 几何回归测试**:
  - 验证垂直居中放置时内容中心与 anchor 中心一致。
  - 验证靠近左右边界时水平位置被 clamp 到视口内。

### Verification
- `cargo check` passed.
- `cargo test -p aura-core` passed: 2 tests.
- `cargo test -p aura-components` passed.
- `timeout 10s cargo run -p aura-gallery` compiled and launched; command ended by timeout after run start.

### Key Discoveries
- GPUI 已提供 `anchored()` + `snap_to_window_with_margin()`，比手写大尺寸 pivot 容器更适合弹层定位；它会按子元素实际布局尺寸做窗口边界吸附。

### Follow-up — Popover trigger identity collision
- 修复 `Popover::new()` 的 `#[track_caller]` 调用点未被持久化的问题: 之前实际在 `RenderOnce::render()` 中读取 caller，多个 Popover/Popconfirm 会共享同一个渲染调用点生成的 `ElementId`，导致 TopCenter / Popconfirm 等相邻用例触发状态混淆，表现为点击后未如期弹出。
- `Popover` 现在在 `new()` 时生成并保存稳定 `trigger_id`，`render()` 直接使用该实例 ID。
- 新增 `Popover::id(...)` 作为高级覆盖入口。
- `Popconfirm` 通过 `.id(format!("popconfirm-trigger-{}", caller))` 显式转发自身调用点，避免嵌套 `Popover::new(self.trigger)` 统一落到 `popconfirm.rs` 内部同一行。

### Verification
- `cargo check` passed.
- `cargo test -p aura-core` passed: 2 tests.
- `cargo test -p aura-components` passed.
- `timeout 10s cargo run -p aura-gallery` compiled and launched; command ended by timeout after run start.

### Follow-up — Popconfirm instance identity persisted at construction
- 修复 Archive Popconfirm 仍不弹的问题: 上一次只让 `Popover` 在 `new()` 持久化 ID，但 `Popconfirm` 仍在 `render()` 中读取 caller；多个 Popconfirm 实例仍可能共享同一个 render 调用点。
- `Popconfirm` 现在在 `new()` 时保存自身 `trigger_id`，并在 render 时传给内部 `Popover::id(...)`。
- 新增 `Popconfirm::id(...)` 高级覆盖入口，用于同一调用点批量渲染时手动去重。

### Verification
- `cargo check` passed.
- `cargo test -p aura-core` passed: 2 tests.
- `cargo test -p aura-components` passed.
- `cargo run -p aura-gallery` compiled, then failed at runtime with Linux `NoCompositor` in this tmux environment.

## Session 11 — 2026-05-06

### Actions
- **修复点击气泡弹层自身会关闭的问题**:
  - 根因: Popover 弹层内容节点使用 `.on_mouse_down(MouseButton::Left, |_, _, _| {})` 空闭包，未调用 `cx.stop_propagation()`，事件仍冒泡到全屏遮罩层的 close handler。
  - 改为在弹层内容的 mouse down handler 中调用 `cx.stop_propagation()`，阻止内部点击触发外层 click-outside 关闭逻辑。
  - 该修复覆盖所有基于 Popover 的气泡弹层，包括 Popover、Popconfirm、Dropdown。

### Verification
- `cargo check` passed.
- `cargo test -p aura-core` passed: 2 tests.
- `cargo test -p aura-components` passed.
- `cargo run -p aura-gallery` compiled, then failed at runtime with Linux `NoCompositor` in this tmux environment.

## Session 12 — 2026-05-06

### Actions
- **检查 Dialog 手动修复并补齐关闭能力**:
  - 确认 Dialog 内容面板已使用 `cx.stop_propagation()` 阻止内部点击冒泡到遮罩层。
  - 新增 `Dialog::close(cx)` 可编程关闭 API，供内容按钮或业务逻辑手动关闭。
  - 新增 `.close_on_escape(bool)`，允许将 Dialog 配置为 ESC 不可关闭。
  - 新增 `Dialog::register_key_bindings(cx)` 注册 `escape -> DialogClose`。
- **修复 Drawer 同类问题**:
  - Drawer panel 原先只有空 mouse-down handler，不能阻止事件冒泡；改为 `cx.stop_propagation()`。
  - 新增 `Drawer::close(cx)` 可编程关闭 API。
  - 新增 `.close_on_escape(bool)` 与 `Drawer::register_key_bindings(cx)`。
- **修复 MessageBox 关闭语义**:
  - MessageBox 按钮原先调用 `clear_portals(cx)`，只清 Portal 临时渲染队列，不能可靠清除 ActiveModal 状态。
  - 改为调用 `Dialog::close(cx)` / `MessageBox::close(cx)`。
  - 新增 `.close_on_click_outside(bool)` / `.close_on_escape(bool)` 转发到底层 Dialog，支持必须手动关闭的场景。
- **Gallery 接入**:
  - 在 `aura-gallery` 启动时注册 Dialog / Drawer 的 ESC key binding。

### Verification
- `cargo check` passed.
- `cargo test -p aura-core` passed: 2 tests.
- `cargo test -p aura-components` passed.
- `cargo run -p aura-gallery` compiled, then failed at runtime with Linux `NoCompositor` in this tmux environment.

## Session 13 — 2026-05-06

### Actions
- **修复弹层 hover / mouse-move 穿透**:
  - Popover 全屏浮层和气泡内容均增加 `on_mouse_move(... cx.stop_propagation())`，避免 hover 事件继续传递到底层组件。
  - Dialog 遮罩层和内容面板均增加 mouse-move propagation stop。
  - Drawer 遮罩层和 panel 均增加 mouse-move propagation stop。
  - 该修复覆盖基于 Popover 的 Popover / Popconfirm / Dropdown，以及基于 Dialog 的 MessageBox。
- **扩充弹层组件 Demo 覆盖**:
  - Popover: 基础用法、十二方向 placement、禁用空白关闭、手动关闭、自定义 offset。
  - Popconfirm: 基础 Delete/Archive、自定义文案、多个 placement。
  - Dropdown: 基础 actions、BottomEnd/TopStart、Top/Bottom/Left/Right placement。
  - Dialog: 基础、手动关闭-only、复杂内容与内部按钮关闭。
  - Drawer: 四方向、宽/高尺寸、手动关闭-only。
  - MessageBox: alert/confirm、禁用空白与 ESC 的手动关闭场景。

### Verification
- `cargo check` passed.
- `cargo test -p aura-core` passed: 2 tests.
- `cargo test -p aura-components` passed.
- `timeout 10s cargo run -p aura-gallery` compiled and launched; command ended by timeout after run start.

### Follow-up — Demo popover/popconfirm id collisions
- 修复扩展 Demo 后 Popover / Popconfirm 仅第一个用例能弹出的问题。
- 根因: Demo helper (`simple_popover`, `card_popover`, `confirm_at`) 在同一函数调用点批量创建多个 Popover/Popconfirm；组件默认 track_caller ID 会相同，导致触发状态冲突。
- 为 Popover / Popconfirm Demo 中每个示例显式设置唯一 `.id(...)`。

### Verification
- `cargo check` passed.
- `cargo test -p aura-core` passed: 2 tests.
- `cargo test -p aura-components` passed.
- `timeout 10s cargo run -p aura-gallery` compiled and launched; command ended by timeout after run start.

## Session 14 — 2026-05-06

### Actions
- **修复 Message 全局提示样式**:
  - Message 不再手写 `base.opacity(0.1)` 作为背景色。
  - 新增 `message_style(theme, msg_type)`，直接复用 `Theme::color_by_variant(variant, secondary=true, background=true, border=true)`。
  - Info / Success / Warning / Error 分别映射到 ButtonVariant::Info / Success / Warning / Danger。
  - 图标颜色、文字颜色、边框颜色统一使用 secondary button variant 的 `text` / `border`。
  - 背景使用 secondary button variant 的 `bg`，即类型颜色计算出的浅色背景，并跟随当前主题。
- **补充回归测试**:
  - `message_styles_reuse_secondary_button_variant_colors` 锁定 Message 样式与 secondary Button variant 颜色一致。

### Verification
- `cargo test -p aura-components message_styles_reuse_secondary_button_variant_colors -- --nocapture` passed.
- `cargo check` passed.
- `cargo test -p aura-core` passed: 2 tests.
- `cargo test -p aura-components` passed: 1 test.
- `timeout 10s cargo run -p aura-gallery` compiled and launched; command ended by timeout after run start.

### Follow-up — Message solid type color style
- 根据反馈调整 Message 样式: 不再复用 secondary Button 的浅色背景。
- Message 现在按 type 使用实色背景: Info/Success/Warning/Error 分别使用 theme.info/success/warning/danger.base。
- 图标和文字使用 `theme.neutral.card` 作为反色/白色前景，边框与背景同色。
- 更新测试为 `message_styles_use_solid_type_background_and_inverted_foreground`。

### Verification
- `cargo test -p aura-components message_styles_use_solid_type_background_and_inverted_foreground -- --nocapture` passed.
- `cargo check` passed.
- `cargo test -p aura-core` passed: 2 tests.
- `cargo test -p aura-components` passed: 1 test.
- `timeout 10s cargo run -p aura-gallery` compiled and launched; command ended by timeout after run start.
