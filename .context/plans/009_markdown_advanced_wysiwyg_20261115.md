# 009 Advanced Markdown and Native Editing - 2026-11-15

- 状态: 进行中
- 目标: 在纯 Rust + 原生 GPUI 边界内，逐步实现 Liora 的高级 Markdown 原生渲染、交互和可控 Rich 编辑。
- 当前任务: `.context/tasks/009_markdown_advanced_wysiwyg_20261115.md`

## 约束

- Source remains the durable Markdown truth; Rich mode must preserve source when mapping is unambiguous.
- No HTML/DOM/WebView/JavaScript execution, implicit network access, arbitrary HTML/CSS, or host file writes.
- Math/diagram rendering must use a documented Rust/native engine or remain a safe source-preserving fallback; parser events alone are not support.
- SDK exposes callbacks/events for source mutation and navigation; it does not silently modify host files.
- Cross-platform claims require evidence from the corresponding native runtime. Linux CI/headless cannot certify real IME, macOS, or Windows behavior.

## Capability order

1. Heading IDs, document-local fragments, generated TOC and navigation callback.
2. Native task-list checkbox presentation and host-owned source mutation event.
3. Footnote numbering, definition presentation, reference/definition/backreference events.
4. Highlight, subscript, superscript, callout and metadata model/rendering.
5. Safe math/diagram adapter contracts with explicit fallback/error states.
6. Rich editing operations for supported inline and block mappings, undo grouping and composition preservation.
7. Table editing operations and source round-trip fixtures.

## Verification gates

- Parser/model: unit tests for ASCII, Unicode, escaped syntax, nesting, malformed input, stable IDs and source ranges.
- Native rendering: focused GPUI component tests and Linux real-display smoke for keyboard/pointer interaction.
- Security: HTML remains inert; unsafe links/images and implicit downloads remain denied.
- Rich editing: source round-trip, marker preservation, ambiguous mapping rejection, undo and composition tests.
- Platform: macOS and Windows must be run on those platforms with native IME and clipboard; unavailable evidence remains BLOCKED, never PASS.

## Explicit non-claim

This plan does not promise full Typora/MarkText implementation equivalence. It defines observable native capabilities that can be implemented safely and verified in this repository. Full parity requires a separate product-level acceptance matrix and target-platform test runners.
