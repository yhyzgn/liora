//! Virtualized Table module.
//!
//! This public module implements the Liora virtualized table component for large row sets. It keeps the reusable
//! component logic inside `liora-components` rather than Gallery or Docs so
//! downstream GPUI applications can compose the same behavior with their own
//! app state, assets, and release policy.
//!
//! ## Usage model
//!
//! Components in this module render native GPUI element trees. Stateless builder
//! values can be constructed inline, while controls with focus, selection,
//! popup, drag, or editing state should be stored as `gpui::Entity<T>` fields in
//! the parent view so state survives GPUI render passes.
//!
//! ## Design contract
//!
//! The implementation should use Liora theme tokens from `liora-core` and
//! `liora-theme`, keep accessibility-oriented keyboard/pointer behavior close to
//! the component, and avoid app-specific Gallery/Docs resources in this SDK
//! crate.

use crate::VirtualScrollbar;
use crate::gpui_compat::element_id;
use crate::table::{TableAlign, TableColumn, TableColumnFixed, TableSortOrder, TableSortState};
use gpui::{
    AnyElement, App, Component, IntoElement, ListAlignment, ListState, Pixels, RenderOnce,
    SharedString, Window, div, list, prelude::*, px,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::sync::Arc;

type RenderCell = dyn Fn(usize, &SharedString, &mut Window, &mut App) -> AnyElement + 'static;
type SortCallback = dyn Fn(TableSortState, &mut Window, &mut App) + 'static;
type RowSelectCallback = dyn Fn(usize, &mut Window, &mut App) + 'static;

/// Virtualized table for large structured datasets.
///
/// `VirtualizedTable` keeps the table header native and fixed while GPUI's
/// `ListState` renders only visible rows. Cells are produced from a row index
/// and column key each frame, so callers must not cache frame-local GPUI
/// elements between renders.
pub struct VirtualizedTable {
    id: SharedString,
    columns: Vec<TableColumn>,
    row_count: usize,
    list_state: ListState,
    render_cell: Arc<RenderCell>,
    height: Pixels,
    overdraw: Pixels,
    row_height: Pixels,
    border: bool,
    stripe: bool,
    loading: bool,
    empty_text: SharedString,
    sort_key: Option<SharedString>,
    sort_order: Option<TableSortOrder>,
    on_sort_change: Option<Arc<SortCallback>>,
    selected_rows: Vec<usize>,
    active_row: Option<usize>,
    on_row_select: Option<Arc<RowSelectCallback>>,
    footer: Option<AnyElement>,
}

impl VirtualizedTable {
    /// Creates `VirtualizedTable` with default theme-driven styling and no optional callbacks attached.
    pub fn new(
        columns: Vec<TableColumn>,
        row_count: usize,
        render_cell: impl Fn(usize, &SharedString, &mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        let overdraw = px(640.0);
        Self {
            id: liora_core::unique_id("virtualized-table"),
            columns,
            row_count,
            list_state: ListState::new(row_count, ListAlignment::Top, overdraw),
            render_cell: Arc::new(render_cell),
            height: px(360.0),
            overdraw,
            row_height: px(48.0),
            border: false,
            stripe: false,
            loading: false,
            empty_text: "暂无数据".into(),
            sort_key: None,
            sort_order: None,
            on_sort_change: None,
            selected_rows: Vec::new(),
            active_row: None,
            on_row_select: None,
            footer: None,
        }
    }

    /// Assigns a stable element id used by GPUI state, hit testing, and automated interaction tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Sets the component height token used during GPUI layout.
    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = height.into();
        self
    }

    /// Applies the predefined height md sizing preset.
    pub fn height_md(self) -> Self {
        self.height(px(360.0))
    }

    /// Sets the fixed row height used by virtualized layout.
    pub fn row_height(mut self, height: impl Into<Pixels>) -> Self {
        self.row_height = height.into();
        self.list_state.reset(self.row_count);
        self
    }

    /// Sets how many extra virtual rows are rendered outside the viewport.
    pub fn overdraw(mut self, overdraw: impl Into<Pixels>) -> Self {
        let overdraw = overdraw.into();
        self.overdraw = overdraw;
        self.list_state = ListState::new(self.row_count, ListAlignment::Top, overdraw);
        self
    }

    /// Toggles or applies the component border treatment.
    pub fn border(mut self, border: bool) -> Self {
        self.border = border;
        self
    }

    /// Sets the stripe value used by the component.
    pub fn stripe(mut self, stripe: bool) -> Self {
        self.stripe = stripe;
        self
    }

    /// Toggles the loading state and associated spinner treatment.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Sets the message displayed when no rows are available.
    pub fn empty_text(mut self, text: impl Into<SharedString>) -> Self {
        self.empty_text = text.into();
        self
    }

    /// Sets the sort value used by the component.
    pub fn sort(mut self, key: impl Into<SharedString>, order: Option<TableSortOrder>) -> Self {
        self.sort_key = Some(key.into());
        self.sort_order = order;
        self
    }

    /// Registers a callback that runs when sort change occurs.
    pub fn on_sort_change(
        mut self,
        callback: impl Fn(TableSortState, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_sort_change = Some(Arc::new(callback));
        self
    }

    /// Sets selected row indices for data-table style selection highlighting.
    pub fn selected_rows(mut self, rows: impl IntoIterator<Item = usize>) -> Self {
        self.selected_rows = rows
            .into_iter()
            .filter(|row| *row < self.row_count)
            .collect();
        self
    }

    /// Sets the active row index for keyboard or master-detail data-table layouts.
    pub fn active_row(mut self, row: Option<usize>) -> Self {
        self.active_row = row.filter(|row| *row < self.row_count);
        self
    }

    /// Registers a callback that runs when a row is clicked.
    pub fn on_row_select(
        mut self,
        callback: impl Fn(usize, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_row_select = Some(Arc::new(callback));
        self
    }

    /// Adds a footer or load-more slot below the virtualized body.
    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }

    /// Adds a simple load-more button-like footer slot.
    pub fn load_more(
        mut self,
        label: impl Into<SharedString>,
        callback: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        let callback = Arc::new(callback);
        let label = label.into();
        self.footer = Some(
            div()
                .w_full()
                .py_3()
                .flex()
                .items_center()
                .justify_center()
                .text_sm()
                .cursor_pointer()
                .child(label)
                .on_mouse_up(gpui::MouseButton::Left, move |_, window, cx| {
                    callback(window, cx)
                })
                .into_any_element(),
        );
        self
    }

    /// Performs the list state operation used by this component.
    pub fn list_state(&self) -> ListState {
        self.list_state.clone()
    }

    /// Performs the row count operation used by this component.
    pub fn row_count(&self) -> usize {
        self.row_count
    }
}

impl RenderOnce for VirtualizedTable {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let mut columns = self.columns;
        let border = self.border;
        let stripe = self.stripe;
        let row_height = self.row_height;
        let render_cell = self.render_cell.clone();
        let list_state = self.list_state.clone();
        let has_rows = self.row_count > 0;
        let sort_key = self.sort_key;
        let sort_order = self.sort_order;
        let on_sort_change = self.on_sort_change;
        let selected_rows = self.selected_rows;
        let active_row = self.active_row;
        let on_row_select = self.on_row_select;
        let footer = self.footer;

        let header = div()
            .flex()
            .flex_row()
            .w_full()
            .bg(theme.neutral.hover)
            .border_b_1()
            .border_color(theme.neutral.border)
            .children(columns.iter_mut().enumerate().map(|(index, column)| {
                let active_order = if sort_key.as_ref() == Some(&column.key) {
                    sort_order
                } else {
                    None
                };
                virtual_table_header_cell(
                    column,
                    border,
                    index,
                    &theme,
                    active_order,
                    on_sort_change.clone(),
                    &self.id,
                )
            }));

        let body = if has_rows {
            div()
                .relative()
                .w_full()
                .h(self.height)
                .overflow_hidden()
                .child(
                    list(list_state.clone(), move |row_index, window, cx| {
                        let striped = stripe && row_index % 2 == 1;
                        let selected = selected_rows.contains(&row_index);
                        let active = active_row == Some(row_index);
                        let callback = on_row_select.clone();
                        div()
                            .flex()
                            .flex_row()
                            .w_full()
                            .min_h(row_height)
                            .bg(if selected || active {
                                theme.primary.light_9
                            } else if striped {
                                theme.neutral.hover.opacity(0.45)
                            } else {
                                theme.neutral.card
                            })
                            .when(selected, |s| {
                                s.border_l_4().border_color(theme.primary.base)
                            })
                            .when(active && !selected, |s| {
                                s.border_l_4().border_color(theme.info.base)
                            })
                            .when(callback.is_some(), |s| {
                                s.cursor_pointer().on_mouse_down(
                                    gpui::MouseButton::Left,
                                    move |_, window, cx| {
                                        if let Some(callback) = &callback {
                                            callback(row_index, window, cx);
                                        }
                                    },
                                )
                            })
                            .hover(|s| s.bg(theme.primary.light_9))
                            .when(row_index > 0, |s| {
                                s.border_t_1().border_color(theme.neutral.divider)
                            })
                            .children(columns.iter().enumerate().map(|(index, column)| {
                                let value = render_cell(row_index, &column.key, window, cx);
                                virtual_table_cell_shell(column, border, index)
                                    .min_h(row_height)
                                    .py_3()
                                    .child(
                                        div()
                                            .text_size(px(theme.font_size.sm))
                                            .text_color(theme.neutral.text_1)
                                            .child(value),
                                    )
                            }))
                            .into_any_element()
                    })
                    .size_full(),
                )
                .child(VirtualScrollbar::new(list_state))
                .into_any_element()
        } else {
            div()
                .w_full()
                .min_h(px(180.0))
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap_2()
                        .child(
                            Icon::new(IconName::PackageOpen)
                                .size(px(40.0))
                                .color(theme.neutral.text_3),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(theme.neutral.text_3)
                                .child(self.empty_text),
                        ),
                )
                .into_any_element()
        };

        div()
            .relative()
            .w_full()
            .overflow_hidden()
            .rounded(px(theme.radius.md))
            .border_1()
            .border_color(theme.neutral.border)
            .bg(theme.neutral.card)
            .child(header)
            .child(body)
            .when_some(footer, |s, footer| {
                s.child(
                    div()
                        .border_t_1()
                        .border_color(theme.neutral.border)
                        .bg(theme.neutral.card)
                        .child(footer),
                )
            })
            .when(self.loading, |s| {
                s.child(
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .bg(theme.neutral.card.opacity(0.72))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .items_center()
                                .gap_2()
                                .child(
                                    Icon::new(IconName::LoaderCircle)
                                        .size(px(32.0))
                                        .color(theme.primary.base),
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(theme.primary.base)
                                        .child("加载中"),
                                ),
                        ),
                )
            })
    }
}

