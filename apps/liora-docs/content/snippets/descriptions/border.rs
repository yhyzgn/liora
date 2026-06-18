//! Bordered Descriptions with an action area.

use liora_components::{Button, Descriptions};

pub fn bordered_descriptions() -> Descriptions {
    Descriptions::new()
        .title("用户信息")
        .border(true)
        .extra(Button::new("操作").primary().small())
        .item("用户名", "kooriookami", 1)
        .item("手机号", "18100000000", 1)
        .item("居住地", "苏州市", 1)
        .item("备注", "学校", 1)
        .item("联系地址", "江苏省苏州市吴中区越溪街道月苑路", 2)
}
