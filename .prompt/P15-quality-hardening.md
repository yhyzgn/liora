# P15 — Quality Hardening

> 上游: `.prompt/P13-component-expansion.md` / `.prompt/P14-deferred-advanced.md` / `.prompt/P12-packaging.md`
> 状态: Complete
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
- [x] 评估并拆分 Linux GUI workspace 质量 job 与 lightweight packaging dry-run job。
- [x] 将 release/package workflow 与 CI workflow 的职责边界写入 docs。

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

### 2026-06-17 — Track C themed control text slice

- Replaced hard-coded production white text for Button gradient text, Badge text, and Pagination active-background text with `theme.neutral.inverted`.
- Kept remaining `gpui::white()` occurrences where they are non-text color math, marker/border overlays, tests, or caller-provided example values.
- Extended visual/theme audit tests for Badge/Pagination colored surfaces and Button gradient text.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo test -p aura-components -- --nocapture` passed: 198 unit tests plus package integration tests.
- `cargo check --workspace --all-targets` passed.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.

### 2026-06-18 — Track D popover wrapper outside-close policy

- Added `close_on_click_outside(...)` to Dropdown and Popconfirm so Popover wrappers expose the same outside-click close policy as their underlying overlay shell.
- Forwarded the policy to `Popover::close_on_click_outside(...)` while preserving default close-on-outside behavior.
- Added source-level coverage for wrapper defaults, public builders, and forwarding.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo test -p aura-components -- --nocapture` passed: 199 unit tests plus package integration tests.
- `cargo check --workspace --all-targets` passed.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.

### 2026-06-18 — Track D outside-close docs/examples slice

- Added Dropdown close-strategy docs, live demo, compile-checked snippet, and Gallery example for `close_on_click_outside(false)` / `close_on_escape(false)`.
- Updated Popconfirm custom close-policy examples in Docs, snippets, and Gallery to show both ESC and outside-click close configuration.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo check -p aura-docs --bin check_snippets` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.

### 2026-06-18 — Track D input popup outside-close policy

- Added `close_on_click_outside(...)` to Select and Autocomplete so common input popups can opt out of automatic outside-click close without losing ESC policy control.
- Kept defaults unchanged (`true`) and bound outside-click handlers conditionally.
- Added source-level regression coverage for input popup outside-close defaults, builders, and conditional bindings.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo test -p aura-components overlay_escape_coverage_tests -- --nocapture` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test -p aura-components -- --nocapture` passed: 200 unit tests plus package integration tests.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.

### 2026-06-18 — Track D picker outside-close policy

- Added `close_on_click_outside(...)` to Cascader, DatePicker, DateTimePicker, TimePicker, and ColorPicker.
- Preserved default outside-click close behavior while making portal backdrop close handlers conditional on the new policy flag.
- Extended source-level popup policy coverage across input popups and picker popups.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo test -p aura-components overlay_escape_coverage_tests -- --nocapture` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test -p aura-components -- --nocapture` passed: 200 unit tests plus package integration tests.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.

### 2026-06-18 — Track D popup close-policy docs/examples slice

- Added close-policy examples to Select, Autocomplete, DatePicker, TimePicker, and ColorPicker Docs/Gallery coverage by applying `close_on_click_outside(false)` and `close_on_escape(false)` in representative scenarios.
- Updated compile-checked snippets and live docs renderers so the new public popup policy builders remain exercised.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo check -p aura-docs --bin check_snippets` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.

### 2026-06-18 — Track D Preview outside-close policy

- Added `close_on_click_outside(...)` to Preview and ActiveImagePreview state so image preview overlays can opt out of backdrop click dismissal independently from ESC handling.
- Preserved default outside-click close behavior and made the overlay click handler conditional on the policy flag.
- Added source-level regression coverage for Preview outside-click policy.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo test -p aura-components overlay_escape_coverage_tests -- --nocapture` passed.
- `cargo test -p aura-components preview::tests::preview_overlay_has_escape_close_action_and_image_sized_hitbox -- --nocapture` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test -p aura-components -- --nocapture` passed: 201 unit tests plus package integration tests.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.

### 2026-06-18 — Track D Preview close-policy docs/examples slice

- Documented Preview close policy controls in Docs by expanding ESC-only guidance into combined ESC and outside-click close strategy guidance.
- Updated the compile-checked Preview snippet and live docs renderer to exercise both `close_on_escape(false)` and `close_on_click_outside(false)`.
- Added a Gallery Preview close-policy example so the API-only overlay behavior is discoverable in the native demo app.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo check -p aura-docs --bin check_snippets` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.

### 2026-06-18 — Track D Tour close-policy docs/examples slice

- Hardened Tour overlay close-policy coverage by locking its ESC and outside-click conditional handlers in source-level tests.
- Added a controlled-close Tour Gallery example that disables both ESC and outside-click dismissal for critical guided flows.
- Added Tour close-policy docs and a compile-checked snippet, and fixed the docs snippet loader to display authored Tour snippets instead of falling back to missing-source text.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo test -p aura-components tour::tests -- --nocapture` passed.
- `cargo check -p aura-docs --bin check_snippets` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.

