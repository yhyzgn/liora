use gpui::{IntoElement, px};
use liora_components::{Tag, TagFlow};

pub fn tag_flow() -> impl IntoElement {
    // TagFlow 仍然使用 Tag 控件，只负责自动换行、间距和折叠策略。
    TagFlow::new([
        Tag::new("Design").round(true),
        Tag::new("GPUI").success().round(true),
        Tag::new("Animation").warning().round(true),
        Tag::new("Native Rust").danger().round(true),
        Tag::new("Charts").round(true),
        Tag::new("Docs").success().round(true),
        Tag::new("Installer").warning().round(true),
        Tag::new("Tray").round(true),
    ])
    .gap(px(10.0))
    .max_rows(2)
    .estimated_items_per_row(3)
    .overflow_indicator("更多")
}
