use aura_components::Scrollbar;
use gpui::{App, div, prelude::*, px, Entity};

pub fn render(cx: &mut App) -> Entity<Scrollbar> {
    cx.new(|cx| {
        Scrollbar::new(cx, |_, _| {
            let items: Vec<String> = (1..=20).map(|i| format!("Scrollable line {}", i)).collect();
            div().flex().flex_col().p_2()
                .children(items.iter().map(|s| {
                    div().h(px(36.0)).flex().items_center().border_b_1()
                        .border_color(gpui::hsla(0.0, 0.0, 0.5, 0.2)).child(s.clone())
                }))
        }).height(150.0)
    })
}
