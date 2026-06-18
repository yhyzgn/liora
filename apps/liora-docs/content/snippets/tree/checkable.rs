//! Tree with checkboxes and multiple selection.

use liora_components::{Tree, TreeNode, toast_info};

pub fn checkable_tree() -> Tree {
    Tree::new(vec![
        TreeNode::new("docs", "docs").child(TreeNode::new("quick-start", "quick_start.md")),
        TreeNode::new("src", "src").child(TreeNode::new("components", "liora-components")),
    ])
    .show_checkbox(true)
    .multiple(true)
    .on_node_click(|id, _, _| toast_info!("selected node: {}", id))
}
