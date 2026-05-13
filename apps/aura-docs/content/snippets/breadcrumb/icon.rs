//! Breadcrumb item with an icon.

use aura_components::{Breadcrumb, BreadcrumbItem};
use aura_icons_lucide::IconName;

pub fn icon_breadcrumb() -> Breadcrumb {
    Breadcrumb::new()
        .item(BreadcrumbItem::new("首页").icon(IconName::House))
        .item(BreadcrumbItem::new("推广管理"))
        .item(BreadcrumbItem::new("推广列表"))
        .item(BreadcrumbItem::new("推广详情"))
}
