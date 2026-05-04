use aura_components::{Divider, Space, Text, Title, Row, Col, Button};
use aura_core::Config;
use gpui::{App, Context, Entity, IntoElement, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> Entity<LayoutDemo> {
    cx.new(|_| LayoutDemo)
}

pub struct LayoutDemo;
impl Render for LayoutDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        div().flex().flex_col().gap_6()
            .child(div().flex().flex_col().gap_2()
                .child(Title::new("Divider 分割线").h2())
                .child(Text::new("Horizontal (default)"))
                .child(Divider::new())
                .child(Text::new("With label"))
                .child(Divider::new().label("Center Text"))
                .child(Text::new("Vertical"))
                .child(div().flex().flex_row().h(px(60.0)).gap_4().items_center()
                    .child(Text::new("Left"))
                    .child(Divider::new().vertical())
                    .child(Text::new("Right")))
            )
            .child(div().flex().flex_col().gap_2()
                .child(Title::new("Space 间距").h2())
                .child(Text::new("Horizontal gap (default 8px):"))
                .child(Space::new()
                    .child(Button::new("Button 1"))
                    .child(Button::new("Button 2"))
                    .child(Button::new("Button 3")))
                .child(Text::new("Vertical gap 20px:"))
                .child(Space::new().vertical().gap(px(20.0))
                    .child(Button::new("Vertical 1").primary())
                    .child(Button::new("Vertical 2").primary()))
            )
            .child(div().flex().flex_col().gap_2()
                .child(Title::new("Grid 栅格 (24格)").h2())
                .child(
                    Row::new()
                        .column(Col::new(24).child(grid_box(theme, "span 24", gpui::blue())))
                )
                .child(
                    Row::new()
                        .column(Col::new(12).child(grid_box(theme, "span 12", gpui::red())))
                        .column(Col::new(12).child(grid_box(theme, "span 12", gpui::green())))
                )
                .child(
                    Row::new()
                        .column(Col::new(8).child(grid_box(theme, "span 8", gpui::blue())))
                        .column(Col::new(8).child(grid_box(theme, "span 8", gpui::red())))
                        .column(Col::new(8).child(grid_box(theme, "span 8", gpui::green())))
                )
                .child(
                    Row::new()
                        .column(Col::new(6).child(grid_box(theme, "span 6", gpui::blue())))
                        .column(Col::new(6).child(grid_box(theme, "span 6", gpui::red())))
                        .column(Col::new(6).child(grid_box(theme, "span 6", gpui::green())))
                        .column(Col::new(6).child(grid_box(theme, "span 6", gpui::blue())))
                )
            )
    }
}

fn grid_box(theme: &aura_theme::Theme, text: &str, color: gpui::Hsla) -> impl IntoElement {
    div()
        .bg(color.opacity(0.5))
        .h(px(36.0))
        .rounded(px(4.0))
        .flex()
        .items_center()
        .justify_center()
        .text_color(theme.neutral.text_1)
        .text_size(px(12.0))
        .child(text.to_string())
}
