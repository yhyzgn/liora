//! Breadcrumb items can execute native callbacks.

use aura_components::{Breadcrumb, BreadcrumbItem, toast_info};

pub fn clickable_breadcrumb() -> Breadcrumb {
    Breadcrumb::new()
        .item(BreadcrumbItem::new("首页").on_click(|_, _| toast_info!("Home Clicked")))
        .item(BreadcrumbItem::new("推广管理").on_click(|_, _| toast_info!("Management Clicked")))
        .item(BreadcrumbItem::new("推广列表"))
}
