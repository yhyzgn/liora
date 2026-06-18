//! Enable the searchable result panel and optionally seed a query.

use gpui::Context;
use liora_components::{Cascader, CascaderOption};

pub fn filterable_cascader(cx: &mut Context<Cascader>) -> Cascader {
    Cascader::new(region_options(), cx)
        .filterable(true)
        .search_query("hang")
        .placeholder("搜索 hang")
        .width_md()
}

fn region_options() -> Vec<CascaderOption> {
    vec![
        CascaderOption::new("zhejiang", "浙江").child(
            CascaderOption::new("hangzhou", "杭州")
                .child(CascaderOption::new("xihu", "西湖区"))
                .child(CascaderOption::new("yuhang", "余杭区")),
        ),
    ]
}
