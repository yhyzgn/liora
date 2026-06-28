//! Horizontal Menu bar example.

use gpui::IntoElement;
use liora_components::{Menu, MenuAction, MenuBar, MenuItem};

pub fn native_menu_bar() -> impl IntoElement {
    MenuBar::new([
        file_menu().preview_width(gpui::px(240.0)),
        edit_menu().preview_width(gpui::px(220.0)),
        view_menu().preview_width(gpui::px(240.0)),
        help_menu().preview_width(gpui::px(260.0)),
    ])
}

fn file_menu() -> Menu {
    Menu::new("File")
        .perform_builtin_actions(false)
        .preview_width(gpui::px(220.0))
        .item(MenuItem::new_window())
        .item(MenuItem::open_file())
        .item(MenuItem::open_folder())
        .item(MenuItem::separator())
        .item(MenuItem::save())
        .item(MenuItem::quit())
}

fn edit_menu() -> Menu {
    Menu::new("Edit")
        .perform_builtin_actions(false)
        .preview_width(gpui::px(220.0))
        .item(MenuItem::undo())
        .item(MenuItem::redo())
        .item(MenuItem::separator())
        .item(MenuItem::cut())
        .item(MenuItem::copy())
        .item(MenuItem::paste().disabled(true))
        .item(MenuItem::separator())
        .item(MenuItem::new("find", "Find").shortcut("Ctrl+F"))
}

fn view_menu() -> Menu {
    Menu::new("View")
        .perform_builtin_actions(false)
        .preview_width(gpui::px(240.0))
        .item(MenuItem::command_palette())
        .item(MenuItem::toggle_sidebar())
        .item(MenuItem::toggle_statusbar())
        .item(MenuItem::separator())
        .item(MenuItem::action(MenuAction::ZoomIn, "Zoom In"))
        .item(MenuItem::action(MenuAction::ZoomOut, "Zoom Out"))
}

fn help_menu() -> Menu {
    Menu::new("Help")
        .perform_builtin_actions(false)
        .preview_width(gpui::px(260.0))
        .item(MenuItem::open_url(
            "Open GitHub Repository",
            "https://github.com/yhyzgn/liora",
        ))
        .item(MenuItem::new("about", "About Liora"))
}
