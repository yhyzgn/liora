use aura_components::{Anchor, AnchorLink, AnchorTarget, Flex, Space, Text};
use aura_core::Config;
use gpui::{AnyView, App, Context, Entity, IntoElement, Render, ScrollHandle, Window, prelude::*};

use aura_components::layout_helpers::page;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| AnchorDemo::new(cx)).into()
}

struct AnchorDemo {
    scroll_handle: ScrollHandle,
    anchor: Entity<Anchor>,
}

impl AnchorDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let scroll_handle = ScrollHandle::new();
        let anchor = cx.new({
            let scroll_handle = scroll_handle.clone();
            |_| {
                Anchor::new(scroll_handle)
                    .offset_sm()
                    .link(AnchorLink::new("基础用法", "basic"))
                    .link(
                        AnchorLink::new("API", "api")
                            .child(AnchorLink::new("Attributes", "attributes"))
                            .child(AnchorLink::new("Events", "events")),
                    )
            }
        });
        Self {
            scroll_handle,
            anchor,
        }
    }
}

impl Render for AnchorDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let scroll_handle = self.scroll_handle.clone();
        let anchor = self.anchor.clone();

        page(
            "Anchor 锚点",
            "用于在长页面中提供快速跳转。",
            Flex::new()
                .row()
                .gap_lg()
                .height_units(620.0)
                .overflow_hidden()
                .border()
                .border_color(theme.neutral.border)
                .rounded_md()
                .padding_md()
                .child(
                    Flex::new().column().width_percent(75.0).h_full().child(
                        Flex::new()
                            .flex_1()
                            .id("anchor-scroll-view")
                            .overflow_y_scroll()
                            .track_scroll(&scroll_handle)
                            .child(
                                Space::new()
                                    .vertical()
                                    .gap_xl()
                                    .child(AnchorTarget::new(
                                        "basic",
                                        anchor.clone(),
                                        anchor_panel(&theme, "基础用法内容区域", 400.0),
                                    ))
                                    .child(AnchorTarget::new(
                                        "api",
                                        anchor.clone(),
                                        anchor_panel(&theme, "API 内容区域", 200.0),
                                    ))
                                    .child(AnchorTarget::new(
                                        "attributes",
                                        anchor.clone(),
                                        anchor_panel(&theme, "Attributes 内容区域", 400.0),
                                    ))
                                    .child(AnchorTarget::new(
                                        "events",
                                        anchor.clone(),
                                        anchor_panel(&theme, "Events 内容区域", 400.0),
                                    ))
                                    .child(Flex::new().height_units(400.0)),
                            ),
                    ),
                )
                .child(Flex::new().width_percent(25.0).child(anchor)),
        )
    }
}

fn anchor_panel(theme: &aura_theme::Theme, label: &'static str, height: f32) -> impl IntoElement {
    Flex::new()
        .height_units(height)
        .bg(theme.neutral.hover)
        .rounded_md()
        .center()
        .child(Text::new(label))
}
