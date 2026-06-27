//! Card module.
//!
//! This public module implements the Liora card container for grouped content with optional header and footer regions. It keeps the reusable
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
    AnyElement, App, Component, IntoElement, Pixels, RenderOnce, SharedString, Window, div,
    prelude::*, px,
};
use liora_core::{Config, stable_unique_id};

/// Fluent native GPUI component for rendering Liora card.
pub struct Card {
    title: Option<SharedString>,
    header: Option<AnyElement>,
    footer: Option<AnyElement>,
    body: AnyElement,
    hoverable: bool,
    shadow: bool,
    width: Option<Pixels>,
    shrink: bool,
}

impl Card {
    /// Creates `Card` initialized from the supplied body.
    pub fn new(body: impl IntoElement) -> Self {
        Self {
            title: None,
            header: None,
            footer: None,
            body: body.into_any_element(),
            hoverable: false,
            shadow: true,
            width: None,
            shrink: true,
        }
    }

    /// Sets the primary title text displayed by the component.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the header value used by the component.
    pub fn header(mut self, header: impl IntoElement) -> Self {
        self.header = Some(header.into_any_element());
        self
    }

    /// Sets the footer value used by the component.
    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }

    /// Enables hover styling for the component frame.
    pub fn hoverable(mut self) -> Self {
        self.hoverable = true;
        self
    }

    /// Disables the default card shadow.
    pub fn no_shadow(mut self) -> Self {
        self.shadow = false;
        self
    }

    /// Sets the component width token used during GPUI layout.
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Applies the predefined width md sizing preset.
    pub fn width_md(self) -> Self {
        self.width(px(300.0))
    }

    /// Applies the predefined width lg sizing preset.
    pub fn width_lg(self) -> Self {
        self.width(px(400.0))
    }

    /// Prevents the component from shrinking in flex layouts.
    pub fn no_shrink(mut self) -> Self {
        self.shrink = false;
        self
    }
}

impl RenderOnce for Card {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let id = stable_unique_id("card", "card", _window, cx);

        let mut el = div()
            .id(id)
            .bg(theme.neutral.card)
            .text_color(theme.neutral.text_2)
            .border_1()
            .border_color(theme.neutral.border)
            .rounded(px(theme.radius.md))
            .overflow_hidden()
            .when(!self.shrink, |s| s.flex_none())
            .when_some(self.width, |s, width| s.w(width));

        if self.shadow {
            el = el.shadow_md();
        }

        if self.hoverable {
            el = el.hover(|s| s.shadow_xl().border_color(theme.primary.base));
        }

        // We use on_click to ensure the ID-based hover and other interactions are registered correctly
        el = el.on_click(|_, _, _| {});

        // Header
        if let Some(title) = self.title {
            el = el.child(
                div()
                    .p_4()
                    .border_b_1()
                    .border_color(theme.neutral.border)
                    .child(
                        div()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(theme.neutral.text_1)
                            .child(title),
                    ),
            );
        } else if let Some(header) = self.header {
            el = el.child(
                div()
                    .p_4()
                    .border_b_1()
                    .border_color(theme.neutral.border)
                    .child(header),
            );
        }

        // Body
        el = el.child(
            div()
                .p_4()
                .text_color(theme.neutral.text_2)
                .child(self.body),
        );

        // Footer
        if let Some(footer) = self.footer {
            el = el.child(
                div()
                    .p_4()
                    .border_t_1()
                    .border_color(theme.neutral.border)
                    .bg(theme.neutral.hover)
                    .text_color(theme.neutral.text_2)
                    .child(footer),
            );
        }

        el
    }
}

impl IntoElement for Card {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_width_helpers_set_demo_widths() {
        assert_eq!(Card::new("body").width_md().width, Some(px(300.0)));
        assert_eq!(Card::new("body").width_lg().width, Some(px(400.0)));
    }

    #[test]
    fn card_no_shrink_tracks_scroll_container_usage() {
        assert!(!Card::new("body").no_shrink().shrink);
    }

    #[test]
    fn card_surfaces_and_text_use_theme_tokens() {
        let production = include_str!("card.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap_or_default();

        assert!(production.contains(".bg(theme.neutral.card)"));
        assert!(production.contains(".border_color(theme.neutral.border)"));
        assert!(
            production.contains(".text_color(theme.neutral.text_2)"),
            "card root/body/footer text should inherit a theme-aware body color"
        );
        assert!(
            production.contains(".text_color(theme.neutral.text_1)"),
            "card title text should use the primary theme-aware text color"
        );
    }
}
