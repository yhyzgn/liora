# P18 — Dashboard Polish and API Ergonomics

> 上游: `.prompt/P17-dogfood-dashboard.md`
> 状态: Complete
> 目标: 用 P17 Dashboard Dogfooding App 反向优化 Aura 的真实应用组合体验。

## Scope

- Polish `examples/dashboard-app` so it behaves more like a real product screen.
- Add light/dark theme switching in the dashboard dogfood app.
- Extract small composition helpers instead of broad abstractions: `DashboardGrid`, `dashboard_card`, and `metric_card`.
- Add native Docs `Dashboard Patterns` guidance.
- Update README / prompt / memory and regression tests.

## Completion evidence

- `cargo check -p aura-dashboard-app` passes.
- `cargo test -p aura-components dashboard::tests::dashboard_grid_presets_track_columns -- --nocapture` passes.
- `cargo test -p aura-docs markdown::tests::dashboard_patterns_cover_composition_helpers_and_theme_switching -- --nocapture` passes.
- Workspace checks/tests/docs/package dry-run gates pass.
- GUI smoke for Gallery, Docs, Minimal App, and Dashboard App starts and exits by expected timeout.
- Commit pushed.
