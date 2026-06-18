use gpui::{App, AppContext, Entity};
use liora_components::{TreeSelect, TreeSelectNode};

pub fn multiple_tree_select(cx: &mut App) -> Entity<TreeSelect> {
    cx.new(|cx| {
        TreeSelect::new(nodes(), cx)
            .multiple(true)
            .selected(["button", "line_chart"])
            .disabled_keys(["internal"])
    })
}

fn nodes() -> Vec<TreeSelectNode> {
    vec![
        TreeSelectNode::new("components", "Components")
            .child(TreeSelectNode::new("button", "Button"))
            .child(TreeSelectNode::new("internal", "Internal Draft")),
        TreeSelectNode::new("charts", "Charts")
            .child(TreeSelectNode::new("line_chart", "LineChart")),
    ]
}
