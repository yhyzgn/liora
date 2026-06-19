//! Avatar module.
//!
//! This public module implements the Liora compact identity image/text fallback component. It keeps the reusable
//! component logic inside `liora-components` rather than Gallery or Docs so
//! downstream GPUI applications can compose the same behavior with their own
//! app state, assets, and release policy.
//!
//! ## Usage model
//!
//! Components in this module render native GPUI element trees. Stateless builder
//! values can be constructed inline, while controls with focus, selection,
//! popup, drag, or editing state should be stored as `gpui::Entity<T>` fields in
//! the parent view so state survives GPUI render passes.
//!
//! ## Design contract
//!
//! The implementation should use Liora theme tokens from `liora-core` and
//! `liora-theme`, keep accessibility-oriented keyboard/pointer behavior close to
//! the component, and avoid app-specific Gallery/Docs resources in this SDK
//! crate.

use gpui::{
    App, Component, Hsla, IntoElement, RenderOnce, SharedString, Window, div, img, prelude::*, px,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarShape {
    #[default]
    Circle,
    Square,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarSize {
    Small,
    #[default]
    Default,
    Large,
}

pub struct Avatar {
    src: Option<SharedString>,
    icon: Option<IconName>,
    size: AvatarSize,
    shape: AvatarShape,
    alt: Option<SharedString>,
    background: Option<Hsla>,
}

impl Avatar {
    pub fn new() -> Self {
        Self {
            src: None,
            icon: None,
            size: AvatarSize::Default,
            shape: AvatarShape::Circle,
            alt: None,
            background: None,
        }
    }

    pub fn src(mut self, src: impl Into<SharedString>) -> Self {
        self.src = Some(src.into());
        self
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }

    pub fn small(mut self) -> Self {
        self.size = AvatarSize::Small;
        self
    }

    pub fn large(mut self) -> Self {
        self.size = AvatarSize::Large;
        self
    }

    pub fn shape(mut self, shape: AvatarShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn square(mut self) -> Self {
        self.shape = AvatarShape::Square;
        self
    }

    pub fn alt(mut self, alt: impl Into<SharedString>) -> Self {
        self.alt = Some(alt.into());
        self
    }

    pub fn background(mut self, background: Hsla) -> Self {
        self.background = Some(background);
        self
    }
}

impl RenderOnce for Avatar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        let size_px = match self.size {
            AvatarSize::Small => px(24.0),
            AvatarSize::Default => px(40.0),
            AvatarSize::Large => px(56.0),
        };

        let radius = match self.shape {
            AvatarShape::Circle => size_px / 2.0,
            AvatarShape::Square => px(theme.radius.md),
        };

        let mut container = div()
            .flex()
            .items_center()
            .justify_center()
            .size(size_px)
            .rounded(radius)
            .bg(self.background.unwrap_or(theme.neutral.border))
            .overflow_hidden();

        if let Some(src) = self.src {
            container = container.child(img(src).size_full());
        } else if let Some(icon) = self.icon {
            container = container.child(
                Icon::new(icon)
                    .size(size_px * 0.6)
                    .color(theme.neutral.text_3),
            );
        } else {
            // Default icon if nothing provided
            container = container.child(
                Icon::new(IconName::User)
                    .size(size_px * 0.6)
                    .color(theme.neutral.text_3),
            );
        }

        container
    }
}

impl IntoElement for Avatar {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn avatar_background_tracks_custom_color() {
        let color = gpui::blue();

        assert_eq!(Avatar::new().background(color).background, Some(color));
    }
}
