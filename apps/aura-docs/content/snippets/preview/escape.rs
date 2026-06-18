//! Preview can opt out of ESC and outside-click closing for controlled flows.

use aura_components::{Button, Preview};

pub fn preview_without_escape() -> Preview {
    let remote = "https://cube.elemecdn.com/6/94/4d3ea53c084bad6931a56d5158a48jpeg.jpeg";

    Preview::new(remote)
        .close_on_escape(false)
        .close_on_click_outside(false)
        .hover_effect(false)
        .child(Button::new("打开预览（ESC / 外部点击不关闭）").primary())
}

fn main() {
    let _ = preview_without_escape();
}
