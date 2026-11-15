# 006 Markdown and Diff Upgrade Execution - 2026-11-14

- 状态: 已完成
- 计划: `.context/plans/006_markdown_diff_upgrade_20261114.md`
- 目标: 在原生 GPUI 中完成 Markdown Rich 段落编辑/源码切换与 Diff 主题/审阅增强

## 已应用

- `markdown/model.rs`: parser enables tables, strike-through, task lists, footnotes, definition lists, superscript, subscript and GFM extensions; added regression coverage for visible text retention.
- `markdown/render.rs`: approved decoded images use intrinsic aspect-preserving dimensions bounded to 640×420; approved files retain a bounded loading frame; added sizing tests.
- `markdown.rs`: added `MarkdownEditor`, durable source synchronization, native Input-backed Source mode, Rich mode with editable paragraph blocks, inline source-range preservation when unambiguous, mode toggle and change events.
- `diff_view.rs`: configured UI/code fonts, CodeTheme-backed inline code rendering, hunk navigation, active-hunk state, copy actions, and keyboard-accessible toolbar actions; added copy-payload regression coverage.
- Docs pages document the new APIs and the explicit Markdown editor boundary.

## 当前边界

完整 Typora-like 同区 WYSIWYG 的全部结构化能力（富文本光标到 Markdown source offset 的全量双向映射、跨块结构编辑、富文本 IME composition/undo 分组）仍未声称完成；当前 Rich mode 已支持可编辑段落和可验证的 inline marker 保留，复杂结构继续使用 Source mode。

## 对标复核（2026-11-15）

参考 Typora Markdown Reference、Typora diagrams/math 文档和 MarkText README/syntax/editing 文档后，确认 parser extension 不能作为控件能力证据：当前任务列表、脚注、上下标、数学、图表、目录/内部链接、前置元数据、图片编辑工具和表格工具仍未达到对标交互。HTML/iframe/video 执行继续明确禁止；图片保持宿主授权、默认拒绝；富编辑继续以源码为真相，不承诺完整 Typora/MarkText WYSIWYG。

逐项能力、原生 GPUI 验收条件和安全边界记录在 `docs/markdown-capability-matrix.md`。本轮先交付规格，不实现上述高级缺口，也不把 fallback 可见文本标为已支持。

## 范围

- [x] CommonMark + GFM 常规语法及 pulldown 已有脚注、定义列表、上下标/数学事件。
- [x] Markdown 同区 Rich 段落编辑、即时渲染、源码模式切换和可验证 source offset 保持。
- [x] 宿主授权图片的比例保持、最大尺寸、布局失败状态和 alt 文本。
- [x] Diff 使用 Liora UI/code 字体、主题 token 和 `CodeTheme`，支持主题切换。
- [x] Diff hunk 导航、复制、审阅状态和 locate 能力增强。
- [x] Gallery/Docs 示例、snippets、测试和原生 smoke。

## 非目标

- 不引入 WebView、HTML/DOM、JavaScript、浏览器编辑器、公式渲染引擎或 PTY/文件写入业务。
- 不实现自动网络图片请求；不让 Markdown 控件执行 HTML 或命令。
- 不把 Diff 变成可写编辑器；不隐式应用或拒绝文件修改。
- 不顺便重构无关组件或重做全局设计系统。

## 预期文件

- `crates/liora-components/src/markdown.rs`
- `crates/liora-components/src/markdown/model.rs`
- `crates/liora-components/src/markdown/render.rs`
- 必要时新增 `crates/liora-components/src/markdown/editor.rs`
- `crates/liora-components/src/diff_view.rs`
- `apps/liora-gallery/src/demos/markdown_demo.rs`
- `apps/liora-gallery/src/demos/diff_demo.rs`
- Docs pages/snippets and stable project context

## 验收

- Parser tests prove common syntax events render without raw Markdown leakage for supported constructs.
- Markdown editor tests prove source round-trip, live update, paragraph edit boundaries, inline marker preservation when mapping is unambiguous, and source-mode switch; full rich-text composition/undo mapping remains out of scope.
- Image tests prove width/height bounds, aspect preservation and unresolved-resource state.
- Diff tests prove default typography/theme, theme changes, navigation order, copy payload and terminal decisions.
- Native smoke visibly exercises the supported GPUI surface; source-string-only claims do not substitute for runtime behavior.

## 验证

- Focused component tests first.
- `cargo fmt --all --check`.
- Targeted Gallery smoke and Docs snippet check.
- `cargo check --workspace --all-targets --locked` and `python scripts/context_bootstrap.py validate --root .` before delivery.

## 已执行验证

- `cargo test -p liora-components editor_mapping_tests --locked`: PASS, 3 tests.
- `cargo test --workspace --locked`: PASS, 715 passed, 1 ignored.
- `cargo check --workspace --all-targets --locked`: PASS.
- `cargo check -p liora-docs --bin check_snippets --locked`: PASS.
- `cargo fmt --all --check`: PASS.
- `python scripts/context_bootstrap.py validate --root .`: PASS.
- `git diff --check -- . ':(exclude).omx'`: PASS.
- `timeout 30s cargo run -p liora-gallery --example gpui_upgrade_smoke --locked`: PASS; native headless text/layout and retargeted spring smoke.
- GitHub Actions for pushed `79ffd4da`: Runtime verify, quality gates, and package workflows all completed successfully; API evidence: runs `43564703634`, `43563310673`, `43563310691`.

The native smoke does not cover real IME candidate windows or full hidden-markup cursor/composition/undo mapping; those remain the explicit current boundary.

## 风险与回滚

- Keep source text as the durable truth; revert only the new Markdown/Diff files and integration registrations if offset mapping or rendering regressions appear.
- Do not reset or restore unrelated user changes.
- Real IME candidate windows and Windows/macOS rendering remain separate validation boundaries unless explicitly run.
