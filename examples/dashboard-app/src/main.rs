//! Dogfooding dashboard app for Aura.
//!
//! This example intentionally combines many components in one realistic screen
//! so Aura can validate real composition, not only isolated component demos.

mod model;

use aura_components::{
    BarChart, Button, Card, ChartPoint, ChartSeries, CodeBlock, DashboardGrid, Empty, Input,
    LineChart, MessageManager, Progress, Select, Space, Switch, Table, TableColumn, TableRow, Tag,
    Text, Title, dashboard_card, metric_card, toast_info, toast_success,
};
use aura_core::{Config, init_aura};
use aura_theme::Theme;
use gpui::{
    App, AppContext, Bounds, Context, Entity, InteractiveElement, IntoElement, ParentElement,
    Render, StatefulInteractiveElement, Styled, Window, WindowBounds, WindowOptions, div, px, size,
};
use model::{
    DashboardData, DashboardFilters, DashboardStatus, RegionFilter, ServiceDatum, apply_filters,
    status_for,
};

struct DashboardApp {
    search: Entity<Input>,
    region: Entity<Select>,
    alerts_enabled: Entity<Switch>,
    dark_mode: Entity<Switch>,
    data: DashboardData,
    filters: DashboardFilters,
}

impl DashboardApp {
    fn new(cx: &mut Context<Self>) -> Self {
        let filters = DashboardFilters::default();
        Self {
            search: cx.new(|cx| Input::new("", cx).placeholder("Filter services or owners")),
            region: cx
                .new(|cx| Select::new(RegionFilter::LABELS.to_vec(), Some(0), cx).width(px(160.0))),
            alerts_enabled: cx.new(|cx| Switch::new(true, cx)),
            dark_mode: cx.new(|cx| {
                Switch::new(false, cx).on_change(|enabled, window, cx| {
                    cx.global_mut::<Config>().theme = if enabled {
                        Theme::dark()
                    } else {
                        Theme::light()
                    };
                    window.refresh();
                    toast_success!(
                        "Dashboard theme switched to {}",
                        if enabled { "dark" } else { "light" }
                    );
                })
            }),
            data: DashboardData::generate(1, filters.region),
            filters,
        }
    }

    fn wire_controls(&self, cx: &mut Context<Self>) {
        let view = cx.entity().clone();
        cx.update_entity(&self.search, |input, _cx| {
            input.set_on_change({
                let view = view.clone();
                move |value, cx| {
                    view.update(cx, |dashboard: &mut DashboardApp, cx| {
                        dashboard.filters.query = value.to_string();
                        cx.notify();
                    });
                }
            });
        });

        let view = cx.entity().clone();
        cx.update_entity(&self.region, |select, _cx| {
            select.set_on_change(move |index, _window, cx| {
                view.update(cx, |dashboard: &mut DashboardApp, cx| {
                    dashboard.filters.region = RegionFilter::from_index(index);
                    dashboard.data = DashboardData::generate(
                        dashboard.data.revision + 1,
                        dashboard.filters.region,
                    );
                    cx.notify();
                    toast_info!("Region switched to {}", dashboard.filters.region.label());
                });
            });
        });

        let view = cx.entity().clone();
        cx.update_entity(&self.alerts_enabled, |switch, _cx| {
            switch.set_on_change(move |enabled, _window, cx| {
                view.update(cx, |dashboard: &mut DashboardApp, cx| {
                    dashboard.filters.alerts_only = enabled;
                    cx.notify();
                    toast_info!(
                        "Alert filter {}",
                        if enabled { "enabled" } else { "disabled" }
                    );
                });
            });
        });
    }

    fn refresh_dashboard(&mut self, cx: &mut Context<Self>) {
        self.data = DashboardData::generate(self.data.revision + 1, self.filters.region);
        cx.notify();
    }

    fn visible_services(&self) -> Vec<&ServiceDatum> {
        apply_filters(&self.data.services, &self.filters)
    }
}

