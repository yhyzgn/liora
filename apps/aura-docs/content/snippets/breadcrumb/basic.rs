//! Basic Breadcrumb with the default slash separator.

use aura_components::{Breadcrumb, BreadcrumbItem};

pub fn basic_breadcrumb() -> Breadcrumb {
    Breadcrumb::new()
        .item(BreadcrumbItem::new("首页"))
        .item(BreadcrumbItem::new("活动管理"))
        .item(BreadcrumbItem::new("活动列表"))
        .item(BreadcrumbItem::new("活动详情"))
}
