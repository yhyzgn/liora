//! Full-screen loading overlay configuration.

use liora_components::Loading;

pub fn fullscreen_loading() -> Loading {
    Loading::new().text("Preparing workspace...").full_screen()
}
