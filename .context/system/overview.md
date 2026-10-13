# System Overview

## Verified project summary

Liora is a Rust edition 2024 Cargo workspace for a pure Rust + GPUI native enterprise UI component library. Evidence: `Cargo.toml`, `crates/`, `apps/liora-gallery`, `apps/liora-docs`, `README.md`.

## Workspace topology

- `crates/liora`: one-stop SDK facade.
- `crates/liora-core`: global config, initialization, locale/codegen support, utility traits.
- `crates/liora-theme`: design tokens, theme modes, font configuration.
- `crates/liora-icons` and `crates/liora-icons-*`: icon primitive, asset loading, bundled typed icon libraries.
- `crates/liora-icons-optimizer`: build-time SVG bundle optimizer.
- `crates/liora-components`: reusable GPUI components.
- `crates/liora-tray`, `liora-packager`, `liora-updater`: reusable app-shell/packaging/update SDK crates.
- `apps/liora-gallery`: native component gallery.
- `apps/liora-docs`: native documentation application.

## Build, test, and release commands

Verified command set from `README.md`, `.github/workflows/ci.yml`, and local release history:

```bash
cargo fmt --all --check
cargo check --workspace --all-targets
cargo test --workspace
cargo check -p liora-docs --bin check_snippets
cargo run --release -p xtask -- package validate
cargo run --release -p xtask -- package release-readiness
git diff --check -- . ':(exclude).omx'
```

## Current GPUI baseline

Current validated refresh target: official `zed-industries/zed` revision `1de16662ee0d704285c524a7753f0f8858e86abf` for `gpui`/`gpui_platform`/`collections` and the Zed dependency closure. GPUI crate version remains `0.2.2`; the git SHA defines compatibility. Intake date: 2026-10-13. Final observed main `d3267a39ff900f114711c26720c7cbf12c41f204` changes only agent-server/agent-UI/terminal-view code, with no GPUI/collections/workspace dependency diff.

## Current work and release boundary

Current plan/task: `004_gpui_native_replacement_20261013`. Workspace package versions remain `0.3.0`; this refresh does not publish, tag, commit or push. Prior release-candidate context is historical, not proof of current published assets.

## GPUI refresh integration

- `MotionSpring` provides opt-in native interruptible presets; old duration-based helpers remain unchanged.
- Gallery Spinner demonstrates spring retargeting; `gpui_upgrade_smoke` example exercises native Linux headless layout and reduced-motion target resolution without a display or GPU.
- Custom macOS frame setup separates app-owned titlebar dragging from native movability; System resets ownership.
- New touch/headless/display/profiler APIs are available upstream; no blanket touch/mobile/web support claim is made for custom Liora controls.
- Upgrade evidence and risk details: `docs/gpui-latest-upgrade.md` and `.context/tasks/004_gpui_native_replacement_20261013.md`.

<!-- ctx-managed-legacy-migration:start -->

## Migrated legacy source units

The following sections preserve legacy context content verbatim enough for auditability. Prefer the summarized CTX sections above for day-to-day work.

### memory-inventory-md-0001-4a998b152360

<!-- ctx-migration source=".memory/inventory.md" unit="memory-inventory-md-0001-4a998b152360" sha256="4a998b152360782c2d591e371534d251cfde34e6ca865e2f6d7d7b6937bcb86a" -->

# Component Inventory


### memory-inventory-md-0002-16a203a5a4dc

<!-- ctx-migration source=".memory/inventory.md" unit="memory-inventory-md-0002-16a203a5a4dc" sha256="16a203a5a4dcb9cff836ea638902b50de6bb362cc59b2be5ed38143add06f253" -->

## P0 Foundation ✅

| Component | File | Tests | Demo |
|-----------|------|-------|------|
| Theme (light/dark) | `crates/liora-theme/src/lib.rs` | — | — |
| Config (Global) | `crates/liora-core/src/lib.rs` | — | — |
| ContextExt trait | `crates/liora-core/src/lib.rs` | — | — |
| ElementExt trait | `crates/liora-core/src/lib.rs` | — | — |
| Z-Index utils | `crates/liora-core/src/lib.rs` | — | — |
| Button | `crates/liora-components/src/button.rs` | — | ✅ |
| Gallery app | `apps/liora-gallery/src/` | — | ✅ |


### memory-inventory-md-0003-16428a8cccd0

<!-- ctx-migration source=".memory/inventory.md" unit="memory-inventory-md-0003-16428a8cccd0" sha256="16428a8cccd09c4f527544ec70ea6ecc77931522c42db6aecdff9810aced515c" -->

## P1 Basic Elements ✅ (15/15)

| # | Component | File | Demo | Status |
|---|-----------|------|------|--------|
| 1 | Icon + liora-icons-lucide | `crates/liora-icons/` `crates/liora-icons-lucide/` | ✅ | ✅ Icon system done |
| 2 | Button (codex refactor) | `crates/liora-components/src/button.rs` | ✅ | ✅ codex complete |
| 3 | Link | `crates/liora-components/src/link.rs` | ✅ | ✅ |
| 4 | Text | `crates/liora-components/src/text.rs` | ✅ | ✅ |
| 5 | Title | `crates/liora-components/src/title.rs` | ✅ | ✅ |
| 6 | Paragraph | `crates/liora-components/src/paragraph.rs` | ✅ | ✅ |
| 7 | Space | `crates/liora-components/src/space.rs` | ✅ | ✅ Container gap support |
| 8 | Divider | `crates/liora-components/src/divider.rs` | ✅ | ✅ |
| 9 | Row (栅格) | `crates/liora-components/src/row.rs` | ✅ | ✅ |
| 10 | Col (栅格) | `crates/liora-components/src/col.rs` | ✅ | ✅ Percent width fix |
| 11 | Container | `crates/liora-components/src/container.rs` | ✅ | ✅ |
| 12 | Scrollbar | `crates/liora-components/src/scrollbar.rs` | ✅ | ✅ |
| 13 | Splitter | `crates/liora-components/src/splitter.rs` | ✅ | ✅ |
| 14 | ButtonGroup | `crates/liora-components/src/button_group.rs` | ✅ | ✅ |
| 15 | CodeBlock | `crates/liora-components/src/code_block.rs` | ✅ | ✅ Code highlighting, language label, copy button, inline/block formats |


### memory-inventory-md-0004-67a119437867

<!-- ctx-migration source=".memory/inventory.md" unit="memory-inventory-md-0004-67a119437867" sha256="67a119437867a4122558de7b0796dfe98dbf7127c125e626af4dc16febcb11fa" -->

## P2 Form Controls 🔄 (8/10)

| # | Component | File | Status |
|---|-----------|------|--------|
| 1 | Input | `crates/liora-components/src/input.rs` | ✅ |
| 2 | InputNumber | `crates/liora-components/src/input_number.rs` | ✅ |
| 3 | Textarea | `crates/liora-components/src/textarea.rs` | ✅ |
| 4 | Checkbox / CheckboxGroup | `crates/liora-components/src/checkbox.rs`, `checkbox_group.rs` | ✅ Vertical/Horizontal/Button layouts |
| 5 | Radio / RadioGroup | `crates/liora-components/src/radio.rs`, `radio_group.rs` | ✅ Vertical/Horizontal/Button layouts |
| 6 | Switch | `crates/liora-components/src/switch.rs` | ✅ |
| 7 | Select | `crates/liora-components/src/select.rs` | ✅ |
| 8 | Slider | `crates/liora-components/src/slider.rs` | ✅ |
| 9 | Form / FormItem | `crates/liora-components/src/form.rs` | ✅ |
| 10 | Rate | `crates/liora-components/src/rate.rs` | ✅ |


### memory-inventory-md-0005-f67014d08359

<!-- ctx-migration source=".memory/inventory.md" unit="memory-inventory-md-0005-f67014d08359" sha256="f67014d08359cae48f3fbaaea6989f7866c9055b2c3222d80fdc4d029d4bf6e7" -->

## P3 Popper + Feedback ✅ (13/13)

| # | Component | Notes | Status |
|---|-----------|-------|--------|
| — | Popper/Portal 基建 | `crates/liora-core/src/popper.rs` | ✅ Done |
| 1 | Tooltip | — | ✅ Done |
| 2 | Popover | — | ✅ Done |
| 3 | Popconfirm | — | ✅ Done |
| 4 | Dialog | — | ✅ Done |
| 5 | Drawer | — | ✅ Done |
| 6 | Message | — | ✅ Done |
| 7 | Notification | — | ✅ Done |
| 8 | Alert | — | ✅ Done |
| 9 | Loading | — | ✅ Done |
| 10 | MessageBox | — | ✅ Done |
| 11 | Dropdown | — | ✅ Done |
| 12 | Card | — | ✅ Done |
| 13 | Collapse | — | ✅ Done |


### memory-inventory-md-0006-d3c4746a7787

<!-- ctx-migration source=".memory/inventory.md" unit="memory-inventory-md-0006-d3c4746a7787" sha256="d3c4746a77871ac0269a09bd6201cfdc1c75ae1222d42e4315965d78dcbd196d" -->

## P4 Nav + Data 🔄 (1/20)

| # | Component | File | Demo | Status |
|---|-----------|------|------|--------|
| 1 | Menu | `crates/liora-components/src/menu.rs` | ✅ | ✅ Horizontal/Vertical/Collapse |
| 2 | Tabs | `crates/liora-components/src/tabs.rs` | ✅ | ✅ Positions/Styles/Editable |
| 3 | Breadcrumb | `crates/liora-components/src/breadcrumb.rs` | ✅ | ✅ String/Icon Separators |
| 4 | Steps | `crates/liora-components/src/steps.rs` | ✅ | ✅ Horizontal/Vertical/Status |
| 5 | PageHeader | `crates/liora-components/src/page_header.rs` | ✅ | ✅ Title/SubTitle/Slots |
| 6 | Affix | `crates/liora-components/src/affix.rs` | ✅ | ✅ Top/Bottom Sticky |
| 7 | Backtop | `crates/liora-components/src/backtop.rs` | ✅ | ✅ Visibility Height |
| 8 | Anchor | `crates/liora-components/src/anchor.rs` | ✅ | ✅ Scroll Sync / Jump |
| 9 | Progress | `crates/liora-components/src/progress.rs` | ✅ | ✅ Line Style / Status |
| 10 | Skeleton | `crates/liora-components/src/skeleton.rs` | ✅ | ✅ Variants / Rows |
| 11 | Empty | `crates/liora-components/src/empty.rs` | ✅ | ✅ Default / Custom / Action |
| 12 | Result | `crates/liora-components/src/result.rs` | ✅ | ✅ Success/Warning/Error/Info |
| 13 | Descriptions | `crates/liora-components/src/descriptions.rs` | ✅ | ✅ Border / Direction / Grid |
| 14 | Timeline | `crates/liora-components/src/timeline.rs` | ✅ | ✅ Node variants / Reverse |
| 15 | Tree | `crates/liora-components/src/tree.rs` | ✅ | ✅ Expand / Collapse |
| 16 | Pagination | `crates/liora-components/src/pagination.rs` | ✅ | ✅ Layout / Pager |
| 17 | Statistic | `crates/liora-components/src/statistic.rs` | ✅ | ✅ Prefix / Suffix |
| 18 | Segmented | `crates/liora-components/src/segmented.rs` | ✅ | ✅ Block / Disabled |
| 19 | Tag | `crates/liora-components/src/tag.rs` | ✅ | ✅ Light / Dark / Plain |
| 20 | Avatar | `crates/liora-components/src/avatar.rs` | ✅ | ✅ Image / Icon / Shapes |
| 21 | Badge | `crates/liora-components/src/badge.rs` | ✅ | ✅ Value / Dot / Max |


### memory-inventory-md-0007-0e9aee39e226

<!-- ctx-migration source=".memory/inventory.md" unit="memory-inventory-md-0007-0e9aee39e226" sha256="0e9aee39e22627401f569ae5533f5d727810dee2e17888044a8e3b8f77d52502" -->

## P5 Advanced 🏁 (11/20 complete; deferred scope moved to P9)

