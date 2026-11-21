//! Alert module.
//!
//! This public module implements the Liora status messaging component for inline feedback and contextual notices. It keeps the reusable
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
    App, ElementId, IntoElement, RenderOnce, SharedString, ViewElement as Component, Window, div,
    prelude::*, px,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control alert type behavior.
pub enum AlertType {
    #[default]
    /// Uses informational semantic color tokens.
    Info,
    /// Uses success semantic color tokens.
    Success,
    /// Uses warning semantic color tokens.
    Warning,
    /// Reports a error failure.
    Error,
}

/// Fluent native GPUI component for rendering Liora alert.
pub struct Alert {
    id: Option<ElementId>,
    title: SharedString,
    description: Option<SharedString>,
    alert_type: AlertType,
    closable: bool,
    show_icon: bool,
    on_close: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl Alert {
    /// Creates `Alert` initialized from the supplied title.
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            id: None,
            title: title.into(),
            description: None,
            alert_type: AlertType::Info,
            closable: false,
            show_icon: true,
            on_close: None,
        }
    }

    /// Sets a stable element ID. Give sibling closable alerts distinct IDs when
    /// they share the same ancestor ID scope.
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets secondary descriptive text shown below the primary label.
    pub fn description(mut self, desc: impl Into<SharedString>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Selects the alert semantic tone used for color and icon styling.
    pub fn alert_type(mut self, t: AlertType) -> Self {
        self.alert_type = t;
        self
    }

    /// Shows the close affordance.
    pub fn closable(mut self, c: bool) -> Self {
        self.closable = c;
        self
    }

    /// Configures whether icon is visible in the rendered component.
    pub fn show_icon(mut self, s: bool) -> Self {
        self.show_icon = s;
        self
    }

    /// Registers a callback that runs when close occurs.
    pub fn on_close(mut self, f: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Box::new(f));
        self
    }
}

impl RenderOnce for Alert {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        let (color, icon_name) = match self.alert_type {
            AlertType::Info => (theme.primary.base, IconName::Info),
            AlertType::Success => (theme.success.base, IconName::Check),
            AlertType::Warning => (theme.warning.base, IconName::TriangleAlert),
            AlertType::Error => (theme.danger.base, IconName::CircleX),
        };

        let bg = color.opacity(0.1);

        let mut root = div()
            .flex()
            .flex_row()
            .items_center()
            .gap_3()
            .p_3()
            .bg(bg)
            .border_1()
            .border_color(color)
            .rounded(px(theme.radius.md))
            .child(div().flex().items_center().when(self.show_icon, |s| {
                s.child(Icon::new(icon_name).size(px(20.0)).color(color))
            }))
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .min_h(px(20.0))
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(color)
                            .child(self.title),
                    )
                    .when_some(self.description, |s, d| {
                        s.child(div().text_sm().text_color(color).child(d))
                    }),
            )
            .child(div().flex().items_center().when(self.closable, |s| {
                let on_close = self.on_close;
                s.child(
                    div()
                        .id("close-btn")
                        .debug_selector(|| "alert-close".to_string())
                        .tab_index(0)
                        .cursor_pointer()
                        .role(gpui::Role::Button)
                        .border_2()
                        .border_color(theme.primary.base.opacity(0.0))
                        .focus_visible(move |style| style.border_color(theme.primary.base))
                        .aria_label("Dismiss alert")
                        .child(Icon::new(IconName::X).size(px(14.0)).color(color))
                        .on_mouse_down(gpui::MouseButton::Left, |_, _, cx| cx.stop_propagation())
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
                            } else if !modifiers.modified()
                                && matches!(event.keystroke.key.as_str(), "enter" | "space")
                            {
                                cx.stop_propagation();
                            }
                        })
                        .on_click(move |_, window, cx| {
                            if let Some(on_close) = &on_close {
                                on_close(window, cx);
                            }
                            cx.stop_propagation();
                        }),
                )
            }));
        root.interactivity().element_id = self.id;
        root
    }
}

