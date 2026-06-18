use aura_core::{ThemeMode, apply_theme_mode, init_aura_with_mode, sync_system_theme};
use gpui::{App, Window};

pub fn init_app_theme(cx: &mut App) {
    // 推荐默认跟随系统外观。
    init_aura_with_mode(cx, ThemeMode::System);
}

pub fn observe_system_theme(window: &mut Window, _cx: &mut App) {
    // 窗口创建后注册；System 模式下会自动跟随 OS light/dark 变化。
    let _ = window.observe_window_appearance(|window, cx| sync_system_theme(window, cx));
}

pub fn switch_to_dark(window: &mut Window, cx: &mut App) {
    // 用户显式选择 dark 后，不再跟随后续系统外观变化。
    apply_theme_mode(window, cx, ThemeMode::Dark);
}