| # | Component | File | Demo | Status |
|---|-----------|------|------|--------|
| 1 | Table | `crates/liora-components/src/table.rs` | ✅ | ✅ P0 basic / custom header / opt-in sort / fixed header / empty / loading / border / stripe |
| 2 | DatePicker | `crates/liora-components/src/date_picker.rs` | ✅ | ✅ Formats / date range / month + month range / year + year range / disabled / callback |
| 3 | TimePicker | `crates/liora-components/src/time_picker.rs` | ✅ | ✅ Fixed time / steps / formats / hide seconds / disabled / callback |
| 4 | DateTimePicker | `crates/liora-components/src/date_time_picker.rs` | ✅ | ✅ DateTime / DateTime range / formats / steps / hide seconds / confirm / disabled / callbacks |
| 5 | Upload | `crates/liora-components/src/upload.rs` | ✅ | ✅ Button / drag style / text list / picture card / progress / status / limit / disabled / callbacks |
| 6 | Cascader | `crates/liora-components/src/cascader.rs` | ✅ | ✅ Multi-level / default selected / disabled / clearable / loading / lazy load / search results / callbacks |
| 7 | Transfer | `crates/liora-components/src/transfer.rs` | ✅ | ✅ Source/target panels / checked move / disabled items / filter display / callbacks |
| 8 | ColorPicker | `crates/liora-components/src/color_picker.rs` | ✅ | ✅ Cube trigger / popup HSV panel / hue bar / alpha bar / rgba display / presets / disabled / callback |
| 9 | Carousel | — | — | ↩️ Moved to P9 deferred backlog |
| 10 | Image | `crates/liora-components/src/image.rs` | ✅ | ✅ Remote URL / local file / fit modes / circle + round options / transparent ring sleeve / loading + fallback / empty / preview |
| 11 | Calendar | — | — | ↩️ Moved to P9 deferred backlog |
| 12 | TreeSelect | — | — | ↩️ Moved to P9 deferred backlog |
| 13 | Autocomplete | `crates/liora-components/src/autocomplete.rs` | ✅ | ✅ Static suggestions / filtering / click select / clearable / disabled / demo |
| 14 | InputTag | — | — | ↩️ Moved to P9 deferred backlog |
| 15 | Mention | — | — | ↩️ Moved to P9 deferred backlog |
| 16 | Watermark | — | — | ↩️ Moved to P9 deferred backlog |
| 17 | Tour | — | — | ↩️ Moved to P9 deferred backlog |
| 18 | Scrollbar | `crates/liora-components/src/scrollbar.rs` | ✅ | ✅ Already completed in P1 |
| 19 | Splitter | `crates/liora-components/src/splitter.rs` | ✅ | ✅ Already completed in P1 |
| 20 | VirtualizedTable/VirtualizedTree | — | — | ↩️ Moved to P9 deferred backlog |


### memory-inventory-md-0008-988651ee8057

<!-- ctx-migration source=".memory/inventory.md" unit="memory-inventory-md-0008-988651ee8057" sha256="988651ee805724ff3b6ed1b1c83ba244182e6d0080a8f05ab1f504ed11cd64b5" -->

## P6 Built-in Unique ID ✅

| Item | File(s) | Status |
|------|---------|--------|
| Global unique ID generator | `crates/liora-core/src/lib.rs` | ✅ `next_unique_id()` + `unique_id(prefix)` + `stable_unique_id(...)`; direct allocation only at persistent construction, `stable_unique_id` in render paths |
| Component default IDs | `crates/liora-components/src/*.rs` | ✅ Runtime unique IDs replace `track_caller`/literal repeated interactive IDs in migrated components |
| Override APIs | Multiple components | ✅ `.id(...)` retained or added for migrated interactive components |


### memory-inventory-md-0009-e25ed89a8746

<!-- ctx-migration source=".memory/inventory.md" unit="memory-inventory-md-0009-e25ed89a8746" sha256="e25ed89a87469ec0c804f291892c6c5e602e370060b9b21dfb5d8bf7b393860a" -->

## P7 Demo Self-Contained 🔄

— See `.prompt/P7-demo-self-contained.md` for full task list.

| Item | File(s) | Status |
|------|---------|--------|
| Gallery registry ASC order | `apps/liora-gallery/src/demos/mod.rs` | ✅ Runtime sort + regression test |
| Button demo self-contained slice | `apps/liora-gallery/src/demos/button_demo.rs` | ✅ Uses Liora `Space`/`Title` instead of direct `div()`/`px()` layout primitives |
| Liora demo helpers | `crates/liora-components/src/space.rs`, `button.rs` | ✅ `Space::wrap` + semantic gaps; Button rounded helpers |


### memory-inventory-md-0010-74d8f04700bd

<!-- ctx-migration source=".memory/inventory.md" unit="memory-inventory-md-0010-74d8f04700bd" sha256="74d8f04700bd8939c776f01a8145d5ff61d0a3f28cade0065983691a73cdc7d3" -->

## P8 Native Gallery Documentation ✅ Core Done

— See `.prompt/P8-engineering.md` for the updated native documentation plan.

| Item | Target | Status |
|------|--------|--------|
| Typography bootstrapping | `crates/liora-components/src/` rich text/paragraph primitives | ✅ `Paragraph` now renders GPUI `StyledText` runs from `Text` segments |
| Markdown renderer | `apps/liora-docs/src/markdown.rs` + `pulldown-cmark` | ✅ Stack-based native renderer for headings, paragraphs, inline strong/em/code/strike, lists, blockquotes |
| Docs content pages | `apps/liora-docs/content/pages/*.md` | ✅ One Markdown file per page/component |
| Docs code snippets | `apps/liora-docs/content/snippets/<page>/*.rs` | ✅ External `.rs` snippets referenced by fenced code `src="..."` |
| Code block styling + document shell | Native Liora/GPUI two-column docs UI | ✅ Fenced code blocks + `Liora Docs` main window with `Container`/`Menu` shell |
| Live Demo injection | `::LioraDemo{component="..."}::` → real Liora view nodes | ✅ Button demo marker maps to a real Liora `Button` node |


### memory-inventory-md-0011-a86366f32583

<!-- ctx-migration source=".memory/inventory.md" unit="memory-inventory-md-0011-a86366f32583" sha256="a86366f32583140d843dd697d4ba08aff41120e05c3bb00799ac71da53530b2f" -->

## P9 Deferred Advanced ↗️ (migrated to P14)

| # | Component | File | Demo | Status |
|---|-----------|------|------|--------|
| 1 | Carousel | `crates/liora-components/src/carousel.rs` | ✅ | ✅ Migrated to P14 Wave 1 and implemented |
| 2 | Calendar | `crates/liora-components/src/calendar.rs` | ✅ | ✅ Migrated to P14 Wave 1 and implemented |
| 3 | TreeSelect | `crates/liora-components/src/tree_select.rs` | ✅ | ✅ Migrated to P14 Wave 3 and implemented |
| 4 | InputTag | `crates/liora-components/src/input_tag.rs` | ✅ | ✅ Migrated to P14 Wave 1 and implemented |
| 5 | Mention | `crates/liora-components/src/mention.rs` | ✅ | ✅ Migrated to P14 Wave 2 and implemented |
| 6 | Watermark | `crates/liora-components/src/watermark.rs` | ✅ | ✅ Migrated to P14 Wave 2 and implemented |
| 7 | Tour | `crates/liora-components/src/tour.rs` | ✅ | ✅ Migrated to P14 Wave 4 and implemented |
| 8 | VirtualizedTable | `crates/liora-components/src/virtualized_table.rs` | ✅ | ✅ Migrated to P14 Wave 5 and implemented |
| 9 | VirtualizedTree | `crates/liora-components/src/virtualized_tree.rs` | ✅ | ✅ Migrated to P14 Wave 6 and implemented |


### memory-inventory-md-0012-7caf901bd93a

<!-- ctx-migration source=".memory/inventory.md" unit="memory-inventory-md-0012-7caf901bd93a" sha256="7caf901bd93a63952909ecd4becb3649c6a794b1d6420338e2eb186569c84a1f" -->

## P10 Native Charts ✅ (7/7 — implemented, hover tooltips complete)

| # | Component | File | Demo | Docs | Status |
|---|-----------|------|------|------|--------|
| 1 | Chart infrastructure | `crates/liora-components/src/chart*.rs` | — | — | ✅ Scale/domain/shape/frame/legend foundation; native tooltip/hit-test helpers |
| 2 | LineChart | `crates/liora-components/src/line_chart.rs` | ✅ | ✅ | ✅ Multi-series, axis/grid, legend, point markers, empty state, downsampling, hover tooltip |
| 3 | AreaChart | `crates/liora-components/src/area_chart.rs` | ✅ | ✅ | ✅ Overlay/stacked area, axis/grid, legend, downsampling, overlay hover tooltip |
| 4 | BarChart | `crates/liora-components/src/bar_chart.rs` | ✅ | ✅ | ✅ Grouped/stacked vertical bars, axis/grid, legend, standalone mini/range color, hover tooltip |
| 5 | PieChart | `crates/liora-components/src/pie_chart.rs` | ✅ | ✅ | ✅ Implemented with value labels, outside labels, percentage/value patterns, polar hover tooltip |
| 6 | RingChart | `crates/liora-components/src/pie_chart.rs` | ✅ | ✅ | ✅ Implemented with donut mode, external legends, ring-segment hover tooltip excluding inner hole |
| 7 | Sparkline | `crates/liora-components/src/sparkline.rs` | ✅ | ✅ | ✅ Implemented: compact trend chart with trend colors, fill, baseline, line styles |


### memory-inventory-md-0013-8054f0049d1f

<!-- ctx-migration source=".memory/inventory.md" unit="memory-inventory-md-0013-8054f0049d1f" sha256="8054f0049d1fd507cbca45215f60d2b1110f6db9fdbde58cc3c97aaa042d3ae3" -->

## P11 Native Tray / Process Resident 🔄

| # | Capability | File | Demo | Docs | Status |
|---|------------|------|------|------|--------|
| 1 | Tray facade crate | `crates/liora-tray/src/lib.rs` | — | — | ✅ `TrayConfig`, `TrayMenuItemSpec`, `TrayCommand`, `LioraTray` |
| 2 | Dynamic icon API | `crates/liora-tray/src/lib.rs` | ✅ | ✅ | ✅ `set_icon`, `clear_icon`, `set_icon_from_rgba`, `set_icon_from_path` |
| 3 | CheckBox menu state | `crates/liora-tray/src/lib.rs` | ✅ | ✅ | ✅ Check menu item config + state sync |
| 4 | Recursive native menus | `crates/liora-tray/src/lib.rs` | ✅ | ✅ | ✅ Action, separator, 2nd/3rd/N-level submenu DSL |
| 5 | Gallery/docs examples | `apps/liora-gallery/src/demos/tray_demo.rs`, `apps/liora-docs/content/pages/tray.md` | ✅ | ✅ | ✅ Rich non-intrusive config preview + compile-checked snippets |



### memory-inventory-md-0014-60494c9c48bd

<!-- ctx-migration source=".memory/inventory.md" unit="memory-inventory-md-0014-60494c9c48bd" sha256="60494c9c48bd02919dedcca17c50d19bc45322caccba0406df32389f4328f404" -->

## P13 Component Expansion ✅ Implemented

| # | Component / Enhancement | File Target | Demo | Docs | Status |
|---|-------------------------|-------------|------|------|--------|
| 1 | QrCode | `crates/liora-components/src/qr_code.rs` | ✅ | ✅ | Implemented: QR generation display, color/size/ECC config, decode_bytes/decode_file/decode_image API |
| 2 | CodeEditor | `crates/liora-components/src/code_editor.rs` | ✅ | ✅ | Implemented: native editor, line numbers, selection/copy, indentation, highlighting, diagnostics provider |
| 3 | SignalMeter | `crates/liora-components/src/signal_meter.rs` | ✅ | ✅ | Implemented Wave 1 |
| 4 | HeatBar | `crates/liora-components/src/heat_bar.rs` | ✅ | ✅ | Implemented Wave 1; time-axis dense vertical-bar heat chart with legend/count summary |
| 5 | BarChart standalone mini mode | existing `bar_chart.rs` | ✅ | ✅ | Implemented in-place; no separate FlatBarMeter |
| 6 | SegmentRatioBar | `crates/liora-components/src/segment_ratio_bar.rs` | ✅ | ✅ | Implemented Wave 1; segmented bar plus top/bottom/both/hidden legend-value text |
| 7 | HorizontalList | `crates/liora-components/src/horizontal_list.rs` | ✅ | ✅ | Implemented: horizontal scrolling, custom item/divider rendering, internal drag reorder, on_reorder callback |
| 8 | Vertical list drag | existing `virtualized_list.rs` / list components | ✅ | ✅ | Implemented in-place on VirtualizedList: drag reorder, internal order, on_reorder callback |
| 9 | RingChart external labels | existing `ring_chart.rs` / chart modules | ✅ | ✅ | Implemented in-place: external vertical/horizontal legends, side placement, item limits, content/decimal options |
| 10 | LineChart per-series style | existing `line_chart.rs` | ✅ | ✅ | Implemented in-place: solid/dashed/dotted/custom dash, per-series color/width/smooth |
| 11 | BarChart range colors | existing `bar_chart.rs` | ✅ | ✅ | Implemented in-place: value_color_ranges with docs/gallery coverage |
| 12 | RingProgress gradient | existing `progress.rs` | ✅ | ✅ | Implemented in-place: ring gradient plus completion color |
| 13 | Timer | `crates/liora-components/src/timer.rs` | ✅ | ✅ | Implemented controlled count-up/count-down display with units and result snapshot API |
| 14 | Button gradient/custom colors | existing `button.rs` | ✅ | ✅ | Implemented in-place: custom solid/outline colors, gradient backgrounds, derived hover/active/disabled states |
| 15 | Tag flow layout | existing `tag.rs` | ✅ | ✅ | Implemented in-place via TagFlow layout helper |
| 16 | Label | `crates/liora-components/src/label.rs` | ✅ | ✅ | Implemented Wave 1 |
| 17 | Operation | `crates/liora-components/src/operation.rs` | ✅ | ✅ | Implemented Wave 1 |
| 18 | Radio/Checkbox option customization | existing `radio*.rs`, `checkbox*.rs` | ✅ | ✅ | Implemented in-place: option card/chip styling, selected/hover/border/text/padding/radius/indicator customization |


