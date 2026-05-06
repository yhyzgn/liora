use aura_core::Config;
use gpui::{AnyElement, App, Hsla, IntoElement, RenderOnce, SharedString, Window, div, prelude::*};

pub struct Statistic {
    title: SharedString,
    value: SharedString,
    prefix: Option<AnyElement>,
    suffix: Option<AnyElement>,
    value_color: Option<Hsla>,
}

impl Statistic {
    pub fn new(title: impl Into<SharedString>, value: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            value: value.into(),
            prefix: None,
            suffix: None,
            value_color: None,
        }
    }

    pub fn prefix(mut self, prefix: impl IntoElement) -> Self {
        self.prefix = Some(prefix.into_any_element());
        self
    }

    pub fn suffix(mut self, suffix: impl IntoElement) -> Self {
        self.suffix = Some(suffix.into_any_element());
        self
    }

    pub fn value_color(mut self, color: Hsla) -> Self {
        self.value_color = Some(color);
        self
    }
}

impl RenderOnce for Statistic {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        div()
            .flex()
            .flex_col()
            .gap_1()
            .child(
                div()
                    .text_sm()
                    .text_color(theme.neutral.text_3)
                    .child(self.title),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_baseline()
                    .gap_1()
                    .when_some(self.prefix, |s, p| s.child(p))
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(self.value_color.unwrap_or(theme.neutral.text_1))
                            .child(self.value),
                    )
                    .when_some(self.suffix, |s, p| s.child(p)),
            )
    }
}

impl IntoElement for Statistic {
    type Element = gpui::Component<Self>;
    fn into_element(self) -> Self::Element {
        gpui::Component::new(self)
    }
}
