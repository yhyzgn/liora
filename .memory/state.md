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
- Components: LineChart, AreaChart, BarChart, PieChart, RingChart, Sparkline.
- Gallery demos and Aura Docs pages/snippets for each chart.
