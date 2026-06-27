use gpui::IntoElement;
use liora_components::{NativeMenu, NativeMenuItem, Text};

pub fn native_menu_descriptor() -> impl IntoElement {
    let menu = NativeMenu::new("File").item(NativeMenuItem::new("open", "Open").shortcut("Ctrl+O"));
    Text::new(format!("{} menu items", menu.items.len()))
}
