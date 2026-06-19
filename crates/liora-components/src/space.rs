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

/// Public builder and render state for the Liora space component.
pub struct Space {
    children: Vec<AnyElement>,
    vertical: bool,
    wrap: bool,
    gap: Option<DefiniteLength>,
    align: Option<SpaceAlign>,
    grow: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Enumerates the supported space align modes and options.
pub enum SpaceAlign {
    /// Uses the start variant.
    Start,
    /// Uses the center variant.
    Center,
    /// Uses the end variant.
    End,
}

impl Space {
    /// Creates a new value with the required baseline configuration.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            vertical: false,
            wrap: false,
            gap: None,
            align: None,
            grow: false,
        }
    }

    /// Configures the vertical option.
    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }

    /// Configures the gap option.
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

    /// Configures the wrap option.
    pub fn wrap(mut self) -> Self {
        self.wrap = true;
        self
    }

    /// Configures the align option.
    pub fn align(mut self, align: SpaceAlign) -> Self {
        self.align = Some(align);
        self
    }

    /// Configures the align start option.
    pub fn align_start(self) -> Self {
        self.align(SpaceAlign::Start)
    }

    /// Configures the align center option.
    pub fn align_center(self) -> Self {
        self.align(SpaceAlign::Center)
    }

    /// Configures the align end option.
    pub fn align_end(self) -> Self {
        self.align(SpaceAlign::End)
    }

    /// Configures the grow option.
    pub fn grow(mut self) -> Self {
        self.grow = true;
        self
    }

    /// Configures the child option.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    /// Configures the children option.
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
}
