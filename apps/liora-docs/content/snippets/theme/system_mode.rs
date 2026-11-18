use gpui::{App, Context, Entity, Render, Window, prelude::*};
use liora_components::{Button, Card, Space, Text, ThemeMode, Title, init_liora_with_mode};
use liora_core::Config;
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

// Retain the view in the host; its render reads the same Config as the shared live demo.
pub fn theme_preview(cx: &mut App) -> Entity<ThemePreview> {
    cx.new(|_| ThemePreview)
}

pub struct ThemePreview;

impl Render for ThemePreview {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let config = cx.global::<Config>();
        let mode = config.theme_mode;
        let theme = &config.theme;
        Card::new(
            Space::new()
                .vertical()
                .gap_md()
                .child(Title::new(format!("{} · {}", mode.label(), theme.name)).h4())
                .child(Text::new(format!("Theme mode: {}", mode.label())))
                .child(Text::new(match mode {
                    ThemeMode::System => {
                        "System resolves through GPUI WindowAppearance and observes system changes."
                    }
                    ThemeMode::Light => "Explicit light mode ignores later OS appearance changes.",
                    ThemeMode::Dark => "Explicit dark mode ignores later OS appearance changes.",
                }))
                .child(Button::new("Primary").primary())
                .child(Button::new("Disabled").primary().disabled(true)),
        )
    }
}
