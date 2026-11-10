//! Native plain-text clipboard helpers shared by copyable components.
//!
//! Call [`write_text_to_clipboard`] from an event handler to replace the system
//! clipboard, or use [`clipboard_text`] when composing a GPUI clipboard item.
//! Read back text with `cx.read_from_clipboard().and_then(|item| item.text())`.

use gpui::{App, ClipboardItem};

/// Writes plain text to the native clipboard.
pub fn write_text_to_clipboard(cx: &mut App, text: impl Into<String>) {
    cx.write_to_clipboard(clipboard_text(text));
}
/// Creates a reusable GPUI clipboard item for plain text.
pub fn clipboard_text(text: impl Into<String>) -> ClipboardItem {
    ClipboardItem::new_string(text.into())
}
