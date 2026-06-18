//! Minimal Liora application bootstrap.

use gpui::App;
use liora_core::init_liora;
use liora_theme::Theme;

fn main() {
    gpui_platform::application().run(|cx: &mut App| {
        init_liora(cx, Theme::light());
        // Open your first GPUI window here with cx.open_window(...).
    });
}
