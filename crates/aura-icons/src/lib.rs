use gpui::{prelude::*, px, Hsla, IntoElement};
use std::borrow::Cow;

pub trait IntoIconPath {
    fn icon_path(&self) -> Cow<'static, str>;
}

impl IntoIconPath for &str {
    fn icon_path(&self) -> Cow<'static, str> { Cow::Owned(self.to_string()) }
}
impl IntoIconPath for String {
    fn icon_path(&self) -> Cow<'static, str> { Cow::Owned(self.clone()) }
}

pub struct AuraIcon {
    size: Option<f32>,
    color: Option<Hsla>,
    asset_path: String,
}

impl AuraIcon {
    pub fn new(path: impl IntoIconPath) -> Self {
        Self { size: None, color: None, asset_path: path.icon_path().into_owned() }
    }

    pub fn size(mut self, px_size: f32) -> Self { self.size = Some(px_size); self }
    pub fn color(mut self, color: Hsla) -> Self { self.color = Some(color); self }

    pub fn build(self, theme: &aura_theme::AuraTheme) -> impl IntoElement {
        let sz = self.size.unwrap_or(18.0);
        let color = self.color.unwrap_or(theme.neutral.icon);

        gpui::svg()
            .external_path(self.asset_path)
            .size(px(sz))
            .text_color(color)
    }
}
