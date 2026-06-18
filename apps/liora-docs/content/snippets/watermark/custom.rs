use gpui::{IntoElement, div, prelude::*, px, rgb};
use liora_components::Watermark;

pub fn custom_watermark() -> impl IntoElement {
    Watermark::new(div().min_h(px(160.0)).p_4().child("Design draft"), "DRAFT")
        .density(4, 5)
        .gap(px(72.0), px(48.0))
        .color(rgb(0xf97316).into())
        .opacity(0.22)
        .rotate(-32.0)
}
