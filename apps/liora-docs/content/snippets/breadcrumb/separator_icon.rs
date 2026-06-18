//! Breadcrumb with an icon separator.

use liora_components::{Breadcrumb, BreadcrumbItem};
use liora_icons_lucide::IconName;

pub fn icon_separator_breadcrumb() -> Breadcrumb {
    Breadcrumb::new()
        .separator_icon(IconName::ChevronRight)
        .item(BreadcrumbItem::new("首页"))
        .item(BreadcrumbItem::new("推广管理"))
        .item(BreadcrumbItem::new("推广列表"))
        .item(BreadcrumbItem::new("推广详情"))
}