### 2026-06-18 — Track A CI/package workflow boundary docs

- Documented the responsibility split between `.github/workflows/ci.yml` and `.github/workflows/package.yml` in the Packaging Workflow docs page.
- Clarified that ordinary CI stops at validation/dry-run gates, while package workflow owns platform-specific packaging, raw binary staging, artifact upload, grouped changelog generation, and `v*` tag GitHub Release publishing.
- Added a docs regression test so the workflow boundary and release-asset rule stay visible in the native docs app.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo test -p aura-docs markdown::tests::packaging_docs_explain_ci_and_release_workflow_boundaries -- --nocapture` passed.
- `cargo check -p aura-docs --bin check_snippets` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.

### 2026-06-18 — Track F docs snippet loader completeness

- Audited all authored docs `src="..."` code blocks against the Docs UI snippet loader and compile-check harness.
- Fixed 22 Docs UI loader gaps for Calendar, Carousel, InputTag, Mention, Progress gradient completion, TreeSelect, VirtualizedTable, VirtualizedTree, and Watermark snippets; all files were already present and compile-checked, but the UI could not display them.
- Added a regression test that parses every docs page and asserts every referenced snippet can be loaded by the native Docs renderer.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo test -p aura-docs markdown::tests::authored_page_snippets_are_available_to_docs_loader -- --nocapture` passed.
- `cargo check -p aura-docs --bin check_snippets` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.

### 2026-06-18 — Track A split CI quality and packaging dry-run jobs

- Split `.github/workflows/ci.yml` into `rust-quality` and `packaging-dry-run` jobs.
- Kept full GPUI/Linux native dependencies only on the workspace fmt/check/test/docs-snippet job, while the packaging dry-run job now installs only lightweight tooling needed by `xtask` dry-run gates.
- Updated Packaging Workflow docs and regression tests to lock the CI job split and prevent packaging dry-run from silently inheriting unused rpm/zsync/native GUI dependency setup.

Validation evidence for this slice:
- Workflow YAML parsed successfully with PyYAML.
- `cargo fmt --all --check` passed.
- `cargo test -p aura-docs packaging -- --nocapture` passed.
- `cargo check -p aura-docs --bin check_snippets` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `cargo run -p xtask -- package validate` passed.
- `cargo run -p xtask -- package ci --all-apps --format platform-defaults --dry-run --skip-build` passed.
- `cargo run -p xtask -- package install-smoke --all-apps --format platform-defaults --dry-run` passed.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.

### 2026-06-18 — Track F QuickStart key binding completeness

- Updated the QuickStart minimal window snippet to register `CodeEditor` and `Tour` key bindings alongside the rest of the core app-level bindings.
- Added a docs regression test that compares the QuickStart example against Gallery and Docs for key bindings that affect text selection, code editing, Preview, and Tour overlay behavior.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo test -p aura-docs markdown::tests::quick_start_registers_core_app_key_bindings -- --nocapture` passed.
- `cargo check -p aura-docs --bin check_snippets` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.

### 2026-06-18 — Track E CodeBlock highlight cache eviction

- Replaced CodeBlock's all-or-nothing highlight cache clear with a bounded FIFO eviction policy so one cache overflow no longer invalidates every highlighted snippet at once.
- Kept HashMap lookup behavior for render speed while adding insertion-order tracking for incremental eviction.
- Added a regression test proving the oldest entry is evicted while newer cached runs survive at capacity.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo test -p aura-components code_block::tests -- --nocapture` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.

### 2026-06-18 — Track E CodeBlock shared highlight runs

- Changed the CodeBlock highlight cache value from owned `Vec<TextRun>` to shared `Arc<[TextRun]>` storage so repeated block renders reuse the cached highlight run allocation instead of cloning the full run vector for every visible CodeBlock/CodeEditor preview.
- Added a cached helper that returns the highlight key together with shared runs, letting selectable/read-only code layouts invalidate from the cache key while preserving existing public `cached_highlight_runs` behavior for inline styled text.
- Added a regression test proving repeated block highlight lookups share the same Arc-backed run storage.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo test -p aura-components code_block::tests -- --nocapture` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `cargo check -p aura-docs --bin check_snippets` passed.
- `git diff --check -- . ':(exclude).omx'` passed after removing markdown EOF whitespace.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.

### 2026-06-18 — Track B synchronized state panic hardening

- Replaced production `expect("... lock poisoned")` paths in CodeBlock highlight/selection state, SelectableText selection state, and Timer runtime registries with poisoned-lock recovery via `into_inner()`.
- Added small lock helper functions so cache/selection/timer runtime state can continue operating after an unrelated panic poisons a mutex instead of crashing the GPUI UI loop.
- Extended the API consistency panic audit to lock this behavior for CodeBlock, SelectableText, and Timer synchronized runtime state.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo test -p aura-components code_block::tests -- --nocapture` passed.
- `cargo test -p aura-components selectable_text::tests -- --nocapture` passed.
- `cargo test -p aura-components timer::tests -- --nocapture` passed.
- `cargo test -p aura-components api_consistency_audit_tests::avoidable_runtime_panics_stay_out_of_hardened_paths -- --nocapture` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `cargo check -p aura-docs --bin check_snippets` passed.
- `git diff --check -- . ':(exclude).omx'` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.

