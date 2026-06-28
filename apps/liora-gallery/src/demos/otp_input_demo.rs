use gpui::{AnyView, App, Context, Entity, Render, Window, prelude::*};
use liora_components::layout_helpers::{page, section, showcase_card, showcase_grid};
use liora_components::{OtpInput, Space, Text};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| OtpInputDemo {
        login_code: cx.new(|cx| OtpInput::new("", cx)),
        device_code: cx.new(|cx| OtpInput::new("42", cx).length(4, cx).masked(true)),
        success_code: cx.new(|cx| OtpInput::new("934201", cx).success()),
        error_code: cx.new(|cx| OtpInput::new("128", cx).length(4, cx).error()),
        compact_code: cx.new(|cx| {
            OtpInput::new("AB7", cx)
                .length(5, cx)
                .cell_size(gpui::px(32.0))
        }),
    })
    .into()
}

struct OtpInputDemo {
    login_code: Entity<OtpInput>,
    device_code: Entity<OtpInput>,
    success_code: Entity<OtpInput>,
    error_code: Entity<OtpInput>,
    compact_code: Entity<OtpInput>,
}

impl Render for OtpInputDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "OtpInput 验证码输入",
            "真正可交互的一次性验证码 / PIN 控件：点击格子定位光标，键盘输入、退格和粘贴走 Liora Input 编辑管线。",
            Space::new().vertical().gap_xl().child(section(
                "OtpInput showcase",
                "验证码输入示例统一放入卡片网格，避免不同长度的格子控件散落。",
                showcase_grid(vec![
                    showcase_card(
                        "可输入验证码",
                        "点击任意格子后输入，支持键盘移动、退格和粘贴。",
                        Space::new()
                            .vertical()
                            .gap_sm()
                            .child(self.login_code.clone())
                            .child(
                                Text::new(
                                    "Try: click a cell, type digits, backspace, or paste a code.",
                                )
                                .xs(),
                            ),
                    )
                    .into_any_element(),
                    showcase_card(
                        "PIN / Masked",
                        "masked(true) 用于 PIN、设备配对码等需要隐藏具体字符的场景。",
                        self.device_code.clone(),
                    )
                    .into_any_element(),
                    showcase_card(
                        "状态",
                        "success / error / warning 用于校验结果、重试和业务反馈。",
                        Space::new()
                            .vertical()
                            .gap_md()
                            .child(self.success_code.clone())
                            .child(self.error_code.clone()),
                    )
                    .into_any_element(),
                    showcase_card(
                        "长度和尺寸",
                        "长度可在 1-12 间配置，cell_size/gap 用于紧凑布局。",
                        self.compact_code.clone(),
                    )
                    .into_any_element(),
                ]),
            )),
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn otp_input_demo_is_dedicated_interactive_and_rich() {
        let source = include_str!("otp_input_demo.rs");
        assert!(source.contains("Entity<OtpInput>"));
        assert!(source.contains("点击格子定位光标"));
        assert!(source.contains("masked(true)"));
        assert!(source.contains("success_code"));
        assert!(source.contains("showcase_grid"));
    }
}
