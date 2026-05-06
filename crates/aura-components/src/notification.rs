use aura_core::{Config, push_portal};
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    App, Context, Entity, Global, IntoElement, Render, SharedString, Window, div, prelude::*, px,
};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationType {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Clone)]
pub struct NotificationItem {
    pub id: usize,
    pub title: SharedString,
    pub description: Option<SharedString>,
    pub msg_type: NotificationType,
}

pub struct NotificationManager {
    notifications: Vec<NotificationItem>,
    next_id: usize,
}

pub struct NotificationManagerGlobal(pub Entity<NotificationManager>);
impl Global for NotificationManagerGlobal {}

impl NotificationManager {
    pub fn new() -> Self {
        Self {
            notifications: vec![],
            next_id: 0,
        }
    }

    pub fn init(cx: &mut App) {
        if !cx.has_global::<NotificationManagerGlobal>() {
            let manager = cx.new(|_| Self::new());
            cx.set_global(NotificationManagerGlobal(manager));
        }
    }

    pub fn show(
        title: impl Into<SharedString>,
        description: Option<SharedString>,
        msg_type: NotificationType,
        cx: &mut App,
    ) {
        Self::init(cx);
        let manager = cx.global::<NotificationManagerGlobal>().0.clone();
        let title = title.into();

        manager.update(cx, |this, cx| {
            let id = this.next_id;
            this.notifications.push(NotificationItem {
                id,
                title: title.clone(),
                description: description.clone(),
                msg_type,
            });
            this.next_id += 1;

            let async_cx = cx.to_async();
            let executor = cx.background_executor().clone();
            cx.foreground_executor()
                .spawn(async move {
                    executor.timer(Duration::from_secs(4)).await;
                    async_cx.update(|cx| {
                        if cx.has_global::<NotificationManagerGlobal>() {
                            let manager = cx.global::<NotificationManagerGlobal>().0.clone();
                            manager.update(cx, |this, cx| {
                                this.notifications.retain(|n| n.id != id);
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

impl Render for NotificationManager {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let items = self.notifications.clone();
        if items.is_empty() {
            return div();
        }

        let theme = cx.global::<Config>().theme.clone();

        div()
            .absolute()
            .top_8()
            .right_8()
            .flex()
            .flex_col()
            .items_end()
            .gap_4()
            .children(items.into_iter().map(|item| {
                let (color, icon) = match item.msg_type {
                    NotificationType::Info => (theme.primary.base, IconName::Info),
                    NotificationType::Success => (theme.success.base, IconName::Check),
                    NotificationType::Warning => (theme.warning.base, IconName::TriangleAlert),
                    NotificationType::Error => (theme.danger.base, IconName::CircleX),
                };

                div()
                    .w(px(320.0))
                    .bg(theme.neutral.card)
                    .border_1()
                    .border_color(theme.neutral.border)
                    .p_4()
                    .rounded(px(theme.radius.md))
                    .shadow_lg()
                    .flex()
                    .flex_row()
                    .gap_3()
                    .child(Icon::new(icon).size(px(24.0)).color(color))
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(div().font_weight(gpui::FontWeight::BOLD).child(item.title))
                            .when_some(item.description, |s, d| {
                                s.child(div().text_sm().text_color(theme.neutral.text_3).child(d))
                            }),
                    )
            }))
    }
}

pub fn show_notification(
    title: impl Into<SharedString>,
    description: Option<SharedString>,
    msg_type: NotificationType,
    cx: &mut App,
) {
    NotificationManager::show(title, description, msg_type, cx);
}

pub fn render_notifications(cx: &mut App) {
    if cx.has_global::<NotificationManagerGlobal>() {
        let manager = cx.global::<NotificationManagerGlobal>().0.clone();
        push_portal(move |_window, _cx| manager.clone().into_any_element(), cx);
    }
}
