//! Pagination with background button style.

use liora_components::{Pagination, toast_info};

pub fn background_pagination() -> Pagination {
    Pagination::new(100)
        .id("docs-pagination-background")
        .background(true)
        .on_change(|page, _, _| toast_info!("Page changed to: {}", page))
}
