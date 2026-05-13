//! Breadcrumb with a custom text separator.

use aura_components::{Breadcrumb, BreadcrumbItem};

pub fn text_separator_breadcrumb() -> Breadcrumb {
    Breadcrumb::new()
        .separator(">")
        .item(BreadcrumbItem::new("首页"))
        .item(BreadcrumbItem::new("推广管理"))
        .item(BreadcrumbItem::new("推广列表"))
        .item(BreadcrumbItem::new("推广详情"))
}
