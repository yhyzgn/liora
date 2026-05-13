//! Result status variants.

use aura_components::{Button, Result, ResultStatus, Space};
use gpui::IntoElement;

pub fn result_statuses() -> impl IntoElement {
    Space::new().vertical().gap_lg().children(vec![
        Result::new("您的账户存在安全风险")
            .status(ResultStatus::Warning)
            .sub_title("请及时修改密码并开启双重验证。")
            .extra(|_, _| Button::new("立即处理").primary().into_any_element()),
        Result::new("提交失败")
            .status(ResultStatus::Error)
            .sub_title("请检查网络连接并重试。")
            .extra(|_, _| Button::new("重新提交").primary().into_any_element()),
        Result::new("您的申请已提交")
            .status(ResultStatus::Info)
            .sub_title("我们将在 3 个工作日内完成审核。")
            .extra(|_, _| Button::new("知道了").into_any_element()),
    ])
}
