//! Kbd module.
//!
//! This public module implements the Liora keyboard shortcut keycap component. It keeps the reusable
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
    App, Component, Hsla, IntoElement, Pixels, RenderOnce, SharedString, Window, div, prelude::*,
    px,
};
use liora_core::Config;

/// Fluent native GPUI component for rendering keyboard shortcut keycaps.
pub struct Kbd {
    label: SharedString,
    size: KbdSize,
    color: Option<Hsla>,
    bg: Option<Hsla>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Size presets for [`Kbd`].
pub enum KbdSize {
    /// Compact keycap for dense toolbars.
    Small,
    /// Default keycap size.
    Medium,
    /// Larger keycap for docs and empty states.
    Large,
}

impl Kbd {
    /// Creates a keycap from a display label such as `⌘K`, `Ctrl`, or `Enter`.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            size: KbdSize::Medium,
            color: None,
            bg: None,
        }
    }

    /// Applies the small sizing preset.
    pub fn small(mut self) -> Self {
        self.size = KbdSize::Small;
        self
    }

    /// Applies the large sizing preset.
    pub fn large(mut self) -> Self {
        self.size = KbdSize::Large;
        self
    }

    /// Applies an explicit text color.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    /// Applies an explicit keycap background color.
    pub fn bg(mut self, bg: Hsla) -> Self {
        self.bg = Some(bg);
        self
    }

    fn metrics(&self) -> (Pixels, Pixels, Pixels) {
        match self.size {
            KbdSize::Small => (px(11.0), px(3.0), px(6.0)),
            KbdSize::Medium => (px(12.0), px(4.0), px(8.0)),
            KbdSize::Large => (px(14.0), px(5.0), px(10.0)),
        }
    }
}

impl RenderOnce for Kbd {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let (font_size, py, pxv) = self.metrics();
        div()
            .flex()
            .items_center()
            .justify_center()
            .rounded_md()
            .border_1()
            .border_color(theme.neutral.border)
            .bg(self.bg.unwrap_or(theme.neutral.hover))
            .px(pxv)
            .py(py)
            .text_size(font_size)
            .text_color(self.color.unwrap_or(theme.neutral.text_2))
            .font_family("monospace")
            .child(self.label)
    }
}

impl IntoElement for Kbd {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kbd_size_presets_track_state() {
        assert_eq!(Kbd::new("Esc").small().size, KbdSize::Small);
        assert_eq!(Kbd::new("Enter").large().size, KbdSize::Large);
    }
}
