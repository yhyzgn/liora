//! Gpui Compat module.
//!
//! This crate-internal module implements the Liora small compatibility helpers that isolate GPUI API differences. It keeps the reusable
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

use gpui::{ElementId, Pixels, SharedString};

pub trait PixelsExt {
    fn as_f32(self) -> f32;
}

impl PixelsExt for Pixels {
    fn as_f32(self) -> f32 {
        f32::from(self)
    }
}

pub fn element_id(id: impl Into<SharedString>) -> ElementId {
    ElementId::from(id.into())
}
