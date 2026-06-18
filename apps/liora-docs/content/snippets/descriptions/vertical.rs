//! Vertical bordered Descriptions.

use liora_components::{Descriptions, DescriptionsDirection};

pub fn vertical_descriptions() -> Descriptions {
    Descriptions::new()
        .title("垂直布局")
        .border(true)
        .direction(DescriptionsDirection::Vertical)
        .item("用户名", "kooriookami", 1)
        .item("手机号", "18100000000", 1)
        .item("居住地", "苏州市", 1)
        .item("备注", "学校", 1)
        .item("联系地址", "江苏省苏州市吴中区越溪街道月苑路", 2)
}
