//! Horizontal Menu for top navigation.

use liora_components::{NavigationMenu, NavigationMenuMode, toast_info};
use liora_icons_lucide::IconName;

pub fn horizontal_menu() -> NavigationMenu {
    NavigationMenu::new()
        .id("docs-menu-horizontal")
        .mode(NavigationMenuMode::Horizontal)
        .default_active("1")
        .on_select(|id, _, _| toast_info!("active menu: {}", id))
        .item("1", "处理中心", Some(IconName::List))
        .submenu("2", "我的工作台", Some(IconName::Briefcase), |s| {
            s.item("2-1", "选项1", None).item("2-2", "选项2", None)
        })
        .item("3", "消息中心", Some(IconName::Bell))
}
