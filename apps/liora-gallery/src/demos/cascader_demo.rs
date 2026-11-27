use gpui::{AnyView, App, Context, Entity, Render, Window, prelude::*};
use liora_components::{Card, Cascader, Space, Text};

use liora_components::layout_helpers::{page, section};

#[path = "../../../liora-docs/content/snippets/cascader/basic.rs"]
mod cascader_basic_example;
#[path = "../../../liora-docs/content/snippets/cascader/disabled.rs"]
mod cascader_disabled_example;
#[path = "../../../liora-docs/content/snippets/cascader/filterable.rs"]
mod cascader_filterable_example;
#[path = "../../../liora-docs/content/snippets/cascader/lazy.rs"]
mod cascader_lazy_example;
#[path = "../../../liora-docs/content/snippets/cascader/selected.rs"]
mod cascader_selected_example;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| CascaderDemo::new(cx)).into()
}

struct CascaderDemo {
    basic: Entity<Cascader>,
    selected: Entity<Cascader>,
    disabled: Entity<Cascader>,
    searchable: Entity<Cascader>,
    lazy: Entity<Cascader>,
}

impl CascaderDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            basic: cx.new(cascader_basic_example::basic_cascader),
            selected: cx.new(cascader_selected_example::selected_cascader),
            disabled: cx.new(cascader_disabled_example::disabled_cascader),
            searchable: cx.new(cascader_filterable_example::filterable_cascader),
            lazy: cx.new(cascader_lazy_example::lazy_cascader),
        }
    }
}

impl Render for CascaderDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Cascader 级联选择器",
            "从一组相关联的数据集合中逐级选择，支持默认选中、禁用、清空、搜索结果面板和懒加载。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "基础用法",
                    "点击分支展开、点击叶子选择；聚焦后 Enter / Space 展开，Escape 关闭。",
                    Card::new(
                        Space::new()
                            .vertical()
                            .gap_md()
                            .child(self.basic.clone())
                            .child(Text::new("选择后显示实际路径消息；点击清空显示 Selection cleared。")),
                    ),
                ))
                .child(section("默认选中", "预置已选择的路径。", Card::new(self.selected.clone())))
                .child(section("禁用状态", "禁用后不可展开或修改。", Card::new(self.disabled.clone())))
                .child(section(
                    "可搜索",
                    "展开后编辑搜索词；预置杭州，可改为宁波，清空恢复逐级选项。",
                    Card::new(
                        Space::new()
                            .vertical()
                            .gap_md()
                            .child(self.searchable.clone())
                            .child(Text::new("无匹配显示 No results；Down / Up 高亮结果且边界不循环，Enter 选择高亮路径。无高亮时 Enter 不提交；逐级选项仍使用鼠标。")),
                    ),
                ))
                .child(section(
                    "懒加载",
                    "同步演示数据：点击空子级分支后，回调立即写回固定子节点，无网络请求。",
                    Card::new(
                        Space::new()
                            .vertical()
                            .gap_md()
                            .child(self.lazy.clone())
                            .child(Text::new("on_lazy_load 通过 set_children_at_path 同步写回固定数据；点击 leaf(true) 节点选择并显示实际路径消息。")),
                    ),
                )),
        )
    }
}
