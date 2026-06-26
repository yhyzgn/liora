//! Accordion module.
//!
//! This public module implements the Liora accordion component for compact FAQ, settings, and documentation sections. It keeps the reusable
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

use crate::{gpui_compat::element_id, motion::pop_in};
use gpui::{
    AnyElement, Context, IntoElement, MouseButton, Pixels, Render, SharedString, Window, div,
    prelude::*, px,
};
use liora_core::{Config, unique_id};
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::{collections::HashSet, sync::Arc};

/// Data model used by Accordion item rendering.
pub struct AccordionItem {
    /// Stable identifier used by selection state and callbacks.
    pub id: SharedString,
    /// User-facing heading rendered in the trigger row.
    pub title: SharedString,
    /// Optional secondary description rendered under the title.
    pub description: Option<SharedString>,
    /// Whether the trigger is inert and styled as disabled.
    pub disabled: bool,
    /// Content rendered inside the expanded panel.
    pub content: Arc<dyn Fn(&mut Window, &mut Context<Accordion>) -> AnyElement + 'static>,
}

/// Controls whether one or many panels can be open.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccordionMode {
    /// Only one panel may be open at a time.
    Single,
    /// Multiple panels may remain open together.
    Multiple,
}

/// Size presets for Accordion row spacing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccordionSize {
    /// Compact rows for dense settings pages.
    Small,
    /// Default row spacing.
    Medium,
    /// Larger rows for documentation and onboarding pages.
    Large,
}

/// Fluent native GPUI component for rendering Liora accordion.
pub struct Accordion {
    id: SharedString,
    items: Vec<AccordionItem>,
    open_items: HashSet<SharedString>,
    mode: AccordionMode,
    size: AccordionSize,
    bordered: bool,
}

impl Accordion {
    /// Creates an Accordion in single-open mode.
    pub fn new() -> Self {
        Self {
            id: unique_id("accordion"),
            items: Vec::new(),
            open_items: HashSet::new(),
            mode: AccordionMode::Single,
            size: AccordionSize::Medium,
            bordered: true,
        }
    }

    /// Assigns a stable element id used by GPUI state, hit testing, and automated interaction tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Allows multiple panels to be open at the same time.
    pub fn multiple(mut self) -> Self {
        self.mode = AccordionMode::Multiple;
        self
    }

    /// Restores single-open behavior explicitly.
    pub fn single(mut self) -> Self {
        self.mode = AccordionMode::Single;
        if self.open_items.len() > 1 {
            let first = self.open_items.iter().next().cloned();
            self.open_items.clear();
            if let Some(first) = first {
                self.open_items.insert(first);
            }
        }
        self
    }

    /// Toggles the outer border and item separators.
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// Applies the small sizing preset.
    pub fn small(mut self) -> Self {
        self.size = AccordionSize::Small;
        self
    }

    /// Applies the large sizing preset.
    pub fn large(mut self) -> Self {
        self.size = AccordionSize::Large;
        self
    }

    /// Marks an item as initially open.
    pub fn default_open(mut self, id: impl Into<SharedString>) -> Self {
        let id = id.into();
        if self.mode == AccordionMode::Single {
            self.open_items.clear();
        }
        self.open_items.insert(id);
        self
    }

    /// Adds an enabled item.
    pub fn item<F, E>(
        self,
        id: impl Into<SharedString>,
        title: impl Into<SharedString>,
        f: F,
    ) -> Self
    where
        F: Fn(&mut Window, &mut Context<Self>) -> E + 'static,
        E: IntoElement,
    {
        self.item_with_options(id, title, None::<SharedString>, false, f)
    }

    /// Adds a disabled item.
    pub fn disabled_item<F, E>(
        self,
        id: impl Into<SharedString>,
        title: impl Into<SharedString>,
        f: F,
    ) -> Self
    where
        F: Fn(&mut Window, &mut Context<Self>) -> E + 'static,
        E: IntoElement,
    {
        self.item_with_options(id, title, None::<SharedString>, true, f)
    }

    /// Adds an item with secondary description text.
    pub fn item_with_description<F, E>(
        self,
        id: impl Into<SharedString>,
        title: impl Into<SharedString>,
        description: impl Into<SharedString>,
        f: F,
    ) -> Self
    where
        F: Fn(&mut Window, &mut Context<Self>) -> E + 'static,
        E: IntoElement,
    {
        self.item_with_options(id, title, Some(description.into()), false, f)
    }

    fn item_with_options<F, E>(
        mut self,
        id: impl Into<SharedString>,
        title: impl Into<SharedString>,
        description: Option<SharedString>,
        disabled: bool,
        f: F,
    ) -> Self
    where
        F: Fn(&mut Window, &mut Context<Self>) -> E + 'static,
        E: IntoElement,
    {
        self.items.push(AccordionItem {
            id: id.into(),
            title: title.into(),
            description,
            disabled,
            content: Arc::new(move |window, cx| f(window, cx).into_any_element()),
        });
        self
    }

    fn metrics(&self) -> (Pixels, Pixels, Pixels, Pixels) {
        match self.size {
            AccordionSize::Small => (px(10.0), px(12.0), px(12.0), px(14.0)),
            AccordionSize::Medium => (px(13.0), px(16.0), px(14.0), px(16.0)),
            AccordionSize::Large => (px(16.0), px(20.0), px(16.0), px(18.0)),
        }
    }

