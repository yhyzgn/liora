# Component Inventory

## P0 Foundation ✅

| Component | File | Tests | Demo |
|-----------|------|-------|------|
| Theme (light/dark) | `crates/aura-theme/src/lib.rs` | — | — |
| Config (Global) | `crates/aura-core/src/lib.rs` | — | — |
| ContextExt trait | `crates/aura-core/src/lib.rs` | — | — |
| ElementExt trait | `crates/aura-core/src/lib.rs` | — | — |
| Z-Index utils | `crates/aura-core/src/lib.rs` | — | — |
| Button | `crates/aura-components/src/button.rs` | — | ✅ |
| Gallery app | `apps/aura-gallery/src/` | — | ✅ |

## P1 Basic Elements ✅ (15/15)

| # | Component | File | Demo | Status |
|---|-----------|------|------|--------|
| 1 | Icon + aura-icons-lucide | `crates/aura-icons/` `crates/aura-icons-lucide/` | ✅ | ✅ Icon system done |
| 2 | Button (codex refactor) | `crates/aura-components/src/button.rs` | ✅ | ✅ codex complete |
| 3 | Link | `crates/aura-components/src/link.rs` | ✅ | ✅ |
| 4 | Text | `crates/aura-components/src/text.rs` | ✅ | ✅ |
| 5 | Title | `crates/aura-components/src/title.rs` | ✅ | ✅ |
| 6 | Paragraph | `crates/aura-components/src/paragraph.rs` | ✅ | ✅ |
| 7 | Space | `crates/aura-components/src/space.rs` | ✅ | ✅ Container gap support |
| 8 | Divider | `crates/aura-components/src/divider.rs` | ✅ | ✅ |
| 9 | Row (栅格) | `crates/aura-components/src/row.rs` | ✅ | ✅ |
| 10 | Col (栅格) | `crates/aura-components/src/col.rs` | ✅ | ✅ Percent width fix |
| 11 | Container | `crates/aura-components/src/container.rs` | ✅ | ✅ |
| 12 | Scrollbar | `crates/aura-components/src/scrollbar.rs` | ✅ | ✅ |
| 13 | Splitter | `crates/aura-components/src/splitter.rs` | ✅ | ✅ |
| 14 | ButtonGroup | `crates/aura-components/src/button_group.rs` | ✅ | ✅ |
| 15 | CodeBlock | `crates/aura-components/src/code_block.rs` | ✅ | ✅ Code highlighting, language label, copy button, inline/block formats |

## P2 Form Controls 🔄 (8/10)

| # | Component | File | Status |
|---|-----------|------|--------|
| 1 | Input | `crates/aura-components/src/input.rs` | ✅ |
| 2 | InputNumber | `crates/aura-components/src/input_number.rs` | ✅ |
| 3 | Textarea | `crates/aura-components/src/textarea.rs` | ✅ |
| 4 | Checkbox / CheckboxGroup | `crates/aura-components/src/checkbox.rs`, `checkbox_group.rs` | ✅ Vertical/Horizontal/Button layouts |
| 5 | Radio / RadioGroup | `crates/aura-components/src/radio.rs`, `radio_group.rs` | ✅ Vertical/Horizontal/Button layouts |
| 6 | Switch | `crates/aura-components/src/switch.rs` | ✅ |
| 7 | Select | `crates/aura-components/src/select.rs` | ✅ |
| 8 | Slider | `crates/aura-components/src/slider.rs` | ✅ |
| 9 | Form / FormItem | `crates/aura-components/src/form.rs` | ✅ |
| 10 | Rate | `crates/aura-components/src/rate.rs` | ✅ |

## P3 Popper + Feedback ✅ (13/13)

| # | Component | Notes | Status |
|---|-----------|-------|--------|
| — | Popper/Portal 基建 | `crates/aura-core/src/popper.rs` | ✅ Done |
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

## P4 Nav + Data 🔄 (1/20)

