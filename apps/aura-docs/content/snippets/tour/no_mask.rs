use aura_components::{Tour, TourStep};
use gpui::IntoElement;

pub fn no_mask_tour() -> impl IntoElement {
    Tour::new(vec![TourStep::new("简洁模式", "不显示强调边框背景。")])
        .show_mask(false)
        .finish_text("完成")
}
