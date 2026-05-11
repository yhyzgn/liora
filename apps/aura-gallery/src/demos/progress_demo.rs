use aura_components::{Progress, ProgressStatus, Space};
use gpui::{AnyView, App, Context, Render, Window, prelude::*};

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
            "展示操作进度，告知用户当前状态。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "基础用法",
                    "展示不同百分比的基础进度。",
                    progress_stack(vec![
                        Progress::new(0.0),
                        Progress::new(30.0),
                        Progress::new(50.0),
                        Progress::new(100.0).status(ProgressStatus::Success),
                    ]),
                ))
                .child(section(
                    "百分比内显",
                    "在进度条内部展示百分比。",
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
                    "支持单色和多色渐变。",
                    progress_stack(vec![
                        Progress::new(50.0).primary(),
                        Progress::new(75.0).gradient(vec![
                            theme.success.base,
                            theme.warning.base,
                            theme.danger.base,
                            theme.primary.base,
                        ]),
                    ]),
                )),
        )
    }
}

fn progress_stack(items: Vec<Progress>) -> impl IntoElement {
    Space::new().vertical().gap_md().children(items)
}
