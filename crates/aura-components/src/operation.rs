use crate::Label;
use aura_core::Config;
use gpui::{
    AnyElement, App, Component, Hsla, IntoElement, Pixels, RenderOnce, SharedString, Window, div,
    prelude::*, px,
};

pub struct Operation {
    label: AnyElement,
    action: AnyElement,
    description: Option<SharedString>,
    status: Option<SharedString>,
    status_color: Option<Hsla>,
    gap: Pixels,
    padded: bool,
    disabled: bool,
}

impl Operation {
    pub fn new(label: impl IntoElement, action: impl IntoElement) -> Self {
        Self {
            label: label.into_any_element(),
            action: action.into_any_element(),
            description: None,
            status: None,
            status_color: None,
            gap: px(16.0),
            padded: true,
            disabled: false,
        }
    }

    pub fn with_text(text: impl Into<gpui::SharedString>, action: impl IntoElement) -> Self {
        Self::new(Label::new(text), action)
    }
    pub fn gap(mut self, gap: Pixels) -> Self {
        self.gap = gap.max(px(0.0));
        self
    }
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
    pub fn status(mut self, status: impl Into<SharedString>) -> Self {
        self.status = Some(status.into());
        self
    }
    pub fn status_color(mut self, color: Hsla) -> Self {
        self.status_color = Some(color);
        self
    }
    pub fn success(self) -> Self {
        self.status("正常").status_color(gpui::green())
    }
    pub fn warning(self) -> Self {
        self.status("注意").status_color(gpui::yellow())
    }
    pub fn danger(self) -> Self {
        self.status("异常").status_color(gpui::red())
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn no_padding(mut self) -> Self {
        self.padded = false;
        self
    }
}

impl RenderOnce for Operation {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let status_color = self.status_color.unwrap_or(theme.primary.base);
        div()
            .flex()
            .items_center()
            .justify_between()
            .gap(self.gap)
            .w_full()
            .when(self.disabled, |s| s.opacity(0.52))
            .when(self.padded, |s| {
                s.p_3()
                    .rounded_md()
                    .border_1()
                    .border_color(theme.neutral.border)
                    .bg(theme.neutral.card)
            })
            .child(
                div()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(self.label)
                            .when_some(self.status, |s, status| {
                                s.child(
                                    div()
                                        .rounded_full()
                                        .px_2()
                                        .py(px(1.0))
                                        .text_xs()
                                        .bg(status_color.opacity(0.12))
                                        .text_color(status_color)
                                        .child(status),
                                )
                            }),
                    )
                    .when_some(self.description, |s, description| {
                        s.child(
                            div()
                                .text_sm()
                                .text_color(theme.neutral.text_3)
                                .child(description),
                        )
                    }),
            )
            .child(div().flex_none().child(self.action))
    }
}

impl IntoElement for Operation {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn operation_tracks_layout_options() {
        let op = Operation::with_text("Auto save", div())
            .gap(px(20.0))
            .description("Save changes automatically")
            .status("Enabled")
            .disabled(true)
            .no_padding();
        assert_eq!(op.gap, px(20.0));
        assert!(!op.padded);
        assert_eq!(
            op.description.as_deref(),
            Some("Save changes automatically")
        );
        assert_eq!(op.status.as_deref(), Some("Enabled"));
        assert!(op.disabled);
    }
}
