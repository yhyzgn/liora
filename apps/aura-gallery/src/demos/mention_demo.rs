use aura_components::layout_helpers::{page, section};
use aura_components::{Card, Mention, MentionItem, Space, toast_info};
use gpui::{AnyView, App, Context, Entity, Render, Window, prelude::*};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| MentionDemo {
        people: cx.new(|cx| {
            Mention::new(people(), cx)
                .placeholder("Type @ to mention a teammate")
                .on_select(|item, _, _| toast_info!("Mention {}", item.label))
        }),
        issue: cx.new(|cx| {
            Mention::new(issues(), cx)
                .trigger('#')
                .placeholder("Type # to reference an issue")
                .max_suggestions(4)
        }),
        disabled: cx.new(|cx| {
            Mention::new(people(), cx)
                .placeholder("Disabled mention")
                .disabled(true)
        }),
    })
    .into()
}

struct MentionDemo {
    people: Entity<Mention>,
    issue: Entity<Mention>,
    disabled: Entity<Mention>,
}

fn people() -> Vec<MentionItem> {
    vec![
        MentionItem::new("alice", "Alice Chen").description("Design systems"),
        MentionItem::new("bob", "Bob Smith").description("Release engineering"),
        MentionItem::new("carol", "Carol Li").description("Docs and examples"),
        MentionItem::new("dora", "Dora Wang").description("Quality assurance"),
    ]
}

fn issues() -> Vec<MentionItem> {
    vec![
        MentionItem::new("128", "#128 Improve chart hover"),
        MentionItem::new("142", "#142 Package smoke scripts"),
        MentionItem::new("176", "#176 Add TreeSelect"),
        MentionItem::new("201", "#201 Polish docs navigation"),
    ]
}

impl Render for MentionDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Mention 提及",
            "输入触发符后展示候选项，用于 @成员、#事项或命令引用。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "@ 成员提及",
                    "基于 Input 输入内核和候选列表过滤。",
                    Card::new(self.people.clone()),
                ))
                .child(section(
                    "自定义触发符",
                    "trigger('#') 可用于 issue、任务或频道引用。",
                    Card::new(self.issue.clone()),
                ))
                .child(section(
                    "禁用状态",
                    "disabled(true) 保留布局但禁用输入和候选。",
                    Card::new(self.disabled.clone()),
                )),
        )
    }
}
