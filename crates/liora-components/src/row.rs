//! Row module.
//!
//! This public module implements the Liora grid row helper for row/column layouts. It keeps the reusable
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

use crate::Col;
use gpui::{App, Component, IntoElement, RenderOnce, Window, prelude::*};

/// Public builder and render state for the Liora row component.
pub struct Row {
    justify: Option<RowJustify>,
    align: Option<RowAlign>,
    children: Vec<gpui::AnyElement>,
}

#[derive(Clone, Copy)]
/// Enumerates the supported row justify modes and options.
pub enum RowJustify {
    /// Uses the start variant.
    Start,
    /// Uses the center variant.
    Center,
    /// Uses the end variant.
    End,
    /// Uses the space between variant.
    SpaceBetween,
    /// Uses the space around variant.
    SpaceAround,
}
#[derive(Clone, Copy)]
/// Enumerates the supported row align modes and options.
pub enum RowAlign {
    /// Uses the top variant.
    Top,
    /// Uses the middle variant.
    Middle,
    /// Uses the bottom variant.
    Bottom,
}

impl Row {
    /// Creates a new value with the required baseline configuration.
    pub fn new() -> Self {
        Self {
            justify: None,
            align: None,
            children: vec![],
        }
    }
    /// Configures the justify option.
    pub fn justify(mut self, j: RowJustify) -> Self {
        self.justify = Some(j);
        self
    }
    /// Configures the align option.
    pub fn align(mut self, a: RowAlign) -> Self {
        self.align = Some(a);
        self
    }
    /// Configures the child option.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
    /// Shorthand for adding a column.
    pub fn column(mut self, col: Col) -> Self {
        self.children.push(col.into_any_element());
        self
    }
}

impl RenderOnce for Row {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut row = gpui::div().flex().flex_row().flex_wrap().gap_2();
        match self.justify {
            Some(RowJustify::Start) => {
                row = row.justify_start();
            }
            Some(RowJustify::Center) => {
                row = row.justify_center();
            }
            Some(RowJustify::End) => {
                row = row.justify_end();
            }
            Some(RowJustify::SpaceBetween) => {
                row = row.justify_between();
            }
            Some(RowJustify::SpaceAround) => {
                row = row.justify_around();
            }
            None => {}
        }
        match self.align {
            Some(RowAlign::Top) => {
                row = row.items_start();
            }
            Some(RowAlign::Middle) => {
                row = row.items_center();
            }
            Some(RowAlign::Bottom) => {
                row = row.items_end();
            }
            None => {}
        }
        row.children(self.children)
    }
}

impl IntoElement for Row {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
