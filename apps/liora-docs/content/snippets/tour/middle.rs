//! Start Tour from a specific step.

use gpui::App;
use liora_components::{Tour, TourStep};

pub fn show_middle_tour(cx: &mut App) {
    Tour::new(vec![
        TourStep::new("第一步", "介绍入口。"),
        TourStep::new("第二步", "当前步骤前后按钮都可用。"),
        TourStep::new("第三步", "结束引导。"),
    ])
    .active_index(1)
    .previous_text("上一步")
    .next_text("下一步")
    .finish_text("完成")
    .show(cx);
}
