use aura_components::{Backtop, Button, ButtonVariant};
use aura_core::{Config};
use gpui::{prelude::*, App, Context, Render, Window, div, AnyView, px, ScrollHandle};
use aura_icons::Icon;
use aura_icons_lucide::IconName;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| BacktopDemo::new()).into()
}

struct BacktopDemo {
    scroll_handle: ScrollHandle,
}

impl BacktopDemo {
    fn new() -> Self {
        Self {
            scroll_handle: ScrollHandle::new(),
        }
    }
}

impl Render for BacktopDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let scroll_handle = self.scroll_handle.clone();

        div().flex().flex_col().gap_8().p_4().h_full().relative()
            .child(
                div().flex().flex_col().gap_2()
                    .child(div().text_lg().font_weight(gpui::FontWeight::BOLD).child("Backtop 回到顶部"))
                    .child(div().text_sm().text_color(theme.neutral.text_3).child("向下滚动查看右下角的回到顶部按钮。"))
            )
            .child(
                div().flex_1().id("backtop-scroll-view").overflow_y_scroll().track_scroll(&scroll_handle)
                    .on_scroll_wheel(cx.listener(|_, _, _, cx| {
                        cx.notify();
                    }))
                    .child(
                        div().flex().flex_col().gap_4()
                            .children((0..50).map(|i| {
                                div().h(px(40.0)).flex().items_center().px_4().bg(theme.neutral.hover).rounded(px(theme.radius.sm))
                                    .child(format!("Scroll Item {}", i))
                            }))
                    )
            )
            .child(
                cx.new(|_| {
                    Backtop::new(scroll_handle.clone())
                        .visibility_height(px(100.0))
                })
            )
            .child(
                cx.new(|_| {
                    Backtop::new(scroll_handle.clone())
                        .visibility_height(px(200.0))
                        .right(px(100.0))
                        .content(|_, _| {
                            div().flex().items_center().justify_center().size_full().bg(gpui::blue())
                                .child(Icon::new(IconName::ArrowUp).size(px(20.0)).color(gpui::white()))
                                .into_any_element()
                        })
                })
            )
    }
}
