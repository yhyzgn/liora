use aura_components::{Button, MessageBox, Space, alert, confirm};
use aura_core::Config;
use gpui::{AnyView, App, Context, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| MessageBoxDemo).into()
}

struct MessageBoxDemo;

impl Render for MessageBoxDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

        div()
            .flex()
            .flex_col()
            .gap_6()
            .child(section(theme, "Basic 基础用法", "模拟系统消息提示框。"))
            .child(
                Space::new()
                    .gap(px(16.0))
                    .child(Button::new("Open Alert").on_click(|_, _, cx| {
                        alert("Alert Title", "This is an alert message.", cx);
                    }))
                    .child(Button::new("Open Confirm").primary().on_click(|_, _, cx| {
                        confirm(
                            "Confirm Title",
                            "Are you sure you want to proceed?",
                            |_, _| println!("Confirmed in MB!"),
                            cx,
                        );
                    })),
            )
            .child(section(
                theme,
                "Close strategy 关闭策略",
                "可禁用空白处和 ESC 关闭。",
            ))
            .child(
                Space::new()
                    .gap(px(16.0))
                    .child(Button::new("Manual Alert").warning().on_click(|_, _, cx| {
                        MessageBox::new(
                            "Manual Alert",
                            "Only the OK button can close this message box.",
                        )
                        .close_on_click_outside(false)
                        .close_on_escape(false)
                        .alert(cx);
                    }))
                    .child(Button::new("Manual Confirm").danger().on_click(|_, _, cx| {
                        MessageBox::new(
                            "Manual Confirm",
                            "Only Cancel or Confirm can close this message box.",
                        )
                        .close_on_click_outside(false)
                        .close_on_escape(false)
                        .confirm(|_, _| println!("Manual confirm accepted"), cx);
                    })),
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
