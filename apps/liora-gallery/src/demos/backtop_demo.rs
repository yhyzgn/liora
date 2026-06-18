use gpui::{AnyView, App, Context, Entity, IntoElement, Render, ScrollHandle, Window, prelude::*};
use liora_components::{Backtop, Flex, Space, Text};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

use liora_components::layout_helpers::page;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| BacktopDemo::new(cx)).into()
}

struct BacktopDemo {
    scroll_handle: ScrollHandle,
    primary: Entity<Backtop>,
    custom: Entity<Backtop>,
}

impl BacktopDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let scroll_handle = ScrollHandle::new();
        Self {
            primary: cx.new({
                let scroll_handle = scroll_handle.clone();
                |_| {
                    Backtop::new(scroll_handle)
                        .id("backtop-demo-primary")
                        .visibility_height_sm()
                }
            }),
            custom: cx.new({
                let scroll_handle = scroll_handle.clone();
                |_| {
                    Backtop::new(scroll_handle)
                        .id("backtop-demo-custom")
                        .right_lg()
                        .content(|_, cx| {
                            let theme = cx.global::<Config>().theme.clone();
                            Flex::new()
                                .size_full()
                                .center()
                                .bg(theme.primary.base)
                                .child(
                                    Icon::new(IconName::ArrowUp)
                                        .size_md()
                                        .color(theme.neutral.card),
                                )
                                .into_any_element()
                        })
                }
            }),
            scroll_handle,
        }
    }
}

impl Render for BacktopDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let scroll_handle = self.scroll_handle.clone();

        page(
            "Backtop 回到顶部",
            "向下滚动查看右下角的回到顶部按钮。",
            Flex::new()
                .relative()
                .height_units(560.0)
                .overflow_hidden()
                .border()
                .border_color(theme.neutral.border)
                .rounded_md()
                .child(
                    Flex::new()
                        .size_full()
                        .id("backtop-scroll-view")
                        .overflow_y_scroll()
                        .track_scroll(&scroll_handle)
                        .child(Space::new().vertical().gap_sm().children((0..50).map(|i| {
                            Flex::new()
                                .height_units(40.0)
                                .row()
                                .align_center()
                                .padding_x_units(16.0)
                                .bg(theme.neutral.hover)
                                .rounded_units(4.0)
                                .child(Text::new(format!("Scroll Item {}", i)))
                        }))),
                )
                .child(self.primary.clone())
                .child(self.custom.clone()),
        )
    }
}
