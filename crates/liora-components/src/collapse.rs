use crate::motion::pop_in;
use gpui::{AnyElement, Context, IntoElement, Render, SharedString, Window, div, prelude::*, px};
use liora_core::{Config, unique_id};
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::collections::HashSet;
use std::sync::Arc;

pub struct CollapseItem {
    pub name: SharedString,
    pub title: SharedString,
    pub content: Arc<dyn Fn(&mut Window, &mut Context<Collapse>) -> AnyElement + 'static>,
}

pub struct Collapse {
    items: Vec<CollapseItem>,
    active_names: HashSet<SharedString>,
    accordion: bool,
    id: SharedString,
}

impl Collapse {
    pub fn new() -> Self {
        Self {
            items: vec![],
            active_names: HashSet::new(),
            accordion: false,
            id: unique_id("collapse"),
        }
    }

    pub fn accordion(mut self) -> Self {
        self.accordion = true;
        self
    }

    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    pub fn item<F, E>(
        mut self,
        name: impl Into<SharedString>,
        title: impl Into<SharedString>,
        f: F,
    ) -> Self
    where
        F: Fn(&mut Window, &mut Context<Self>) -> E + 'static,
        E: IntoElement,
    {
        self.items.push(CollapseItem {
            name: name.into(),
            title: title.into(),
            content: Arc::new(move |window, cx| f(window, cx).into_any_element()),
        });
        self
    }

    fn toggle(&mut self, name: SharedString, cx: &mut Context<Self>) {
        if self.active_names.contains(&name) {
            self.active_names.remove(&name);
        } else {
            if self.accordion {
                self.active_names.clear();
            }
            self.active_names.insert(name);
        }
        cx.notify();
    }
}

impl Render for Collapse {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        div()
            .flex()
            .flex_col()
            .border_1()
            .border_color(theme.neutral.border)
            .rounded(px(theme.radius.md))
            .children(self.items.iter().enumerate().map(|(i, item)| {
                let name = item.name.clone();
                let is_active = self.active_names.contains(&name);
                let is_last = i == self.items.len() - 1;
                let header_id = format!("{}-header-{}", self.id, name);
                let content_motion_id = format!("{}-content-motion-{}", self.id, name);

                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .id(header_id)
                            .cursor_pointer()
                            .px_4()
                            .py_3()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .bg(if is_active {
                                theme.neutral.hover
                            } else {
                                theme.neutral.card
                            })
                            .hover(|s| s.bg(theme.neutral.hover))
                            .when(!is_last, |s| {
                                s.border_b_1().border_color(theme.neutral.border)
                            })
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.toggle(name.clone(), cx);
                            }))
                            .child(
                                div()
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .child(item.title.clone()),
                            )
                            .child(
                                Icon::new(if is_active {
                                    IconName::ChevronDown
                                } else {
                                    IconName::ChevronRight
                                })
                                .size(px(16.0))
                                .color(theme.neutral.icon),
                            ),
                    )
                    .when(is_active, |s| {
                        s.child(pop_in(
                            content_motion_id,
                            div()
                                .p_4()
                                .bg(theme.neutral.card)
                                .when(!is_last, |s| {
                                    s.border_b_1().border_color(theme.neutral.border)
                                })
                                .child((item.content)(_window, cx)),
                        ))
                    })
            }))
    }
}