| # | Component | File | Demo | Status |
|---|-----------|------|------|--------|
| 1 | Menu | `crates/aura-components/src/menu.rs` | ✅ | ✅ Horizontal/Vertical/Collapse |
| 2 | Tabs | `crates/aura-components/src/tabs.rs` | ✅ | ✅ Positions/Styles/Editable |
| 3 | Breadcrumb | `crates/aura-components/src/breadcrumb.rs` | ✅ | ✅ String/Icon Separators |
| 4 | Steps | `crates/aura-components/src/steps.rs` | ✅ | ✅ Horizontal/Vertical/Status |
| 5 | PageHeader | `crates/aura-components/src/page_header.rs` | ✅ | ✅ Title/SubTitle/Slots |
| 6 | Affix | `crates/aura-components/src/affix.rs` | ✅ | ✅ Top/Bottom Sticky |
| 7 | Backtop | `crates/aura-components/src/backtop.rs` | ✅ | ✅ Visibility Height |
| 8 | Anchor | `crates/aura-components/src/anchor.rs` | ✅ | ✅ Scroll Sync / Jump |
| 9 | Progress | `crates/aura-components/src/progress.rs` | ✅ | ✅ Line Style / Status |
| 10 | Skeleton | `crates/aura-components/src/skeleton.rs` | ✅ | ✅ Variants / Rows |
| 11 | Empty | `crates/aura-components/src/empty.rs` | ✅ | ✅ Default / Custom / Action |
| 12 | Result | `crates/aura-components/src/result.rs` | ✅ | ✅ Success/Warning/Error/Info |
| 13 | Descriptions | `crates/aura-components/src/descriptions.rs` | ✅ | ✅ Border / Direction / Grid |
| 14 | Timeline | `crates/aura-components/src/timeline.rs` | ✅ | ✅ Node variants / Reverse |
| 15 | Tree | `crates/aura-components/src/tree.rs` | ✅ | ✅ Expand / Collapse |
| 16 | Pagination | `crates/aura-components/src/pagination.rs` | ✅ | ✅ Layout / Pager |
| 17 | Statistic | `crates/aura-components/src/statistic.rs` | ✅ | ✅ Prefix / Suffix |
| 18 | Segmented | `crates/aura-components/src/segmented.rs` | ✅ | ✅ Block / Disabled |
| 19 | Tag | `crates/aura-components/src/tag.rs` | ✅ | ✅ Light / Dark / Plain |
| 20 | Avatar | `crates/aura-components/src/avatar.rs` | ✅ | ✅ Image / Icon / Shapes |
| 21 | Badge | `crates/aura-components/src/badge.rs` | ✅ | ✅ Value / Dot / Max |

## P5 Advanced 🏁 (11/20 complete; deferred scope moved to P9)

