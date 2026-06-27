//! Data-table style virtualized table snippet used by Liora Docs.

use liora_components::VirtualizedTable;

pub fn build_data_table_virtualized_table() -> VirtualizedTable {
    VirtualizedTable::new(
        crate::virtualized_table_basic::columns(false),
        10_000,
        move |row, key, _window, _cx| crate::virtualized_table_basic::cell(row, key, false),
    )
    .height(360.0)
    .row_height(52.0)
    .stripe(true)
    .border(true)
    .selected_rows([1, 3, 5])
    .active_row(Some(8))
    .load_more("加载更多数据", |_window, _cx| {
        // Trigger a backend page fetch or append rows in parent state.
    })
}
