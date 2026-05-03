# Session History

## Session 3 — 2026-05-03

### Actions
- 实现完整图标系统（见 icon-design.md）
- aura-icons: AuraIcon 容器，RenderOnce + IntoElement (Component)
- aura-icons-lucide: build.rs 代码生成，1,703 Lucide 图标
- scripts/sync-lucide.sh: 自动克隆 Lucide 仓库同步 SVG
- AuraIcon 主题色自动从 cx.global::<AuraConfig>() 读取
- 修复 SVG 渲染：text_color 必须挂 svg() 元素上（不能靠父 div 继承）
- 修复 GPUI SVG 加载：external_path 走 SvgAsset::load (fs::read 直接读文件系统)
- 确立 codex 范式：组件 RenderOnce + IntoElement，禁止 .build(theme)
- 更新 architecture-design.md/prompt.md/.memory/.prompt 文档

### Key Discoveries
- GPUI SvgAsset::load() 直接 fs::read()，不走 AssetSource 抽象层
- external_path 渲染需要 style.text.color 在 SVG 元素自身上
- Lucide 将 home.svg 重命名为 house.svg

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
- 实现 aura-theme: AuraTheme, Design Tokens, light/dark 模式
- 实现 aura-core: AuraConfig (Global), init_aura(), AuraContextExt, Z-Index utils
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
- Replaced public `AuraButton::build(&theme)` usage with GPUI `IntoElement + RenderOnce`; Button now reads `AuraConfig.theme` from `App` during render.
- Gallery demo registry no longer passes theme through function pointers; button demo wraps content in a `RenderOnce` demo component and reads global theme internally.
- Updated prompt.md, P1 prompt, and decisions to supersede explicit `.build(&theme)` policy.
