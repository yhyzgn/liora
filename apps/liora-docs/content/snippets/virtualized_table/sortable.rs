//! Sortable virtualized table snippet used by Liora Docs.
//!
//! Store sort state in your view, rebuild the table, then call `.sort(...)`.

use gpui::SharedString;
use liora_components::{TableSortOrder, VirtualizedTable};

pub fn build_sortable_virtualized_table(
    sort_key: Option<SharedString>,
    sort_order: Option<TableSortOrder>,
) -> VirtualizedTable {
    let reverse = sort_order == Some(TableSortOrder::Descending);
    let mut table = VirtualizedTable::new(
        crate::virtualized_table_basic::columns(true),
        10_000,
        move |row, key, _window, _cx| crate::virtualized_table_basic::cell(row, key, reverse),
    )
    .height(360.0)
    .row_height(52.0)
    .stripe(true)
    .border(true)
    .on_sort_change(|state, _window, _cx| {
        let _next_key = state.key;
        let _next_order = state.order;
        // In a real view, persist this into component state and notify.
    });

    if let Some(key) = sort_key {
        table = table.sort(key, sort_order);
    }

    table
}
