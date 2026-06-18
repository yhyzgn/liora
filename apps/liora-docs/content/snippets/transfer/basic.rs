//! Basic Transfer with checked source items.

use liora_components::{Transfer, TransferItem};

pub fn transfer_basic() -> Transfer {
    Transfer::new(city_items())
        .titles("待选城市", "已选城市")
        .target_keys(["shanghai"])
        .checked_source_keys(["beijing", "shenzhen"])
}

fn city_items() -> Vec<TransferItem> {
    vec![
        TransferItem::new("beijing", "北京").description("华北区域"),
        TransferItem::new("shanghai", "上海").description("华东区域"),
        TransferItem::new("shenzhen", "深圳").description("华南区域"),
        TransferItem::new("guangzhou", "广州").description("华南区域"),
    ]
}
