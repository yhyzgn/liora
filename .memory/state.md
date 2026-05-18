# Aura Session State

## Current Phase

**P8 - Native Gallery Documentation** (4 个核心子阶段已完成；后续可进入保留工程化任务：主题切换、搜索、窗口标题、CI/发布文档等；P9 deferred backlog 保持等待明确请求)

## Completed in P4

- ✅ 全部导航组件: Menu, Tabs, Breadcrumb, Steps, PageHeader, Affix, Backtop, Anchor
- ✅ 核心数据展示: Progress, Skeleton, Empty, Result, Descriptions, Timeline, Tree, Pagination, Statistic, Segmented, Tag, Avatar, Badge


## Completed in P6

- ✅ `aura-core` added process-wide atomic unique ID helpers: `next_unique_id()` and `unique_id(prefix)`.
- ✅ Replaced `track_caller` / render-site / literal repeated interactive IDs in high-risk components with component-prefixed runtime unique IDs.
- ✅ Preserved/added `.id(...)` override APIs for migrated components where applicable.

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
| P9 Deferred Advanced | ⏸️ Deferred backlog | 0/9 | 9 |

## Git Status

- Branch: main
- Remote: https://github.com/yhyzgn/aura.git


## Deferred Backlog

- P9 Deferred Advanced created in `.prompt/P9-deferred-advanced.md`.
- Components moved from P5 deferred/skipped scope: Carousel, Calendar, TreeSelect, InputTag, Mention, Watermark, Tour, VirtualizedTable, VirtualizedTree.
- P9 is the latest phase and should be supplemented later only when explicitly requested.

## Current New Phase — P10 Native Charts

User has started a new phase to develop statistics/chart components. P10 is now the active implementation phase, while P9 remains deferred backlog.

Initial technical direction:
- Primary reference: local/current GPUI official source, especially `canvas(...)`, `PathBuilder`, `Window::paint_path`, `Window::paint_quad`, and text rendering primitives.
- Secondary case study: `https://github.com/vicanso/zedis` Metrics implementation, which uses GPUI canvas and a scale/axis/shape split for Area/Line/Bar charts.
- Strict native boundary remains: no HTML/CSS/DOM/WebView/WASM/Web chart runtime.

Expected P10 deliverables:
- Shared chart infrastructure: scale, axis/grid, shapes, legend, tooltip/hover.
- Completed so far: LineChart, AreaChart, BarChart MVPs with Gallery demos, Docs pages, external snippets, and tests.
- Remaining P10 components: PieChart, RingChart, Sparkline, plus tooltip/hover and larger-data performance work.

## Current New Phase — P11 Native Tray / Process Resident

P11 is active after P10 chart rendering work. The project now needs a native system tray facade for GPUI apps.

Technical direction:
- New crate: `crates/aura-tray`.
- Dependencies: `tray-icon` plus `muda` via `tray_icon::menu` re-export; no vendored source by default.
- Required APIs: install from `TrayConfig`, dynamic icon updates, tooltip/visibility updates, checkbox menu state, recursive submenus, stable `TrayCommand` mapping.
- GPUI integration rule: tray-enabled apps must use `QuitMode::Explicit` and keep `AuraTray` alive for process lifetime.
- Demo/docs rule: Gallery and Docs must show rich tray examples (CheckBox, dynamic icons, 2nd/3rd/N-level menus) without creating real OS tray side effects during normal browsing.

### P11 follow-up: real Gallery tray runtime

After user feedback, `aura-gallery` no longer only previews tray config. On native startup it installs a real OS tray icon, stores `AuraTray` in GPUI global state, routes `MenuEvent`/tray click events through a foreground command loop, and handles show/hide/toggle/quit/set-icon/auto-show commands. If tray installation fails, Gallery falls back to `QuitMode::LastWindowClosed` to avoid a resident process without a tray entry.

### P11 follow-up: Gallery and Docs tray demos

Both `aura-gallery` and `aura-docs` now create independent demonstration tray icons on native startup. Gallery uses `aura-gallery`/blue default icon; Docs uses `aura-docs`/purple default icon. Tray menu includes a `resident-enabled` CheckBox for status-bar residency, and app handlers toggle `QuitMode::Explicit` versus `QuitMode::LastWindowClosed` plus tray visibility. Tray docs now include a compile-checked `tray/residency.rs` snippet for page-level residency configuration.

