# Aura Session State

## Current Phase

**P15 Quality Hardening — Complete (2026-06-18)**

Local implementation phases are complete through P14; P15 is complete for local release-quality hardening:

- P10 Native Charts: complete, including downsampling plus Line/Area/Bar/Pie/Ring hover hit testing.
- P11 Native Tray: complete, including `aura-tray`, dynamic icons, nested/check menus, Gallery/Docs controls, and close-to-tray behavior.
- P12 Native Packaging: repository-owned release readiness complete, including `release-readiness` gate, explicit `LicenseRef-Aura` policy, signing/notarization policy docs, CI dry-run gate, and strict `v*` release gate. Real credentials and destructive system-level installs remain protected-environment responsibilities.
- P13 Component Expansion: implemented and documented.
- P14 Deferred Advanced: complete; the P9 backlog has been migrated and delivered.
- P15 Quality Hardening: complete; Track A CI gates, Track B API consistency/panic cleanup, Track C visual/theme token hardening, Track D overlay/keyboard close-policy coverage, Track E CodeBlock/cache performance hardening, and Track F docs/snippet completeness all passed the final local gate suite.
- P16 Public API & Adoption Readiness: complete; root README, contributing/changelog docs, minimal app workspace example, crate-level Rustdoc, Docs Adoption Guide, and adoption regression tests are in place.

P12 external-policy items remain tracked but do not block local P15 hardening work. Do not mark P12 fully complete until signing/notarization, real system installs, license policy, and real `v*` release validation are satisfied or formally declared out of scope.




## P16 Adoption Readiness — 2026-06-18

P16 is complete. Added root `README.md`, `CONTRIBUTING.md`, `CHANGELOG.md`, `examples/minimal-app`, crate-level Rustdoc entrypoints, native Docs `Adoption Guide`, and regression coverage for adoption docs/workflow wiring. The minimal app is a workspace member and compiles with `cargo check -p aura-minimal-app`. `cargo doc --workspace --no-deps` passes. Aura remains pure Rust + GPUI native; `LicenseRef-Aura` remains explicit until owner selects formal license terms.

## P12 Final Closure — 2026-06-18

P12 native packaging is complete for repository-owned scope. Added `cargo run -p xtask -- package release-readiness`, `LICENSE.md` with explicit `LicenseRef-Aura`, `packaging/signing-policy.md`, CI non-strict readiness checks, and package workflow strict `v*` release readiness checks. The release path now blocks missing macOS/Windows signing inputs when `AURA_REQUIRE_SIGNING=true` instead of silently publishing unsigned formal releases. Real signing credentials, notarization accounts, protected runner policy, destructive install/uninstall execution, and real public tag publishing remain owner-controlled release-environment actions, not missing local implementation.

## P15 Final Completion Audit — 2026-06-18

P15 local quality hardening is complete. Final gate evidence:

- `cargo fmt --all --check` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `cargo check -p aura-docs --bin check_snippets` passed.
- `cargo run -p xtask -- package validate` passed.
- `cargo run -p xtask -- package ci --all-apps --format platform-defaults --dry-run --skip-build` passed.
- `cargo run -p xtask -- package install-smoke --all-apps --format platform-defaults --dry-run` passed.
- `git diff --check -- . ':(exclude).omx'` passed.
- Gallery and Docs GUI startup smoke both started and exited via expected `timeout 10s` status `124`.

Non-blocking residuals: `MessageManager::init` panic is intentional usage-contract enforcement; Gallery fixed date/time demo `expect(...)` calls are demo constant assumptions; P12 external-policy items remain signing/notarization, real install/uninstall, formal license policy, and real `v*` release validation.

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
| P9 Deferred Advanced | ✅ Migrated to P14 | 9/9 | 9 |
| P10 Native Charts | ✅ Done | 7/7 | 7 |
| P11 Native Tray | ✅ Done | 1/1 | 1 |
| P12 Native Packaging | ✅ Done | release readiness gate done | external credentials/protected runners gated |
| P13 Component Expansion | ✅ Done | 18/18 | 18 |
| P14 Deferred Advanced | ✅ Done | 9/9 | 9 |
| P15 Quality Hardening | ✅ Done | final gate passed | CI gates + API consistency + visual/theme + overlay behavior + CodeBlock performance + docs completeness |
| P16 Adoption Readiness | ✅ Done | adoption gate passed | README + examples/minimal-app + Rustdoc + Docs Adoption Guide |

## Git Status

- Branch: main
- Remote: git@github.com:yhyzgn/aura.git


## Deferred Backlog