### memory-inventory-md-0015-3208a38bb950

<!-- ctx-migration source=".memory/inventory.md" unit="memory-inventory-md-0015-3208a38bb950" sha256="3208a38bb9500f9e4474969a1fee28040b4ced6ced973d572ac8baa5f5640caf" -->

## P14 Deferred Advanced ✅ Complete

| # | Component | File | Demo | Docs | Status |
|---|-----------|------|------|------|--------|
| 1 | Carousel | `crates/liora-components/src/carousel.rs` | ✅ | ✅ | Implemented Wave 1 |
| 2 | Calendar | `crates/liora-components/src/calendar.rs` | ✅ | ✅ | Implemented Wave 1 |
| 3 | InputTag | `crates/liora-components/src/input_tag.rs` | ✅ | ✅ | Implemented Wave 1 |
| 4 | TreeSelect | `crates/liora-components/src/tree_select.rs` | ✅ | ✅ | Implemented Wave 3 |
| 5 | Mention | `crates/liora-components/src/mention.rs` | ✅ | ✅ | Implemented Wave 2 |
| 6 | Watermark | `crates/liora-components/src/watermark.rs` | ✅ | ✅ | Implemented Wave 2 |
| 7 | Tour | `crates/liora-components/src/tour.rs` | ✅ | ✅ | Implemented Wave 4 |
| 8 | VirtualizedTable | `crates/liora-components/src/virtualized_table.rs` | ✅ | ✅ | Implemented Wave 5 |
| 9 | VirtualizedTree | `crates/liora-components/src/virtualized_tree.rs` | ✅ | ✅ | Implemented Wave 6 |


### memory-inventory-md-0016-3ff5e27b7ffb

<!-- ctx-migration source=".memory/inventory.md" unit="memory-inventory-md-0016-3ff5e27b7ffb" sha256="3ff5e27b7ffb7fbf33a01ad0dee147d3fd35ebd7f94f7aa411ba627781ee5d31" -->

## P22 gpui-component Harvest ✅ Complete

| # | Component / Enhancement | File | Demo | Docs | Status |
|---|-------------------------|------|------|------|--------|
| 1 | Spinner | `crates/liora-components/src/spinner.rs` | ✅ | ✅ | Implemented Wave A: standalone inline loading indicator with size/icon/color builders |
| 2 | Kbd | `crates/liora-components/src/kbd.rs` | ✅ | ✅ | Implemented Wave A: keyboard shortcut keycap display with size/color/background builders |
| 3 | OtpInput | `crates/liora-components/src/otp_input.rs` | ✅ | ✅ | Implemented Wave A: interactive OTP/PIN input backed by Liora Input editing, with dedicated Gallery/Docs pages and length/mask/status/size demos |
| 4 | DropdownButton | `crates/liora-components/src/dropdown_button.rs` | ✅ | ✅ | Implemented Wave A: split-capable dropdown command button with item icons, disabled/danger states, placement, close policy, sizes, and semantic variants |
| 5 | Accordion | `crates/liora-components/src/accordion.rs` | `apps/liora-gallery/src/demos/accordion_demo.rs` | `apps/liora-docs/content/pages/accordion.md` | Done Wave A |
| 6 | Combobox | `crates/liora-components/src/select.rs`, `crates/liora-components/src/autocomplete.rs`, `crates/liora-components/src/searchable_list.rs` | Existing Select/Autocomplete/SearchableList demos | Existing Select/Autocomplete/SearchableList docs | Closed as in-place coverage; no standalone Combobox |



### memory-inventory-md-0017-7b276a9a2cbc

<!-- ctx-migration source=".memory/inventory.md" unit="memory-inventory-md-0017-7b276a9a2cbc" sha256="7b276a9a2cbc38012824147dd9e0f2260f15b53480cbca18f4b8e1ec91e26333" -->

### P22 collection closure notes

The remaining `design/gpui-component-collection-list.md` candidates are closed as follows:

| Candidate | Closure |
|---|---|
| Sidebar / StatusBar / DockLayout / Settings / Sheet | Implemented as Liora app-shell components with Gallery/Docs coverage |
| DataTable enhancement | Folded into `VirtualizedTable` capabilities rather than a parallel DataTable component |
| TextView / Document View | Covered by `Text`, `SelectableText`, `CodeBlock`, and the native Docs markdown renderer |
| CandlestickChart | Implemented as native chart component |
| SearchableList | Implemented as shared filtering/list primitive and reused by searchable Select flows |
| Native menu | Covered by existing menu/dropdown/tray surfaces; no separate component backlog remains |
| FocusTrap | Folded into overlay interaction infrastructure rather than public standalone component |
| WindowExt / TitleBar / WindowBorder | Covered by `WindowFrame` / `TitleBar` surfaces |
| HoverCard / GroupBox / ScrollableMask | Implemented as Liora components |
| Toggle / ToggleGroup | Removed as duplicate standalone surface; use Switch for boolean settings and Segmented/button-style selections for toolbar/view-mode choices |
| Clipboard helper / Resizable panels / Root global-state patterns | Covered by existing CodeBlock/clipboard usage, `Splitter`, and `Config`/context infrastructure |
| WebView / WASM web gallery / browser runtime paths | Explicitly not collected due to pure Rust + GPUI native boundary |

### memory-state-md-0001-77b27be47e61

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0001-77b27be47e61" sha256="77b27be47e61b9f5fbc7685d8ec838fcc230d8d70618e39bdca94b0c76b84859" -->

# Liora Session State


### memory-state-md-0002-417e521cca79

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0002-417e521cca79" sha256="417e521cca79a339cb5093f409f225767cb7183f3fa9922e5c72319d9522c735" -->

## Current Phase

**P21 Release Candidate Readiness — Complete (2026-06-18)**

Local implementation phases are complete through P21. Current repository-owned status:

- P10 Native Charts: complete, including downsampling plus Line/Area/Bar/Pie/Ring hover hit testing.
- P11 Native Tray: complete, including `liora-tray`, dynamic icons, nested/check menus, Gallery/Docs controls, and close-to-tray behavior.
- P12 Native Packaging: repository-owned release readiness complete, including `release-readiness` gate, explicit `LicenseRef-Liora` installer/app policy, signing/notarization policy docs, CI dry-run gate, dedicated crates.io SDK publishing workflow using `CRATES_IO_TOKEN`, and strict `v*` app release gate. Docs ships as cross-platform raw executables; Gallery ships as raw executables plus planned native installer formats. Real credentials and destructive system-level installs remain protected-environment responsibilities.
- P13 Component Expansion: implemented and documented.
- P14 Deferred Advanced: complete; the P9 backlog has been migrated and delivered.
- P15 Quality Hardening: complete; Track A CI gates, Track B API consistency/panic cleanup, Track C visual/theme token hardening, Track D overlay/keyboard close-policy coverage, Track E CodeBlock/cache performance hardening, and Track F docs/snippet completeness all passed the final local gate suite.
- P16 Public API & Adoption Readiness: complete; root README, contributing/changelog docs, crate-level Rustdoc, Docs Adoption Guide, and adoption regression tests are in place; standalone minimal app was removed and folded into Gallery/Docs guidance.
- P17 Dashboard Dogfooding: complete; dashboard dogfooding has been folded into Gallery/Docs; standalone `examples/dashboard-app` was removed to avoid sample-app drift.
- P18 Dashboard Polish/API Ergonomics: complete; Gallery now owns shell dogfooding such as search/filtering, theme switching, refresh status, toasts, and tray behavior; dashboard/sample-specific helpers remain app-local and are not exported from `liora-components`.
- P19 Dashboard State/Data Flow: complete; state/data-flow guidance now lives in Docs and app-layer Gallery patterns; business sample models are not stored in `liora-components`.
- P20 Theme/Interaction Polish: complete; System/Light/Dark theme mode, tokenized overlays/masks, custom window frame polish, and Theme docs/demo are in place.
- P21 Release Candidate Readiness: complete; `docs/release-candidate-checklist.md`, explicit package metadata, updated README/CHANGELOG/prompt/memory state, and release-boundary regression tests lock the local `0.1.0` RC path.
- P22 gpui-component Harvest: complete; the full `design/gpui-component-collection-list.md` backlog is closed through standalone components, in-place enhancements, or explicit non-collection decisions.

P12/P21 external-policy items remain protected-environment work: crates.io SDK publication approval, real macOS notarization, Windows signing, destructive system-level installs, formal license replacement if any, and real `v*` release publication.



### memory-state-md-0003-cce5c817a551

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0003-cce5c817a551" sha256="cce5c817a5517f7074b6f6d7c35a0dfea1bd8f033b94bdf28eef9f801a0fc50e" -->

## P22 gpui-component Harvest — Complete (2026-06-29)

P22 is complete. The full `design/gpui-component-collection-list.md` backlog has been handled either by standalone Liora components, by in-place enhancement of existing controls, or by explicit non-collection decisions for boundary violations/duplicates. Standalone delivered surfaces include Spinner, Kbd, OtpInput, DropdownButton, Accordion, Sidebar, StatusBar, DockLayout, Settings, Sheet, HoverCard, GroupBox, ScrollableMask, CandlestickChart, and SearchableList. Combobox is intentionally not a standalone component: those workflows are covered by searchable Select/Autocomplete plus shared SearchableList. DataTable/TextView/WindowExt/resizable/code-editor style items are folded into existing VirtualizedTable/Text+Docs renderer/WindowFrame+TitleBar/Splitter/CodeEditor surfaces; Toggle-style workflows are covered by Switch/Segmented/Button patterns after removing the duplicate standalone Toggle surface. No P22 backlog remains.



### memory-state-md-0004-fd25ed6e18c2

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0004-fd25ed6e18c2" sha256="fd25ed6e18c2c4f6fa4a83fdf782540c340d548cce2d1d8695e59594dbc41db0" -->

## P19 Dashboard State/Data Flow — 2026-06-18

P19 is complete. Documented dashboard-style state/data-flow patterns in native Docs and folded shell-level search/filtering/refresh feedback into Gallery. Business mock models stay out of `liora-components`.



### memory-state-md-0005-251abdade453

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0005-251abdade453" sha256="251abdade45394b315119e13dcb8c439584fa16374b9c1d50aab00367c1ff7c1" -->

## P18 Dashboard Polish/API Ergonomics — 2026-06-18

P18 is complete. Moved theme switching/search/filtering dogfooding into Gallery, added native Docs `Dashboard Patterns`, and updated README/prompt/memory. Dashboard/sample-specific helpers stay in the app layer; `liora-components` exports reusable controls only.



### memory-state-md-0006-b38678844b43

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0006-b38678844b43" sha256="b38678844b43b313e2c32a59ef5a8058e15aa19a46868fea04c160666e142e53" -->

## P17 Dashboard Dogfooding — 2026-06-18

P17 is complete. Dashboard dogfooding is folded into Gallery and Docs instead of a standalone workspace package. Gallery validates shell search/filtering, theme switching, toasts, tray flow, and component composition while preserving pure Rust + GPUI native constraints.



### memory-state-md-0007-4d95016dbe4e

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0007-4d95016dbe4e" sha256="4d95016dbe4e5d6bfc92cbc10911e451d26186a8f8a79f1d7c193c42e0748eb7" -->

## P16 Adoption Readiness — 2026-06-18

P16 is complete. Added root `README.md`, `CONTRIBUTING.md`, `CHANGELOG.md`, crate-level Rustdoc entrypoints, native Docs `Adoption Guide`, and regression coverage for adoption docs/workflow wiring. The former minimal app guidance is folded into Gallery/Docs rather than a standalone workspace package. `cargo doc --workspace --no-deps` passes. Liora remains pure Rust + GPUI native; `LicenseRef-Liora` remains explicit until owner selects formal license terms.


### memory-state-md-0008-7f5c387da560

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0008-7f5c387da560" sha256="7f5c387da5609719f03714d3afb7c03a4843ebf2a955cc17fe321dcaf8a66212" -->

## P12 Final Closure — 2026-06-18

P12 native packaging is complete for repository-owned scope. Added `cargo run -p xtask -- package release-readiness`, `LICENSE.md` with explicit `LicenseRef-Liora`, `packaging/signing-policy.md`, CI non-strict readiness checks, package workflow strict `v*` app release readiness checks, and a dedicated SDK crates.io workflow using `CRATES_IO_TOKEN`. The app release path now blocks missing macOS/Windows signing inputs when `LIORA_REQUIRE_SIGNING=true` instead of silently publishing unsigned formal releases. Docs is raw-executable only; Gallery owns installer artifacts. Real signing credentials, notarization accounts, protected runner policy, destructive install/uninstall execution, SDK publish approval, and real public tag publishing remain owner-controlled release-environment actions, not missing local implementation.


### memory-state-md-0009-94b4afb2b40d

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0009-94b4afb2b40d" sha256="94b4afb2b40dd74d693b5a610ad71d36d94ff2175d2f0d43d151c949c94ffa00" -->

## P15 Final Completion Audit — 2026-06-18

P15 local quality hardening is complete. Final gate evidence:

