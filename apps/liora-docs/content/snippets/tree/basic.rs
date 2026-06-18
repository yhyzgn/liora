//! Basic Tree hierarchy.

use liora_components::{Tree, TreeNode};

pub fn basic_tree() -> Tree {
    Tree::new(vec![
        TreeNode::new("1", "一级 1")
            .child(TreeNode::new("1-1", "二级 1-1").child(TreeNode::new("1-1-1", "三级 1-1-1")))
            .child(TreeNode::new("1-2", "二级 1-2")),
        TreeNode::new("2", "一级 2").child(TreeNode::new("2-1", "二级 2-1")),
    ])
}