impl Render for DashboardApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.wire_controls(cx);
        let theme = cx.global::<Config>().theme.clone();
        let visible_services = self.visible_services();
        let status = status_for(&self.data, visible_services.len());

        div()
            .size_full()
            .bg(theme.neutral.body)
            .flex()
            .flex_col()
            .child(self.header(&theme, &status))
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .id("dashboard-content-scroll")
                    .overflow_y_scroll()
                    .p_6()
                    .child(
                        Space::new()
                            .vertical()
                            .gap_xl()
                            .child(self.filters(cx))
                            .child(self.state_panel(&status, visible_services.len()))
                            .child(self.metrics())
                            .child(self.charts())
                            .child(self.operations())
                            .child(self.incidents(visible_services))
                            .child(self.runbook()),
                    ),
            )
    }
}

impl DashboardApp {
    fn header(&self, theme: &Theme, status: &DashboardStatus) -> impl IntoElement {
        div()
            .px_6()
            .py_4()
            .bg(theme.neutral.card)
            .border_b_1()
            .border_color(theme.neutral.border)
            .flex()
            .items_center()
            .justify_between()
            .child(
                Space::new()
                    .vertical()
                    .gap_xs()
                    .child(Title::new("Aura Ops Dashboard").h2())
                    .child(Text::new(format!(
                        "{} data flow · revision {} · {}",
                        self.filters.region.label(),
                        self.data.revision,
                        status_label(status)
                    ))),
            )
            .child(
                Space::new()
                    .gap_md()
                    .child(status_tag_for_dashboard(status))
                    .child(Tag::new("P19 data flow").success().round(true))
                    .child(
                        Space::new()
                            .gap_sm()
                            .child(Text::new("Dark"))
                            .child(self.dark_mode.clone()),
                    ),
            )
    }

    fn filters(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let label = self_view_button_label(self.data.revision);
        let view = cx.entity().clone();
        Card::new(
            div()
                .flex()
                .items_center()
                .justify_between()
                .gap_4()
                .child(
                    Space::new()
                        .gap_md()
                        .wrap()
                        .child(self.search.clone())
                        .child(self.region.clone())
                        .child(
                            Space::new()
                                .gap_sm()
                                .child(Text::new("Alerts only"))
                                .child(self.alerts_enabled.clone()),
                        ),
                )
                .child(
                    Button::new(label)
                        .primary()
                        .on_click(move |_, _window, cx| {
                            view.update(cx, |dashboard: &mut DashboardApp, cx| {
                                dashboard.refresh_dashboard(cx);
                            });
                            toast_success!("Dashboard refreshed");
                        }),
                ),
        )
        .no_shadow()
        .no_shrink()
    }

    fn state_panel(&self, status: &DashboardStatus, visible_count: usize) -> impl IntoElement {
        let body = match status {
            DashboardStatus::Loading => Text::new("Loading dashboard data...").into_any_element(),
            DashboardStatus::Ready => Text::new(format!(
                "Showing {visible_count} services for {}. Filters are live and data is revisioned.",
                self.filters.region.label()
            ))
            .into_any_element(),
            DashboardStatus::Empty => Empty::new()
                .description("No services match the current filters. Clear search or disable alerts-only.")
                .into_any_element(),
            DashboardStatus::Degraded => Text::new(format!(
                "{visible_count} filtered services include degraded or watched workloads. Review the service table."
            ))
            .into_any_element(),
        };

        Card::new(body).no_shadow().no_shrink()
    }

    fn metrics(&self) -> impl IntoElement {
        self.data
            .metrics
            .iter()
            .fold(DashboardGrid::metrics(), |grid, metric| {
                grid.child(metric_card(
                    metric.title,
                    metric.value.clone(),
                    metric.delta.clone(),
                    metric.positive,
                ))
            })
    }

    fn charts(&self) -> impl IntoElement {
        DashboardGrid::charts()
            .child(dashboard_card(
                "Traffic trend",
                LineChart::new([
                    ChartSeries::new("Traffic", chart_points(self.data.traffic.clone())),
                    ChartSeries::new("Errors", chart_points(self.data.errors.clone())).dashed(),
                ])
                .height(px(280.0))
                .smooth(true)
                .area_fill(true)
                .show_value_labels(false),
            ))
            .child(dashboard_card(
                "Capacity by service",
                BarChart::new([
                    ChartSeries::new("API", chart_points(self.data.api_capacity.clone())),
                    ChartSeries::new("Worker", chart_points(self.data.worker_capacity.clone())),
                ])
                .height(px(280.0))
                .show_value_labels(false),
            ))
    }