- `cargo fmt --all --check` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `cargo check -p liora-docs --bin check_snippets` passed.
- `cargo run -p xtask -- package validate` passed.
- `cargo run -p xtask -- package ci --all-apps --format platform-defaults --dry-run --skip-build` passed.
- `cargo run -p xtask -- package install-smoke --all-apps --format platform-defaults --dry-run` passed.
- `git diff --check -- . ':(exclude).omx'` passed.
- Gallery and Docs GUI startup smoke both started and exited via expected `timeout 10s` status `124`.

Non-blocking residuals: `MessageManager::init` panic is intentional usage-contract enforcement; Gallery fixed date/time demo `expect(...)` calls are demo constant assumptions; P12 external-policy items remain signing/notarization, real install/uninstall, formal license policy, and real `v*` release validation.


### memory-state-md-0010-6522a4028c23

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0010-6522a4028c23" sha256="6522a4028c235aab18b00fc70552bb471e430e8a80670d57f344427c79d48211" -->

## Completed in P4

- ✅ 全部导航组件: Menu, Tabs, Breadcrumb, Steps, PageHeader, Affix, Backtop, Anchor
- ✅ 核心数据展示: Progress, Skeleton, Empty, Result, Descriptions, Timeline, Tree, Pagination, Statistic, Segmented, Tag, Avatar, Badge



### memory-state-md-0011-869076f6170d

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0011-869076f6170d" sha256="869076f6170d0df52a8451c1eec69bcbdcc4988ed7e44a68e98513a36186cf6d" -->

## Completed in P6

- ✅ `liora-core` added process-wide atomic unique ID helpers: `next_unique_id()` and `unique_id(prefix)`.
- ✅ Replaced `track_caller` / render-site / literal repeated interactive IDs in high-risk components with component-prefixed runtime unique IDs.
- ✅ Preserved/added `.id(...)` override APIs for migrated components where applicable.


### memory-state-md-0012-ba34fdd9ebfc

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0012-ba34fdd9ebfc" sha256="ba34fdd9ebfc04216da01cb12bf4270b6ce75928a44a7a7cdb23fe769596d334" -->

## Phase Progress

| Phase | Status | Completed | Total |
|-------|--------|-----------|-------|
| P0 Foundation | ✅ Done | 10/10 | 10 |
| P1 Basic | ✅ Done | 13/13 | 13 |
| P2 Form | ✅ Done | 10/10 | 10 |
| P3 Popper+Feedback | ✅ Done | 13/13 | 13 |
| P4 Nav+Data | ✅ Done | 21/21 | 21 |
| P5 Advanced | 🏁 Requested subset complete / remaining deferred | 11/20 | 20 |
| P6 Built-in Unique ID | ✅ Done | 1/1 | 1 |
| P7 Demo Self-Contained | ✅ Done | 1/1 | 1 |
| P8 Native Docs App | ✅ Core Done | 4/4 | 4 |
| P9 Deferred Advanced | ✅ Migrated to P14 | 9/9 | 9 |
| P10 Native Charts | ✅ Done | 7/7 | 7 |
| P11 Native Tray | ✅ Done | 1/1 | 1 |
| P12 Native Packaging | ✅ Done | release readiness gate done | external credentials/protected runners gated |
| P13 Component Expansion | ✅ Done | 18/18 | 18 |
| P14 Deferred Advanced | ✅ Done | 9/9 | 9 |
| P15 Quality Hardening | ✅ Done | final gate passed | CI gates + API consistency + visual/theme + overlay behavior + CodeBlock performance + docs completeness |
| P16 Adoption Readiness | ✅ Done | adoption gate passed | README + Rustdoc + Docs Adoption Guide; standalone minimal app removed |
| P17 Dashboard Dogfooding | ✅ Done | dogfood gate passed | folded into Gallery/Docs; standalone dashboard app removed |
| P18 Dashboard Polish/API Ergonomics | ✅ Done | ergonomics gate passed | Gallery shell polish + theme toggle + Dashboard Patterns docs; no dashboard sample exports |
| P19 Dashboard State/Data Flow | ✅ Done | state/data gate passed | app-layer state/filter/refresh guidance in Gallery/Docs |
| P20 Theme/Interaction Polish | ✅ Done | theme/interaction gate passed | System/Light/Dark + tokenized overlays/masks |
| P21 Release Candidate Readiness | ✅ Done | RC gate passed | release checklist + package metadata + protected release boundaries |
| P22 gpui-component Harvest | ✅ Done | collection backlog closed | full gpui-component collection list handled; no P22 backlog remains |


### memory-state-md-0013-b4cbab269f4e

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0013-b4cbab269f4e" sha256="b4cbab269f4ed127690bd893aac9ff6376fdb296d1a8b0e7e2c359ec72dee2ed" -->

## Git Status

- Branch: main
- Remote: git@github.com:yhyzgn/liora.git



### memory-state-md-0014-e1e7793f9e67

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0014-e1e7793f9e67" sha256="e1e7793f9e67c27fc86e68d2849581326e3b74233f44006219403bb4ff238bc2" -->

## Deferred Backlog

- P9 Deferred Advanced was migrated into P14 and is complete.
- Delivered components: Carousel, Calendar, TreeSelect, InputTag, Mention, Watermark, Tour, VirtualizedTable, VirtualizedTree.
- Do not reopen these as deferred backlog unless a new user request changes their requirements.


### memory-state-md-0015-a549b269bdd8

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0015-a549b269bdd8" sha256="a549b269bdd8320217c2ba3571013de3f0aad4babaa4217ba6a999d7afb1fd3e" -->

## Historical Phase — P10 Native Charts

P10 native statistics/chart components are complete. P9 deferred backlog was later migrated and completed by P14.

Initial technical direction:
- Primary reference: local/current GPUI official source, especially `canvas(...)`, `PathBuilder`, `Window::paint_path`, `Window::paint_quad`, and text rendering primitives.
- Secondary case study: `https://github.com/vicanso/zedis` Metrics implementation, which uses GPUI canvas and a scale/axis/shape split for Area/Line/Bar charts.
- Strict native boundary remains: no HTML/CSS/DOM/WebView/WASM/Web chart runtime.

Expected P10 deliverables:
- Shared chart infrastructure: scale, axis/grid, shapes, legend, tooltip/hover.
- Completed so far: LineChart, AreaChart, BarChart MVPs with Gallery demos, Docs pages, external snippets, and tests.
- Dense chart performance follow-up (2026-06-16): LineChart/AreaChart now avoid full-label point scales and cap axis/value labels by default. The later correction moved sampling into shared core helpers (`downsample_index_range`, `downsample_indexed_values`) so LineChart/AreaChart/Sparkline no longer allocate full dense intermediate point vectors before sampling. Public knobs remain `max_render_points(...)`, `max_axis_labels(...)`, `max_value_labels(...)`, `disable_downsampling()`.
- Final P10 status: LineChart, AreaChart, BarChart, PieChart, RingChart, Sparkline, downsampling, and Line/Area/Bar/Pie/Ring hover hit testing are complete. Future cache work requires fresh profiling evidence.


### memory-state-md-0016-2c33b63c3378

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0016-2c33b63c3378" sha256="2c33b63c33784741a4f7e6795892ec3074bd60135adb247474535e2160099686" -->

## Historical Phase — P11 Native Tray / Process Resident

P11 native system tray / process resident support is complete for GPUI apps.

Technical direction:
- New crate: `crates/liora-tray`.
- Dependencies: `tray-icon` plus `muda` via `tray_icon::menu` re-export; no vendored source by default.
- Required APIs: install from `TrayConfig`, dynamic icon updates, tooltip/visibility updates, checkbox menu state, recursive submenus, stable `TrayCommand` mapping.
- GPUI integration rule: tray-enabled apps must use `QuitMode::Explicit` and keep `LioraTray` alive for process lifetime.
- Demo/docs rule: Gallery and Docs must show rich tray examples (CheckBox, dynamic icons, 2nd/3rd/N-level menus) without creating real OS tray side effects during normal browsing.


### memory-state-md-0017-9d5ac65d8eb7

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0017-9d5ac65d8eb7" sha256="9d5ac65d8eb7554b5a736f33d704d4724404fda53a2c46ac85ecabb7431ca577" -->

### P11 follow-up: real Gallery tray runtime

After user feedback, `liora-gallery` no longer only previews tray config. On native startup it installs a real OS tray icon, stores `LioraTray` in GPUI global state, routes `MenuEvent`/tray click events through a foreground command loop, and handles show/hide/toggle/quit/set-icon/auto-show commands. If tray installation fails, Gallery falls back to `QuitMode::LastWindowClosed` to avoid a resident process without a tray entry.


### memory-state-md-0018-f7443cb1d69e

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0018-f7443cb1d69e" sha256="f7443cb1d69e120f9aec272230b9ce57acf8b371d302c2b45aad7cfc2b262343" -->

### P11 follow-up: Gallery and Docs tray demos

Both `liora-gallery` and `liora-docs` now create independent demonstration tray icons on native startup. Gallery uses `liora-gallery`/blue default icon; Docs uses `liora-docs`/purple default icon. Tray menu includes a `resident-enabled` CheckBox for status-bar residency, and app handlers toggle `QuitMode::Explicit` versus `QuitMode::LastWindowClosed` plus tray visibility. Tray docs now include a compile-checked `tray/residency.rs` snippet for page-level residency configuration.


### memory-state-md-0019-5a7d4ae9ddd3

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0019-5a7d4ae9ddd3" sha256="5a7d4ae9ddd3c62c7845982807c7dbcbb8b634884cc947d926759025f59dc07e" -->

### P11 follow-up: bundled tray icons and in-window controls

`liora-tray` now includes bundled PNG tray icon assets under `crates/liora-tray/assets/tray-icons/` for Gallery and Docs default/syncing/error states. Apps use `bundled_tray_icon(...)` rather than generated solid-color placeholders. `TrayControlCenter` is a GPUI global command bridge so the Tray page buttons in the main window dispatch real tray commands, not just local previews.


### memory-state-md-0020-e5cd20d96e5c

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0020-e5cd20d96e5c" sha256="e5cd20d96e5cdd26413e959c71cc3bc868697e752b38d649f8a80012be198e63" -->

### P11 follow-up: close confirmation with remembered choice

Gallery and Docs now intercept window close through GPUI `on_window_should_close`. If `TrayControlCenter.state.remembered_close_action` is `Ask`, a native Dialog asks whether to `关闭进程` or `隐藏到托盘`, with a `记住本次选择` checkbox. Remembered choices are stored in runtime tray control state as `TrayCloseAction::{ExitProcess, HideToTray}`; the Tray demo page can reset to Ask or preselect either behavior.



### memory-state-md-0021-5307eee4fe31

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0021-5307eee4fe31" sha256="5307eee4fe3132859b13a1af231542ef7b7c3f45c7087d9319bf18c9863cc2ca" -->

## Historical Phase — P13 Component Expansion

User requested a new planning phase for additional widgets and customization enhancements. P13 is implemented; `.prompt/P13-component-expansion.md` is now the maintenance contract for those components.

Scope highlights:
- New widgets: QrCode generation/recognition, CodeEditor, SignalMeter, HeatBar, SegmentRatioBar, HorizontalList, Timer, Label, Operation. The user-provided “standalone bar chart” screenshot is interpreted as an in-place BarChart standalone mini mode, not a new FlatBarMeter component.
- Enhancements: RingChart external labels, LineChart per-series stroke style, BarChart standalone mini mode and value range colors, RingProgress gradient/completion color, Button gradient/custom color derived states, Tag flow layout, Radio/Checkbox option customization, vertical list drag.
- Execution is split into five waves: simple meters/bars/layout, chart/progress enhancements, draggable lists, QR/CodeEditor, and form-control deep customization.
- Existing-widget enhancement rule: existing controls must be enhanced in-place in their current source/demo/docs; do not add parallel replacement components for Tag flow, RingProgress gradients, chart style options and BarChart standalone mini mode, Button custom colors, or Radio/Checkbox option customization.

P13 screenshot clarifications:
- HeatBar means a time-axis dense vertical-bar heat chart with top legend/count summary, not a calendar grid heatmap.
- SegmentRatioBar means one horizontal segmented ratio bar with configurable legend/value text placement: top, bottom, both, or hidden; segment labels and percent/value patterns are customizable.


### memory-state-md-0022-42c782bc19ea

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0022-42c782bc19ea" sha256="42c782bc19ea5f90a658c3b552be346f599e040a99cb8a382cbf2508c565c254" -->

### P13 Wave 1 implementation progress — 2026-05-18

Wave 1 has started and the first simple/native components are implemented:
- Added `SignalMeter` for mobile/Wi-Fi signal style bars with level, max level, colors, bar width, gap, and height configuration.
- Added `HeatBar` as the user-requested time-axis dense vertical-bar heat chart with optional legends/count summary, axis/grid, max value, bar width/gap, and x labels.
- Added `SegmentRatioBar` with segment color/value configuration, top/bottom/both/hidden legend placement, split legend layout, decimal control, and label/value pattern support.
- Added `Label` (Icon + Text with gap/color/size) and `Operation` (left label + right action, two-end aligned) components.
- Enhanced existing `BarChart` in-place with standalone mini mode, rounded bars, explicit bar width/gap, and value range colors; did not add a separate flat bar component.
- Enhanced existing `Tag` in-place with `TagFlow` layout helper for wrapping tag groups.
- Gallery demos and Docs pages/snippets were added for these Wave 1 pieces; BarChart and Tag existing docs now include the new in-place enhancement examples.

