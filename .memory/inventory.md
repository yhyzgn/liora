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
| Code block styling + document shell | Native Aura/GPUI two-column docs UI | ✅ Fenced code blocks + `Aura Docs` main window with `Container`/`Menu` shell |
| Live Demo injection | `::AuraDemo{component="..."}::` → real Aura view nodes | ✅ Button demo marker maps to a real Aura `Button` node |

## P9 Deferred Advanced ⏸️ (0/9 — backlog for later supplementation)

| # | Component | File | Demo | Status |
|---|-----------|------|------|--------|
| 1 | Carousel | — | — | ⏸️ Deferred from P5; implement later when requested |
| 2 | Calendar | — | — | ⏸️ Deferred from P5; implement later when requested |
| 3 | TreeSelect | — | — | ⏸️ Deferred from P5; implement later when requested |
| 4 | InputTag | — | — | ⏸️ Deferred from P5; implement later when requested |
| 5 | Mention | — | — | ⏸️ Deferred from P5; implement later when requested |
| 6 | Watermark | — | — | ⏸️ Deferred from P5; implement later when requested |
| 7 | Tour | — | — | ⏸️ Deferred from P5; implement later when requested |
| 8 | VirtualizedTable | — | — | ⏸️ Deferred from P5; implement later when requested |
| 9 | VirtualizedTree | — | — | ⏸️ Deferred from P5; implement later when requested |
