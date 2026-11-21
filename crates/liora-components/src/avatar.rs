//! Avatar module.
//!
//! This public module implements the Liora compact identity image/text fallback component. It keeps the reusable
//! component logic inside `liora-components` rather than host applications so
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
//! the component, and avoid app-specific host-application resources in this SDK
//! crate.

use gpui::{
    App, Hsla, IntoElement, RenderOnce, SharedString, StyledImage, ViewElement as Component,
    Window, div, img, prelude::*, px,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control avatar shape behavior.
pub enum AvatarShape {
    #[default]
    /// Uses circular geometry.
    Circle,
    /// Uses square geometry.
    Square,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control avatar size behavior.
pub enum AvatarSize {
    /// Uses compact sizing metrics.
    Small,
    #[default]
    /// Uses the default neutral treatment.
    Default,
    /// Uses expanded sizing metrics.
    Large,
}

/// Fluent native GPUI component for rendering Liora avatar.
pub struct Avatar {
    src: Option<SharedString>,
    icon: Option<IconName>,
    size: AvatarSize,
    shape: AvatarShape,
    alt: Option<SharedString>,
    background: Option<Hsla>,
}

impl Avatar {
    /// Creates `Avatar` with default theme-driven styling and no optional callbacks attached.
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

    /// Sets the image or preview source.
    pub fn src(mut self, src: impl Into<SharedString>) -> Self {
        self.src = Some(src.into());
        self
    }

    /// Sets the tray icon configuration value.
    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Sets an explicit icon size while preserving the default color behavior.
    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }

    /// Uses the compact size preset.
    pub fn small(mut self) -> Self {
        self.size = AvatarSize::Small;
        self
    }

    /// Uses the large size preset.
    pub fn large(mut self) -> Self {
        self.size = AvatarSize::Large;
        self
    }

    /// Selects the avatar or image shape.
    pub fn shape(mut self, shape: AvatarShape) -> Self {
        self.shape = shape;
        self
    }

    /// Uses square shape or clipping.
    pub fn square(mut self) -> Self {
        self.shape = AvatarShape::Square;
        self
    }

    /// Sets alternate text used when an image source is unavailable.
    pub fn alt(mut self, alt: impl Into<SharedString>) -> Self {
        self.alt = Some(alt.into());
        self
    }

    /// Toggles or applies the component background treatment.
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
            let mut image = img(src).size_full();
            if let Some(alt) = self.alt {
                let color = theme.neutral.text_3;
                image = image.with_fallback(move || {
                    let fallback = div()
                        .size_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(color);
                    #[cfg(test)]
                    let fallback = fallback.debug_selector(|| alt.to_string());
                    fallback.child(alt.clone()).into_any_element()
                });
            }
            container = container.child(image);
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

    #[cfg(debug_assertions)]
    #[test]
    fn avatar_displays_alt_when_image_fails_to_load() {
        struct AvatarProbe;

        impl gpui::Render for AvatarProbe {
            fn render(&mut self, _: &mut Window, _: &mut gpui::Context<Self>) -> impl IntoElement {
                Avatar::new()
                    .src("https://example.invalid/avatar.png")
                    .alt("AB")
            }
        }

        // GPUI's test context supplies a fake HTTP client that returns 404.
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let handle = app.add_window(|_, _| AvatarProbe);
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        window.update(|window, cx| window.draw(cx).clear(cx));
        app.run_until_parked();
        window.update(|window, cx| window.draw(cx).clear(cx));

        assert_eq!(
            window.debug_bounds("AB").map(|bounds| bounds.size),
            Some(gpui::size(px(40.0), px(40.0))),
            "failed image should lay out its alt fallback in the avatar"
        );
        app.quit();
    }

    #[test]
    fn avatar_background_tracks_custom_color() {
        let color = gpui::blue();

        assert_eq!(Avatar::new().background(color).background, Some(color));
    }
}