Validation evidence:
- `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets` passed.
- `cargo test -p liora-components` passed: 117 lib tests + integration tests all green.

Final P13 status is implemented; see `.memory/inventory.md` for the completed component matrix and `.prompt/P13-component-expansion.md` for the maintenance contract.


### memory-state-md-0023-9ad5d785eeba

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0023-9ad5d785eeba" sha256="9ad5d785eebaef133b7ec95a5be85cc35640b256497352a946366be91fbb9ab2" -->

### P13 Wave 2 partial progress — 2026-05-18

Implemented two high-priority in-place enhancements:
- LineChart/ChartSeries now supports per-series line style: `ChartLineStyle::{Solid, Dashed, Dotted}`, `.dashed()`, `.dotted()`, `.solid()`, and custom `.dash_pattern([...])`, while preserving per-series color, stroke width, and smooth toggles. Rendering uses GPUI `PathBuilder::dash_array` through shared chart shape helpers.
- Progress circle/ring now supports gradient rings and `.complete_color(...)`; completed gradient rings can resolve to a specified final color. Gallery/docs/snippets include the ring gradient completion example.

Docs and Gallery were updated:
- `LineChart` page now has a per-line style section with checked snippet `line_chart/line_styles.rs`.
- `Progress` page now has a ring gradient/completion-color section with checked snippet `progress/circle_gradient.rs`.

Validation evidence:
- `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets` passed.
- `cargo test -p liora-components` passed.


### memory-state-md-0024-7424594ea13b

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0024-7424594ea13b" sha256="7424594ea13b7b4cf78c85b0163ccd1ad5a2fb9f50c061010c32bc77cb973bff" -->

### P13 SegmentRatioBar correction — 2026-05-18

User clarified SegmentRatioBar legend/text must be horizontally arranged, not a vertical list. Updated `segment_ratio_bar.rs` so `render_segment_legend` uses a horizontal wrapping flex row (`flex_row` + `flex_wrap` + wider gaps). `split_legend(true)` now splits label/value within each horizontal legend item via `min_w`, instead of stretching each item to a full row. Gallery/docs wording updated to describe horizontal legend text.

Validation evidence:
- `cargo test -p liora-components segment_ratio_bar` passed.
- `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets` passed.


### memory-state-md-0025-4558165cae2c

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0025-4558165cae2c" sha256="4558165cae2c2fc98ca8375beb365c61ccc3cf93df13fd1d7493a8b56d5466e4" -->

### P13 SegmentRatioBar split legend refinement — 2026-05-18

User clarified each SegmentRatioBar segment text item should split alignment internally: left side is color legend dot + label, right side is ratio/value text (still pattern-customizable). Updated `SegmentRatioBar` so `split_legend(true)` is the default and each horizontal legend item uses a configurable `legend_item_width`, `justify_between`, left legend+label, and right-aligned value/pattern text. Added `legend_item_width(...)` builder for custom per-item width.

Validation evidence:
- `cargo test -p liora-components segment_ratio_bar` passed.
- `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets` passed.


### memory-state-md-0026-d7149d451972

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0026-d7149d451972" sha256="d7149d45197259c807202a2ef6c2ffac66da882d2196ac5c73cd63558d10ad9c" -->

### P13 SegmentRatioBar segment-aligned text correction — 2026-05-18

User clarified the intended layout: for each individual ratio segment, the text block must have the same width and horizontal start/end as that segment. The left legend dot + label aligns to the segment's left edge, and the right value/percent aligns to the segment's right edge. Reworked `render_segment_legend` accordingly: it now renders a full-width horizontal row where each legend text cell uses `gpui::relative(item.value / total)` just like the colored bar segment. Removed fixed `legend_item_width` behavior because it could not align to variable segment boundaries. Pattern customization remains on label/value text.

Validation evidence:
- `cargo test -p liora-components segment_ratio_bar` passed.
- `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets` passed.


### memory-state-md-0027-3108777cd0dd

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0027-3108777cd0dd" sha256="3108777cd0ddd84f5299bb4134d16610d5b270ebebd178c59bf553bbad5f1880" -->

### P13 SegmentRatioBar text inset — 2026-05-18

Added configurable left/right text inset for SegmentRatioBar's segment-aligned legend cells. New builders: `legend_inset_x(Pixels)` and alias `legend_text_inset(Pixels)`. The inset applies inside each proportional segment text cell, preserving alignment to the segment boundaries while avoiding text touching segment edges. Gallery and docs snippets now demonstrate non-default inset values.

Validation evidence:
- `cargo test -p liora-components segment_ratio_bar` passed.
- `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets` passed.


### memory-state-md-0028-9bb2e8d1020b

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0028-9bb2e8d1020b" sha256="9bb2e8d1020b7c7122cfde077c3086bbb78669a1c385eba2eebde1697a66cf9d" -->

### P13 SegmentRatioBar radius controls — 2026-05-18

Added separate radius controls for SegmentRatioBar: existing `radius(...)` configures the overall bar container radius, and new `segment_radius(...)` / alias `rounded_segments(...)` configures each colored segment's own radius. This supports both whole-bar rounding and per-segment rounding while preserving segment-aligned text cells and text inset behavior. Gallery and docs snippets now demonstrate both levels of rounding.

Validation evidence:
- `cargo test -p liora-components segment_ratio_bar` passed.
- `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets` passed.


### memory-state-md-0029-b1abae010b56

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0029-b1abae010b56" sha256="b1abae010b56a5d7022784b55c03b877cfc0be0fd499bb32d8fdc2e4881b3290" -->

### P13 SignalMeter total/count and per-level colors — 2026-05-18

Enhanced `SignalMeter` with explicit total signal count aliases and per-level active colors. Existing `max_level(...)` remains; new `total_signals(...)` and `signal_count(...)` aliases configure total bars. New `level_colors(...)` / `signal_colors(...)` lets callers assign different active colors for each signal level; inactive bars still use `inactive_color(...)`. Gallery and docs now include total-count/per-level-color examples.

Validation evidence:
- `cargo test -p liora-components signal_meter` passed.
- `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets` passed.


### memory-state-md-0030-cb3c208bfcba

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0030-cb3c208bfcba" sha256="cb3c208bfcba369ae9a71979cb7678c3ba8ec6d1e9cd61246eaacb03c9870254" -->

### P13 docs/demo coverage standard — 2026-05-18

User clarified that every new component and future new component must have Gallery and Docs examples covering the major style/configuration combinations, not just one happy-path example. Applied immediately to SegmentRatioBar: Gallery and Docs now cover bottom legend, top legend, both top+bottom legends, hidden legend, custom label/value pattern, compact thin bar, overall radius, per-segment radius, text inset, split legend, and percentage precision.

Validation evidence:
- `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets` passed.
- `cargo test -p liora-components segment_ratio_bar` passed.


### memory-state-md-0031-08afd4b21d01

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0031-08afd4b21d01" sha256="08afd4b21d0190b57dad2bf06ac2a9f652210f7dd9e12299bb2ecd8173a25a66" -->

### P13 SignalMeter threshold-wide colors — 2026-05-18

User clarified that per-level colors also need a threshold-wide mode: when the current signal reaches a configured level, all active bars use one unified color for that current level (e.g. level 2 = red, 3 = yellow, 4 = orange, 5 = green). Kept the existing per-bar `level_colors(...)` / `signal_colors(...)` behavior and added `SignalLevelColor`, `threshold_colors(...)`, `level_threshold_colors(...)`, and incremental `level_color(level, color)`. Rendering prioritizes threshold-wide color over per-bar level colors when a matching threshold exists. Gallery and Docs now include threshold-wide examples.

Validation evidence:
- `cargo test -p liora-components signal_meter` passed.
- `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets` passed.


### memory-state-md-0032-98fbdd5c350f

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0032-98fbdd5c350f" sha256="98fbdd5c350f93f1197a8925b7ef53f4979b922d20baa1f259d39dac3103b0d0" -->

### P13 Timer component — 2026-05-18

Implemented new `Timer` component in `crates/liora-components/src/timer.rs`. It is a controlled display component for count-up/count-down timers, with `TimerDirection`, `TimerUnit`, `TimerSnapshot`, `count_up`, `count_down`, `display_unit`, `show_unit`, `prefix`, `suffix`, `compact`, `snapshot`, `elapsed_as`, and `remaining_as`. Countdown remaining time saturates at zero and exposes `finished`. Gallery and Docs now include count-up, count-down, unit/compact, and result-reading examples. This follows the new docs/demo coverage standard for newly added components.

Validation evidence:
- `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets` passed.
- `cargo test -p liora-components timer` passed.
- `cargo test -p liora-gallery timer_demo_uses_timer_api` passed.


### memory-state-md-0033-1a93990f7d7c

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0033-1a93990f7d7c" sha256="1a93990f7d7ca169e3da06145d18332748df2c59708b5cfb207933f8ad8d1367" -->

### P13 Timer clock format — 2026-05-18

Enhanced Timer with clock-style formatting for `00:00:00` / `HH:MM:SS`. Added `TimerFormat::{Unit, Clock}`, `Timer::format(TimerFormat)`, `Timer::clock_format()`, and public `format_clock(Duration)`. Gallery and Docs now include a clock-format section and checked snippet `timer/clock.rs`.

Validation evidence:
- `cargo test -p liora-components timer` passed.
- `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets` passed.
- `cargo test -p liora-gallery timer_demo_uses_timer_api` passed.


### memory-state-md-0034-d0394047f054

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0034-d0394047f054" sha256="d0394047f0545e3f0eb4f0ce9ae820de1e4844b2059b9c3628a1733be3d0823a" -->

### P13 Button gradient/custom color enhancement — 2026-05-18

Enhanced existing `Button` in-place with custom color and gradient styling:
- Added `ButtonColors` for fully custom solid/outline button colors, including explicit base/hover/active/text/border/disabled slots.
- Added `.custom_color(bg, text)`, `.colors(ButtonColors)`, and `.custom_colors(ButtonColors)` builders.
- Added `ButtonGradient` plus `.gradient(from, to)` and `.gradient_with_angle(angle, from, to)` builders.
- Hover, active/clicked, and disabled states are automatically derived for simple custom colors and gradients, while preserving the existing theme variants by default.
- Gallery `Button` demo and Docs `button.md` now show custom solid/outline/disabled and gradient/loading/disabled examples with compile-checked snippets.

Validation evidence:
- `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets` passed.
- `cargo test -p liora-components button` passed.
- `cargo test -p liora-gallery button_demo_uses_liora_layout_primitives` passed.


### memory-state-md-0035-163c98936093

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0035-163c98936093" sha256="163c989360933432b46f964c9185e7478466a004429b2e538af882fee528a583" -->

### P13 Radio/Checkbox option style customization — 2026-05-18

Enhanced existing `CheckboxGroup` and `RadioGroup` in-place with option-level layout and selected-style customization:
- Added `CheckboxOptionStyle` and `RadioOptionStyle` builders for option background, selected background, hover background, text/selected text colors, border/selected border colors, radius, padding, gap, indicator visibility, and selected icon/dot visibility.
- Added `.option_style(...)` and `.card_options()` to both group components.
- Non-button vertical/horizontal groups now render styled option cards/chips when option style is configured; default rendering remains unchanged.
- Button-style groups also honor selected/background/text/border/gap/padding/icon options where applicable.
- Gallery Form Controls demo and Docs `checkbox.md` / `radio.md` now include card-like and chip-like custom option examples with compile-checked snippets.

Validation evidence:
- `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets` passed.
- `cargo test -p liora-components option_style` passed.
- `cargo test -p liora-gallery form_controls` completed with no failures.



### memory-state-md-0036-53eb23f89250

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0036-53eb23f89250" sha256="53eb23f8925089b96f67ca144c98dabc655292d2d8c30c8e590b7553df0cc721" -->

### P13 QrCode generation and recognition — 2026-05-18

Added new `QrCode` component in `crates/liora-components/src/qr_code.rs` using pure Rust dependencies (`qrcode` for generation and `rqrr` for recognition). Capabilities:
- Native GPUI-rendered QR display via generated `RenderImage`, with configurable size, quiet zone, foreground/background colors, and error-correction level (`QrEcLevel`).
- Public generation helpers: `encode_matrix(...)` and `render_image(...)`.
- Recognition helpers: `decode_bytes(...)`, `decode_file(...)`, and `decode_image(...)`, returning `QrDecoded { content, ecc_level, version }`.
- Gallery demo added as `QrCode 二维码`; Docs page `qr_code.md` added with basic, style/ECC, and recognition API snippets.

Validation evidence:
- `cargo test -p liora-components qr_` passed, including a generated-image decode round trip.
- `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets` passed.


### memory-state-md-0037-208d12874ed5

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0037-208d12874ed5" sha256="208d12874ed5d9078cd42416997dd2d35c412e8bf573e3f39e42445a2d9a0e74" -->

### P13 QrCode interactive demo refinement — 2026-05-18

