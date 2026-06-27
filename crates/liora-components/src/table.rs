//! Table module.
//!
//! This public module implements the Liora table component with columns, rows, sorting, and fixed-header options. It keeps the reusable
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

use crate::gpui_compat::element_id;
use gpui::{
    AnyElement, App, Component, IntoElement, MouseButton, Pixels, RenderOnce, SharedString, Window,
    div, prelude::*, px,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control table align behavior.
pub enum TableAlign {
    #[default]
    /// Places the overlay to the left of the anchor.
    Left,
    /// Aligns content using the center position.
    Center,
    /// Places the overlay to the right of the anchor.
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Options that control table fixed-column edge behavior.
pub enum TableColumnFixed {
    /// Pins the column to the leading side in data-table layouts.
    Left,
    /// Pins the column to the trailing side in data-table layouts.
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Options that control table sort order behavior.
pub enum TableSortOrder {
    /// Sorts table rows in ascending order.
    Ascending,
    /// Sorts table rows in descending order.
    Descending,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Fluent native GPUI component for rendering Liora table sort state.
pub struct TableSortState {
    /// Stable key used to identify this entry in collections and callbacks.
    pub key: SharedString,
    /// Current sort order for this column, if sorting is enabled.
    pub order: Option<TableSortOrder>,
}

/// Data model used by table column rendering.
pub struct TableColumn {
    /// Stable key used to identify this entry in collections and callbacks.
    pub key: SharedString,
    /// User-facing label rendered for this item.
    pub label: SharedString,
    /// Custom header element rendered instead of a text label.
    pub header: Option<AnyElement>,
    /// Width used by layout or hit-testing calculations.
    pub width: Option<Pixels>,
    /// Minimum column width requested by table layout.
    pub min_width: Pixels,
    /// Horizontal alignment for cell content.
    pub align: TableAlign,
    /// Whether the column can be toggled through sort states.
    pub sortable: bool,
    /// Optional fixed edge used by data-table and virtualized-table layouts.
    pub fixed: Option<TableColumnFixed>,
}

/// Data model used by table cell rendering.
pub struct TableCell {
    /// Stable key used to identify this entry in collections and callbacks.
    pub key: SharedString,
    /// Machine-readable value represented by this item.
    pub value: AnyElement,
}

/// Data model used by table row rendering.
pub struct TableRow {
    cells: Vec<TableCell>,
}

/// Fluent native GPUI component for rendering Liora table.
pub struct Table {
    id: SharedString,
    columns: Vec<TableColumn>,
    rows: Vec<TableRow>,
    border: bool,
    stripe: bool,
    loading: bool,
    fixed_header: bool,
    height: Option<Pixels>,
    empty_text: SharedString,
    sort_key: Option<SharedString>,
    sort_order: Option<TableSortOrder>,
    on_sort_change: Option<Arc<dyn Fn(TableSortState, &mut Window, &mut App) + 'static>>,
}

impl TableColumn {
    /// Creates `TableColumn` initialized from the supplied key, and label.
    pub fn new(key: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            header: None,
            width: None,
            min_width: px(120.0),
            align: TableAlign::Left,
            sortable: false,
            fixed: None,
        }
    }

    /// Sets the header value used by the component.
    pub fn header(mut self, header: impl IntoElement) -> Self {
        self.header = Some(header.into_any_element());
        self
    }

    /// Sets the component width token used during GPUI layout.
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Applies the predefined width sm sizing preset.
    pub fn width_sm(self) -> Self {
        self.width(px(120.0))
    }

    /// Sets the minimum width limit.
    pub fn min_width(mut self, width: impl Into<Pixels>) -> Self {
        self.min_width = width.into();
        self
    }

    /// Applies the predefined min width lg sizing preset.
    pub fn min_width_lg(self) -> Self {
        self.min_width(px(260.0))
    }

    /// Sets cross-axis alignment for child content.
    pub fn align(mut self, align: TableAlign) -> Self {
        self.align = align;
        self
    }

    /// Toggles sortable behavior.
    pub fn sortable(mut self) -> Self {
        self.sortable = true;
        self
    }

    /// Marks the column as fixed to the leading side for data-table layouts.
    pub fn fixed_left(mut self) -> Self {
        self.fixed = Some(TableColumnFixed::Left);
        self
    }

    /// Marks the column as fixed to the trailing side for data-table layouts.
    pub fn fixed_right(mut self) -> Self {
        self.fixed = Some(TableColumnFixed::Right);
        self
    }

    /// Assigns an explicit fixed-column edge.
    pub fn fixed(mut self, fixed: Option<TableColumnFixed>) -> Self {
        self.fixed = fixed;
        self
    }
}

impl TableRow {
    /// Creates `TableRow` with default theme-driven styling and no optional callbacks attached.
    pub fn new() -> Self {
        Self { cells: vec![] }
    }

    /// Adds the supplied cell to the component.
    pub fn cell(mut self, key: impl Into<SharedString>, value: impl IntoElement) -> Self {
        self.cells.push(TableCell {
            key: key.into(),
            value: value.into_any_element(),
        });
        self
    }

    fn take_cell(&mut self, key: &SharedString) -> Option<AnyElement> {
        self.cells
            .iter()
            .position(|cell| &cell.key == key)
            .map(|index| self.cells.remove(index).value)
    }
}

impl Table {
    /// Creates `Table` that renders the supplied columns collection.
    pub fn new(columns: Vec<TableColumn>) -> Self {
        Self {
            id: liora_core::unique_id("table"),
            columns,
            rows: vec![],
            border: false,
            stripe: false,
            loading: false,
            fixed_header: false,
            height: None,
            empty_text: "暂无数据".into(),
            sort_key: None,
            sort_order: None,
            on_sort_change: None,
        }
    }

    /// Assigns a stable element id used by GPUI state, hit testing, and automated interaction tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Sets the row value used by the component.
    pub fn row(mut self, row: TableRow) -> Self {
        self.rows.push(row);
        self
    }

    /// Sets the visible row count for editor-like controls.
    pub fn rows(mut self, rows: impl IntoIterator<Item = TableRow>) -> Self {
        self.rows.extend(rows);
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

    /// Sets the fixed header value used by the component.
    pub fn fixed_header(mut self, fixed_header: bool) -> Self {
        self.fixed_header = fixed_header;
        self
    }

    /// Sets the component height token used during GPUI layout.
    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Applies the predefined height md sizing preset.
    pub fn height_md(self) -> Self {
        self.height(px(260.0))
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
        f: impl Fn(TableSortState, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_sort_change = Some(Arc::new(f));
        self
    }
}

impl RenderOnce for Table {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let mut columns = self.columns;
        let has_rows = !self.rows.is_empty();
        let border = self.border;
        let stripe = self.stripe;
        let fixed_header = self.fixed_header || self.height.is_some();
        let height = self.height;
        let body_id = format!("{}-body", self.id);
        let sort_key = self.sort_key;
        let sort_order = self.sort_order;
        let on_sort_change = self.on_sort_change;

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
                table_header_cell(
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
                .flex()
                .flex_col()
                .w_full()
                .children(
                    self.rows
                        .into_iter()
                        .enumerate()
                        .map(|(row_index, mut row)| {
                            let striped = stripe && row_index % 2 == 1;
                            div()
                                .flex()
                                .flex_row()
                                .w_full()
                                .bg(if striped {
                                    theme.neutral.hover.opacity(0.45)
                                } else {
                                    theme.neutral.card
                                })
                                .hover(|s| s.bg(theme.primary.light_9))
                                .when(row_index > 0, |s| {
                                    s.border_t_1().border_color(theme.neutral.divider)
                                })
                                .children(columns.iter().enumerate().map(move |(index, column)| {
                                    let value = row
                                        .take_cell(&column.key)
                                        .unwrap_or_else(|| div().into_any_element());
                                    table_cell_shell(column, border, index)
                                        .min_h(px(48.0))
                                        .py_3()
                                        .child(
                                            div()
                                                .text_size(px(theme.font_size.sm))
                                                .text_color(theme.neutral.text_1)
                                                .child(value),
                                        )
                                }))
                        }),
                )
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

        let body = div()
            .w_full()
            .id(element_id(body_id))
            .when(fixed_header, |s| s.overflow_y_scroll())
            .when_some(height, |s, h| s.max_h(h))
            .child(body);

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

impl IntoElement for Table {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

fn table_cell_shell(column: &TableColumn, border: bool, index: usize) -> gpui::Div {
    let mut cell = div()
        .flex()
        .items_center()
        .px_4()
        .min_w(column.min_width)
        .when(border && index > 0, |s| s.border_l_1());

    cell = match column.width {
        Some(width) => cell.w(width).flex_shrink_0(),
        None => cell.flex_1(),
    };

    match column.align {
        TableAlign::Left => cell.justify_start(),
        TableAlign::Center => cell.justify_center(),
        TableAlign::Right => cell.justify_end(),
    }
}

fn table_header_cell(
    column: &mut TableColumn,
    border: bool,
    index: usize,
    theme: &liora_theme::Theme,
    active_order: Option<TableSortOrder>,
    on_sort_change: Option<Arc<dyn Fn(TableSortState, &mut Window, &mut App) + 'static>>,
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

    let cell = table_cell_shell(column, border, index)
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
        .on_mouse_up(MouseButton::Left, move |_, window, cx| {
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