### P11 follow-up: bundled tray icons and in-window controls

`aura-tray` now includes bundled PNG tray icon assets under `crates/aura-tray/assets/tray-icons/` for Gallery and Docs default/syncing/error states. Apps use `bundled_tray_icon(...)` rather than generated solid-color placeholders. `TrayControlCenter` is a GPUI global command bridge so the Tray page buttons in the main window dispatch real tray commands, not just local previews.

### P11 follow-up: close confirmation with remembered choice

Gallery and Docs now intercept window close through GPUI `on_window_should_close`. If `TrayControlCenter.state.remembered_close_action` is `Ask`, a native Dialog asks whether to `关闭进程` or `隐藏到托盘`, with a `记住本次选择` checkbox. Remembered choices are stored in runtime tray control state as `TrayCloseAction::{ExitProcess, HideToTray}`; the Tray demo page can reset to Ask or preselect either behavior.


## Current New Phase — P13 Component Expansion

User requested a new planning phase for additional widgets and customization enhancements. P13 is now planned in `.prompt/P13-component-expansion.md`; implementation has not started yet.

Scope highlights:
- New widgets: QrCode generation/recognition, CodeEditor, SignalMeter, HeatBar, SegmentRatioBar, HorizontalList, Timer, Label, Operation. The user-provided “standalone bar chart” screenshot is interpreted as an in-place BarChart standalone mini mode, not a new FlatBarMeter component.
- Enhancements: RingChart external labels, LineChart per-series stroke style, BarChart standalone mini mode and value range colors, RingProgress gradient/completion color, Button gradient/custom color derived states, Tag flow layout, Radio/Checkbox option customization, vertical list drag.
- Execution is split into five waves: simple meters/bars/layout, chart/progress enhancements, draggable lists, QR/CodeEditor, and form-control deep customization.
- Existing-widget enhancement rule: existing controls must be enhanced in-place in their current source/demo/docs; do not add parallel replacement components for Tag flow, RingProgress gradients, chart style options and BarChart standalone mini mode, Button custom colors, or Radio/Checkbox option customization.

P13 screenshot clarifications:
- HeatBar means a time-axis dense vertical-bar heat chart with top legend/count summary, not a calendar grid heatmap.
- SegmentRatioBar means one horizontal segmented ratio bar with configurable legend/value text placement: top, bottom, both, or hidden; segment labels and percent/value patterns are customizable.

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
- `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets` passed.
- `cargo test -p aura-components` passed: 117 lib tests + integration tests all green.

Remaining P13 scope includes QR generation/recognition, CodeEditor, HorizontalList/item drag, Vertical list drag, RingChart external-label refinements, LineChart per-series style, BarChart interval color docs expansion if needed, RingProgress gradient completion color, Timer, Button gradient/custom derived states, and Radio/Checkbox option customization.

### P13 Wave 2 partial progress — 2026-05-18

Implemented two high-priority in-place enhancements:
- LineChart/ChartSeries now supports per-series line style: `ChartLineStyle::{Solid, Dashed, Dotted}`, `.dashed()`, `.dotted()`, `.solid()`, and custom `.dash_pattern([...])`, while preserving per-series color, stroke width, and smooth toggles. Rendering uses GPUI `PathBuilder::dash_array` through shared chart shape helpers.
- Progress circle/ring now supports gradient rings and `.complete_color(...)`; completed gradient rings can resolve to a specified final color. Gallery/docs/snippets include the ring gradient completion example.

Docs and Gallery were updated:
- `LineChart` page now has a per-line style section with checked snippet `line_chart/line_styles.rs`.
- `Progress` page now has a ring gradient/completion-color section with checked snippet `progress/circle_gradient.rs`.

Validation evidence:
- `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets` passed.
- `cargo test -p aura-components` passed.

### P13 SegmentRatioBar correction — 2026-05-18

