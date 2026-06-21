//! Space module.
//!
//! This public module implements the Liora spacing layout helper for horizontal/vertical stacks. It keeps the reusable
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
    AnyElement, App, Component, DefiniteLength, IntoElement, RenderOnce, Window, prelude::*, px,
};

/// Fluent native GPUI component for rendering Liora space.
pub struct Space {
    children: Vec<AnyElement>,
    vertical: bool,
    wrap: bool,
    gap: Option<DefiniteLength>,
    align: Option<SpaceAlign>,
    grow: bool,
    shrink: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Options that control space align behavior.
pub enum SpaceAlign {
    /// Aligns content using the start position.
    Start,
    /// Aligns content using the center position.
    Center,
    /// Aligns content using the end position.
    End,
}

impl Space {
    /// Creates `Space` with default theme-driven styling and no optional callbacks attached.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            vertical: false,
            wrap: false,
            gap: None,
            align: None,
            grow: false,
            shrink: false,
        }
    }

    /// Uses vertical orientation or gradient direction.
    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }

    /// Sets the spacing between child elements.
    pub fn gap(mut self, gap: impl Into<DefiniteLength>) -> Self {
        self.gap = Some(gap.into());
        self
    }

    /// Applies the predefined `gap_xs` sizing preset.
    pub fn gap_xs(self) -> Self {
        self.gap(px(4.0))
    }

    /// Applies the predefined gap sm sizing preset.
    pub fn gap_sm(self) -> Self {
        self.gap(px(8.0))
    }

    /// Applies the predefined gap md sizing preset.
    pub fn gap_md(self) -> Self {
        self.gap(px(12.0))
    }

    /// Applies the predefined gap lg sizing preset.
    pub fn gap_lg(self) -> Self {
        self.gap(px(16.0))
    }

    /// Applies the predefined gap xl sizing preset.
    pub fn gap_xl(self) -> Self {
        self.gap(px(24.0))
    }

    /// Allows child content to wrap onto additional lines.
    pub fn wrap(mut self) -> Self {
        self.wrap = true;
        self
    }

    /// Sets cross-axis alignment for child content.
    pub fn align(mut self, align: SpaceAlign) -> Self {
        self.align = Some(align);
        self
    }

    /// Sets the align start value used by the component.
    pub fn align_start(self) -> Self {
        self.align(SpaceAlign::Start)
    }

    /// Sets the align center value used by the component.
    pub fn align_center(self) -> Self {
        self.align(SpaceAlign::Center)
    }

    /// Sets the align end value used by the component.
    pub fn align_end(self) -> Self {
        self.align(SpaceAlign::End)
    }

    /// Sets the grow value used by the component.
    pub fn grow(mut self) -> Self {
        self.grow = true;
        self
    }

    /// Allows the space container to shrink within constrained parents instead of forcing overflow.
    pub fn shrink(mut self) -> Self {
        self.shrink = true;
        self
    }

    /// Adds a child element to the component body.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    /// Replaces or appends child elements rendered by the component.
    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(|c| c.into_any_element()));
        self
    }
}

impl RenderOnce for Space {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut div = gpui::div().flex();
        if self.grow {
            div = div.flex_1();
        }
        if self.shrink {
            div = div.min_w(px(0.0)).flex_shrink();
        }

        if self.vertical {
            div = div.flex_col();
        } else {
            div = div.flex_row().items_center();
        }

        if self.wrap {
            div = div.flex_wrap();
        }

        div = match self.align {
            Some(SpaceAlign::Start) => div.items_start(),
            Some(SpaceAlign::Center) => div.items_center(),
            Some(SpaceAlign::End) => div.items_end(),
            None => div,
        };

        if let Some(gap) = self.gap {
            div = div.gap(gap);
        } else {
            div = div.gap_2(); // Default gap
        }

        div.children(self.children)
    }
}

impl IntoElement for Space {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn space_wrap_builder_tracks_state() {
        let space = Space::new().wrap();

        assert!(space.wrap);
    }

    #[test]
    fn space_align_center_tracks_cross_axis_alignment() {
        let space = Space::new().vertical().align_center();

        assert_eq!(space.align, Some(SpaceAlign::Center));
    }

    #[test]
    fn space_grow_tracks_flex_growth() {
        assert!(Space::new().grow().grow);
    }

    #[test]
    fn space_shrink_tracks_flex_shrink() {
        let source = include_str!("space.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(Space::new().shrink().shrink);
        assert!(source.contains("div.min_w(px(0.0)).flex_shrink()"));
    }
}
