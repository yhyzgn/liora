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

use gpui::{App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px};
use liora_core::{Config, stable_unique_id};
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

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
    id: Option<SharedString>,
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

    /// Sets the leading icon rendered with the item label.
    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Registers an ancestor's pointer/Enter activation callback; the final item stays inert.
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
            id: None,
        }
    }

    /// Sets the text rendered between path items.
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

    /// Assigns a stable namespace; use distinct IDs for repeated identical trails.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = Some(id.into());
        self
    }
}

impl RenderOnce for Breadcrumb {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        let (text, secondary, muted, icon_color, accent) = (
            theme.neutral.text_1,
            theme.neutral.text_2,
            theme.neutral.text_3,
            theme.neutral.icon,
            theme.primary.base,
        );
        let id = self.id.unwrap_or_else(|| {
            let mut key = DefaultHasher::new();
            for item in &self.items {
                item.label.hash(&mut key);
            }
            stable_unique_id(
                format!("breadcrumb:{:?}:{}", self.separator, key.finish()),
                "breadcrumb",
                window,
                cx,
            )
        });
        let items_count = self.items.len();
        let separator = self.separator;

        window.with_global_id(id.clone().into(), |_, window| {
            let mut trail = div().id(id).flex().flex_row().items_center().gap_1();
            for (i, item) in self.items.into_iter().enumerate() {
                let is_last = i == items_count - 1;
                let item_id = gpui::ElementId::NamedInteger(item.label.clone(), i as u64);
                let on_click = item.on_click.filter(|_| !is_last);
                let mut label = div()
                    .id(item_id.clone())
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_1()
                    .text_color(if is_last { text } else { secondary })
                    .when(is_last, |s| s.font_weight(gpui::FontWeight::BOLD))
                    .when_some(item.icon, |s, icon| {
                        s.child(Icon::new(icon).size(px(14.0)).color(icon_color))
                    });

                if let Some(on_click) = on_click {
                    let focus = window.with_global_id(item_id, |global_id, window| {
                        window.with_element_state::<gpui::FocusHandle, _>(
                            global_id,
                            |previous, _| {
                                let handle = previous.unwrap_or_else(|| {
                                    cx.focus_handle().tab_stop(true).tab_index(0)
                                });
                                (handle.clone(), handle)
                            },
                        )
                    });
                    let paint_focus = focus.clone();
                    let outline = gpui::canvas(
                        |_, _, _| (),
                        move |bounds, (), window, _| {
                            if paint_focus.is_focused(window) && window.last_input_was_keyboard() {
                                window.paint_quad(
                                    gpui::outline(
                                        bounds.dilate(px(2.0)),
                                        accent,
                                        gpui::BorderStyle::default(),
                                    )
                                    .border_widths(px(2.0))
                                    .corner_radii(px(2.0)),
                                );
                            }
                        },
                    )
                    .absolute()
                    .inset(px(0.0));
                    label = label
                        .role(gpui::Role::Link)
                        .aria_label(item.label.clone())
                        .cursor_pointer()
                        .hover(move |s| s.text_color(accent))
                        .track_focus(&focus)
                        .tab_index(0)
                        .capture_key_down(|event, window, _| {
                            // Space must not arm GPUI's shared Enter/Space click marker.
                            if event.keystroke.key == "space"
                                && !event.keystroke.modifiers.modified()
                            {
                                window.prevent_default();
                            }
                        })
                        .on_key_down(|event, window, cx| {
                            let modifiers = event.keystroke.modifiers;
                            if event.keystroke.key == "tab"
                                && !modifiers.control
                                && !modifiers.alt
                                && !modifiers.platform
                                && !modifiers.function
                            {
                                if modifiers.shift {
                                    window.focus_prev(cx);
                                } else {
                                    window.focus_next(cx);
                                }
                                cx.stop_propagation();
                            } else if event.keystroke.key == "enter" && !modifiers.modified() {
                                cx.stop_propagation();
                            }
                        })
                        .on_click(move |event, window, cx| {
                            let activate = match event {
                                gpui::ClickEvent::Mouse(click) => {
                                    click.up.button == gpui::MouseButton::Left
                                }
                                gpui::ClickEvent::Keyboard(click) => {
                                    let enter = click.button == gpui::KeyboardButton::Enter;
                                    if enter {
                                        cx.stop_propagation();
                                    }
                                    enter
                                }
                                gpui::ClickEvent::Touch(_) => true,
                            };
                            if activate {
                                on_click(window, cx);
                            }
                        })
                        .child(outline);
                } else {
                    window.with_global_id(item_id, |global_id, window| {
                        window.with_element_state::<gpui::InteractiveElementState, _>(
                            global_id,
                            |_, _| ((), Default::default()),
                        );
                    });
                }

                let mut part = div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .child(label.child(div().text_sm().child(item.label)));
                if !is_last {
                    part = part.child(
                        div().px_2().flex().items_center().justify_center().child(
                            match &separator {
                                BreadcrumbSeparator::String(sep) => div()
                                    .text_sm()
                                    .text_color(muted)
                                    .child(sep.clone())
                                    .into_any_element(),
                                BreadcrumbSeparator::Icon(icon) => Icon::new(*icon)
                                    .size(px(12.0))
                                    .color(icon_color)
                                    .into_any_element(),
                            },
                        ),
                    );
                }
                trail = trail.child(part);
            }
            trail
        })
    }
}

