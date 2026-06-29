//! Segmented module.
//!
//! This public module implements the Liora segmented control for compact mutually exclusive choices. It keeps the reusable
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

use crate::gpui_compat::element_id;
use crate::motion::pop_in;
use gpui::{App, Context, IntoElement, Render, SharedString, Window, div, prelude::*, px};
use liora_core::{Config, LocalizedText};

/// Fluent native GPUI component for rendering Liora segmented option.
pub struct SegmentedOption {
    /// User-facing label rendered for this item.
    pub label: LocalizedText,
    /// Machine-readable value represented by this item.
    pub value: SharedString,
    /// Whether user interaction is disabled for this item.
    pub disabled: bool,
}

impl SegmentedOption {
    /// Creates `SegmentedOption` initialized from the supplied label, and value.
    pub fn new(label: impl Into<LocalizedText>, value: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            disabled: false,
        }
    }

    /// Toggles the disabled state and suppresses user interaction when enabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Fluent native GPUI component for rendering Liora segmented.
pub struct Segmented {
    id: SharedString,
    options: Vec<SegmentedOption>,
    value: Option<SharedString>,
    block: bool,
    on_change: Option<Box<dyn Fn(SharedString, &mut Window, &mut App) + 'static>>,
}

impl Segmented {
    /// Creates `Segmented` that renders the supplied options collection.
    pub fn new(options: Vec<SegmentedOption>) -> Self {
        let first_value = options.first().map(|o| o.value.clone());
        Self {
            id: liora_core::unique_id("segmented"),
            options,
            value: first_value,
            block: false,
            on_change: None,
        }
    }

    /// Assigns a stable element id used by GPUI state, hit testing, and automated interaction tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Returns the serialized value used by forms, configuration, or persistence.
    pub fn value(mut self, val: impl Into<SharedString>) -> Self {
        self.value = Some(val.into());
        self
    }

    /// Makes the component occupy the available inline width.
    pub fn block(mut self, block: bool) -> Self {
        self.block = block;
        self
    }

    /// Registers a callback that runs when change occurs.
    pub fn on_change(mut self, f: impl Fn(SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    /// Updates the stored on change value and keeps the existing component identity.
    pub fn set_on_change(&mut self, f: impl Fn(SharedString, &mut Window, &mut App) + 'static) {
        self.on_change = Some(Box::new(f));
    }

    /// Replaces options while preserving the selected value when possible.
    pub fn set_options(&mut self, options: Vec<SegmentedOption>) {
        let current = self.value.clone();
        self.value = current
            .filter(|value| options.iter().any(|option| &option.value == value))
            .or_else(|| options.first().map(|option| option.value.clone()));
        self.options = options;
    }

    fn select_option(&mut self, value: SharedString, window: &mut Window, cx: &mut Context<Self>) {
        if Some(&value) != self.value.as_ref() {
            self.value = Some(value.clone());
            if let Some(ref on_change) = self.on_change {
                (on_change)(value, window, cx);
            }
            cx.notify();
        }
    }
}

impl Render for Segmented {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        div()
            .flex()
            .flex_row()
            .items_center()
            .p(px(2.0))
            .gap(px(2.0))
            .bg(theme.neutral.hover)
            .rounded(px(theme.radius.md))
            .when(self.block, |s| s.w_full())
            .children(self.options.iter().enumerate().map(|(i, opt)| {
                let is_active = self.value.as_ref() == Some(&opt.value);
                let value = opt.value.clone();
                let disabled = opt.disabled;

                let option = div()
                    .id(element_id(format!("{}-option-{}", self.id, i)))
                    .flex()
                    .items_center()
                    .justify_center()
                    .h(px(28.0))
                    .px_3()
                    .rounded(px(theme.radius.sm))
                    .when(self.block, |s| s.flex_1())
                    .when(is_active, |s| {
                        s.bg(theme.neutral.card)
                            .shadow_sm()
                            .text_color(theme.neutral.text_1)
                            .font_weight(gpui::FontWeight::BOLD)
                    })
                    .when(!is_active && !disabled, |s| {
                        s.text_color(theme.neutral.text_2).hover(|s| {
                            s.cursor_pointer()
                                .bg(theme.neutral.card.opacity(0.6))
                                .text_color(theme.neutral.text_1)
                        })
                    })
                    .when(disabled, |s| {
                        s.text_color(theme.neutral.text_3)
                            .opacity(0.5)
                            .cursor_not_allowed()
                    })
                    .when(!disabled && !is_active, |s| {
                        s.cursor_pointer().on_click(cx.listener({
                            let value = value.clone();
                            move |this, _, window, cx| {
                                this.select_option(value.clone(), window, cx);
                            }
                        }))
                    })
                    .child(div().text_sm().child(opt.label.resolve(cx)));

                if is_active {
                    pop_in(
                        element_id(format!("{}-option-motion-{}", self.id, i)),
                        option,
                    )
                    .into_any_element()
                } else {
                    option.into_any_element()
                }
            }))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn segmented_supports_runtime_on_change_binding() {
        let source = include_str!("segmented.rs");
        assert!(source.contains("pub fn set_on_change"));
        assert!(source.contains("on_change: Option"));
    }
}
