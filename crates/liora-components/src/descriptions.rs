//! Descriptions module.
//!
//! This public module implements the Liora description-list component for structured key/value details. It keeps the reusable
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

use gpui::{AnyElement, App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px};
use liora_core::Config;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control descriptions direction behavior.
pub enum DescriptionsDirection {
    #[default]
    /// Lays out content in the horizontal direction.
    Horizontal,
    /// Lays out content in the vertical direction.
    Vertical,
}

/// Data model used by description item rendering.
pub struct DescriptionItem {
    /// User-facing label rendered for this item.
    pub label: SharedString,
    /// Machine-readable value represented by this item.
    pub value: AnyElement,
    /// Number of columns occupied by this descriptions item.
    pub span: u32,
}

/// Fluent native GPUI component for rendering Liora descriptions.
pub struct Descriptions {
    title: Option<SharedString>,
    extra: Option<AnyElement>,
    column: u32,
    direction: DescriptionsDirection,
    border: bool,
    items: Vec<DescriptionItem>,
}

impl Descriptions {
    /// Creates `Descriptions` with default theme-driven styling and no optional callbacks attached.
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

    /// Sets the primary title text displayed by the component.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the extra value used by the component.
    pub fn extra(mut self, extra: impl IntoElement) -> Self {
        self.extra = Some(extra.into_any_element());
        self
    }

    /// Sets the column value used by the component.
    pub fn column(mut self, c: u32) -> Self {
        self.column = c.max(1);
        self
    }

    /// Selects the layout or animation direction.
    pub fn direction(mut self, d: DescriptionsDirection) -> Self {
        self.direction = d;
        self
    }

    /// Toggles or applies the component border treatment.
    pub fn border(mut self, b: bool) -> Self {
        self.border = b;
        self
    }

    /// Performs the item operation used by this component.
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
                                                .items_start()
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
