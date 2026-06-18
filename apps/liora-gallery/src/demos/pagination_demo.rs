use gpui::{AnyView, App, Context, Entity, Render, Window, prelude::*};
use liora_components::{Card, Pagination, Space};

use liora_components::layout_helpers::{page, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| PaginationDemo {
        basic: cx.new(|_| {
            Pagination::new(50)
                .id("pagination-demo-basic")
                .on_change(|page, _, _| println!("Page changed to: {}", page))
        }),
        background: cx.new(|_| {
            Pagination::new(100)
                .id("pagination-demo-background")
                .background(true)
                .on_change(|page, _, _| println!("Page changed to: {}", page))
        }),
        page_sizes: cx.new(|_| {
            Pagination::new(400)
                .id("pagination-demo-page-sizes")
                .page_size(20)
                .page_sizes(vec![10, 20, 50, 100])
                .background(true)
                .layout("total, sizes, prev, pager, next, jumper")
                .on_change(|page, _, _| println!("Page changed to: {}", page))
                .on_page_size_change(|size, _, _| println!("Page size changed to: {}", size))
        }),
    })
    .into()
}

struct PaginationDemo {
    basic: Entity<Pagination>,
    background: Entity<Pagination>,
    page_sizes: Entity<Pagination>,
}

impl Render for PaginationDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Pagination 分页",
            "当数据量过多时，使用分页分解数据。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "基础用法",
                    "基础分页控制。",
                    Card::new(self.basic.clone()),
                ))
                .child(section(
                    "带有背景色的分页",
                    "使用背景样式突出页码按钮。",
                    Card::new(self.background.clone()),
                ))
                .child(section(
                    "附加功能 (Total, Sizes, Jumper)",
                    "组合总数、页尺寸和跳转输入。",
                    Card::new(self.page_sizes.clone()),
                )),
        )
    }
}
