use aura_core::{Config, push_portal};
use gpui::{
    prelude::*, px, App, IntoElement, Window,
    div, SharedString, Global, Entity, Context, Render,
};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Clone)]
pub struct MessageItem {
    pub id: usize,
    pub content: SharedString,
    pub msg_type: MessageType,
}

pub struct MessageManager {
    messages: Vec<MessageItem>,
    next_id: usize,
}

pub struct MessageManagerGlobal(pub Entity<MessageManager>);
impl Global for MessageManagerGlobal {}

impl MessageManager {
    pub fn new() -> Self {
        Self {
            messages: vec![],
            next_id: 0,
        }
    }

    pub fn init(cx: &mut App) {
        if !cx.has_global::<MessageManagerGlobal>() {
            let manager = cx.new(|_| Self::new());
            cx.set_global(MessageManagerGlobal(manager));
        }
    }

    pub fn show(content: impl Into<SharedString>, msg_type: MessageType, cx: &mut App) {
        Self::init(cx);
        let manager = cx.global::<MessageManagerGlobal>().0.clone();
        let content = content.into();
        
        manager.update(cx, |this, cx| {
            let id = this.next_id;
            this.messages.push(MessageItem {
                id,
                content: content.clone(),
                msg_type,
            });
            this.next_id += 1;

            let async_cx = cx.to_async();
            let executor = cx.background_executor().clone();
            cx.foreground_executor().spawn(async move {
                executor.timer(Duration::from_secs(3)).await;
                async_cx.update(|cx| {
                    if cx.has_global::<MessageManagerGlobal>() {
                        let manager = cx.global::<MessageManagerGlobal>().0.clone();
                        manager.update(cx, |this, cx| {
                            this.messages.retain(|m| m.id != id);
                            cx.notify();
                        });
                    }
                });
            }).detach();
            
            cx.notify();
        });
    }
}

impl Render for MessageManager {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let messages = self.messages.clone();
        if messages.is_empty() { return div(); }

        let theme = cx.global::<Config>().theme.clone();

        div()
            .absolute()
            .top_8()
            .left_0()
            .w_full()
            .flex().flex_col().items_center().gap_2()
            .children(messages.into_iter().map(|msg| {
                let color = match msg.msg_type {
                    MessageType::Info => theme.primary.base,
                    MessageType::Success => theme.success.base,
                    MessageType::Warning => theme.warning.base,
                    MessageType::Error => theme.danger.base,
                };

                div()
                    .bg(theme.neutral.card)
                    .border_1().border_color(theme.neutral.border)
                    .px_4().py_2()
                    .rounded(px(theme.radius.md))
                    .shadow_lg()
                    .flex().flex_row().items_center().gap_2()
                    .child(div().w_2().h_2().rounded_full().bg(color))
                    .child(msg.content)
            }))
    }
}

pub fn show_message(content: impl Into<SharedString>, msg_type: MessageType, cx: &mut App) {
    MessageManager::show(content, msg_type, cx);
}

pub fn render_messages(cx: &mut App) {
    if cx.has_global::<MessageManagerGlobal>() {
        let manager = cx.global::<MessageManagerGlobal>().0.clone();
        push_portal(move |_window, _cx| {
            manager.clone().into_any_element()
        }, cx);
    }
}
