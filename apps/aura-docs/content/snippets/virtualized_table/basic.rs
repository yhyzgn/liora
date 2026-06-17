//! Basic virtualized table snippet used by Aura Docs.

use aura_components::{Button, TableAlign, TableColumn, Tag, Text, VirtualizedTable};
use gpui::{IntoElement, SharedString};

pub fn build_virtualized_table() -> VirtualizedTable {
    VirtualizedTable::new(columns(false), 10_000, |row, key, _window, _cx| {
        cell(row, key, false)
    })
    .height(360.0)
    .row_height(52.0)
    .stripe(true)
    .border(true)
}

pub fn columns(sortable: bool) -> Vec<TableColumn> {
    let columns = vec![
        TableColumn::new("date", "日期").width_sm(),
        TableColumn::new("name", "客户").width_sm(),
        TableColumn::new("region", "区域").width_sm(),
        TableColumn::new("amount", "金额")
            .width_sm()
            .align(TableAlign::Right),
        TableColumn::new("status", "状态")
            .width_sm()
            .align(TableAlign::Center),
        TableColumn::new("action", "操作")
            .width_sm()
            .align(TableAlign::Right),
    ];

    if sortable {
        columns
            .into_iter()
            .map(|column| column.sortable())
            .collect()
    } else {
        columns
    }
}

pub fn cell(row: usize, key: &SharedString, reverse: bool) -> gpui::AnyElement {
    let index = if reverse { 9_999 - row } else { row };
    match key.as_ref() {
        "date" => Text::new(format!("2026-06-{:02}", index % 28 + 1)).into_any_element(),
        "name" => Text::new(format!("客户 #{:04}", index + 1))
            .bold()
            .into_any_element(),
        "region" => Text::new(["华东", "华南", "华北", "西南"][index % 4]).into_any_element(),
        "amount" => Text::new(format!(
            "¥{:>6.2}",
            (1_000 + index * 17 % 90_000) as f32 / 10.0
        ))
        .into_any_element(),
        "status" => status_tag(["已完成", "进行中", "待处理"][index % 3]).into_any_element(),
        "action" => Button::new("查看").primary().small().into_any_element(),
        _ => Text::new("-").into_any_element(),
    }
}

fn status_tag(status: &'static str) -> Tag {
    let tag = Tag::new(status).round(true).small();
    match status {
        "已完成" => tag.success(),
        "进行中" => tag.info(),
        _ => tag.warning(),
    }
}
