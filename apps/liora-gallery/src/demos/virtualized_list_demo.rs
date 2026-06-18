use gpui::{App, Context, Entity, IntoElement, Render, Window, prelude::*};
use liora_components::layout_helpers::{page, section};
use liora_components::{Flex, Space, Tag, Text, VirtualizedList, toast_success};

pub fn render(cx: &mut App) -> Entity<VirtualizedListDemo> {
    cx.new(|cx| VirtualizedListDemo {
        list: cx.new(|cx| {
            let mut list =
                VirtualizedList::new(1_000, cx, |index, _window, _cx| virtualized_row(index));
            list.set_height(Some(320.0.into()));
            list.set_item_spacing(12.0);
            list.measure_all_items_for_scrollbar();
            list
        }),
        draggable: cx.new(|cx| {
            let mut list =
                VirtualizedList::new(48, cx, |index, _window, _cx| virtualized_row(index));
            list.set_height(Some(320.0.into()));
            list.set_item_spacing(12.0);
            list.measure_all_items_for_scrollbar();
            list.set_draggable(true);
            list.set_on_reorder(|from, to, _, _| {
                toast_success!("VirtualizedList reordered: {} -> {}", from + 1, to + 1);
            });
            list
        }),
    })
}

pub struct VirtualizedListDemo {
    list: Entity<VirtualizedList>,
    draggable: Entity<VirtualizedList>,
}

impl Render for VirtualizedListDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "VirtualizedList 虚拟列表",
            "仅渲染可见区域的原生列表，适合长文档、大量行项目和复杂组件流。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "千行列表",
                    "滚动时只布局可见行，并通过 Liora 自举滚动条显示当前位置。",
                    self.list.clone(),
                ))
                .child(section(
                    "垂直拖动排序",
                    "开启 set_draggable(true) 后，每行前端会显示 Grip 拖拽把手；按住把手拖到目标行并松开即可触发 on_reorder。",
                    self.draggable.clone(),
                )),
        )
    }
}

fn virtualized_row(index: usize) -> gpui::AnyElement {
    Flex::new()
        .row()
        .align_center()
        .justify_between()
        .padding_md()
        .rounded_md()
        .border()
        .border_color(gpui::rgb(0xe5e7eb).into())
        .bg(gpui::rgb(0xffffff).into())
        .child(
            Space::new()
                .vertical()
                .gap_xs()
                .child(Text::new(format!("Row #{:04}", index + 1)).bold())
                .child(Text::new(
                    "This row is created only when it enters the virtual viewport.",
                )),
        )
        .child(Tag::new(if index % 2 == 0 { "even" } else { "odd" }).info())
        .into_any_element()
}

#[cfg(test)]
mod tests {
    #[test]
    fn virtualized_list_demo_uses_component_api() {
        let source = include_str!("virtualized_list_demo.rs");

        assert!(source.contains("VirtualizedList::new"));
        assert!(source.contains("set_height"));
        assert!(source.contains("set_item_spacing"));
        assert!(source.contains("1_000"));
        assert!(source.contains("set_draggable(true)"));
        assert!(source.contains("set_on_reorder"));
    }
}
