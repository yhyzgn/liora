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

## P1 Basic Elements ✅ (13/13)

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

## P2 Form Controls 🔄 (8/10)

| # | Component | File | Status |
|---|-----------|------|--------|
| 1 | Input | `crates/aura-components/src/input.rs` | ✅ |
| 2 | InputNumber | `crates/aura-components/src/input_number.rs` | ✅ |
| 3 | Textarea | `crates/aura-components/src/textarea.rs` | ✅ |
| 4 | Checkbox / CheckboxGroup | `crates/aura-components/src/checkbox.rs`, `checkbox_group.rs` | ✅ |
| 5 | Radio / RadioGroup | `crates/aura-components/src/radio.rs` | ✅ |
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

## P5 Advanced 🏃 (6/20)

| # | Component | File | Demo | Status |
|---|-----------|------|------|--------|
| 1 | Table | `crates/aura-components/src/table.rs` | ✅ | ✅ P0 basic / custom header / opt-in sort / fixed header / empty / loading / border / stripe |
| 2 | DatePicker | `crates/aura-components/src/date_picker.rs` | ✅ | ✅ Formats / date range / month + month range / year + year range / disabled / callback |
| 3 | TimePicker | `crates/aura-components/src/time_picker.rs` | ✅ | ✅ Fixed time / steps / formats / hide seconds / disabled / callback |
| 4 | DateTimePicker | `crates/aura-components/src/date_time_picker.rs` | ✅ | ✅ DateTime / DateTime range / formats / steps / hide seconds / confirm / disabled / callbacks |
| 5 | Upload | `crates/aura-components/src/upload.rs` | ✅ | ✅ Button / drag style / text list / picture card / progress / status / limit / disabled / callbacks |
| 6 | Cascader | `crates/aura-components/src/cascader.rs` | ✅ | ✅ Multi-level / default selected / disabled / clearable / loading / search results / callbacks |
| 7 | Transfer | — | — | ⬜ |
| 8 | ColorPicker | — | — | ⬜ |
| 9 | Carousel | — | — | ⬜ |
| 10 | Image | — | — | ⬜ |
| 11 | Calendar | — | — | ⬜ |
| 12 | TreeSelect | — | — | ⬜ |
| 13 | Autocomplete | — | — | ⬜ |
| 14 | InputTag | — | — | ⬜ |
| 15 | Mention | — | — | ⬜ |
| 16 | Watermark | — | — | ⬜ |
| 17 | Tour | — | — | ⬜ |
| 18 | Scrollbar | `crates/aura-components/src/scrollbar.rs` | ✅ | ✅ Already completed in P1 |
| 19 | Splitter | `crates/aura-components/src/splitter.rs` | ✅ | ✅ Already completed in P1 |
| 20 | VirtualizedTable/VirtualizedTree | — | — | ⬜ Deferred |

## P6 Built-in Unique ID ⬜

— See `.prompt/P6-builtin-id.md` for task list

## P7 Demo Self-Contained ⬜

— See `.prompt/P7-demo-self-contained.md` for task list

## P8 Engineering ⬜

— See `.prompt/P8-engineering.md` for task list
