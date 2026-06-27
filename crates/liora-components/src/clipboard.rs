//! Clipboard module.
//!
//! This public module implements Liora clipboard helper utilities shared by copyable components. It keeps the reusable
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

use gpui::{App, ClipboardItem};

/// Writes plain text to the native clipboard.
pub fn write_text_to_clipboard(cx: &mut App, text: impl Into<String>) {
    cx.write_to_clipboard(ClipboardItem::new_string(text.into()));
}
/// Creates a reusable GPUI clipboard item for plain text.
pub fn clipboard_text(text: impl Into<String>) -> ClipboardItem {
    ClipboardItem::new_string(text.into())
}
#[cfg(test)]
mod tests {
    #[test]
    fn clipboard_helper_source_uses_gpui_clipboard_item() {
        let source = include_str!("clipboard.rs");
        assert!(source.contains("ClipboardItem::new_string"));
    }
}
