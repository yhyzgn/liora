//! NativeMenu descriptor and preview example.

use liora_components::{NativeMenu, NativeMenuItem};

pub fn native_menu_descriptor() -> NativeMenu {
    NativeMenu::new("File")
        .item(NativeMenuItem::new("new-window", "New Window").shortcut("Ctrl+Shift+N"))
        .item(NativeMenuItem::new("open", "Open...").shortcut("Ctrl+O"))
        .item(
            NativeMenuItem::new("recent", "Open Recent")
                .child(NativeMenuItem::new("recent-gallery", "liora-gallery"))
                .child(NativeMenuItem::new("recent-docs", "liora-docs")),
        )
        .item(NativeMenuItem::separator())
        .item(NativeMenuItem::new("save", "Save").shortcut("Ctrl+S"))
        .item(NativeMenuItem::new("publish", "Publish Release").disabled(true))
}
