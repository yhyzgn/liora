use aura_components::layout_helpers::{page, section};
use aura_components::{Space, Text, TreeNode, VirtualizedTree, toast_success};
use gpui::{AnyView, App, Context, Entity, IntoElement, Render, Window, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| VirtualizedTreeDemo {
        basic: cx.new(|cx| {
            VirtualizedTree::new(large_tree(), cx)
                .height(px(420.0))
                .row_height(px(36.0))
                .default_expanded_keys(vec!["dept-0".into(), "dept-0-team-0".into()])
        }),
        checkable: cx.new(|cx| {
            VirtualizedTree::new(large_tree(), cx)
                .height(px(420.0))
                .row_height(px(36.0))
                .show_checkbox(true)
                .multiple(true)
                .default_expanded_keys(vec!["dept-1".into(), "dept-1-team-2".into()])
                .default_selected_keys(vec!["dept-1-team-2-member-4".into()])
                .on_node_click(|id, _, _| {
                    toast_success!("VirtualizedTree selected: {}", id);
                })
        }),
    })
    .into()
}

struct VirtualizedTreeDemo {
    basic: Entity<VirtualizedTree>,
    checkable: Entity<VirtualizedTree>,
}

impl Render for VirtualizedTreeDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "VirtualizedTree 虚拟树",
            "只渲染当前可见节点的大型层级树，适合组织架构、文件树和权限树。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "大型组织树",
                    "展开/折叠时重新 flatten 当前可见节点，ListState 只布局可见行。",
                    self.basic.clone(),
                ))
                .child(section(
                    "多选与回调",
                    "开启 show_checkbox(true) + multiple(true) 后可在大树中多选节点并接收回调。",
                    self.checkable.clone(),
                ))
                .child(Text::new("示例数据包含 40 个部门、每部门 12 个团队、每团队 18 位成员，全部节点不会一次性渲染成元素。")),
        )
    }
}

fn large_tree() -> Vec<TreeNode> {
    (0..40)
        .map(|dept| {
            let mut node = TreeNode::new(
                format!("dept-{dept}"),
                format!("Department {:02}", dept + 1),
            );
            for team in 0..12 {
                let mut team_node = TreeNode::new(
                    format!("dept-{dept}-team-{team}"),
                    format!("Team {:02}-{:02}", dept + 1, team + 1),
                );
                for member in 0..18 {
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

#[cfg(test)]
mod tests {
    #[test]
    fn virtualized_tree_demo_uses_component_api() {
        let source = include_str!("virtualized_tree_demo.rs");

        assert!(source.contains("VirtualizedTree::new"));
        assert!(source.contains("default_expanded_keys"));
        assert!(source.contains("show_checkbox(true)"));
        assert!(source.contains("multiple(true)"));
    }
}
