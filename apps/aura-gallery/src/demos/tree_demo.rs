use aura_components::{Tree, TreeNode, Card};
use aura_core::{Config};
use gpui::{prelude::*, App, Context, Render, Window, div, AnyView};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| TreeDemo).into()
}

struct TreeDemo;

impl Render for TreeDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        
        div().flex().flex_col().gap_8().p_4()
            .child(
                div().flex().flex_col().gap_2()
                    .child(div().text_lg().font_weight(gpui::FontWeight::BOLD).child("Tree 树形控件"))
                    .child(div().text_sm().text_color(theme.neutral.text_3).child("用清晰的层级结构展示信息，可展开或折叠。"))
            )
            .child(
                div().flex().flex_col().gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("基础用法"))
                    .child(
                        Card::new(
                            cx.new(|_| {
                                Tree::new(vec![
                                    TreeNode::new("1", "一级 1")
                                        .child(TreeNode::new("1-1", "二级 1-1")
                                            .child(TreeNode::new("1-1-1", "三级 1-1-1"))
                                            .child(TreeNode::new("1-1-2", "三级 1-1-2"))
                                        )
                                        .child(TreeNode::new("1-2", "二级 1-2")),
                                    TreeNode::new("2", "一级 2")
                                        .child(TreeNode::new("2-1", "二级 2-1"))
                                        .child(TreeNode::new("2-2", "二级 2-2")),
                                    TreeNode::new("3", "一级 3")
                                        .child(TreeNode::new("3-1", "二级 3-1"))
                                        .child(TreeNode::new("3-2", "二级 3-2")),
                                ])
                            })
                        )
                    )
            )
    }
}