- P9 Deferred Advanced was migrated into P14 and is complete.
- Delivered components: Carousel, Calendar, TreeSelect, InputTag, Mention, Watermark, Tour, VirtualizedTable, VirtualizedTree.
- Do not reopen these as deferred backlog unless a new user request changes their requirements.

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

## Historical Phase — P11 Native Tray / Process Resident

P11 native system tray / process resident support is complete for GPUI apps.

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

Final P13 status is implemented; see `.memory/inventory.md` for the completed component matrix and `.prompt/P13-component-expansion.md` for the maintenance contract.

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

## 2026-06-16 P13 docs navigation cleanup

Split the combined `LabelOperation` docs surface into separate `Label` and `Operation` pages so each P13 component is independently discoverable in aura-docs. Added dedicated compile-checked snippets under `content/snippets/label/basic.rs` and `content/snippets/operation/basic.rs`, wired both snippets into `check_snippets`, and updated the docs page registry. Also refreshed `.memory/inventory.md` to mark CodeEditor, RingChart external labels, and BarChart value range colors as implemented based on current source/docs coverage.

Validation evidence:
- `cargo check -p aura-docs --bin check_snippets` passed.
- `cargo check -p aura-docs` passed.

## 2026-06-16 P13 gallery navigation cleanup

Split the combined Gallery `LabelOperation` demo into independent `Label` and `Operation` demo entries to match the docs split. `Label` now demonstrates basic icons, semantic colors, spacing, sizing, and custom icon elements. `Operation` now demonstrates Switch/Button actions, status labels/colors, disabled rows, and compact no-padding rows. The old combined gallery module was removed so P13 components are independently searchable in both Gallery and Docs.

Validation evidence:
- `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets` passed.
- `cargo test -p aura-gallery label_demo` passed.
- `cargo test -p aura-gallery operation_demo` passed.

## 2026-06-16 P13 plan status refresh

Updated `.prompt/P13-component-expansion.md` from planned/waiting status to implemented/maintenance status, checked off all five implementation waves, and added a current implementation snapshot. Updated `prompt.md` so the top-level project prompt no longer describes P13 as merely planned.

Validation evidence:
- `cargo check -p aura-components -p aura-gallery -p aura-docs --bin check_snippets` passed before the status-only documentation update.

## 2026-06-16 P10 Sparkline completion

Audited recent work logs after the user recalled unfinished component supplementation. Found P13 was implemented, but P10 chart inventory still had a real missing `Sparkline` component while PieChart/RingChart were implemented but marked planned. Added native `Sparkline` to `aura-components`, Gallery, Docs, and compile-checked snippets. Updated P10 inventory to reflect PieChart/RingChart/Sparkline status.

Sparkline capabilities:
- Compact native GPUI canvas/path rendering for metric cards, table cells, and dashboards.
- Trend-aware positive/negative colors, custom color, area fill, 0 baseline, fixed y-domain, smooth/straight lines, solid/dashed/dotted style, custom dash pattern, and optional last-point marker.

Validation evidence:
- `cargo test -p aura-components sparkline` passed.
- `cargo test -p aura-gallery sparkline_demo` passed.
- `cargo check -p aura-docs --bin check_snippets` passed.

## 2026-06-16 P10 chart downsampling performance pass

Implemented the first P10 performance review item for native chart rendering. Added shared min/max bucket downsampling in `chart.rs` and wired it into `LineChart`, `AreaChart` (overlay and stacked paths), and `Sparkline`. New public builders: `max_render_points(...)` and `disable_downsampling()` on LineChart/AreaChart/Sparkline. The strategy preserves first/last points plus local extrema so long monitoring series keep spikes while bounding GPUI path complexity. Gallery and Docs now include downsampling examples for LineChart, AreaChart, and Sparkline with compile-checked snippets.

Remaining P10 maintenance work: hover/tooltip hit testing and any further large-data cache policy beyond draw-point downsampling.


## 2026-06-16 P14 Wave 1 deferred advanced components

Promoted the old P9 deferred backlog into the active P14 Deferred Advanced phase and implemented the first batch of high-value advanced controls:
- `Carousel`: native carousel item model, indicator placement, arrow visibility, direction/autoplay configuration, and custom content slot.
- `Calendar`: month grid, selected date, range highlighting, disabled dates, event labels, and selection callback.
- `InputTag`: `Input` + `TagFlow` composition with Enter-to-add, closable tags, max tag limit, duplicate policy, and change callback.

