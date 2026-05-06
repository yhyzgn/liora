use aura_core::Config;
use gpui::{AnyElement, App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DescriptionsDirection {
    #[default]
    Horizontal,
    Vertical,
}

pub struct DescriptionItem {
    pub label: SharedString,
    pub value: AnyElement,
    pub span: u32,
}

pub struct Descriptions {
    title: Option<SharedString>,
    extra: Option<AnyElement>,
    column: u32,
    direction: DescriptionsDirection,
    border: bool,
    items: Vec<DescriptionItem>,
}

impl Descriptions {
    pub fn new() -> Self {
        Self {
            title: None,
            extra: None,
            column: 3,
            direction: DescriptionsDirection::Horizontal,
            border: false,
            items: vec![],
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn extra(mut self, extra: impl IntoElement) -> Self {
        self.extra = Some(extra.into_any_element());
        self
    }

    pub fn column(mut self, c: u32) -> Self {
        self.column = c.max(1);
        self
    }

    pub fn direction(mut self, d: DescriptionsDirection) -> Self {
        self.direction = d;
        self
    }

    pub fn border(mut self, b: bool) -> Self {
        self.border = b;
        self
    }

    pub fn item(
        mut self,
        label: impl Into<SharedString>,
        value: impl IntoElement,
        span: u32,
    ) -> Self {
        self.items.push(DescriptionItem {
            label: label.into(),
            value: value.into_any_element(),
            span: span.max(1),
        });
        self
    }
}

impl RenderOnce for Descriptions {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let column = self.column;
        let border = self.border;
        let direction = self.direction;

        // Group items into rows
        let mut rows: Vec<Vec<DescriptionItem>> = vec![];
        let mut current_row: Vec<DescriptionItem> = vec![];
        let mut current_span = 0;

        for item in self.items {
            let item_span = item.span.min(column);
            if current_span + item_span > column {
                if let Some(last) = current_row.last_mut() {
                    last.span += column - current_span;
                }
                rows.push(current_row);
                current_row = vec![];
                current_span = 0;
            }
            current_span += item_span;
            current_row.push(item);
        }
        if !current_row.is_empty() {
            if let Some(last) = current_row.last_mut() {
                last.span += column - current_span;
            }
            rows.push(current_row);
        }

        div()
            .flex()
            .flex_col()
            .w_full()
            .gap_4()
            .when(self.title.is_some() || self.extra.is_some(), |s| {
                s.child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .when_some(self.title, |s, t| {
                            s.child(
                                div()
                                    .text_lg()
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .text_color(theme.neutral.text_1)
                                    .child(t),
                            )
                        })
                        .when_some(self.extra, |s, e| s.child(e)),
                )
            })
            .child(
                div()
                    .flex()
                    .flex_col()
                    .when(border, |s| {
                        s.border_1()
                            .border_color(theme.neutral.border)
                            .rounded(px(theme.radius.sm))
                            .overflow_hidden()
                    })
                    .children(rows.into_iter().enumerate().map(|(row_idx, row)| {
                        div()
                            .flex()
                            .flex_row()
                            .w_full()
                            .when(border && row_idx > 0, |s| {
                                s.border_t_1().border_color(theme.neutral.border)
                            })
                            .children(row.into_iter().enumerate().map(|(col_idx, item)| {
                                let width = gpui::relative(item.span as f32 / column as f32);
                                let cell = div().w(width).flex().when(border && col_idx > 0, |s| {
                                    s.border_l_1().border_color(theme.neutral.border)
                                });

                                match direction {
                                    DescriptionsDirection::Horizontal => {
                                        if border {
                                            cell.flex_row()
                                                .items_stretch()
                                                .child(
                                                    div()
                                                        .p_3()
                                                        .bg(theme.neutral.hover)
                                                        .border_r_1()
                                                        .border_color(theme.neutral.border)
                                                        .flex()
                                                        .items_center()
                                                        .child(
                                                            div()
                                                                .text_sm()
                                                                .font_weight(gpui::FontWeight::BOLD)
                                                                .text_color(theme.neutral.text_2)
                                                                .child(item.label),
                                                        ),
                                                )
                                                .child(
                                                    div()
                                                        .flex_1()
                                                        .p_3()
                                                        .flex()
                                                        .items_center()
                                                        .child(
                                                            div()
                                                                .text_sm()
                                                                .text_color(theme.neutral.text_1)
                                                                .child(item.value),
                                                        ),
                                                )
                                        } else {
                                            cell.flex_row()
                                                .items_center()
                                                .gap_2()
                                                .p_1()
                                                .child(
                                                    div()
                                                        .text_sm()
                                                        .text_color(theme.neutral.text_3)
                                                        .child(format!("{}:", item.label)),
                                                )
                                                .child(
                                                    div()
                                                        .text_sm()
                                                        .text_color(theme.neutral.text_1)
                                                        .child(item.value),
                                                )
                                        }
                                    }
                                    DescriptionsDirection::Vertical => {
                                        if border {
                                            cell.flex_col()
                                                .child(
                                                    div()
                                                        .p_3()
                                                        .bg(theme.neutral.hover)
                                                        .border_b_1()
                                                        .border_color(theme.neutral.border)
                                                        .child(
                                                            div()
                                                                .text_sm()
                                                                .font_weight(gpui::FontWeight::BOLD)
                                                                .text_color(theme.neutral.text_2)
                                                                .child(item.label),
                                                        ),
                                                )
                                                .child(
                                                    div().p_3().child(
                                                        div()
                                                            .text_sm()
                                                            .text_color(theme.neutral.text_1)
                                                            .child(item.value),
                                                    ),
                                                )
                                        } else {
                                            cell.flex_col()
                                                .gap_1()
                                                .p_1()
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(theme.neutral.text_3)
                                                        .child(item.label),
                                                )
                                                .child(
                                                    div()
                                                        .text_sm()
                                                        .text_color(theme.neutral.text_1)
                                                        .child(item.value),
                                                )
                                        }
                                    }
                                }
                            }))
                    })),
            )
    }
}

impl IntoElement for Descriptions {
    type Element = gpui::Component<Self>;
    fn into_element(self) -> Self::Element {
        gpui::Component::new(self)
    }
}
