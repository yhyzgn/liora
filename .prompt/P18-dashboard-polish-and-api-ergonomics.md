# P18 — Dashboard Polish and API Ergonomics

> 上游: `.prompt/P17-dogfood-dashboard.md`
> 状态: Complete
> 目标: 用 P17 Dashboard Dogfooding App 反向优化 Aura 的真实应用组合体验。

## Scope

- Polish Gallery/Docs so dashboard-style shell behaviors live in maintained surfaces, not standalone sample apps.
- Add light/dark theme switching in the dashboard dogfood app.
- Keep dashboard-specific composition helpers app-local unless they become neutral reusable component APIs across maintained surfaces.
- Add native Docs `Dashboard Patterns` guidance.
- Update README / prompt / memory and regression tests.

## Completion evidence

- `cargo check -p aura-gallery` and `cargo check -p aura-docs` pass.
- `aura-components` has no `dashboard` module or dashboard sample/model exports.
- `cargo test -p aura-docs markdown::tests::dashboard_patterns_keep_sample_code_out_of_components -- --nocapture` passes.
- Workspace checks/tests/docs/package dry-run gates pass.
- GUI smoke for Gallery and Docs starts and exits by expected timeout.
- Commit pushed.
