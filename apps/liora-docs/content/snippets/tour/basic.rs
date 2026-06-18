//! Start a top-level Tour overlay.

use gpui::App;
use liora_components::{Tour, TourPlacement, TourStep};

pub fn show_basic_tour(cx: &mut App) {
    Tour::new(vec![
        TourStep::new("选择组件", "从左侧菜单进入组件示例。")
            .target("Gallery menu")
            .placement(TourPlacement::Right),
        TourStep::new("查看效果", "右侧展示实际原生控件。")
            .target("Preview panel")
            .placement(TourPlacement::Bottom),
    ])
    .on_change(|index, _window, _cx| {
        let _current_step = index + 1;
    })
    .show(cx);
}
