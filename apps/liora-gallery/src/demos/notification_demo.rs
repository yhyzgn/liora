use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::layout_helpers::page;

#[path = "../../../liora-docs/content/snippets/notification/types.rs"]
mod notification_types_example;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| NotificationDemo).into()
}

struct NotificationDemo;

impl Render for NotificationDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Notification 通知",
            "全局通知在未悬停时显示 4 秒；悬停保留剩余时间，移开后继续，也可单独关闭。",
            notification_types_example::notification_types(),
        )
    }
}
