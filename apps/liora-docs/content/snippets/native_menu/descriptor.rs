//! NativeMenu descriptor and preview example.

use liora_components::{NativeMenu, NativeMenuAction, NativeMenuItem};

pub fn native_menu_descriptor() -> NativeMenu {
    NativeMenu::new("File")
        .item(NativeMenuItem::new_window())
        .item(NativeMenuItem::open())
        .item(NativeMenuItem::open_file())
        .item(NativeMenuItem::open_files())
        .item(NativeMenuItem::open_folder())
        .item(NativeMenuItem::open_folders())
        .item(
            NativeMenuItem::new("recent", "Open Recent")
                .child(NativeMenuItem::new("recent-gallery", "liora-gallery"))
                .child(NativeMenuItem::new("recent-docs", "liora-docs")),
        )
        .item(NativeMenuItem::separator())
        .item(NativeMenuItem::save())
        .item(NativeMenuItem::save_as())
        .item(NativeMenuItem::open_url(
            "Open GitHub Repository",
            "https://github.com/yhyzgn/liora",
        ))
        .item(
            NativeMenuItem::new("check-updates", "Check for Updates")
                .with_action(NativeMenuAction::Custom("check-updates".into())),
        )
        .item(NativeMenuItem::new("publish", "Publish Release").disabled(true))
        .on_paths_selected(|action, paths, _| {
            println!("{} -> {:?}", action.info().name, paths);
        })
}
