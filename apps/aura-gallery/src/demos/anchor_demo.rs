use aura_components::{Anchor, AnchorLink, AnchorTarget};
use aura_core::Config;
use gpui::{AnyView, App, Context, Render, ScrollHandle, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| AnchorDemo::new()).into()
}

struct AnchorDemo {
    scroll_handle: ScrollHandle,
}

impl AnchorDemo {
    fn new() -> Self {
        Self {
            scroll_handle: ScrollHandle::new(),
        }
    }
}

impl Render for AnchorDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let scroll_handle = self.scroll_handle.clone();

        let anchor = cx.new(|_| {
            Anchor::new(scroll_handle.clone())
                .offset(px(20.0))
                .link(AnchorLink::new("基础用法", "basic"))
                .link(
                    AnchorLink::new("API", "api")
                        .child(AnchorLink::new("Attributes", "attributes"))
                        .child(AnchorLink::new("Events", "events")),
                )
        });

        div()
            .flex()
            .flex_col()
            .gap_8()
            .p_4()
            .h_full()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Anchor 锚点"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("用于在长页面中提供快速跳转。"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_4()
                    .flex_1()
                    .overflow_hidden()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .w(gpui::relative(0.75))
                            .h_full()
                            .child(
                                div()
                                    .flex_1()
                                    .id("anchor-scroll-view")
                                    .overflow_y_scroll()
                                    .track_scroll(&scroll_handle)
                                    .on_scroll_wheel(cx.listener(|_, _, _, cx| {
                                        cx.notify();
                                    }))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_10()
                                            .p_4()
                                            .child(AnchorTarget::new(
                                                "basic",
                                                anchor.clone(),
                                                div()
                                                    .h(px(400.0))
                                                    .bg(theme.neutral.hover)
                                                    .rounded(px(theme.radius.md))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .child("基础用法内容区域"),
                                            ))
                                            .child(AnchorTarget::new(
                                                "api",
                                                anchor.clone(),
                                                div()
                                                    .h(px(200.0))
                                                    .bg(theme.neutral.hover)
                                                    .rounded(px(theme.radius.md))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .child("API 内容区域"),
                                            ))
                                            .child(AnchorTarget::new(
                                                "attributes",
                                                anchor.clone(),
                                                div()
                                                    .h(px(400.0))
                                                    .bg(theme.neutral.hover)
                                                    .rounded(px(theme.radius.md))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .child("Attributes 内容区域"),
                                            ))
                                            .child(AnchorTarget::new(
                                                "events",
                                                anchor.clone(),
                                                div()
                                                    .h(px(400.0))
                                                    .bg(theme.neutral.hover)
                                                    .rounded(px(theme.radius.md))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .child("Events 内容区域"),
                                            ))
                                            .child(div().h(px(400.0))), // Bottom spacer
                                    ),
                            ),
                    )
                    .child(div().w(gpui::relative(0.25)).child(anchor)),
            )
    }
}
