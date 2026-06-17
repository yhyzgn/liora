use aura_components::{Tour, TourStep};
use gpui::IntoElement;

pub fn middle_tour() -> impl IntoElement {
    Tour::new(vec![
        TourStep::new("第一步", "介绍入口。"),
        TourStep::new("第二步", "当前步骤前后按钮都可用。"),
        TourStep::new("第三步", "结束引导。"),
    ])
    .active_index(1)
}
