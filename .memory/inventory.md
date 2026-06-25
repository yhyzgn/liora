# Component Inventory

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

## P6 Built-in Unique ID ✅

| Item | File(s) | Status |
|------|---------|--------|
| Global unique ID generator | `crates/liora-core/src/lib.rs` | ✅ `next_unique_id()` + `unique_id(prefix)` + `stable_unique_id(...)`; direct allocation only at persistent construction, `stable_unique_id` in render paths |
| Component default IDs | `crates/liora-components/src/*.rs` | ✅ Runtime unique IDs replace `track_caller`/literal repeated interactive IDs in migrated components |
| Override APIs | Multiple components | ✅ `.id(...)` retained or added for migrated interactive components |

## P7 Demo Self-Contained 🔄

— See `.prompt/P7-demo-self-contained.md` for full task list.

| Item | File(s) | Status |
|------|---------|--------|
| Gallery registry ASC order | `apps/liora-gallery/src/demos/mod.rs` | ✅ Runtime sort + regression test |
| Button demo self-contained slice | `apps/liora-gallery/src/demos/button_demo.rs` | ✅ Uses Liora `Space`/`Title` instead of direct `div()`/`px()` layout primitives |
| Liora demo helpers | `crates/liora-components/src/space.rs`, `button.rs` | ✅ `Space::wrap` + semantic gaps; Button rounded helpers |

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

## P11 Native Tray / Process Resident 🔄

| # | Capability | File | Demo | Docs | Status |
|---|------------|------|------|------|--------|
| 1 | Tray facade crate | `crates/liora-tray/src/lib.rs` | — | — | ✅ `TrayConfig`, `TrayMenuItemSpec`, `TrayCommand`, `LioraTray` |
| 2 | Dynamic icon API | `crates/liora-tray/src/lib.rs` | ✅ | ✅ | ✅ `set_icon`, `clear_icon`, `set_icon_from_rgba`, `set_icon_from_path` |
| 3 | CheckBox menu state | `crates/liora-tray/src/lib.rs` | ✅ | ✅ | ✅ Check menu item config + state sync |
| 4 | Recursive native menus | `crates/liora-tray/src/lib.rs` | ✅ | ✅ | ✅ Action, separator, 2nd/3rd/N-level submenu DSL |
| 5 | Gallery/docs examples | `apps/liora-gallery/src/demos/tray_demo.rs`, `apps/liora-docs/content/pages/tray.md` | ✅ | ✅ | ✅ Rich non-intrusive config preview + compile-checked snippets |


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

## P22 gpui-component Harvest 🧭 Active

| # | Component / Enhancement | File | Demo | Docs | Status |
|---|-------------------------|------|------|------|--------|
| 1 | Spinner | `crates/liora-components/src/spinner.rs` | ✅ | ✅ | Implemented Wave A: standalone inline loading indicator with size/icon/color builders |
| 2 | Kbd | `crates/liora-components/src/kbd.rs` | ✅ | ✅ | Implemented Wave A: keyboard shortcut keycap display with size/color/background builders |
| 3 | OtpInput | `crates/liora-components/src/otp_input.rs` | ✅ | ✅ | Implemented Wave A: controlled OTP/PIN cell display with length/mask/status/active-index builders |
| 4 | DropdownButton | — | — | — | Planned Wave A |
| 5 | Accordion | — | — | — | Planned Wave A |
| 6 | Combobox | — | — | — | Planned Wave A; likely needs SearchableList/shared popup infrastructure |