| # | Component | File | Demo | Status |
|---|-----------|------|------|--------|
| 1 | Table | `crates/aura-components/src/table.rs` | ✅ | ✅ P0 basic / custom header / opt-in sort / fixed header / empty / loading / border / stripe |
| 2 | DatePicker | `crates/aura-components/src/date_picker.rs` | ✅ | ✅ Formats / date range / month + month range / year + year range / disabled / callback |
| 3 | TimePicker | `crates/aura-components/src/time_picker.rs` | ✅ | ✅ Fixed time / steps / formats / hide seconds / disabled / callback |
| 4 | DateTimePicker | `crates/aura-components/src/date_time_picker.rs` | ✅ | ✅ DateTime / DateTime range / formats / steps / hide seconds / confirm / disabled / callbacks |
| 5 | Upload | `crates/aura-components/src/upload.rs` | ✅ | ✅ Button / drag style / text list / picture card / progress / status / limit / disabled / callbacks |
| 6 | Cascader | `crates/aura-components/src/cascader.rs` | ✅ | ✅ Multi-level / default selected / disabled / clearable / loading / lazy load / search results / callbacks |
| 7 | Transfer | `crates/aura-components/src/transfer.rs` | ✅ | ✅ Source/target panels / checked move / disabled items / filter display / callbacks |
| 8 | ColorPicker | `crates/aura-components/src/color_picker.rs` | ✅ | ✅ Cube trigger / popup HSV panel / hue bar / alpha bar / rgba display / presets / disabled / callback |
| 9 | Carousel | — | — | ↩️ Moved to P9 deferred backlog |
| 10 | Image | `crates/aura-components/src/image.rs` | ✅ | ✅ Remote URL / local file / fit modes / circle + round options / transparent ring sleeve / loading + fallback / empty / preview |
| 11 | Calendar | — | — | ↩️ Moved to P9 deferred backlog |
| 12 | TreeSelect | — | — | ↩️ Moved to P9 deferred backlog |
| 13 | Autocomplete | `crates/aura-components/src/autocomplete.rs` | ✅ | ✅ Static suggestions / filtering / click select / clearable / disabled / demo |
| 14 | InputTag | — | — | ↩️ Moved to P9 deferred backlog |
| 15 | Mention | — | — | ↩️ Moved to P9 deferred backlog |
| 16 | Watermark | — | — | ↩️ Moved to P9 deferred backlog |
| 17 | Tour | — | — | ↩️ Moved to P9 deferred backlog |
| 18 | Scrollbar | `crates/aura-components/src/scrollbar.rs` | ✅ | ✅ Already completed in P1 |
| 19 | Splitter | `crates/aura-components/src/splitter.rs` | ✅ | ✅ Already completed in P1 |
| 20 | VirtualizedTable/VirtualizedTree | — | — | ↩️ Moved to P9 deferred backlog |

## P6 Built-in Unique ID ✅

| Item | File(s) | Status |
|------|---------|--------|
| Global unique ID generator | `crates/aura-core/src/lib.rs` | ✅ `next_unique_id()` + `unique_id(prefix)` + `stable_unique_id(...)`; direct allocation only at persistent construction, `stable_unique_id` in render paths |
| Component default IDs | `crates/aura-components/src/*.rs` | ✅ Runtime unique IDs replace `track_caller`/literal repeated interactive IDs in migrated components |
| Override APIs | Multiple components | ✅ `.id(...)` retained or added for migrated interactive components |

## P7 Demo Self-Contained 🔄

— See `.prompt/P7-demo-self-contained.md` for full task list.

| Item | File(s) | Status |
|------|---------|--------|
| Gallery registry ASC order | `apps/aura-gallery/src/demos/mod.rs` | ✅ Runtime sort + regression test |
| Button demo self-contained slice | `apps/aura-gallery/src/demos/button_demo.rs` | ✅ Uses Aura `Space`/`Title` instead of direct `div()`/`px()` layout primitives |
| Aura demo helpers | `crates/aura-components/src/space.rs`, `button.rs` | ✅ `Space::wrap` + semantic gaps; Button rounded helpers |

## P8 Native Gallery Documentation ✅ Core Done

— See `.prompt/P8-engineering.md` for the updated native documentation plan.

| Item | Target | Status |
|------|--------|--------|
| Typography bootstrapping | `crates/aura-components/src/` rich text/paragraph primitives | ✅ `Paragraph` now renders GPUI `StyledText` runs from `Text` segments |
| Markdown renderer | `apps/aura-docs/src/markdown.rs` + `pulldown-cmark` | ✅ Stack-based native renderer for headings, paragraphs, inline strong/em/code/strike, lists, blockquotes |
| Docs content pages | `apps/aura-docs/content/pages/*.md` | ✅ One Markdown file per page/component |
| Docs code snippets | `apps/aura-docs/content/snippets/<page>/*.rs` | ✅ External `.rs` snippets referenced by fenced code `src="..."` |
| Code block styling + document shell | Native Aura/GPUI two-column docs UI | ✅ Fenced code blocks + `Aura Docs` main window with `Container`/`Menu` shell |
| Live Demo injection | `::AuraDemo{component="..."}::` → real Aura view nodes | ✅ Button demo marker maps to a real Aura `Button` node |

