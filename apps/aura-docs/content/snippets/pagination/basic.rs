//! Basic Pagination with page change callback.

use aura_components::{Pagination, toast_info};

pub fn basic_pagination() -> Pagination {
    Pagination::new(50)
        .id("docs-pagination-basic")
        .on_change(|page, _, _| toast_info!("Page changed to: {}", page))
}
