//! Sortable table columns with a custom header.
//!
//! In a real view, keep `sort_key` and `sort_order` in your view state, rebuild
//! `rows(sorted_rows(...))`, then call `.sort(...)` when a sort is active.

use gpui::SharedString;
use liora_components::{
    Button, Table, TableAlign, TableColumn, TableRow, TableSortOrder, Tag, Text,
};

pub fn sortable_table(
    sort_key: Option<&SharedString>,
    sort_order: Option<TableSortOrder>,
) -> Table {
    let mut table = Table::new(sortable_columns())
        .rows(sorted_rows(sort_key, sort_order))
        .stripe(true)
        .border(true);

    if let Some(key) = sort_key {
        table = table.sort(key.clone(), sort_order);
    }

    table
}

fn sortable_columns() -> Vec<TableColumn> {
    vec![
        TableColumn::new("date", "日期").width_sm().sortable(),
        TableColumn::new("name", "姓名")
            .header(Text::new("客户").bold().text_color(gpui::blue()).nowrap())
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

fn record_row(record: OrderRecord) -> TableRow {
    TableRow::new()
        .cell("date", record.date)
        .cell("name", record.name)
        .cell("address", record.address)
        .cell("status", status_tag(record.status))
        .cell("action", Button::new("查看").primary().small())
}

fn status_tag(status: &'static str) -> Tag {
    let tag = Tag::new(status).round(true).small();
    match status {
        "已完成" => tag.success(),
        "进行中" => tag.info(),
        _ => tag.warning(),
    }
}
