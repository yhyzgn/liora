//! Empty module.
//!
//! This public module implements the Liora empty-state component for no-data and no-result surfaces. It keeps the reusable
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

use gpui::{AnyElement, App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

/// Public builder and render state for the Liora empty component.
pub struct Empty {
    image: Option<AnyElement>,
    description: SharedString,
    extra: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>>,
}

impl Empty {
    /// Creates a new value with the required baseline configuration.
    pub fn new() -> Self {
        Self {
            image: None,
            description: "暂无数据".into(),
            extra: None,
        }
    }

    /// Configures the image option.
    pub fn image(mut self, image: impl IntoElement) -> Self {
        self.image = Some(image.into_any_element());
        self
    }

    /// Configures the description option.
    pub fn description(mut self, d: impl Into<SharedString>) -> Self {
        self.description = d.into();
        self
    }

    /// Configures the extra option.
    pub fn extra<F>(mut self, f: F) -> Self
    where
        F: Fn(&mut Window, &mut App) -> AnyElement + 'static,
    {
        self.extra = Some(Box::new(f));
        self
    }
}

impl RenderOnce for Empty {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .w_full()
            .p_10()
            .gap_4()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(160.0))
                    .h(px(160.0))
                    .child(match self.image {
                        Some(img) => img,
                        None => Icon::new(IconName::PackageOpen)
                            .size(px(100.0))
                            .color(theme.neutral.text_3)
                            .into_any_element(),
                    }),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(theme.neutral.text_3)
                    .child(self.description),
            )
            .when_some(self.extra, |s, extra| {
                s.child(div().mt_2().child((extra)(window, cx)))
            })
    }
}

impl IntoElement for Empty {
    type Element = gpui::Component<Self>;
    fn into_element(self) -> Self::Element {
        gpui::Component::new(self)
    }
}
