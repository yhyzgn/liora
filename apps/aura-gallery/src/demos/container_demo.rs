use aura_components::{Button, Container, Divider, Flex, Space, Text, Title};
use gpui::{App, Context, Entity, IntoElement, Render, Window, prelude::*};

use super::common::{page, section};

pub fn render(cx: &mut App) -> Entity<ContainerDemo> {
    cx.new(|_| ContainerDemo)
}

pub struct ContainerDemo;

impl Render for ContainerDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Container 容器",
            "页面级容器与布局辅助组件组合。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "Space 间距",
                    "横向与纵向间距示例。",
                    Space::new()
                        .vertical()
                        .gap_sm()
                        .child(
                            Space::new()
                                .child(Button::new("Button 1"))
                                .child(Button::new("Button 2"))
                                .child(Button::new("Button 3")),
                        )
                        .child(
                            Space::new()
                                .vertical()
                                .child(Button::new("Vertical 1").primary())
                                .child(Button::new("Vertical 2").primary()),
                        ),
                ))
                .child(section(
                    "Divider 分割线",
                    "展示水平、带标签和垂直分割线。",
                    Space::new()
                        .vertical()
                        .gap_sm()
                        .child(
                            Space::new()
                                .vertical()
                                .child(Text::new("Above divider"))
                                .child(Divider::new())
                                .child(Text::new("Below divider")),
                        )
                        .child(Divider::new().label("Center Label"))
                        .child(
                            Flex::new()
                                .row()
                                .align_center()
                                .gap_lg()
                                .height_units(48.0)
                                .child(Text::new("Section 1"))
                                .child(Divider::new().vertical())
                                .child(Text::new("Section 2"))
                                .child(Divider::new().vertical())
                                .child(Text::new("Section 3")),
                        ),
                ))
                .child(section(
                    "Container 布局容器",
                    "Header / Aside / Main / Footer 组合。",
                    Flex::new().height_units(300.0).w_full().border().child(
                        Container::new()
                            .header(Title::new("Header").h5())
                            .aside(Flex::new().padding_md().child(Text::new("Aside Sidebar")))
                            .footer(Text::new("Footer"))
                            .child(
                                Flex::new()
                                    .padding_md()
                                    .child(Text::new("Main Content Area")),
                            ),
                    ),
                )),
        )
    }
}
