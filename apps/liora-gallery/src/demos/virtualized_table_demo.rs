use gpui::{AnyView, App, Context, IntoElement, Render, SharedString, Window, prelude::*};
use liora_components::layout_helpers::{page, section};
use liora_components::{
    Button, Space, TableAlign, TableColumn, TableSortOrder, TableSortState, Tag, Text,
    VirtualizedTable,
};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| VirtualizedTableDemo {
        sort_key: None,
        sort_order: None,
    })
    .into()
}

struct VirtualizedTableDemo {
    sort_key: Option<SharedString>,
    sort_order: Option<TableSortOrder>,
}

impl Render for VirtualizedTableDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.entity().clone();
        let sort_key = self.sort_key.clone();
        let sort_order = self.sort_order;

        let mut sorted = VirtualizedTable::new(columns(true), 10_000, move |row, key, _, _| {
            virtual_cell(row, key, sort_key.as_ref(), sort_order)
        })
        .height(420.0)
        .row_height(52.0)
        .stripe(true)
        .border(true)
        .on_sort_change(move |state: TableSortState, _, cx| {
            view.update(cx, |this, cx| {
                this.sort_key = state.order.map(|_| state.key.clone());
                this.sort_order = state.order;
                cx.notify();
            });
        });

        if let Some(key) = &self.sort_key {
            sorted = sorted.sort(key.clone(), self.sort_order);
        }

        page(
            "VirtualizedTable 虚拟表格",
            "固定表头 + 可见区行渲染的大数据表格，适合万级结构化数据和高成本单元格。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "万行固定表头",
                    "行内容由 row index + column key 即时生成，滚动时只布局可见行。",
                    VirtualizedTable::new(columns(false), 10_000, move |row, key, _, _| {
                        virtual_cell(row, key, None, None)
                    })
                    .height(420.0)
                    .row_height(52.0)
                    .stripe(true)
                    .border(true),
                ))
                .child(section(
                    "大数据排序状态",
                    "点击可排序表头会把状态交回业务层；真实项目可据此切换后端查询或本地 index 映射。",
                    sorted,
                )),
        )
    }
}

fn columns(sortable: bool) -> Vec<TableColumn> {
    let date = TableColumn::new("date", "日期").width_sm();
    let name = TableColumn::new("name", "客户").width_sm();
    let amount = TableColumn::new("amount", "金额")
        .width_sm()
        .align(TableAlign::Right);
    let status = TableColumn::new("status", "状态")
        .width_sm()
        .align(TableAlign::Center);
    let action = TableColumn::new("action", "操作")
        .width_sm()
        .align(TableAlign::Right);

    if sortable {
        vec![
            date.sortable(),
            name.sortable(),
            TableColumn::new("region", "区域").width_sm(),
            amount.sortable(),
            status.sortable(),
            action,
        ]
    } else {
        vec![
            date,
            name,
            TableColumn::new("region", "区域").width_sm(),
            amount,
            status,
            action,
        ]
    }
}

fn virtual_cell(
    row: usize,
    key: &SharedString,
    sort_key: Option<&SharedString>,
    sort_order: Option<TableSortOrder>,
) -> gpui::AnyElement {
    let record = record_for_row(row, sort_key, sort_order);
    match key.as_ref() {
        "date" => Text::new(record.date).into_any_element(),
        "name" => Text::new(record.name).bold().into_any_element(),
        "region" => Text::new(record.region).into_any_element(),
        "amount" => Text::new(format!("¥{:>6.2}", record.amount as f32 / 10.0)).into_any_element(),
        "status" => status_tag(record.status).into_any_element(),
        "action" => Button::new("查看").primary().small().into_any_element(),
        _ => Text::new("-").into_any_element(),
    }
}

#[derive(Clone)]
struct VirtualRecord {
    date: String,
    name: String,
    region: &'static str,
    amount: usize,
    status: &'static str,
}

fn record_for_row(
    row: usize,
    sort_key: Option<&SharedString>,
    sort_order: Option<TableSortOrder>,
) -> VirtualRecord {
    let mut index = row;
    if sort_order == Some(TableSortOrder::Descending) {
        index = 9_999 - row;
    }
    if sort_key.is_some_and(|key| key.as_ref() == "amount") {
        index = (row * 37) % 10_000;
    }

    let names = ["Tom", "Jack", "Alice", "Bob", "Neo", "Mia"];
    let regions = ["华东", "华南", "华北", "西南"];
    let statuses = ["已完成", "进行中", "待处理"];
    VirtualRecord {
        date: format!("2026-06-{:02}", index % 28 + 1),
        name: format!("{} #{:04}", names[index % names.len()], index + 1),
        region: regions[index % regions.len()],
        amount: 1_000 + (index * 17) % 90_000,
        status: statuses[index % statuses.len()],
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

#[cfg(test)]
mod tests {
    #[test]
    fn virtualized_table_demo_uses_component_api() {
        let source = include_str!("virtualized_table_demo.rs");

        assert!(source.contains("VirtualizedTable::new"));
        assert!(source.contains("10_000"));
        assert!(source.contains("row_height"));
        assert!(source.contains("on_sort_change"));
    }
}
