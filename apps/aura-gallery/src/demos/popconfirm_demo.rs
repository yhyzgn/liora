use aura_components::{Button, Popconfirm, Space};
use aura_core::Placement;
use gpui::{AnyView, App, Context, Render, Window, prelude::*};

use aura_components::layout_helpers::{page, row, row_md, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| PopconfirmDemo).into()
}

struct PopconfirmDemo;

impl Render for PopconfirmDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Popconfirm 气泡确认框",
            "点击按钮出现气泡确认框。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "Basic 基础用法",
                    "点击按钮出现气泡确认框。",
                    row(vec![
                        Popconfirm::new(Button::new("Delete").danger())
                            .id("popconfirm-demo-delete")
                            .title("Are you sure to delete this task?")
                            .on_confirm(|_, _| println!("Confirmed!"))
                            .on_cancel(|_, _| println!("Cancelled!")),
                        Popconfirm::new(Button::new("Archive"))
                            .id("popconfirm-demo-archive")
                            .title("Archive this item?")
                            .confirm_text("Yes")
                            .cancel_text("No"),
                    ]),
                ))
                .child(section(
                    "Placements 位置",
                    "Popconfirm 继承 Popover 的定位能力。",
                    row_md(vec![
                        confirm_at("Top", Placement::Top),
                        confirm_at("Bottom", Placement::Bottom),
                        confirm_at("Left", Placement::Left),
                        confirm_at("Right", Placement::Right),
                        confirm_at("BottomEnd", Placement::BottomEnd),
                    ]),
                ))
                .child(section(
                    "Custom text 自定义文案",
                    "可自定义确认和取消按钮文本。",
                    row(vec![
                        Popconfirm::new(Button::new("Publish").success())
                            .id("popconfirm-demo-publish")
                            .title("Publish current draft?")
                            .confirm_text("Publish")
                            .cancel_text("Keep editing")
                            .placement(Placement::Top),
                        Popconfirm::new(Button::new("Danger action").danger())
                            .id("popconfirm-demo-danger-action")
                            .title("This action cannot be undone.")
                            .confirm_text("I understand")
                            .cancel_text("Abort")
                            .placement(Placement::BottomStart),
                    ]),
                )),
        )
    }
}

fn confirm_at(label: &'static str, placement: Placement) -> Popconfirm {
    Popconfirm::new(Button::new(label).small())
        .id(format!("popconfirm-demo-placement-{}", label))
        .title(format!("Confirm at {:?}?", placement))
        .placement(placement)
}
