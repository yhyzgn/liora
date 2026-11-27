//! Preselect a valid path by value segments.

use gpui::Context;
use liora_components::{Cascader, CascaderOption, MessageType, show_message};

pub fn selected_cascader(cx: &mut Context<Cascader>) -> Cascader {
    Cascader::new(product_options(), cx)
        .selected_path(["cloud", "compute", "ecs"])
        .placeholder("请选择产品")
        .width_md()
        .on_change(|path, _window, cx| {
            let values = path.join(" / ");
            show_message(format!("Selected {values}"), MessageType::Info, cx);
        })
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
