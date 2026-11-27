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
        sorted_rows: (0..1_000).collect(),
        loaded_rows: 500,
        #[cfg(test)]
        basic_state: None,
    })
    .into()
}

struct VirtualizedTableDemo {
    sort_key: Option<SharedString>,
    sort_order: Option<TableSortOrder>,
    sorted_rows: Vec<usize>,
    loaded_rows: usize,
    #[cfg(test)]
    basic_state: Option<gpui::ListState>,
}

impl VirtualizedTableDemo {
    fn sortable_table(
        &self,
        columns: Vec<TableColumn>,
        cx: &mut Context<Self>,
    ) -> VirtualizedTable {
        let cell_view = cx.entity().clone();
        let sort_view = cx.entity().clone();
        let mut table =
            VirtualizedTable::new(columns, self.sorted_rows.len(), move |row, key, _, cx| {
                let source = cell_view.read(cx).sorted_rows[row];
                virtual_cell(source, key)
            })
            .height(300.0)
            .row_height(44.0)
            .overdraw(gpui::px(80.0))
            .stripe(true)
            .border(true)
            .on_sort_change(move |state, _, cx| {
                sort_view.update(cx, |this, cx| {
                    this.sort_rows(state);
                    cx.notify();
                });
            });
        if let Some(key) = &self.sort_key {
            table = table.sort(key.clone(), self.sort_order);
        }
        table
    }

    fn sort_rows(&mut self, state: TableSortState) {
        if let Some(order) = state.order {
            let compare: fn(usize, usize) -> std::cmp::Ordering = match state.key.as_ref() {
                "date" => |left, right| (left % 28).cmp(&(right % 28)),
                "name" => |left, right| {
                    (NAMES[left % NAMES.len()], left).cmp(&(NAMES[right % NAMES.len()], right))
                },
                "amount" => |left, right| row_amount(left).cmp(&row_amount(right)),
                "status" => |left, right| {
                    STATUSES[left % STATUSES.len()].cmp(STATUSES[right % STATUSES.len()])
                },
                _ => return,
            };
            self.sorted_rows.sort_unstable_by(|&left, &right| {
                let primary = compare(left, right);
                let primary = if order == TableSortOrder::Descending {
                    primary.reverse()
                } else {
                    primary
                };
                primary.then_with(|| left.cmp(&right))
            });
        } else {
            for (id, row) in self.sorted_rows.iter_mut().enumerate() {
                *row = id;
            }
        }
        self.sort_key = state.order.map(|_| state.key);
        self.sort_order = state.order;
    }
}

impl Render for VirtualizedTableDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let loaded_rows = self.loaded_rows;
        let load_view = cx.entity().clone();
        let sorted = self.sortable_table(columns(true), cx);

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
                                move |row, key, _, _| virtual_cell(row, key),
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
                            virtual_cell(row, key)
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

const NAMES: [&str; 6] = ["Tom", "Jack", "Alice", "Bob", "Neo", "Mia"];
const REGIONS: [&str; 4] = ["华东", "华南", "华北", "西南"];
const STATUSES: [&str; 3] = ["已完成", "进行中", "待处理"];

fn row_date(row: usize) -> String {
    format!("2026-06-{:02}", row % 28 + 1)
}

fn row_name(row: usize) -> String {
    format!("{} #{:04}", NAMES[row % NAMES.len()], row + 1)
}

fn row_amount(row: usize) -> usize {
    1_000 + (row * 17) % 90_000
}

