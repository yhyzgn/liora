use gpui::{App, AppContext, Entity};
use liora_components::{Mention, MentionItem};

pub fn issue_mention(cx: &mut App) -> Entity<Mention> {
    cx.new(|cx| {
        Mention::new(
            vec![MentionItem::new("128", "#128 Improve chart hover")],
            cx,
        )
        .trigger('#')
        .placeholder("Type # to reference an issue")
        .max_suggestions(4)
    })
}
