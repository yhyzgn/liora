use aura_components::{
    Button, ButtonSize, ButtonVariant, Table, TableAlign, TableColumn, TableRow,
};
use aura_core::Config;
use gpui::{AnyView, App, Context, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| TableDemo).into()
}

struct TableDemo;

impl Render for TableDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

        div()
            .flex()
            .flex_col()
            .gap_8()
            .p_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Table 表格"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("用于展示多条结构化数据，支持基础表格、斑马纹、边框、固定表头、加载和空状态。"),
                    ),
            )
            .child(section(
                "基础用法",
                Table::new(basic_columns()).rows(basic_rows()).into_any_element(),
            ))
            .child(section(
                "斑马纹与边框",
                Table::new(basic_columns())
                    .rows(basic_rows())
                    .stripe(true)
                    .border(true)
                    .into_any_element(),
            ))
            .child(section(
                "固定表头",
                Table::new(basic_columns())
                    .rows(long_rows())
                    .stripe(true)
                    .fixed_header(true)
                    .height(px(260.0))
                    .into_any_element(),
            ))
            .child(section(
                "加载状态",
                Table::new(basic_columns())
                    .rows(basic_rows())
                    .loading(true)
                    .into_any_element(),
            ))
            .child(section(
                "空数据",
                Table::new(basic_columns())
                    .empty_text("暂无订单数据")
                    .into_any_element(),
            ))
    }
}

fn section(title: &'static str, content: gpui::AnyElement) -> gpui::Div {
    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(div().font_weight(gpui::FontWeight::BOLD).child(title))
        .child(content)
}

fn basic_columns() -> Vec<TableColumn> {
    vec![
        TableColumn::new("date", "日期").width(px(120.0)),
        TableColumn::new("name", "姓名").width(px(120.0)),
        TableColumn::new("address", "地址").min_width(px(260.0)),
        TableColumn::new("status", "状态")
            .width(px(120.0))
            .align(TableAlign::Center),
        TableColumn::new("action", "操作")
            .width(px(120.0))
            .align(TableAlign::Right),
    ]
}

fn basic_rows() -> Vec<TableRow> {
    vec![
        row(
            "2016-05-03",
            "Tom",
            "上海市普陀区金沙江路 1518 弄",
            "已完成",
        ),
        row(
            "2016-05-02",
            "Jack",
            "上海市普陀区金沙江路 1517 弄",
            "进行中",
        ),
        row(
            "2016-05-04",
            "Alice",
            "上海市普陀区金沙江路 1519 弄",
            "已完成",
        ),
        row(
            "2016-05-01",
            "Bob",
            "上海市普陀区金沙江路 1516 弄",
            "待处理",
        ),
    ]
}

fn long_rows() -> Vec<TableRow> {
    (1..=16)
        .map(|i| {
            row(
                match i % 4 {
                    0 => "2016-05-04",
                    1 => "2016-05-01",
                    2 => "2016-05-02",
                    _ => "2016-05-03",
                },
                match i % 4 {
                    0 => "Tom",
                    1 => "Jack",
                    2 => "Alice",
                    _ => "Bob",
                },
                "上海市普陀区金沙江路 1518 弄",
                if i % 3 == 0 { "待处理" } else { "已完成" },
            )
        })
        .collect()
}

fn row(
    date: &'static str,
    name: &'static str,
    address: &'static str,
    status: &'static str,
) -> TableRow {
    TableRow::new()
        .cell("date", date)
        .cell("name", name)
        .cell("address", address)
        .cell(
            "status",
            div()
                .px_2()
                .py_1()
                .rounded(px(999.0))
                .bg(match status {
                    "已完成" => gpui::green().opacity(0.12),
                    "进行中" => gpui::blue().opacity(0.12),
                    _ => gpui::yellow().opacity(0.18),
                })
                .text_color(match status {
                    "已完成" => gpui::green(),
                    "进行中" => gpui::blue(),
                    _ => gpui::yellow(),
                })
                .text_xs()
                .child(status),
        )
        .cell(
            "action",
            Button::new("查看")
                .variant(ButtonVariant::Primary)
                .size(ButtonSize::Small),
        )
}
