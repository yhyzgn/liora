//! Bottom Affix fixed near the viewport bottom.

use gpui::{IntoElement, px};
use liora_components::{Affix, AffixPosition, Button};

pub fn bottom_affix() -> Affix {
    Affix::new()
        .position(AffixPosition::Bottom)
        .offset(px(20.0))
        .content(|_, _| {
            Button::new("固钉在距离底部 20px 的位置")
                .success()
                .into_any_element()
        })
}
