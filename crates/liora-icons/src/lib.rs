//! Native GPUI icon rendering helpers for Liora.
//!
//! This crate exposes an `Icon` component plus the `IntoIconPath` trait used by
//! Liora demos and docs. Icons are rendered as GPUI SVG/image assets and do not
//! require a web icon runtime.

use gpui::{
    App, Component, DefiniteLength, Hsla, IntoElement, Radians, RenderOnce, SharedString,
    Transformation, Window, prelude::*, px,
};
use liora_core::Config;
use std::borrow::Cow;

pub trait IntoIconPath {
    fn icon_path(&self) -> Cow<'static, str>;
}

impl IntoIconPath for &str {
    fn icon_path(&self) -> Cow<'static, str> {
        Cow::Owned(self.to_string())
    }
}
impl IntoIconPath for String {
    fn icon_path(&self) -> Cow<'static, str> {
        Cow::Owned(self.clone())
    }
}

pub struct Icon {
    size: Option<DefiniteLength>,
    color: Option<Hsla>,
    group_hover_color: Option<(SharedString, Hsla)>,
    rotation: Option<Radians>,
    asset_path: String,
}

impl Icon {
    pub fn new(path: impl IntoIconPath) -> Self {
        Self {
            size: None,
            color: None,
            group_hover_color: None,
            rotation: None,
            asset_path: path.icon_path().into_owned(),
        }
    }

    pub fn size(mut self, sz: impl Into<DefiniteLength>) -> Self {
        self.size = Some(sz.into());
        self
    }

    pub fn size_xs(self) -> Self {
        self.size(px(12.0))
    }

    pub fn size_md(self) -> Self {
        self.size(px(18.0))
    }

    pub fn size_lg(self) -> Self {
        self.size(px(24.0))
    }

    pub fn size_xl(self) -> Self {
        self.size(px(32.0))
    }

    /// Set explicit color. If not called, inherits parent's text_color.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    /// Change icon color when a parent/group member is hovered.
    pub fn group_hover_color(mut self, group: impl Into<SharedString>, color: Hsla) -> Self {
        self.group_hover_color = Some((group.into(), color));
        self
    }

    /// Rotate the icon around its center while preserving layout and hitbox.
    pub fn rotation(mut self, rotation: Radians) -> Self {
        self.rotation = Some(rotation);
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

        let sz = self.size.unwrap_or_else(|| px(18.0).into());
        let mut el = gpui::svg().external_path(self.asset_path).size(sz);
        if let Some(color) = self.color {
            el = el.text_color(color);
        } else {
            el = el.text_color(theme.neutral.icon);
        }
        if let Some((group, color)) = self.group_hover_color {
            el = el.group_hover(group, move |style| style.text_color(color));
        }
        if let Some(rotation) = self.rotation {
            el = el.with_transformation(Transformation::rotate(rotation));
        }
        el
    }
}

impl IntoElement for Icon {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn icon_size_helpers_set_common_demo_sizes() {
        assert_eq!(Icon::new("home").size_xs().size, Some(px(12.0).into()));
        assert_eq!(Icon::new("home").size_md().size, Some(px(18.0).into()));
        assert_eq!(Icon::new("home").size_lg().size, Some(px(24.0).into()));
        assert_eq!(Icon::new("home").size_xl().size, Some(px(32.0).into()));
    }

    #[test]
    fn icon_rotation_tracks_transform_request() {
        assert_eq!(
            Icon::new("loader").rotation(gpui::radians(1.0)).rotation,
            Some(gpui::radians(1.0))
        );
    }
}
