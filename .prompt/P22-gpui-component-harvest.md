# P22 — gpui-component Harvest

## Status

🧭 Active — started 2026-06-25

## Goal

Turn the `design/gpui-component-collection-list.md` research into Liora-native components and enhancements while preserving Liora's Element-Plus-style builder APIs, theme tokens, and pure Rust + GPUI native architecture.

## Non-negotiable boundaries

- Liora remains pure Rust + GPUI native.
- Do not introduce WebView, HTML/CSS/DOM, browser runtime, WASM gallery, or Tauri.
- Do not copy `longbridge/gpui-component` APIs directly; adapt only the product capability into Liora style.
- Existing components must be enhanced in place rather than replaced by parallel controls.
- Keep Gallery and Docs as the canonical adoption/dogfooding apps; do not re-add standalone sample apps.

## Wave A — low-risk high-value controls

Initial wave:

- ✅ `Spinner` — standalone inline loading indicator for buttons, status bars, rows, and toolbars.
- ✅ `Kbd` — keyboard shortcut keycap display for menus, command palettes, empty states, and docs.
- ✅ `OtpInput` — interactive OTP/PIN input for 2FA/device pairing flows; click-to-focus, caret positioning, keyboard input, backspace, paste, mask/status/size demos.
- ✅ `DropdownButton` — split-capable dropdown command button with Gallery/Docs/snippet coverage
- ✅ `Accordion`
- ⬜ `Combobox`

## Required pattern for each new component

1. Add reusable component source under `crates/liora-components/src/<name>.rs`.
2. Export from `crates/liora-components/src/lib.rs` and keep module docs covered by the public-doc regression.
3. Add Gallery coverage under `apps/liora-gallery/src/demos/` and register it.
4. Add Docs page, external snippet, snippet compile-check import, and live demo mapping.
5. Add focused unit/source tests for non-visual behavior.
6. Run targeted checks before broad workspace gates.

## Next recommended work

Continue Wave A with `Accordion`. Treat `Combobox` as a larger step because it should likely share SearchableList/Select/Autocomplete infrastructure instead of duplicating popup filtering logic.
