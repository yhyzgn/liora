# P17 — Dogfooding Dashboard App

> 上游: `.prompt/P16-adoption-readiness.md`
> 状态: Complete
> 目标: 用 Aura 自己构建一个真实 dashboard 示例，验证组件组合、布局、图表、表格、toast、CodeBlock 和启动流程是否适合外部项目采用。

## Scope

- 新增 `examples/dashboard-app` workspace package。
- Dashboard 覆盖 header、filters、metric cards、LineChart、BarChart、Progress、Table、CodeBlock、toast 和 key binding setup。
- Native Docs 增加 `Dashboard App` 页面。
- README / adoption docs 补 dashboard app 入口。
- 添加回归测试，锁住 dashboard app workspace、docs 页面和 README 入口。

## Completion evidence

- `cargo check -p aura-dashboard-app` passes.
- `timeout 10s cargo run -p aura-dashboard-app` starts and exits with expected timeout status 124.
- Workspace checks/tests/docs/package dry-run gates pass.
- Commit pushed.
