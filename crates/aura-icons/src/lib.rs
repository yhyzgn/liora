use gpui::{Hsla, Styled, prelude::*, px};

pub trait AuraIcon: IntoElement + Styled + Sized {
    fn icon_size(self, size: IconSize) -> Self {
        let px_size = size.px();
        self.size(px(px_size)).size(px(px_size))
    }

    fn icon_color(self, color: Hsla) -> Self {
        self.text_color(color)
    }
}

pub enum IconSize {
    Small,
    Default,
    Large,
}

impl IconSize {
    pub fn px(&self) -> f32 {
        match self {
            IconSize::Small => 14.0,
            IconSize::Default => 18.0,
            IconSize::Large => 24.0,
        }
    }
}

pub fn icon_check() -> impl IntoElement {
    gpui::div()
        .flex()
        .items_center()
        .justify_center()
        .child("✓")
}

pub fn icon_close() -> impl IntoElement {
    gpui::div()
        .flex()
        .items_center()
        .justify_center()
        .child("✕")
}

pub fn icon_chevron_down() -> impl IntoElement {
    gpui::div()
        .flex()
        .items_center()
        .justify_center()
        .child("▼")
}

pub fn icon_chevron_right() -> impl IntoElement {
    gpui::div()
        .flex()
        .items_center()
        .justify_center()
        .child("▶")
}

pub fn icon_search() -> impl IntoElement {
    gpui::div()
        .flex()
        .items_center()
        .justify_center()
        .child("🔍")
}

pub fn icon_star() -> impl IntoElement {
    gpui::div()
        .flex()
        .items_center()
        .justify_center()
        .child("★")
}

pub fn icon_info() -> impl IntoElement {
    gpui::div()
        .flex()
        .items_center()
        .justify_center()
        .child("ℹ")
}

pub fn icon_warning() -> impl IntoElement {
    gpui::div()
        .flex()
        .items_center()
        .justify_center()
        .child("⚠")
}

pub fn icon_error() -> impl IntoElement {
    gpui::div()
        .flex()
        .items_center()
        .justify_center()
        .child("✕")
}

pub fn icon_loading() -> impl IntoElement {
    gpui::div()
        .flex()
        .items_center()
        .justify_center()
        .child("⟳")
}
