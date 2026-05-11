use aura_components::{Card, Cascader, CascaderOption, Space, Text};
use gpui::{AnyView, App, Context, Entity, Render, Window, prelude::*};

use aura_components::layout_helpers::{page, section};

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
            basic: cx.new(|cx| {
                Cascader::new(region_options(), cx)
                    .placeholder("请选择地区")
                    .clearable(true)
                    .width_md()
            }),
            selected: cx.new(|cx| {
                Cascader::new(product_options(), cx)
                    .selected_path(["cloud", "compute", "ecs"])
                    .placeholder("请选择产品")
                    .width_md()
            }),
            disabled: cx.new(|cx| {
                Cascader::new(region_options(), cx)
                    .disabled(true)
                    .selected_path(["zhejiang", "hangzhou", "xihu"])
                    .width_md()
            }),
            searchable: cx.new(|cx| {
                Cascader::new(region_options(), cx)
                    .filterable(true)
                    .search_query("hang")
                    .placeholder("搜索 hang")
                    .width_md()
            }),
            lazy: cx.new(|cx| {
                Cascader::new(lazy_options(), cx)
                    .lazy(true)
                    .placeholder("请选择远程节点")
                    .width_md()
                    .on_lazy_load(|cascader, path, _, cx| {
                        let children = lazy_children_for(&path);
                        cascader.set_children_at_path(&path, children, cx);
                    })
            }),
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
                    "点击含子级的选项会展开下一列，点击叶子节点完成选择。",
                    Card::new(
                        Space::new()
                            .vertical()
                            .gap_md()
                            .child(self.basic.clone())
                            .child(Text::new("点击含子级的选项会展开下一列，点击叶子节点完成选择。")),
                    ),
                ))
                .child(section("默认选中", "预置已选择的路径。", Card::new(self.selected.clone())))
                .child(section("禁用状态", "禁用后不可展开或修改。", Card::new(self.disabled.clone())))
                .child(section(
                    "可搜索",
                    "示例预置 search_query=\"hang\" 展示叶子路径匹配结果。",
                    Card::new(
                        Space::new()
                            .vertical()
                            .gap_md()
                            .child(self.searchable.clone())
                            .child(Text::new("示例预置 search_query=\"hang\" 展示叶子路径匹配结果。")),
                    ),
                ))
                .child(section(
                    "懒加载",
                    "点击空子级的分支会触发 on_lazy_load，回调内写回远程子节点。",
                    Card::new(
                        Space::new()
                            .vertical()
                            .gap_md()
                            .child(self.lazy.clone())
                            .child(Text::new("点击空子级的分支会触发 on_lazy_load，回调内通过 set_children_at_path 写回远程子节点；点击最终 leaf(true) 节点才会选择并关闭。")),
                    ),
                )),
        )
    }
}

fn region_options() -> Vec<CascaderOption> {
    vec![
        CascaderOption::new("zhejiang", "浙江")
            .child(
                CascaderOption::new("hangzhou", "杭州")
                    .child(CascaderOption::new("xihu", "西湖区"))
                    .child(CascaderOption::new("yuhang", "余杭区")),
            )
            .child(
                CascaderOption::new("ningbo", "宁波")
                    .child(CascaderOption::new("haishu", "海曙区"))
                    .child(CascaderOption::new("jiangbei", "江北区")),
            ),
        CascaderOption::new("jiangsu", "江苏")
            .child(
                CascaderOption::new("nanjing", "南京")
                    .child(CascaderOption::new("xuanwu", "玄武区"))
                    .child(CascaderOption::new("gulou", "鼓楼区")),
            )
            .child(
                CascaderOption::new("suzhou", "苏州")
                    .child(CascaderOption::new("gusu", "姑苏区"))
                    .child(CascaderOption::new("wuzhong", "吴中区").disabled(true)),
            ),
        CascaderOption::new("loading", "动态加载中").loading(true),
    ]
}

fn product_options() -> Vec<CascaderOption> {
    vec![
        CascaderOption::new("cloud", "云产品")
            .child(
                CascaderOption::new("compute", "计算")
                    .child(CascaderOption::new("ecs", "云服务器 ECS"))
                    .child(CascaderOption::new("fc", "函数计算")),
            )
            .child(
                CascaderOption::new("storage", "存储")
                    .child(CascaderOption::new("oss", "对象存储 OSS"))
                    .child(CascaderOption::new("nas", "文件存储 NAS")),
            ),
        CascaderOption::new("data", "数据服务").child(
            CascaderOption::new("database", "数据库")
                .child(CascaderOption::new("mysql", "云数据库 MySQL"))
                .child(CascaderOption::new("redis", "Redis")),
        ),
    ]
}

fn lazy_options() -> Vec<CascaderOption> {
    vec![
        CascaderOption::new("remote-a", "远程分组 A"),
        CascaderOption::new("remote-b", "远程分组 B"),
        CascaderOption::new("ready", "本地叶子").leaf(true),
    ]
}

fn lazy_children_for(path: &[gpui::SharedString]) -> Vec<CascaderOption> {
    let key = path
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("/");

    match key.as_str() {
        "remote-a" => vec![
            CascaderOption::new("team", "团队")
                .child(CascaderOption::new("design", "设计组").leaf(true)),
            CascaderOption::new("project", "项目")
                .child(CascaderOption::new("aura", "Aura UI").leaf(true)),
        ],
        "remote-b" => vec![
            CascaderOption::new("north", "华北").leaf(true),
            CascaderOption::new("south", "华南").leaf(true),
        ],
        "remote-a/team" => vec![
            CascaderOption::new("frontend", "前端组").leaf(true),
            CascaderOption::new("native", "Native 组").leaf(true),
        ],
        _ => vec![CascaderOption::new("loaded", "加载结果").leaf(true)],
    }
}