Updated QrCode demos/docs to meet the interaction requirement:
- Gallery QrCode demo now includes an input field and `生成二维码` button; clicking updates the displayed QR code from the current string.
- Gallery QrCode demo now includes a local image path input and `识别图片` button; clicking calls `QrCode::decode_file(...)` and displays success/failure text plus toast feedback.
- Docs QrCode page now uses the full interactive Gallery demo for the effect area, and snippets show complete interactive generation and local-file recognition patterns.

Validation evidence:
- `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets` passed.
- `cargo test -p liora-components qr_` passed.


### memory-state-md-0038-30effdb53060

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0038-30effdb53060" sha256="30effdb53060d2542d9b05abf2b9950af3d81be6d4cc7bc1001fd53d17144afc" -->

## 2026-05-18 P13 QrCode style/upload refinement
- QrCode generation now supports screenshot-like styles: square/rounded/dot modules, square/rounded/circle finder styles, high-recovery center logo badge, corner mini badge, custom foreground/background/logo colors, and logo size ratio.
- QrCode recognition demos/docs now use Liora Upload to open local image files instead of typing paths; selected file is decoded with QrCode::decode_file and result is shown in the page plus toast feedback.
- Validation: cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets; cargo test -p liora-components qr_.

Update: QrCode also gained generic logo(...) and corner_logo(...) builders accepting any GPUI element, in addition to logo_text/corner_logo_text convenience APIs, so callers can render images/icons/custom badges in QR overlays. Validation rerun after this API: cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets; cargo test -p liora-components qr_.


### memory-state-md-0039-f29539641e70

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0039-f29539641e70" sha256="f29539641e70cfd36c45af122190df1a6c745499391d54d03a224b978b24758d" -->

## 2026-05-18 P13 QrCode recognition/result and social styles refinement
- QrCode recognition examples now show the decode result persistently in an on-page result box; toast remains only supplemental feedback.
- Corrected social QR styling direction by adding `QrPatternStyle::{Matrix, MiniProgram, Douyin}` with radial rendering for mini-program-like and Douyin-like codes instead of rendering them as ordinary dot-matrix QR only.
- Added builders: `pattern_style(...)`, `matrix_style()`, `mini_program_style()`, `douyin_style()`, and `douyin_badge()`; `mini_program_badge()` now uses the radial mini-program preset.
- Gallery and Docs style demos now show normal QR, mini-program style, Douyin style, and custom-logo rounded QR.
- Validation: `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets`; `cargo test -p liora-components qr_`.


### memory-state-md-0040-0ca17b4f76c1

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0040-0ca17b4f76c1" sha256="0ca17b4f76c1020f1c4c654428de0b7d419961007bfed0699899080d4d0da242" -->

## 2026-05-18 P13 QrCode social style rewrite
- Rewrote MiniProgram/Douyin QR style rendering after screenshot feedback showed the previous polar matrix transform looked like noisy broken QR fragments.
- New social style renderer samples encoded QR content but renders clean radial capsules/dots with deterministic thinning, skips QR finder squares, and draws explicit social-code locator dots plus Douyin-style outer arcs.
- Validation: `cargo test -p liora-components qr_`; `cargo check -p liora-gallery -p liora-docs --bin check_snippets`.


### memory-state-md-0041-cb9f467cc131

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0041-cb9f467cc131" sha256="cb9f467cc13129f1825d8e5dec4bec9ed3c991de28a763ad7ac03c065ec01b74" -->

## 2026-05-18 P13 QrCode social presets second rewrite
- User clarified the previous social-code output still did not resemble the reference images. Replaced content-matrix polar module plotting with visual-template renderers: MiniProgram now uses sunburst radial capsules/dots plus three locator circles; Douyin now uses segmented circular tracks, sparse radial texture, three locator circles, and bold outer arcs.
- The render remains deterministic per encoded content via a visual seed, but intentionally prioritizes the requested social-code style instead of QR-matrix readability.
- Validation: `cargo test -p liora-components qr_`; `cargo check -p liora-gallery -p liora-docs --bin check_snippets`.


### memory-state-md-0042-836935c8bc74

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0042-836935c8bc74" sha256="836935c8bc74409ae90a740120363c9c2274a6fd8fb5f5f26cfaa19ad99a24fa" -->

## 2026-05-18 P13 QrCode social presets removed, gradient foreground added
- Removed failed MiniProgram/Douyin social-code style APIs and render branches per user request; no `QrPatternStyle`, `mini_program_*`, or `douyin_*` API remains.
- Added QR foreground gradient support with color arrays and eight directions via `QrGradientDirection::{ToTop, ToTopRight, ToRight, ToBottomRight, ToBottom, ToBottomLeft, ToLeft, ToTopLeft}`.
- New builders: `gradient(colors, direction)`, `foreground_gradient(colors, direction)`, `gradient_colors(colors)`, and `gradient_direction(direction)`. Calling `foreground(...)` clears gradient and restores solid color behavior.
- Gallery and Docs QrCode style examples now show gradient QR variants instead of removed social-code presets.
- Validation: `cargo test -p liora-components qr_`; `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets`.


### memory-state-md-0043-2b9e1fa8c53d

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0043-2b9e1fa8c53d" sha256="2b9e1fa8c53d1dcf7b9a2db94d724adb1748bd16e013dacf5b5d33119f45f17f" -->

## 2026-05-18 P13 RingChart external legend enhancement
- Enhanced existing `RingChart` in-place with fully external legend/value display: `RingExternalLegendOptions`, `RingExternalLegendLayout::{Vertical, Horizontal}`, `external_legend(...)`, `external_vertical_legend()`, `external_horizontal_legend()`, `external_legend_content(...)`, and `external_legend_percentage_decimals(...)`.
- External legend mode disables inline chart labels and normal legend, avoiding leader lines and putting all label/value/percentage text into a vertical or horizontal legend area.
- Gallery and Docs now include external legend examples; docs snippet `ring_chart/external.rs` is compile-checked.
- Validation: `cargo test -p liora-components ring_chart`; `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets`.


### memory-state-md-0044-dfeccbddce8f

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0044-dfeccbddce8f" sha256="dfeccbddce8f4f01075053dbc7f8babca87748fad581cee1a6b6b0dac471631c" -->

## 2026-05-18 P13 RingChart external vertical side and item limit
- Enhanced RingChart external legend mode so vertical legends are rendered beside the chart instead of below it. Added `RingExternalLegendSide::{Left, Right}`, `external_legend_side(...)`, `external_legend_left()`, and `external_legend_right()`.
- Added `max_items(...)` on `RingExternalLegendOptions` and `external_legend_max_items(...)` on `RingChart` to show only the first N non-zero slices.
- Gallery and Docs now demonstrate a right-side vertical external legend limited to the first 3 items, plus horizontal external legend coverage.
- Validation: `cargo test -p liora-components ring_chart`; `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets`.


### memory-state-md-0045-8b08ebb4fdd7

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0045-8b08ebb4fdd7" sha256="8b08ebb4fdd78ed69f74a768ddedbbdc1d686c61bebc4a0de2c96b2270a068da" -->

## 2026-05-18 P13 RingChart external vertical layout fix
- Fixed vertical external legend layout regression where the legend consumed full row width and hid/squeezed the chart. Vertical legend now has fixed side width and `flex_none`, while the chart container uses `flex_1().min_w(0)`.
- Validation: `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets`; `cargo test -p liora-components ring_chart`.


### memory-state-md-0046-6d8db300733f

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0046-6d8db300733f" sha256="6d8db300733f79fef5b469aac4378a3ff50b9b881f7db6fe13cea667e6c69ffd" -->

## 2026-05-18 P13 RingChart side legend spacing tightening
- Tightened RingChart vertical external legend placement so text sits next to the chart instead of far away: reduced side-layout gap, narrowed vertical legend width, and slightly reduced side-layout canvas height to remove excessive empty horizontal/vertical space.
- Validation: `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets`; `cargo test -p liora-components ring_chart`.


### memory-state-md-0047-6d5a68cb5d4f

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0047-6d5a68cb5d4f" sha256="6d5a68cb5d4febd9fe5a14730fecc2eba6326d1e10673a68ce00b8f61aea2eba" -->

## 2026-05-18 P13 Timer live ticking

Enhanced `Timer` from a static controlled display into an optional live ticking component while preserving the controlled API. `Timer::start()` / `.running(true)` now registers a native GPUI refresh runtime, uses stable `id(...)` values to preserve each timer's start instant across renders, and supports count-up, count-down, and `00:00:00` clock displays that continue updating in Gallery and Docs. Docs live demos and checked snippets now use `.start()` for interactive timer examples.



### memory-state-md-0048-d289dff56fd3

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0048-d289dff56fd3" sha256="d289dff56fd339e436876c6edae3835c37e159bb958099c1c802cd63c90a585f" -->

## 2026-05-18 P13 HorizontalList component

Implemented `HorizontalList` in `crates/liora-components/src/horizontal_list.rs` as a native horizontal scroll list with custom item renderer, custom divider renderer, internal order state, drag-to-reorder interaction, and `on_reorder(from_index, to_index, ...)` callback. Added Gallery demo sections for base horizontal cards, custom arrow divider, and draggable reorder with toast feedback. Added Docs page `horizontal_list.md` and compile-checked snippets for basic/divider/draggable usage.

Validation evidence:
- `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets` passed.
- `cargo test -p liora-components horizontal_list` passed.
- `cargo test -p liora-gallery horizontal_list_demo` passed.



### memory-state-md-0049-0eb77dca8344

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0049-0eb77dca8344" sha256="0eb77dca83446d0b71050b445c028e82493a4c7be0c7d51207ec15368f01d767" -->

## 2026-05-18 P13 VirtualizedList drag reorder

Enhanced existing `VirtualizedList` in-place with optional vertical drag reorder. The component now keeps an internal item order, renders original item indices through that order, supports `set_draggable(true)`, exposes `set_on_reorder(from_index, to_index, ...)`, and remeasures after reorder without storing `AnyElement` across frames. Gallery and Docs now include a vertical drag sorting example plus compile-checked snippet `virtualized_list/draggable.rs`.

Validation evidence:
- `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets` passed.
- `cargo test -p liora-components virtualized_list` passed.
- `cargo test -p liora-gallery virtualized_list_demo` passed.



### memory-state-md-0050-8c49af414a40

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0050-8c49af414a40" sha256="8c49af414a40ee6a85d78fa90f17cc91b627116217f06b53cd4045531ce15f2e" -->

## 2026-05-18 Drag reorder handle correction

Corrected HorizontalList and VirtualizedList drag UX after feedback that invisible whole-item dragging was not acceptable. Dragging now starts only from an explicit front-side `GripVertical` handle rendered before each draggable item/row, while hover/drop detection stays on the item shell. Gallery and docs wording now points users to the visible drag handle.



### memory-state-md-0051-63cb1f6fe341

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0051-63cb1f6fe341" sha256="63cb1f6fe341660a378e7d297dc56ee75b5c7fe2b43eb804be94f1f14a2b3c95" -->

## 2026-05-18 Drag reorder live hover fix

Fixed reorder interaction after testing feedback: drag handles are now full-height flex boxes so the Grip icon is centered, and dragging reorders immediately when the pointer moves over a target item/row instead of waiting for final mouse-up delivery. This avoids lost drops when GPUI mouse-up is delivered to the original drag handle instead of the hovered item.



### memory-state-md-0052-a95d556b816b

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0052-a95d556b816b" sha256="a95d556b816bfcf13bb2fc737ae2f2d4b764fa57bb231507d14b7cd3cf5286ed" -->

## 2026-05-18 Generic draggable helper and follow-pointer list motion

Added reusable `draggable` module inspired by drag-rs' operation model (start point, current pointer, result/reorder callback) while staying pure GPUI/native. The module provides `DragState`, `DragAxis`, default `drag_handle`, and shared `reorder_indices` helpers so future controls can add handle-based dragging without duplicating pointer bookkeeping or storing rendered elements. HorizontalList and VirtualizedList now use this module and apply axis-specific margin offsets to the active item, producing a native follow-pointer drag motion plus live reorder-on-hover.

Validation evidence:
- `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets` passed.
- `cargo test -p liora-components draggable` passed.
- `cargo test -p liora-components horizontal_list` passed.
- `cargo test -p liora-components virtualized_list` passed.
- `cargo test -p liora-gallery horizontal_list_demo` passed.
- `cargo test -p liora-gallery virtualized_list_demo` passed.



### memory-state-md-0053-d730a2aea888

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0053-d730a2aea888" sha256="d730a2aea888886bdecec448673b366124fde485fcc0866e17fcf245a17ca499" -->

## 2026-05-18 Drag follow-pointer positioning fix

Corrected draggable follow-pointer rendering: the previous implementation used margin offsets (`ml`/`mt`), which changed layout and created empty space but did not visually move the dragged item as a floating object. HorizontalList and VirtualizedList now apply `relative().left(dx).top(dy)` with shadow while active, so the item is visually offset along the drag axis without using margin-based layout movement.

Validation evidence:
- `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets` passed.
- `cargo test -p liora-components draggable` passed.
- `cargo test -p liora-components horizontal_list` passed.
- `cargo test -p liora-components virtualized_list` passed.



### memory-state-md-0054-c00d2c6ab758

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0054-c00d2c6ab758" sha256="c00d2c6ab75872181c9daf60cbfc45a28866458b4b9a5acd7780467c1925f95c" -->

