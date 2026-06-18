# Dashboard Patterns

Dashboard-style screens are where Liora components meet real product constraints: dense data, mixed controls, long-lived state, repeated chart/table/card composition, shell search, theme switching, and feedback.

Liora keeps these patterns in maintained apps instead of standalone sample apps or the core component crate:

- Gallery demonstrates shell-level behavior and component composition.
- Docs explains setup and reusable app-layer patterns.
- `liora-components` contains reusable controls only. Do not add dashboard sample screens, mock models, or app-specific helper APIs to the core crate.

## Recommended structure

A production dashboard usually follows this native GPUI shape:

1. Initialize Liora once with `liora_components::init_liora(cx)`; use `liora_components::init_liora_with_mode(cx, ThemeMode::Light | ThemeMode::Dark | ThemeMode::System)` when a product wants an explicit startup mode.
2. Keep filters and toggles as `Entity<T>` fields in the app/view layer.
3. Render content as header, filters, metric cards, chart cards, operation panels, table, and runbook.
4. Keep business models, refresh logic, mock data, and dashboard-specific composition helpers in the app crate.

```rust
use liora_components::{Card, LineChart, Space, Statistic, Tag};
use gpui::{IntoElement, div, px};

fn app_metric_card(title: &str, value: &str, delta: &str, positive: bool) -> Card {
    Card::new(
        Space::new()
            .vertical()
            .gap_sm()
            .child(Statistic::new(title, value))
            .child(if positive {
                Tag::new(delta).success().round(true).into_any_element()
            } else {
                Tag::new(delta).warning().round(true).into_any_element()
            }),
    )
    .hoverable()
    .no_shrink()
}

fn metric_row() -> impl IntoElement {
    div()
        .grid()
        .grid_cols(4)
        .gap(px(16.0))
        .child(app_metric_card("Requests", "1.24M", "+12.6%", true))
        .child(app_metric_card("Latency p95", "184ms", "-8.4%", true))
        .child(app_metric_card("Errors", "0.18%", "+0.03%", false))
        .child(app_metric_card("SLO", "99.92%", "healthy", true))
}

fn chart_panel(chart: LineChart) -> Card {
    Card::new(chart).title("Traffic trend").no_shrink()
}
```

If the same composition helper becomes useful across Gallery, Docs, and real applications, promote it as a neutral component API with a component name and tests. Do not promote sample-specific dashboard screens.

## Theme switching

Theme switching should update the global `Config` and refresh the current window:

```rust
cx.global_mut::<liora_core::Config>().theme = liora_theme::Theme::dark();
window.refresh();
```

Gallery owns this behavior now. Keep theme state at the app shell level. Individual components should consume the active Liora theme from `Config` instead of carrying independent theme values.

## API friction checklist

Use Gallery and Docs after component API changes. Treat the following as signals that Liora should improve its ergonomics:

- repeating the same generic control composition across multiple maintained surfaces;
- table rows requiring verbose cells for common text/tag/status patterns;
- missing key binding registration causing silent interaction gaps;
- Gallery/Docs needing app-specific raw GPUI glue for common component-library behavior.

When friction appears in more than one maintained surface, prefer a neutral reusable component/helper in `liora-components` over duplicating app-local glue. Keep app-specific dashboard/sample code in `apps/liora-gallery` or `apps/liora-docs`.

## Smoke workflow

```bash
cargo check -p liora-gallery
cargo check -p liora-docs
timeout 10s cargo run -p liora-gallery
timeout 10s cargo run -p liora-docs
```
