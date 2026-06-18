use aura_components::{Button, Dropdown, Space};
use aura_core::Placement;
use gpui::{AnyView, App, Context, Render, Window, prelude::*};

use aura_components::layout_helpers::{row, row_md, section};

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
                "可禁用点击外部或 ESC 关闭；菜单项点击仍会执行回调并关闭菜单。",
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
    }
}

fn menu(id: &'static str, label: &'static str, placement: Placement) -> Dropdown {
    Dropdown::new(Button::new(label))
        .id(id)
        .placement(placement)
        .item("Action 1", |_, _| println!("Action 1"))
        .item("Action 2", |_, _| println!("Action 2"))
        .item("Disabled-looking item", |_, _| println!("Action 3"))
}
