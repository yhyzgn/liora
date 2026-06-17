use aura_components::{TreeSelect, TreeSelectNode};
use gpui::{App, AppContext, Entity};

pub fn filterable_tree_select(cx: &mut App) -> Entity<TreeSelect> {
    cx.new(|cx| {
        TreeSelect::new(nodes(), cx)
            .filterable(true)
            .placeholder("Search docs tree")
    })
}

fn nodes() -> Vec<TreeSelectNode> {
    vec![
        TreeSelectNode::new("guide", "Guide").child(TreeSelectNode::new("overview", "Overview")),
        TreeSelectNode::new("charts", "Charts")
            .child(TreeSelectNode::new("ring_chart", "RingChart")),
    ]
}
