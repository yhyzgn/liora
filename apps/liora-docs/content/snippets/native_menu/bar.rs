//! Horizontal NativeMenu bar example.

use gpui::IntoElement;
use liora_components::{NativeMenu, NativeMenuAction, NativeMenuItem, Space};

pub fn native_menu_bar() -> impl IntoElement {
    Space::new()
        .gap_md()
        .wrap()
        .align_start()
        .child(file_menu())
        .child(edit_menu())
        .child(view_menu())
        .child(help_menu())
}

fn file_menu() -> NativeMenu {
    NativeMenu::new("File")
        .perform_builtin_actions(false)
        .preview_width(gpui::px(220.0))
        .item(NativeMenuItem::new_window())
        .item(NativeMenuItem::open_file())
        .item(NativeMenuItem::open_folder())
        .item(NativeMenuItem::separator())
        .item(NativeMenuItem::save())
        .item(NativeMenuItem::quit())
}

fn edit_menu() -> NativeMenu {
    NativeMenu::new("Edit")
        .perform_builtin_actions(false)
        .preview_width(gpui::px(220.0))
        .item(NativeMenuItem::copy_text("Copy", "Liora NativeMenu"))
        .item(NativeMenuItem::new("paste", "Paste").disabled(true))
        .item(NativeMenuItem::separator())
        .item(NativeMenuItem::new("find", "Find").shortcut("Ctrl+F"))
}

fn view_menu() -> NativeMenu {
    NativeMenu::new("View")
        .perform_builtin_actions(false)
        .preview_width(gpui::px(240.0))
        .item(NativeMenuItem::command_palette())
        .item(NativeMenuItem::toggle_sidebar())
        .item(NativeMenuItem::toggle_statusbar())
        .item(NativeMenuItem::separator())
        .item(NativeMenuItem::action(NativeMenuAction::ZoomIn, "Zoom In"))
        .item(NativeMenuItem::action(
            NativeMenuAction::ZoomOut,
            "Zoom Out",
        ))
}

fn help_menu() -> NativeMenu {
    NativeMenu::new("Help")
        .perform_builtin_actions(false)
        .preview_width(gpui::px(260.0))
        .item(NativeMenuItem::open_url(
            "Open GitHub Repository",
            "https://github.com/yhyzgn/liora",
        ))
        .item(NativeMenuItem::new("about", "About Liora"))
}
