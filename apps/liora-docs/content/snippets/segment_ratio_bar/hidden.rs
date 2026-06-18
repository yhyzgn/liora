use gpui::{IntoElement, px, rgb};
use liora_components::{SegmentRatioBar, SegmentRatioItem};

pub fn segment_ratio_bar_hidden() -> impl IntoElement {
    SegmentRatioBar::new([
        SegmentRatioItem::new("Direct", 42.0, rgb(0x3b82f6).into()),
        SegmentRatioItem::new("Proxy", 51.0, rgb(0x22c55e).into()),
        SegmentRatioItem::new("Reject", 7.0, rgb(0xef4444).into()),
    ])
    .hide_legend()
    .height(px(18.0))
    .radius(px(9.0))
    .segment_radius(px(4.0))
}
