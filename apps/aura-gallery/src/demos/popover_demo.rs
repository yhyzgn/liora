use aura_components::{Button, Popover, Space};
use aura_core::{Config, Placement};
use gpui::{prelude::*, px, AnyView, App, Context, Render, Window, div};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| PopoverDemo).into()
}

struct PopoverDemo;

impl Render for PopoverDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

        div().flex().flex_col().gap_6()
            .child(section(theme, "Basic 基础用法", "点击触发元素显示卡片内容。"))
            .child(
                Space::new().gap(px(16.0))
                    .child(card_popover("Bottom Center", Placement::Bottom))
                    .child(card_popover("Top Center", Placement::Top))
                    .child(card_popover("Left Center", Placement::Left))
                    .child(card_popover("Right Center", Placement::Right))
            )
            .child(section(theme, "Placements 十二方向", "覆盖 Top/Bottom/Left/Right 及 Start/End 对齐。"))
            .child(
                div().flex().flex_col().gap_3()
                    .child(Space::new().gap(px(10.0))
                        .child(simple_popover("TopStart", Placement::TopStart))
                        .child(simple_popover("Top", Placement::Top))
                        .child(simple_popover("TopEnd", Placement::TopEnd)))
                    .child(Space::new().gap(px(10.0))
                        .child(simple_popover("LeftStart", Placement::LeftStart))
                        .child(simple_popover("RightStart", Placement::RightStart)))
                    .child(Space::new().gap(px(10.0))
                        .child(simple_popover("Left", Placement::Left))
                        .child(simple_popover("Right", Placement::Right)))
                    .child(Space::new().gap(px(10.0))
                        .child(simple_popover("LeftEnd", Placement::LeftEnd))
                        .child(simple_popover("RightEnd", Placement::RightEnd)))
                    .child(Space::new().gap(px(10.0))
                        .child(simple_popover("BottomStart", Placement::BottomStart))
                        .child(simple_popover("Bottom", Placement::Bottom))
                        .child(simple_popover("BottomEnd", Placement::BottomEnd)))
            )
            .child(section(theme, "Close strategy 关闭策略", "支持禁用点击空白处关闭，只能通过内容按钮手动关闭。"))
            .child(
                Space::new().gap(px(16.0))
                    .child(
                        Popover::new(Button::new("Manual Close Only").warning())
                            .id("popover-demo-manual-close")
                            .placement(Placement::Bottom)
                            .close_on_click_outside(false)
                            .content(|_, _| {
                                div().p_4().flex().flex_col().gap_3()
                                    .child(div().font_weight(gpui::FontWeight::BOLD).child("Manual close"))
                                    .child(div().text_sm().child("点击空白处不会关闭；点击按钮手动关闭。"))
                                    .child(Button::new("Close Popover").primary().small().on_click(|_, _, cx| {
                                        aura_core::clear_active_popover(cx);
                                    }))
                            })
                    )
                    .child(
                        Popover::new(Button::new("Custom Offset"))
                            .id("popover-demo-custom-offset")
                            .placement(Placement::Bottom)
                            .offset(px(20.0))
                            .content(|_, _| div().p_4().child("Offset = 20px"))
                    )
            )
    }
}

fn section(theme: &aura_theme::Theme, title: &'static str, desc: &'static str) -> impl IntoElement {
    div().flex().flex_col().gap_1()
        .child(div().text_lg().font_weight(gpui::FontWeight::BOLD).child(title))
        .child(div().text_sm().text_color(theme.neutral.text_3).child(desc))
}

fn simple_popover(label: &'static str, placement: Placement) -> Popover {
    Popover::new(Button::new(label).small())
        .id(format!("popover-demo-simple-{}", label))
        .placement(placement)
        .content(move |_, _| div().p_3().child(format!("Placement: {:?}", placement)))
}

fn card_popover(label: &'static str, placement: Placement) -> Popover {
    Popover::new(Button::new(label).primary())
        .id(format!("popover-demo-card-{}", label))
        .placement(placement)
        .content(|_, _| {
            div().p_4().flex().flex_col().gap_2()
                .child(div().font_weight(gpui::FontWeight::BOLD).child("Title"))
                .child(div().text_sm().child("This is the popover content."))
                .child(Button::new("Confirm").primary().small())
        })
}
