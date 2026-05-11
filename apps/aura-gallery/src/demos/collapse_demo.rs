use aura_components::{Collapse, Space, Text};
use gpui::{AnyView, App, Context, Entity, Render, Window, prelude::*};

use aura_components::layout_helpers::{page, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| CollapseDemo {
        basic: cx.new(|_| {
            Collapse::new()
                .item("item1", "Consistency", |_, _| {
                    Text::new("Consistent with real life: in line with the process and intuition of real life.")
                })
                .item("item2", "Feedback", |_, _| {
                    Text::new("Operation feedback: enable the users to clearly perceive their operations by style updates.")
                })
        }),
        accordion: cx.new(|_| {
            Collapse::new()
                .accordion()
                .item("item1", "Consistency", |_, _| {
                    Text::new("Consistent with real life: in line with the process and intuition of real life.")
                })
                .item("item2", "Feedback", |_, _| {
                    Text::new("Operation feedback: enable the users to clearly perceive their operations by style updates.")
                })
        }),
    })
    .into()
}

struct CollapseDemo {
    basic: Entity<Collapse>,
    accordion: Entity<Collapse>,
}

impl Render for CollapseDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Collapse 折叠面板",
            "通过折叠面板收纳内容区域。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "基础用法",
                    "多个面板可以独立展开与收起。",
                    self.basic.clone(),
                ))
                .child(section(
                    "手风琴模式 (Accordion)",
                    "同一时间只保留一个面板展开。",
                    self.accordion.clone(),
                )),
        )
    }
}