impl IntoElement for Alert {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::Cell, rc::Rc};

    struct AlertSurface {
        calls: Rc<Cell<usize>>,
        parent_actions: Rc<Cell<usize>>,
    }

    impl gpui::Render for AlertSurface {
        fn render(&mut self, _: &mut Window, _: &mut gpui::Context<Self>) -> impl IntoElement {
            let calls = self.calls.clone();
            let parent_clicks = self.parent_actions.clone();
            let parent_keys = self.parent_actions.clone();
            div()
                .id("alert-parent")
                .w(px(400.0))
                .on_click(move |_, _, _| parent_clicks.set(parent_clicks.get() + 1))
                .on_key_down(move |_, _, _| parent_keys.set(parent_keys.get() + 1))
                .child(
                    Alert::new("Dismissible warning")
                        .closable(true)
                        .on_close(move |_, _| calls.set(calls.get() + 1)),
                )
        }
    }

    struct SiblingAlertSurface {
        first_calls: Rc<Cell<usize>>,
        second_calls: Rc<Cell<usize>>,
        parent_actions: Rc<Cell<usize>>,
    }

    impl gpui::Render for SiblingAlertSurface {
        fn render(&mut self, _: &mut Window, _: &mut gpui::Context<Self>) -> impl IntoElement {
            let first_calls = self.first_calls.clone();
            let second_calls = self.second_calls.clone();
            let parent_actions = self.parent_actions.clone();
            div()
                .id("alert-parent")
                .w(px(400.0))
                .on_click({
                    let parent_actions = parent_actions.clone();
                    move |_, _, _| parent_actions.set(parent_actions.get() + 1)
                })
                .on_key_down({
                    let parent_actions = parent_actions.clone();
                    move |_, _, _| parent_actions.set(parent_actions.get() + 1)
                })
                .child(
                    Alert::new("Dismissible alert")
                        .id("first-alert")
                        .closable(true)
                        .on_close(move |_, _| first_calls.set(first_calls.get() + 1)),
                )
                .child(
                    Alert::new("Dismissible alert")
                        .id("second-alert")
                        .closable(true)
                        .on_close(move |_, _| second_calls.set(second_calls.get() + 1)),
                )
        }
    }

    #[test]
    fn sibling_closers_follow_dispatched_tab_and_shift_tab_without_bubbling() {
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let first_calls = Rc::new(Cell::new(0));
        let second_calls = Rc::new(Cell::new(0));
        let parent_actions = Rc::new(Cell::new(0));
        let handle = app.add_window({
            let first_calls = first_calls.clone();
            let second_calls = second_calls.clone();
            let parent_actions = parent_actions.clone();
            move |_, _| SiblingAlertSurface {
                first_calls,
                second_calls,
                parent_actions,
            }
        });
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        window.update(|window, cx| {
            window.draw(cx).clear(cx);
        });
        let unfocused_bounds = window
            .debug_bounds("alert-close")
            .expect("second close bounds");
        window.update(|window, cx| {
            window.focus_next(cx);
        });
        window.update(|window, cx| window.draw(cx).clear(cx));

        window.simulate_keystrokes("tab");
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert_eq!(
            window
                .debug_bounds("alert-close")
                .expect("focused second close bounds"),
            unfocused_bounds,
            "keyboard focus must not change close-control bounds"
        );
        window.simulate_keystrokes("enter");
        window.update(|window, cx| {
            let keystroke = gpui::Keystroke::parse_with_os("enter", cx.platform_info().os).unwrap();
            window.dispatch_event(
                gpui::PlatformInput::KeyUp(gpui::KeyUpEvent { keystroke }),
                cx,
            );
            window.draw(cx).clear(cx);
        });
        assert_eq!(
            first_calls.get(),
            0,
            "Tab must move to the second close control"
        );
        assert_eq!(
            second_calls.get(),
            1,
            "Enter must close the focused second alert once"
        );

        window.simulate_keystrokes("shift-tab");
        window.update(|window, cx| window.draw(cx).clear(cx));
        window.simulate_keystrokes("space");
        window.update(|window, cx| {
            let keystroke = gpui::Keystroke::parse_with_os("space", cx.platform_info().os).unwrap();
            window.dispatch_event(
                gpui::PlatformInput::KeyUp(gpui::KeyUpEvent { keystroke }),
                cx,
            );
            window.draw(cx).clear(cx);
        });
        assert_eq!(
            first_calls.get(),
            1,
            "Shift-Tab must return to the first close control"
        );
        assert_eq!(second_calls.get(), 1);
        assert_eq!(
            parent_actions.get(),
            0,
            "close keys and Tab must not bubble"
        );
        app.quit();
    }

    #[test]
    fn pointer_close_dispatches_once_without_hiding_or_bubbling() {
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let calls = Rc::new(Cell::new(0));
        let parent_actions = Rc::new(Cell::new(0));
        let handle = app.add_window({
            let calls = calls.clone();
            let parent_actions = parent_actions.clone();
            move |_, _| AlertSurface {
                calls,
                parent_actions,
            }
        });
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        window.update(|window, cx| window.draw(cx).clear(cx));
        let close_bounds = window
            .debug_bounds("alert-close")
            .expect("visible close control");
        window.simulate_click(close_bounds.center(), gpui::Modifiers::none());
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert_eq!(
            calls.get(),
            1,
            "pointer close must invoke callback exactly once"
        );
        assert!(
            window.debug_bounds("alert-close").is_some(),
            "callback does not hide Alert itself"
        );
        assert_eq!(parent_actions.get(), 0, "close must not trigger its parent");
        app.quit();
    }

    #[test]
    fn keyboard_close_dispatches_once_without_hiding_or_bubbling() {
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let calls = Rc::new(Cell::new(0));
        let parent_actions = Rc::new(Cell::new(0));
        let handle = app.add_window({
            let calls = calls.clone();
            let parent_actions = parent_actions.clone();
            move |_, _| AlertSurface {
                calls,
                parent_actions,
            }
        });
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        window.update(|window, cx| {
            window.draw(cx).clear(cx);
            window.focus_next(cx);
        });
        window.update(|window, cx| window.draw(cx).clear(cx));
        for (index, key) in ["enter", "space"].into_iter().enumerate() {
            window.simulate_keystrokes(key);
            assert_eq!(calls.get(), index, "activation waits for native key-up");
            window.update(|window, cx| {
                let keystroke = gpui::Keystroke::parse_with_os(key, cx.platform_info().os).unwrap();
                window.dispatch_event(
                    gpui::PlatformInput::KeyUp(gpui::KeyUpEvent { keystroke }),
                    cx,
                );
                window.draw(cx).clear(cx);
            });
            assert_eq!(calls.get(), index + 1, "{key} invokes close exactly once");
            assert!(
                window.debug_bounds("alert-close").is_some(),
                "parent retains visibility"
            );
            assert_eq!(
                parent_actions.get(),
                0,
                "{key} must not activate its parent"
            );
        }
        app.quit();
    }
}
