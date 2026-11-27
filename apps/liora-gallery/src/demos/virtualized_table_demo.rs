use gpui::{AnyView, App, Context, IntoElement, Render, SharedString, Window, prelude::*};
use liora_components::layout_helpers::{page, section, showcase_card_wide, showcase_stack};
use liora_components::{
    Button, Divider, Space, Statistic, TableAlign, TableColumn, TableSortOrder, TableSortState,
    Tag, Text, VirtualizedTable,
};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| VirtualizedTableDemo {
        sort_key: None,
        sort_order: None,
        loaded_rows: 500,
        #[cfg(test)]
        basic_state: None,
    })
    .into()
}

struct VirtualizedTableDemo {
    sort_key: Option<SharedString>,
    sort_order: Option<TableSortOrder>,
    loaded_rows: usize,
    #[cfg(test)]
    basic_state: Option<gpui::ListState>,
}

impl Render for VirtualizedTableDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.entity().clone();
        let sort_key = self.sort_key.clone();
        let sort_order = self.sort_order;
        let loaded_rows = self.loaded_rows;
        let load_view = cx.entity().clone();

        let mut sorted = VirtualizedTable::new(columns(true), 1_000, move |row, key, _, _| {
            virtual_cell(row, key, sort_key.as_ref(), sort_order)
        })
        .height(300.0)
        .row_height(44.0)
        .overdraw(gpui::px(80.0))
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
                .child(virtual_performance_metrics(10_000, 9, "< 0.1%"))
                .child(section(
                "Table showcase",
                "虚拟表格统一使用宽卡片承载，避免长表格直接打散 Gallery 页面。",
                showcase_stack(vec![
                    showcase_card_wide(
                        "万行固定表头",
                        "10,000 行数据只渲染视口附近行；拖动滚动条时应保持流畅，滚动位置不会因重新渲染丢失。",
                        {
                            let basic = VirtualizedTable::new(
                                columns(false),
                                10_000,
                                move |row, key, _, _| virtual_cell(row, key, None, None),
                            )
                            .height(300.0)
                            .row_height(44.0)
                            .overdraw(gpui::px(80.0))
                            .stripe(true)
                            .border(true);
                            #[cfg(test)]
                            self.basic_state.get_or_insert_with(|| basic.list_state());
                            basic
                        },
                    )
                    .into_any_element(),
                    showcase_card_wide(
                        "大数据排序状态",
                        "点击可排序表头会把状态交回业务层；真实项目可据此切换后端查询或本地 index 映射。",
                        sorted,
                    )
                    .into_any_element(),
                    showcase_card_wide(
                        "DataTable 增强：选择、固定列和 load more",
                        "点击加载更多会增量追加行数并保留 ListState；不是重新创建整张表。",
                        VirtualizedTable::new(columns(false), loaded_rows, move |row, key, _, _| {
                            virtual_cell(row, key, None, None)
                        })
                        .height(300.0)
                        .row_height(44.0)
                        .overdraw(gpui::px(80.0))
                        .stripe(true)
                        .border(true)
                        .selected_rows([1, 3, 5])
                        .active_row(Some(8))
                        .load_more(format!("加载更多数据（当前 {loaded_rows} 行）"), move |_, cx| {
                            load_view.update(cx, |this, cx| {
                                this.loaded_rows = (this.loaded_rows + 500).min(10_000);
                                cx.notify();
                            });
                        }),
                    )
                    .into_any_element(),
                ]),
            )),
        )
    }
}

fn virtual_performance_metrics(
    total_rows: usize,
    visible_rows: usize,
    ratio: &'static str,
) -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_md()
        .child(Text::new("虚拟化性能表现").bold())
        .child(
            Space::new()
                .gap_lg()
                .wrap()
                .child(Statistic::new("Total rows", total_rows.to_string()).icon(liora_icons_lucide::IconName::Database))
                .child(Statistic::new("Visible rows", visible_rows.to_string()).icon(liora_icons_lucide::IconName::Eye))
                .child(Statistic::new("Rendered ratio", ratio).icon(liora_icons_lucide::IconName::Gauge)),
        )
        .child(Text::new("表格数据按 row index 按需生成，滚动时只请求视口附近的 cell；加载更多只 splice 追加行数并保留滚动状态。").wrap())
        .child(Divider::new())
}

fn columns(sortable: bool) -> Vec<TableColumn> {
    let date = TableColumn::new("date", "日期").width_sm().fixed_left();
    let name = TableColumn::new("name", "客户").width_sm();
    let amount = TableColumn::new("amount", "金额")
        .width_sm()
        .align(TableAlign::Right);
    let status = TableColumn::new("status", "状态")
        .width_sm()
        .align(TableAlign::Center);
    let action = TableColumn::new("action", "操作")
        .width_sm()
        .fixed_right()
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
    use super::*;

    #[gpui::test]
    fn basic_mounts_ten_thousand_rows_and_terminal_customer(cx: &mut gpui::TestAppContext) {
        cx.update(liora_components::init_liora);
        let root = cx.add_window(|_, _| VirtualizedTableDemo {
            sort_key: None,
            sort_order: None,
            loaded_rows: 500,
            basic_state: None,
        });
        let mut window = gpui::VisualTestContext::from_window(root.into(), cx);
        window.simulate_resize(gpui::size(gpui::px(1568.0), gpui::px(900.0)));
        window.update(|window, cx| window.draw(cx).clear(cx));
        let state = root
            .update(&mut window, |demo, _, _| {
                demo.basic_state
                    .clone()
                    .expect("actual Gallery Basic mounted")
            })
            .unwrap();
        assert_eq!(state.item_count(), 10_000, "advertised Basic row count");
        state.scroll_to(gpui::ListOffset {
            item_ix: 9_999,
            offset_in_item: gpui::px(0.0),
        });
        window.update(|window, cx| {
            window.refresh();
            window.draw(cx).clear(cx);
        });
        let terminal = state
            .bounds_for_item(9_999)
            .expect("terminal Basic row rendered");
        assert!(state.viewport_bounds().intersects(&terminal));
        // Geometry plus the same row factory; this is not captured rendered text.
        assert_eq!(record_for_row(9_999, None, None).name, "Bob #10000");
    }
}
