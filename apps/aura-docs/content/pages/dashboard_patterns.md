# Dashboard Patterns

Dashboard pages are where Aura components meet real product constraints: dense data, mixed controls, long-lived state, and repeated chart/table/card composition. Prefer using small Aura composition helpers before inventing app-specific wrappers.

## Recommended structure

A production dashboard usually follows this native GPUI shape:

1. Initialize Aura once with `init_aura(cx, Theme::light())` or `Theme::dark()`.
2. Initialize global feedback with `MessageManager::init(cx)`.
3. Register key bindings for every interactive component used by the screen.
4. Keep filters and toggles as `Entity<T>` fields.
5. Render content as header, filters, metric cards, chart cards, operation panels, table, and runbook.
6. Use `DashboardGrid` and `dashboard_card` to keep repeated layout glue out of app code.

```rust
use aura_components::{DashboardGrid, LineChart, dashboard_card, metric_card};

fn metric_row() -> impl gpui::IntoElement {
    DashboardGrid::metrics()
        .child(metric_card("Requests", "1.24M", "+12.6%", true))
        .child(metric_card("Latency p95", "184ms", "-8.4%", true))
        .child(metric_card("Errors", "0.18%", "+0.03%", false))
        .child(metric_card("SLO", "99.92%", "healthy", true))
}

fn chart_panel(chart: LineChart) -> aura_components::Card {
    dashboard_card("Traffic trend", chart)
}
```

## Theme switching

Theme switching should update the global `Config` and refresh the current window. The dashboard dogfood app demonstrates this with a `Switch` stored as an entity:

```rust
cx.global_mut::<aura_core::Config>().theme = aura_theme::Theme::dark();
window.refresh();
```

Keep theme state at the app shell level. Individual components should keep consuming the active Aura theme from `Config` instead of carrying independent theme values.

## API friction checklist

Use `examples/dashboard-app` after component API changes. Treat the following as signals that Aura should improve its ergonomics:

- repeating the same `Card::new(...).title(...).no_shrink()` wrapper many times;
- manually choosing grid columns for standard dashboard rows;
- table rows requiring verbose cells for common text/tag/status patterns;
- missing key binding registration causing silent interaction gaps;
- a component needing app-specific style glue for basic dashboard placement.

When friction appears in more than one dashboard section, prefer a tiny helper in `aura-components` over duplicating app-local glue.

## Smoke workflow

```bash
cargo check -p aura-dashboard-app
timeout 10s cargo run -p aura-dashboard-app
```

The window should open as a native GPUI app, keep filters/selects/switches responsive, render chart/table/code sections, and support switching light/dark mode without restarting.