User clarified SegmentRatioBar legend/text must be horizontally arranged, not a vertical list. Updated `segment_ratio_bar.rs` so `render_segment_legend` uses a horizontal wrapping flex row (`flex_row` + `flex_wrap` + wider gaps). `split_legend(true)` now splits label/value within each horizontal legend item via `min_w`, instead of stretching each item to a full row. Gallery/docs wording updated to describe horizontal legend text.

Validation evidence:
- `cargo test -p aura-components segment_ratio_bar` passed.
- `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets` passed.

### P13 SegmentRatioBar split legend refinement — 2026-05-18

User clarified each SegmentRatioBar segment text item should split alignment internally: left side is color legend dot + label, right side is ratio/value text (still pattern-customizable). Updated `SegmentRatioBar` so `split_legend(true)` is the default and each horizontal legend item uses a configurable `legend_item_width`, `justify_between`, left legend+label, and right-aligned value/pattern text. Added `legend_item_width(...)` builder for custom per-item width.

Validation evidence:
- `cargo test -p aura-components segment_ratio_bar` passed.
- `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets` passed.

### P13 SegmentRatioBar segment-aligned text correction — 2026-05-18

User clarified the intended layout: for each individual ratio segment, the text block must have the same width and horizontal start/end as that segment. The left legend dot + label aligns to the segment's left edge, and the right value/percent aligns to the segment's right edge. Reworked `render_segment_legend` accordingly: it now renders a full-width horizontal row where each legend text cell uses `gpui::relative(item.value / total)` just like the colored bar segment. Removed fixed `legend_item_width` behavior because it could not align to variable segment boundaries. Pattern customization remains on label/value text.

Validation evidence:
- `cargo test -p aura-components segment_ratio_bar` passed.
- `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets` passed.

### P13 SegmentRatioBar text inset — 2026-05-18

Added configurable left/right text inset for SegmentRatioBar's segment-aligned legend cells. New builders: `legend_inset_x(Pixels)` and alias `legend_text_inset(Pixels)`. The inset applies inside each proportional segment text cell, preserving alignment to the segment boundaries while avoiding text touching segment edges. Gallery and docs snippets now demonstrate non-default inset values.

Validation evidence:
- `cargo test -p aura-components segment_ratio_bar` passed.
- `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets` passed.

### P13 SegmentRatioBar radius controls — 2026-05-18

Added separate radius controls for SegmentRatioBar: existing `radius(...)` configures the overall bar container radius, and new `segment_radius(...)` / alias `rounded_segments(...)` configures each colored segment's own radius. This supports both whole-bar rounding and per-segment rounding while preserving segment-aligned text cells and text inset behavior. Gallery and docs snippets now demonstrate both levels of rounding.

Validation evidence:
- `cargo test -p aura-components segment_ratio_bar` passed.
- `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets` passed.

### P13 SignalMeter total/count and per-level colors — 2026-05-18

Enhanced `SignalMeter` with explicit total signal count aliases and per-level active colors. Existing `max_level(...)` remains; new `total_signals(...)` and `signal_count(...)` aliases configure total bars. New `level_colors(...)` / `signal_colors(...)` lets callers assign different active colors for each signal level; inactive bars still use `inactive_color(...)`. Gallery and docs now include total-count/per-level-color examples.

Validation evidence:
- `cargo test -p aura-components signal_meter` passed.
- `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets` passed.

### P13 docs/demo coverage standard — 2026-05-18

User clarified that every new component and future new component must have Gallery and Docs examples covering the major style/configuration combinations, not just one happy-path example. Applied immediately to SegmentRatioBar: Gallery and Docs now cover bottom legend, top legend, both top+bottom legends, hidden legend, custom label/value pattern, compact thin bar, overall radius, per-segment radius, text inset, split legend, and percentage precision.

Validation evidence:
- `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets` passed.
- `cargo test -p aura-components segment_ratio_bar` passed.

### P13 SignalMeter threshold-wide colors — 2026-05-18

