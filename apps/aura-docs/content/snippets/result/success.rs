//! Success Result with action buttons.

use aura_components::{Button, Result, ResultStatus, Space};
use gpui::IntoElement;

pub fn success_result() -> Result {
    Result::new("成功购买云服务器")
        .status(ResultStatus::Success)
        .sub_title("订单编号：2017182818828182881，请耐心等待审核。")
        .extra(|_, _| {
            Space::new()
                .gap_md()
                .child(Button::new("返回列表"))
                .child(Button::new("查看详情").primary())
                .into_any_element()
        })
}
