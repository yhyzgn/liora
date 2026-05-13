use crate::{SelectableText, SelectableTextOptions, SelectableTextWrap};
use aura_core::Config;
use gpui::{
    App, Component, ElementId, IntoElement, RenderOnce, SharedString, TextStyle, Window,
    prelude::*, px,
};

pub struct Title {
    content: SharedString,
    level: u8,
    selectable: bool,
    id: SharedString,
}

impl Title {
    pub fn new(content: impl Into<SharedString>) -> Self {
        Self {
            content: content.into(),
            level: 1,
            selectable: true,
            id: aura_core::unique_id("title"),
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

    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }

    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    pub fn register_key_bindings(cx: &mut App) {
        SelectableText::register_key_bindings(cx);
    }

    fn render_with_theme(
        self,
        theme: &aura_theme::Theme,
        window: &mut Window,
        cx: &mut App,
    ) -> gpui::AnyElement {
        let (size, weight) = match self.level {
            1 => (theme.font_size.xl + 4.0, gpui::FontWeight::BOLD),
            2 => (theme.font_size.xl, gpui::FontWeight::BOLD),
            3 => (theme.font_size.lg + 2.0, gpui::FontWeight::BOLD),
            4 => (theme.font_size.lg, gpui::FontWeight::BOLD),
            5 => (theme.font_size.md, gpui::FontWeight::BOLD),
            _ => (theme.font_size.sm, gpui::FontWeight::BOLD),
        };

        let font_size = px(size);
        let line_height = font_size * 1.35;
        let text_color = theme.neutral.text_1;

        if self.selectable {
            let mut style = TextStyle::default();
            style.color = text_color;
            style.font_size = font_size.into();
            style.line_height = line_height.into();
            style.font_weight = weight;
            style.white_space = gpui::WhiteSpace::Normal;
            return SelectableText::view(
                SelectableTextOptions {
                    id: ElementId::from(self.id.clone()),
                    text: self.content.clone(),
                    runs: vec![style.to_run(self.content.len())],
                    font_size,
                    line_height,
                    text_color,
                    wrap: SelectableTextWrap::Normal,
                    key_context: "SelectableText",
                    fill_width: true,
                },
                window,
                cx,
            );
        }

        gpui::div()
            .text_size(font_size)
            .line_height(line_height)
            .font_weight(weight)
            .text_color(text_color)
            .child(self.content.clone())
            .into_any_element()
    }
}

impl RenderOnce for Title {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        self.render_with_theme(&theme, _window, cx)
    }
}

impl IntoElement for Title {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn title_uses_selectable_text_for_native_selection() {
        let source = include_str!("title.rs");
        assert!(source.contains("SelectableText::view"));
        assert!(source.contains("pub fn selectable"));
        assert!(source.contains("pub fn register_key_bindings"));
    }
}
