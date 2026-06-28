use gpui::{AnyView, App, Context, IntoElement, Render, Window, prelude::*};
use liora_components::layout_helpers::{page, section, showcase_card_wide, showcase_stack};
use liora_components::{
    NativeMenu, NativeMenuAction, NativeMenuItem, Space, Tag, Text, toast_info,
};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| NativeMenuDemo).into()
}

struct NativeMenuDemo;

impl Render for NativeMenuDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "NativeMenu 原生菜单",
            "平台中立的应用菜单描述和可视化预览，可用于系统菜单适配、自定义标题栏菜单和命令面板桥接。",
            Space::new().vertical().gap_xl().child(section(
                "NativeMenu showcase",
                "同一份 descriptor 可以交给平台适配层，也可以直接用 Liora 原生预览渲染；内置常用 action，普通 item 悬停显示小手并可回调分发。",
                showcase_stack(vec![
                    showcase_card_wide(
                        "Horizontal menu bar",
                        "横向展示 File / Edit / View / Help 多个菜单项组，模拟真实桌面应用菜单栏或自定义 TitleBar 菜单区域。",
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
                        "完整列出 NativeMenu 内置 action：哪些会由 Liora 直接执行，哪些只是标准命令语义，需要应用在 on_select 中分发。",
                        action_catalog(),
                    )
                    .into_any_element(),
                ]),
            )),
        )
    }
}

fn horizontal_menu_bar() -> impl IntoElement {
    Space::new()
        .gap_md()
        .wrap()
        .align_start()
        .child(file_menu().preview_width(gpui::px(220.0)))
        .child(edit_menu().preview_width(gpui::px(220.0)))
        .child(view_menu().preview_width(gpui::px(240.0)))
        .child(help_menu().preview_width(gpui::px(260.0)))
}

fn edit_menu() -> NativeMenu {
    NativeMenu::new("Edit")
        .perform_builtin_actions(false)
        .item(NativeMenuItem::copy_text("Copy", "Liora NativeMenu"))
        .item(NativeMenuItem::new("paste", "Paste").disabled(true))
        .item(NativeMenuItem::separator())
        .item(NativeMenuItem::new("find", "Find").shortcut("Ctrl+F"))
        .item(NativeMenuItem::new("replace", "Replace").shortcut("Ctrl+H"))
        .on_select(|action, item, _, _| {
            toast_info!("Edit menu: {} ({})", action.info().name, item.id);
        })
}

fn file_menu() -> NativeMenu {
    NativeMenu::new("File")
        .perform_builtin_actions(false)
        .item(NativeMenuItem::new_window())
        .item(NativeMenuItem::open())
        .item(NativeMenuItem::open_file())
        .item(NativeMenuItem::open_folder())
        .item(
            NativeMenuItem::new("recent", "Open Recent")
                .child(NativeMenuItem::new("recent-gallery", "liora-gallery"))
                .child(NativeMenuItem::new("recent-docs", "liora-docs")),
        )
        .item(NativeMenuItem::separator())
        .item(NativeMenuItem::save())
        .item(NativeMenuItem::new("publish", "Publish Release").disabled(true))
        .item(NativeMenuItem::separator())
        .item(NativeMenuItem::quit())
}

fn system_dialog_menu() -> NativeMenu {
    NativeMenu::new("Dialogs")
        .preview_width(gpui::px(360.0))
        .item(NativeMenuItem::open_file())
        .item(NativeMenuItem::open_files())
        .item(NativeMenuItem::open_folder())
        .item(NativeMenuItem::open_folders())
        .item(NativeMenuItem::separator())
        .item(NativeMenuItem::save_as())
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
            toast_info!(
                "NativeMenu dispatched: {} ({})",
                action.info().name,
                item.id
            );
        })
}

fn view_menu() -> NativeMenu {
    NativeMenu::new("View")
        .perform_builtin_actions(false)
        .preview_width(gpui::px(320.0))
        .item(NativeMenuItem::command_palette())
        .item(NativeMenuItem::toggle_sidebar())
        .item(NativeMenuItem::toggle_statusbar())
        .item(NativeMenuItem::separator())
        .item(
            NativeMenuItem::new("zoom", "Zoom")
                .child(
                    NativeMenuItem::action(NativeMenuAction::ZoomIn, "Zoom In").shortcut("Ctrl++"),
                )
                .child(
                    NativeMenuItem::action(NativeMenuAction::ZoomOut, "Zoom Out")
                        .shortcut("Ctrl+-"),
                )
                .child(
                    NativeMenuItem::action(NativeMenuAction::ZoomReset, "Reset Zoom")
                        .shortcut("Ctrl+0"),
                ),
        )
}

fn help_menu() -> NativeMenu {
    NativeMenu::new("Help")
        .perform_builtin_actions(false)
        .preview_width(gpui::px(340.0))
        .item(NativeMenuItem::open_url(
            "Open GitHub Repository",
            "https://github.com/yhyzgn/liora",
        ))
        .item(NativeMenuItem::copy_text("Copy crate name", "liora"))
        .item(
            NativeMenuItem::new("check-updates", "Check for Updates")
                .with_action(NativeMenuAction::Custom("check-updates".into())),
        )
        .on_select(|action, item, _, _| {
            toast_info!("NativeMenu action: {} ({})", action.info().name, item.id);
        })
}

fn action_catalog() -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_md()
        .children(NativeMenuAction::catalog().into_iter().map(action_row))
}

fn action_row(action: NativeMenuAction) -> impl IntoElement {
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

        assert!(source.contains("NativeMenu 原生菜单"));
        assert!(source.contains("Horizontal menu bar"));
        assert!(source.contains("horizontal_menu_bar"));
        assert!(source.contains("fn edit_menu"));
        assert!(source.contains("NativeMenu::new"));
        assert!(source.contains("NativeMenuItem::separator"));
        assert!(source.contains(".child(NativeMenuItem"));
        assert!(source.contains("NativeMenuItem::open_url"));
        assert!(source.contains("NativeMenuItem::open_file"));
        assert!(source.contains("NativeMenuItem::open_files"));
        assert!(source.contains("NativeMenuItem::open_folder"));
        assert!(source.contains("NativeMenuItem::open_folders"));
        assert!(source.contains("NativeMenuItem::save_as"));
        assert!(source.contains("on_paths_selected"));
        assert!(source.contains("NativeMenuAction::ZoomIn"));
        assert!(source.contains("on_select"));
        assert!(source.contains("Action catalog"));
        assert!(source.contains("NativeMenuAction::catalog"));
        assert!(source.contains("perform_builtin_actions(false)"));
        assert!(source.contains("showcase_stack"));
    }
}
