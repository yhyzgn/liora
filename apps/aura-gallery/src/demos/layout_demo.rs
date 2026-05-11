use aura_components::{Button, Col, Divider, Flex, Row, Space, Text};
use aura_core::Config;
use gpui::{App, Context, Entity, IntoElement, Render, Window, prelude::*};

use aura_components::layout_helpers::{page, section};

pub fn render(cx: &mut App) -> Entity<LayoutDemo> {
    cx.new(|_| LayoutDemo)
}

pub struct LayoutDemo;
impl Render for LayoutDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        page(
            "Layout 布局",
            "分割线、间距和栅格布局。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "Divider 分割线",
                    "基础分割线、带文字分割线和垂直分割线。",
                    Space::new()
                        .vertical()
                        .gap_sm()
                        .child(Text::new("Horizontal (default)"))
                        .child(Divider::new())
                        .child(Text::new("With label"))
                        .child(Divider::new().label("Center Text"))
                        .child(Text::new("Vertical"))
                        .child(
                            Flex::new()
                                .row()
                                .height_units(60.0)
                                .gap_lg()
                                .align_center()
                                .child(Text::new("Left"))
                                .child(Divider::new().vertical())
                                .child(Text::new("Right")),
                        ),
                ))
                .child(section(
                    "Space 间距",
                    "横向和纵向间距组合。",
                    Space::new()
                        .vertical()
                        .gap_sm()
                        .child(Text::new("Horizontal gap (default 8px):"))
                        .child(
                            Space::new()
                                .child(Button::new("Button 1"))
                                .child(Button::new("Button 2"))
                                .child(Button::new("Button 3")),
                        )
                        .child(Text::new("Vertical gap:"))
                        .child(
                            Space::new()
                                .vertical()
                                .gap_xl()
                                .child(Button::new("Vertical 1").primary())
                                .child(Button::new("Vertical 2").primary()),
                        ),
                ))
                .child(section(
                    "Grid 栅格 (24格)",
                    "使用 Row 和 Col 组合不同 span。",
                    Space::new()
                        .vertical()
                        .gap_sm()
                        .child(Row::new().column(Col::new(24).child(grid_box(
                            theme,
                            "span 24",
                            gpui::blue(),
                        ))))
                        .child(
                            Row::new()
                                .column(Col::new(12).child(grid_box(theme, "span 12", gpui::red())))
                                .column(Col::new(12).child(grid_box(
                                    theme,
                                    "span 12",
                                    gpui::green(),
                                ))),
                        )
                        .child(
                            Row::new()
                                .column(Col::new(8).child(grid_box(theme, "span 8", gpui::blue())))
                                .column(Col::new(8).child(grid_box(theme, "span 8", gpui::red())))
                                .column(Col::new(8).child(grid_box(
                                    theme,
                                    "span 8",
                                    gpui::green(),
                                ))),
                        )
                        .child(
                            Row::new()
                                .column(Col::new(6).child(grid_box(theme, "span 6", gpui::blue())))
                                .column(Col::new(6).child(grid_box(theme, "span 6", gpui::red())))
                                .column(Col::new(6).child(grid_box(theme, "span 6", gpui::green())))
                                .column(Col::new(6).child(grid_box(theme, "span 6", gpui::blue()))),
                        ),
                )),
        )
    }
}

fn grid_box(theme: &aura_theme::Theme, text: &str, color: gpui::Hsla) -> impl IntoElement {
    Flex::new()
        .row()
        .bg(color.opacity(0.5))
        .height_units(36.0)
        .rounded_units(4.0)
        .center()
        .text_color(theme.neutral.text_1)
        .text_xs()
        .child(text.to_string())
}
