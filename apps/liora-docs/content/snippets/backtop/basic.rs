//! Basic Backtop bound to a scroll handle.

use gpui::ScrollHandle;
use liora_components::Backtop;

pub fn basic_backtop(scroll_handle: ScrollHandle) -> Backtop {
    Backtop::new(scroll_handle)
        .id("docs-backtop-basic")
        .visibility_height_sm()
}
