use gpui::{IntoElement, rgb};
use liora_components::{SegmentRatioBar, SegmentRatioItem};

pub fn segment_ratio_bar_bottom() -> impl IntoElement {
    SegmentRatioBar::new([
        SegmentRatioItem::new("Direct", 42.0, rgb(0x3b82f6).into()),
        SegmentRatioItem::new("Proxy", 51.0, rgb(0x22c55e).into()),
        SegmentRatioItem::new("Reject", 7.0, rgb(0xef4444).into()),
    ])
    .radius(gpui::px(8.0))
    .segment_radius(gpui::px(3.0))
    .legend_inset_x(gpui::px(8.0))
    .percentage_decimals(0)
    .split_legend(true)
}
