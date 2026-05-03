use aura_theme::{AuraTheme, ButtonSize, ButtonVariant, ButtonVariantColors};
use gpui::{ElementId, Hsla, Rgba, SharedString, prelude::*, px};

fn rgba(r: u8, g: u8, b: u8, a: f32) -> Hsla {
    Rgba {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a,
    }
    .into()
}

pub struct AuraButton {
    label: SharedString,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
    loading: bool,
    secondary: bool,
    background: bool,
    border: bool,
    rounded: Option<f32>,
    id: Option<ElementId>,
}

impl AuraButton {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            disabled: false,
            loading: false,
            secondary: false,
            background: true,
            border: true,
            rounded: None,
            id: None,
        }
    }
    pub fn variant(mut self, v: ButtonVariant) -> Self {
        self.variant = v;
        self
    }
    pub fn primary(mut self) -> Self {
        self.variant = ButtonVariant::Primary;
        self
    }
    pub fn tertiary(mut self) -> Self {
        self.variant = ButtonVariant::Tertiary;
        self
    }
    pub fn info(mut self) -> Self {
        self.variant = ButtonVariant::Info;
        self
    }
    pub fn success(mut self) -> Self {
        self.variant = ButtonVariant::Success;
        self
    }
    pub fn warning(mut self) -> Self {
        self.variant = ButtonVariant::Warning;
        self
    }
    pub fn danger(mut self) -> Self {
        self.variant = ButtonVariant::Danger;
        self
    }
    pub fn size(mut self, s: ButtonSize) -> Self {
        self.size = s;
        self
    }
    pub fn small(mut self) -> Self {
        self.size = ButtonSize::Small;
        self
    }
    pub fn large(mut self) -> Self {
        self.size = ButtonSize::Large;
        self
    }
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }
    pub fn loading(mut self, l: bool) -> Self {
        self.loading = l;
        self
    }
    pub fn secondary(mut self) -> Self {
        self.secondary = true;
        self
    }
    pub fn background(mut self, show: bool) -> Self {
        self.background = show;
        self
    }
    pub fn border(mut self, show: bool) -> Self {
        self.border = show;
        self
    }
    pub fn rounded(mut self, r: f32) -> Self {
        self.rounded = Some(r);
        self
    }
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    fn colors(&self, theme: &AuraTheme) -> ButtonVariantColors {
        if self.disabled {
            ButtonVariantColors {
                bg: rgba(0, 0, 0, 0.0),
                hover_bg: rgba(0, 0, 0, 0.0),
                active_bg: rgba(0, 0, 0, 0.0),
                text: theme.neutral.text_disabled,
                border: theme.neutral.border,
                text_hover: theme.neutral.text_disabled,
                border_hover: theme.neutral.border,
            }
        } else {
            theme.color_by_variant(self.variant, self.secondary, self.background, self.border)
        }
    }

    /// Build a theme-explicit GPUI element.
    #[track_caller]
    pub fn build(self, theme: &AuraTheme) -> impl IntoElement {
        let c = self.colors(theme);
        let h = self.size.height();
        let px_h = self.size.padding_x();
        let fs = match self.size {
            ButtonSize::Small => theme.font_size.xs,
            ButtonSize::Default => theme.font_size.md,
            ButtonSize::Large => theme.font_size.lg,
        };
        let r = self.rounded.unwrap_or(theme.radius.md);
        let label = if self.loading {
            SharedString::from(format!("⟳ {}", self.label))
        } else {
            self.label.clone()
        };
        let id = self.id.unwrap_or_else(|| {
            SharedString::from(format!(
                "aura-button:{}:{}",
                std::panic::Location::caller(),
                self.label
            ))
            .into()
        });

        let mut div = gpui::div()
            .flex()
            .flex_row()
            .justify_center()
            .items_center()
            .gap_1()
            .h(px(h))
            .px(px(px_h))
            .rounded(px(r))
            .bg(c.bg)
            .text_color(c.text)
            .text_size(px(fs));

        if !self.disabled {
            div = div.cursor_pointer();
        } else {
            div = div.cursor_not_allowed();
        }
        if !c.border.is_transparent() {
            div = div.border_1().border_color(c.border);
        }
        if self.disabled {
            return div.child(label).into_any_element();
        }

        div.id(id)
            .hover(move |style| {
                let mut s = style.bg(c.hover_bg).text_color(c.text_hover);
                if !c.border_hover.is_transparent() {
                    s = s.border_color(c.border_hover);
                }
                s
            })
            .active(move |style| style.bg(c.active_bg))
            .child(label)
            .into_any_element()
    }
}
