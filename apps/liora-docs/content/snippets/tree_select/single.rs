use gpui::{App, AppContext, Entity};
use liora_components::{TreeSelect, TreeSelectNode};

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
