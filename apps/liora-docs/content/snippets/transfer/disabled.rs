//! Transfer disabled items.

use liora_components::{Transfer, TransferItem};

pub fn transfer_disabled_items() -> Transfer {
    Transfer::new(vec![
        TransferItem::new("beijing", "北京"),
        TransferItem::new("shanghai", "上海"),
        TransferItem::new("disabled", "成都（禁用）")
            .description("不可移动")
            .disabled(true),
    ])
    .titles("源列表", "目标列表")
    .target_keys(["disabled"])
    .checked_target_keys(["disabled"])
}
