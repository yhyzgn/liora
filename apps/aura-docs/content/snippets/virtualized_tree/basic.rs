//! Basic virtualized tree snippet used by Aura Docs.

use aura_components::{TreeNode, VirtualizedTree};
use gpui::{Context, px};

pub fn build_virtualized_tree(cx: &mut Context<VirtualizedTree>) -> VirtualizedTree {
    VirtualizedTree::new(sample_tree(), cx)
        .height(px(360.0))
        .row_height(px(36.0))
        .default_expanded_keys(vec!["dept-0".into(), "dept-0-team-0".into()])
}

pub fn sample_tree() -> Vec<TreeNode> {
    (0..24)
        .map(|dept| {
            let mut node = TreeNode::new(
                format!("dept-{dept}"),
                format!("Department {:02}", dept + 1),
            );
            for team in 0..8 {
                let mut team_node = TreeNode::new(
                    format!("dept-{dept}-team-{team}"),
                    format!("Team {:02}-{:02}", dept + 1, team + 1),
                );
                for member in 0..16 {
                    team_node = team_node.child(TreeNode::new(
                        format!("dept-{dept}-team-{team}-member-{member}"),
                        format!("Member {:02}-{:02}-{:02}", dept + 1, team + 1, member + 1),
                    ));
                }
                node = node.child(team_node);
            }
            node
        })
        .collect()
}
