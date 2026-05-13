//! Full PageHeader embedded in a Card with content and footer slots.

use aura_components::{Button, ButtonVariant, Card, PageHeader, Row, Space, Text, toast_info};
use gpui::IntoElement;

pub fn full_page_header() -> Card {
    Card::new(
        PageHeader::new("详情页面")
            .sub_title("子标题")
            .on_back(|_, _| toast_info!("Back Clicked"))
            .extra(|_, _| {
                Space::new()
                    .gap_sm()
                    .child(Button::new("刷新"))
                    .child(Button::new("提交").variant(ButtonVariant::Primary))
                    .into_any_element()
            })
            .content(|_, _| {
                Row::new()
                    .child(
                        Space::new()
                            .vertical()
                            .gap_xs()
                            .child(Text::new("创建人"))
                            .child(Text::new("张三").bold()),
                    )
                    .child(
                        Space::new()
                            .vertical()
                            .gap_xs()
                            .child(Text::new("创建时间"))
                            .child(Text::new("2026-05-06").bold()),
                    )
                    .into_any_element()
            })
            .footer(|_, _| Text::new("页脚内容区域").into_any_element()),
    )
    .no_shadow()
}
