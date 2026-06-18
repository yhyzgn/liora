use gpui::{IntoElement, div, prelude::*, px};
use liora_components::Watermark;

pub fn cover_watermark() -> impl IntoElement {
    Watermark::new(
        div().min_h(px(160.0)).p_4().child("Internal report"),
        "LIORA CONFIDENTIAL",
    )
    .density(3, 4)
    .opacity(0.18)
}
