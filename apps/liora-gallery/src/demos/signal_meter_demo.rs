use gpui::{AnyView, App, Context, Render, Window, green, prelude::*, px, rgb};
use liora_components::layout_helpers::{page, row_md, section};
use liora_components::{Card, SignalLevelColor, SignalMeter, Space, Text};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| SignalMeterDemo).into()
}

struct SignalMeterDemo;

impl Render for SignalMeterDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "SignalMeter 信号图",
            "模拟手机移动信号与 Wi-Fi 信号等级，可配置总信号数、每级颜色和尺寸。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "移动信号",
                    "柱状阶梯展示 0 到 4 格信号。",
                    row_md(
                        (0..=4)
                            .map(|level| {
                                Card::new(
                                    Space::new()
                                        .vertical()
                                        .gap_sm()
                                        .align_center()
                                        .child(SignalMeter::new(level))
                                        .child(Text::new(format!("{} 格", level)).size(px(12.0))),
                                )
                                .width(px(88.0))
                            })
                            .collect::<Vec<_>>(),
                    ),
                ))
                .child(section(
                    "总信号数与每级颜色",
                    "total_signals / signal_count 可指定总格数，level_colors 可按等级分别指定颜色。",
                    row_md(vec![
                        SignalMeter::new(5)
                            .total_signals(6)
                            .level_colors([
                                rgb(0xef4444).into(),
                                rgb(0xf97316).into(),
                                rgb(0xf59e0b).into(),
                                rgb(0x84cc16).into(),
                                rgb(0x22c55e).into(),
                                rgb(0x16a34a).into(),
                            ])
                            .height(px(44.0))
                            .bar_width(px(7.0))
                            .gap(px(5.0))
                            .into_any_element(),
                        SignalMeter::new(4)
                            .wifi()
                            .signal_count(5)
                            .signal_colors([
                                rgb(0x60a5fa).into(),
                                rgb(0x3b82f6).into(),
                                rgb(0x2563eb).into(),
                                rgb(0x1d4ed8).into(),
                                rgb(0x1e40af).into(),
                            ])
                            .inactive_color(rgb(0xdbeafe).into())
                            .height(px(44.0))
                            .bar_width(px(8.0))
                            .gap(px(5.0))
                            .into_any_element(),
                    ]),
                ))
                .child(section(
                    "按当前等级统一着色",
                    "threshold_colors / level_color 可按当前信号等级决定所有激活格的统一颜色。",
                    row_md(vec![
                        SignalMeter::new(2)
                            .total_signals(5)
                            .threshold_colors(threshold_colors())
                            .inactive_color(rgb(0xfee2e2).into())
                            .height(px(42.0))
                            .bar_width(px(7.0))
                            .gap(px(5.0))
                            .into_any_element(),
                        SignalMeter::new(3)
                            .total_signals(5)
                            .threshold_colors(threshold_colors())
                            .inactive_color(rgb(0xfef3c7).into())
                            .height(px(42.0))
                            .bar_width(px(7.0))
                            .gap(px(5.0))
                            .into_any_element(),
                        SignalMeter::new(5)
                            .total_signals(5)
                            .threshold_colors(threshold_colors())
                            .inactive_color(rgb(0xdcfce7).into())
                            .height(px(42.0))
                            .bar_width(px(7.0))
                            .gap(px(5.0))
                            .into_any_element(),
                    ]),
                ))
                .child(section(
                    "Wi-Fi 风格与颜色",
                    "可切换为 Wi-Fi 风格，并指定激活/未激活颜色。",
                    row_md(vec![
                        SignalMeter::new(3)
                            .wifi()
                            .active_color(green())
                            .height(px(40.0))
                            .into_any_element(),
                        SignalMeter::new(2)
                            .wifi()
                            .active_color(rgb(0x3b82f6).into())
                            .inactive_color(rgb(0xdbeafe).into())
                            .bar_width(px(8.0))
                            .gap(px(5.0))
                            .height(px(44.0))
                            .into_any_element(),
                    ]),
                )),
        )
    }
}

fn threshold_colors() -> Vec<SignalLevelColor> {
    vec![
        SignalLevelColor::new(2, rgb(0xef4444).into()),
        SignalLevelColor::new(3, rgb(0xeab308).into()),
        SignalLevelColor::new(4, rgb(0xf97316).into()),
        SignalLevelColor::new(5, rgb(0x22c55e).into()),
    ]
}
