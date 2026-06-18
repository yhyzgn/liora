//! Start a top-level Tour without the dimmed mask.

use gpui::App;
use liora_components::{Tour, TourStep};

pub fn show_no_mask_tour(cx: &mut App) {
    Tour::new(vec![TourStep::new(
        "透明遮罩",
        "Tour 仍然在顶层浮动，但不显示半透明遮罩。",
    )])
    .show_mask(false)
    .close_on_click_outside(true)
    .finish_text("完成")
    .show(cx);
}
