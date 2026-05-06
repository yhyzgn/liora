use aura_core::Config;
use gpui::{App, Component, IntoElement, RenderOnce, SharedString, Window, prelude::*, px};

pub struct Title {
    content: SharedString,
    level: u8,
}

impl Title {
    pub fn new(content: impl Into<SharedString>) -> Self {
        Self {
            content: content.into(),
            level: 1,
        }
    }

    pub fn h1(mut self) -> Self {
        self.level = 1;
        self
    }
    pub fn h2(mut self) -> Self {
        self.level = 2;
        self
    }
    pub fn h3(mut self) -> Self {
        self.level = 3;
        self
    }
    pub fn h4(mut self) -> Self {
        self.level = 4;
        self
    }
    pub fn h5(mut self) -> Self {
        self.level = 5;
        self
    }
    pub fn h6(mut self) -> Self {
        self.level = 6;
        self
    }

    fn render_with_theme(self, theme: &aura_theme::Theme) -> impl IntoElement {
        let (size, weight) = match self.level {
            1 => (theme.font_size.xl + 4.0, gpui::FontWeight::BOLD),
            2 => (theme.font_size.xl, gpui::FontWeight::BOLD),
            3 => (theme.font_size.lg + 2.0, gpui::FontWeight::BOLD),
            4 => (theme.font_size.lg, gpui::FontWeight::BOLD),
            5 => (theme.font_size.md, gpui::FontWeight::BOLD),
            _ => (theme.font_size.sm, gpui::FontWeight::BOLD),
        };

        gpui::div()
            .text_size(px(size))
            .font_weight(weight)
            .text_color(theme.neutral.text_1)
            .child(self.content.clone())
    }
}

impl RenderOnce for Title {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        self.render_with_theme(theme)
    }
}

impl IntoElement for Title {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
