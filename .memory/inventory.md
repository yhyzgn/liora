# Component Inventory

## P0 Foundation ✅

| Component | File | Tests | Demo |
|-----------|------|-------|------|
| AuraTheme (light/dark) | `crates/aura-theme/src/lib.rs` | — | — |
| AuraConfig (Global) | `crates/aura-core/src/lib.rs` | — | — |
| AuraContextExt trait | `crates/aura-core/src/lib.rs` | — | — |
| AuraElement trait | `crates/aura-core/src/lib.rs` | — | — |
| Z-Index utils | `crates/aura-core/src/lib.rs` | — | — |
| AuraButton | `crates/aura-components/src/button.rs` | — | ✅ |
| AuraIcon trait | `crates/aura-icons/src/lib.rs` | — | — |
| Gallery app | `apps/aura-gallery/src/` | — | ✅ |

## P1 Basic Elements ⬜ (13)

| # | Component | File | Demo |
|---|-----------|------|------|
| 1 | Button (完善 icon/ghost/group) | `crates/aura-components/src/button.rs` | update |
| 2 | Icon (SVG 集成) | `crates/aura-icons/src/` | new |
| 3 | Link | `crates/aura-components/src/link.rs` | new |
| 4 | Text | `crates/aura-components/src/text.rs` | new |
| 5 | Title | `crates/aura-components/src/title.rs` | new |
| 6 | Paragraph | `crates/aura-components/src/paragraph.rs` | new |
| 7 | Space | `crates/aura-components/src/space.rs` | new |
| 8 | Divider | `crates/aura-components/src/divider.rs` | new |
| 9 | Row (栅格) | `crates/aura-components/src/row.rs` | new |
| 10 | Col (栅格) | `crates/aura-components/src/col.rs` | new |
| 11 | Container | `crates/aura-components/src/container.rs` | new |
| 12 | Scrollbar | `crates/aura-components/src/scrollbar.rs` | new |
| 13 | Splitter | `crates/aura-components/src/splitter.rs` | new |

## P2 Form Controls ⬜ (10)

| # | Component | File |
|---|-----------|------|
| 1 | Input | `crates/aura-components/src/input.rs` |
| 2 | InputNumber | `crates/aura-components/src/input_number.rs` |
| 3 | Textarea | `crates/aura-components/src/textarea.rs` |
| 4 | Checkbox / CheckboxGroup | `crates/aura-components/src/checkbox.rs` |
| 5 | Radio / RadioGroup | `crates/aura-components/src/radio.rs` |
| 6 | Switch | `crates/aura-components/src/switch.rs` |
| 7 | Select | `crates/aura-components/src/select.rs` |
| 8 | Slider | `crates/aura-components/src/slider.rs` |
| 9 | Form / FormItem | `crates/aura-components/src/form.rs` |
| 10 | Rate | `crates/aura-components/src/rate.rs` |

## P3 Popper + Feedback ⬜ (13)

| # | Component | Notes |
|---|-----------|-------|
| — | Popper/Portal 基建 | `crates/aura-core/src/popper.rs` |
| 1 | Tooltip | Popper 依赖 |
| 2 | Popover | Popper 依赖 |
| 3 | Popconfirm | Popper 依赖 |
| 4 | Dialog | FocusTrap 依赖 |
| 5 | Drawer | — |
| 6 | Message | 全局 toast |
| 7 | Notification | 全局通知 |
| 8 | Alert | — |
| 9 | Loading | — |
| 10 | MessageBox | — |
| 11 | Dropdown | Popper 依赖 |
| 12 | Card | — |
| 13 | Collapse | — |

## P4 Nav + Data ⬜ (20)

— See architecture-design.md for full list

## P5 Advanced ⬜ (20)

— See architecture-design.md for full list