impl IntoElement for VirtualizedTable {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

fn virtual_table_cell_shell(column: &TableColumn, border: bool, index: usize) -> gpui::Div {
    let mut cell = div()
        .flex()
        .items_center()
        .px_4()
        .min_w(column.min_width)
        .when(border && index > 0, |s| s.border_l_1())
        .when(column.fixed.is_some(), |s| s.bg(gpui::transparent_black()));

    cell = match column.width {
        Some(width) => cell.w(width).flex_shrink_0(),
        None => cell.flex_1(),
    };

    cell = match column.fixed {
        Some(TableColumnFixed::Left) => cell.border_r_1(),
        Some(TableColumnFixed::Right) => cell.border_l_1(),
        None => cell,
    };

    match column.align {
        TableAlign::Left => cell.justify_start(),
        TableAlign::Center => cell.justify_center(),
        TableAlign::Right => cell.justify_end(),
    }
}

fn virtual_table_header_cell(
    column: &mut TableColumn,
    border: bool,
    index: usize,
    theme: &liora_theme::Theme,
    active_order: Option<TableSortOrder>,
    on_sort_change: Option<Arc<SortCallback>>,
    table_id: &SharedString,
) -> AnyElement {
    let header_content = column.header.take().unwrap_or_else(|| {
        div()
            .text_size(px(theme.font_size.sm))
            .font_weight(gpui::FontWeight::BOLD)
            .text_color(theme.neutral.text_2)
            .child(column.label.clone())
            .into_any_element()
    });

    let icon = match active_order {
        Some(TableSortOrder::Ascending) => IconName::ArrowUp,
        Some(TableSortOrder::Descending) => IconName::ArrowDown,
        None => IconName::ArrowUpDown,
    };
    let icon_color = if active_order.is_some() {
        theme.primary.base
    } else {
        theme.neutral.text_3
    };

    let content = div()
        .flex()
        .items_center()
        .gap_1()
        .child(header_content)
        .when(column.sortable, |s| {
            s.child(Icon::new(icon).size(px(14.0)).color(icon_color))
        });

    let cell = virtual_table_cell_shell(column, border, index)
        .py_3()
        .child(content);

    if !column.sortable {
        return cell.into_any_element();
    }

    let column_key = column.key.clone();
    let next_order = match active_order {
        None => Some(TableSortOrder::Ascending),
        Some(TableSortOrder::Ascending) => Some(TableSortOrder::Descending),
        Some(TableSortOrder::Descending) => None,
    };
    let callback = on_sort_change.clone();

    cell.id(element_id(format!("{}-sort-{}", table_id, column.key)))
        .cursor_pointer()
        .hover(|s| s.bg(theme.neutral.pressed))
        .on_mouse_up(gpui::MouseButton::Left, move |_, window, cx| {
            if let Some(callback) = &callback {
                callback(
                    TableSortState {
                        key: column_key.clone(),
                        order: next_order,
                    },
                    window,
                    cx,
                );
            }
        })
        .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn virtualized_table_uses_list_state_without_row_element_cache() {
        let source = include_str!("virtualized_table.rs");

        assert!(source.contains("pub struct VirtualizedTable"));
        assert!(source.contains("ListState::new(row_count"));
        assert!(source.contains("list(list_state.clone()"));
        assert!(source.contains("VirtualScrollbar::new"));
        assert!(source.contains("render_cell"));
        assert!(source.contains("row_count: usize,"));
        assert!(source.contains("render_cell: Arc<RenderCell>,"));
    }

    #[test]
    fn virtualized_table_keeps_row_count_and_sort_state() {
        let table =
            VirtualizedTable::new(vec![TableColumn::new("name", "Name")], 250, |_, _, _, _| {
                div().into_any_element()
            })
            .height(px(240.0))
            .row_height(px(44.0))
            .stripe(true)
            .border(true)
            .sort("name", Some(TableSortOrder::Ascending))
            .selected_rows([1, 3])
            .active_row(Some(4))
            .footer(div());

        assert_eq!(table.row_count(), 250);
        assert_eq!(table.height, px(240.0));
        assert_eq!(table.row_height, px(44.0));
        assert!(table.stripe);
        assert!(table.border);
        assert_eq!(
            table.sort_key.as_ref().map(|text| text.as_ref()),
            Some("name")
        );
        assert_eq!(table.sort_order, Some(TableSortOrder::Ascending));
        assert_eq!(table.selected_rows, vec![1, 3]);
        assert_eq!(table.active_row, Some(4));
        assert!(table.footer.is_some());
    }

    #[test]
    fn virtualized_table_exposes_data_table_enhancements_without_new_parallel_component() {
        let fixed = TableColumn::new("id", "ID").fixed_left();
        assert_eq!(fixed.fixed, Some(TableColumnFixed::Left));

        let source = include_str!("virtualized_table.rs");
        assert!(source.contains("selected_rows"));
        assert!(source.contains("active_row"));
        assert!(source.contains("on_row_select"));
        assert!(source.contains("load_more"));
    }
}
