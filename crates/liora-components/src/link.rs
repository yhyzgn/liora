//! Link module.
//!
//! This public module implements the Liora interactive text link component with optional icon placement. It keeps the reusable
//! component logic inside `liora-components` rather than Gallery or Docs so
//! downstream GPUI applications can compose the same behavior with their own
//! app state, assets, and release policy.
//!
//! ## Usage model
//!
//! Components in this module render native GPUI element trees. Stateless builder
//! values can be constructed inline, while controls with focus, selection,
//! popup, drag, or editing state should be stored as `gpui::Entity<T>` fields in
//! the parent view so state survives GPUI render passes.
//!
//! ## Design contract
//!
//! The implementation should use Liora theme tokens from `liora-core` and
//! `liora-theme`, keep accessibility-oriented keyboard/pointer behavior close to
//! the component, and avoid app-specific Gallery/Docs resources in this SDK
//! crate.

use gpui::{
    App, Component, Hsla, IntoElement, MouseButton, RenderOnce, SharedString, Window, prelude::*,
    px,
};
use liora_core::{Config, stable_unique_id};
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use liora_theme::{ButtonVariant, Theme};

/// Public builder and render state for the Liora link component.
pub struct Link {
    label: SharedString,
    href: Option<SharedString>,
    variant: ButtonVariant,
    disabled: bool,
    underline: bool,
    icon_start: Option<IconName>,
    icon_end: Option<IconName>,
    id: Option<SharedString>,
}

impl Link {
    /// Creates a new value with the required baseline configuration.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            href: None,
            variant: ButtonVariant::Default,
            disabled: false,
            underline: true,
            icon_start: None,
            icon_end: None,
            id: None,
        }
    }
    /// Configures the href option.
    pub fn href(mut self, url: impl Into<SharedString>) -> Self {
        self.href = Some(url.into());
        self
    }
    /// Configures the variant option.
    pub fn variant(mut self, v: ButtonVariant) -> Self {
        self.variant = v;
        self
    }
    /// Configures the primary option.
    pub fn primary(mut self) -> Self {
        self.variant = ButtonVariant::Primary;
        self
    }
    /// Configures the success option.
    pub fn success(mut self) -> Self {
        self.variant = ButtonVariant::Success;
        self
    }
    /// Configures the warning option.
    pub fn warning(mut self) -> Self {
        self.variant = ButtonVariant::Warning;
        self
    }
    /// Configures the danger option.
    pub fn danger(mut self) -> Self {
        self.variant = ButtonVariant::Danger;
        self
    }
    /// Configures the info option.
    pub fn info(mut self) -> Self {
        self.variant = ButtonVariant::Info;
        self
    }
    /// Configures the disabled option.
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }
    /// Configures the underline option.
    pub fn underline(mut self, u: bool) -> Self {
        self.underline = u;
        self
    }
    /// Configures the icon start option.
    pub fn icon_start(mut self, icon: IconName) -> Self {
        self.icon_start = Some(icon);
        self
    }
    /// Configures the icon end option.
    pub fn icon_end(mut self, icon: IconName) -> Self {
        self.icon_end = Some(icon);
        self
    }

    /// Returns the stable tray command identifier used for menu event routing.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = Some(id.into());
        self
    }

    fn color_for(&self, theme: &Theme) -> (Hsla, Hsla) {
        if self.disabled {
            return (theme.neutral.text_disabled, theme.neutral.text_disabled);
        }
        let family = match self.variant {
            ButtonVariant::Default | ButtonVariant::Tertiary | ButtonVariant::Text => {
                &theme.primary
            }
            ButtonVariant::Primary => &theme.primary,
            ButtonVariant::Success => &theme.success,
            ButtonVariant::Warning => &theme.warning,
            ButtonVariant::Danger => &theme.danger,
            ButtonVariant::Info => &theme.info,
        };
        (family.base, family.hover)
    }

    fn render_with_theme(
        self,
        theme: Theme,
        window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        let (color, hover_color) = self.color_for(&theme);
        let fs = theme.font_size.md;
        let icon_sz = 14.0;
        let id = self.id.clone().unwrap_or_else(|| {
            stable_unique_id(
                format!(
                    "link:{}:{:?}:disabled={}:underline={}",
                    self.label, self.variant, self.disabled, self.underline
                ),
                "link",
                window,
                cx,
            )
        });

        let mut div = gpui::div()
            .flex()
            .flex_row()
            .items_center()
            .gap_1()
            .text_size(px(fs))
            .text_color(color)
            .id(id);

        if !self.disabled {
            div = div.cursor_pointer();
        } else {
            div = div.cursor_not_allowed();
        }
        if self.underline {
            div = div.underline();
        }

        let mut children: Vec<Box<dyn FnOnce() -> gpui::AnyElement>> = Vec::new();
        if let Some(icon) = self.icon_start {
            children.push(Box::new(move || {
                Icon::new(icon)
                    .size(px(icon_sz))
                    .color(color)
                    .into_any_element()
            }));
        }
        let label = self.label.clone();
        children.push(Box::new(move || {
            gpui::div().child(label).into_any_element()
        }));
        if let Some(icon) = self.icon_end {
            children.push(Box::new(move || {
                Icon::new(icon)
                    .size(px(icon_sz))
                    .color(color)
                    .into_any_element()
            }));
        }

        if !self.disabled {
            if let Some(ref href) = self.href {
                let url = href.clone();
                div = div.on_mouse_up(MouseButton::Left, move |_, _, _| {
                    open_url(&url);
                });
            }
            div = div.hover(move |style| style.text_color(hover_color));
        }

        div.children(children.into_iter().map(|f| f()))
    }
}

fn open_url(url: &str) {
    #[cfg(target_os = "linux")]
    {
        if let Err(e) = std::process::Command::new("xdg-open").arg(url).spawn() {
            eprintln!("Link: failed to open URL: {}", e);
        }
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(url).spawn();
    }
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .args(["/c", "start", "", url])
            .spawn();
    }
}

impl RenderOnce for Link {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        self.render_with_theme(theme, _window, cx)
    }
}

impl IntoElement for Link {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
