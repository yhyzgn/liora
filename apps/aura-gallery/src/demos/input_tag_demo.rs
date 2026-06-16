use aura_components::layout_helpers::{page, section};
use aura_components::{Card, InputTag, Space, toast_success};
use gpui::{AnyView, App, Context, Entity, Render, Window, prelude::*};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| InputTagDemo {
        basic: cx
            .new(|cx| InputTag::new(vec!["Rust", "GPUI", "Aura"], cx).placeholder("Add skill")),
        limited: cx.new(|cx| {
            InputTag::new(vec!["Design", "Docs"], cx)
                .placeholder("Max 4")
                .max_tags(4)
                .on_change(|tags, _, _| toast_success!("{} tags", tags.len()))
        }),
        duplicates: cx.new(|cx| {
            InputTag::new(vec!["blue", "blue"], cx)
                .allow_duplicates(true)
                .placeholder("Duplicates allowed")
        }),
    })
    .into()
}

struct InputTagDemo {
    basic: Entity<InputTag>,
    limited: Entity<InputTag>,
    duplicates: Entity<InputTag>,
}

impl Render for InputTagDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "InputTag 标签输入",
            "在输入框中回车生成标签，支持删除、最大数量和重复项策略。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "基础用法",
                    "输入文本并按 Enter 添加标签。",
                    Card::new(self.basic.clone()),
                ))
                .child(section(
                    "数量限制",
                    "max_tags 达到上限后会禁用输入。",
                    Card::new(self.limited.clone()),
                ))
                .child(section(
                    "允许重复",
                    "allow_duplicates(true) 适用于搜索 token 或原始标签记录。",
                    Card::new(self.duplicates.clone()),
                )),
        )
    }
}
