# P22 — gpui-component Harvest

## Status

✅ Complete — started 2026-06-25; closed 2026-06-29

## Goal

Turn the `design/gpui-component-collection-list.md` research into Liora-native components and enhancements while preserving Liora's Element-Plus-style builder APIs, theme tokens, and pure Rust + GPUI native architecture.

## Non-negotiable boundaries

- Liora remains pure Rust + GPUI native.
- Do not introduce WebView, HTML/CSS/DOM, browser runtime, WASM gallery, or Tauri.
- Do not copy `longbridge/gpui-component` APIs directly; adapt only the product capability into Liora style.
- Existing components must be enhanced in place rather than replaced by parallel controls.
- Keep Gallery and Docs as the canonical adoption/dogfooding apps; do not re-add standalone sample apps.

## Harvest closure

The full `design/gpui-component-collection-list.md` backlog is closed. Outcomes are one of:

- ✅ Standalone Liora components added where the capability needed a distinct public surface: `Spinner`, `Kbd`, `OtpInput`, `DropdownButton`, `Accordion`, `Sidebar`, `StatusBar`, `DockLayout`, `Settings`, `Sheet`, `HoverCard`, `GroupBox`, `ScrollableMask`, `CandlestickChart`, and `SearchableList`.
- ✅ Existing controls enhanced in place where a parallel component would fragment the API: Combobox-style workflows live in searchable `Select`/`Autocomplete` plus shared `SearchableList`; DataTable capability is folded into `VirtualizedTable`; TextView/document needs are covered by `Text`, `SelectableText`, `CodeBlock`, and the native Docs markdown renderer; WindowExt/TitleBar/WindowBorder capability is covered by `WindowFrame`/`TitleBar`; resizable panel capability is covered by `Splitter`; Toggle-style toolbar and view-mode workflows are covered by `Switch`, `Segmented`, and button-style selections; CodeEditor advanced work remains in the existing `CodeEditor` extension surface.
- ✅ Explicitly not collected where it conflicts with project boundaries or duplicates existing coverage: WebView, WASM gallery, browser runtime paths, and basic controls already present in Liora.

## Required pattern for any future follow-up

1. Add reusable component source under `crates/liora-components/src/<name>.rs`.
2. Export from `crates/liora-components/src/lib.rs` and keep module docs covered by the public-doc regression.
3. Add Gallery coverage under `apps/liora-gallery/src/demos/` and register it.
4. Add Docs page, external snippet, snippet compile-check import, and live demo mapping.
5. Add focused unit/source tests for non-visual behavior.
6. Run targeted checks before broad workspace gates.

## Next recommended work

No remaining P22 collection backlog. Future requests that resemble gpui-component items should be treated as ordinary Liora maintenance: enhance the existing closest component in place, keep Gallery/Docs canonical, and do not revive standalone `Combobox` or browser/WebView/WASM paths.
