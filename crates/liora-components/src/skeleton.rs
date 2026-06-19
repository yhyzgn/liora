//! Skeleton module.
//!
//! This public module implements the Liora skeleton loading placeholder components. It keeps the reusable
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

use crate::motion::pulse;
use gpui::{AnyElement, App, DefiniteLength, IntoElement, RenderOnce, Window, div, prelude::*, px};
use liora_core::Config;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control skeleton variant behavior.
pub enum SkeletonVariant {
    #[default]
    /// Renders a paragraph-shaped skeleton placeholder.
    Paragraph,
    /// Uses circular geometry.
    Circle,
    /// Uses square geometry.
    Square,
    /// Reports a image failure.
    Image,
}

/// Data model used by skeleton item rendering.
pub struct SkeletonItem {
    variant: SkeletonVariant,
    width: Option<DefiniteLength>,
}

/// Fluent native GPUI component for rendering Liora skeleton.
pub struct Skeleton {
    loading: bool,
    rows: u32,
    animated: bool,
    template: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>>,
    child: Option<AnyElement>,
}

impl SkeletonItem {
    /// Creates `SkeletonItem` initialized from the supplied variant.
    pub fn new(variant: SkeletonVariant) -> Self {
        Self {
            variant,
            width: None,
        }
    }

    /// Sets the component width token used during GPUI layout.
    pub fn width(mut self, width: impl Into<DefiniteLength>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Applies the predefined width 2 5 sizing preset.
    pub fn width_2_5(self) -> Self {
        self.width(gpui::relative(0.4))
    }
}

impl Skeleton {
    /// Creates `Skeleton` with default theme-driven styling and no optional callbacks attached.
    pub fn new() -> Self {
        Self {
            loading: true,
            rows: 3,
            animated: true,
            template: None,
            child: None,
        }
    }

    /// Toggles the loading state and associated spinner treatment.
    pub fn loading(mut self, l: bool) -> Self {
        self.loading = l;
        self
    }

    /// Sets the visible row count for editor-like controls.
    pub fn rows(mut self, r: u32) -> Self {
        self.rows = r;
        self
    }

    /// Enables animation for progress or motion visuals.
    pub fn animated(mut self, a: bool) -> Self {
        self.animated = a;
        self
    }

    /// Sets the template value used by the component.
    pub fn template<F>(mut self, f: F) -> Self
    where
        F: Fn(&mut Window, &mut App) -> AnyElement + 'static,
    {
        self.template = Some(Box::new(f));
        self
    }

    /// Adds a child element to the component body.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }
}

impl RenderOnce for SkeletonItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let skeleton_bg = theme.neutral.hover;

        let item = match self.variant {
            SkeletonVariant::Circle => div().w(px(40.0)).h(px(40.0)).bg(skeleton_bg).rounded_full(),
            SkeletonVariant::Square => div()
                .w_full()
                .h(px(40.0))
                .bg(skeleton_bg)
                .rounded(px(theme.radius.sm)),
            SkeletonVariant::Paragraph => {
                div().w_full().h(px(16.0)).bg(skeleton_bg).rounded(px(4.0))
            }
            SkeletonVariant::Image => div()
                .w(px(200.0))
                .h(px(150.0))
                .bg(skeleton_bg)
                .rounded(px(theme.radius.sm)),
        };

        item.when_some(self.width, |s, width| s.w(width))
    }
}

impl IntoElement for SkeletonItem {
    type Element = gpui::Component<Self>;
    fn into_element(self) -> Self::Element {
        gpui::Component::new(self)
    }
}

impl RenderOnce for Skeleton {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        if !self.loading {
            return div()
                .child(self.child.unwrap_or_else(|| div().into_any_element()))
                .into_any_element();
        }

        if let Some(template) = self.template {
            return (template)(window, cx).into_any_element();
        }

        // Default: multiple rows of paragraph
        let animated = self.animated;

        div()
            .flex()
            .flex_col()
            .gap_3()
            .w_full()
            .children((0..self.rows).map(|i| {
                let width = if i == self.rows - 1 && self.rows > 1 {
                    gpui::relative(0.6)
                } else {
                    gpui::relative(1.0)
                };
                let row = div()
                    .w(width)
                    .child(SkeletonItem::new(SkeletonVariant::Paragraph));

                if animated {
                    pulse(("liora-skeleton-row-motion", i as usize), row).into_any_element()
                } else {
                    row.into_any_element()
                }
            }))
            .into_any_element()
    }
}

impl IntoElement for Skeleton {
    type Element = gpui::Component<Self>;
    fn into_element(self) -> Self::Element {
        gpui::Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skeleton_item_width_2_5_sets_fraction_width() {
        assert_eq!(
            SkeletonItem::new(SkeletonVariant::Paragraph)
                .width_2_5()
                .width,
            Some(gpui::relative(0.4))
        );
    }

    #[test]
    fn skeleton_default_rows_use_pulse_motion_when_animated() {
        let source = include_str!("skeleton.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains("pulse("));
        assert!(source.contains("liora-skeleton-row-motion"));
        assert!(source.contains("if animated"));
    }
}
