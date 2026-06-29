//! Watermark module.
//!
//! This public module implements the Liora watermark overlay component for repeated background labels. It keeps the reusable
//! component logic inside `liora-components` rather than host applications so
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
//! the component, and avoid app-specific host-application resources in this SDK
//! crate.

use gpui::{
    AnyElement, App, Component, Hsla, IntoElement, Pixels, RenderOnce, SharedString, Window, div,
    prelude::*, px,
};
use liora_core::Config;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control watermark placement behavior.
pub enum WatermarkPlacement {
    #[default]
    /// Places the watermark in the cover region.
    Cover,
    /// Places the watermark in the header region.
    Header,
    /// Places the watermark in the footer region.
    Footer,
}

/// Fluent native GPUI component for rendering Liora watermark.
pub struct Watermark {
    content: AnyElement,
    text: SharedString,
    placement: WatermarkPlacement,
    opacity: f32,
    color: Option<Hsla>,
    gap_x: Pixels,
    gap_y: Pixels,
    rotate_degrees: f32,
    z_index: i32,
    rows: usize,
    columns: usize,
}

impl Watermark {
    /// Creates `Watermark` initialized from the supplied content, and text.
    pub fn new(content: impl IntoElement, text: impl Into<SharedString>) -> Self {
        Self {
            content: content.into_any_element(),
            text: text.into(),
            placement: WatermarkPlacement::Cover,
            opacity: 0.16,
            color: None,
            gap_x: px(96.0),
            gap_y: px(72.0),
            rotate_degrees: -22.0,
            z_index: 10,
            rows: 4,
            columns: 4,
        }
    }

    /// Selects the popup, label, or overlay placement.
    pub fn placement(mut self, placement: WatermarkPlacement) -> Self {
        self.placement = placement;
        self
    }
    /// Sets the header value used by the component.
    pub fn header(self) -> Self {
        self.placement(WatermarkPlacement::Header)
    }
    /// Sets the footer value used by the component.
    pub fn footer(self) -> Self {
        self.placement(WatermarkPlacement::Footer)
    }
    /// Sets the opacity value used by the component.
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }
    /// Applies an explicit color instead of the theme-derived default.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
    /// Sets the spacing between child elements.
    pub fn gap(mut self, x: impl Into<Pixels>, y: impl Into<Pixels>) -> Self {
        self.gap_x = x.into().max(px(8.0));
        self.gap_y = y.into().max(px(8.0));
        self
    }
    /// Sets the rotate value used by the component.
    pub fn rotate(mut self, degrees: f32) -> Self {
        self.rotate_degrees = degrees;
        self
    }
    /// Sets the density value used by the component.
    pub fn density(mut self, rows: usize, columns: usize) -> Self {
        self.rows = rows.max(1);
        self.columns = columns.max(1);
        self
    }
    /// Sets the z index value used by the component.
    pub fn z_index(mut self, z: i32) -> Self {
        self.z_index = z;
        self
    }
    /// Performs the tile count operation used by this component.
    pub fn tile_count(&self) -> usize {
        match self.placement {
            WatermarkPlacement::Cover => self.rows * self.columns,
            WatermarkPlacement::Header | WatermarkPlacement::Footer => self.columns,
        }
    }
}

impl RenderOnce for Watermark {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let color = self
            .color
            .unwrap_or(theme.neutral.text_3)
            .opacity(self.opacity);
        let count = self.tile_count();
        let overlay = div()
            .absolute()
            .top_0()
            .left_0()
            .right_0()
            .bottom_0()
            .overflow_hidden()
            .when(self.placement == WatermarkPlacement::Header, |s| {
                s.bottom_auto().h(px(72.0))
            })
            .when(self.placement == WatermarkPlacement::Footer, |s| {
                s.top_auto().h(px(72.0))
            })
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(self.gap_x)
                    .gap_y(self.gap_y)
                    .p_4()
                    .children((0..count).map(|_| {
                        div()
                            .flex_none()
                            .min_w(px(120.0))
                            .text_sm()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(color)
                            .child(self.text.clone())
                            .into_any_element()
                    })),
            );
        div()
            .id(liora_core::unique_id("watermark"))
            .relative()
            .overflow_hidden()
            .child(self.content)
            .child(overlay)
            .child(
                div()
                    .absolute()
                    .right_2()
                    .bottom_1()
                    .text_xs()
                    .text_color(theme.neutral.text_3.opacity(0.5))
                    .child(format!("rotate {}°", self.rotate_degrees)),
            )
    }
}

impl IntoElement for Watermark {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn watermark_tracks_density_and_opacity() {
        let watermark = Watermark::new("body", "CONFIDENTIAL")
            .density(3, 5)
            .opacity(2.0);
        assert_eq!(watermark.tile_count(), 15);
        assert_eq!(watermark.opacity, 1.0);
    }
    #[test]
    fn watermark_header_uses_columns_only() {
        let watermark = Watermark::new("body", "LIORA").header().density(8, 3);
        assert_eq!(watermark.tile_count(), 3);
    }
}
