//! Breadcrumb item with an icon.

use liora_components::{Breadcrumb, BreadcrumbItem};
use liora_icons_lucide::IconName;

pub fn icon_breadcrumb() -> Breadcrumb {
    Breadcrumb::new()
        .item(BreadcrumbItem::new("首页").icon(IconName::House))
        .item(BreadcrumbItem::new("推广管理"))
        .item(BreadcrumbItem::new("推广列表"))
        .item(BreadcrumbItem::new("推广详情"))
}
