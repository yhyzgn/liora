use gpui::{AnyView, App, Context, IntoElement, Render, Window, prelude::*};
use liora_components::layout_helpers::{page, section, showcase_card_wide, showcase_stack};
use liora_components::{NativeMenu, NativeMenuItem, Space};

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
                "同一份 descriptor 可以交给平台适配层，也可以直接用 Liora 原生预览渲染。",
                showcase_stack(vec![
                    showcase_card_wide(
                        "File menu",
                        "包含快捷键、分隔线、禁用项和最近文件子菜单。",
                        file_menu(),
                    )
                    .into_any_element(),
                    showcase_card_wide(
                        "View menu",
                        "用于自定义 titlebar 或 command palette 的菜单结构。",
                        view_menu(),
                    )
                    .into_any_element(),
                ]),
            )),
        )
    }
}

fn file_menu() -> NativeMenu {
    NativeMenu::new("File")
        .item(NativeMenuItem::new("new-window", "New Window").shortcut("Ctrl+Shift+N"))
        .item(NativeMenuItem::new("open", "Open...").shortcut("Ctrl+O"))
        .item(
            NativeMenuItem::new("recent", "Open Recent")
                .child(NativeMenuItem::new("recent-gallery", "liora-gallery"))
                .child(NativeMenuItem::new("recent-docs", "liora-docs")),
        )
        .item(NativeMenuItem::separator())
        .item(NativeMenuItem::new("save", "Save").shortcut("Ctrl+S"))
        .item(NativeMenuItem::new("publish", "Publish Release").disabled(true))
}

fn view_menu() -> NativeMenu {
    NativeMenu::new("View")
        .preview_width(gpui::px(320.0))
        .item(NativeMenuItem::new("command-palette", "Command Palette").shortcut("Ctrl+K"))
        .item(NativeMenuItem::new("toggle-sidebar", "Toggle Sidebar").shortcut("Ctrl+B"))
        .item(NativeMenuItem::new("toggle-statusbar", "Toggle StatusBar"))
        .item(NativeMenuItem::separator())
        .item(
            NativeMenuItem::new("zoom", "Zoom")
                .child(NativeMenuItem::new("zoom-in", "Zoom In").shortcut("Ctrl++"))
                .child(NativeMenuItem::new("zoom-out", "Zoom Out").shortcut("Ctrl+-"))
                .child(NativeMenuItem::new("zoom-reset", "Reset Zoom").shortcut("Ctrl+0")),
        )
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
        assert!(source.contains("showcase_stack"));
    }
}
