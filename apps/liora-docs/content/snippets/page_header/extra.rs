//! PageHeader with subtitle and right-side actions.

use gpui::IntoElement;
use liora_components::{Button, ButtonVariant, PageHeader, Space, toast_info};

pub fn page_header_with_extra() -> PageHeader {
    PageHeader::new("详情页面")
        .sub_title("子标题")
        .on_back(|_, _| toast_info!("Back Clicked"))
        .extra(|_, _| {
            Space::new()
                .gap_sm()
                .child(Button::new("编辑"))
                .child(Button::new("主要操作").variant(ButtonVariant::Primary))
                .into_any_element()
        })
}
