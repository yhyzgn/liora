# 011 Quiet.md-informed native Markdown editor — 2026-11-17

- Status: completed within the canonical Markdown contract; Typora-style syntax-driven correction and exact verification: `.context/tasks/011_quiet_markdown_native_20261117.md`.
- User request: study https://github.com/s1dashu/quiet-md and implement its reusable Markdown editor capabilities and smooth native interaction in Liora.
- Accepted document contract: canonical Markdown, chosen explicitly by the user. Preserve untouched source bytes; never replace persistence with DocxXML or silently flatten unsupported content.
- Reference pinned at e9cb5408d0ffcad5bf5b31814e10e1b2f275347f under `/tmp/liora-quiet-md-reference`; MIT. Upstream assets and patched GPUI are not imported. Reimplement architecture using official workspace GPUI f0d896f2c18a543335eb155f729d36b624695591.

## Scope and acceptance

1. One editable native surface: type Markdown syntax such as `# `, `**strong**`, `- ` and code fences directly, with immediate native rendering and canonical punctuation preserved. No format button, external program call or mode switch is required for this flow. This targets the Typora-style interaction model, not full Typora parity.
2. One retained editor focus/input owner, document-wide selection, shared native layout for painting/hit testing/IME, and variable-height virtual rows. Preserve literal programmatic `insert_text` semantics separately from committed syntax input.
3. Source-preserving insertion/replacement, grapheme deletion, Enter splitting/list continuation and exit, Backspace joining/outdent, optional keyboard commands, code editing, task toggles, block move/fold, literal find/replace, selection-aware undo/redo and composition grouping.
4. Preserve existing math/diagram/images/footnotes/metadata/definition/callout rendering and host image authorization. Native tables retain source editing/reorder/resize; no merged-cell persistence in Markdown. Migrate concrete paragraph Input consumers to canonical selection APIs and keep source `input()` integration available without requiring it for typing Markdown.
5. Exercise the actual Gallery/Docs shared bounded editor through native `EntityInputHandler` callbacks; type syntax character by character, assert canonical source and undo/redo, and preserve wheel scrolling. Updated tests, actual native captures and real XIM evidence are recorded in task 011; historical performance figures remain CPU-only.

## Format and host boundaries

- Markdown-compatible features are implemented through source transactions; global reference/footnote dependencies require a full-parse correctness fallback when necessary.
- No implicit file IO, downloads, application storage, navigation database, account, update or save service in SDK.
- Underline/colors, columns, merged cells, crops/dimensions, buttons/reminders/countdowns and boards have no accepted lossless Markdown storage format. They remain explicitly outside this Markdown contract rather than being exported lossily.
- Windows/macOS and actual system IME claims require their own observed evidence.

## Implementation ownership

- `rich_engine.rs`: host/UI-independent source transactions, document positions/selection, commands and history.
- `rich_native.rs`: one GPUI input owner, styled layout and virtual rows; the default editor and shared demo expose no formatting toolbar.
- `markdown.rs` and consumers: public wrapper/cutover and acceptance integration.
