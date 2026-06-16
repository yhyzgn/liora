use aura_components::Watermark;
use gpui::{IntoElement, div, prelude::*, px, rgb};

pub fn header_watermark() -> impl IntoElement {
    Watermark::new(
        div().min_h(px(160.0)).p_4().child("Preview asset"),
        "PREVIEW",
    )
    .header()
    .density(1, 3)
    .color(rgb(0x2563eb).into())
    .opacity(0.24)
}
