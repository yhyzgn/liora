//! Menu descriptor and preview example.

use liora_components::{Menu, MenuAction, MenuItem};

pub fn native_menu_descriptor() -> Menu {
    Menu::new("File")
        .item(MenuItem::new_window())
        .item(MenuItem::open())
        .item(MenuItem::open_file())
        .item(MenuItem::open_files())
        .item(MenuItem::open_folder())
        .item(MenuItem::open_folders())
        .item(
            MenuItem::new("recent", "Open Recent")
                .child(MenuItem::new("recent-gallery", "liora-gallery"))
                .child(MenuItem::new("recent-docs", "liora-docs")),
        )
        .item(MenuItem::separator())
        .item(MenuItem::save())
        .item(MenuItem::save_as())
        .item(MenuItem::open_url(
            "Open GitHub Repository",
            "https://github.com/yhyzgn/liora",
        ))
        .item(
            MenuItem::new("check-updates", "Check for Updates")
                .with_action(MenuAction::Custom("check-updates".into())),
        )
        .item(MenuItem::new("publish", "Publish Release").disabled(true))
        .on_paths_selected(|action, paths, _| {
            println!("{} -> {:?}", action.info().name, paths);
        })
}
