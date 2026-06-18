//! Checkable virtualized tree snippet used by Liora Docs.

use gpui::{Context, px};
use liora_components::{VirtualizedTree, toast_success};

pub fn build_checkable_virtualized_tree(cx: &mut Context<VirtualizedTree>) -> VirtualizedTree {
    VirtualizedTree::new(crate::virtualized_tree_basic::sample_tree(), cx)
        .height(px(360.0))
        .row_height(px(36.0))
        .show_checkbox(true)
        .multiple(true)
        .default_expanded_keys(vec!["dept-1".into(), "dept-1-team-1".into()])
        .default_selected_keys(vec!["dept-1-team-1-member-3".into()])
        .on_node_click(|id, _window, _cx| {
            toast_success!("VirtualizedTree selected: {}", id);
        })
}
