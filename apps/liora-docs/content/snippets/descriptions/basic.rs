//! Basic borderless Descriptions.

use liora_components::{Descriptions, Text};

pub fn basic_descriptions() -> Descriptions {
    Descriptions::new()
        .title("用户信息")
        .item("用户名", "kooriookami", 1)
        .item("手机号", "18100000000", 1)
        .item("居住地", "苏州市", 1)
        .item("备注", Text::new("学校").bg(gpui::blue().opacity(0.1)), 1)
        .item("联系地址", "江苏省苏州市吴中区越溪街道月苑路", 2)
}
