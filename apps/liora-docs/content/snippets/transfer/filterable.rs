//! Filterable Transfer with prefilled filters.

use liora_components::{Transfer, TransferItem};

pub fn transfer_filterable() -> Transfer {
    Transfer::new(role_items())
        .titles("全部角色", "已授权")
        .filterable(true)
        .source_filter("admin")
        .target_filter("ops")
        .target_keys(["ops"])
        .checked_source_keys(["admin"])
        .width_lg()
}

fn role_items() -> Vec<TransferItem> {
    vec![
        TransferItem::new("admin", "Admin 管理员").description("admin / full access"),
        TransferItem::new("editor", "Editor 编辑").description("content write"),
        TransferItem::new("viewer", "Viewer 只读").description("read only"),
        TransferItem::new("ops", "Ops 运维").description("ops / deploy"),
        TransferItem::new("auditor", "Auditor 审计").description("compliance"),
    ]
}
