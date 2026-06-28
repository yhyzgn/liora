//! Toggle module.
//!
//! This public module implements the Liora toggle and toggle-group controls for toolbar-like binary state selection. It keeps the reusable
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
    App, Component, IntoElement, MouseButton, RenderOnce, SharedString, Window, div, prelude::*, px,
};
use liora_core::Config;
use std::sync::Arc;

type ToggleCallback = dyn Fn(bool, &mut Window, &mut App) + 'static;
type ToggleGroupCallback = dyn Fn(SharedString, &mut Window, &mut App) + 'static;

/// Binary toolbar button that exposes selected/unselected state.
pub struct Toggle {
    label: SharedString,
    selected: bool,
    disabled: bool,
    on_change: Option<Arc<ToggleCallback>>,
}

impl Toggle {
    /// Creates a toggle from a label and selected state.
    pub fn new(label: impl Into<SharedString>, selected: bool) -> Self {
        Self {
            label: label.into(),
            selected,
            disabled: false,
            on_change: None,
        }
    }

    /// Toggles disabled visual state and suppresses interaction.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Registers a callback that receives the next selected state.
    pub fn on_change(mut self, callback: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Arc::new(callback));
        self
    }

    /// Returns whether the toggle is selected.
    pub fn selected(&self) -> bool {
        self.selected
    }
}

impl IntoElement for Toggle {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl RenderOnce for Toggle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let selected = self.selected;
        let disabled = self.disabled;
        let callback = self.on_change.clone();
        div()
            .px_3()
            .py_2()
            .rounded(px(theme.radius.sm))
            .border_1()
            .border_color(if selected {
                theme.primary.base
            } else {
                theme.neutral.border
            })
            .bg(if selected {
                theme.primary.light_9
            } else {
                theme.neutral.card
            })
            .text_color(if selected {
                theme.primary.base
            } else {
                theme.neutral.text_2
            })
            .text_sm()
            .when(disabled, |s| s.opacity(0.55).cursor_not_allowed())
            .when(!disabled, |s| {
                s.cursor_pointer()
                    .hover(|s| s.bg(theme.neutral.hover))
                    .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                        if let Some(callback) = &callback {
                            callback(!selected, window, cx);
                        }
                    })
            })
            .child(self.label)
    }
}

/// Option model for [`ToggleGroup`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToggleOption {
    /// Stable option value emitted when the option is selected.
    pub value: SharedString,
    /// Human-readable option label displayed in the control.
    pub label: SharedString,
    /// Whether the option is visible but not interactive.
    pub disabled: bool,
}

impl ToggleOption {
    /// Creates a selectable option.
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }
    /// Toggles disabled visual state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Single-select group of toolbar toggles.
pub struct ToggleGroup {
    options: Vec<ToggleOption>,
    selected: Option<SharedString>,
    on_change: Option<Arc<ToggleGroupCallback>>,
}

impl ToggleGroup {
    /// Creates a toggle group from options.
    pub fn new(options: impl IntoIterator<Item = ToggleOption>) -> Self {
        Self {
            options: options.into_iter().collect(),
            selected: None,
            on_change: None,
        }
    }
    /// Sets the selected option value.
    pub fn selected(mut self, value: impl Into<SharedString>) -> Self {
        self.selected = Some(value.into());
        self
    }
    /// Registers a callback that receives the selected value.
    pub fn on_change(
        mut self,
        callback: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Arc::new(callback));
        self
    }
    /// Returns option count.
    pub fn len(&self) -> usize {
        self.options.len()
    }
    /// Returns whether there are no options.
    pub fn is_empty(&self) -> bool {
        self.options.is_empty()
    }
}

impl IntoElement for ToggleGroup {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl RenderOnce for ToggleGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        div()
            .flex()
            .flex_row()
            .rounded(px(theme.radius.sm))
            .border_1()
            .border_color(theme.neutral.border)
            .overflow_hidden()
            .children(self.options.into_iter().enumerate().map(|(index, option)| {
                let selected = self.selected.as_ref() == Some(&option.value);
                let disabled = option.disabled;
                let value = option.value.clone();
                let callback = self.on_change.clone();
                div()
                    .px_3()
                    .py_2()
                    .text_sm()
                    .when(index > 0, |s| {
                        s.border_l_1().border_color(theme.neutral.border)
                    })
                    .bg(if selected {
                        theme.primary.light_9
                    } else {
                        theme.neutral.card
                    })
                    .text_color(if selected {
                        theme.primary.base
                    } else {
                        theme.neutral.text_2
                    })
                    .when(disabled, |s| s.opacity(0.55).cursor_not_allowed())
                    .when(!disabled, |s| {
                        s.cursor_pointer()
                            .hover(|s| s.bg(theme.neutral.hover))
                            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                if let Some(callback) = &callback {
                                    callback(value.clone(), window, cx);
                                }
                            })
                    })
                    .child(option.label)
                    .into_any_element()
            }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn toggle_group_tracks_options_and_selection() {
        let group = ToggleGroup::new([
            ToggleOption::new("left", "Left"),
            ToggleOption::new("right", "Right"),
        ])
        .selected("right");
        assert_eq!(group.len(), 2);
        assert_eq!(group.selected.as_ref().map(|v| v.as_ref()), Some("right"));
        assert!(Toggle::new("Bold", true).selected());
    }
}
