use gpui::{App, Context, Entity, IntoElement, Render, Window, prelude::*, px};
use liora_components::layout_helpers::{page, section};
use liora_components::splitter::SplitterOrientation;
use liora_components::{Card, Splitter, Text};
use std::{cell::Cell, rc::Rc};

pub fn render(cx: &mut App) -> Entity<SplitterDemo> {
    cx.new(|_| SplitterDemo {
        size: Rc::new(Cell::new(px(150.0))),
    })
}

pub struct SplitterDemo {
    size: Rc<Cell<gpui::Pixels>>,
}
impl Render for SplitterDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let size = self.size.clone();
        let terminal_label = format!("Terminal · top pane: {:.0}px", self.size.get() / px(1.));
        page(
            "Splitter 分隔面板",
            "拖动分隔条，或点击后使用方向键、Shift+方向键、Home/End。",
            gpui::div()
                .flex()
                .flex_col()
                .gap_4()
                .child(section(
                    "持久拖动与键盘调整",
                    "重绘保留尺寸；第一面板 100–400px。",
                    Splitter::new()
                        .id("gallery-splitter-horizontal")
                        .height_md()
                        .bordered()
                        .default_size(px(180.))
                        .min_size(px(100.))
                        .max_size(px(400.))
                        .left(Card::new(Text::new("Explorer")).no_shadow())
                        .right(Card::new(Text::new("Host content")).no_shadow()),
                ))
                .child(section(
                    "垂直受控尺寸",
                    "回调更新宿主尺寸，方向键使用 ↑ / ↓。",
                    Splitter::new()
                        .id("gallery-splitter-vertical")
                        .height(px(320.))
                        .bordered()
                        .orientation(SplitterOrientation::Vertical)
                        .min_size(px(80.))
                        .max_size(px(220.))
                        .size(size.get())
                        .on_change(move |value| size.set(value))
                        .left(Text::new("Editor"))
                        .right(Text::new(terminal_label)),
                )),
        )
    }
}
