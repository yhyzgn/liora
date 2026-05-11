# Aura Session State

## Current Phase

**P8 - Native Gallery Documentation** (进行中；Phase 1 Typography 自举与 Phase 2 Markdown renderer/state machine 已完成，下一步是 Phase 3 code block styling + two-column docs shell；P9 deferred backlog 保持等待明确请求)

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
| P8 Native Gallery Docs | 🚧 In Progress | 2/4 | 4 |
| P9 Deferred Advanced | ⏸️ Deferred backlog | 0/9 | 9 |

## Git Status

- Branch: main
- Remote: https://github.com/yhyzgn/aura.git


## Deferred Backlog

- P9 Deferred Advanced created in `.prompt/P9-deferred-advanced.md`.
- Components moved from P5 deferred/skipped scope: Carousel, Calendar, TreeSelect, InputTag, Mention, Watermark, Tour, VirtualizedTable, VirtualizedTree.
- P9 is the latest phase and should be supplemented later only when explicitly requested.
