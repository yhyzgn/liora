use gpui::{IntoElement, px};
use liora_components::Progress;

pub fn progress_circle_gradient() -> impl IntoElement {
    // 环形进度支持渐变色，并可指定完成时最终颜色。
    Progress::new(100.0)
        .circle()
        .circle_size(px(148.0))
        .ring_width(px(12.0))
        .gradient(vec![gpui::blue(), gpui::green()])
        .complete_color(gpui::green())
        .inner_color(gpui::white().opacity(0.88))
        .center_text("Done")
        .text_color(gpui::green())
        .text_size(px(22.0))
}
