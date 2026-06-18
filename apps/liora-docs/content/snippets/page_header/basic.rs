//! Basic PageHeader with a native back callback.

use liora_components::{PageHeader, toast_info};

pub fn basic_page_header() -> PageHeader {
    PageHeader::new("详情页面").on_back(|_, _| toast_info!("Back Clicked"))
}
