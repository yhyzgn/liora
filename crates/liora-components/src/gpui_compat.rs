//! GPUI compatibility helpers used by Liora components.
//!
//! Liora intentionally targets the official Zed GPUI git revision pinned by the
//! workspace. This module only keeps small repo-local convenience wrappers that
//! do not emulate older crates.io GPUI releases.
//!
//! ## Usage model
//!
//! Components import these helpers only for tiny repeated GPUI call shapes, such
//! as stable element-id construction. They are not an abstraction boundary for
//! alternate GPUI backends, dependency fallback, or local patches.
//!
//! ## Design contract
//!
//! Keep this module minimal and transparent. Prefer direct official GPUI APIs
//! whenever a wrapper would hide behavior, change types, or preserve support for
//! an older crates.io GPUI release.

use gpui::{ElementId, SharedString};

/// Returns a stable GPUI element id from a string-like value.
pub fn element_id(id: impl Into<SharedString>) -> ElementId {
    ElementId::from(id.into())
}
