//! Row module.
//!
//! This public module implements the Liora grid row helper for row/column layouts. It keeps the reusable
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

use crate::Col;
use gpui::{App, Component, IntoElement, RenderOnce, Window, prelude::*};

/// Data model used by row rendering.
pub struct Row {
    justify: Option<RowJustify>,
    align: Option<RowAlign>,
    children: Vec<gpui::AnyElement>,
}

#[derive(Clone, Copy)]
/// Options that control row justify behavior.
pub enum RowJustify {
    /// Packs row children at the start edge.
    Start,
    /// Centers row children on the main axis.
    Center,
    /// Packs row children at the end edge.
    End,
    /// Distributes row children with space between items.
    SpaceBetween,
    /// Distributes row children with space around items.
    SpaceAround,
}
#[derive(Clone, Copy)]
/// Options that control row align behavior.
pub enum RowAlign {
    /// Places the overlay above the anchor.
    Top,
    /// Aligns row children to the vertical center.
    Middle,
    /// Places the overlay below the anchor.
    Bottom,
}

impl Row {
    /// Creates `Row` with default theme-driven styling and no optional callbacks attached.
    pub fn new() -> Self {
        Self {
            justify: None,
            align: None,
            children: vec![],
        }
    }
    /// Sets main-axis distribution for child content.
    pub fn justify(mut self, j: RowJustify) -> Self {
        self.justify = Some(j);
        self
    }
    /// Sets cross-axis alignment for child content.
    pub fn align(mut self, a: RowAlign) -> Self {
        self.align = Some(a);
        self
    }
    /// Adds a child element to the component body.
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
