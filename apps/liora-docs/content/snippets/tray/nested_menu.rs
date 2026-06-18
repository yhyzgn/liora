//! Recursive submenus: second-level, third-level, and deeper native menus.

use liora_tray::{TrayCommand, TrayMenuItemSpec};

pub fn nested_menu() -> Vec<TrayMenuItemSpec> {
    vec![TrayMenuItemSpec::submenu(
        "项目",
        vec![
            TrayMenuItemSpec::action("打开最近项目", TrayCommand::Custom("open-recent".into())),
            TrayMenuItemSpec::submenu(
                "工作区",
                vec![TrayMenuItemSpec::submenu(
                    "生产环境",
                    vec![
                        TrayMenuItemSpec::action(
                            "打开仪表盘",
                            TrayCommand::Custom("workspace-prod-dashboard".into()),
                        ),
                        TrayMenuItemSpec::action(
                            "打开日志",
                            TrayCommand::Custom("workspace-prod-logs".into()),
                        ),
                    ],
                )],
            ),
        ],
    )]
}
