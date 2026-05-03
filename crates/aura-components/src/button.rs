use gpui::{prelude::*, px, SharedString, Hsla};
use aura_theme::{ButtonVariant, ButtonSize, ButtonVariantColors, AuraTheme};

fn rgba(r: u8, g: u8, b: u8, a: f32) -> Hsla {
    gpui::hsla(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a)
}

pub struct AuraButton {
    label: SharedString,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
    loading: bool,
}

impl AuraButton {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            disabled: false,
            loading: false,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn primary(mut self) -> Self   { self.variant = ButtonVariant::Primary; self }
    pub fn tertiary(mut self) -> Self  { self.variant = ButtonVariant::Tertiary; self }
    pub fn info(mut self) -> Self      { self.variant = ButtonVariant::Info; self }
    pub fn success(mut self) -> Self   { self.variant = ButtonVariant::Success; self }
    pub fn warning(mut self) -> Self   { self.variant = ButtonVariant::Warning; self }
    pub fn danger(mut self) -> Self    { self.variant = ButtonVariant::Danger; self }

    pub fn size(mut self, size: ButtonSize) -> Self { self.size = size; self }

    pub fn small(mut self) -> Self  { self.size = ButtonSize::Small; self }
    pub fn large(mut self) -> Self  { self.size = ButtonSize::Large; self }

    pub fn disabled(mut self, disabled: bool) -> Self { self.disabled = disabled; self }
    pub fn loading(mut self, loading: bool) -> Self   { self.loading = loading; self }

    pub fn build(self, theme: &AuraTheme) -> impl IntoElement {
        let height = self.size.height();
        let padding_x = self.size.padding_x();
        let font_size = match self.size {
            ButtonSize::Small => theme.font_size.xs,
            ButtonSize::Default => theme.font_size.md,
            ButtonSize::Large => theme.font_size.lg,
        };
        let radius = theme.radius.md;

        let colors = if self.disabled {
            ButtonVariantColors {
                bg: rgba(0,0,0,0.0),
                hover_bg: rgba(0,0,0,0.0),
                active_bg: rgba(0,0,0,0.0),
                text: theme.neutral.text_disabled,
                border: theme.neutral.border,
                text_hover: theme.neutral.text_disabled,
                border_hover: theme.neutral.border,
            }
        } else {
            theme.color_by_variant(self.variant)
        };

        let label_text = if self.loading {
            SharedString::from(format!("⟳ {}", self.label))
        } else {
            self.label.clone()
        };

        let mut el = gpui::div()
            .flex()
            .flex_row()
            .justify_center()
            .items_center()
            .gap_1()
            .h(px(height))
            .px(px(padding_x))
            .rounded(px(radius))
            .bg(colors.bg)
            .text_color(colors.text)
            .border_1()
            .border_color(colors.border)
            .text_size(px(font_size));

        if !self.disabled {
            el = el.hover(|style| {
                style
                    .bg(colors.hover_bg)
                    .text_color(colors.text_hover)
                    .border_color(colors.border_hover)
                    .cursor_pointer()
            });
        } else {
            el = el.cursor_default();
        }

        el.child(label_text).into_any_element()
    }
}
