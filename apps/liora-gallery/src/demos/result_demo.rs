use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::{Button, ButtonVariant, Result, ResultStatus};

use liora_components::layout_helpers::{page, row_md, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| ResultDemo).into()
}

struct ResultDemo;

impl Render for ResultDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Result 结果页",
            "用于在任务完成后通知用户结果。",
            row_md(vec![
                section(
                    "成功状态",
                    "用于展示成功结果。",
                    Result::new("成功购买云服务器")
                        .status(ResultStatus::Success)
                        .sub_title("订单编号：2017182818828182881，请耐心等待审核。")
                        .extra(|_, _| {
                            row_md(vec![
                                Button::new("返回列表"),
                                Button::new("查看详情").variant(ButtonVariant::Primary),
                            ])
                            .into_any_element()
                        }),
                )
                .into_any_element(),
                section(
                    "警告状态",
                    "用于展示需要注意的结果。",
                    Result::new("您的账户存在安全风险")
                        .status(ResultStatus::Warning)
                        .sub_title("请及时修改密码并开启双重验证。")
                        .extra(|_, _| {
                            Button::new("立即处理")
                                .variant(ButtonVariant::Primary)
                                .into_any_element()
                        }),
                )
                .into_any_element(),
                section(
                    "错误状态",
                    "用于展示失败结果。",
                    Result::new("提交失败")
                        .status(ResultStatus::Error)
                        .sub_title("请检查网络连接并重试。")
                        .extra(|_, _| {
                            Button::new("重新提交")
                                .variant(ButtonVariant::Primary)
                                .into_any_element()
                        }),
                )
                .into_any_element(),
                section(
                    "信息状态",
                    "用于展示普通提示结果。",
                    Result::new("您的申请已提交")
                        .status(ResultStatus::Info)
                        .sub_title("我们将在 3 个工作日内完成审核。")
                        .extra(|_, _| Button::new("知道了").into_any_element()),
                )
                .into_any_element(),
            ]),
        )
    }
}
