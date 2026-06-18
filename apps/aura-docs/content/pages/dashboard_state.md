# Dashboard State Patterns

P19 turns the dashboard dogfood app from a static composition sample into a small state/data-flow sample. The goal is still native GPUI + Aura primitives, not a framework on top of GPUI.

## Model first

Keep dashboard data in explicit Rust structs before rendering components:

- `DashboardData` owns metrics, chart series, service rows, progress values, and a revision number.
- `DashboardFilters` owns search query, region, and alerts-only state.
- `DashboardStatus` models loading, ready, empty, and degraded branches.

This keeps rendering code focused on Aura composition and makes filtering/refresh logic unit-testable.

```rust
let data = DashboardData::generate(revision, filters.region);
let visible = apply_filters(&data.services, &filters);
let status = status_for(&data, visible.len());
```

## Control wiring

Long-lived controls remain `Entity<T>` fields. Their callbacks update the parent dashboard view and call `cx.notify()`:

```rust
input.set_on_change(move |value, cx| {
    view.update(cx, |dashboard, cx| {
        dashboard.filters.query = value.to_string();
        cx.notify();
    });
});
```

Use the same pattern for region select and alert toggle. Refresh increments the model revision and regenerates mock data so metric cards, charts, table rows, and progress panels move together.

## State branches

The dashboard uses ordinary Aura components for state branches:

- loading: simple text state;
- ready: summary text;
- empty: `Empty` component;
- degraded: warning text plus service table details.

No special dashboard runtime is needed. State is normal Rust data; UI is normal Aura composition.

## Verification

```bash
cargo test -p aura-dashboard-app model::tests::filters_match_query_region_and_alerts -- --nocapture
cargo test -p aura-dashboard-app model::tests::empty_status_is_reported_for_no_visible_services -- --nocapture
cargo check -p aura-dashboard-app
timeout 10s cargo run -p aura-dashboard-app
```
