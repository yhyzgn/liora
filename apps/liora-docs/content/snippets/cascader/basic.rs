//! Basic Cascader with clearable selection.

use gpui::Context;
use liora_components::{Cascader, CascaderOption};

pub fn basic_cascader(cx: &mut Context<Cascader>) -> Cascader {
    Cascader::new(region_options(), cx)
        .placeholder("请选择地区")
        .clearable(true)
        .width_md()
}

fn region_options() -> Vec<CascaderOption> {
    vec![
        CascaderOption::new("zhejiang", "浙江").child(
            CascaderOption::new("hangzhou", "杭州")
                .child(CascaderOption::new("xihu", "西湖区"))
                .child(CascaderOption::new("yuhang", "余杭区")),
        ),
        CascaderOption::new("jiangsu", "江苏").child(
            CascaderOption::new("suzhou", "苏州")
                .child(CascaderOption::new("gusu", "姑苏区"))
                .child(CascaderOption::new("wuzhong", "吴中区").disabled(true)),
        ),
    ]
}
