use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::{Button, MessageBox, alert, confirm};

use liora_components::layout_helpers::{row, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| MessageBoxDemo).into()
}

struct MessageBoxDemo;

impl Render for MessageBoxDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        section(
            "Basic 基础用法",
            "模拟系统消息提示框。",
            row(vec![
                Button::new("Open Alert").on_click(|_, _, cx| {
                    alert("Alert Title", "This is an alert message.", cx);
                }),
                Button::new("Open Confirm").primary().on_click(|_, _, cx| {
                    confirm(
                        "Confirm Title",
                        "Are you sure you want to proceed?",
                        |_, _| println!("Confirmed in MB!"),
                        cx,
                    );
                }),
                Button::new("Manual Alert").warning().on_click(|_, _, cx| {
                    MessageBox::new(
                        "Manual Alert",
                        "Only the OK button can close this message box.",
                    )
                    .close_on_click_outside(false)
                    .close_on_escape(false)
                    .alert(cx);
                }),
                Button::new("Manual Confirm").danger().on_click(|_, _, cx| {
                    MessageBox::new(
                        "Manual Confirm",
                        "Only Cancel or Confirm can close this message box.",
                    )
                    .close_on_click_outside(false)
                    .close_on_escape(false)
                    .confirm(|_, _| println!("Manual confirm accepted"), cx);
                }),
            ]),
        )
    }
}