## P9 Deferred Advanced ↗️ (migrated to P14)

| # | Component | File | Demo | Status |
|---|-----------|------|------|--------|
| 1 | Carousel | `crates/aura-components/src/carousel.rs` | ✅ | ✅ Migrated to P14 Wave 1 and implemented |
| 2 | Calendar | `crates/aura-components/src/calendar.rs` | ✅ | ✅ Migrated to P14 Wave 1 and implemented |
| 3 | TreeSelect | — | — | ⏸️ Deferred from P5; implement later when requested |
| 4 | InputTag | `crates/aura-components/src/input_tag.rs` | ✅ | ✅ Migrated to P14 Wave 1 and implemented |
| 5 | Mention | — | — | ⏸️ Deferred from P5; implement later when requested |
| 6 | Watermark | — | — | ⏸️ Deferred from P5; implement later when requested |
| 7 | Tour | — | — | ⏸️ Deferred from P5; implement later when requested |
| 8 | VirtualizedTable | — | — | ⏸️ Deferred from P5; implement later when requested |
| 9 | VirtualizedTree | — | — | ⏸️ Deferred from P5; implement later when requested |

## P10 Native Charts ✅ (7/7 — implemented; tooltip polish remains optional)

| # | Component | File | Demo | Docs | Status |
|---|-----------|------|------|------|--------|
| 1 | Chart infrastructure | `crates/aura-components/src/chart*.rs` | — | — | ✅ Scale/domain/shape/frame/legend foundation; tooltip remains optional polish |
| 2 | LineChart | `crates/aura-components/src/line_chart.rs` | ✅ | ✅ | ✅ MVP: multi-series, axis/grid, legend, point markers, empty state |
| 3 | AreaChart | `crates/aura-components/src/area_chart.rs` | ✅ | ✅ | ✅ MVP: overlay/stacked area, axis/grid, legend |
| 4 | BarChart | `crates/aura-components/src/bar_chart.rs` | ✅ | ✅ | ✅ MVP: grouped/stacked vertical bars, axis/grid, legend |
| 5 | PieChart | `crates/aura-components/src/pie_chart.rs` | ✅ | ✅ | ✅ Implemented with value labels, outside labels, percentage/value patterns |
| 6 | RingChart | `crates/aura-components/src/pie_chart.rs` | ✅ | ✅ | ✅ Implemented with donut mode and external legends |
| 7 | Sparkline | `crates/aura-components/src/sparkline.rs` | ✅ | ✅ | ✅ Implemented: compact trend chart with trend colors, fill, baseline, line styles |

## P11 Native Tray / Process Resident 🔄

| # | Capability | File | Demo | Docs | Status |
|---|------------|------|------|------|--------|
| 1 | Tray facade crate | `crates/aura-tray/src/lib.rs` | — | — | ✅ `TrayConfig`, `TrayMenuItemSpec`, `TrayCommand`, `AuraTray` |
| 2 | Dynamic icon API | `crates/aura-tray/src/lib.rs` | ✅ | ✅ | ✅ `set_icon`, `clear_icon`, `set_icon_from_rgba`, `set_icon_from_path` |
| 3 | CheckBox menu state | `crates/aura-tray/src/lib.rs` | ✅ | ✅ | ✅ Check menu item config + state sync |
| 4 | Recursive native menus | `crates/aura-tray/src/lib.rs` | ✅ | ✅ | ✅ Action, separator, 2nd/3rd/N-level submenu DSL |
| 5 | Gallery/docs examples | `apps/aura-gallery/src/demos/tray_demo.rs`, `apps/aura-docs/content/pages/tray.md` | ✅ | ✅ | ✅ Rich non-intrusive config preview + compile-checked snippets |


## P13 Component Expansion ✅ Implemented