### 2026-06-18 — Track B tray icon fallback hardening

- Replaced Gallery and Docs startup tray icon `expect(...)` calls with recoverable bundled-icon loading helpers.
- Added app-specific solid-color fallback icons for Gallery and Docs, and allowed tray installation to proceed without an icon only if both bundled and fallback icon creation fail.
- Updated dynamic tray icon switching to skip invalid icon updates instead of panicking during command handling.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo check -p aura-gallery --all-targets` passed.
- `cargo check -p aura-docs --all-targets` passed.
- `cargo test -p aura-gallery shell_tests::gallery_shell_uses_container_and_menu -- --nocapture` passed.
- `cargo test -p aura-docs markdown::tests::docs_shell_registers_core_documentation_pages -- --nocapture` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `cargo check -p aura-docs --bin check_snippets` passed.
- `git diff --check -- . ':(exclude).omx'` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.

### 2026-06-18 — Track B packager string rendering panic cleanup

- Removed avoidable `expect("write to string")` calls from `aura-packager` checksum and manifest rendering paths.
- Switched checksum hex and manifest text/JSON assembly to infallible `push_str(format!(...))` style output while preserving existing generated formats.
- Verified `aura-packager` unit tests plus full workspace gates so packaging metadata generation remains stable.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo test -p aura-packager -- --nocapture` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `cargo check -p aura-docs --bin check_snippets` passed.
- `git diff --check -- . ':(exclude).omx'` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.

### 2026-06-18 — Track B lucide build script error handling

- Replaced `aura-icons-lucide` build script `unwrap()` calls with a `try_main() -> io::Result<()>` flow.
- Build failures now emit a clear `cargo:error=...` message and exit non-zero instead of panicking with an unwrap stack.
- Added explicit UTF-8 validation errors for generated icon file names and propagated directory/file/write errors with context from the build script path being processed.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo check -p aura-icons-lucide --all-targets` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `cargo check -p aura-docs --bin check_snippets` passed.
- `git diff --check -- . ':(exclude).omx'` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.

### 2026-06-18 — P15 final completion audit

P15 is complete. The final audit confirms all local quality hardening tracks are covered:

- Track A: CI/verification gates and CI/package workflow boundaries are in place and documented.
- Track B: API consistency and avoidable runtime panic cleanup passed the hardened-path audit, including synchronized UI state recovery, tray icon fallback handling, packager string rendering, and lucide build-script error handling.
- Track C: visual/theme consistency hardening uses theme tokens for representative colored text surfaces and chart labels.
- Track D: interaction/overlay behavior is covered for ESC and outside-click close policy across common overlays, popups, Preview, and Tour.
- Track E: CodeBlock performance hardening uses incremental highlight-cache eviction and shared `Arc<[TextRun]>` highlight-run storage; chart downsampling remains covered from prior phases.
- Track F: docs/snippet completeness is covered by the native Docs loader audit and compile-checked snippets; QuickStart key bindings are aligned with Gallery/Docs startup registration.

Final gate evidence:

- `cargo fmt --all --check` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `cargo check -p aura-docs --bin check_snippets` passed.
- `cargo run -p xtask -- package validate` passed.
- `cargo run -p xtask -- package ci --all-apps --format platform-defaults --dry-run --skip-build` passed.
- `cargo run -p xtask -- package install-smoke --all-apps --format platform-defaults --dry-run` passed.
- `git diff --check -- . ':(exclude).omx'` passed.
- `timeout 10s cargo run -p aura-gallery` started successfully and exited via expected timeout status `124`.
- `timeout 10s cargo run -p aura-docs` started successfully and exited via expected timeout status `124`.

Residuals intentionally outside P15 local completion:

- `MessageManager::init` panic remains intentional usage-contract enforcement.
- Gallery fixed date/time demo `expect(...)` calls are compile-time/demo constant assumptions, not production component paths.
- P12 external-policy items remain outside P15: signing, notarization, real system installs/uninstalls, formal license policy, and a real `v*` release validation run.
