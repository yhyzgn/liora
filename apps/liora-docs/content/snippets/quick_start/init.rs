//! Minimal Liora application bootstrap.

use gpui::{App, Application};
use liora_components::{ThemeMode, init_liora, init_liora_with_mode};

fn main() {
    Application::new().run(|cx: &mut App| {
        // Default: follow the operating system theme and initialize component runtime.
        init_liora(cx);

        // Optional: choose an explicit startup mode instead.
        // init_liora_with_mode(cx, ThemeMode::Dark);

        // Open your first GPUI window here with cx.open_window(...).
    });
}
