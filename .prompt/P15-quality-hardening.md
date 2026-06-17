# P15 — Quality Hardening

> 上游: `.prompt/P13-component-expansion.md` / `.prompt/P14-deferred-advanced.md` / `.prompt/P12-packaging.md`  
> 状态: Active  
> 目标: 在组件功能补齐后，进入发布前质量收口阶段，系统性提升 Aura 的可维护性、一致性、性能、文档完整性和 CI 防回归能力。

## 背景

P13 高级控件扩展和 P14 deferred backlog 已完成，P12 本地 runner-safe packaging readiness 已完成但仍受签名、公证、真实系统安装、license policy、真实 release tag 等外部策略约束。P15 不继续盲目堆新控件，而是把现有组件库从“功能可用”推进到“长期可维护、可发布、可被外部项目放心依赖”。

## 非目标 / 边界

- 不引入 WebView、HTML/CSS/DOM/browser runtime；Aura 继续保持纯 Rust + GPUI native。
- 不把 P12 外部策略项伪装成本地完成项；签名、公证、真实系统安装卸载和 license 仍需 owner policy。
- 不新增平行替代控件来掩盖已有控件问题；优先修原组件、原 demo、原 docs。
- 不用减少 demo 数据量替代真实性能优化；性能项必须基于 profiling 或可复现的基准/测试证据。

## 工作流

每个 hardening slice 都必须包含：

1. 明确审计目标和验收证据。
2. 小范围、可回滚的代码/文档/CI 改动。
3. 运行对应验证命令。
4. 更新 `.memory/state.md` / `.memory/sessions.md` / 本文件进度。
5. commit + push。

## Track A — CI / Verification Gates

目标：让每次普通提交都能自动验证核心质量，而不是只依赖 packaging workflow。

- [x] 新增通用 CI workflow：fmt、workspace check/test、docs snippet check、packaging validate、packaging dry-run、install-smoke dry-run。
- [ ] 评估是否需要拆分 Linux GUI 依赖缓存和 workspace test matrix。
- [ ] 将 release/package workflow 与 CI workflow 的职责边界写入 docs。

## Track B — API Consistency Audit

目标：统一组件 builder、事件回调和状态命名。

审计重点：

- `on_change` / `on_select` / `on_click` / `on_close` / `on_finish` 签名一致性。
- `disabled(...)`、`size(...)`、`variant(...)`、`open(...)`、`default_*` builder 命名一致性。
- P13/P14 新增控件是否遵守已有 Aura 组件 API 范式。
- 避免生产路径中不必要的 `unwrap()` / `expect()` / `panic!()`。

## Track C — Visual / Theme Consistency

目标：新增控件和历史控件在 light/dark/theme token 下保持一致。

审计重点：

- spacing / radius / border / shadow token 使用。
- disabled / hover / active / selected 状态。
- Button、Tag、Radio、Checkbox、Chart、Progress、Tour、Virtualized* 等 P13/P14 控件视觉一致性。

## Track D — Interaction / Keyboard / Overlay Behavior

目标：减少交互类控件的边界问题。

审计重点：

- ESC 关闭能力：Modal、Drawer、Popover、Dropdown、Select、Tour 等。
- Tab / Enter / Space 基础键盘操作。
- 点击外部关闭、焦点释放、选区取消、拖拽释放等状态清理。
- overlay 层级与 Portal/Modal/Drawer/Tooltip 的 z-index 关系。

## Track E — Performance Hardening

目标：用证据驱动优化，而不是降低示例规模。

审计重点：

- CodeBlock / CodeEditor 高亮与选区性能。
- Line/Area/Sparkline 大数据降采样后的剩余热点。
- VirtualizedList / VirtualizedTable / VirtualizedTree 滚动和拖拽。
- Docs QuickStart 等长页面渲染/滚动性能。

## Track F — Docs Completeness

目标：每个可公开组件都具备足够文档和可编译代码片段。

审计重点：

- 每个组件是否有页面、效果、对应 snippet。
- snippets 是否是完整 Rust 文件并由 `check_snippets` 覆盖。
- Gallery 与 Docs 示例是否同步。
- P12 打包流程、P15 质量门禁是否在 docs 中有清晰入口。

