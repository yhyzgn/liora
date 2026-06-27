//! Sheet placement examples.

use gpui::IntoElement;
use liora_components::{Button, Sheet, Space, Text};
use liora_icons_lucide::IconName;

pub fn sheet_placements() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_md()
        .child(
            Button::new("Right")
                .icon_start(IconName::PanelRightOpen)
                .on_click(|_, _, cx| {
                    Sheet::new()
                        .title("Inspector")
                        .right()
                        .content(|_| sheet_body("Right inspector"))
                        .show(cx);
                }),
        )
        .child(
            Button::new("Left")
                .icon_start(IconName::PanelLeftOpen)
                .on_click(|_, _, cx| {
                    Sheet::new()
                        .title("Navigator")
                        .left()
                        .content(|_| sheet_body("Left navigator"))
                        .show(cx);
                }),
        )
        .child(
            Button::new("Top")
                .icon_start(IconName::PanelTopOpen)
                .on_click(|_, _, cx| {
                    Sheet::new()
                        .title("Command")
                        .top()
                        .height_sm()
                        .content(|_| sheet_body("Top command"))
                        .show(cx);
                }),
        )
        .child(
            Button::new("Bottom")
                .icon_start(IconName::PanelBottomOpen)
                .on_click(|_, _, cx| {
                    Sheet::new()
                        .title("Actions")
                        .bottom()
                        .height_sm()
                        .content(|_| sheet_body("Bottom actions"))
                        .show(cx);
                }),
        )
}

fn sheet_body(title: &'static str) -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_md()
        .child(Text::new(title).bold())
        .child(Text::new("Use Sheet for a short contextual flow."))
}
