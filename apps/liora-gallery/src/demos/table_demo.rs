use gpui::{AnyView, App, Context, IntoElement, Render, SharedString, Window, prelude::*};
use liora_components::{
    Button, ButtonSize, ButtonVariant, Space, Table, TableAlign, TableColumn, TableRow,
    TableSortOrder, TableSortState, Tag, Text,
};
use liora_core::Config;

use liora_components::layout_helpers::{page, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| TableDemo {
        sort_key: None,
        sort_order: None,
    })
    .into()
}

struct TableDemo {
    sort_key: Option<SharedString>,
    sort_order: Option<TableSortOrder>,
}

impl Render for TableDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        let view = cx.entity().clone();
        let sort_key = self.sort_key.clone();
        let sort_order = self.sort_order;

        let mut sortable_table = Table::new(sortable_columns(theme))
            .rows(sorted_rows(sort_key.as_ref(), sort_order))
            .stripe(true)
            .border(true)
            .on_sort_change(move |state: TableSortState, _, cx| {
                view.update(cx, |this, cx| {
                    this.sort_key = state.order.map(|_| state.key.clone());
                    this.sort_order = state.order;
                    cx.notify();
                });
            });

        if let Some(key) = sort_key {
            sortable_table = sortable_table.sort(key, sort_order);
        }

        page(
            "Table 表格",
            "用于展示多条结构化数据，支持基础表格、斑马纹、边框、固定表头、加载、空状态、自定义表头和开发者启用的三态排序。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "自定义表头与可排序列",
                    "点击表头可切换三态排序。",
                    sortable_table,
                ))
                .child(section(
                    "基础用法",
                    "基础表格行列展示。",
                    Table::new(basic_columns()).rows(basic_rows()),
                ))
                .child(section(
                    "斑马纹与边框",
                    "通过 stripe 和 border 强化表格层次。",
                    Table::new(basic_columns())
                        .rows(basic_rows())
                        .stripe(true)
                        .border(true),
                ))
                .child(section(
                    "固定表头",
                    "长列表在固定高度中滚动。",
                    Table::new(basic_columns())
                        .rows(long_rows())
                        .stripe(true)
                        .fixed_header(true)
                        .height_md(),
                ))
                .child(section(
                    "加载状态",
                    "加载遮罩覆盖表格内容。",
                    Table::new(basic_columns()).rows(basic_rows()).loading(true),
                ))
                .child(section(
                    "空数据",
                    "无数据时展示空状态文案。",
                    Table::new(basic_columns()).empty_text("暂无订单数据"),
                )),
        )
    }
}

fn basic_columns() -> Vec<TableColumn> {
    vec![
        TableColumn::new("date", "日期").width_sm(),
        TableColumn::new("name", "姓名").width_sm(),
        TableColumn::new("address", "地址").min_width_lg(),
        TableColumn::new("status", "状态")
            .width_sm()
            .align(TableAlign::Center),
        TableColumn::new("action", "操作")
            .width_sm()
            .align(TableAlign::Right),
    ]
}

fn sortable_columns(theme: &liora_theme::Theme) -> Vec<TableColumn> {
    vec![
        TableColumn::new("date", "日期").width_sm().sortable(),
        TableColumn::new("name", "姓名")
            .header(
                Text::new("客户")
                    .bold()
                    .text_color(theme.primary.base)
                    .nowrap(),
            )
            .width_sm()
            .sortable(),
        TableColumn::new("address", "地址").min_width_lg(),
        TableColumn::new("status", "状态")
            .width_sm()
            .align(TableAlign::Center)
            .sortable(),
        TableColumn::new("action", "操作")
            .width_sm()
            .align(TableAlign::Right),
    ]
}

#[derive(Clone)]
struct OrderRecord {
    date: &'static str,
    name: &'static str,
    address: &'static str,
    status: &'static str,
}

fn records() -> Vec<OrderRecord> {
    vec![
        OrderRecord {
            date: "2016-05-03",
            name: "Tom",
            address: "上海市普陀区金沙江路 1518 弄",
            status: "已完成",
        },
        OrderRecord {
            date: "2016-05-02",
            name: "Jack",
            address: "上海市普陀区金沙江路 1517 弄",
            status: "进行中",
        },
        OrderRecord {
            date: "2016-05-04",
            name: "Alice",
            address: "上海市普陀区金沙江路 1519 弄",
            status: "已完成",
        },
        OrderRecord {
            date: "2016-05-01",
            name: "Bob",
            address: "上海市普陀区金沙江路 1516 弄",
            status: "待处理",
        },
    ]
}

fn basic_rows() -> Vec<TableRow> {
    records().into_iter().map(record_row).collect()
}

fn sorted_rows(
    sort_key: Option<&SharedString>,
    sort_order: Option<TableSortOrder>,
) -> Vec<TableRow> {
    let mut records = records();

    if let (Some(key), Some(order)) = (sort_key, sort_order) {
        records.sort_by(|a, b| field_value(a, key).cmp(field_value(b, key)));
        if order == TableSortOrder::Descending {
            records.reverse();
        }
    }

    records.into_iter().map(record_row).collect()
}

fn field_value<'a>(record: &'a OrderRecord, key: &SharedString) -> &'a str {
    match key.as_ref() {
        "date" => record.date,
        "name" => record.name,
        "status" => record.status,
        "address" => record.address,
        _ => "",
    }
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

fn record_row(record: OrderRecord) -> TableRow {
    row(record.date, record.name, record.address, record.status)
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
        .cell("status", status_tag(status))
        .cell(
            "action",
            Button::new("查看")
                .variant(ButtonVariant::Primary)
                .size(ButtonSize::Small),
        )
}

fn status_tag(status: &'static str) -> impl IntoElement {
    let tag = Tag::new(status).round(true).small();
    match status {
        "已完成" => tag.success(),
        "进行中" => tag.info(),
        _ => tag.warning(),
    }
}