## P15 Progress

### 2026-06-17 — Track A initial quality gates

- Added `.github/workflows/ci.yml` for general quality gates independent from packaging release workflow.
- Gates cover Linux dependency install, `cargo fmt --all --check`, workspace check/test, docs snippet check, `xtask package validate`, packaging dry-run, and install-smoke dry-run.
### 2026-06-17 — Track B API consistency slice

- Broadened remaining exact-`Pixels` public builder parameters to `impl Into<Pixels>` for chart dimensions/strokes, P13 visual components, `TagFlow`, `Operation`, and `HorizontalList` height. This is source-compatible for existing `px(...)` calls and aligns these APIs with newer controls such as Input, Select, Progress, QR Code, virtualized controls, and form controls.
- Kept call sites/tests using explicit `px(...)` where they document visual dimensions so unit intent remains clear.
- Added/extended builder-state assertions for SignalMeter, HeatBar, SegmentRatioBar, Label, Operation, and TagFlow dimension/gap options.

Validation evidence for this slice:
- `cargo test -p aura-components -- --nocapture` passed: 192 unit tests plus integration tests.
- Full P15 gate suite passed: fmt, workspace check/test, docs snippet check, package validate, packaging dry-run, install-smoke dry-run, and `git diff --check`.
- GUI smoke passed: `timeout 10s cargo run -p aura-gallery` and `timeout 10s cargo run -p aura-docs` both started successfully and exited via expected timeout.

### 2026-06-17 — Track B callback/state and panic audit

- Added API consistency audit tests that lock the public callback convention: value callbacks use `(value, &mut Window, &mut App)`, while entity-local controls such as `Input`, `CodeEditor`, and `HorizontalList` explicitly use `Context<...>` callbacks.
- Added state-builder audit coverage for `disabled(...)` and `close_on_escape(...)` naming across representative controls.
- Removed avoidable production-path `unwrap()` / `expect()` / paint-result panics from hardened paths: Button icon-only rendering, DateTimePicker defaults, Input masked/word selection and paint, InputNumber filtering, Chart downsampling, Sparkline empty-data handling, and CodeBlock paint paths.

Validation evidence for this slice:
- `cargo test -p aura-components api_consistency_audit_tests -- --nocapture` passed.
- `cargo test -p aura-components -- --nocapture` passed: 195 unit tests plus package integration tests.
- Full P15 gate suite passed after whitespace cleanup: fmt, workspace check/test, docs snippet check, package validate, packaging dry-run, install-smoke dry-run, and `git diff --check`.
- GUI smoke passed: `timeout 10s cargo run -p aura-gallery` and `timeout 10s cargo run -p aura-docs` both started successfully and exited via expected timeout.

### 2026-06-17 — Track C visual/theme consistency slice

- Replaced hard-coded production `gpui::white()` text on colored/dark Tag and line Progress surfaces with `theme.neutral.inverted`, preserving contrast intent while honoring light/dark theme tokens.
- Added visual/theme audit tests for hardened colored surfaces and representative Virtualized* surface/border/radius token usage.

Validation evidence for this slice:
- `cargo test -p aura-components -- --nocapture` passed: 197 unit tests plus package integration tests.
- Full P15 gate suite passed: fmt, workspace check/test, docs snippet check, package validate, packaging dry-run, install-smoke dry-run, and `git diff --check`.
- GUI smoke passed: `timeout 10s cargo run -p aura-gallery` and `timeout 10s cargo run -p aura-docs` both started successfully and exited via expected timeout.

### 2026-06-17 — Track C chart label theme-token slice

- Replaced hard-coded production `gpui::white()` value labels inside stacked BarChart fills and Pie/Ring slices with `theme.neutral.inverted` passed through render helpers.
- Extended visual/theme audit coverage so chart value labels stay covered alongside Tag and Progress colored surfaces.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo test -p aura-components -- --nocapture` passed: 197 unit tests plus package integration tests.
- `cargo check --workspace --all-targets` passed.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.
