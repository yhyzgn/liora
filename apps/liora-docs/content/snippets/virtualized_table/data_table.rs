//! Data-table style virtualized table snippet used by Liora Docs.

use gpui::{App, AppContext, Context, Entity, IntoElement, Render, Window};
use liora_components::VirtualizedTable;

pub fn build_data_table_virtualized_table(cx: &mut App) -> Entity<DataTableDemo> {
    cx.new(|_| DataTableDemo { loaded_rows: 500 })
}

pub struct DataTableDemo {
    loaded_rows: usize,
}

impl Render for DataTableDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.entity().clone();
        let loaded_rows = self.loaded_rows;
        VirtualizedTable::new(
            crate::virtualized_table_basic::columns(false),
            loaded_rows,
            move |row, key, _window, _cx| crate::virtualized_table_basic::cell(row, key, false),
        )
        .height(360.0)
        .row_height(52.0)
        .stripe(true)
        .border(true)
        .selected_rows([1, 3, 5])
        .active_row(Some(8))
        .load_more(
            format!("加载更多数据（当前 {loaded_rows} 行）"),
            move |_, cx| {
                view.update(cx, |this, cx| {
                    if this.loaded_rows < 10_000 {
                        this.loaded_rows = (this.loaded_rows + 500).min(10_000);
                        cx.notify();
                    }
                });
            },
        )
    }
}
