use aura_components::{Progress, ProgressStatus, Space};
use gpui::{AnyView, App, Context, FontWeight, Render, Window, prelude::*};

use aura_components::layout_helpers::{page, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| ProgressDemo).into()
}

struct ProgressDemo;

impl Render for ProgressDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<aura_core::Config>().theme.clone();

        page(
            "Progress 进度条",
            "展示操作进度，支持条状、环状、动画、状态、渐变和中心文本自定义。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "基础用法",
                    "条状进度默认带进入动画，用于展示任务完成度。",
                    progress_stack(vec![
                        Progress::new(0.0),
                        Progress::new(30.0),
                        Progress::new(50.0),
                        Progress::new(100.0).status(ProgressStatus::Success),
                    ]),
                ))
                .child(section(
                    "百分比内显",
                    "可将百分比放入进度条内部，也可以居中显示。",
                    progress_stack(vec![
                        Progress::new(15.0).thick().text_inside(true),
                        Progress::new(70.0).thick().text_inside(true),
                        Progress::new(70.0).thick().text_inside_centered(),
                        Progress::new(100.0)
                            .thick()
                            .text_inside(true)
                            .status(ProgressStatus::Success),
                    ]),
                ))
                .child(section(
                    "不同状态",
                    "通过状态表达成功、警告和异常。",
                    progress_stack(vec![
                        Progress::new(30.0),
                        Progress::new(50.0).status(ProgressStatus::Warning),
                        Progress::new(70.0).status(ProgressStatus::Exception),
                        Progress::new(100.0).status(ProgressStatus::Success),
                    ]),
                ))
                .child(section(
                    "自定义颜色",
                    "支持单色和多色渐变，也可以关闭动画用于静态指标。",
                    progress_stack(vec![
                        Progress::new(50.0).primary(),
                        Progress::new(75.0).gradient(vec![
                            theme.success.base,
                            theme.warning.base,
                            theme.danger.base,
                            theme.primary.base,
                        ]),
                        Progress::new(64.0)
                            .color(theme.info.base)
                            .track_color(theme.neutral.hover)
                            .animated(false),
                    ]),
                ))
                .child(section(
                    "环状进度条",
                    "环状进度条使用原生 GPUI Path 绘制，支持状态色和进入动画。",
                    Space::new().gap_lg().wrap().children(vec![
                        Progress::new(32.0).circle(),
                        Progress::new(58.0).circle().status(ProgressStatus::Warning),
                        Progress::new(76.0).circle().status(ProgressStatus::Exception),
                        Progress::new(100.0).circle().status(ProgressStatus::Success),
                    ]),
                ))
                .child(section(
                    "自定义中心文本与样式",
                    "中心进度显示可以替换为业务文案，并独立配置尺寸、颜色、字重、圆环尺寸和轨道颜色。",
                    Space::new().gap_lg().wrap().children(vec![
                        Progress::new(86.0)
                            .circle()
                            .circle_size(148.0)
                            .ring_width(12.0)
                            .ring_color(theme.neutral.hover)
                            .progress_color(theme.primary.base)
                            .inner_color(theme.neutral.card)
                            .center_text("Deploy")
                            .text_size(22.0)
                            .text_color(theme.primary.base),
                        Progress::new(42.0)
                            .circle()
                            .circle_size(132.0)
                            .ring_width(10.0)
                            .ring_color(theme.success.hover.opacity(0.24))
                            .progress_color(theme.success.base)
                            .inner_color(theme.neutral.body)
                            .center_text("42 / 100")
                            .text_size(16.0)
                            .text_weight(FontWeight::NORMAL),
                        Progress::new(68.0)
                            .circle()
                            .circle_size(132.0)
                            .ring_width(14.0)
                            .ring_color(theme.warning.hover.opacity(0.28))
                            .progress_color(theme.warning.base)
                            .inner_color(theme.neutral.card.opacity(0.78))
                            .center_text("CPU")
                            .text_size(18.0)
                            .text_color(theme.warning.base)
                            .animated(false),
                    ]),
                )),
        )
    }
}

fn progress_stack(items: Vec<Progress>) -> impl IntoElement {
    Space::new().vertical().gap_md().children(items)
}
