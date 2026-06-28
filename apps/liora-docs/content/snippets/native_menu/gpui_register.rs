//! Register Menu descriptors through GPUI's official application menu API.

use gpui::App;
use liora_components::{Menu, MenuItem};

pub fn register_app_menus(cx: &mut App) {
    // This delegates to `App::set_menus`, GPUI's official platform menu API.
    // Use `register_gpui_menus_with_action_mapper` when each menu item should
    // invoke an application-owned `gpui::Action` registered with `cx.on_action`.
    Menu::register_gpui_menus(
        cx,
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
                .item(MenuItem::cut())
                .item(MenuItem::copy())
                .item(MenuItem::paste())
                .item(MenuItem::separator())
                .item(MenuItem::select_all()),
        ],
    );
}
