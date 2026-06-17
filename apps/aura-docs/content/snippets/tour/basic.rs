use aura_components::{Tour, TourPlacement, TourStep};
use gpui::IntoElement;

pub fn basic_tour() -> impl IntoElement {
    Tour::new(vec![
        TourStep::new("选择组件", "从左侧菜单进入组件示例。")
            .target("Gallery menu")
            .placement(TourPlacement::Right),
        TourStep::new("查看效果", "右侧展示实际原生控件。"),
    ])
    .active_index(0)
}
