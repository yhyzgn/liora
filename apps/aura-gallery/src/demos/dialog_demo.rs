use aura_components::{Button, Dialog, Space};
use aura_core::Config;
use gpui::{prelude::*, px, AnyView, App, Context, Render, Window, div};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| DialogDemo).into()
}

struct DialogDemo;

impl Render for DialogDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

        div().flex().flex_col().gap_6()
            .child(section(theme, "Basic 基础用法", "保留当前页面状态时展示重要信息。"))
            .child(
                Space::new().gap(px(16.0))
                    .child(Button::new("Open Dialog").primary().on_click(|_, _, cx| {
                        Dialog::new()
                            .title("Tips")
                            .content(|_, _| dialog_body("This is a message from the dialog."))
                            .show(cx);
                    }))
                    .child(Button::new("Manual Close Only").warning().on_click(|_, _, cx| {
                        Dialog::new()
                            .title("Manual close dialog")
                            .close_on_click_outside(false)
                            .close_on_escape(false)
                            .content(|_, _| {
                                div().flex().flex_col().gap_4()
                                    .child("点击遮罩和按 ESC 都不会关闭，只能点击按钮手动关闭。")
                                    .child(div().flex().justify_end().child(
                                        Button::new("I understand").primary().on_click(|_, _, cx| Dialog::close(cx))
                                    ))
                            })
                            .show(cx);
                    }))
            )
            .child(section(theme, "Content 内容", "内容区域可放置任意组件。"))
            .child(
                Space::new().gap(px(16.0))
                    .child(Button::new("Form-like Content").on_click(|_, _, cx| {
                        Dialog::new()
                            .title("Edit profile")
                            .content(|_, _| {
                                div().flex().flex_col().gap_3()
                                    .child(div().child("Name: Aura User"))
                                    .child(div().child("Role: Designer"))
                                    .child(div().flex().justify_end().gap_2()
                                        .child(Button::new("Cancel").on_click(|_, _, cx| Dialog::close(cx)))
                                        .child(Button::new("Save").primary().on_click(|_, _, cx| Dialog::close(cx))))
                            })
                            .show(cx);
                    }))
            )
    }
}

fn section(theme: &aura_theme::Theme, title: &'static str, desc: &'static str) -> impl IntoElement {
    div().flex().flex_col().gap_1()
        .child(div().text_lg().font_weight(gpui::FontWeight::BOLD).child(title))
        .child(div().text_sm().text_color(theme.neutral.text_3).child(desc))
}

fn dialog_body(message: &'static str) -> impl IntoElement {
    div().flex().flex_col().gap_4()
        .child(message)
        .child(div().flex().justify_end().child(
            Button::new("Close").primary().on_click(|_, _, cx| Dialog::close(cx))
        ))
}
