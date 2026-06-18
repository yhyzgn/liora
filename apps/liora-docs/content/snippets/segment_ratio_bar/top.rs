use gpui::{IntoElement, rgb};
use liora_components::{SegmentLegendPosition, SegmentRatioBar, SegmentRatioItem};

pub fn segment_ratio_bar_top() -> impl IntoElement {
    // 文本可以放在上方，也可用 legend_both 同时展示上下文本。
    SegmentRatioBar::new([
        SegmentRatioItem::new("Direct", 42.0, rgb(0x3b82f6).into()),
        SegmentRatioItem::new("Proxy", 51.0, rgb(0x22c55e).into()),
        SegmentRatioItem::new("Reject", 7.0, rgb(0xef4444).into()),
    ])
    .legend_position(SegmentLegendPosition::Top)
    .radius(gpui::px(8.0))
    .rounded_segments(gpui::px(4.0))
    .legend_inset_x(gpui::px(10.0))
    .percentage_decimals(1)
}
