use gpui::{App, Window};
use liora_components::{ThemeMode, init_liora_with_mode};
use liora_core::{apply_theme_mode, attach_system_theme_observer};

pub fn init_app_theme(cx: &mut App) {
    // 推荐默认跟随系统外观。
    init_liora_with_mode(cx, ThemeMode::System);
}

pub fn observe_system_theme(window: &mut Window, cx: &mut App) {
    // 在创建 root view 前调用：先同步真实窗口外观，避免首帧主题闪烁，再保活 observer。
    attach_system_theme_observer(window, cx);
}

pub fn switch_to_dark(window: &mut Window, cx: &mut App) {
    // 用户显式选择 dark 后，不再跟随后续系统外观变化。
    apply_theme_mode(window, cx, ThemeMode::Dark);
}
