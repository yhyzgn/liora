use aura_components::{Button, Popconfirm, Space};
use aura_core::{Config, Placement};
use gpui::{prelude::*, px, AnyView, App, Context, Render, Window, div};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| PopconfirmDemo).into()
}

struct PopconfirmDemo;

impl Render for PopconfirmDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

        div().flex().flex_col().gap_6()
            .child(section(theme, "Basic 基础用法", "点击按钮出现气泡确认框。"))
            .child(
                Space::new().gap(px(16.0))
                    .child(
                        Popconfirm::new(Button::new("Delete").danger())
                            .id("popconfirm-demo-delete")
                            .title("Are you sure to delete this task?")
                            .on_confirm(|_, _| println!("Confirmed!"))
                            .on_cancel(|_, _| println!("Cancelled!"))
                    )
                    .child(
                        Popconfirm::new(Button::new("Archive"))
                            .id("popconfirm-demo-archive")
                            .title("Archive this item?")
                            .confirm_text("Yes")
                            .cancel_text("No")
                    )
            )
            .child(section(theme, "Placements 位置", "Popconfirm 继承 Popover 的定位能力。"))
            .child(
                Space::new().gap(px(10.0))
                    .child(confirm_at("Top", Placement::Top))
                    .child(confirm_at("Bottom", Placement::Bottom))
                    .child(confirm_at("Left", Placement::Left))
                    .child(confirm_at("Right", Placement::Right))
                    .child(confirm_at("BottomEnd", Placement::BottomEnd))
            )
            .child(section(theme, "Custom text 自定义文案", "可自定义确认和取消按钮文本。"))
            .child(
                Space::new().gap(px(16.0))
                    .child(
                        Popconfirm::new(Button::new("Publish").success())
                            .id("popconfirm-demo-publish")
                            .title("Publish current draft?")
                            .confirm_text("Publish")
                            .cancel_text("Keep editing")
                            .placement(Placement::Top)
                    )
                    .child(
                        Popconfirm::new(Button::new("Danger action").danger())
                            .id("popconfirm-demo-danger-action")
                            .title("This action cannot be undone.")
                            .confirm_text("I understand")
                            .cancel_text("Abort")
                            .placement(Placement::BottomStart)
                    )
            )
    }
}

fn section(theme: &aura_theme::Theme, title: &'static str, desc: &'static str) -> impl IntoElement {
    div().flex().flex_col().gap_1()
        .child(div().text_lg().font_weight(gpui::FontWeight::BOLD).child(title))
        .child(div().text_sm().text_color(theme.neutral.text_3).child(desc))
}

fn confirm_at(label: &'static str, placement: Placement) -> Popconfirm {
    Popconfirm::new(Button::new(label).small())
        .id(format!("popconfirm-demo-placement-{}", label))
        .title(format!("Confirm at {:?}?", placement))
        .placement(placement)
}
