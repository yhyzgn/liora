use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::{Avatar, Badge, BadgeType, Button, Space, Text};

use liora_components::layout_helpers::{page, row, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| BadgeDemo).into()
}

struct BadgeDemo;

impl Render for BadgeDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Badge 徽章",
            "按钮和图标右上角的提示信息。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "基础用法",
                    "展示不同语义类型的数字徽章。",
                    row(vec![
                        Badge::new(Button::new("Messages"))
                            .value("5")
                            .into_any_element(),
                        Badge::new(Button::new("Updates"))
                            .value("10")
                            .badge_type(BadgeType::Primary)
                            .into_any_element(),
                        Badge::new(Button::new("Alerts"))
                            .value("2")
                            .badge_type(BadgeType::Warning)
                            .into_any_element(),
                    ]),
                ))
                .child(section(
                    "最大值",
                    "超过最大值时显示 max+。",
                    row(vec![
                        Badge::new(Button::new("Messages"))
                            .value("200")
                            .max(99)
                            .into_any_element(),
                        Badge::new(Button::new("Updates"))
                            .value("50")
                            .max(10)
                            .into_any_element(),
                    ]),
                ))
                .child(section(
                    "小红点",
                    "使用 dot 模式提示有更新。",
                    row(vec![
                        Badge::new(Text::new("Query"))
                            .is_dot(true)
                            .into_any_element(),
                        Badge::new(Avatar::new()).is_dot(true).into_any_element(),
                    ]),
                )),
        )
    }
}
