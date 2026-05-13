//! Pagination with total, page-size picker, pager, and jumper sections.

use aura_components::{Pagination, toast_info};

pub fn advanced_pagination() -> Pagination {
    Pagination::new(400)
        .id("docs-pagination-advanced")
        .page_size(20)
        .page_sizes(vec![10, 20, 50, 100])
        .background(true)
        .layout("total, sizes, prev, pager, next, jumper")
        .on_change(|page, _, _| toast_info!("Page changed to: {}", page))
        .on_page_size_change(|size, _, _| toast_info!("Page size changed to: {}", size))
}