## 2026-05-18 Drag reorder stability correction

Fixed the follow-pointer drag instability where the dragged element jumped and then appeared to run away. Root cause: reordering during hover changed the dragged item layout slot while offsets were still computed from the original pointer anchor. Dragging now keeps the original order during movement, only updates the over/target index and pointer offset, and performs the actual reorder once on mouse-up/out using the last hovered target. This keeps the active element following the pointer from its original slot instead of recalculating against a moving slot.

Validation evidence:
- `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets` passed.
- `cargo test -p liora-components draggable` passed.
- `cargo test -p liora-components horizontal_list` passed.
- `cargo test -p liora-components virtualized_list` passed.
- Follow-up correction: root list containers now also track mouse movement while the left button is pressed, and active item hover no longer overwrites the drop target. This prevents the translated active item from stealing hover events and making the target/offset look random.
- Additional validation: `cargo test -p liora-gallery horizontal_list_demo` passed; `cargo test -p liora-gallery virtualized_list_demo` passed.


### memory-state-md-0055-30ce4ba5179c

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0055-30ce4ba5179c" sha256="30ce4ba5179c313b6f5baa27af808dae51038bb70019ba631ce945263561778a" -->

## 2026-05-18 Drag reorder top-layer and live slot preview

Adjusted draggable list behavior so the active dragged row/card is painted above siblings with GPUI deferred drawing priority while preserving its layout participation. Hovering another item now performs a live visual reorder so surrounding items give way immediately; DragState keeps the original position for the final callback and resets the pointer anchor when the active slot changes to avoid runaway offsets.

Validation evidence:
- `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets` passed.
- `cargo test -p liora-components draggable` passed.
- `cargo test -p liora-components horizontal_list` passed.
- `cargo test -p liora-components virtualized_list` passed.
- `cargo test -p liora-gallery horizontal_list_demo` passed.
- `cargo test -p liora-gallery virtualized_list_demo` passed.


### memory-state-md-0056-8b5c2464e7b7

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0056-8b5c2464e7b7" sha256="8b5c2464e7b76c0ad416aa914421a1ceb36d194443e4f3005a1bb6da5d6902f1" -->

## 2026-06-16 P13 docs navigation cleanup

Split the combined `LabelOperation` docs surface into separate `Label` and `Operation` pages so each P13 component is independently discoverable in liora-docs. Added dedicated compile-checked snippets under `content/snippets/label/basic.rs` and `content/snippets/operation/basic.rs`, wired both snippets into `check_snippets`, and updated the docs page registry. Also refreshed `.memory/inventory.md` to mark CodeEditor, RingChart external labels, and BarChart value range colors as implemented based on current source/docs coverage.

Validation evidence:
- `cargo check -p liora-docs --bin check_snippets` passed.
- `cargo check -p liora-docs` passed.


### memory-state-md-0057-a0f487549f93

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0057-a0f487549f93" sha256="a0f487549f93f8d8f0004319cb3094112735c0e3d85aae7f0b6916b23dbdd860" -->

## 2026-06-16 P13 gallery navigation cleanup

Split the combined Gallery `LabelOperation` demo into independent `Label` and `Operation` demo entries to match the docs split. `Label` now demonstrates basic icons, semantic colors, spacing, sizing, and custom icon elements. `Operation` now demonstrates Switch/Button actions, status labels/colors, disabled rows, and compact no-padding rows. The old combined gallery module was removed so P13 components are independently searchable in both Gallery and Docs.

Validation evidence:
- `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets` passed.
- `cargo test -p liora-gallery label_demo` passed.
- `cargo test -p liora-gallery operation_demo` passed.


### memory-state-md-0058-8d3caa524d02

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0058-8d3caa524d02" sha256="8d3caa524d02d07eb7a628c61200cc53b7bfa2d67056c6d2372ef83a039931da" -->

## 2026-06-16 P13 plan status refresh

Updated `.prompt/P13-component-expansion.md` from planned/waiting status to implemented/maintenance status, checked off all five implementation waves, and added a current implementation snapshot. Updated `prompt.md` so the top-level project prompt no longer describes P13 as merely planned.

Validation evidence:
- `cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets` passed before the status-only documentation update.


### memory-state-md-0059-24dce1765cf4

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0059-24dce1765cf4" sha256="24dce1765cf4f41ca189e5eb4d34ec25da7d3fb4dff59aab22c6dc2d3faa7322" -->

## 2026-06-16 P10 Sparkline completion

Audited recent work logs after the user recalled unfinished component supplementation. Found P13 was implemented, but P10 chart inventory still had a real missing `Sparkline` component while PieChart/RingChart were implemented but marked planned. Added native `Sparkline` to `liora-components`, Gallery, Docs, and compile-checked snippets. Updated P10 inventory to reflect PieChart/RingChart/Sparkline status.

Sparkline capabilities:
- Compact native GPUI canvas/path rendering for metric cards, table cells, and dashboards.
- Trend-aware positive/negative colors, custom color, area fill, 0 baseline, fixed y-domain, smooth/straight lines, solid/dashed/dotted style, custom dash pattern, and optional last-point marker.

Validation evidence:
- `cargo test -p liora-components sparkline` passed.
- `cargo test -p liora-gallery sparkline_demo` passed.
- `cargo check -p liora-docs --bin check_snippets` passed.


### memory-state-md-0060-8b9edeb3dc44

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0060-8b9edeb3dc44" sha256="8b9edeb3dc44e0a7de44ad7ea512328137bfe5b7c738eed947a9fdcd808ebb07" -->

## 2026-06-16 P10 chart downsampling performance pass

Implemented the first P10 performance review item for native chart rendering. Added shared min/max bucket downsampling in `chart.rs` and wired it into `LineChart`, `AreaChart` (overlay and stacked paths), and `Sparkline`. New public builders: `max_render_points(...)` and `disable_downsampling()` on LineChart/AreaChart/Sparkline. The strategy preserves first/last points plus local extrema so long monitoring series keep spikes while bounding GPUI path complexity. Gallery and Docs now include downsampling examples for LineChart, AreaChart, and Sparkline with compile-checked snippets.

Remaining P10 maintenance work: hover/tooltip hit testing and any further large-data cache policy beyond draw-point downsampling.



### memory-state-md-0061-d46d8e25c45c

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0061-d46d8e25c45c" sha256="d46d8e25c45c3234f6c76b81127d56fd2314c4bbe95468a97ab07066a0a8c3fd" -->

## 2026-06-16 P14 Wave 1 deferred advanced components

Promoted the old P9 deferred backlog into the active P14 Deferred Advanced phase and implemented the first batch of high-value advanced controls:
- `Carousel`: native carousel item model, indicator placement, arrow visibility, direction/autoplay configuration, and custom content slot.
- `Calendar`: month grid, selected date, range highlighting, disabled dates, event labels, and selection callback.
- `InputTag`: `Input` + `TagFlow` composition with Enter-to-add, closable tags, max tag limit, duplicate policy, and change callback.

Added Gallery demos, Docs pages, external compile-checked snippets, and updated `.prompt/P14-deferred-advanced.md`. Remaining P14 backlog: TreeSelect, Mention, Watermark, Tour, VirtualizedTable, VirtualizedTree.



### memory-state-md-0062-4848f6f04a3a

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0062-4848f6f04a3a" sha256="4848f6f04a3af2349c149274090845cb1092534e1c200910a23cc984b846384b" -->

## 2026-06-16 P14 Wave 2 mention and watermark

Implemented two additional deferred advanced controls:
- `Mention`: an Input-backed mention field with configurable trigger character, candidate filtering, max suggestions, disabled state, and select callback. It follows the existing Input/Autocomplete composition direction instead of reimplementing text input.
- `Watermark`: a native wrapper for text watermarks over arbitrary GPUI content, with cover/header/footer placement, density, gap, opacity, color, and rotation configuration metadata.

Added Gallery demos, Docs pages, and compile-checked snippets for both controls. Remaining P14 backlog: TreeSelect, Tour, VirtualizedTable, VirtualizedTree.



### memory-state-md-0063-ebf070359af7

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0063-ebf070359af7" sha256="ebf070359af78411ac386cff213138784dd1c0dfbfb7326028657f086fc7f564" -->

## 2026-06-16 P14 Wave 3 TreeSelect

Implemented `TreeSelect` as the next deferred advanced control. It supports hierarchical nodes, single and multiple selection, default selected keys, disabled keys, filterable search, selected label flattening, and selection callbacks. Added Gallery demo, Docs page, compile-checked snippets, and focused tests for tree filtering/flattening helpers. Remaining P14 backlog: Tour, VirtualizedTable, VirtualizedTree.



### memory-state-md-0064-3b5e716c0e00

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0064-3b5e716c0e00" sha256="3b5e716c0e00beb0a3ebcd28a99cb3fd88df55ecd42cdfb7c681711226df3a9f" -->

## 2026-06-17 P14 Wave 4 Tour

Implemented `Tour` as a controlled native step-guide component with step list, active index, target labels, placement metadata, progress/mask switches, previous/next/finish/close callbacks, Gallery demo, Docs page, compile-checked snippets, and focused navigation tests. Remaining P14 backlog: VirtualizedTable and VirtualizedTree.



### memory-state-md-0065-69fad9205637

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0065-69fad9205637" sha256="69fad920563798034c705a1d3b45e1993a642be9186f3de46da656e51933fb2c" -->

## 2026-06-17 P14 Wave 5 VirtualizedTable

Implemented `VirtualizedTable` as a fixed-header large-data table that reuses `TableColumn` definitions and GPUI `ListState` to render visible rows only. Cells are generated from row index + column key each frame to avoid stale GPUI element caching. Added Liora scrollbar, height/row-height/overdraw configuration, stripe/border/loading/empty states, sorting callback, Gallery demos, Docs page, compile-checked snippets, and focused tests. Remaining P14 backlog: VirtualizedTree.



### memory-state-md-0066-888f80dc71eb

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0066-888f80dc71eb" sha256="888f80dc71eb0627860be3721e219e1833b86d3a40b22f47b2127fe74c2814f1" -->

## 2026-06-17 P14 Wave 6 VirtualizedTree

Implemented `VirtualizedTree` as the final P14 deferred advanced control. It virtualizes large hierarchical datasets by flattening the currently expanded tree into lightweight visible-node metadata and rendering visible rows via GPUI `ListState`, with Liora scrollbar, expand/collapse, single/multiple selection, checkbox mode, default expanded/selected keys, callbacks, Gallery demos, Docs page, compile-checked snippets, and focused tests. P14 backlog is complete.



### memory-state-md-0067-e93ce4915e17

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0067-e93ce4915e17" sha256="e93ce4915e17c745b3ea7637c9cba923ca88a0f547985baa3f04d03b99297ed3" -->

## 2026-06-17 P12 install/uninstall smoke plan

Added `cargo run -p xtask -- package install-smoke ...` as a runner-safe install/uninstall readiness gate. The command defaults to plan-only mode: it reuses package artifact discovery and smoke validation, prints per-format install / launch-smoke / uninstall commands, and writes `target/packages/install-smoke-plan.md` for CI artifacts. `--execute-install` is intentionally restricted to portable `.tar.gz`, where it extracts to `target/install-smoke/<package>`, verifies launcher + `bin/<binary>`, then removes the directory. CI now runs the plan-only install/uninstall smoke gate after artifact smoke and before artifact upload. Remaining P12 blockers are external/policy-heavy: signing/notarization, real system package install/uninstall execution on dedicated runners, release tag validation, and license policy finalization.


### memory-state-md-0068-c185c7a6740a

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0068-c185c7a6740a" sha256="c185c7a6740a5bde9e2f1f69b18dbecc955105989065ed4f8b627d42f72672a6" -->

## 2026-06-17 P10 Cartesian chart hover hit testing

Completed a concrete P10 maintenance slice for chart hover behavior. Added shared pure cartesian hit-testing helpers (`ChartHitPoint`, `nearest_cartesian_hit_point`, `format_hit_tooltip`) plus a reusable `ChartBoundsTracker` that records canvas bounds without storing frame-local GPUI elements. `LineChart` now exposes and uses `.show_tooltip(...)` / `.tooltip_hit_radius(...)` for native hover tooltips. `AreaChart` exposes the same API and enables tooltip hit testing for Overlay mode; Stacked mode intentionally avoids false cartesian hit reporting until a cumulative-layer hit model is added. Gallery and Docs/snippets now surface tooltip radius and disabled-tooltip examples. Remaining optional P10 tooltip polish: BarChart rectangular hit testing and Pie/Ring polar sector hit testing.


### memory-state-md-0069-6dc5cbef57d7

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0069-6dc5cbef57d7" sha256="6dc5cbef57d74b16d03162b21b0a7b87747f4a4b0cd1d6c0acd703e1fac1ccf7" -->

## 2026-06-17 P10 BarChart hover hit testing

Completed the next chart tooltip polish slice for `BarChart`. Added tested grouped and stacked rectangular hit-box geometry (`BarChartHitBox`, `bar_chart_hit_boxes`, `nearest_bar_chart_hit_point`) and wired it into the native hover tooltip portal. Grouped mode hits individual side-by-side bars; stacked mode hits the concrete segment inside a stacked column. Gallery and Docs/snippets now show tooltip radius and disabled-tooltip examples. Remaining optional P10 tooltip polish: PieChart/RingChart polar sector hit testing and any further large-data cache policy.


