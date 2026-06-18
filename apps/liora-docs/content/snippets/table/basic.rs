//! Basic table rows and columns.

use liora_components::{Button, Table, TableAlign, TableColumn, TableRow, Tag};

pub fn basic_table() -> Table {
    Table::new(basic_columns()).rows(basic_rows())
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
