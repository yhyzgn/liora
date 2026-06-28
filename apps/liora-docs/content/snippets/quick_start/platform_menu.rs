//! Register the GPUI platform menu and render an in-window fallback MenuBar.

use gpui::{App, IntoElement, ParentElement, Styled, Window, div, px};
use liora_components::{AppWindowFrame, Container, Menu, MenuBar, MenuItem};

fn app_menus() -> [Menu; 2] {
    [
        Menu::new("File")
            .item(MenuItem::open_file())
            .item(MenuItem::open_folder())
            .item(MenuItem::separator())
            .item(MenuItem::save())
            .item(MenuItem::quit()),
        Menu::new("Edit")
            .item(MenuItem::undo())
            .item(MenuItem::redo())
            .item(MenuItem::separator())
            .item(MenuItem::copy())
            .item(MenuItem::paste()),
    ]
}

pub fn register_platform_menu(cx: &mut App) {
    // Official GPUI platform menu path: delegates to App::set_menus.
    Menu::register(cx, app_menus());
}

pub fn render_root_with_menu(window: &mut Window, cx: &mut App) -> impl IntoElement {
    // Required for the in-window MenuBar dropdowns.
    liora_core::render_active_popover_in_window(window, cx);

    AppWindowFrame::new(
        "Liora Native Demo",
        Container::new()
            .header(
                div()
                    .w_full()
                    .px_3()
                    .py_2()
                    .child(MenuBar::new(app_menus()).perform_builtin_actions(false)),
            )
            .header_height(px(48.0))
            .child("Window body"),
    )
}