Added Gallery demos, Docs pages, external compile-checked snippets, and updated `.prompt/P14-deferred-advanced.md`. Remaining P14 backlog: TreeSelect, Mention, Watermark, Tour, VirtualizedTable, VirtualizedTree.


## 2026-06-16 P14 Wave 2 mention and watermark

Implemented two additional deferred advanced controls:
- `Mention`: an Input-backed mention field with configurable trigger character, candidate filtering, max suggestions, disabled state, and select callback. It follows the existing Input/Autocomplete composition direction instead of reimplementing text input.
- `Watermark`: a native wrapper for text watermarks over arbitrary GPUI content, with cover/header/footer placement, density, gap, opacity, color, and rotation configuration metadata.

Added Gallery demos, Docs pages, and compile-checked snippets for both controls. Remaining P14 backlog: TreeSelect, Tour, VirtualizedTable, VirtualizedTree.


## 2026-06-16 P14 Wave 3 TreeSelect

Implemented `TreeSelect` as the next deferred advanced control. It supports hierarchical nodes, single and multiple selection, default selected keys, disabled keys, filterable search, selected label flattening, and selection callbacks. Added Gallery demo, Docs page, compile-checked snippets, and focused tests for tree filtering/flattening helpers. Remaining P14 backlog: Tour, VirtualizedTable, VirtualizedTree.


## 2026-06-17 P14 Wave 4 Tour

Implemented `Tour` as a controlled native step-guide component with step list, active index, target labels, placement metadata, progress/mask switches, previous/next/finish/close callbacks, Gallery demo, Docs page, compile-checked snippets, and focused navigation tests. Remaining P14 backlog: VirtualizedTable and VirtualizedTree.


## 2026-06-17 P14 Wave 5 VirtualizedTable

Implemented `VirtualizedTable` as a fixed-header large-data table that reuses `TableColumn` definitions and GPUI `ListState` to render visible rows only. Cells are generated from row index + column key each frame to avoid stale GPUI element caching. Added Aura scrollbar, height/row-height/overdraw configuration, stripe/border/loading/empty states, sorting callback, Gallery demos, Docs page, compile-checked snippets, and focused tests. Remaining P14 backlog: VirtualizedTree.


## 2026-06-17 P14 Wave 6 VirtualizedTree

Implemented `VirtualizedTree` as the final P14 deferred advanced control. It virtualizes large hierarchical datasets by flattening the currently expanded tree into lightweight visible-node metadata and rendering visible rows via GPUI `ListState`, with Aura scrollbar, expand/collapse, single/multiple selection, checkbox mode, default expanded/selected keys, callbacks, Gallery demos, Docs page, compile-checked snippets, and focused tests. P14 backlog is complete.


## 2026-06-17 P12 install/uninstall smoke plan

Added `cargo run -p xtask -- package install-smoke ...` as a runner-safe install/uninstall readiness gate. The command defaults to plan-only mode: it reuses package artifact discovery and smoke validation, prints per-format install / launch-smoke / uninstall commands, and writes `target/packages/install-smoke-plan.md` for CI artifacts. `--execute-install` is intentionally restricted to portable `.tar.gz`, where it extracts to `target/install-smoke/<package>`, verifies launcher + `bin/<binary>`, then removes the directory. CI now runs the plan-only install/uninstall smoke gate after artifact smoke and before artifact upload. Remaining P12 blockers are external/policy-heavy: signing/notarization, real system package install/uninstall execution on dedicated runners, release tag validation, and license policy finalization.

## 2026-06-17 P10 Cartesian chart hover hit testing

Completed a concrete P10 maintenance slice for chart hover behavior. Added shared pure cartesian hit-testing helpers (`ChartHitPoint`, `nearest_cartesian_hit_point`, `format_hit_tooltip`) plus a reusable `ChartBoundsTracker` that records canvas bounds without storing frame-local GPUI elements. `LineChart` now exposes and uses `.show_tooltip(...)` / `.tooltip_hit_radius(...)` for native hover tooltips. `AreaChart` exposes the same API and enables tooltip hit testing for Overlay mode; Stacked mode intentionally avoids false cartesian hit reporting until a cumulative-layer hit model is added. Gallery and Docs/snippets now surface tooltip radius and disabled-tooltip examples. Remaining optional P10 tooltip polish: BarChart rectangular hit testing and Pie/Ring polar sector hit testing.

## 2026-06-17 P10 BarChart hover hit testing

