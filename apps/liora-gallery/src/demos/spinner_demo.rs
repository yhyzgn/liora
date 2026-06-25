use gpui::{AnyView, App, Context, Render, Window, prelude::*, px, rgb};
use liora_components::layout_helpers::{page, row_md, section};
use liora_components::{Kbd, OtpInput, Space, Spinner};
use liora_icons_lucide::IconName;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| SpinnerDemo).into()
}

struct SpinnerDemo;

impl Render for SpinnerDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "P22 快捷控件",
            "从 gpui-component 采集清单落地的低风险高收益控件：Spinner、Kbd 与 OtpInput。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "Spinner 细粒度加载",
                    "独立旋转图标可嵌入按钮、状态栏、列表行和工具栏。",
                    row_md(vec![
                        Spinner::new().small().into_any_element(),
                        Spinner::new().into_any_element(),
                        Spinner::new().large().into_any_element(),
                        Spinner::new()
                            .icon(IconName::RefreshCw)
                            .size(px(20.0))
                            .color(rgb(0x2563eb).into())
                            .into_any_element(),
                    ]),
                ))
                .child(section(
                    "Kbd 快捷键提示",
                    "用于命令面板、菜单、空状态和帮助说明中的键盘快捷键展示。",
                    row_md(vec![
                        Kbd::new("⌘K").into_any_element(),
                        Kbd::new("Ctrl").small().into_any_element(),
                        Kbd::new("Enter").large().into_any_element(),
                        Kbd::new("Esc")
                            .color(rgb(0xdc2626).into())
                            .into_any_element(),
                    ]),
                ))
                .child(section(
                    "OtpInput 一次性验证码",
                    "受控父组件可把真实输入状态同步为格子展示，用于 2FA、设备配对和 PIN 输入。",
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(OtpInput::new("1284").length(6).active_index(4))
                        .child(OtpInput::new("934201").length(6).success())
                        .child(OtpInput::new("12 8").length(4).masked(true).error()),
                )),
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn p22_demo_covers_first_low_risk_controls() {
        let source = include_str!("spinner_demo.rs");
        assert!(source.contains("Spinner::new"));
        assert!(source.contains("Kbd::new"));
        assert!(source.contains("OtpInput::new"));
    }
}
