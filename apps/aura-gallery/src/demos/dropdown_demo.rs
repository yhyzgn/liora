use aura_components::{Button, Dropdown, Space};
use aura_core::{Config, Placement};
use gpui::{AnyView, App, Context, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| DropdownDemo).into()
}

struct DropdownDemo;

impl Render for DropdownDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

        div()
            .flex()
            .flex_col()
            .gap_6()
            .child(section(
                theme,
                "Basic 基础用法",
                "点击触发下拉菜单，点击菜单项后自动关闭。",
            ))
            .child(
                Space::new()
                    .gap(px(16.0))
                    .child(menu(
                        "dropdown-demo-actions",
                        "Actions",
                        Placement::BottomStart,
                    ))
                    .child(menu(
                        "dropdown-demo-bottom-end",
                        "Bottom End",
                        Placement::BottomEnd,
                    ))
                    .child(menu(
                        "dropdown-demo-top-start",
                        "Top Start",
                        Placement::TopStart,
                    )),
            )
            .child(section(
                theme,
                "Placements 位置",
                "可通过 placement 调整下拉方向和对齐。",
            ))
            .child(
                Space::new()
                    .gap(px(10.0))
                    .child(menu("dropdown-demo-top", "Top", Placement::Top))
                    .child(menu("dropdown-demo-bottom", "Bottom", Placement::Bottom))
                    .child(menu("dropdown-demo-left", "Left", Placement::Left))
                    .child(menu("dropdown-demo-right", "Right", Placement::Right)),
            )
    }
}

fn section(theme: &aura_theme::Theme, title: &'static str, desc: &'static str) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(
            div()
                .text_lg()
                .font_weight(gpui::FontWeight::BOLD)
                .child(title),
        )
        .child(div().text_sm().text_color(theme.neutral.text_3).child(desc))
}

fn menu(id: &'static str, label: &'static str, placement: Placement) -> Dropdown {
    Dropdown::new(Button::new(label))
        .id(id)
        .placement(placement)
        .item("Action 1", |_, _| println!("Action 1"))
        .item("Action 2", |_, _| println!("Action 2"))
        .item("Disabled-looking item", |_, _| println!("Action 3"))
}
