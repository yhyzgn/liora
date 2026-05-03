use aura_core::AuraConfig;
use aura_icons::AuraIcon;
use aura_icons_lucide::IconName;
use aura_theme::{AuraTheme, ButtonSize, ButtonVariant, ButtonVariantColors};
use gpui::{
    App, Component, ElementId, Hsla, IntoElement, RenderOnce, Rgba, SharedString, Window,
    prelude::*, px,
};
use std::panic::Location;

fn rgba(r: u8, g: u8, b: u8, a: f32) -> Hsla {
    Rgba { r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a }.into()
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
    icon_start: Option<IconName>,
    icon_end: Option<IconName>,
    creation_site: &'static Location<'static>,
}

impl AuraButton {
    #[track_caller]
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(), variant: ButtonVariant::Default, size: ButtonSize::Default,
            disabled: false, loading: false, secondary: false, background: true, border: true,
            rounded: None, id: None, icon_start: None, icon_end: None,
            creation_site: Location::caller(),
        }
    }
    pub fn variant(mut self, v: ButtonVariant) -> Self { self.variant = v; self }
    pub fn primary(mut self) -> Self   { self.variant = ButtonVariant::Primary; self }
    pub fn tertiary(mut self) -> Self  { self.variant = ButtonVariant::Tertiary; self }
    pub fn info(mut self) -> Self      { self.variant = ButtonVariant::Info; self }
    pub fn success(mut self) -> Self   { self.variant = ButtonVariant::Success; self }
    pub fn warning(mut self) -> Self   { self.variant = ButtonVariant::Warning; self }
    pub fn danger(mut self) -> Self    { self.variant = ButtonVariant::Danger; self }
    pub fn size(mut self, s: ButtonSize) -> Self { self.size = s; self }
    pub fn small(mut self) -> Self  { self.size = ButtonSize::Small; self }
    pub fn large(mut self) -> Self  { self.size = ButtonSize::Large; self }
    pub fn disabled(mut self, d: bool) -> Self { self.disabled = d; self }
    pub fn loading(mut self, l: bool) -> Self   { self.loading = l; self }
    pub fn secondary(mut self) -> Self { self.secondary = true; self }
    pub fn background(mut self, show: bool) -> Self { self.background = show; self }
    pub fn border(mut self, show: bool) -> Self { self.border = show; self }
    pub fn rounded(mut self, r: f32) -> Self { self.rounded = Some(r); self }
    pub fn id(mut self, id: impl Into<ElementId>) -> Self { self.id = Some(id.into()); self }
    pub fn icon_start(mut self, icon: IconName) -> Self { self.icon_start = Some(icon); self }
    pub fn icon_end(mut self, icon: IconName) -> Self   { self.icon_end = Some(icon); self }

    fn colors(&self, theme: &AuraTheme) -> ButtonVariantColors {
        if self.disabled {
            ButtonVariantColors {
                bg: rgba(0,0,0,0.0), hover_bg: rgba(0,0,0,0.0), active_bg: rgba(0,0,0,0.0),
                text: theme.neutral.text_disabled, border: theme.neutral.border,
                text_hover: theme.neutral.text_disabled, border_hover: theme.neutral.border,
            }
        } else {
            theme.color_by_variant(self.variant, self.secondary, self.background, self.border)
        }
    }

    fn auto_id(&self) -> ElementId {
        SharedString::from(format!(
            "aura-button:{}:{}:{:?}:{:?}:secondary={}:background={}:border={}:rounded={:?}",
            self.creation_site, self.label, self.variant, self.size,
            self.secondary, self.background, self.border, self.rounded
        )).into()
    }

    fn icon_size(&self) -> f32 {
        match self.size {
            ButtonSize::Small => 12.0,
            ButtonSize::Default => 14.0,
            ButtonSize::Large => 16.0,
        }
    }

    fn render_with_theme(self, theme: &AuraTheme) -> impl IntoElement {
        let c = self.colors(theme);
        let h = self.size.height(); let px_h = self.size.padding_x();
        let fs = match self.size {
            ButtonSize::Small=>theme.font_size.xs, ButtonSize::Default=>theme.font_size.md,
            ButtonSize::Large=>theme.font_size.lg,
        };
        let r = self.rounded.unwrap_or(theme.radius.md);
        let id = self.id.clone().unwrap_or_else(|| self.auto_id());
        let icon_sz = self.icon_size();

        let mut div = gpui::div()
            .flex().flex_row().justify_center().items_center().gap_1()
            .h(px(h)).px(px(px_h)).rounded(px(r))
            .bg(c.bg).text_color(c.text).text_size(px(fs));

        if !self.disabled { div = div.cursor_pointer(); }
        else { div = div.cursor_not_allowed(); }
        if !c.border.is_transparent() { div = div.border_1().border_color(c.border); }
        if self.disabled {
            return div.child(self.label.clone()).into_any_element();
        }

        // Build children: [icon_start] label|loader [icon_end]
        let mut children: Vec<Box<dyn FnOnce() -> gpui::AnyElement>> = Vec::new();

        if self.loading {
            let sz = icon_sz;
            children.push(Box::new(move || AuraIcon::new(IconName::LoaderCircle).size(sz).color(c.text).into_any_element()));
            children.push(Box::new(move || gpui::div().child(self.label.clone()).into_any_element()));
        } else {
            if let Some(icon) = self.icon_start {
                let sz = icon_sz;
                children.push(Box::new(move || AuraIcon::new(icon).size(sz).color(c.text).into_any_element()));
            }
            children.push(Box::new(move || gpui::div().child(self.label.clone()).into_any_element()));
            if let Some(icon) = self.icon_end {
                let sz = icon_sz;
                children.push(Box::new(move || AuraIcon::new(icon).size(sz).color(c.text).into_any_element()));
            }
        }

        div.id(id)
            .hover(move |style| {
                let mut s = style.bg(c.hover_bg).text_color(c.text_hover);
                if !c.border_hover.is_transparent() { s = s.border_color(c.border_hover); }
                s
            })
            .active(move |style| style.bg(c.active_bg))
            .children(children.into_iter().map(|f| f()))
            .into_any_element()
    }
}

impl RenderOnce for AuraButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<AuraConfig>().theme;
        self.render_with_theme(theme)
    }
}

impl IntoElement for AuraButton {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element { Component::new(self) }
}
