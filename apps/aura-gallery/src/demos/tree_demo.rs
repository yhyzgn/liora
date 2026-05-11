use aura_components::{Card, Tree, TreeNode};
use gpui::{AnyView, App, Context, Entity, Render, Window, prelude::*};

use aura_components::layout_helpers::{page, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| TreeDemo {
        tree: cx.new(|_| {
            Tree::new(vec![
                TreeNode::new("1", "一级 1")
                    .child(
                        TreeNode::new("1-1", "二级 1-1")
                            .child(TreeNode::new("1-1-1", "三级 1-1-1"))
                            .child(TreeNode::new("1-1-2", "三级 1-1-2")),
                    )
                    .child(TreeNode::new("1-2", "二级 1-2")),
                TreeNode::new("2", "一级 2")
                    .child(TreeNode::new("2-1", "二级 2-1"))
                    .child(TreeNode::new("2-2", "二级 2-2")),
                TreeNode::new("3", "一级 3")
                    .child(TreeNode::new("3-1", "二级 3-1"))
                    .child(TreeNode::new("3-2", "二级 3-2")),
            ])
        }),
    })
    .into()
}

struct TreeDemo {
    tree: Entity<Tree>,
}

impl Render for TreeDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Tree 树形控件",
            "用清晰的层级结构展示信息，可展开或折叠。",
            section(
                "基础用法",
                "点击节点可展开或选中。",
                Card::new(self.tree.clone()),
            ),
        )
    }
}