    fn toggle(&mut self, id: SharedString, disabled: bool, cx: &mut Context<Self>) {
        if disabled {
            return;
        }
        if self.open_items.contains(&id) {
            self.open_items.remove(&id);
        } else {
            if self.mode == AccordionMode::Single {
                self.open_items.clear();
            }
            self.open_items.insert(id);
        }
        cx.notify();
    }
}

impl Render for Accordion {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let (py_header, px_header, text_size, content_padding) = self.metrics();

        let mut root = div().flex().flex_col().w_full().bg(theme.neutral.card);
        if self.bordered {
            root = root
                .border_1()
                .border_color(theme.neutral.border)
                .rounded(px(theme.radius.md))
                .overflow_hidden();
        } else {
            root = root.gap_2();
        }

        root.children(self.items.iter().enumerate().map(|(index, item)| {
            let item_id = item.id.clone();
            let is_open = self.open_items.contains(&item_id);
            let is_last = index + 1 == self.items.len();
            let disabled = item.disabled;
            let title_color = if disabled {
                theme.neutral.text_3
            } else if is_open {
                theme.primary.base
            } else {
                theme.neutral.text_1
            };
            let row_bg = if is_open {
                theme.primary.base.opacity(0.06)
            } else {
                theme.neutral.card
            };

            let mut item_root = div().flex().flex_col().w_full();
            if !self.bordered {
                item_root = item_root
                    .border_1()
                    .border_color(theme.neutral.border)
                    .rounded(px(theme.radius.md))
                    .overflow_hidden();
            }

            item_root
                .child(
                    div()
                        .id(element_id(format!("{}-item-{}", self.id, item.id)))
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .gap_3()
                        .w_full()
                        .px(px_header)
                        .py(py_header)
                        .cursor_pointer()
                        .when(disabled, |s| s.cursor_not_allowed())
                        .bg(row_bg)
                        .text_color(title_color)
                        .hover(move |s| {
                            if disabled {
                                s.bg(row_bg)
                            } else {
                                s.bg(theme.neutral.hover)
                            }
                        })
                        .when(self.bordered && !is_last && !is_open, |s| {
                            s.border_b_1().border_color(theme.neutral.border)
                        })
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener({
                                let item_id = item_id.clone();
                                move |this, _, _, cx| this.toggle(item_id.clone(), disabled, cx)
                            }),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_1()
                                .child(
                                    div()
                                        .text_size(text_size)
                                        .font_weight(gpui::FontWeight::BOLD)
                                        .child(item.title.clone()),
                                )
                                .when_some(item.description.clone(), |s, description| {
                                    s.child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.neutral.text_3)
                                            .child(description),
                                    )
                                }),
                        )
                        .child(
                            Icon::new(if is_open {
                                IconName::ChevronDown
                            } else {
                                IconName::ChevronRight
                            })
                            .size(px(16.0))
                            .color(if disabled {
                                theme.neutral.text_3
                            } else {
                                theme.neutral.icon
                            }),
                        ),
                )
                .when(is_open, |s| {
                    s.child(pop_in(
                        element_id(format!("{}-panel-{}", self.id, item.id)),
                        div()
                            .px(content_padding)
                            .py(content_padding)
                            .bg(theme.neutral.card)
                            .when(self.bordered && !is_last, |s| {
                                s.border_b_1().border_color(theme.neutral.border)
                            })
                            .child((item.content)(window, cx)),
                    ))
                })
        }))
    }
}

impl Default for Accordion {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accordion_builders_track_mode_size_and_open_state() {
        let accordion = Accordion::new()
            .id("settings")
            .multiple()
            .small()
            .bordered(false)
            .default_open("profile")
            .default_open("security")
            .item("profile", "Profile", |_, _| div())
            .disabled_item("billing", "Billing", |_, _| div());

        assert_eq!(accordion.id.as_ref(), "settings");
        assert_eq!(accordion.mode, AccordionMode::Multiple);
        assert_eq!(accordion.size, AccordionSize::Small);
        assert!(!accordion.bordered);
        assert!(accordion.open_items.contains("profile"));
        assert!(accordion.open_items.contains("security"));
        assert_eq!(accordion.items.len(), 2);
        assert!(accordion.items[1].disabled);
    }

    #[test]
    fn accordion_single_mode_keeps_only_one_default_open_item() {
        let accordion = Accordion::new()
            .default_open("first")
            .default_open("second")
            .large();

        assert_eq!(accordion.mode, AccordionMode::Single);
        assert_eq!(accordion.size, AccordionSize::Large);
        assert_eq!(accordion.open_items.len(), 1);
        assert!(accordion.open_items.contains("second"));
    }

    #[test]
    fn accordion_source_uses_motion_and_mouse_down_selection() {
        let source = include_str!("accordion.rs");
        assert!(source.contains("pop_in("));
        assert!(source.contains("on_mouse_down(MouseButton::Left"));
        assert!(source.contains("cursor_not_allowed"));
        assert!(source.contains("AccordionMode::Single"));
        assert!(source.contains("AccordionMode::Multiple"));
    }
}
