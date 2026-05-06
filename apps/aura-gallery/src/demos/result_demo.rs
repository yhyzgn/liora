use aura_components::{Button, ButtonVariant, Result, ResultStatus, Space};
use aura_core::Config;
use gpui::{AnyView, App, Context, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| ResultDemo).into()
}

struct ResultDemo;

impl Render for ResultDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

        div()
            .flex()
            .flex_col()
            .gap_8()
            .p_4()
            .id("result-scroll")
            .overflow_y_scroll()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Result 结果页"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("用于在任务完成后通知用户结果。"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("成功状态"))
                    .child(
                        Result::new("成功购买云服务器")
                            .status(ResultStatus::Success)
                            .sub_title("订单编号：2017182818828182881，请耐心等待审核。")
                            .extra(|_, _| {
                                Space::new()
                                    .gap(px(12.0))
                                    .child(Button::new("返回列表"))
                                    .child(Button::new("查看详情").variant(ButtonVariant::Primary))
                                    .into_any_element()
                            }),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("警告状态"))
                    .child(
                        Result::new("您的账户存在安全风险")
                            .status(ResultStatus::Warning)
                            .sub_title("请及时修改密码并开启双重验证。")
                            .extra(|_, _| {
                                Button::new("立即处理")
                                    .variant(ButtonVariant::Primary)
                                    .into_any_element()
                            }),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("错误状态"))
                    .child(
                        Result::new("提交失败")
                            .status(ResultStatus::Error)
                            .sub_title("请检查网络连接并重试。")
                            .extra(|_, _| {
                                Button::new("重新提交")
                                    .variant(ButtonVariant::Primary)
                                    .into_any_element()
                            }),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("信息状态"))
                    .child(
                        Result::new("您的申请已提交")
                            .status(ResultStatus::Info)
                            .sub_title("我们将在 3 个工作日内完成审核。")
                            .extra(|_, _| Button::new("知道了").into_any_element()),
                    ),
            )
    }
}
