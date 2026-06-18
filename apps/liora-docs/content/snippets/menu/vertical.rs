//! Vertical Menu for side navigation.

use liora_components::{Menu, MenuMode, toast_info};
use liora_icons_lucide::IconName;

pub fn vertical_menu() -> Menu {
    Menu::new()
        .id("docs-menu-vertical")
        .mode(MenuMode::Vertical)
        .default_active("1")
        .on_select(|id, _, _| toast_info!("active menu: {}", id))
        .item("1", "导航一", Some(IconName::House))
        .submenu("2", "导航二", Some(IconName::Settings), |s| {
            s.item("2-1", "选项1", None)
                .item("2-2", "选项2", None)
                .group("分组一", |g| {
                    g.item("2-3", "选项3", None).item("2-4", "选项4", None)
                })
        })
        .item("3", "导航三", Some(IconName::MessageSquare))
}