impl IntoElement for Breadcrumb {
    type Element = gpui::ViewElement<Self>;
    fn into_element(self) -> Self::Element {
        gpui::ViewElement::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::Cell, rc::Rc};

    struct BreadcrumbInputTest {
        activated: Rc<Cell<Option<usize>>>,
    }

    impl gpui::Render for BreadcrumbInputTest {
        fn render(&mut self, _: &mut Window, _: &mut gpui::Context<Self>) -> impl IntoElement {
            let home = self.activated.clone();
            let projects = self.activated.clone();
            let current = self.activated.clone();
            div().w(px(240.0)).h(px(40.0)).child(
                Breadcrumb::new()
                    .item(BreadcrumbItem::new("Home").on_click(move |_, _| home.set(Some(0))))
                    .item(
                        BreadcrumbItem::new("Projects").on_click(move |_, _| projects.set(Some(1))),
                    )
                    .item(
                        BreadcrumbItem::new("Current").on_click(move |_, _| current.set(Some(2))),
                    ),
            )
        }
    }

    fn release_key(window: &mut gpui::VisualTestContext, key: &str) -> Result<(), String> {
        window.update(|window, cx| {
            let keystroke = gpui::Keystroke::parse_with_os(key, cx.platform_info().os)
                .map_err(|error| error.to_string())?;
            window.dispatch_event(
                gpui::PlatformInput::KeyUp(gpui::KeyUpEvent { keystroke }),
                cx,
            );
            window.draw(cx).clear(cx);
            Ok(())
        })
    }

    #[test]
    fn keyboard_navigation_activates_ancestors_after_release_and_redraw() -> Result<(), String> {
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let activated = Rc::new(Cell::new(None));
        let observed = activated.clone();
        let handle = app.add_window(move |_, _| BreadcrumbInputTest {
            activated: observed,
        });
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        window.update(|window, cx| {
            window.draw(cx).clear(cx);
            window.focus_next(cx);
            window.draw(cx).clear(cx);
        });
        window.simulate_keystrokes("enter");
        assert_eq!(activated.get(), None, "keydown must not navigate");
        window.update(|window, cx| {
            window.refresh();
            window.draw(cx).clear(cx);
        });
        release_key(&mut window, "enter")?;
        assert_eq!(
            activated.get(),
            Some(0),
            "first ancestor must be keyboard reachable"
        );

        for key in ["space", "ctrl-enter"] {
            activated.set(None);
            window.simulate_keystrokes(key);
            release_key(&mut window, key)?;
            assert_eq!(activated.get(), None, "{key} must not navigate");
        }
        window.simulate_keystrokes("space");
        release_key(&mut window, "enter")?;
        assert_eq!(activated.get(), None, "Space must not arm an Enter release");
        release_key(&mut window, "space")?;

        for (key, expected) in [("tab", 1), ("shift-tab", 0), ("shift-tab", 1)] {
            activated.set(None);
            window.simulate_keystrokes(key);
            release_key(&mut window, key)?;
            window.simulate_keystrokes("enter");
            release_key(&mut window, "enter")?;
            assert_eq!(
                activated.get(),
                Some(expected),
                "{key}: skip the current item"
            );
        }
        app.quit();
        Ok(())
    }

    struct RepeatedBreadcrumbInputTest {
        activated: Rc<Cell<Option<usize>>>,
    }

    impl gpui::Render for RepeatedBreadcrumbInputTest {
        fn render(&mut self, _: &mut Window, _: &mut gpui::Context<Self>) -> impl IntoElement {
            let first = self.activated.clone();
            let second = self.activated.clone();
            div()
                .flex()
                .flex_col()
                .child(
                    Breadcrumb::new()
                        .id("first-trail")
                        .item(BreadcrumbItem::new("Home").on_click(move |_, _| first.set(Some(0))))
                        .item(BreadcrumbItem::new("Current")),
                )
                .child(
                    Breadcrumb::new()
                        .id("second-trail")
                        .item(BreadcrumbItem::new("Home").on_click(move |_, _| second.set(Some(1))))
                        .item(BreadcrumbItem::new("Current")),
                )
        }
    }

    #[test]
    fn repeated_trails_with_explicit_namespaces_keep_distinct_native_focus() -> Result<(), String> {
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let activated = Rc::new(Cell::new(None));
        let observed = activated.clone();
        let handle = app.add_window(move |_, _| RepeatedBreadcrumbInputTest {
            activated: observed,
        });
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        window.update(|window, cx| {
            window.draw(cx).clear(cx);
            window.focus_next(cx);
            window.draw(cx).clear(cx);
        });
        for expected in [0, 1, 0] {
            activated.set(None);
            window.simulate_keystrokes("enter");
            release_key(&mut window, "enter")?;
            assert_eq!(activated.get(), Some(expected));
            window.simulate_keystrokes("tab");
            release_key(&mut window, "tab")?;
        }
        app.quit();
        Ok(())
    }
}
