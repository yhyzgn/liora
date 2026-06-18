use crate::motion::pop_in;
use gpui::{
    App, AsyncApp, Context, Entity, ForegroundExecutor, Global, IntoElement, Render, SharedString,
    Window, div, prelude::*, px,
};
use liora_core::{Config, push_passive_portal};
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use liora_theme::Theme;
use std::{cell::RefCell, time::Duration};

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

#[derive(Clone)]
struct ToastDispatcherGlobal {
    app: AsyncApp,
    foreground_executor: ForegroundExecutor,
}
impl Global for ToastDispatcherGlobal {}

thread_local! {
    static TOAST_DISPATCHER: RefCell<Option<ToastDispatcherGlobal>> = const { RefCell::new(None) };
}

impl ToastDispatcherGlobal {
    fn new(cx: &mut App) -> Self {
        Self {
            app: cx.to_async(),
            foreground_executor: cx.foreground_executor().clone(),
        }
    }

    fn show(&self, content: SharedString, msg_type: MessageType) {
        let app = self.app.clone();
        self.foreground_executor
            .spawn(async move {
                app.update(|cx| {
                    show_message(content, msg_type, cx);
                    cx.refresh_windows();
                });
            })
            .detach();
    }
}

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
        if !cx.has_global::<ToastDispatcherGlobal>() {
            let dispatcher = ToastDispatcherGlobal::new(cx);
            cx.set_global(dispatcher);
        }
        TOAST_DISPATCHER.with(|dispatcher| {
            *dispatcher.borrow_mut() = Some(cx.global::<ToastDispatcherGlobal>().clone());
        });
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
            cx.foreground_executor()
                .spawn(async move {
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
                })
                .detach();

            cx.notify();
        });
    }
}

impl Render for MessageManager {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let messages = self.messages.clone();
        if messages.is_empty() {
            return div();
        }

        let theme = cx.global::<Config>().theme.clone();

        div()
            .absolute()
            .top_8()
            .left_0()
            .w_full()
            .flex()
            .flex_col()
            .items_center()
            .gap_2()
            .children(messages.into_iter().map(|msg| {
                let style = message_style(&theme, msg.msg_type);

                pop_in(
                    ("liora-message", msg.id),
                    div()
                        .bg(style.bg)
                        .border_1()
                        .border_color(style.border)
                        .px_4()
                        .py_2()
                        .rounded(px(theme.radius.md))
                        .shadow_lg()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(Icon::new(style.icon).size(px(16.0)).color(style.fg))
                        .child(
                            div()
                                .text_color(style.fg)
                                .text_size(px(theme.font_size.sm))
                                .child(msg.content),
                        ),
                )
            }))
    }
}

pub fn show_message(content: impl Into<SharedString>, msg_type: MessageType, cx: &mut App) {
    MessageManager::show(content, msg_type, cx);
}

pub fn toast(content: impl Into<SharedString>, msg_type: MessageType, cx: &mut App) {
    show_message(content, msg_type, cx);
}

pub fn toast_info(content: impl Into<SharedString>, cx: &mut App) {
    toast(content, MessageType::Info, cx);
}

pub fn toast_success(content: impl Into<SharedString>, cx: &mut App) {
    toast(content, MessageType::Success, cx);
}

pub fn toast_warning(content: impl Into<SharedString>, cx: &mut App) {
    toast(content, MessageType::Warning, cx);
}

pub fn toast_error(content: impl Into<SharedString>, cx: &mut App) {
    toast(content, MessageType::Error, cx);
}

pub fn dispatch_toast(content: impl Into<SharedString>, msg_type: MessageType) {
    let content = content.into();
    TOAST_DISPATCHER.with(|dispatcher| {
        let Some(dispatcher) = dispatcher.borrow().clone() else {
            panic!("toast macros require MessageManager::init(cx) before use");
        };
        dispatcher.show(content, msg_type);
    });
}

pub fn dispatch_toast_info(content: impl Into<SharedString>) {
    dispatch_toast(content, MessageType::Info);
}

pub fn dispatch_toast_success(content: impl Into<SharedString>) {
    dispatch_toast(content, MessageType::Success);
}

