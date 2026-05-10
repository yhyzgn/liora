use aura_components::{Avatar, Button, Skeleton, SkeletonItem, SkeletonVariant, Space, Text};
use aura_core::Config;
use gpui::{AnyElement, AnyView, App, Context, Hsla, Render, Window, prelude::*};

use super::common::{page, row, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| SkeletonDemo { loading: true }).into()
}

struct SkeletonDemo {
    loading: bool,
}

impl Render for SkeletonDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        let loading = self.loading;

        page(
            "Skeleton 骨架屏",
            "在页面数据加载时展示占位内容。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(
                    Space::new()
                        .gap_lg()
                        .child(Text::new("切换 Loading 状态:").nowrap())
                        .child(
                            Button::new(if loading {
                                "停止加载"
                            } else {
                                "开始加载"
                            })
                            .on_click(cx.listener(|this, _, _, _| {
                                this.loading = !this.loading;
                            })),
                        ),
                )
                .child(section(
                    "基础用法",
                    "通过 rows 配置基础段落占位行数。",
                    Skeleton::new().loading(loading).rows(4),
                ))
                .child(section(
                    "常见占位类型",
                    "提供圆形、方形、图片和段落占位。",
                    row(vec![
                        SkeletonItem::new(SkeletonVariant::Circle),
                        SkeletonItem::new(SkeletonVariant::Square),
                        SkeletonItem::new(SkeletonVariant::Image),
                    ]),
                ))
                .child(section(
                    "自定义模板",
                    "组合 SkeletonItem、Skeleton 与真实内容，加载结束后显示业务内容。",
                    Skeleton::new()
                        .loading(loading)
                        .template(|_, _| skeleton_template())
                        .child(loaded_content(theme.primary.base)),
                )),
        )
    }
}

fn skeleton_template() -> AnyElement {
    Space::new()
        .align_start()
        .gap_lg()
        .child(SkeletonItem::new(SkeletonVariant::Circle))
        .child(
            Space::new()
                .vertical()
                .grow()
                .gap_sm()
                .child(SkeletonItem::new(SkeletonVariant::Paragraph).width_2_5())
                .child(Skeleton::new().rows(2)),
        )
        .into_any_element()
}

fn loaded_content(avatar_bg: Hsla) -> impl IntoElement {
    Space::new()
        .align_start()
        .gap_lg()
        .child(Avatar::new().background(avatar_bg))
        .child(
            Space::new()
                .vertical()
                .grow()
                .gap_sm()
                .child(Text::new("Zed Industries").bold())
                .child(Text::new(
                    "GPUI 是一套基于 GPU 加速的 Rust UI 框架，用于构建 Zed 编辑器。",
                )),
        )
}
