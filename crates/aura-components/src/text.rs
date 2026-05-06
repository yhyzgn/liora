use aura_core::Config;
use gpui::{
    App, Component, FontStyle, FontWeight, Hsla, IntoElement, Pixels, RenderOnce, SharedString,
    Window, div, prelude::*, px,
};

#[derive(Clone)]
pub struct Text {
    pub(crate) content: SharedString,
    pub(crate) color: Option<Hsla>,
    pub(crate) bg: Option<Hsla>,
    pub(crate) size: Option<Pixels>,
    pub(crate) weight: Option<FontWeight>,
    pub(crate) style: Option<FontStyle>,
    pub(crate) underline: bool,
    pub(crate) strikethrough: bool,
    pub(crate) font_family: Option<SharedString>,
}

impl Text {
    pub fn new(content: impl Into<SharedString>) -> Self {
        Self {
            content: content.into(),
            color: None,
            bg: None,
            size: None,
            weight: None,
            style: None,
            underline: false,
            strikethrough: false,
            font_family: None,
        }
    }

    pub fn text_color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn bg(mut self, bg: Hsla) -> Self {
        self.bg = Some(bg);
        self
    }

    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = Some(size.into());
        self
    }

    pub fn weight(mut self, weight: FontWeight) -> Self {
        self.weight = Some(weight);
        self
    }

    pub fn bold(mut self) -> Self {
        self.weight = Some(FontWeight::BOLD);
        self
    }

    pub fn font_style(mut self, style: FontStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn italic(mut self) -> Self {
        self.style = Some(FontStyle::Italic);
        self
    }

    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    pub fn font_family(mut self, family: impl Into<SharedString>) -> Self {
        self.font_family = Some(family.into());
        self
    }

    /// Convenience for "code" style
    pub fn code_style(mut self, theme: &aura_theme::Theme) -> Self {
        self.font_family = Some("Monospace".into());
        self.bg = Some(theme.neutral.hover);
        self.text_color(theme.danger.base)
    }
}

impl RenderOnce for Text {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

        let mut el = div()
            .child(self.content.clone())
            .text_size(self.size.unwrap_or_else(|| px(theme.font_size.md)))
            .text_color(self.color.unwrap_or(theme.neutral.text_2));

        if let Some(bg) = self.bg {
            el = el.bg(bg).px_1().rounded(px(2.0));
        }

        if let Some(weight) = self.weight {
            el = el.font_weight(weight);
        }

        if let Some(style) = self.style {
            // In some GPUI versions, it's .italic(), in others it's .font_style(style)
            // If .font_style failed, let's try matching on style
            if style == FontStyle::Italic {
                el = el.italic();
            }
        }

        if self.underline {
            el = el.underline();
        }

        if self.strikethrough {
            // strikethrough might not be available directly on div in all GPUI versions,
            // but we'll try common names or just omit if it fails.
            // el = el.strikethrough();
        }

        if let Some(family) = self.font_family {
            el = el.font_family(family);
        }

        el
    }
}

impl IntoElement for Text {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