Completed the next chart tooltip polish slice for `BarChart`. Added tested grouped and stacked rectangular hit-box geometry (`BarChartHitBox`, `bar_chart_hit_boxes`, `nearest_bar_chart_hit_point`) and wired it into the native hover tooltip portal. Grouped mode hits individual side-by-side bars; stacked mode hits the concrete segment inside a stacked column. Gallery and Docs/snippets now show tooltip radius and disabled-tooltip examples. Remaining optional P10 tooltip polish: PieChart/RingChart polar sector hit testing and any further large-data cache policy.

## 2026-06-17 P10 Pie/Ring polar chart hover hit testing

Completed the remaining chart tooltip slice for `PieChart` and `RingChart`. Added pure polar-sector hit-testing helpers and wired native hover tooltip support into both charts. `PieChart` hits rendered sectors; `RingChart` hits only donut segments and excludes the inner hole. Public builders `show_tooltip(...)` and `tooltip_hit_radius(...)` are now documented in Gallery, Docs live demos, and compile-checked snippets. Remaining P10 maintenance item: any further cache policy beyond existing downsampling.

## 2026-06-17 P12 install-smoke dry-run readiness

Fixed `xtask package install-smoke --dry-run` so plan-only mode no longer requires real backend artifacts or scans stale `target/packages` files. Dry-run now derives expected artifact paths per app/platform/format and writes install/uninstall plans; non-dry-run still discovers and smokes real artifacts, while `--execute-install` remains restricted to portable `.tar.gz`. Validation passed: `cargo check -p xtask -p aura-packager`, `cargo test -p aura-packager`, `cargo test -p xtask install_smoke -- --nocapture`, `cargo run -p xtask -- package validate`, `cargo run -p xtask -- package ci --all-apps --format platform-defaults --dry-run --skip-build`, `cargo run -p xtask -- package install-smoke --all-apps --format platform-defaults --dry-run`, `cargo fmt --all --check`, and `git diff --check`.

## 2026-06-17 phase readiness documentation sync

Synchronized architecture and inventory records with current evidence: P10 native charts are complete with downsampling and Line/Area/Bar/Pie/Ring hover hit testing; P14 deferred advanced backlog is complete; P12 remains in readiness with local runner-safe packaging gates and external-policy work for signing/notarization/real system installs/license.

## 2026-06-17 P12 release tag validation

Added GitHub Actions release tag validation in the package workflow. `v*` release builds now require `vX.Y.Z` and the tag version must match `crates/aura-packager/Cargo.toml`; this prevents prerelease/mismatched tags from reaching package backends such as Windows MSI that require numeric versions. Updated P12 technical plan and prompt handoff docs to distinguish completed preview packaging from remaining real `v*` release-runner/signing/system-install policy work.

## 2026-06-17 phase handoff stale-state cleanup

Updated the handoff state so new sessions no longer start from the obsolete P8/P9-era “current phase” text. Current source-of-truth summary: P10/P11/P13/P14 are complete; P12 has local runner-safe packaging readiness implemented and only external-policy items remain. Repository remote is SSH (`git@github.com:yhyzgn/aura.git`).

## 2026-06-17 P15 quality hardening kickoff

Started P15 as the release-quality hardening phase after P13/P14 completion and P12 runner-safe readiness. First concrete slice: add `.github/workflows/ci.yml` as a general quality gate for fmt, workspace check/test, docs snippet check, packaging validate, packaging dry-run, and install-smoke dry-run.

## 2026-06-18 P15 Track D Preview close-policy docs/examples

Preview outside-click close policy is now discoverable in Docs, compile-checked snippets, live docs demos, and Gallery. The representative example disables both ESC and outside-click dismissal for controlled overlay flows. Validation passed: fmt, docs snippet check, workspace check/test, diff whitespace check, and Gallery/Docs GUI startup smoke.

## 2026-06-18 P15 Track D Tour close-policy docs/examples

Tour overlay close-policy behavior is now covered in source-level tests, Gallery, Docs, compile-checked snippets, and the docs snippet loader. The controlled-close example disables both ESC and outside-click dismissal for critical guided flows. Validation passed: fmt, focused Tour tests, docs snippet check, workspace check/test, diff whitespace check, and Gallery/Docs GUI startup smoke.

## 2026-06-18 P15 Track A CI/package workflow boundary docs

Packaging Workflow docs now explicitly separate `.github/workflows/ci.yml` quality gates from `.github/workflows/package.yml` native packaging/release responsibilities. CI is documented as validation/dry-run only; package workflow owns platform-specific packaging, raw binaries, artifacts, changelog, and `v*` GitHub Release publishing. A docs regression test locks this boundary. Validation passed: fmt, focused docs test, docs snippet check, workspace check/test, diff whitespace check, and Gallery/Docs GUI startup smoke.

