//! Spinner examples.

use gpui::{IntoElement, px};
use liora_components::{Space, Spinner};
use liora_icons_lucide::IconName;

pub fn spinner_basic() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_lg()
        .child(Spinner::new().small())
        .child(Spinner::new())
        .child(Spinner::new().large())
        .child(Spinner::new().icon(IconName::RefreshCw).size(px(20.0)))
}
