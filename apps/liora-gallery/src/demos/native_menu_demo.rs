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
                        "File menu",
                        "使用内置 New/Open/Save/Quit action，包含快捷键、分隔线、禁用项和最近文件子菜单。",
                        file_menu(),
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

fn file_menu() -> NativeMenu {
    NativeMenu::new("File")
        .perform_builtin_actions(false)
        .item(NativeMenuItem::new_window())
        .item(NativeMenuItem::open())
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
        assert!(source.contains("NativeMenu::new"));
        assert!(source.contains("NativeMenuItem::separator"));
        assert!(source.contains(".child(NativeMenuItem"));
        assert!(source.contains("NativeMenuItem::open_url"));
        assert!(source.contains("NativeMenuAction::ZoomIn"));
        assert!(source.contains("on_select"));
        assert!(source.contains("Action catalog"));
        assert!(source.contains("NativeMenuAction::catalog"));
        assert!(source.contains("perform_builtin_actions(false)"));
        assert!(source.contains("showcase_stack"));
    }
}
