use aura_components::{Mention, MentionItem};
use gpui::{App, AppContext, Entity};

pub fn disabled_mention(cx: &mut App) -> Entity<Mention> {
    cx.new(|cx| {
        Mention::new(vec![MentionItem::new("alice", "Alice Chen")], cx)
            .placeholder("Disabled mention")
            .disabled(true)
    })
}
