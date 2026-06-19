//! Layout Helpers module.
//!
//! This public module implements the Liora shared demo/documentation layout helpers built from public components. It keeps the reusable
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

use crate::{Paragraph, Space, Title};
use gpui::IntoElement;

/// Performs the page operation used by this component.
pub fn page(
    title: &'static str,
    description: &'static str,
    body: impl IntoElement,
) -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_xl()
        .child(header(title, description))
        .child(body)
}

/// Performs the section operation used by this component.
pub fn section(
    title: &'static str,
    description: &'static str,
    body: impl IntoElement,
) -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_md()
        .child(header(title, description))
        .child(body)
}

/// Performs the header operation used by this component.
pub fn header(title: &'static str, description: &'static str) -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_xs()
        .child(Title::new(title).h3())
        .child(Paragraph::with_text(description))
}

/// Performs the row operation used by this component.
pub fn row(children: Vec<impl IntoElement>) -> impl IntoElement {
    Space::new().wrap().gap_lg().children(children)
}

/// Applies the predefined row md sizing preset.
pub fn row_md(children: Vec<impl IntoElement>) -> impl IntoElement {
    Space::new().wrap().gap_md().children(children)
}
