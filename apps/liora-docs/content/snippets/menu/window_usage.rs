//! Minimal in-window MenuBar placement example.

use gpui::{App, IntoElement, ParentElement, Styled, Window, div, px};
use liora_components::{AppWindowFrame, Container, Menu, MenuBar, MenuItem};

fn app_menus() -> [Menu; 2] {
    [
        Menu::new("File")
            .item(MenuItem::open_file())
            .item(MenuItem::open_folder())
            .item(MenuItem::separator())
            .item(MenuItem::quit()),
        Menu::new("Edit")
            .item(MenuItem::undo())
            .item(MenuItem::redo())
            .item(MenuItem::separator())
            .item(MenuItem::copy())
            .item(MenuItem::paste()),
    ]
}

pub fn render_root(window: &mut Window, cx: &mut App) -> impl IntoElement {
    let menu_bar = MenuBar::new(app_menus()).perform_builtin_actions(false);

    // Required for MenuBar dropdowns and every Liora popover-based component.
    liora_core::render_active_popover_in_window(window, cx);

    AppWindowFrame::new(
        "My App",
        Container::new()
            .header(div().w_full().child(menu_bar))
            .header_height(px(40.0))
            .child("Window body"),
    )
}
