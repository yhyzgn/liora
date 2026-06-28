use gpui::{AnyView, App, Context, IntoElement, Render, Window, prelude::*};
use liora_components::layout_helpers::{page, section, showcase_card_wide, showcase_stack};
use liora_components::{Menu, MenuAction, MenuBar, MenuItem, Space, Tag, Text, toast_info};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| MenuDemo).into()
}

struct MenuDemo;

impl Render for MenuDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Menu 原生菜单",
            "平台中立的应用菜单描述和可视化预览，可用于系统菜单适配、自定义标题栏菜单和命令面板桥接。",
            Space::new().vertical().gap_xl().child(section(
                "Menu showcase",
                "同一份 descriptor 可以交给平台适配层，也可以直接用 Liora 原生预览渲染；内置常用 action，普通 item 悬停显示小手并可回调分发。",
                showcase_stack(vec![
                    showcase_card_wide(
                        "Horizontal menu bar",
                        "紧凑展示 File / Edit / View / Help 顶层菜单名，点击菜单名后才弹出对应菜单面板，适合自定义 TitleBar 菜单区域。",
                        horizontal_menu_bar(),
                    )
                    .into_any_element(),
                    showcase_card_wide(
                        "File menu",
                        "使用内置 New/Open/Save/Quit action，包含快捷键、分隔线、禁用项和最近文件子菜单。",
                        file_menu(),
                    )
                    .into_any_element(),
                    showcase_card_wide(
                        "System dialogs",
                        "OpenFile/OpenFiles/OpenFolder/OpenFolders/SaveAs 会调用 GPUI 官方系统路径对话框；选择或取消后通过 on_paths_selected 回传。",
                        system_dialog_menu(),
                    )
                    .into_any_element(),
                    showcase_card_wide(
                        "View menu",
                        "内置 command palette、sidebar/statusbar toggle、zoom action，便于自定义 TitleBar 复用。",
                        view_menu(),
                    )
                    .into_any_element(),
                    showcase_card_wide(
                        "Help actions",
                        "OpenUrl/CopyText 具备通用平台行为；其他 action 通过 on_select 交给应用分发。",
                        help_menu(),
                    )
                    .into_any_element(),
                    showcase_card_wide(
                        "Action catalog",
                        "完整列出 Menu 内置 action：哪些会由 Liora 直接执行，哪些只是标准命令语义，需要应用在 on_select 中分发。",
                        action_catalog(),
                    )
                    .into_any_element(),
                ]),
            )),
        )
    }
}

fn horizontal_menu_bar() -> impl IntoElement {
    MenuBar::new([
        file_menu().preview_width(gpui::px(240.0)),
        edit_menu().preview_width(gpui::px(220.0)),
        view_menu().preview_width(gpui::px(240.0)),
        help_menu().preview_width(gpui::px(260.0)),
    ])
    .on_select(|action, item, _, _| {
        toast_info!("Menu bar action: {} ({})", action.info().name, item.id);
    })
}

fn edit_menu() -> Menu {
    Menu::new("Edit")
        .perform_builtin_actions(false)
        .item(MenuItem::undo())
        .item(MenuItem::redo())
        .item(MenuItem::separator())
        .item(MenuItem::cut())
        .item(MenuItem::copy())
        .item(MenuItem::paste().disabled(true))
        .item(MenuItem::separator())
        .item(MenuItem::new("find", "Find").shortcut("Ctrl+F"))
        .item(MenuItem::new("replace", "Replace").shortcut("Ctrl+H"))
        .on_select(|action, item, _, _| {
            toast_info!("Edit menu: {} ({})", action.info().name, item.id);
        })
}

fn file_menu() -> Menu {
    Menu::new("File")
        .perform_builtin_actions(false)
        .item(MenuItem::new_window())
        .item(MenuItem::open())
        .item(MenuItem::open_file())
        .item(MenuItem::open_folder())
        .item(
            MenuItem::new("recent", "Open Recent")
                .child(MenuItem::new("recent-gallery", "liora-gallery"))
                .child(MenuItem::new("recent-docs", "liora-docs")),
        )
        .item(MenuItem::separator())
        .item(MenuItem::save())
        .item(MenuItem::new("publish", "Publish Release").disabled(true))
        .item(MenuItem::separator())
        .item(MenuItem::quit())
}

