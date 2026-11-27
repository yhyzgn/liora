//! Disable all Cascader interaction while keeping the current value visible.

use gpui::Context;
use liora_components::{Cascader, CascaderOption};

pub fn disabled_cascader(cx: &mut Context<Cascader>) -> Cascader {
    Cascader::new(region_options(), cx)
        .disabled(true)
        .selected_path(["zhejiang", "hangzhou", "xihu"])
        .width_md()
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