User clarified that per-level colors also need a threshold-wide mode: when the current signal reaches a configured level, all active bars use one unified color for that current level (e.g. level 2 = red, 3 = yellow, 4 = orange, 5 = green). Kept the existing per-bar `level_colors(...)` / `signal_colors(...)` behavior and added `SignalLevelColor`, `threshold_colors(...)`, `level_threshold_colors(...)`, and incremental `level_color(level, color)`. Rendering prioritizes threshold-wide color over per-bar level colors when a matching threshold exists. Gallery and Docs now include threshold-wide examples.

Validation evidence:
- `cargo test -p aura-components signal_meter` passed.
- `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets` passed.

### P13 Timer component — 2026-05-18

Implemented new `Timer` component in `crates/aura-components/src/timer.rs`. It is a controlled display component for count-up/count-down timers, with `TimerDirection`, `TimerUnit`, `TimerSnapshot`, `count_up`, `count_down`, `display_unit`, `show_unit`, `prefix`, `suffix`, `compact`, `snapshot`, `elapsed_as`, and `remaining_as`. Countdown remaining time saturates at zero and exposes `finished`. Gallery and Docs now include count-up, count-down, unit/compact, and result-reading examples. This follows the new docs/demo coverage standard for newly added components.

Validation evidence:
- `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets` passed.
- `cargo test -p aura-components timer` passed.
- `cargo test -p aura-gallery timer_demo_uses_timer_api` passed.

### P13 Timer clock format — 2026-05-18

Enhanced Timer with clock-style formatting for `00:00:00` / `HH:MM:SS`. Added `TimerFormat::{Unit, Clock}`, `Timer::format(TimerFormat)`, `Timer::clock_format()`, and public `format_clock(Duration)`. Gallery and Docs now include a clock-format section and checked snippet `timer/clock.rs`.

Validation evidence:
- `cargo test -p aura-components timer` passed.
- `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets` passed.
- `cargo test -p aura-gallery timer_demo_uses_timer_api` passed.

### P13 Button gradient/custom color enhancement — 2026-05-18

Enhanced existing `Button` in-place with custom color and gradient styling:
- Added `ButtonColors` for fully custom solid/outline button colors, including explicit base/hover/active/text/border/disabled slots.
- Added `.custom_color(bg, text)`, `.colors(ButtonColors)`, and `.custom_colors(ButtonColors)` builders.
- Added `ButtonGradient` plus `.gradient(from, to)` and `.gradient_with_angle(angle, from, to)` builders.
- Hover, active/clicked, and disabled states are automatically derived for simple custom colors and gradients, while preserving the existing theme variants by default.
- Gallery `Button` demo and Docs `button.md` now show custom solid/outline/disabled and gradient/loading/disabled examples with compile-checked snippets.

Validation evidence:
- `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets` passed.
- `cargo test -p aura-components button` passed.
- `cargo test -p aura-gallery button_demo_uses_aura_layout_primitives` passed.

### P13 Radio/Checkbox option style customization — 2026-05-18

Enhanced existing `CheckboxGroup` and `RadioGroup` in-place with option-level layout and selected-style customization:
- Added `CheckboxOptionStyle` and `RadioOptionStyle` builders for option background, selected background, hover background, text/selected text colors, border/selected border colors, radius, padding, gap, indicator visibility, and selected icon/dot visibility.
- Added `.option_style(...)` and `.card_options()` to both group components.
- Non-button vertical/horizontal groups now render styled option cards/chips when option style is configured; default rendering remains unchanged.
- Button-style groups also honor selected/background/text/border/gap/padding/icon options where applicable.
- Gallery Form Controls demo and Docs `checkbox.md` / `radio.md` now include card-like and chip-like custom option examples with compile-checked snippets.

Validation evidence:
- `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets` passed.
- `cargo test -p aura-components option_style` passed.
- `cargo test -p aura-gallery form_controls` completed with no failures.


### P13 QrCode generation and recognition — 2026-05-18

Added new `QrCode` component in `crates/aura-components/src/qr_code.rs` using pure Rust dependencies (`qrcode` for generation and `rqrr` for recognition). Capabilities:
- Native GPUI-rendered QR display via generated `RenderImage`, with configurable size, quiet zone, foreground/background colors, and error-correction level (`QrEcLevel`).
- Public generation helpers: `encode_matrix(...)` and `render_image(...)`.
- Recognition helpers: `decode_bytes(...)`, `decode_file(...)`, and `decode_image(...)`, returning `QrDecoded { content, ecc_level, version }`.
- Gallery demo added as `QrCode 二维码`; Docs page `qr_code.md` added with basic, style/ECC, and recognition API snippets.

