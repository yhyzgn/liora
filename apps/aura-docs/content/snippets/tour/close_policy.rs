//! Start a controlled Tour that only closes through explicit actions.

use aura_components::{Tour, TourStep};
use gpui::App;

pub fn show_controlled_close_tour(cx: &mut App) {
    Tour::new(vec![
        TourStep::new("确认引导", "ESC 和外部点击都不会关闭。"),
        TourStep::new("显式完成", "用户需要点击关闭图标或完成按钮。"),
    ])
    .close_on_escape(false)
    .close_on_click_outside(false)
    .finish_text("我已了解")
    .show(cx);
}
