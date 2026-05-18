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
- New widgets: QrCode generation/recognition, CodeEditor, SignalMeter, HeatBar, FlatBarMeter, SegmentRatioBar, HorizontalList, Timer, Label, Operation.
- Enhancements: RingChart external labels, LineChart per-series stroke style, BarChart value range colors, RingProgress gradient/completion color, Button gradient/custom color derived states, Tag flow layout, Radio/Checkbox option customization, vertical list drag.
- Execution is split into five waves: simple meters/bars/layout, chart/progress enhancements, draggable lists, QR/CodeEditor, and form-control deep customization.
- Existing-widget enhancement rule: existing controls must be enhanced in-place in their current source/demo/docs; do not add parallel replacement components for Tag flow, RingProgress gradients, chart style options, Button custom colors, or Radio/Checkbox option customization.
