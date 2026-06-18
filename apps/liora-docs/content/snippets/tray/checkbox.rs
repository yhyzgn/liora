//! CheckBox menu items for persistent tray preferences.

use liora_tray::{TrayCommand, TrayMenuItemSpec};

pub fn preference_menu() -> Vec<TrayMenuItemSpec> {
    vec![
        TrayMenuItemSpec::check(
            "启动时自动显示主窗口",
            TrayCommand::Custom("auto-show-window".into()),
            true,
        ),
        TrayMenuItemSpec::check(
            "静音通知",
            TrayCommand::Custom("mute-notifications".into()),
            false,
        ),
    ]
}