## 2026-06-18 P15 Track F docs snippet loader completeness

Docs UI snippet loading is now complete for all authored docs page `src="..."` code blocks. Fixed 22 loader gaps for snippets that were present and compile-checked but not displayable in Docs. Added a regression test that parses every docs page and asserts each referenced snippet resolves through `load_code_snippet`. Validation passed: fmt, focused docs loader test, docs snippet check, workspace check/test, diff whitespace check, and Gallery/Docs GUI startup smoke.

## 2026-06-18 P15 Track A split CI quality and packaging dry-run jobs

General CI now has separate `rust-quality` and `packaging-dry-run` jobs. Workspace fmt/check/test/snippet checks keep GPUI/Linux native dependencies; package metadata/dry-run/install-smoke dry-run executes in a lightweight job with only `file` tooling. Packaging Workflow docs and docs tests lock the job split. Validation passed: workflow YAML parse, fmt, focused docs packaging tests, docs snippet check, workspace check/test, xtask package validate/dry-run/install-smoke dry-run, diff whitespace check, and Gallery/Docs GUI startup smoke.

## 2026-06-18 P15 Track F QuickStart key binding completeness

QuickStart minimal window setup now registers CodeEditor and Tour key bindings in addition to existing input, selection, preview, popup, and typography bindings. A docs regression test compares QuickStart against Gallery and Docs for key bindings that affect text selection, code editing, Preview, and Tour overlay behavior. Validation passed: fmt, focused docs test, docs snippet check, workspace check/test, diff whitespace check, and Gallery/Docs GUI startup smoke.

## 2026-06-18 P15 Track E CodeBlock highlight cache eviction

CodeBlock highlight cache now uses bounded FIFO eviction instead of clearing the entire cache after overflow. This prevents long docs pages or theme/language variations from forcing a full cache cold start after a single over-capacity insertion. Validation passed: fmt, focused CodeBlock tests, workspace check/test, diff whitespace check, and Gallery/Docs GUI startup smoke.

## 2026-06-18 P15 Track E CodeBlock shared highlight runs

CodeBlock highlight cache values now use shared `Arc<[TextRun]>` storage. Selectable and read-only block code paths retrieve a highlight cache key plus shared runs, so repeated visible CodeBlock/CodeEditor preview renders do not allocate-clone the full TextRun vector unless an inline `StyledText` API still requires owned runs. Focused regression coverage proves repeated cached block lookups pointer-share the same Arc storage. Validation passed: fmt, focused CodeBlock tests, workspace check/test, docs snippet check, diff whitespace check, and Gallery/Docs GUI startup smoke.

## 2026-06-18 P15 Track B synchronized state panic hardening

CodeBlock highlight/selection state, SelectableText selection state, and Timer runtime registries now recover poisoned mutexes with helper functions instead of panicking on `expect("... lock poisoned")`. The avoidable runtime panic audit now explicitly locks this behavior for those synchronized runtime-state paths. Validation passed: fmt, focused component tests, workspace check/test, docs snippet check, diff whitespace check, and Gallery/Docs GUI startup smoke.

## 2026-06-18 P15 Track B tray icon fallback hardening

Gallery and Docs no longer panic if bundled tray icon decoding fails. Both apps now attempt the bundled icon first, then app-specific solid-color fallback icons, and finally continue tray installation without an icon if all icon creation fails. Dynamic tray icon changes also skip failed icon loads instead of crashing command handling. Validation passed: app checks, focused tests, workspace check/test, docs snippet check, diff whitespace check, and Gallery/Docs GUI startup smoke.

## 2026-06-18 P15 Track B packager string rendering panic cleanup

`aura-packager` no longer uses `expect("write to string")` for SHA-256 hex, checksum text, release notes, or package manifest JSON rendering. Those paths now assemble strings with `format!` and `push_str`, preserving generated output while removing impossible-but-panic-based string write assumptions from the packaging pipeline. Validation passed: aura-packager tests, workspace check/test, docs snippet check, diff whitespace check, and Gallery/Docs GUI startup smoke.

## 2026-06-18 P15 Track B lucide build script error handling

`aura-icons-lucide` build script now uses `try_main() -> io::Result<()>` instead of unwraps for OUT_DIR, SVG directory reads, file names, generated file creation, and writes. Build failures now produce clear cargo error output while preserving the generated `IconName` format and rerun behavior. Validation passed: lucide check, workspace check/test, docs snippet check, diff whitespace check, and Gallery/Docs GUI startup smoke.
