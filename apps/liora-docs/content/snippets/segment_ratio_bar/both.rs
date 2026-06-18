use gpui::{IntoElement, px, rgb};
use liora_components::{SegmentRatioBar, SegmentRatioItem};

pub fn segment_ratio_bar_both() -> impl IntoElement {
    SegmentRatioBar::new([
        SegmentRatioItem::new("Direct", 42.0, rgb(0x3b82f6).into()),
        SegmentRatioItem::new("Proxy", 51.0, rgb(0x22c55e).into()),
        SegmentRatioItem::new("Reject", 7.0, rgb(0xef4444).into()),
    ])
    .legend_both()
    .height(px(14.0))
    .radius(px(7.0))
    .segment_radius(px(3.0))
    .legend_text_inset(px(8.0))
    .percentage_decimals(1)
}
