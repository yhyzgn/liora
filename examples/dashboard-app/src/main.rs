//! Dogfooding dashboard app for Aura.
//!
//! This example intentionally combines many components in one realistic screen
//! so Aura can validate real composition, not only isolated component demos.

use aura_components::{
    BarChart, Button, Card, ChartPoint, ChartSeries, CodeBlock, Input, LineChart, MessageManager,
    Progress, Select, Space, Statistic, Switch, Table, TableColumn, TableRow, Tag, Text, Title,
    toast_success,
};
use aura_core::init_aura;
use aura_theme::Theme;
use gpui::{
    App, AppContext, Bounds, Context, Entity, InteractiveElement, IntoElement, ParentElement,
    Render, StatefulInteractiveElement, Styled, Window, WindowBounds, WindowOptions, div, px, size,
};

struct DashboardApp {
    search: Entity<Input>,
    region: Entity<Select>,
    alerts_enabled: Entity<Switch>,
}

impl DashboardApp {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            search: cx.new(|cx| Input::new("", cx).placeholder("Filter services")),
            region: cx.new(|cx| {
                Select::new(vec!["Global", "APAC", "Europe", "Americas"], Some(0), cx)
                    .width(px(160.0))
            }),
            alerts_enabled: cx.new(|cx| Switch::new(true, cx)),
        }
    }
}

impl Render for DashboardApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(gpui::rgb(0xf6f8fb))
            .flex()
            .flex_col()
            .child(self.header())
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
                            .gap_lg()
                            .child(self.filters())
                            .child(self.metrics())
                            .child(self.charts())
                            .child(self.operations())
                            .child(self.incidents())
                            .child(self.runbook()),
                    ),
            )
    }
}

impl DashboardApp {
    fn header(&self) -> impl IntoElement {
        div()
            .px_6()
            .py_4()
            .bg(gpui::white())
            .border_b_1()
            .border_color(gpui::rgb(0xe5e7eb))
            .flex()
            .items_center()
            .justify_between()
            .child(
                Space::new()
                    .vertical()
                    .gap_xs()
                    .child(Title::new("Aura Ops Dashboard").h2())
                    .child(Text::new(
                        "Dogfooding real-world composition with native GPUI components.",
                    )),
            )
            .child(Tag::new("P17 dogfood").success().round(true))
    }

    fn filters(&self) -> impl IntoElement {
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
                        .child(self.alerts_enabled.clone()),
                )
                .child(
                    Button::new("Refresh")
                        .primary()
                        .on_click(|_, _, _| toast_success!("Dashboard refreshed")),
                ),
        )
        .no_shadow()
    }

    fn metrics(&self) -> impl IntoElement {
        div()
            .grid()
            .grid_cols(4)
            .gap_4()
            .child(metric_card("Requests", "1.24M", "+12.6%", true))
            .child(metric_card("Latency p95", "184ms", "-8.4%", true))
            .child(metric_card("Errors", "0.18%", "+0.03%", false))
            .child(metric_card("SLO", "99.92%", "healthy", true))
    }

    fn charts(&self) -> impl IntoElement {
        div()
            .grid()
            .grid_cols(2)
            .gap_4()
            .child(
                Card::new(
                    LineChart::new([
                        ChartSeries::new(
                            "Traffic",
                            chart_points([120.0, 180.0, 160.0, 240.0, 310.0, 280.0, 360.0]),
                        ),
                        ChartSeries::new(
                            "Errors",
                            chart_points([12.0, 18.0, 16.0, 20.0, 19.0, 24.0, 21.0]),
                        )
                        .dashed(),
                    ])
                    .height(px(260.0))
                    .smooth(true)
                    .area_fill(true)
                    .show_value_labels(false),
                )
                .title("Traffic trend"),
            )
            .child(
                Card::new(
                    BarChart::new([
                        ChartSeries::new(
                            "API",
                            chart_points([72.0, 68.0, 81.0, 77.0, 85.0, 92.0, 88.0]),
                        ),
                        ChartSeries::new(
                            "Worker",
                            chart_points([42.0, 58.0, 61.0, 64.0, 69.0, 73.0, 78.0]),
                        ),
                    ])
                    .height(px(260.0))
                    .show_value_labels(false),
                )
                .title("Capacity by service"),
            )
    }

    fn operations(&self) -> impl IntoElement {
        div()
            .grid()
            .grid_cols(3)
            .gap_4()
            .child(Card::new(Progress::new(76.0).text_inside(true)).title("CPU budget"))
            .child(Card::new(Progress::new(58.0).text_inside(true)).title("Memory budget"))
            .child(Card::new(Progress::new(92.0).circle()).title("Release readiness"))
    }

    fn incidents(&self) -> impl IntoElement {
        let columns = vec![
            TableColumn::new("service", "Service")
                .width(px(180.0))
                .sortable(),
            TableColumn::new("status", "Status").width(px(140.0)),
            TableColumn::new("owner", "Owner").width(px(160.0)),
            TableColumn::new("latency", "p95")
                .width(px(120.0))
                .sortable(),
        ];
        let rows = [
            ("Gateway", "Healthy", "Platform", "121ms"),
            ("Billing", "Watching", "Revenue", "232ms"),
            ("Search", "Healthy", "Discovery", "148ms"),
            ("Jobs", "Degraded", "Infra", "390ms"),
        ]
        .into_iter()
        .map(|(service, status, owner, latency)| {
            TableRow::new()
                .cell("service", Text::new(service))
                .cell("status", status_tag(status))
                .cell("owner", Text::new(owner))
                .cell("latency", Text::new(latency))
        });

        Card::new(
            Table::new(columns)
                .rows(rows)
                .stripe(true)
                .border(true)
                .fixed_header(true)
                .height(px(260.0)),
        )
        .title("Service health")
    }

    fn runbook(&self) -> impl IntoElement {
        Card::new(
            CodeBlock::new(
                r#"cargo run -p xtask -- package release-readiness
cargo run -p xtask -- package ci --all-apps --format platform-defaults --dry-run --skip-build
cargo run -p aura-dashboard-app"#,
            )
            .shell(),
        )
        .title("Release runbook")
    }
}

fn metric_card(title: &str, value: &str, delta: &str, good: bool) -> impl IntoElement {
    Card::new(
        Space::new()
            .vertical()
            .gap_sm()
            .child(Statistic::new(title, value))
            .child(if good {
                Tag::new(delta).success().round(true).into_any_element()
            } else {
                Tag::new(delta).warning().round(true).into_any_element()
            }),
    )
    .hoverable()
}

fn status_tag(status: &str) -> impl IntoElement {
    match status {
        "Healthy" => Tag::new(status).success().round(true),
        "Watching" => Tag::new(status).warning().round(true),
        _ => Tag::new(status).danger().round(true),
    }
}

fn chart_points(values: impl IntoIterator<Item = f64>) -> Vec<ChartPoint> {
    values
        .into_iter()
        .enumerate()
        .map(|(idx, value)| ChartPoint::new(format!("D{}", idx + 1), value))
        .collect()
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
                    size: size(px(1240.0), px(860.0)),
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
