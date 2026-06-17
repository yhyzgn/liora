use aura_components::layout_helpers::{page, section};
use aura_components::{Card, Space, TreeSelect, TreeSelectNode, toast_success};
use gpui::{AnyView, App, Context, Entity, Render, Window, prelude::*};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| TreeSelectDemo {
        single: cx.new(|cx| {
            TreeSelect::new(nodes(), cx)
                .placeholder("Choose documentation page")
                .selected(["quick_start"])
                .on_change(|keys, _, _| toast_success!("selected {:?}", keys))
        }),
        multiple: cx.new(|cx| {
            TreeSelect::new(nodes(), cx)
                .multiple(true)
                .selected(["button", "line_chart"])
                .disabled_keys(["internal"])
                .placeholder("Choose multiple modules")
        }),
        filterable: cx.new(|cx| {
            TreeSelect::new(nodes(), cx)
                .filterable(true)
                .placeholder("Search docs tree")
        }),
    })
    .into()
}

struct TreeSelectDemo {
    single: Entity<TreeSelect>,
    multiple: Entity<TreeSelect>,
    filterable: Entity<TreeSelect>,
}

fn nodes() -> Vec<TreeSelectNode> {
    vec![
        TreeSelectNode::new("guide", "Guide")
            .child(TreeSelectNode::new("overview", "Overview"))
            .child(TreeSelectNode::new("quick_start", "Quick Start"))
            .child(TreeSelectNode::new("internal", "Internal Draft")),
        TreeSelectNode::new("components", "Components")
            .child(TreeSelectNode::new("button", "Button"))
            .child(TreeSelectNode::new("input", "Input"))
            .child(TreeSelectNode::new("tree_select", "TreeSelect")),
        TreeSelectNode::new("charts", "Charts")
            .child(TreeSelectNode::new("line_chart", "LineChart"))
            .child(TreeSelectNode::new("ring_chart", "RingChart")),
    ]
}

impl Render for TreeSelectDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "TreeSelect 树形选择",
            "在选择器面板中展示层级数据，支持单选、多选、搜索过滤和禁用节点。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "单选",
                    "选择后自动收起面板。",
                    Card::new(self.single.clone()),
                ))
                .child(section(
                    "多选与禁用节点",
                    "multiple(true) 允许勾选多个树节点，disabled_keys 禁止选择。",
                    Card::new(self.multiple.clone()),
                ))
                .child(section(
                    "搜索过滤",
                    "filterable(true) 在面板顶部显示搜索框。",
                    Card::new(self.filterable.clone()),
                )),
        )
    }
}