Validation evidence:
- `cargo test -p aura-components qr_` passed, including a generated-image decode round trip.
- `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets` passed.

### P13 QrCode interactive demo refinement — 2026-05-18

Updated QrCode demos/docs to meet the interaction requirement:
- Gallery QrCode demo now includes an input field and `生成二维码` button; clicking updates the displayed QR code from the current string.
- Gallery QrCode demo now includes a local image path input and `识别图片` button; clicking calls `QrCode::decode_file(...)` and displays success/failure text plus toast feedback.
- Docs QrCode page now uses the full interactive Gallery demo for the effect area, and snippets show complete interactive generation and local-file recognition patterns.

Validation evidence:
- `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets` passed.
- `cargo test -p aura-components qr_` passed.

## 2026-05-18 P13 QrCode style/upload refinement
- QrCode generation now supports screenshot-like styles: square/rounded/dot modules, square/rounded/circle finder styles, high-recovery center logo badge, corner mini badge, custom foreground/background/logo colors, and logo size ratio.
- QrCode recognition demos/docs now use Aura Upload to open local image files instead of typing paths; selected file is decoded with QrCode::decode_file and result is shown in the page plus toast feedback.
- Validation: cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets; cargo test -p aura-components qr_.

Update: QrCode also gained generic logo(...) and corner_logo(...) builders accepting any GPUI element, in addition to logo_text/corner_logo_text convenience APIs, so callers can render images/icons/custom badges in QR overlays. Validation rerun after this API: cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets; cargo test -p aura-components qr_.

## 2026-05-18 P13 QrCode recognition/result and social styles refinement
- QrCode recognition examples now show the decode result persistently in an on-page result box; toast remains only supplemental feedback.
- Corrected social QR styling direction by adding `QrPatternStyle::{Matrix, MiniProgram, Douyin}` with radial rendering for mini-program-like and Douyin-like codes instead of rendering them as ordinary dot-matrix QR only.
- Added builders: `pattern_style(...)`, `matrix_style()`, `mini_program_style()`, `douyin_style()`, and `douyin_badge()`; `mini_program_badge()` now uses the radial mini-program preset.
- Gallery and Docs style demos now show normal QR, mini-program style, Douyin style, and custom-logo rounded QR.
- Validation: `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets`; `cargo test -p aura-components qr_`.

## 2026-05-18 P13 QrCode social style rewrite
- Rewrote MiniProgram/Douyin QR style rendering after screenshot feedback showed the previous polar matrix transform looked like noisy broken QR fragments.
- New social style renderer samples encoded QR content but renders clean radial capsules/dots with deterministic thinning, skips QR finder squares, and draws explicit social-code locator dots plus Douyin-style outer arcs.
- Validation: `cargo test -p aura-components qr_`; `cargo check -p aura-gallery -p aura-docs --bin check_snippets`.

## 2026-05-18 P13 QrCode social presets second rewrite
- User clarified the previous social-code output still did not resemble the reference images. Replaced content-matrix polar module plotting with visual-template renderers: MiniProgram now uses sunburst radial capsules/dots plus three locator circles; Douyin now uses segmented circular tracks, sparse radial texture, three locator circles, and bold outer arcs.
- The render remains deterministic per encoded content via a visual seed, but intentionally prioritizes the requested social-code style instead of QR-matrix readability.
- Validation: `cargo test -p aura-components qr_`; `cargo check -p aura-gallery -p aura-docs --bin check_snippets`.

