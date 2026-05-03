use aura_core::AuraConfig;
use aura_icons::AuraIcon;
use aura_icons_lucide::IconName;
use aura_theme::{AuraTheme, ButtonVariant};
use gpui::{
    App, Component, Hsla, IntoElement, MouseButton, RenderOnce, SharedString, Window,
    prelude::*, px,
};
use std::panic::Location;

pub struct AuraLink {
    label: SharedString,
    href: Option<SharedString>,
    variant: ButtonVariant,
    disabled: bool,
    underline: bool,
    icon_start: Option<IconName>,
    icon_end: Option<IconName>,
    creation_site: &'static Location<'static>,
}

impl AuraLink {
    #[track_caller]
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(), href: None, variant: ButtonVariant::Default,
            disabled: false, underline: true, icon_start: None, icon_end: None,
            creation_site: Location::caller(),
        }
    }
    pub fn href(mut self, url: impl Into<SharedString>) -> Self { self.href = Some(url.into()); self }
    pub fn variant(mut self, v: ButtonVariant) -> Self { self.variant = v; self }
    pub fn primary(mut self) -> Self  { self.variant = ButtonVariant::Primary; self }
    pub fn success(mut self) -> Self  { self.variant = ButtonVariant::Success; self }
    pub fn warning(mut self) -> Self  { self.variant = ButtonVariant::Warning; self }
    pub fn danger(mut self) -> Self   { self.variant = ButtonVariant::Danger; self }
    pub fn info(mut self) -> Self     { self.variant = ButtonVariant::Info; self }
    pub fn disabled(mut self, d: bool) -> Self { self.disabled = d; self }
    pub fn underline(mut self, u: bool) -> Self { self.underline = u; self }
    pub fn icon_start(mut self, icon: IconName) -> Self { self.icon_start = Some(icon); self }
    pub fn icon_end(mut self, icon: IconName) -> Self   { self.icon_end = Some(icon); self }

    fn color_for(&self, theme: &AuraTheme) -> (Hsla, Hsla) {
        if self.disabled { return (theme.neutral.text_disabled, theme.neutral.text_disabled); }
        let family = match self.variant {
            ButtonVariant::Default | ButtonVariant::Tertiary => &theme.primary,
            ButtonVariant::Primary => &theme.primary,
            ButtonVariant::Success => &theme.success,
            ButtonVariant::Warning => &theme.warning,
            ButtonVariant::Danger => &theme.danger,
            ButtonVariant::Info => &theme.info,
        };
        (family.base, family.hover)
    }

    fn render_with_theme(self, theme: &AuraTheme) -> impl IntoElement {
        let (color, hover_color) = self.color_for(theme);
        let fs = theme.font_size.md;
        let icon_sz = 14.0;

        let id = SharedString::from(format!(
            "aura-link:{}:{}", self.creation_site, self.label
        ));

        let mut div = gpui::div()
            .flex().flex_row().items_center().gap_1()
            .text_size(px(fs)).text_color(color)
            .id(id);

        if !self.disabled { div = div.cursor_pointer(); }
        else { div = div.cursor_not_allowed(); }
        if self.underline { div = div.underline(); }

        let mut children: Vec<Box<dyn FnOnce() -> gpui::AnyElement>> = Vec::new();
        if let Some(icon) = self.icon_start {
            children.push(Box::new(move || AuraIcon::new(icon).size(icon_sz).color(color).into_any_element()));
        }
        let label = self.label.clone();
        children.push(Box::new(move || gpui::div().child(label).into_any_element()));
        if let Some(icon) = self.icon_end {
            children.push(Box::new(move || AuraIcon::new(icon).size(icon_sz).color(color).into_any_element()));
        }

        if !self.disabled {
            if let Some(ref href) = self.href {
                let url = href.clone();
                div = div.on_click(move |_, _, _| { open_url(&url); });
            }
            div = div.hover(move |style| style.text_color(hover_color));
        }

        div.children(children.into_iter().map(|f| f()))
    }
}

fn open_url(url: &str) {
    #[cfg(target_os = "linux")]
    { let _ = std::process::Command::new("xdg-open").arg(url).spawn(); }
    #[cfg(target_os = "macos")]
    { let _ = std::process::Command::new("open").arg(url).spawn(); }
    #[cfg(target_os = "windows")]
    { let _ = std::process::Command::new("cmd").args(["/c", "start", "", url]).spawn(); }
}

impl RenderOnce for AuraLink {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<AuraConfig>().theme;
        self.render_with_theme(theme)
    }
}

impl IntoElement for AuraLink {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element { Component::new(self) }
}
