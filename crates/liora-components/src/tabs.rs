//! Tabs module.
//!
//! This public module implements the Liora tabs component for switching content panes. It keeps the reusable
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
use gpui::{
    AnyElement, App, Context, FocusHandle, IntoElement, Orientation, Render, Role, SharedString,
    Window, div, prelude::*, px,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control tab position behavior.
pub enum TabPosition {
    #[default]
    /// Places the overlay above the anchor.
    Top,
    /// Places the overlay below the anchor.
    Bottom,
    /// Places the overlay to the left of the anchor.
    Left,
    /// Places the overlay to the right of the anchor.
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control tab type behavior.
pub enum TabType {
    #[default]
    /// Uses the standard tab type treatment.
    Standard,
    /// Uses the card tab type treatment.
    Card,
    /// Uses the border card tab type treatment.
    BorderCard,
}

/// Fluent native GPUI component for rendering Liora tab pane.
pub struct TabPane {
    /// Unique business key within the tab set.
    pub name: SharedString,
    /// User-facing label rendered for this item.
    pub label: SharedString,
    /// Content rendered inside the component body.
    pub content: Arc<dyn Fn(&mut Window, &mut Context<Tabs>) -> AnyElement + 'static>,
    /// Whether the tab shows a close affordance.
    pub closable: bool,
    /// Optional icon rendered with the item.
    pub icon: Option<IconName>,
}

/// Fluent native GPUI component for rendering Liora tabs.
pub struct Tabs {
    id: SharedString,
    active_name: SharedString,
    position: TabPosition,
    tab_type: TabType,
    panes: Vec<TabPane>,
    focus: HashMap<SharedString, (FocusHandle, Option<FocusHandle>)>,
    add_focus: Option<FocusHandle>,
    editable: bool,
    stretch: bool,
    on_tab_click: Option<Box<dyn Fn(SharedString, &mut Window, &mut App) + 'static>>,
    on_tab_remove: Option<Box<dyn Fn(SharedString, &mut Window, &mut App) + 'static>>,
    on_tab_add: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl Tabs {
    /// Creates `Tabs` initialized from the supplied active name.
    pub fn new(active_name: impl Into<SharedString>) -> Self {
        let name = active_name.into();
        Self {
            id: liora_core::unique_id("tabs"),
            active_name: name,
            position: TabPosition::Top,
            tab_type: TabType::Standard,
            panes: vec![],
            focus: HashMap::new(),
            add_focus: None,
            editable: false,
            stretch: false,
            on_tab_click: None,
            on_tab_remove: None,
            on_tab_add: None,
        }
    }

    /// Assigns a stable element id used by GPUI state, hit testing, and automated interaction tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Selects the edge or anchor position used by layout.
    pub fn position(mut self, pos: TabPosition) -> Self {
        self.position = pos;
        self
    }

    /// Selects the type used by the component.
    pub fn type_(mut self, t: TabType) -> Self {
        self.tab_type = t;
        self
    }

    /// Sets the editable value used by the component.
    pub fn editable(mut self, e: bool) -> Self {
        self.editable = e;
        self
    }

    /// Lets items stretch to fill the available cross-axis space.
    pub fn stretch(mut self, stretch: bool) -> Self {
        self.stretch = stretch;
        self
    }

    /// Registers a callback that runs when tab click occurs.
    pub fn on_tab_click(
        mut self,
        f: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_tab_click = Some(Box::new(f));
        self
    }

    /// Registers a callback that runs when tab remove occurs.
    pub fn on_tab_remove(
        mut self,
        f: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_tab_remove = Some(Box::new(f));
        self
    }

    /// Registers a callback that runs when tab add occurs.
    pub fn on_tab_add(mut self, f: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_tab_add = Some(Box::new(f));
        self
    }

    /// Performs the pane operation used by this component.
    pub fn pane<F, E>(
        mut self,
        name: impl Into<SharedString>,
        label: impl Into<SharedString>,
        f: F,
    ) -> Self
    where
        F: Fn(&mut Window, &mut Context<Self>) -> E + 'static,
        E: IntoElement,
    {
        let name = name.into();
        if self.panes.iter().any(|pane| pane.name == name) {
            return self;
        }
        self.panes.push(TabPane {
            name,
            label: label.into(),
            content: Arc::new(move |window, cx| f(window, cx).into_any_element()),
            closable: true,
            icon: None,
        });
        self
    }

    fn select_tab(&mut self, name: SharedString, window: &mut Window, cx: &mut Context<Self>) {
        if let Some((focus, _)) = self.focus.get(&name) {
            focus.focus(window, cx);
        }
        self.active_name = name.clone();
        if let Some(on_click) = &self.on_tab_click {
            (on_click)(name, window, cx);
        }
        cx.notify();
    }

    fn remove_tab(&mut self, name: SharedString, window: &mut Window, cx: &mut Context<Self>) {
        let Some(pos) = self.panes.iter().position(|p| p.name == name) else {
            return;
        };
        self.panes.remove(pos);
        let removed_focus = self.focus.remove(&name).is_some_and(|(tab, close)| {
            tab.is_focused(window) || close.is_some_and(|close| close.is_focused(window))
        });
        let neighbor = self.panes.get(pos).or_else(|| self.panes.last());
        if self.active_name == name {
            self.active_name = neighbor.map(|pane| pane.name.clone()).unwrap_or_default();
        }
        if removed_focus {
            if let Some(focus) = neighbor
                .and_then(|pane| self.focus.get(&pane.name).map(|pair| &pair.0))
                .or(self.add_focus.as_ref())
            {
                focus.focus(window, cx);
            }
        }
        if let Some(on_remove) = &self.on_tab_remove {
            (on_remove)(name, window, cx);
        }
        cx.notify();
    }

    fn add_tab(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let mut next = self.panes.len() + 1;
        let name = loop {
            let candidate = SharedString::from(format!("tab-{}", next));
            if self.panes.iter().all(|pane| pane.name != candidate) {
                break candidate;
            }
            next += 1;
        };
        let label = SharedString::from(format!("Tab {}", next));
        let content_text = SharedString::from(format!("Content of Tab {}", next));

        self.panes.push(TabPane {
            name: name.clone(),
            label,
            content: Arc::new(move |_, _| div().child(content_text.clone()).into_any_element()),
            closable: true,
            icon: None,
        });
        self.active_name = name;
        let focus = cx.focus_handle();
        focus.focus(window, cx);
        self.focus.insert(self.active_name.clone(), (focus, None));

        if let Some(on_add) = &self.on_tab_add {
            (on_add)(window, cx);
        }
        cx.notify();
    }

    fn tab_key(
        &mut self,
        name: &SharedString,
        key: &str,
        vertical: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // A pending key replay can reuse this listener after its pane was removed.
        let Some(index) = self.panes.iter().position(|pane| &pane.name == name) else {
            return;
        };
        let len = self.panes.len();
        let target = match (key, vertical) {
            ("left", false) | ("up", true) => (index + len - 1) % len,
            ("right", false) | ("down", true) => (index + 1) % len,
            ("home", _) => 0,
            ("end", _) => len - 1,
            ("delete", _) if self.editable && self.panes[index].closable => {
                cx.stop_propagation();
                self.remove_tab(self.panes[index].name.clone(), window, cx);
                return;
            }
            _ => return,
        };
        self.focus[&self.panes[target].name].0.focus(window, cx);
        cx.stop_propagation();
    }
}

impl Render for Tabs {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let tab_type = self.tab_type;
        let position = self.position;
        let is_vertical = position == TabPosition::Left || position == TabPosition::Right;
        if !self.panes.iter().any(|pane| pane.name == self.active_name) {
            self.active_name = self
                .panes
                .first()
                .map(|pane| pane.name.clone())
                .unwrap_or_default();
        }
        for pane in &self.panes {
            let (_, close) = self
                .focus
                .entry(pane.name.clone())
                .or_insert_with(|| (cx.focus_handle(), None));
            if self.editable && pane.closable {
                close.get_or_insert_with(|| cx.focus_handle().tab_stop(true));
            }
        }
        if self.editable {
            self.add_focus
                .get_or_insert_with(|| cx.focus_handle().tab_stop(true));
        }
        // While a header holds focus, it alone is in the painted tab-stop map.
        // Outside the headers, entry returns to the selected tab.
        let entry = self
            .panes
            .iter()
            .position(|pane| self.focus[&pane.name].0.is_focused(window))
            .or_else(|| {
                self.panes
                    .iter()
                    .position(|pane| pane.name == self.active_name)
            });
        let focus_color = theme.primary.base;

        let render_header = |this: &Self, cx: &mut Context<Self>| {
            let theme = cx.global::<Config>().theme.clone();
            let close_label = liora_core::locales::tr(cx, liora_core::locales::common::close);
            let add_label = liora_core::locales::tr(cx, liora_core::locales::common::add);
            div()
                .id("tab-list")
                .role(Role::TabList)
                .aria_orientation(if is_vertical {
                    Orientation::Vertical
                } else {
                    Orientation::Horizontal
                })
                .flex()
                .when(!is_vertical, |s| s.flex_row().items_center().w_full())
                .when(is_vertical, |s| s.flex_col().w(px(120.0)))
                .when(tab_type == TabType::Standard, |s| match position {
                    TabPosition::Top => s
                        .when(!this.stretch, |s| s.gap_8())
                        .border_b_1()
                        .border_color(theme.neutral.border),
                    TabPosition::Bottom => s
                        .when(!this.stretch, |s| s.gap_8())
                        .border_t_1()
                        .border_color(theme.neutral.border),
                    TabPosition::Left => s.gap_2().border_r_1().border_color(theme.neutral.border),
                    TabPosition::Right => s.gap_2().border_l_1().border_color(theme.neutral.border),
                })
                .when(
                    tab_type == TabType::Card || tab_type == TabType::BorderCard,
                    |s| {
                        s.bg(theme.neutral.hover)
                            .border_b_1()
                            .border_color(theme.neutral.border)
                    },
                )
                .children(this.panes.iter().enumerate().map(|(index, pane)| {
                    let name = pane.name.clone();
                    let is_active = this.active_name == name;
                    let closable = pane.closable && this.editable;
                    let horizontal_padding = if tab_type == TabType::Standard {
                        8.0
                    } else {
                        20.0
                    };
                    let (focus, close_focus) = &this.focus[&name];
                    let focus = focus.clone().tab_stop(entry == Some(index));

                    div()
                        .id(element_id(format!("item-{name}")))
                        .relative()
                        .flex()
                        .when(this.stretch && !is_vertical, |s| s.flex_1())
                        .when(!is_vertical, |s| s.h(px(40.0)))
                        .when(is_vertical, |s| s.w_full())
                        .when(tab_type == TabType::Standard, |s| {
                            s.text_color(if is_active {
                                theme.primary.base
                            } else {
                                theme.neutral.text_1
                            })
                            .hover(|s| s.text_color(theme.primary.base))
                            .when(is_active, |s| match position {
                                TabPosition::Top => s.child(pop_in(
                                    element_id(format!("{}-indicator-motion-{}", this.id, name)),
                                    div()
                                        .absolute()
                                        .bottom_0()
                                        .w_full()
                                        .h(px(2.0))
                                        .bg(theme.primary.base),
                                )),
                                TabPosition::Bottom => s.child(pop_in(
                                    element_id(format!("{}-indicator-motion-{}", this.id, name)),
                                    div()
                                        .absolute()
                                        .top_0()
                                        .w_full()
                                        .h(px(2.0))
                                        .bg(theme.primary.base),
                                )),
                                TabPosition::Left => s.child(pop_in(
                                    element_id(format!("{}-indicator-motion-{}", this.id, name)),
                                    div()
                                        .absolute()
                                        .right_0()
                                        .h_full()
                                        .w(px(2.0))
                                        .bg(theme.primary.base),
                                )),
                                TabPosition::Right => s.child(pop_in(
                                    element_id(format!("{}-indicator-motion-{}", this.id, name)),
                                    div()
                                        .absolute()
                                        .left_0()
                                        .h_full()
                                        .w(px(2.0))
                                        .bg(theme.primary.base),
                                )),
                            })
                        })
                        .when(
                            tab_type == TabType::Card || tab_type == TabType::BorderCard,
                            |s| {
                                s.border_r_1()
                                    .border_color(theme.neutral.border)
                                    .bg(if is_active {
                                        theme.neutral.card
                                    } else {
                                        gpui::transparent_black()
                                    })
                                    .text_color(if is_active {
                                        theme.primary.base
                                    } else {
                                        theme.neutral.text_1
                                    })
                                    .hover(|s| s.text_color(theme.primary.base))
                                    .when(is_active, |s| {
                                        s.border_b_1().border_color(theme.neutral.card).mb(px(-1.0))
                                    })
                            },
                        )
                        .child(
                            div()
                                .id("tab")
                                .track_focus(&focus)
                                .cursor_pointer()
                                .role(Role::Tab)
                                .aria_label(pane.label.clone())
                                .aria_selected(is_active)
                                .flex_1()
                                .flex()
                                .items_center()
                                .justify_center()
                                .pl(px(horizontal_padding))
                                .pr(px(horizontal_padding + if closable { 24.0 } else { 0.0 }))
                                .when(is_vertical, |s| s.py_3())
                                .child(
                                    gpui::canvas(|_, _, _| (), {
                                        let focus = focus.clone();
                                        move |bounds, (), window, _| {
                                            if focus.is_focused(window)
                                                && window.last_input_was_keyboard()
                                            {
                                                window.paint_quad(gpui::outline(
                                                    bounds,
                                                    focus_color,
                                                    gpui::BorderStyle::default(),
                                                ));
                                            }
                                        }
                                    })
                                    .absolute()
                                    .inset_0(),
                                )
                                .on_click(cx.listener({
                                    let name = name.clone();
                                    move |this, _, window, cx| {
                                        this.select_tab(name.clone(), window, cx)
                                    }
                                }))
                                .on_key_down(cx.listener({
                                    let name = name.clone();
                                    move |this, event: &gpui::KeyDownEvent, window, cx| {
                                        if !event.keystroke.modifiers.modified() {
                                            this.tab_key(
                                                &name,
                                                &event.keystroke.key,
                                                is_vertical,
                                                window,
                                                cx,
                                            );
                                        }
                                    }
                                }))
                                .text_sm()
                                .child(pane.label.clone()),
                        )
                        .when(closable, |s| {
                            s.child(
                                div()
                                    .id("close")
                                    .absolute()
                                    .right(px(horizontal_padding))
                                    .top(gpui::relative(0.5))
                                    .mt(px(-8.0))
                                    .block_mouse_except_scroll()
                                    .track_focus(
                                        close_focus.as_ref().expect("rendered close has a handle"),
                                    )
                                    .role(Role::Button)
                                    .aria_label(format!("{close_label} {}", pane.label))
                                    .cursor_pointer()
                                    .border_1()
                                    .border_color(gpui::transparent_black())
                                    .focus_visible(move |style| style.border_color(focus_color))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .w_4()
                                    .h_4()
                                    .rounded_full()
                                    .hover(|s| s.bg(theme.neutral.hover))
                                    .on_click(cx.listener({
                                        let name = name.clone();
                                        move |this, _, window, cx| {
                                            this.remove_tab(name.clone(), window, cx);
                                            cx.stop_propagation();
                                        }
                                    }))
                                    .child(
                                        Icon::new(IconName::X)
                                            .size(px(12.0))
                                            .color(theme.neutral.icon),
                                    ),
                            )
                        })
                }))
                .when(this.editable, |s| {
                    s.child(
                        div()
                            .id("add-tab")
                            .track_focus(
                                this.add_focus
                                    .as_ref()
                                    .expect("editable tabs have an Add handle"),
                            )
                            .role(Role::Button)
                            .aria_label(add_label)
                            .border_1()
                            .border_color(gpui::transparent_black())
                            .focus_visible(move |style| style.border_color(focus_color))
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .justify_center()
                            .w_10()
                            .h_10()
                            .hover(|s| s.text_color(theme.primary.base))
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.add_tab(window, cx);
                            }))
                            .child(
                                Icon::new(IconName::Plus)
                                    .size(px(16.0))
                                    .color(theme.neutral.icon),
                            ),
                    )
                })
        };

        let content = self
            .panes
            .iter()
            .find(|p| p.name == self.active_name)
            .map(|p| {
                let label = p.label.clone();
                div()
                    .id(element_id(format!("panel-{}", p.name)))
                    .role(Role::TabPanel)
                    .aria_label(label)
                    .flex_1()
                    .p_4()
                    .child((p.content)(window, cx))
                    .into_any_element()
            })
            .unwrap_or_else(|| div().flex_1().p_4().into_any_element());

        div()
            .id(element_id(self.id.clone()))
            .flex()
            .w_full()
            .when(!is_vertical, |s| s.flex_col())
            .when(is_vertical, |s| s.flex_row())
            .when(tab_type == TabType::BorderCard, |s| {
                s.border_1()
                    .border_color(theme.neutral.border)
                    .rounded(px(theme.radius.md))
                    .overflow_hidden()
            })
            .bg(theme.neutral.card)
            .child(match position {
                TabPosition::Top | TabPosition::Left => render_header(self, cx).into_any_element(),
                _ => div().into_any_element(),
            })
            .child(content)
            .child(match position {
                TabPosition::Bottom | TabPosition::Right => {
                    render_header(self, cx).into_any_element()
                }
                _ => div().into_any_element(),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{AppContext, Modifiers, point};

    fn press(window: &mut gpui::VisualTestContext, key: &str) {
        window.simulate_keystrokes(key);
        window.update(|window, cx| {
            let keystroke = gpui::Keystroke::parse_with_os(key, cx.platform_info().os).unwrap();
            window.dispatch_event(
                gpui::PlatformInput::KeyUp(gpui::KeyUpEvent { keystroke }),
                cx,
            );
            window.draw(cx).clear(cx);
        });
    }

    struct KeyboardSurface {
        tabs: gpui::Entity<Tabs>,
        before: gpui::FocusHandle,
        after: gpui::FocusHandle,
    }

    impl Render for KeyboardSurface {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            crate::AppWindowFrame::new(
                "Tabs test",
                div()
                    .flex()
                    .flex_col()
                    .child(div().id("before").track_focus(&self.before).child("Before"))
                    .child(self.tabs.clone())
                    .child(div().id("after").track_focus(&self.after).child("After")),
            )
        }
    }

    fn keyboard_surface(
        app: &mut gpui::TestAppContext,
        tabs: Tabs,
    ) -> (
        gpui::VisualTestContext,
        gpui::FocusHandle,
        gpui::FocusHandle,
    ) {
        app.update(crate::init_liora);
        let (tabs, before, after) = app.update(|cx| {
            (
                cx.new(|_| tabs),
                cx.focus_handle().tab_stop(true),
                cx.focus_handle().tab_stop(true),
            )
        });
        let before_focus = before.clone();
        let after_focus = after.clone();
        let root = app.add_window(move |_, _| KeyboardSurface {
            tabs,
            before,
            after,
        });
        let mut window = gpui::VisualTestContext::from_window(root.into(), app);
        window.update(|window, cx| {
            window.draw(cx).clear(cx);
            window.focus(&before_focus, cx);
        });
        (window, before_focus, after_focus)
    }

    fn focused(window: &mut gpui::VisualTestContext) -> gpui::FocusHandle {
        window.update(|window, cx| {
            window
                .focused(cx)
                .expect("a rendered control must hold focus")
        })
    }

    #[test]
    fn tabs_dispatch_manual_selection_and_exit_in_both_directions() {
        use std::{cell::RefCell, rc::Rc};
        for (position, next, prev) in [
            (TabPosition::Top, "right", "left"),
            (TabPosition::Bottom, "right", "left"),
            (TabPosition::Left, "down", "up"),
            (TabPosition::Right, "down", "up"),
        ] {
            let mut app = gpui::TestAppContext::single();
            let calls = Rc::new(RefCell::new(Vec::<SharedString>::new()));
            let tabs = Tabs::new("two")
                .position(position)
                .pane("one", "Same", |_, _| {
                    div().debug_selector(|| "one-panel".into()).child("One")
                })
                .pane("two", "Same", |_, _| {
                    div().debug_selector(|| "two-panel".into()).child("Two")
                })
                .on_tab_click({
                    let calls = calls.clone();
                    move |name, _, _| calls.borrow_mut().push(name)
                });
            let (mut window, before, after) = keyboard_surface(&mut app, tabs);
            press(&mut window, "tab");
            let second = focused(&mut window);
            assert_ne!(second, after);
            press(&mut window, next);
            let first = focused(&mut window);
            assert_ne!(first, second);
            assert!(window.debug_bounds("two-panel").is_some());
            assert!(calls.borrow().is_empty());

            press(&mut window, "tab");
            assert_eq!(
                focused(&mut window),
                after,
                "must not revisit selected header on exit"
            );
            press(&mut window, "shift-tab");
            assert_eq!(focused(&mut window), second);
            press(&mut window, "home");
            assert_eq!(focused(&mut window), first);
            press(&mut window, "space");
            assert!(window.debug_bounds("one-panel").is_some());
            press(&mut window, prev);
            assert_eq!(focused(&mut window), second);
            press(&mut window, "shift-tab");
            assert_eq!(focused(&mut window), before);
            press(&mut window, "tab");
            assert_eq!(focused(&mut window), first);
            press(&mut window, "end");
            assert_eq!(focused(&mut window), second);
            press(&mut window, "enter");
            assert!(window.debug_bounds("two-panel").is_some());
            assert_eq!(
                calls.borrow().as_slice(),
                &[SharedString::from("one"), SharedString::from("two")]
            );
            app.quit();
        }
    }

    #[test]
    fn tabs_missing_initial_key_and_duplicate_names_use_first_pane_without_activation() {
        let mut app = gpui::TestAppContext::single();
        let tabs = Tabs::new("missing")
            .pane("key", "Same", |_, _| {
                div()
                    .debug_selector(|| "original-panel".into())
                    .child("Original")
            })
            .pane("key", "Duplicate", |_, _| {
                div().debug_selector(|| "duplicate-panel".into())
            })
            .pane("other", "Same", |_, _| div().child("Other"))
            .on_tab_click(|_, _, _| panic!("initial selection/arrow movement must not activate"));
        let (mut window, _, _) = keyboard_surface(&mut app, tabs);
        press(&mut window, "tab");
        assert!(window.debug_bounds("original-panel").is_some());
        let first = focused(&mut window);
        press(&mut window, "right");
        assert_ne!(focused(&mut window), first);
        press(&mut window, "right");
        assert_eq!(
            focused(&mut window),
            first,
            "only the distinct other key is a second stop"
        );
        app.quit();
    }

    #[test]
    fn tabs_edit_actions_fire_once_and_leave_focus_on_a_surviving_control() {
        use std::{cell::RefCell, rc::Rc};
        let mut app = gpui::TestAppContext::single();
        let calls = Rc::new(RefCell::new(Vec::<String>::new()));
        let tabs = Tabs::new("one")
            .editable(true)
            .pane("one", "One", |_, _| {
                div().debug_selector(|| "one-panel".into()).child("One")
            })
            .pane("two", "Two", |_, _| div().child("Two"))
            .pane("three", "Three", |_, _| {
                div().debug_selector(|| "three-panel".into()).child("Three")
            })
            .on_tab_click(|_, _, _| panic!("Close/Add must not activate the enclosing tab"))
            .on_tab_remove({
                let calls = calls.clone();
                move |name, _, _| calls.borrow_mut().push(name.to_string())
            })
            .on_tab_add({
                let calls = calls.clone();
                move |_, _| calls.borrow_mut().push("add".into())
            });
        let (mut window, _, after) = keyboard_surface(&mut app, tabs);
        press(&mut window, "tab");
        let first = focused(&mut window);
        press(&mut window, "right");
        press(&mut window, "delete");
        let third = focused(&mut window);
        assert_ne!(third, first);
        assert!(
            window.debug_bounds("one-panel").is_some(),
            "removing the focused inactive tab must leave the selected panel mounted"
        );
        assert_eq!(calls.borrow().as_slice(), &["two"]);
        press(&mut window, "tab");
        press(&mut window, "enter");
        assert_eq!(
            focused(&mut window),
            first,
            "closing the last focused header uses its previous neighbor"
        );
        assert!(window.debug_bounds("three-panel").is_none());
        press(&mut window, "tab");
        press(&mut window, "space");
        let add = focused(&mut window);
        assert_ne!(add, first);
        assert!(window.debug_bounds("one-panel").is_none());
        assert_eq!(calls.borrow().as_slice(), &["two", "three", "one"]);
        press(&mut window, "tab");
        assert_eq!(focused(&mut window), after);
        press(&mut window, "shift-tab");
        assert_eq!(focused(&mut window), add);
        press(&mut window, "space");
        assert_ne!(focused(&mut window), add);
        assert_eq!(calls.borrow().as_slice(), &["two", "three", "one", "add"]);
        app.quit();
    }

    gpui::actions!(tabs_tests, [CompleteDeleteChord]);

    #[test]
    fn tabs_pending_delete_replay_removes_only_the_original_header() {
        use std::{cell::RefCell, rc::Rc};

        let mut app = gpui::TestAppContext::single();
        let removed = Rc::new(RefCell::new(Vec::new()));
        let log = removed.clone();
        let tabs = Tabs::new("three")
            .editable(true)
            .pane("one", "One", |_, _| div().child("One"))
            .pane("two", "Two", |_, _| {
                div()
                    .debug_selector(|| "delete-two-panel".into())
                    .child("Two")
            })
            .pane("three", "Three", |_, _| {
                div()
                    .debug_selector(|| "delete-three-panel".into())
                    .child("Three")
            })
            .on_tab_click(|_, _, _| panic!("Delete must not activate a tab"))
            .on_tab_remove(move |name, _, _| log.borrow_mut().push(name));
        let (mut window, _, _) = keyboard_surface(&mut app, tabs);
        app.update(|cx| {
            cx.bind_keys([gpui::KeyBinding::new(
                "delete delete x",
                CompleteDeleteChord,
                None,
            )]);
        });
        press(&mut window, "tab");
        // The first two inactive tabs still expose their independent Close stops.
        press(&mut window, "tab");
        press(&mut window, "tab");
        let last = focused(&mut window);
        press(&mut window, "left");
        let middle = focused(&mut window);
        assert_ne!(middle, last);
        press(&mut window, "end");
        assert_eq!(focused(&mut window), last);
        assert!(window.debug_bounds("delete-three-panel").is_some());

        for count in [1, 2] {
            press(&mut window, "delete");
            window.update(|window, _| {
                let pending = window
                    .pending_input_keystrokes()
                    .expect("Delete prefix must be buffered");
                assert_eq!(pending.len(), count);
                assert!(pending.iter().all(|stroke| stroke.key == "delete"));
            });
            assert!(removed.borrow().is_empty());
            assert_eq!(focused(&mut window), last);
        }

        // The mismatch replays both buffered Deletes in one GPUI dispatch.
        press(&mut window, "z");
        assert!(!window.update(|window, _| window.has_pending_keystrokes()));
        assert_eq!(removed.borrow().as_slice(), &[SharedString::from("three")]);
        assert_eq!(focused(&mut window), middle);
        assert!(window.debug_bounds("delete-two-panel").is_some());
        assert!(window.debug_bounds("delete-three-panel").is_none());
        app.quit();
    }

    #[test]
    fn clicking_header_padding_selects_the_tab() {
        use std::{cell::RefCell, rc::Rc};
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let calls = Rc::new(RefCell::new(Vec::new()));
        let clicked = calls.clone();
        let handle = app.add_window(move |_, _| {
            Tabs::new("second")
                .editable(true)
                .pane("first", "First", |_, _| {
                    div()
                        .debug_selector(|| "first-padding-panel".to_string())
                        .child("First panel")
                })
                .pane("second", "Second", |_, _| div().child("Second panel"))
                .on_tab_click(move |name, _, _| clicked.borrow_mut().push(name))
        });
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert!(window.debug_bounds("first-padding-panel").is_none());
        // The first header starts at x=0; its original px_2 leaves this point outside the label.
        window.simulate_click(point(px(3.0), px(20.0)), Modifiers::none());
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert!(window.debug_bounds("first-padding-panel").is_some());
        assert_eq!(calls.borrow().as_slice(), &[SharedString::from("first")]);
        app.quit();
    }

    #[test]
    fn closing_tab_keeps_a_remaining_pane_active() {
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let handle = app.add_window(|_, _| {
            Tabs::new("first")
                .editable(true)
                .pane("first", "", |_, _| div())
                .pane("second", "", |_, _| div())
        });
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        window.update(|window, cx| window.draw(cx).clear(cx));

        window.simulate_click(point(px(24.0), px(20.0)), Modifiers::none());

        handle
            .update(&mut app, |tabs, _, _| {
                assert_eq!(tabs.panes.len(), 1, "the click must remove the first pane");
                assert_eq!(tabs.panes[0].name, "second");
                assert_eq!(tabs.active_name, "second");
            })
            .unwrap();
        app.quit();
    }
}
