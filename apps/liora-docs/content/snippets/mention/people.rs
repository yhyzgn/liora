use gpui::{App, AppContext, Entity};
use liora_components::{Mention, MentionItem};

pub fn people_mention(cx: &mut App) -> Entity<Mention> {
    cx.new(|cx| {
        Mention::new(
            vec![
                MentionItem::new("alice", "Alice Chen").description("Design systems"),
                MentionItem::new("bob", "Bob Smith").description("Release engineering"),
            ],
            cx,
        )
        .placeholder("Type @ to mention a teammate")
    })
}
