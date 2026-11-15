# 009 Advanced Markdown and Native Editing Execution - 2026-11-15

- 状态: 部分完成；表格/Rich/数学与受限 Mermaid 功能通过定向回归；Mermaid 透明嵌套背景、SVG 百分比 opacity、自消息标签/箭头对比度 follow-up 已验证；Linux/Xwayland 亮/暗主题普通文本、上下标和图表标签已实际可见，原截图的白底/暗主题前景不匹配已修正。跨平台真实 IME 仍未验。
- 计划: `.context/plans/009_markdown_advanced_wysiwyg_20261115.md`
- 触发: 用户要求直接实现高级 Markdown，并验证完整 WYSIWYG/IME/macOS/Windows。

## 事实边界

- 当前宿主环境仅提供 Linux/Xwayland；真实 fcitx5/XIM 的段落与表格组合/提交/撤销已通过，不能据此证明 macOS 或 Windows 原生行为。
- 不会把编译、headless smoke 或 Linux 运行结果标为这些平台通过。
- “完整 Typora/MarkText 级 WYSIWYG”不是当前代码的增量验收项；必须拆成可观察能力并逐项通过测试。

 - [x] Heading IDs、`MarkdownDocument::headings()`、native TOC、host scroll/event navigation for fragments and footnotes.
 - [x] Native task checkbox and host-owned `MarkdownTaskToggled` source mutation event.
 - [x] Footnote numbering/source ranges, focused native preview, navigation to definitions and back buttons for each reference.
 - [x] GFM callout, definition, metadata, math parse semantics; `MarkdownDocument::metadata()` exposes flat source-backed fields and diagnostics without evaluating YAML/TOML.
 - [x] Highlight native span; escaped/complex ambiguous rich edits remain guarded.
 - [x] Source-backed table snapshots and edits, including ragged cells, quoted/list rows, CRLF/EOF, literal escaping, reorder and visual resize controls; retained cell entities preserve IME.
 - [x] Bounded rich inline Bold/Italic/Strike/Code/Highlight and paragraph/heading/quote/list/split actions with source-range rejection, undo/redo and host undo grouping.
 - [x] `latex-rust` math and restricted Mermaid flowchart/sequence renders in native GPUI; formulas/source remain selectable, bounded viewport sizing and visible errors. No browser runtime.
 - [x] Script spans use native 75% text runs with positive/negative baseline offsets; parser/offset tests and Linux/Xwayland light/dark captures verify visible prose and script glyphs.
 - [ ] Full Typora/MarkText structural parity; explicitly out of scope, bounded controls only.
 - [x] Linux/fcitx5 real XIM acceptance on private D-Bus/Xwayland: paragraph and table preedit preserve canonical Markdown, actual `你好` commit preserves surrounding Markdown, and undo restores exact source. macOS/Windows runtime remains unavailable.

## Validation status

 - PASS: `cargo test -p liora-components markdown:: --offline` (76 tests before the transparent Mermaid contrast follow-up; contrast follow-up validation recorded below).
 - PASS: `cargo test -p liora-components markdown::diagram::tests:: --offline` (11 diagram tests; nested translucent/`none` rects, inline/class percentage opacity, self-message labels and active self-loop contrast).
 - PASS: `WAYLAND_DISPLAY=wayland-1 bash scripts/markdown_native_visibility.sh` (Linux/Xwayland: light text=1838/math=298/labels=120/edge=277; dark text=2476/math=307/labels=159/edge=277).
 - PASS: `cargo fmt --all --check`.
 - PASS: `cargo check -p liora-gallery --examples --offline`.
 - PASS: `cargo run -p liora-gallery --example markdown_editing_smoke --offline` and `markdown_navigation_smoke --offline` (composition preservation/commit, history, stale-input selection, table entity retention, TOC event).
 - PASS: Xwayland `:27` native smoke (synthetic GPUI marked-text composition commit, undo guard, OS clipboard copy/paste); table editor was inspected in a live native screenshot.
 - PASS: Native MarkdownView light/dark captures show ordinary text, superscript/subscript, inline/display math, Mermaid labels/edges and table text. Missing generic font aliases now fall back to an installed system font; default Mermaid art retains a light canvas.
 - PASS: `cargo run -p liora-docs --bin check_snippets --offline`, `cargo fmt --all --check`, CTX validate.
 - BLOCKED: native macOS/Windows runtime and platform-specific IME/clipboard proof; Linux/fcitx5 XIM is separately verified by `scripts/markdown_real_ime_smoke.py` in an isolated environment.
 - PASS: Real IME harness exit 0; paragraph/table `PREEDIT`, `COMMIT`, `UNDO` handshakes observed, final PASS withheld until app exit and cleanup. Negative `/bin/false` check exits 1 without PASS.
 - NOT CLAIMED: full WYSIWYG parity; complete Unicode line-break behavior; macOS/Windows runtime. Public editor mutation while composing is rejected with `CompositionActive`.