fn system_dialog_menu() -> Menu {
    Menu::new("Dialogs")
        .preview_width(gpui::px(360.0))
        .item(MenuItem::open_file())
        .item(MenuItem::open_files())
        .item(MenuItem::open_folder())
        .item(MenuItem::open_folders())
        .item(MenuItem::separator())
        .item(MenuItem::save_as())
        .on_paths_selected(|action, paths, _| match paths {
            Some(paths) if !paths.is_empty() => {
                toast_info!(
                    "{} selected: {}",
                    action.info().name,
                    paths
                        .iter()
                        .map(|path| path.display().to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
            _ => toast_info!("{} cancelled", action.info().name),
        })
        .on_select(|action, item, _, _| {
            toast_info!("Menu dispatched: {} ({})", action.info().name, item.id);
        })
}

fn view_menu() -> Menu {
    Menu::new("View")
        .perform_builtin_actions(false)
        .preview_width(gpui::px(320.0))
        .item(MenuItem::command_palette())
        .item(MenuItem::toggle_sidebar())
        .item(MenuItem::toggle_statusbar())
        .item(MenuItem::separator())
        .item(
            MenuItem::new("zoom", "Zoom")
                .child(MenuItem::action(MenuAction::ZoomIn, "Zoom In").shortcut("Ctrl++"))
                .child(MenuItem::action(MenuAction::ZoomOut, "Zoom Out").shortcut("Ctrl+-"))
                .child(MenuItem::action(MenuAction::ZoomReset, "Reset Zoom").shortcut("Ctrl+0")),
        )
}

fn help_menu() -> Menu {
    Menu::new("Help")
        .perform_builtin_actions(false)
        .preview_width(gpui::px(340.0))
        .item(MenuItem::open_url(
            "Open GitHub Repository",
            "https://github.com/yhyzgn/liora",
        ))
        .item(MenuItem::copy_text("Copy crate name", "liora"))
        .item(
            MenuItem::new("check-updates", "Check for Updates")
                .with_action(MenuAction::Custom("check-updates".into())),
        )
        .on_select(|action, item, _, _| {
            toast_info!("Menu action: {} ({})", action.info().name, item.id);
        })
}

fn action_catalog() -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_md()
        .children(MenuAction::catalog().into_iter().map(action_row))
}

fn action_row(action: MenuAction) -> impl IntoElement {
    let info = action.info();
    Space::new()
        .vertical()
        .gap_xs()
        .child(
            Space::new()
                .gap_sm()
                .wrap()
                .child(Text::new(info.name).bold())
                .child(Tag::new(info.id).info().round(true))
                .child(if info.handled_by_liora {
                    Tag::new("Liora handles").success().round(true)
                } else {
                    Tag::new("App dispatch").warning().round(true)
                }),
        )
        .child(Text::new(info.description).sm().wrap())
        .child(Text::new(info.effect).xs().wrap())
}

#[cfg(test)]
mod tests {
    #[test]
    fn native_menu_demo_is_standalone_and_rich() {
        let source = include_str!("native_menu_demo.rs");

        assert!(source.contains("Menu 原生菜单"));
        assert!(source.contains("Horizontal menu bar"));
        assert!(source.contains("horizontal_menu_bar"));
        assert!(source.contains("MenuBar::new"));
        assert!(source.contains("fn edit_menu"));
        assert!(source.contains("Menu::new"));
        assert!(source.contains("MenuItem::separator"));
        assert!(source.contains(".child(MenuItem"));
        assert!(source.contains("MenuItem::open_url"));
        assert!(source.contains("MenuItem::open_file"));
        assert!(source.contains("MenuItem::open_files"));
        assert!(source.contains("MenuItem::open_folder"));
        assert!(source.contains("MenuItem::open_folders"));
        assert!(source.contains("MenuItem::save_as"));
        assert!(source.contains("on_paths_selected"));
        assert!(source.contains("MenuAction::ZoomIn"));
        assert!(source.contains("on_select"));
        assert!(source.contains("Action catalog"));
        assert!(source.contains("MenuAction::catalog"));
        assert!(source.contains("perform_builtin_actions(false)"));
        assert!(source.contains("showcase_stack"));
    }
}
