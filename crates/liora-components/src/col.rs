//! Col module.
//!
//! This public module implements the Liora grid column helper for row/column layouts. It keeps the reusable
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

use crate::Row;
use gpui::{App, Component, IntoElement, RenderOnce, Window, prelude::*};

/// Fluent native GPUI component for rendering Liora col.
pub struct Col {
    span: u8,
    children: Vec<gpui::AnyElement>,
}

impl Col {
    /// Creates `Col` initialized from the supplied span.
    pub fn new(span: u8) -> Self {
        Self {
            span: span.min(24),
            children: vec![],
        }
    }
    /// Adds a child element to the component body.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
    /// Add a nested row.
    pub fn row(mut self, row: Row) -> Self {
        self.children.push(row.into_any_element());
        self
    }
    /// Add multiple nested rows.
    pub fn rows(mut self, rows: Vec<Row>) -> Self {
        self.children
            .extend(rows.into_iter().map(|r| r.into_any_element()));
        self
    }
}

impl RenderOnce for Col {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let span = self.span as f32 / 24.0;
        gpui::div()
            .flex_none()
            .w(gpui::relative(span))
            .children(self.children)
    }
}

impl IntoElement for Col {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
