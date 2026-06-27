use gpui::IntoElement;
use liora_components::{Text, clipboard_text};

pub fn clipboard_helper() -> impl IntoElement {
    let _item = clipboard_text("Liora");
    Text::new("Use write_text_to_clipboard(cx, text) inside event handlers.")
}
