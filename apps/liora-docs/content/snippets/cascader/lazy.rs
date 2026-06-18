//! Lazily attach children when a branch is opened.

use gpui::{Context, SharedString};
use liora_components::{Cascader, CascaderOption};

pub fn lazy_cascader(cx: &mut Context<Cascader>) -> Cascader {
    Cascader::new(lazy_roots(), cx)
        .lazy(true)
        .placeholder("请选择远程节点")
        .width_md()
        .on_lazy_load(|cascader, path, _window, cx| {
            cascader.set_children_at_path(&path, lazy_children_for(&path), cx);
        })
}

fn lazy_roots() -> Vec<CascaderOption> {
    vec![
        CascaderOption::new("remote-a", "远程分组 A"),
        CascaderOption::new("remote-b", "远程分组 B"),
        CascaderOption::new("ready", "本地叶子").leaf(true),
    ]
}

fn lazy_children_for(path: &[SharedString]) -> Vec<CascaderOption> {
    match path
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("/")
        .as_str()
    {
        "remote-a" => vec![
            CascaderOption::new("team", "团队")
                .child(CascaderOption::new("design", "设计组").leaf(true)),
        ],
        "remote-b" => vec![CascaderOption::new("north", "华北").leaf(true)],
        _ => vec![CascaderOption::new("loaded", "加载结果").leaf(true)],
    }
}
