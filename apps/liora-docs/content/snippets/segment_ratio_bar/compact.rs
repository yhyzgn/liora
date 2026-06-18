use gpui::{IntoElement, px, rgb};
use liora_components::{SegmentRatioBar, SegmentRatioItem};

pub fn segment_ratio_bar_compact() -> impl IntoElement {
    SegmentRatioBar::new([
        SegmentRatioItem::new("API", 18.0, rgb(0x8b5cf6).into()),
        SegmentRatioItem::new("Web", 33.0, rgb(0x06b6d4).into()),
        SegmentRatioItem::new("Jobs", 29.0, rgb(0xf59e0b).into()),
        SegmentRatioItem::new("Other", 20.0, rgb(0x64748b).into()),
    ])
    .height(px(8.0))
    .radius(px(4.0))
    .rounded_segments(px(2.0))
    .legend_inset_x(px(14.0))
    .percentage_decimals(2)
}
