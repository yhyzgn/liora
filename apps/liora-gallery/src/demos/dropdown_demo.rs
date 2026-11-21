use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::{Button, Dropdown, Space, toast_info};
use liora_core::Placement;

use liora_components::layout_helpers::{row, row_md, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| DropdownDemo).into()
}

struct DropdownDemo;

impl Render for DropdownDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Space::new()
            .vertical()
            .gap_xl()
            .child(section(
                "Basic 基础用法",
                "点击触发下拉菜单，点击菜单项后自动关闭。",
                row(vec![
                    menu("dropdown-demo-actions", "Actions", Placement::BottomStart),
                    menu(
                        "dropdown-demo-bottom-end",
                        "Bottom End",
                        Placement::BottomEnd,
                    ),
                    menu("dropdown-demo-top-start", "Top Start", Placement::TopStart),
                ]),
            ))
            .child(section(
                "Placements 位置",
                "可通过 placement 调整下拉方向和对齐。",
                row_md(vec![
                    menu("dropdown-demo-top", "Top", Placement::Top),
                    menu("dropdown-demo-bottom", "Bottom", Placement::Bottom),
                    menu("dropdown-demo-left", "Left", Placement::Left),
                    menu("dropdown-demo-right", "Right", Placement::Right),
                ]),
            ))
            .child(section(
                "Close policy 关闭策略",
                "可禁用点击外部或 ESC 关闭；激活菜单项时先关闭原菜单，再执行回调。",
                row(vec![
                    Dropdown::new(Button::new("Manual close menu"))
                        .id("dropdown-demo-manual-close")
                        .placement(Placement::BottomStart)
                        .close_on_click_outside(false)
                        .close_on_escape(false)
                        .item("Save draft", |_, _| println!("Save draft"))
                        .item("Duplicate", |_, _| println!("Duplicate")),
                ]),
            ))
            .child(section(
                "Scrollable menu 长菜单",
                "滚轮访问末项；Home/End 和上下方向键会自动滚动到当前菜单项。",
                scrollable_menu(),
            ))
    }
}

fn menu(id: &'static str, label: &'static str, placement: Placement) -> Dropdown {
    Dropdown::new(Button::new(label))
        .id(id)
        .placement(placement)
        .item("Action 1", |_, _| println!("Action 1"))
        .item("Action 2", |_, _| println!("Action 2"))
        .disabled_item("Disabled item")
}

pub(super) const LONG_MENU_LABELS: [&str; 12] = [
    "Action 01",
    "Action 02",
    "Action 03",
    "Action 04",
    "Action 05",
    "Action 06",
    "Action 07",
    "Action 08",
    "Action 09",
    "Action 10",
    "Action 11",
    "Last action",
];

fn scrollable_menu() -> Dropdown {
    let mut menu = Dropdown::new(Button::new("Long menu"))
        .id("dropdown-demo-scrollable")
        .placement(Placement::BottomStart);
    for label in LONG_MENU_LABELS {
        menu = menu.item(label, move |_, _| toast_info!("{label}"));
    }
    menu
}

pub(super) fn render_scrollable(cx: &mut App) -> AnyView {
    cx.new(|_| ScrollableMenuDemo).into()
}

struct ScrollableMenuDemo;

impl Render for ScrollableMenuDemo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        scrollable_menu()
    }
}