### memory-state-md-0070-8958da926fb1

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0070-8958da926fb1" sha256="8958da926fb1b0b43c921e12c62f37d6bef5ca7b3f7551428cbc2ac70be52051" -->

## 2026-06-17 P10 Pie/Ring polar chart hover hit testing

Completed the remaining chart tooltip slice for `PieChart` and `RingChart`. Added pure polar-sector hit-testing helpers and wired native hover tooltip support into both charts. `PieChart` hits rendered sectors; `RingChart` hits only donut segments and excludes the inner hole. Public builders `show_tooltip(...)` and `tooltip_hit_radius(...)` are now documented in Gallery, Docs live demos, and compile-checked snippets. Remaining P10 maintenance item: any further cache policy beyond existing downsampling.


### memory-state-md-0071-b63347010805

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0071-b63347010805" sha256="b633470108058d4419fac78b6232c67577262ba11a031c4d960e18898b1c1cf7" -->

## 2026-06-17 P12 install-smoke dry-run readiness

Fixed `xtask package install-smoke --dry-run` so plan-only mode no longer requires real backend artifacts or scans stale `target/packages` files. Dry-run now derives expected artifact paths per app/platform/format and writes install/uninstall plans; non-dry-run still discovers and smokes real artifacts, while `--execute-install` remains restricted to portable `.tar.gz`. Validation passed: `cargo check -p xtask -p liora-packager`, `cargo test -p liora-packager`, `cargo test -p xtask install_smoke -- --nocapture`, `cargo run -p xtask -- package validate`, `cargo run -p xtask -- package ci --all-apps --format platform-defaults --dry-run --skip-build`, `cargo run -p xtask -- package install-smoke --all-apps --format platform-defaults --dry-run`, `cargo fmt --all --check`, and `git diff --check`.


### memory-state-md-0072-c38e42fcd325

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0072-c38e42fcd325" sha256="c38e42fcd325dfdc88f7901af14cfc698394b6777f5dcea66aa234fdc7dc5be9" -->

## 2026-06-17 phase readiness documentation sync

Synchronized architecture and inventory records with current evidence: P10 native charts are complete with downsampling and Line/Area/Bar/Pie/Ring hover hit testing; P14 deferred advanced backlog is complete; P12 remains in readiness with local runner-safe packaging gates and external-policy work for signing/notarization/real system installs/license.


### memory-state-md-0073-35bd5a4a0ea1

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0073-35bd5a4a0ea1" sha256="35bd5a4a0ea11e5c36e8a3dbecf5f9aa10864632129bf202f0443286f0c63567" -->

## 2026-06-17 P12 release tag validation

Added GitHub Actions release tag validation in the package workflow. `v*` release builds now require `vX.Y.Z` and the tag version must match `crates/liora-packager/Cargo.toml`; this prevents prerelease/mismatched tags from reaching package backends such as Windows MSI that require numeric versions. Updated P12 technical plan and prompt handoff docs to distinguish completed preview packaging from remaining real `v*` release-runner/signing/system-install policy work.


### memory-state-md-0074-27a7a66c48b7

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0074-27a7a66c48b7" sha256="27a7a66c48b737b5933d79044e85f078917521752e56789dd8c1c888179c5163" -->

## 2026-06-17 phase handoff stale-state cleanup

Updated the handoff state so new sessions no longer start from the obsolete P8/P9-era “current phase” text. Current source-of-truth summary: P10/P11/P13/P14 are complete; P12 has local runner-safe packaging readiness implemented and only external-policy items remain. Repository remote is SSH (`git@github.com:yhyzgn/liora.git`).


### memory-state-md-0075-221cbd65a436

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0075-221cbd65a436" sha256="221cbd65a4360d5aeb9f4a444a7bbee976a617dac67a7a60e6a1eb4136eab28f" -->

## 2026-06-17 P15 quality hardening kickoff

Started P15 as the release-quality hardening phase after P13/P14 completion and P12 runner-safe readiness. First concrete slice: add `.github/workflows/ci.yml` as a general quality gate for fmt, workspace check/test, docs snippet check, packaging validate, packaging dry-run, and install-smoke dry-run.


### memory-state-md-0076-fe5d85f92837

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0076-fe5d85f92837" sha256="fe5d85f92837f674059c6f719e52f92210ad7a89e71270edca82687377373a03" -->

## 2026-06-18 P15 Track D Preview close-policy docs/examples

Preview outside-click close policy is now discoverable in Docs, compile-checked snippets, live docs demos, and Gallery. The representative example disables both ESC and outside-click dismissal for controlled overlay flows. Validation passed: fmt, docs snippet check, workspace check/test, diff whitespace check, and Gallery/Docs GUI startup smoke.


### memory-state-md-0077-ce103215ae07

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0077-ce103215ae07" sha256="ce103215ae07ae79062b31afa508f75209e635e6b28550c1782d3680380b6eba" -->

## 2026-06-18 P15 Track D Tour close-policy docs/examples

Tour overlay close-policy behavior is now covered in source-level tests, Gallery, Docs, compile-checked snippets, and the docs snippet loader. The controlled-close example disables both ESC and outside-click dismissal for critical guided flows. Validation passed: fmt, focused Tour tests, docs snippet check, workspace check/test, diff whitespace check, and Gallery/Docs GUI startup smoke.


### memory-state-md-0078-37d738adc79a

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0078-37d738adc79a" sha256="37d738adc79a2c4def1ff4953808f04c52d9b4cd361c94df1c50b2430809d434" -->

## 2026-06-18 P15 Track A CI/package workflow boundary docs

Packaging Workflow docs now explicitly separate `.github/workflows/ci.yml` quality gates from `.github/workflows/package.yml` native packaging/release responsibilities. CI is documented as validation/dry-run only; package workflow owns platform-specific packaging, raw binaries, artifacts, changelog, and `v*` GitHub Release publishing. A docs regression test locks this boundary. Validation passed: fmt, focused docs test, docs snippet check, workspace check/test, diff whitespace check, and Gallery/Docs GUI startup smoke.


### memory-state-md-0079-29e79d7ff9b9

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0079-29e79d7ff9b9" sha256="29e79d7ff9b954de41b58fefb7ea8f8b64706a2d2e8284531180a9cea2b342a0" -->

## 2026-06-18 P15 Track F docs snippet loader completeness

Docs UI snippet loading is now complete for all authored docs page `src="..."` code blocks. Fixed 22 loader gaps for snippets that were present and compile-checked but not displayable in Docs. Added a regression test that parses every docs page and asserts each referenced snippet resolves through `load_code_snippet`. Validation passed: fmt, focused docs loader test, docs snippet check, workspace check/test, diff whitespace check, and Gallery/Docs GUI startup smoke.


### memory-state-md-0080-fcd4364d728b

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0080-fcd4364d728b" sha256="fcd4364d728bbdbf5ef5d45cddc134a1090b71ae198856b3d6c9db73f9845494" -->

## 2026-06-18 P15 Track A split CI quality and packaging dry-run jobs

General CI now has separate `rust-quality` and `packaging-dry-run` jobs. Workspace fmt/check/test/snippet checks keep GPUI/Linux native dependencies; package metadata/dry-run/install-smoke dry-run executes in a lightweight job with only `file` tooling. Packaging Workflow docs and docs tests lock the job split. Validation passed: workflow YAML parse, fmt, focused docs packaging tests, docs snippet check, workspace check/test, xtask package validate/dry-run/install-smoke dry-run, diff whitespace check, and Gallery/Docs GUI startup smoke.


### memory-state-md-0081-6a45b330a81d

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0081-6a45b330a81d" sha256="6a45b330a81d3c3aa27a7cdd606343170ce271738193da6f1d0f2d780d3de65c" -->

## 2026-06-18 P15 Track F QuickStart key binding completeness

QuickStart minimal window setup now registers CodeEditor and Tour key bindings in addition to existing input, selection, preview, popup, and typography bindings. A docs regression test compares QuickStart against Gallery and Docs for key bindings that affect text selection, code editing, Preview, and Tour overlay behavior. Validation passed: fmt, focused docs test, docs snippet check, workspace check/test, diff whitespace check, and Gallery/Docs GUI startup smoke.


### memory-state-md-0082-5409a8d86fb5

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0082-5409a8d86fb5" sha256="5409a8d86fb588743af1bf1b7792155490bc91118c991e6ca6ce2765553e5151" -->

## 2026-06-18 P15 Track E CodeBlock highlight cache eviction

CodeBlock highlight cache now uses bounded FIFO eviction instead of clearing the entire cache after overflow. This prevents long docs pages or theme/language variations from forcing a full cache cold start after a single over-capacity insertion. Validation passed: fmt, focused CodeBlock tests, workspace check/test, diff whitespace check, and Gallery/Docs GUI startup smoke.


### memory-state-md-0083-deb9a0590346

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0083-deb9a0590346" sha256="deb9a059034646536cfe67497861c64199935f9c49e97c396148c3e4be109903" -->

## 2026-06-18 P15 Track E CodeBlock shared highlight runs

CodeBlock highlight cache values now use shared `Arc<[TextRun]>` storage. Selectable and read-only block code paths retrieve a highlight cache key plus shared runs, so repeated visible CodeBlock/CodeEditor preview renders do not allocate-clone the full TextRun vector unless an inline `StyledText` API still requires owned runs. Focused regression coverage proves repeated cached block lookups pointer-share the same Arc storage. Validation passed: fmt, focused CodeBlock tests, workspace check/test, docs snippet check, diff whitespace check, and Gallery/Docs GUI startup smoke.


### memory-state-md-0084-8cd39553ab86

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0084-8cd39553ab86" sha256="8cd39553ab86aa77591384b73cd176fe0b553cb7616953882aaf85cf6cd4a001" -->

## 2026-06-18 P15 Track B synchronized state panic hardening

CodeBlock highlight/selection state, SelectableText selection state, and Timer runtime registries now recover poisoned mutexes with helper functions instead of panicking on `expect("... lock poisoned")`. The avoidable runtime panic audit now explicitly locks this behavior for those synchronized runtime-state paths. Validation passed: fmt, focused component tests, workspace check/test, docs snippet check, diff whitespace check, and Gallery/Docs GUI startup smoke.


### memory-state-md-0085-7848a9af900e

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0085-7848a9af900e" sha256="7848a9af900eb7a0495003171b43704f3393d75130d76dca049035d6b8589442" -->

## 2026-06-18 P15 Track B tray icon fallback hardening

Gallery and Docs no longer panic if bundled tray icon decoding fails. Both apps now attempt the bundled icon first, then app-specific solid-color fallback icons, and finally continue tray installation without an icon if all icon creation fails. Dynamic tray icon changes also skip failed icon loads instead of crashing command handling. Validation passed: app checks, focused tests, workspace check/test, docs snippet check, diff whitespace check, and Gallery/Docs GUI startup smoke.


### memory-state-md-0086-bc9cba76cefa

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0086-bc9cba76cefa" sha256="bc9cba76cefaa9eef6c45553733b51c0c2a709a472ef99d177b784946c7a5c79" -->

## 2026-06-18 P15 Track B packager string rendering panic cleanup

`liora-packager` no longer uses `expect("write to string")` for SHA-256 hex, checksum text, release notes, or package manifest JSON rendering. Those paths now assemble strings with `format!` and `push_str`, preserving generated output while removing impossible-but-panic-based string write assumptions from the packaging pipeline. Validation passed: liora-packager tests, workspace check/test, docs snippet check, diff whitespace check, and Gallery/Docs GUI startup smoke.


### memory-state-md-0087-c0dd09ba1f4a

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0087-c0dd09ba1f4a" sha256="c0dd09ba1f4a77bf4cd7102b82289bab4e62b81d432d9304bb5218925cb89764" -->

## 2026-06-18 P15 Track B lucide build script error handling

`liora-icons-lucide` build script now uses `try_main() -> io::Result<()>` instead of unwraps for OUT_DIR, SVG directory reads, file names, generated file creation, and writes. Build failures now produce clear cargo error output while preserving the generated `IconName` format and rerun behavior. Validation passed: lucide check, workspace check/test, docs snippet check, diff whitespace check, and Gallery/Docs GUI startup smoke.



### memory-state-md-0088-f82c6d2358fc

<!-- ctx-migration source=".memory/state.md" unit="memory-state-md-0088-f82c6d2358fc" sha256="f82c6d2358fc2dfdb109f3da8c8672975ed2ee3f0d9512094d3c449e47b3b27d" -->

## 2026-06-18 P20 theme and interaction polish

Completed P20 as a focused theme/interaction consistency phase. Dark semantic subtle tokens now use translucent semantic overlays instead of white-mixed tints, preventing selected/hover states from becoming overly bright in dark mode. Dialog, Drawer, Tour, and Loading masks now use theme tokens; CodeEditor gutter borders and custom WindowFrame close hover use semantic theme tokens. Gallery now has a Theme dogfooding demo, Docs has a Theme page with compile-checked snippet coverage, and visual theme regression tests lock the tokenized paths.

<!-- ctx-managed-legacy-migration:end -->
