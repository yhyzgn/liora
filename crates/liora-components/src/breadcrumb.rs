//! Breadcrumb module.
//!
//! This public module implements the Liora hierarchical navigation trail component. It keeps the reusable
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
use gpui::{App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Options that control breadcrumb separator behavior.
pub enum BreadcrumbSeparator {
    /// Stores the separator as text.
    String(SharedString),
    /// Stores the separator or button content as an icon.
    Icon(IconName),
}

/// Data model used by breadcrumb item rendering.
pub struct BreadcrumbItem {
    /// User-facing label rendered for this item.
    pub label: SharedString,
    /// Optional icon rendered with the item.
    pub icon: Option<IconName>,
    /// Callback invoked when the component is activated by pointer or keyboard input.
    pub on_click: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

/// Fluent native GPUI component for rendering Liora breadcrumb.
pub struct Breadcrumb {
    separator: BreadcrumbSeparator,
    items: Vec<BreadcrumbItem>,
}

impl BreadcrumbItem {
    /// Creates `BreadcrumbItem` initialized from the supplied label.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            on_click: None,
        }
    }

    /// Sets the tray icon configuration value.
    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Registers a callback that runs when click occurs.
    pub fn on_click(mut self, f: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(f));
        self
    }
}

impl Breadcrumb {
    /// Creates `Breadcrumb` with default theme-driven styling and no optional callbacks attached.
    pub fn new() -> Self {
        Self {
            separator: BreadcrumbSeparator::String("/".into()),
            items: vec![],
        }
    }

    /// Creates a tray menu separator item specification.
    pub fn separator(mut self, s: impl Into<SharedString>) -> Self {
        self.separator = BreadcrumbSeparator::String(s.into());
        self
    }

    /// Uses an icon element as the breadcrumb separator.
    pub fn separator_icon(mut self, icon: IconName) -> Self {
        self.separator = BreadcrumbSeparator::Icon(icon);
        self
    }

    /// Adds the supplied item to the component.
    pub fn item(mut self, item: BreadcrumbItem) -> Self {
        self.items.push(item);
        self
    }
}

impl RenderOnce for Breadcrumb {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let items_count = self.items.len();
        let separator = self.separator;

        div().flex().flex_row().items_center().gap_1().children(
            self.items.into_iter().enumerate().map(|(i, item)| {
                let is_last = i == items_count - 1;
                let has_click = item.on_click.is_some();
                let on_click = item.on_click;

                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .child(
                        div()
                            .id(element_id(format!("breadcrumb-item-{}", i)))
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_1()
                            .text_color(if is_last {
                                theme.neutral.text_1
                            } else {
                                theme.neutral.text_2
                            })
                            .when(is_last, |s| s.font_weight(gpui::FontWeight::BOLD))
                            .when(!is_last && has_click, |s| {
                                s.cursor_pointer()
                                    .hover(|s| s.text_color(theme.primary.base))
                                    .on_click(move |_, window, cx| {
                                        if let Some(ref f) = on_click {
                                            (f)(window, cx);
                                        }
                                    })
                            })
                            .when_some(item.icon, |s, icon| {
                                s.child(Icon::new(icon).size(px(14.0)).color(theme.neutral.icon))
                            })
                            .child(div().text_sm().child(item.label)),
                    )
                    .when(!is_last, |s| {
                        s.child(
                            div().px_2().flex().items_center().justify_center().child(
                                match separator.clone() {
                                    BreadcrumbSeparator::String(sep) => div()
                                        .text_sm()
                                        .text_color(theme.neutral.text_3)
                                        .child(sep)
                                        .into_any_element(),
                                    BreadcrumbSeparator::Icon(icon) => Icon::new(icon)
                                        .size(px(12.0))
                                        .color(theme.neutral.icon)
                                        .into_any_element(),
                                },
                            ),
                        )
                    })
            }),
        )
    }
}

impl IntoElement for Breadcrumb {
    type Element = gpui::Component<Self>;
    fn into_element(self) -> Self::Element {
        gpui::Component::new(self)
    }
}
