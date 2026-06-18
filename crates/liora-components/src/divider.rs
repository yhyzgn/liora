use gpui::{App, Component, IntoElement, RenderOnce, Window, prelude::*, px};

pub struct Divider {
    vertical: bool,
    label: Option<String>,
}

impl Divider {
    pub fn new() -> Self {
        Self {
            vertical: false,
            label: None,
        }
    }

    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }

    pub fn label(mut self, text: impl Into<String>) -> Self {
        self.label = Some(text.into());
        self
    }
}

impl RenderOnce for Divider {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<liora_core::Config>().theme;

        if self.vertical {
            gpui::div().w(px(1.0)).h_full().bg(theme.neutral.border)
        } else if let Some(text) = self.label {
            gpui::div()
                .flex()
                .flex_row()
                .items_center()
                .gap_2()
                .w_full()
                .child(gpui::div().flex_1().h(px(1.0)).bg(theme.neutral.border))
                .child(
                    gpui::div()
                        .text_size(px(theme.font_size.sm))
                        .text_color(theme.neutral.text_3)
                        .child(text),
                )
                .child(gpui::div().flex_1().h(px(1.0)).bg(theme.neutral.border))
        } else {
            gpui::div().w_full().h(px(1.0)).bg(theme.neutral.border)
        }
    }
}

impl IntoElement for Divider {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
