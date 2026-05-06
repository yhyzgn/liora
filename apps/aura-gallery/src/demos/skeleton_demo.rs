use aura_components::{Button, Skeleton, SkeletonItem, SkeletonVariant};
use aura_core::Config;
use gpui::{AnyView, App, Context, Render, Window, div, prelude::*, px};

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

        div().flex().flex_col().gap_8().p_4()
            .child(
                div().flex().flex_col().gap_2()
                    .child(div().text_lg().font_weight(gpui::FontWeight::BOLD).child("Skeleton 骨架屏"))
                    .child(div().text_sm().text_color(theme.neutral.text_3).child("在页面数据加载时展示占位内容。"))
            )
            .child(
                div().flex().flex_row().items_center().gap_4()
                    .child(div().child("切换 Loading 状态:"))
                    .child(
                        Button::new(if loading { "停止加载" } else { "开始加载" })
                            .on_click(cx.listener(|this, _, _, _| {
                                this.loading = !this.loading;
                            }))
                    )
            )
            .child(
                div().flex().flex_col().gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("基础用法"))
                    .child(Skeleton::new().loading(loading).rows(4))
            )
            .child(
                div().flex().flex_col().gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("常见占位类型"))
                    .child(
                        div().flex().flex_row().items_center().gap_6()
                            .child(SkeletonItem::new(SkeletonVariant::Circle))
                            .child(SkeletonItem::new(SkeletonVariant::Square).into_element())
                            .child(SkeletonItem::new(SkeletonVariant::Image))
                    )
            )
            .child(
                div().flex().flex_col().gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("自定义模板"))
                    .child(
                        Skeleton::new().loading(loading)
                            .template(|_, _| {
                                div().flex().flex_row().gap_4().items_start()
                                    .child(SkeletonItem::new(SkeletonVariant::Circle))
                                    .child(
                                        div().flex_1().flex().flex_col().gap_2()
                                            .child(div().w_2_5().child(SkeletonItem::new(SkeletonVariant::Paragraph)))
                                            .child(Skeleton::new().rows(2))
                                    )
                                    .into_any_element()
                            })
                            .child(
                                div().flex().flex_row().gap_4().items_start()
                                    .child(div().size(px(40.0)).bg(gpui::blue()).rounded_full())
                                    .child(
                                        div().flex_1().flex().flex_col().gap_2()
                                            .child(div().font_weight(gpui::FontWeight::BOLD).child("Zed Industries"))
                                            .child(div().text_sm().child("GPUI 是一套基于 GPU 加速的 Rust UI 框架，用于构建 Zed 编辑器。"))
                                    )
                            )
                    )
            )
    }
}
