use aura_components::{TreeSelect, TreeSelectNode};
use gpui::{App, AppContext, Entity};

pub fn single_tree_select(cx: &mut App) -> Entity<TreeSelect> {
    cx.new(|cx| {
        TreeSelect::new(nodes(), cx)
            .placeholder("Choose documentation page")
            .selected(["quick_start"])
    })
}

fn nodes() -> Vec<TreeSelectNode> {
    vec![
        TreeSelectNode::new("guide", "Guide")
            .child(TreeSelectNode::new("quick_start", "Quick Start")),
    ]
}
