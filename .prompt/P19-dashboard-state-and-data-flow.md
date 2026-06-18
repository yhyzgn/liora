# P19 — Dashboard State and Data Flow

> 上游: `.prompt/P18-dashboard-polish-and-api-ergonomics.md`
> 状态: Complete
> 目标: 将 Dashboard Dogfooding App 从静态展示推进到可测试的数据模型、过滤、刷新和状态分支样板。

## Scope

- Add explicit dashboard model structs for metrics, services, filters, status, and generated data.
- Make search, region, and alerts-only filters affect the service table and generated mock data.
- Make refresh regenerate revisioned dashboard data across metrics, charts, table rows, and progress panels.
- Cover loading/ready/empty/degraded state branches with ordinary Aura components.
- Add native Docs `Dashboard State` page and regression coverage.

## Completion evidence

- `cargo check -p aura-dashboard-app` passes.
- `cargo test -p aura-dashboard-app model::tests::filters_match_query_region_and_alerts -- --nocapture` passes.
- `cargo test -p aura-dashboard-app model::tests::empty_status_is_reported_for_no_visible_services -- --nocapture` passes.
- `cargo test -p aura-docs markdown::tests::dashboard_state_docs_cover_data_flow_model -- --nocapture` passes.
- Workspace checks/tests/docs/package dry-run gates pass.
- GUI smoke for Gallery, Docs, Minimal App, and Dashboard App starts and exits by expected timeout.
- Commit pushed.
