# Markdown capability parity matrix

This is a capability specification, not a claim that Liora currently matches Typora or MarkText. The references are used to define observable behavior; no external source code is copied.

References:

- Typora Markdown reference: <https://support.typora.io/Markdown-Reference/>
- Typora diagrams: <https://support.typora.io/Draw-Diagrams-With-Markdown/>
- Typora math: <https://support.typora.io/Math/>
- MarkText features: <https://github.com/marktext/marktext#features>
- MarkText syntax: <https://marktext.me/docs/markdown-syntax>
- MarkText editing tools: <https://marktext.me/docs/editing>

## Status vocabulary

- **Rendered**: the parsed element has a dedicated visible native GPUI representation.
- **Preserved**: source content remains visible, but the semantic behavior is not implemented.
- **Interactive**: pointer/keyboard actions update state or invoke a host callback.
- **Rich-editable**: editing the rendered form preserves a defined source mapping and native undo/IME behavior.

## Current matrix

| Capability | Current Liora behavior | Required parity target | Native implementation boundary |
|---|---|---|---|
| CommonMark/GFM paragraphs, headings, quotes, lists, rules and fenced/indented code | Rendered as native text, containers and code surfaces. | Keep visible structure, inline styles, wrapping and copy behavior correct. | No HTML/DOM runtime. |
| Strong, emphasis, strike and inline code | Rendered as `TextRun` styles. | Add regression fixtures for nested and escaped spans. | Use GPUI text runs. |
| Heading IDs, internal links and `[toc]` | Parsed headings receive unique document-local IDs, including collisions with suffixed titles; `MarkdownDocument::headings()` provides TOC data, `MarkdownView::set_show_toc(true, cx)` opts into native Contents buttons, and safe `#fragment` activation emits `MarkdownNavigationRequested`. Host must map the fragment to an actual scroll target. | Stable document-local heading IDs, safe fragment activation, generated TOC and scroll-to-target. | Navigation remains host-owned; external URL validation is unchanged. |
| Task lists | Parsed task markers are semantic; list items render native focusable checkboxes and emit `MarkdownTaskToggled` with block and source-marker range. Host owns source replacement and persistence. | Native checkbox presentation, click/keyboard toggle, and host-controlled source update. | No implicit file writes. |
| Tables | Native table-cell inputs retain composition; insert/delete/reorder row/column actions and 24 px column resize controls mutate source-backed rows atomically where supported. Ragged trailing cells can be materialized; literal cell edits are escaped and intentionally discard prior inline markup. | Preserve untouched cells/rows, keep edits within supported container prefixes, and retain native keyboard/IME interaction. | Widths are visual state only; unsupported/ragged structures reject edits instead of partially mutating source. |
| Footnotes | Numbered references and definitions have document-local native targets; `MarkdownDocument::footnotes()` exposes reference source ranges, occurrences and back-link IDs. Reference buttons navigate to definitions, keyboard/pointer back buttons cover every reference, and focused references show a native preview. | Numbering, reference-to-definition navigation, keyboard preview and all backreferences. | Host owns actual scroll-to-target. |
| Inline and display math | Inline/display expressions are typeset with `latex-rust`/STIX Two Math into bounded native GPUI images. Default glyph RGB follows the surrounding text; explicit LaTeX colors retain their scope. Render failures display a diagnostic and source. | Source-preserving edits, full TeX coverage and typographic baseline/layout integration. | CPU raster output; output is capped by source, pixel and PNG limits. |
| Diagram fences (`mermaid`, `flow`, `flowchart`, sequence, PlantUML, Vega-Lite) | Mermaid flowchart and sequence blocks render through `mermaid-svg` + `usvg`/`resvg` into bounded native images. Missing generic font aliases fall back to an installed font; no installed font produces a visible diagnostic. The engine's default light palette retains an opaque light canvas on dark surfaces. Unsupported fences show an explicit diagnostic and source. | Broader diagram-language support, source copy affordance and renderer-specific fidelity. | No SVG href resolution, script execution, or network fetch; PlantUML and Vega-Lite unsupported. |
| Highlight `==…==` | In one parser text event, paired non-empty unescaped markers become a highlighted native `TextRun` span; markers are removed from visible text but remain in source ranges. Rich edits reject ambiguous mappings rather than delete syntax. Cross-event/nested markers need additional fidelity tests. | Full highlight editing and nested-span fidelity. | Model style bit and native background; no HTML styling. |
| Subscript/superscript | Parser semantics feed 75%-size native text runs shifted up/down; source spans remain selectable. Linux/Xwayland light/dark native captures show adjacent prose, scripts, fractions, display math, Mermaid labels/edges and table text. The earlier invisible-text capture used a white smoke background with a system-dark foreground. | Preserve script baselines and selection/copy alongside adjacent prose and formulas. | Native word segmentation is a bounded heuristic, not complete Unicode line breaking. Native pixel smoke and per-label raster tests supplement parser/offset checks. |
| Underline and raw HTML styling | HTML is literal text; links may be underlined. | Keep raw HTML inert. Add Markdown-native underline only if an explicit extension is selected. | Never execute HTML/CSS/DOM. |
| YAML/plus-delimited metadata | Metadata blocks receive a semantic native metadata block and bounded visible label; `MarkdownDocument::metadata()` exposes flat source-backed key/value fields and diagnostics without evaluating YAML/TOML. | Controlled metadata presentation or hide-by-policy, richer format parsing only if required. | No arbitrary metadata-driven resource paths or commands. |
| Images | Host-approved local file or decoded image only; default denied; alt text retained. | Inline placement, alignment, resize metadata and host-controlled insertion. | Host owns network, redirect, file and decode policy; SDK never downloads URLs. |
| Video, iframe and other embeds | Raw HTML is literal text. | No default support. If a host needs media, use an explicit native host-owned asset/control API. | No WebView/media HTML execution in Markdown controls. |
| GitHub alerts/callouts and definition lists | GFM callout kinds and definition-list tags receive semantic native block kinds; callouts get severity labels and definitions get term/body layout. | Dedicated callout severity and definition-term/definition-body layout. | Native containers, source-preserving model kinds. |
| Rich/source editing | Source mode retains native IME, selection and clipboard; rich mode supports bounded inline toggles plus paragraph/heading/quote/list/split actions and a retained table editor. Canonical Markdown remains source of truth; undo/redo and explicit host undo groups remain available. | Expand structural operations only with source-roundtrip tests; keep ambiguous syntax guarded. | Table-cell replacement is literal plain text; rich mode is not Typora/MarkText parity. Composition, clipboard and focus proof is Linux/Xwayland only. |
| Export | No Markdown control-owned HTML/PDF export contract. | Define explicit host export adapters and fidelity tests if requested. | Export is host-owned; no implicit filesystem writes. |

## Acceptance rules for future implementation

1. A parser option is not support. Each row must have a renderer test, and interactive rows need a pointer/keyboard test with observable state or host event output.
2. Rich-editable rows need source round-trip tests for ASCII, Unicode, escaped entities, nested styles, and failed/ambiguous mappings.
3. Security-sensitive rows must prove that HTML, `javascript:`, `file:`, `data:`, relative URLs, unapproved images and implicit downloads stay blocked.
4. Math and diagram work must name the Rust/native engine and license before dependency changes. A rendered screenshot alone does not establish source fidelity or safe failure behavior.
5. Docs and Gallery must label unsupported behavior explicitly; they must not call parser-visible fallback text a rendered feature.
6. Cross-platform claims require native runtime evidence on the target platform. Linux headless checks do not prove IME, clipboard, GPU, macOS or Windows behavior.

## Current decision

The implemented native slices are documented above and covered by focused tests; this is not a parity declaration. Visual sub/superscript baseline shifts, table action UI/reorder/resize, arbitrary structural rich editing, rich-mode IME validation, and macOS/Windows native evidence remain incomplete.