fn virtual_cell(row: usize, key: &SharedString) -> gpui::AnyElement {
    match key.as_ref() {
        "date" => Text::new(row_date(row)).into_any_element(),
        "name" => Text::new(row_name(row)).bold().into_any_element(),
        "region" => Text::new(REGIONS[row % REGIONS.len()]).into_any_element(),
        "amount" => {
            Text::new(format!("¥{:>6.2}", row_amount(row) as f32 / 10.0)).into_any_element()
        }
        "status" => status_tag(STATUSES[row % STATUSES.len()]).into_any_element(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[gpui::test]
    fn basic_mounts_ten_thousand_rows_and_terminal_customer(cx: &mut gpui::TestAppContext) {
        cx.update(liora_components::init_liora);
        let root = cx.add_window(|_, _| VirtualizedTableDemo {
            sort_key: None,
            sort_order: None,
            sorted_rows: (0..1_000).collect(),
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
        assert_eq!(row_name(9_999), "Bob #10000");
    }

    struct SortHeaderSurface {
        demo: gpui::Entity<VirtualizedTableDemo>,
    }

    impl Render for SortHeaderSurface {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let headers = columns(true)
                .into_iter()
                .map(|column| {
                    let label = column.label.clone();
                    let selector = format!("sort-red-header-{}", column.key);
                    column.header(gpui::div().debug_selector(move || selector).child(label))
                })
                .collect();
            let table = self
                .demo
                .update(cx, |demo, cx| demo.sortable_table(headers, cx))
                .id("gallery-sort-red");
            gpui::div()
                .w(gpui::px(1_050.0))
                .h(gpui::px(400.0))
                .child(table)
        }
    }
    #[gpui::test]
    fn sortable_headers_preserve_the_same_thousand_renderer_inputs(cx: &mut gpui::TestAppContext) {
        cx.update(liora_components::init_liora);
        let root = cx.add_window(|_, cx| SortHeaderSurface {
            demo: cx.new(|_| VirtualizedTableDemo {
                sort_key: None,
                sort_order: None,
                sorted_rows: (0..1_000).collect(),
                loaded_rows: 500,
                basic_state: None,
            }),
        });
        let mut window = gpui::VisualTestContext::from_window(root.into(), cx);
        window.simulate_resize(gpui::size(gpui::px(1_100.0), gpui::px(500.0)));
        window.update(|window, cx| window.draw(cx).clear(cx));
        let mut samples = Vec::new();
        let mut foreign_records = Vec::new();
        let mut tie_dates = None;
        for (header, expected_key, expected_order, edge) in [
            (
                "sort-red-header-amount",
                Some("amount"),
                Some(TableSortOrder::Ascending),
                999,
            ),
            (
                "sort-red-header-amount",
                Some("amount"),
                Some(TableSortOrder::Descending),
                999,
            ),
            ("sort-red-header-amount", None, None, 999),
            (
                "sort-red-header-date",
                Some("date"),
                Some(TableSortOrder::Ascending),
                1,
            ),
        ] {
            let bounds = window
                .debug_bounds(header)
                .expect("real sort-header content painted");
            assert!(bounds.size.width > gpui::px(0.0) && bounds.size.height > gpui::px(0.0));
            assert!(
                bounds.origin.x >= gpui::px(0.0)
                    && bounds.origin.y >= gpui::px(0.0)
                    && bounds.origin.x + bounds.size.width <= gpui::px(1_100.0)
                    && bounds.origin.y + bounds.size.height <= gpui::px(500.0)
            );
            window.simulate_click(bounds.center(), gpui::Modifiers::none());
            window.update(|window, cx| window.draw(cx).clear(cx));
            let (first, last, foreign) = root
                .update(&mut window, |this, _, cx| {
                    let this = this.demo.read(cx);
                    assert_eq!(this.sorted_rows.len(), 1_000);
                    let key: Option<&str> = this.sort_key.as_ref().map(|key| key.as_ref());
                    assert_eq!(key, expected_key, "native header updated retained key");
                    assert_eq!(this.sort_order, expected_order, "native header tri-state");
                    // Production renderer input, not captured text or pixels.
                    let input = |row| {
                        let source = this.sorted_rows[row];
                        let name = row_name(source);
                        let id = name
                            .rsplit_once('#')
                            .expect("production customer ID")
                            .1
                            .parse::<usize>()
                            .expect("numeric production customer ID")
                            - 1;
                        (id, row_amount(source), row_date(source))
                    };
                    let foreign = (0..1_000).find_map(|row| {
                        let id = input(row).0;
                        (id >= 1_000).then_some((row, id))
                    });
                    (input(0), input(edge), foreign)
                })
                .unwrap();
            eprintln!(
                "PRODUCTION_RENDERER_INPUT key={expected_key:?} order={expected_order:?} first={first:?} edge={last:?} first_foreign={foreign:?}"
            );
            samples.push((first.0, first.1, last.0, last.1));
            foreign_records.push(foreign);
            if expected_key == Some("date") {
                tie_dates = Some((first.2, last.2));
            }
        }
        assert_eq!(
            foreign_records,
            vec![None, None, None, None],
            "sorting must keep the original 1,000 source records"
        );
        assert_eq!(
            samples,
            vec![
                (0, 1_000, 999, 17_983),
                (999, 17_983, 0, 1_000),
                (0, 1_000, 999, 17_983),
                (0, 1_000, 28, 1_476),
            ],
            "native header actions must order the production renderer inputs"
        );
        assert_eq!(
            tie_dates,
            Some(("2026-06-01".to_owned(), "2026-06-01".to_owned()))
        );
    }
}
