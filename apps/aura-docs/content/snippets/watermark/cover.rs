use aura_components::Watermark;
use gpui::{IntoElement, div, prelude::*, px};

pub fn cover_watermark() -> impl IntoElement {
    Watermark::new(
        div().min_h(px(160.0)).p_4().child("Internal report"),
        "AURA CONFIDENTIAL",
    )
    .density(3, 4)
    .opacity(0.18)
}