    fn operations(&self) -> impl IntoElement {
        DashboardGrid::operations()
            .child(dashboard_card(
                "CPU budget",
                Progress::new(self.data.cpu as f32).text_inside(true),
            ))
            .child(dashboard_card(
                "Memory budget",
                Progress::new(self.data.memory as f32).text_inside(true),
            ))
            .child(dashboard_card(
                "Release readiness",
                Progress::new(self.data.release_readiness as f32).circle(),
            ))
    }

    fn incidents(&self, visible_services: Vec<&ServiceDatum>) -> impl IntoElement {
        let columns = vec![
            TableColumn::new("service", "Service")
                .width(px(180.0))
                .sortable(),
            TableColumn::new("region", "Region").width(px(120.0)),
            TableColumn::new("status", "Status").width(px(140.0)),
            TableColumn::new("owner", "Owner").width(px(160.0)),
            TableColumn::new("latency", "p95")
                .width(px(120.0))
                .sortable(),
        ];
        let rows = visible_services.into_iter().map(|service| {
            TableRow::new()
                .cell("service", Text::new(service.service))
                .cell("region", Text::new(service.region))
                .cell("status", status_tag(service.status))
                .cell("owner", Text::new(service.owner))
                .cell("latency", Text::new(format!("{}ms", service.latency_ms)))
        });

        dashboard_card(
            "Service health",
            Table::new(columns)
                .rows(rows)
                .stripe(true)
                .border(true)
                .fixed_header(true)
                .height(px(280.0)),
        )
    }

    fn runbook(&self) -> impl IntoElement {
        dashboard_card(
            "Release runbook",
            CodeBlock::new(
                r#"cargo run -p xtask -- package release-readiness
cargo run -p xtask -- package ci --all-apps --format platform-defaults --dry-run --skip-build
cargo run -p aura-dashboard-app"#,
            )
            .shell(),
        )
    }
}

fn status_tag(status: &str) -> impl IntoElement {
    match status {
        "Healthy" => Tag::new(status).success().round(true),
        "Watching" => Tag::new(status).warning().round(true),
        _ => Tag::new(status).danger().round(true),
    }
}

fn status_tag_for_dashboard(status: &DashboardStatus) -> impl IntoElement {
    match status {
        DashboardStatus::Loading => Tag::new("Loading").round(true),
        DashboardStatus::Ready => Tag::new("Ready").success().round(true),
        DashboardStatus::Empty => Tag::new("Empty").warning().round(true),
        DashboardStatus::Degraded => Tag::new("Degraded").danger().round(true),
    }
}

fn status_label(status: &DashboardStatus) -> &'static str {
    match status {
        DashboardStatus::Loading => "loading",
        DashboardStatus::Ready => "ready",
        DashboardStatus::Empty => "empty",
        DashboardStatus::Degraded => "degraded",
    }
}

fn chart_points(values: impl IntoIterator<Item = f64>) -> Vec<ChartPoint> {
    values
        .into_iter()
        .enumerate()
        .map(|(idx, value)| ChartPoint::new(format!("D{}", idx + 1), value))
        .collect()
}

fn self_view_button_label(revision: u32) -> String {
    format!("Refresh r{}", revision + 1)
}

fn main() {
    gpui_platform::application().run(|cx: &mut App| {
        init_aura(cx, Theme::light());
        MessageManager::init(cx);
        Input::register_key_bindings(cx);
        Select::register_key_bindings(cx);
        Switch::register_key_bindings(cx);
        CodeBlock::register_key_bindings(cx);
        Text::register_key_bindings(cx);
        Title::register_key_bindings(cx);

        let _ = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: gpui::Point::default(),
                    size: size(px(1280.0), px(900.0)),
                })),
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("Aura Dashboard Dogfood".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| cx.new(DashboardApp::new),
        );
    });
}
