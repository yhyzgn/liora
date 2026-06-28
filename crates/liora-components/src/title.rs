//! Title module.
//!
//! This public module implements the Liora selectable heading/title component. It keeps the reusable
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

use crate::{SelectableText, SelectableTextOptions, SelectableTextWrap};
use gpui::{
    App, Component, ElementId, IntoElement, RenderOnce, SharedString, TextStyle, Window,
    prelude::*, px,
};
use liora_core::{Config, ui_font_family};

/// Fluent native GPUI component for rendering Liora title.
pub struct Title {
    content: SharedString,
    level: u8,
    selectable: bool,
    id: SharedString,
}

impl Title {
    /// Creates `Title` initialized from the supplied content.
    pub fn new(content: impl Into<SharedString>) -> Self {
        Self {
            content: content.into(),
            level: 1,
            selectable: true,
            id: liora_core::unique_id("title"),
        }
    }

    /// Sets the h1 value used by the component.
    pub fn h1(mut self) -> Self {
        self.level = 1;
        self
    }
    /// Sets the h2 value used by the component.
    pub fn h2(mut self) -> Self {
        self.level = 2;
        self
    }
    /// Sets the h3 value used by the component.
    pub fn h3(mut self) -> Self {
        self.level = 3;
        self
    }
    /// Sets the h4 value used by the component.
    pub fn h4(mut self) -> Self {
        self.level = 4;
        self
    }
    /// Sets the h5 value used by the component.
    pub fn h5(mut self) -> Self {
        self.level = 5;
        self
    }
    /// Sets the h6 value used by the component.
    pub fn h6(mut self) -> Self {
        self.level = 6;
        self
    }

    /// Toggles whether the rendered text can be selected.
    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }

    /// Assigns a stable element id used by GPUI state, hit testing, and automated interaction tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Registers GPUI key bindings required for keyboard interaction.
    pub fn register_key_bindings(cx: &mut App) {
        SelectableText::register_key_bindings(cx);
    }

    fn render_with_theme(
        self,
        theme: &liora_theme::Theme,
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

        let ui_family = ui_font_family(cx);

        if self.selectable {
            let mut style = TextStyle::default();
            style.color = text_color;
            style.font_size = font_size.into();
            style.line_height = line_height.into();
            style.font_weight = weight;
            style.white_space = gpui::WhiteSpace::Normal;
            if let Some(family) = ui_family.clone() {
                style.font_family = family;
            }
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
                    font_family: ui_family.clone(),
                },
                window,
                cx,
            );
        }

        let mut title = gpui::div()
            .text_size(font_size)
            .line_height(line_height)
            .font_weight(weight)
            .text_color(text_color)
            .child(self.content.clone());

        if let Some(family) = ui_family {
            title = title.font_family(family);
        }

        title.into_any_element()
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
    use super::*;

    #[test]
    fn title_defaults_to_mouse_selectable() {
        assert!(Title::new("Selectable title").selectable);
    }

    #[test]
    fn title_uses_selectable_text_for_native_selection() {
        let source = include_str!("title.rs");
        assert!(source.contains("SelectableText::view"));
        assert!(source.contains("pub fn selectable"));
        assert!(source.contains("pub fn register_key_bindings"));
    }
}
