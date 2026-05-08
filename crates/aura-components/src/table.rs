use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    AnyElement, App, Component, IntoElement, Pixels, RenderOnce, SharedString, Window, div,
    prelude::*, px,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TableAlign {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Clone)]
pub struct TableColumn {
    pub key: SharedString,
    pub label: SharedString,
    pub width: Option<Pixels>,
    pub min_width: Pixels,
    pub align: TableAlign,
}

pub struct TableCell {
    pub key: SharedString,
    pub value: AnyElement,
}

pub struct TableRow {
    cells: Vec<TableCell>,
}

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
}

impl TableColumn {
    pub fn new(key: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            width: None,
            min_width: px(120.0),
            align: TableAlign::Left,
        }
    }

    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn min_width(mut self, width: impl Into<Pixels>) -> Self {
        self.min_width = width.into();
        self
    }

    pub fn align(mut self, align: TableAlign) -> Self {
        self.align = align;
        self
    }
}

impl TableRow {
    pub fn new() -> Self {
        Self { cells: vec![] }
    }

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
    #[track_caller]
    pub fn new(columns: Vec<TableColumn>) -> Self {
        let caller = std::panic::Location::caller();
        Self {
            id: format!("table-{}", caller).into(),
            columns,
            rows: vec![],
            border: false,
            stripe: false,
            loading: false,
            fixed_header: false,
            height: None,
            empty_text: "暂无数据".into(),
        }
    }

    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    pub fn row(mut self, row: TableRow) -> Self {
        self.rows.push(row);
        self
    }

    pub fn rows(mut self, rows: impl IntoIterator<Item = TableRow>) -> Self {
        self.rows.extend(rows);
        self
    }

    pub fn border(mut self, border: bool) -> Self {
        self.border = border;
        self
    }

    pub fn stripe(mut self, stripe: bool) -> Self {
        self.stripe = stripe;
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn fixed_header(mut self, fixed_header: bool) -> Self {
        self.fixed_header = fixed_header;
        self
    }

    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn empty_text(mut self, text: impl Into<SharedString>) -> Self {
        self.empty_text = text.into();
        self
    }
}

impl RenderOnce for Table {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let columns = self.columns;
        let has_rows = !self.rows.is_empty();
        let border = self.border;
        let stripe = self.stripe;
        let fixed_header = self.fixed_header || self.height.is_some();
        let height = self.height;
        let body_id = format!("{}-body", self.id);

        let header = div()
            .flex()
            .flex_row()
            .w_full()
            .bg(theme.neutral.hover)
            .border_b_1()
            .border_color(theme.neutral.border)
            .children(columns.iter().enumerate().map(|(index, column)| {
                table_cell_shell(column, border, index).py_3().child(
                    div()
                        .text_size(px(theme.font_size.sm))
                        .font_weight(gpui::FontWeight::BOLD)
                        .text_color(theme.neutral.text_2)
                        .child(column.label.clone()),
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
            .id(body_id)
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
