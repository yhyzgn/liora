use aura_components::{Button, Container, Divider, Space, Text, Title};
use aura_core::Config;
use aura_theme::Theme;
use gpui::{App, Context, Entity, IntoElement, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> Entity<ContainerDemo> {
    cx.new(|_| ContainerDemo)
}

pub struct ContainerDemo;

impl Render for ContainerDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        div()
            .flex()
            .flex_col()
            .gap_8()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(hdr(theme, "Space 间距"))
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
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(hdr(theme, "Divider 分割线"))
                    .child(
                        div()
                            .child("Above divider")
                            .child(Divider::new())
                            .child("Below divider"),
                    )
                    .child(Divider::new().label("Center Label"))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_4()
                            .h_6()
                            .child("Section 1")
                            .child(Divider::new().vertical())
                            .child("Section 2")
                            .child(Divider::new().vertical())
                            .child("Section 3"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(hdr(theme, "Container 布局容器"))
                    .child(
                        div()
                            .h(px(300.0))
                            .w_full()
                            .border_1()
                            .border_color(theme.neutral.border)
                            .child(
                                Container::new()
                                    .header(
                                        div()
                                            .flex()
                                            .items_center()
                                            .child(Title::new("Header").h5()),
                                    )
                                    .aside(div().p_4().child(Text::new("Aside Sidebar")))
                                    .footer(div().flex().items_center().child(Text::new("Footer")))
                                    .child(div().p_4().child(Text::new("Main Content Area"))),
                            ),
                    ),
            )
    }
}

fn hdr(theme: &Theme, s: &str) -> impl IntoElement {
    div()
        .text_size(px(theme.font_size.lg))
        .text_color(theme.neutral.text_1)
        .font_weight(gpui::FontWeight::BOLD)
        .child(s.to_string())
}