| # | Component / Enhancement | File Target | Demo | Docs | Status |
|---|-------------------------|-------------|------|------|--------|
| 1 | QrCode | `crates/aura-components/src/qr_code.rs` | ✅ | ✅ | Implemented: QR generation display, color/size/ECC config, decode_bytes/decode_file/decode_image API |
| 2 | CodeEditor | `crates/aura-components/src/code_editor.rs` | ✅ | ✅ | Implemented: native editor, line numbers, selection/copy, indentation, highlighting, diagnostics provider |
| 3 | SignalMeter | `crates/aura-components/src/signal_meter.rs` | ✅ | ✅ | Implemented Wave 1 |
| 4 | HeatBar | `crates/aura-components/src/heat_bar.rs` | ✅ | ✅ | Implemented Wave 1; time-axis dense vertical-bar heat chart with legend/count summary |
| 5 | BarChart standalone mini mode | existing `bar_chart.rs` | ✅ | ✅ | Implemented in-place; no separate FlatBarMeter |
| 6 | SegmentRatioBar | `crates/aura-components/src/segment_ratio_bar.rs` | ✅ | ✅ | Implemented Wave 1; segmented bar plus top/bottom/both/hidden legend-value text |
| 7 | HorizontalList | `crates/aura-components/src/horizontal_list.rs` | ✅ | ✅ | Implemented: horizontal scrolling, custom item/divider rendering, internal drag reorder, on_reorder callback |
| 8 | Vertical list drag | existing `virtualized_list.rs` / list components | ✅ | ✅ | Implemented in-place on VirtualizedList: drag reorder, internal order, on_reorder callback |
| 9 | RingChart external labels | existing `ring_chart.rs` / chart modules | ✅ | ✅ | Implemented in-place: external vertical/horizontal legends, side placement, item limits, content/decimal options |
| 10 | LineChart per-series style | existing `line_chart.rs` | ✅ | ✅ | Implemented in-place: solid/dashed/dotted/custom dash, per-series color/width/smooth |
| 11 | BarChart range colors | existing `bar_chart.rs` | ✅ | ✅ | Implemented in-place: value_color_ranges with docs/gallery coverage |
| 12 | RingProgress gradient | existing `progress.rs` | ✅ | ✅ | Implemented in-place: ring gradient plus completion color |
| 13 | Timer | `crates/aura-components/src/timer.rs` | ✅ | ✅ | Implemented controlled count-up/count-down display with units and result snapshot API |
| 14 | Button gradient/custom colors | existing `button.rs` | ✅ | ✅ | Implemented in-place: custom solid/outline colors, gradient backgrounds, derived hover/active/disabled states |
| 15 | Tag flow layout | existing `tag.rs` | ✅ | ✅ | Implemented in-place via TagFlow layout helper |
| 16 | Label | `crates/aura-components/src/label.rs` | ✅ | ✅ | Implemented Wave 1 |
| 17 | Operation | `crates/aura-components/src/operation.rs` | ✅ | ✅ | Implemented Wave 1 |
| 18 | Radio/Checkbox option customization | existing `radio*.rs`, `checkbox*.rs` | ✅ | ✅ | Implemented in-place: option card/chip styling, selected/hover/border/text/padding/radius/indicator customization |

## P14 Deferred Advanced 🧭 In Progress

| # | Component | File | Demo | Docs | Status |
|---|-----------|------|------|------|--------|
| 1 | Carousel | `crates/aura-components/src/carousel.rs` | ✅ | ✅ | Implemented Wave 1 |
| 2 | Calendar | `crates/aura-components/src/calendar.rs` | ✅ | ✅ | Implemented Wave 1 |
| 3 | InputTag | `crates/aura-components/src/input_tag.rs` | ✅ | ✅ | Implemented Wave 1 |
| 4 | TreeSelect | — | — | — | Pending |
| 5 | Mention | — | — | — | Pending |
| 6 | Watermark | — | — | — | Pending |
| 7 | Tour | — | — | — | Pending |
| 8 | VirtualizedTable | — | — | — | Pending |
| 9 | VirtualizedTree | — | — | — | Pending |