## 2026-05-18 P13 QrCode social presets removed, gradient foreground added
- Removed failed MiniProgram/Douyin social-code style APIs and render branches per user request; no `QrPatternStyle`, `mini_program_*`, or `douyin_*` API remains.
- Added QR foreground gradient support with color arrays and eight directions via `QrGradientDirection::{ToTop, ToTopRight, ToRight, ToBottomRight, ToBottom, ToBottomLeft, ToLeft, ToTopLeft}`.
- New builders: `gradient(colors, direction)`, `foreground_gradient(colors, direction)`, `gradient_colors(colors)`, and `gradient_direction(direction)`. Calling `foreground(...)` clears gradient and restores solid color behavior.
- Gallery and Docs QrCode style examples now show gradient QR variants instead of removed social-code presets.
- Validation: `cargo test -p aura-components qr_`; `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets`.

## 2026-05-18 P13 RingChart external legend enhancement
- Enhanced existing `RingChart` in-place with fully external legend/value display: `RingExternalLegendOptions`, `RingExternalLegendLayout::{Vertical, Horizontal}`, `external_legend(...)`, `external_vertical_legend()`, `external_horizontal_legend()`, `external_legend_content(...)`, and `external_legend_percentage_decimals(...)`.
- External legend mode disables inline chart labels and normal legend, avoiding leader lines and putting all label/value/percentage text into a vertical or horizontal legend area.
- Gallery and Docs now include external legend examples; docs snippet `ring_chart/external.rs` is compile-checked.
- Validation: `cargo test -p aura-components ring_chart`; `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets`.

## 2026-05-18 P13 RingChart external vertical side and item limit
- Enhanced RingChart external legend mode so vertical legends are rendered beside the chart instead of below it. Added `RingExternalLegendSide::{Left, Right}`, `external_legend_side(...)`, `external_legend_left()`, and `external_legend_right()`.
- Added `max_items(...)` on `RingExternalLegendOptions` and `external_legend_max_items(...)` on `RingChart` to show only the first N non-zero slices.
- Gallery and Docs now demonstrate a right-side vertical external legend limited to the first 3 items, plus horizontal external legend coverage.
- Validation: `cargo test -p aura-components ring_chart`; `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets`.

## 2026-05-18 P13 RingChart external vertical layout fix
- Fixed vertical external legend layout regression where the legend consumed full row width and hid/squeezed the chart. Vertical legend now has fixed side width and `flex_none`, while the chart container uses `flex_1().min_w(0)`.
- Validation: `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets`; `cargo test -p aura-components ring_chart`.

## 2026-05-18 P13 RingChart side legend spacing tightening
- Tightened RingChart vertical external legend placement so text sits next to the chart instead of far away: reduced side-layout gap, narrowed vertical legend width, and slightly reduced side-layout canvas height to remove excessive empty horizontal/vertical space.
- Validation: `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets`; `cargo test -p aura-components ring_chart`.

## 2026-05-18 P13 Timer live ticking

Enhanced `Timer` from a static controlled display into an optional live ticking component while preserving the controlled API. `Timer::start()` / `.running(true)` now registers a native GPUI refresh runtime, uses stable `id(...)` values to preserve each timer's start instant across renders, and supports count-up, count-down, and `00:00:00` clock displays that continue updating in Gallery and Docs. Docs live demos and checked snippets now use `.start()` for interactive timer examples.


## 2026-05-18 P13 HorizontalList component

Implemented `HorizontalList` in `crates/aura-components/src/horizontal_list.rs` as a native horizontal scroll list with custom item renderer, custom divider renderer, internal order state, drag-to-reorder interaction, and `on_reorder(from_index, to_index, ...)` callback. Added Gallery demo sections for base horizontal cards, custom arrow divider, and draggable reorder with toast feedback. Added Docs page `horizontal_list.md` and compile-checked snippets for basic/divider/draggable usage.

Validation evidence:
- `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets` passed.
- `cargo test -p aura-components horizontal_list` passed.
- `cargo test -p aura-gallery horizontal_list_demo` passed.


## 2026-05-18 P13 VirtualizedList drag reorder

Enhanced existing `VirtualizedList` in-place with optional vertical drag reorder. The component now keeps an internal item order, renders original item indices through that order, supports `set_draggable(true)`, exposes `set_on_reorder(from_index, to_index, ...)`, and remeasures after reorder without storing `AnyElement` across frames. Gallery and Docs now include a vertical drag sorting example plus compile-checked snippet `virtualized_list/draggable.rs`.

