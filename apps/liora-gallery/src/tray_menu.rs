use liora_tray::{TrayCommand, TrayMenuItemSpec};

pub fn gallery_tray_menu() -> Vec<TrayMenuItemSpec> {
    vec![
        TrayMenuItemSpec::action("显示窗口", TrayCommand::Show),
        TrayMenuItemSpec::action("隐藏窗口", TrayCommand::Hide),
        TrayMenuItemSpec::check(
            "状态栏驻留",
            TrayCommand::Custom("resident-enabled".into()),
            true,
        ),
        TrayMenuItemSpec::check(
            "启动时自动显示",
            TrayCommand::Custom("auto-show".into()),
            true,
        ),
        TrayMenuItemSpec::separator(),
        TrayMenuItemSpec::submenu(
            "切换图标",
            vec![
                TrayMenuItemSpec::action("默认图标", TrayCommand::SetIcon("default".into())),
                TrayMenuItemSpec::action("同步中", TrayCommand::SetIcon("syncing".into())),
                TrayMenuItemSpec::action("错误状态", TrayCommand::SetIcon("error".into())),
            ],
        ),
        TrayMenuItemSpec::submenu(
            "多级菜单",
            vec![TrayMenuItemSpec::submenu(
                "二级菜单",
                vec![TrayMenuItemSpec::submenu(
                    "三级菜单",
                    vec![TrayMenuItemSpec::action(
                        "三级动作",
                        TrayCommand::Custom("deep-action".into()),
                    )],
                )],
            )],
        ),
        TrayMenuItemSpec::separator(),
        TrayMenuItemSpec::action("退出", TrayCommand::Quit),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gallery_tray_menu_covers_gallery_demo_behaviors() {
        let menu = gallery_tray_menu();
        assert!(matches!(menu[2], TrayMenuItemSpec::Check { .. }));
        assert!(matches!(menu[3], TrayMenuItemSpec::Check { .. }));
        assert!(menu.iter().any(
            |item| matches!(item, TrayMenuItemSpec::Submenu { label, .. } if label == "切换图标")
        ));
        assert!(menu.iter().any(|item| matches!(item, TrayMenuItemSpec::Submenu { label, children, .. } if label == "多级菜单" && !children.is_empty())));
    }
}
