use aura_components::{Button, Drawer, DrawerPlacement, Space};
use aura_core::Config;
use gpui::{prelude::*, px, AnyView, App, Context, Render, Window, div};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| DrawerDemo).into()
}

struct DrawerDemo;

impl Render for DrawerDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

        div().flex().flex_col().gap_6()
            .child(section(theme, "Placements 四方向", "屏幕边缘滑出的浮层面板。"))
            .child(
                Space::new().gap(px(16.0))
                    .child(Button::new("Right Drawer").primary().on_click(|_, _, cx| {
                        drawer("Right Drawer", DrawerPlacement::Right).show(cx);
                    }))
                    .child(Button::new("Left Drawer").on_click(|_, _, cx| {
                        drawer("Left Drawer", DrawerPlacement::Left).show(cx);
                    }))
                    .child(Button::new("Top Drawer").on_click(|_, _, cx| {
                        drawer("Top Drawer", DrawerPlacement::Top).height(px(200.0)).show(cx);
                    }))
                    .child(Button::new("Bottom Drawer").on_click(|_, _, cx| {
                        drawer("Bottom Drawer", DrawerPlacement::Bottom).height(px(200.0)).show(cx);
                    }))
            )
            .child(section(theme, "Size 尺寸", "可配置宽度或高度。"))
            .child(
                Space::new().gap(px(16.0))
                    .child(Button::new("Wide Drawer").on_click(|_, _, cx| {
                        drawer("Wide Drawer", DrawerPlacement::Right).width(px(480.0)).show(cx);
                    }))
                    .child(Button::new("Tall Top Drawer").on_click(|_, _, cx| {
                        drawer("Tall Top Drawer", DrawerPlacement::Top).height(px(360.0)).show(cx);
                    }))
            )
            .child(section(theme, "Close strategy 关闭策略", "可禁用遮罩和 ESC 关闭，改由业务按钮手动关闭。"))
            .child(
                Space::new().gap(px(16.0))
                    .child(Button::new("Manual Close Only").warning().on_click(|_, _, cx| {
                        Drawer::new()
                            .title("Manual close drawer")
                            .close_on_click_outside(false)
                            .close_on_escape(false)
                            .content(|_, _| {
                                div().flex().flex_col().gap_4()
                                    .child("点击遮罩和按 ESC 都不会关闭。")
                                    .child(Button::new("Close Drawer").primary().on_click(|_, _, cx| Drawer::close(cx)))
                            })
                            .show(cx);
                    }))
            )
    }
}

fn section(theme: &aura_theme::Theme, title: &'static str, desc: &'static str) -> impl IntoElement {
    div().flex().flex_col().gap_1()
        .child(div().text_lg().font_weight(gpui::FontWeight::BOLD).child(title))
        .child(div().text_sm().text_color(theme.neutral.text_3).child(desc))
}

fn drawer(title: &'static str, placement: DrawerPlacement) -> Drawer {
    Drawer::new()
        .title(title)
        .placement(placement)
        .content(move |_, _| {
            div().flex().flex_col().gap_4()
                .child(format!("This is a {:?} drawer.", placement))
                .child(Button::new("Close").primary().on_click(|_, _, cx| Drawer::close(cx)))
        })
}
