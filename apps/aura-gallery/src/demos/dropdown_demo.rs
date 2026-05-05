use aura_components::{Button, Dropdown, Space};
use aura_core::{Config, Placement};
use gpui::{prelude::*, px, AnyView, App, Context, Render, Window, div};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| DropdownDemo).into()
}

struct DropdownDemo;

impl Render for DropdownDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

        div().flex().flex_col().gap_6()
            .child(section(theme, "Basic 基础用法", "点击触发下拉菜单，点击菜单项后自动关闭。"))
            .child(
                Space::new().gap(px(16.0))
                    .child(menu("Actions", Placement::BottomStart))
                    .child(menu("Bottom End", Placement::BottomEnd))
                    .child(menu("Top Start", Placement::TopStart))
            )
            .child(section(theme, "Placements 位置", "可通过 placement 调整下拉方向和对齐。"))
            .child(
                Space::new().gap(px(10.0))
                    .child(menu("Top", Placement::Top))
                    .child(menu("Bottom", Placement::Bottom))
                    .child(menu("Left", Placement::Left))
                    .child(menu("Right", Placement::Right))
            )
    }
}

fn section(theme: &aura_theme::Theme, title: &'static str, desc: &'static str) -> impl IntoElement {
    div().flex().flex_col().gap_1()
        .child(div().text_lg().font_weight(gpui::FontWeight::BOLD).child(title))
        .child(div().text_sm().text_color(theme.neutral.text_3).child(desc))
}

fn menu(label: &'static str, placement: Placement) -> Dropdown {
    Dropdown::new(Button::new(label))
        .placement(placement)
        .item("Action 1", |_, _| println!("Action 1"))
        .item("Action 2", |_, _| println!("Action 2"))
        .item("Disabled-looking item", |_, _| println!("Action 3"))
}