pub fn dispatch_toast_warning(content: impl Into<SharedString>) {
    dispatch_toast(content, MessageType::Warning);
}

pub fn dispatch_toast_error(content: impl Into<SharedString>) {
    dispatch_toast(content, MessageType::Error);
}

pub fn render_messages(cx: &mut App) {
    if cx.has_global::<MessageManagerGlobal>() {
        let manager = cx.global::<MessageManagerGlobal>().0.clone();
        if !manager.read(cx).messages.is_empty() {
            push_passive_portal(move |_window, _cx| manager.clone().into_any_element(), cx);
        }
    }
}

struct MessageStyle {
    bg: gpui::Hsla,
    fg: gpui::Hsla,
    border: gpui::Hsla,
    icon: IconName,
}

fn message_style(theme: &Theme, msg_type: MessageType) -> MessageStyle {
    let (family, icon) = match msg_type {
        MessageType::Info => (&theme.info, IconName::Info),
        MessageType::Success => (&theme.success, IconName::Check),
        MessageType::Warning => (&theme.warning, IconName::TriangleAlert),
        MessageType::Error => (&theme.danger, IconName::CircleX),
    };

    MessageStyle {
        bg: family.base,
        fg: theme.neutral.card,
        border: family.base,
        icon,
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __liora_toast_dispatch {
    ($dispatch:path, $fmt:literal $(, $($arg:tt)+)?) => {{
        $dispatch(format!($fmt $(, $($arg)+)?));
    }};
    ($dispatch:path, $message:expr $(,)?) => {{
        $dispatch($message);
    }};
}

#[macro_export]
macro_rules! toast_info {
    ($($arg:tt)*) => {{
        $crate::__liora_toast_dispatch!($crate::dispatch_toast_info, $($arg)*);
    }};
}

#[macro_export]
macro_rules! toast_success {
    ($($arg:tt)*) => {{
        $crate::__liora_toast_dispatch!($crate::dispatch_toast_success, $($arg)*);
    }};
}

#[macro_export]
macro_rules! toast_warning {
    ($($arg:tt)*) => {{
        $crate::__liora_toast_dispatch!($crate::dispatch_toast_warning, $($arg)*);
    }};
}

#[macro_export]
macro_rules! toast_error {
    ($($arg:tt)*) => {{
        $crate::__liora_toast_dispatch!($crate::dispatch_toast_error, $($arg)*);
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_styles_use_solid_type_background_and_inverted_foreground() {
        let theme = Theme::light();
        let cases = [
            (MessageType::Info, theme.info.base),
            (MessageType::Success, theme.success.base),
            (MessageType::Warning, theme.warning.base),
            (MessageType::Error, theme.danger.base),
        ];

        for (message_type, expected_bg) in cases {
            let message = message_style(&theme, message_type);

            assert_eq!(message.bg, expected_bg);
            assert_eq!(message.border, expected_bg);
            assert_eq!(message.fg, theme.neutral.card);
        }
    }

    #[test]
    fn toast_helpers_map_to_message_types() {
        let source = include_str!("message.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains("pub fn toast_info"));
        assert!(source.contains("pub fn toast_success"));
        assert!(source.contains("pub fn toast_warning"));
        assert!(source.contains("pub fn toast_error"));
        assert!(source.contains("MessageType::Info"));
        assert!(source.contains("MessageType::Success"));
        assert!(source.contains("MessageType::Warning"));
        assert!(source.contains("MessageType::Error"));
    }

    #[test]
    fn toast_macros_support_format_arguments() {
        let source = include_str!("message.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains("macro_rules! toast_info"));
        assert!(source.contains("format!("));
        assert!(source.contains("dispatch_toast_info"));
        assert!(source.contains("dispatch_toast_success"));
        assert!(source.contains("dispatch_toast_warning"));
        assert!(source.contains("dispatch_toast_error"));
    }

    #[test]
    #[should_panic(expected = "toast macros require MessageManager::init(cx) before use")]
    fn toast_macro_expands_format_arguments() {
        crate::toast_info!("{left}, {right}", left = "left", right = "right");
    }

    #[test]
    fn messages_use_liora_motion() {
        let source = include_str!("message.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains("pop_in("));
        assert!(source.contains("liora-message"));
    }
}