Validation evidence:
- `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets` passed.
- `cargo test -p aura-components virtualized_list` passed.
- `cargo test -p aura-gallery virtualized_list_demo` passed.


## 2026-05-18 Drag reorder handle correction

Corrected HorizontalList and VirtualizedList drag UX after feedback that invisible whole-item dragging was not acceptable. Dragging now starts only from an explicit front-side `GripVertical` handle rendered before each draggable item/row, while hover/drop detection stays on the item shell. Gallery and docs wording now points users to the visible drag handle.


## 2026-05-18 Drag reorder live hover fix

Fixed reorder interaction after testing feedback: drag handles are now full-height flex boxes so the Grip icon is centered, and dragging reorders immediately when the pointer moves over a target item/row instead of waiting for final mouse-up delivery. This avoids lost drops when GPUI mouse-up is delivered to the original drag handle instead of the hovered item.


## 2026-05-18 Generic draggable helper and follow-pointer list motion

Added reusable `draggable` module inspired by drag-rs' operation model (start point, current pointer, result/reorder callback) while staying pure GPUI/native. The module provides `DragState`, `DragAxis`, default `drag_handle`, and shared `reorder_indices` helpers so future controls can add handle-based dragging without duplicating pointer bookkeeping or storing rendered elements. HorizontalList and VirtualizedList now use this module and apply axis-specific margin offsets to the active item, producing a native follow-pointer drag motion plus live reorder-on-hover.

Validation evidence:
- `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets` passed.
- `cargo test -p aura-components draggable` passed.
- `cargo test -p aura-components horizontal_list` passed.
- `cargo test -p aura-components virtualized_list` passed.
- `cargo test -p aura-gallery horizontal_list_demo` passed.
- `cargo test -p aura-gallery virtualized_list_demo` passed.


## 2026-05-18 Drag follow-pointer positioning fix

Corrected draggable follow-pointer rendering: the previous implementation used margin offsets (`ml`/`mt`), which changed layout and created empty space but did not visually move the dragged item as a floating object. HorizontalList and VirtualizedList now apply `relative().left(dx).top(dy)` with shadow while active, so the item is visually offset along the drag axis without using margin-based layout movement.

Validation evidence:
- `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets` passed.
- `cargo test -p aura-components draggable` passed.
- `cargo test -p aura-components horizontal_list` passed.
- `cargo test -p aura-components virtualized_list` passed.


## 2026-05-18 Drag reorder stability correction

Fixed the follow-pointer drag instability where the dragged element jumped and then appeared to run away. Root cause: reordering during hover changed the dragged item layout slot while offsets were still computed from the original pointer anchor. Dragging now keeps the original order during movement, only updates the over/target index and pointer offset, and performs the actual reorder once on mouse-up/out using the last hovered target. This keeps the active element following the pointer from its original slot instead of recalculating against a moving slot.

Validation evidence:
- `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets` passed.
- `cargo test -p aura-components draggable` passed.
- `cargo test -p aura-components horizontal_list` passed.
- `cargo test -p aura-components virtualized_list` passed.
- Follow-up correction: root list containers now also track mouse movement while the left button is pressed, and active item hover no longer overwrites the drop target. This prevents the translated active item from stealing hover events and making the target/offset look random.
- Additional validation: `cargo test -p aura-gallery horizontal_list_demo` passed; `cargo test -p aura-gallery virtualized_list_demo` passed.

## 2026-05-18 Drag reorder top-layer and live slot preview

Adjusted draggable list behavior so the active dragged row/card is painted above siblings with GPUI deferred drawing priority while preserving its layout participation. Hovering another item now performs a live visual reorder so surrounding items give way immediately; DragState keeps the original position for the final callback and resets the pointer anchor when the active slot changes to avoid runaway offsets.

Validation evidence:
- `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets` passed.
- `cargo test -p aura-components draggable` passed.
- `cargo test -p aura-components horizontal_list` passed.
- `cargo test -p aura-components virtualized_list` passed.
- `cargo test -p aura-gallery horizontal_list_demo` passed.
- `cargo test -p aura-gallery virtualized_list_demo` passed.
