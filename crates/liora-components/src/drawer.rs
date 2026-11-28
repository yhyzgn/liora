//! Drawer module.
//!
//! This public module implements the Liora side drawer overlay component for secondary panels. It keeps the reusable
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
//! Opening, replacing, or removing a Drawer schedules redraws of the host windows;
//! callers do not need to refresh them manually. No-op stack updates do not request redraws.
//!
//! ## Design contract
//!
//! The implementation should use Liora theme tokens from `liora-core` and
//! `liora-theme`, keep accessibility-oriented keyboard/pointer behavior close to
//! the component, and avoid app-specific host-application resources in this SDK
//! crate.

use crate::gpui_compat::element_id;
use crate::motion::{fade_in, pop_in};
use gpui::{
    AnyElement, App, Context, IntoElement, KeyBinding, MouseButton, Pixels, Render, SharedString,
    Window, actions, div, prelude::*, px,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::sync::Arc;

actions!(
    drawer,
    [
        #[doc = "Keyboard action that closes the active drawer when dismissal is allowed."]
        DrawerClose
    ]
);

struct DrawerKeyBindingsInstalled;
impl gpui::Global for DrawerKeyBindingsInstalled {}

fn close_top_drawer_if_escape_enabled(cx: &mut App) {
    let Some(view) = cx
        .try_global::<liora_core::ActiveDrawer>()
        .and_then(|drawers| drawers.0.last())
        .map(|entry| entry.view.clone())
    else {
        cx.propagate();
        return;
    };
    let Ok(view) = view.downcast::<DrawerView>() else {
        cx.propagate();
        return;
    };
    let drawer = view.read(cx);
    if drawer.close_on_escape {
        let on_close = drawer.on_close.clone();
        on_close(cx);
    }
    cx.stop_propagation();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control drawer placement behavior.
pub enum DrawerPlacement {
    #[default]
    /// Places the overlay to the right of the anchor.
    Right,
    /// Places the overlay to the left of the anchor.
    Left,
    /// Places the overlay above the anchor.
    Top,
    /// Places the overlay below the anchor.
    Bottom,
}

/// Fluent native GPUI component for rendering Liora drawer.
pub struct Drawer {
    id: SharedString,
    title: SharedString,
    content: Arc<dyn Fn(&mut Window, &mut Context<DrawerView>) -> AnyElement + 'static>,
    placement: DrawerPlacement,
    width: Pixels,
    height: Pixels,
    close_on_click_outside: bool,
    close_on_escape: bool,
}

/// Fluent native GPUI component for rendering Liora drawer view.
pub struct DrawerView {
    id: SharedString,
    title: SharedString,
    content: Arc<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement + 'static>,
    placement: DrawerPlacement,
    width: Pixels,
    height: Pixels,
    close_on_click_outside: bool,
    close_on_escape: bool,
    on_close: Arc<dyn Fn(&mut App) + 'static>,
}

impl DrawerView {
    fn new(
        id: SharedString,
        title: SharedString,
        content: Arc<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement + 'static>,
        placement: DrawerPlacement,
        width: Pixels,
        height: Pixels,
        close_on_click_outside: bool,
        close_on_escape: bool,
        on_close: impl Fn(&mut App) + 'static,
    ) -> Self {
        Self {
            id,
            title,
            content,
            placement,
            width,
            height,
            close_on_click_outside,
            close_on_escape,
            on_close: Arc::new(on_close),
        }
    }
}

impl Render for DrawerView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let id = self.id.clone();
        let title = self.title.clone();
        let content_fn = self.content.clone();
        let on_close = self.on_close.clone();
        let placement = self.placement;
        let width = self.width;
        let height = self.height;
        let close_on_click_outside = self.close_on_click_outside;

        let mut container = div()
            .id(id.clone())
            .absolute()
            .size_full()
            .cursor_default()
            .bg(theme.neutral.overlay)
            .on_mouse_move(|_, _, cx| {
                cx.stop_propagation();
            })
            .on_mouse_down(MouseButton::Left, {
                let on_close = on_close.clone();
                move |_, _, cx| {
                    cx.stop_propagation();
                    if close_on_click_outside {
                        on_close(cx);
                    }
                }
            });

        let mut panel = div()
            .bg(theme.neutral.card)
            .cursor_default()
            .shadow_xl()
            // CONSUME mouse down inside the panel so it doesn't trigger the overlay close
            .on_mouse_move(|_, _, cx| {
                cx.stop_propagation();
            })
            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                cx.stop_propagation();
            });

        match placement {
            DrawerPlacement::Left => {
                container = container.flex().flex_row().justify_start();
                panel = panel
                    .h_full()
                    .w(width)
                    .border_r_1()
                    .border_color(theme.neutral.border);
            }
            DrawerPlacement::Right => {
                container = container.flex().flex_row().justify_end();
                panel = panel
                    .h_full()
                    .w(width)
                    .border_l_1()
                    .border_color(theme.neutral.border);
            }
            DrawerPlacement::Top => {
                container = container.flex().flex_col().justify_start();
                panel = panel
                    .w_full()
                    .h(height)
                    .border_b_1()
                    .border_color(theme.neutral.border);
            }
            DrawerPlacement::Bottom => {
                container = container.flex().flex_col().justify_end();
                panel = panel
                    .w_full()
                    .h(height)
                    .border_t_1()
                    .border_color(theme.neutral.border);
            }
        }

        fade_in(
            element_id(format!("{id}-overlay-motion")),
            container.child(pop_in(
                element_id(format!("{id}-panel-motion")),
                panel
                    .child(
                        div()
                            .p_4()
                            .border_b_1()
                            .border_color(theme.neutral.border)
                            .flex()
                            .justify_between()
                            .items_center()
                            .child(div().font_weight(gpui::FontWeight::BOLD).child(title))
                            .child(
                                div()
                                    .id(element_id(format!("{id}-close-btn")))
                                    .cursor_pointer()
                                    .child(
                                        Icon::new(IconName::X)
                                            .size(px(16.0))
                                            .color(theme.neutral.icon),
                                    )
                                    .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                                        on_close(cx);
                                    }),
                            ),
                    )
                    .child(div().flex_1().p_4().child(content_fn(_window, cx))),
            )),
        )
    }
}

impl Drawer {
    /// Registers GPUI key bindings required for keyboard interaction.
    pub fn register_key_bindings(cx: &mut App) {
        if cx.has_global::<DrawerKeyBindingsInstalled>() {
            return;
        }
        cx.bind_keys([KeyBinding::new("escape", DrawerClose, None)]);
        cx.on_action(|_: &DrawerClose, cx| close_top_drawer_if_escape_enabled(cx));
        cx.set_global(DrawerKeyBindingsInstalled);
    }

    /// Creates `Drawer` with default theme-driven styling and no optional callbacks attached.
    pub fn new() -> Self {
        Self {
            id: liora_core::unique_id("drawer"),
            title: SharedString::default(),
            content: Arc::new(|_, _| div().child("Drawer Content").into_any_element()),
            placement: DrawerPlacement::Right,
            width: px(300.0),
            height: px(300.0),
            close_on_click_outside: true,
            close_on_escape: true,
        }
    }

    /// Creates a lightweight sheet-style drawer for short contextual flows.
    ///
    /// This replaces the previous separate `Sheet` component: a sheet is just a
    /// Drawer with lighter default dimensions and the same overlay lifecycle.
    pub fn sheet() -> Self {
        Self::new()
            .id(liora_core::unique_id("drawer-sheet"))
            .width(px(360.0))
            .height(px(260.0))
    }

    /// Places the drawer on the left edge.
    pub fn left(self) -> Self {
        self.placement(DrawerPlacement::Left)
    }

    /// Places the drawer on the right edge.
    pub fn right(self) -> Self {
        self.placement(DrawerPlacement::Right)
    }

    /// Places the drawer on the top edge.
    pub fn top(self) -> Self {
        self.placement(DrawerPlacement::Top)
    }

    /// Places the drawer on the bottom edge.
    pub fn bottom(self) -> Self {
        self.placement(DrawerPlacement::Bottom)
    }

    /// Applies the former sheet wide-inspector preset.
    pub fn sheet_width_lg(self) -> Self {
        self.width(px(440.0))
    }

    /// Applies the former compact bottom-sheet height preset.
    pub fn sheet_height_sm(self) -> Self {
        self.height(px(220.0))
    }

    /// Sets content that only needs the current window, matching lightweight sheet use cases.
    pub fn content_view<F, E>(mut self, f: F) -> Self
    where
        F: Fn(&mut Window) -> E + 'static,
        E: IntoElement,
    {
        self.content = Arc::new(move |window, _cx| f(window).into_any_element());
        self
    }

    /// Assigns a stable element id used by GPUI state, hit testing, and automated interaction tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Sets the primary title text displayed by the component.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = title.into();
        self
    }

    /// Selects the popup, label, or overlay placement.
    pub fn placement(mut self, p: DrawerPlacement) -> Self {
        self.placement = p;
        self
    }

    /// Sets the component width token used during GPUI layout.
    pub fn width(mut self, w: impl Into<Pixels>) -> Self {
        self.width = w.into();
        self
    }

    /// Applies the predefined width lg sizing preset.
    pub fn width_lg(self) -> Self {
        self.width(px(480.0))
    }

    /// Sets the component height token used during GPUI layout.
    pub fn height(mut self, h: impl Into<Pixels>) -> Self {
        self.height = h.into();
        self
    }

    /// Applies the predefined height sm sizing preset.
    pub fn height_sm(self) -> Self {
        self.height(px(200.0))
    }

    /// Applies the predefined height lg sizing preset.
    pub fn height_lg(self) -> Self {
        self.height(px(360.0))
    }

    /// Toggles whether the popup closes when click outside occurs.
    pub fn close_on_click_outside(mut self, c: bool) -> Self {
        self.close_on_click_outside = c;
        self
    }

    /// Toggles whether the popup closes when escape occurs.
    pub fn close_on_escape(mut self, c: bool) -> Self {
        self.close_on_escape = c;
        self
    }

    /// Sets the rendered content element or text for this component.
    pub fn content<F, E>(mut self, f: F) -> Self
    where
        F: Fn(&mut Window, &mut Context<DrawerView>) -> E + 'static,
        E: IntoElement,
    {
        self.content = Arc::new(move |window, cx| f(window, cx).into_any_element());
        self
    }

    /// Performs the show operation used by this component.
    pub fn show(self, cx: &mut App) {
        Self::register_key_bindings(cx);
        let id = self.id;
        let title = self.title;
        let content = self.content;
        let placement = self.placement;
        let width = self.width;
        let height = self.height;
        let close_on_click_outside = self.close_on_click_outside;
        let close_on_escape = self.close_on_escape;

        let id_for_close = id.clone();
        let id_for_view = id.clone();
        let view = cx.new(|_cx| {
            let view_id = _cx.entity_id();
            DrawerView::new(
                id_for_view,
                title,
                content,
                placement,
                width,
                height,
                close_on_click_outside,
                close_on_escape,
                move |cx| {
                    if !cx
                        .try_global::<liora_core::ActiveDrawer>()
                        .is_some_and(|drawers| {
                            drawers.0.iter().any(|entry| {
                                entry.id == id_for_close && entry.view.entity_id() == view_id
                            })
                        })
                    {
                        return;
                    }
                    liora_core::clear_drawer(&id_for_close, cx);
                },
            )
        });

        liora_core::set_active_drawer(id, view.into(), cx);
    }

    /// Closes only the topmost drawer, leaving any underlying drawers open.
    pub fn close(cx: &mut App) {
        if let Some(id) = cx
            .global::<liora_core::ActiveDrawer>()
            .0
            .last()
            .map(|entry| entry.id.clone())
        {
            liora_core::clear_drawer(&id, cx);
        }
    }

    /// Closes the drawer with the given ID, regardless of its stack position.
    pub fn close_id(id: impl Into<SharedString>, cx: &mut App) {
        let id = id.into();
        liora_core::clear_drawer(&id, cx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_drawer_mutations_repaint_without_extra_input() {
        use std::{cell::Cell, rc::Rc};

        struct Probe(&'static str);
        impl Render for Probe {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                let name = self.0;
                div()
                    .debug_selector(move || name.into())
                    .w(px(40.0))
                    .h(px(20.0))
            }
        }
        struct Root(Rc<Cell<usize>>);
        impl Render for Root {
            fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
                self.0.set(self.0.get() + 1);
                liora_core::render_active_drawer_in_window(window, cx);
                div().size_full().child(crate::PortalLayer)
            }
        }

        let mut stale = Vec::new();
        for operation in ["insert", "replace", "clear-id", "clear-all", "close-top"] {
            let mut app = gpui::TestAppContext::single();
            app.update(crate::init_liora);
            let renders = Rc::new(Cell::new(0));
            let root = app.add_window(|_, _| Root(renders.clone()));
            let mut window = gpui::VisualTestContext::from_window(root.into(), &app);
            let parent = app.new(|_| Probe("drawer-parent"));
            let child = app.new(|_| Probe("drawer-child"));
            let replacement = app.new(|_| Probe("drawer-replacement"));
            app.update(|cx| {
                liora_core::set_active_drawer("parent".into(), parent.clone().into(), cx);
                if operation != "insert" {
                    liora_core::set_active_drawer("child".into(), child.clone().into(), cx);
                }
            });
            window.update(|window, cx| window.draw(cx).clear(cx));
            app.run_until_parked();
            assert!(window.debug_bounds("drawer-parent").is_some());
            assert_eq!(
                window.debug_bounds("drawer-child").is_some(),
                operation != "insert"
            );

            let notifications = Rc::new(Cell::new(0));
            let observed = notifications.clone();
            let _subscription = app.update(|cx| {
                cx.observe_global::<liora_core::ActiveDrawer>(move |_| {
                    observed.set(observed.get() + 1)
                })
            });
            app.update(|cx| match operation {
                "insert" => liora_core::set_active_drawer("child".into(), child.clone().into(), cx),
                "replace" => {
                    liora_core::set_active_drawer("child".into(), replacement.clone().into(), cx)
                }
                "clear-id" => Drawer::close_id("child", cx),
                "clear-all" => liora_core::clear_active_drawer(cx),
                "close-top" => Drawer::close(cx),
                _ => unreachable!(),
            });
            app.run_until_parked();
            // Inspect the naturally painted frame; never draw, refresh, or send input after mutation.
            let painted = (
                window.debug_bounds("drawer-parent").is_some(),
                window.debug_bounds("drawer-child").is_some(),
                window.debug_bounds("drawer-replacement").is_some(),
            );
            let expected = (
                operation != "clear-all",
                operation == "insert",
                operation == "replace",
            );
            if painted != expected {
                stale.push(format!(
                    "{operation}: rendered {painted:?}, expected {expected:?}"
                ));
            }
            app.update(|cx| {
                let drawers = &cx.global::<liora_core::ActiveDrawer>().0;
                assert_eq!(
                    drawers.len(),
                    usize::from(expected.0) + usize::from(expected.1 || expected.2)
                );
                if expected.0 {
                    assert_eq!(drawers[0].view.entity_id(), parent.entity_id());
                }
            });
            let before_noop = (renders.get(), notifications.get());
            app.update(|cx| {
                liora_core::clear_drawer(&"missing".into(), cx);
                if operation != "clear-all" {
                    liora_core::set_active_drawer("parent".into(), parent.clone().into(), cx);
                }
                if operation == "clear-all" {
                    liora_core::clear_active_drawer(cx);
                    Drawer::close(cx);
                }
            });
            app.run_until_parked();
            if (renders.get(), notifications.get()) != before_noop {
                stale.push(format!("{operation}: no-op changed render/notification counts from {before_noop:?} to {:?}", (renders.get(), notifications.get())));
            }
            app.quit();
        }
        assert!(
            stale.is_empty(),
            "stale rendered state or dirty no-op: {stale:?}"
        );
    }

    struct KeyboardRoot(gpui::FocusHandle);

    impl Render for KeyboardRoot {
        fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            liora_core::render_active_drawer_in_window(window, cx);
            div()
                .relative()
                .size_full()
                .track_focus(&self.0)
                .child(crate::PortalLayer)
        }
    }

    fn dispatch_escape(window: &mut gpui::VisualTestContext) -> bool {
        window.update(|window, cx| {
            let keystroke =
                gpui::Keystroke::parse_with_os("escape", cx.platform_info().os).unwrap();
            let result = window.dispatch_event(
                gpui::PlatformInput::KeyDown(gpui::KeyDownEvent {
                    keystroke: keystroke.clone(),
                    is_held: false,
                    prefer_character_input: false,
                }),
                cx,
            );
            window.dispatch_event(
                gpui::PlatformInput::KeyUp(gpui::KeyUpEvent { keystroke }),
                cx,
            );
            result.propagate
        })
    }

    fn click_at(window: &mut gpui::VisualTestContext, position: gpui::Point<gpui::Pixels>) {
        window.update(|window, cx| {
            window.dispatch_event(
                gpui::PlatformInput::MouseDown(gpui::MouseDownEvent {
                    position,
                    button: MouseButton::Left,
                    modifiers: gpui::Modifiers::none(),
                    click_count: 1,
                    first_mouse: false,
                }),
                cx,
            );
            window.draw(cx).clear(cx);
            window.dispatch_event(
                gpui::PlatformInput::MouseUp(gpui::MouseUpEvent {
                    position,
                    button: MouseButton::Left,
                    modifiers: gpui::Modifiers::none(),
                    click_count: 1,
                }),
                cx,
            );
        });
    }

    #[test]
    fn top_drawer_outside_policy_preserves_parent_and_retires_child_hitbox() {
        use std::{cell::Cell, rc::Rc};

        for close_child_on_outside in [false, true] {
            let mut app = gpui::TestAppContext::single();
            app.update(|cx| {
                crate::init_liora(cx);
                cx.set_reduce_motion(true);
            });
            let trigger = app.update(|cx| cx.focus_handle());
            let child_focus = app.update(|cx| cx.focus_handle());
            let root = app.add_window(|_, _| KeyboardRoot(trigger.clone()));
            let mut window = gpui::VisualTestContext::from_window(root.into(), &app);
            let parent_clicks = Rc::new(Cell::new(0));
            window.update(|window, cx| {
                window.activate_window();
                Drawer::new()
                    .id("parent")
                    .content({
                        let parent_clicks = parent_clicks.clone();
                        move |_, _| {
                            let parent_clicks = parent_clicks.clone();
                            div()
                                .debug_selector(|| "parent-action".to_string())
                                .on_mouse_down(MouseButton::Left, move |_, _, _| {
                                    parent_clicks.set(parent_clicks.get() + 1)
                                })
                                .child("Parent action")
                        }
                    })
                    .show(cx);
                Drawer::new()
                    .id("child")
                    .close_on_click_outside(close_child_on_outside)
                    .close_on_escape(false)
                    .content({
                        let child_focus = child_focus.clone();
                        move |_, _| {
                            div().id("child-content").track_focus(&child_focus).child(
                                div()
                                    .debug_selector(|| "child-business-close".to_string())
                                    .on_mouse_down(MouseButton::Left, |_, _, cx| Drawer::close(cx))
                                    .child("Close child"),
                            )
                        }
                    })
                    .show(cx);
                window.focus(&child_focus, cx);
            });
            app.run_until_parked();
            assert!(window.update(|window, _| child_focus.is_focused(window)));
            assert!(window.debug_bounds("child-business-close").is_some());

            assert!(!dispatch_escape(&mut window));
            window.update(|_, cx| {
                let drawers = &cx.global::<liora_core::ActiveDrawer>().0;
                assert_eq!(drawers.len(), 2);
                assert_eq!(drawers[0].id.as_ref(), "parent");
                assert_eq!(drawers[1].id.as_ref(), "child");
            });

            click_at(&mut window, gpui::point(px(1.0), px(1.0)));
            app.run_until_parked();
            window.update(|_, cx| {
                let drawers = &cx.global::<liora_core::ActiveDrawer>().0;
                assert_eq!(drawers.len(), if close_child_on_outside { 1 } else { 2 });
                assert_eq!(drawers[0].id.as_ref(), "parent");
                if !close_child_on_outside {
                    assert_eq!(drawers[1].id.as_ref(), "child");
                }
            });

            if !close_child_on_outside {
                window.update(|window, cx| window.focus(&child_focus, cx));
                assert!(window.update(|window, _| child_focus.is_focused(window)));
                let child_close = window
                    .debug_bounds("child-business-close")
                    .unwrap()
                    .center();
                click_at(&mut window, child_close);
                app.run_until_parked();
                window.update(|_, cx| {
                    let drawers = &cx.global::<liora_core::ActiveDrawer>().0;
                    assert_eq!(drawers.len(), 1);
                    assert_eq!(drawers[0].id.as_ref(), "parent");
                });
            }
            assert!(window.debug_bounds("child-business-close").is_none());

            let parent_action = window.debug_bounds("parent-action").unwrap().center();
            click_at(&mut window, parent_action);
            app.run_until_parked();
            assert_eq!(parent_clicks.get(), 1);
            assert!(!dispatch_escape(&mut window));
            app.run_until_parked();
            window.update(|_, cx| assert!(cx.global::<liora_core::ActiveDrawer>().0.is_empty()));
            assert!(window.debug_bounds("parent-action").is_none());
            app.quit();
        }
    }

    #[test]
    fn trigger_focused_escape_respects_top_drawer_policy() {
        for core_only in [false, true] {
            let mut app = gpui::TestAppContext::single();
            app.update(|cx| {
                if core_only {
                    liora_core::init_liora(cx, liora_theme::Theme::light());
                } else {
                    crate::init_liora(cx);
                }
                cx.set_reduce_motion(true);
            });
            let focus = app.update(|cx| cx.focus_handle());
            let root = app.add_window(|_, _| KeyboardRoot(focus.clone()));
            let mut window = gpui::VisualTestContext::from_window(root.into(), &app);
            window.update(|window, cx| {
                window.activate_window();
                window.focus(&focus, cx);
                Drawer::new().id("parent").show(cx);
                Drawer::new().id("child").close_on_escape(false).show(cx);
            });
            app.run_until_parked();
            dispatch_escape(&mut window);
            window.update(|window, cx| {
                assert_eq!(cx.global::<liora_core::ActiveDrawer>().0.len(), 2);
                assert!(focus.is_focused(window));
                Drawer::close(cx);
            });
            app.run_until_parked();
            dispatch_escape(&mut window);
            window.update(|window, cx| {
                assert!(
                    cx.global::<liora_core::ActiveDrawer>().0.is_empty(),
                    "trigger Escape did not close the enabled parent; core_only={core_only}"
                );
                assert!(focus.is_focused(window), "the trigger lost focus");
            });
            assert!(
                dispatch_escape(&mut window),
                "an empty drawer stack swallowed Escape"
            );
            app.quit();
        }
    }

    #[test]
    fn escape_inside_parent_obeys_child_policy_and_closes_only_child() {
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let trigger = app.update(|cx| cx.focus_handle());
        let parent_focus = app.update(|cx| cx.focus_handle());
        let root = app.add_window(|_, _| KeyboardRoot(trigger));
        let mut window = gpui::VisualTestContext::from_window(root.into(), &app);
        window.update(|window, cx| {
            cx.set_reduce_motion(true);
            window.activate_window();
            Drawer::new()
                .id("parent")
                .content({
                    let focus = parent_focus.clone();
                    move |_, _| div().track_focus(&focus).child("parent content")
                })
                .show(cx);
            Drawer::new().id("child").close_on_escape(false).show(cx);
            window.focus(&parent_focus, cx);
        });
        app.run_until_parked();
        dispatch_escape(&mut window);
        window.update(|_, cx| {
            let drawers = &cx.global::<liora_core::ActiveDrawer>().0;
            assert_eq!(drawers.len(), 2, "Escape bypassed the protected child");
            assert_eq!(drawers[0].id.as_ref(), "parent");
            Drawer::new().id("child").show(cx);
        });
        app.run_until_parked();
        dispatch_escape(&mut window);
        window.update(|window, cx| {
            let drawers = &cx.global::<liora_core::ActiveDrawer>().0;
            assert_eq!(drawers.len(), 1);
            assert_eq!(
                drawers[0].id.as_ref(),
                "parent",
                "Escape closed the focused parent instead of the top child"
            );
            assert!(parent_focus.is_focused(window));
        });
        app.quit();
    }

    #[test]
    fn manual_close_only_removes_topmost_drawer() {
        let mut app = gpui::TestAppContext::single();
        let window = app.add_empty_window();
        window.update(|_, cx| {
            liora_core::init_liora(cx, liora_theme::Theme::light());
            Drawer::new().id("parent").show(cx);
            let parent = cx.global::<liora_core::ActiveDrawer>().0[0]
                .view
                .entity_id();
            Drawer::new().id("child").show(cx);

            Drawer::close(cx);
            let drawers = &cx.global::<liora_core::ActiveDrawer>().0;
            assert_eq!(drawers.len(), 1);
            assert_eq!(drawers[0].view.entity_id(), parent);

            Drawer::close(cx);
            assert!(cx.global::<liora_core::ActiveDrawer>().0.is_empty());
            Drawer::close(cx);
            assert!(cx.global::<liora_core::ActiveDrawer>().0.is_empty());

            Drawer::new().id("parent").show(cx);
            Drawer::new().id("child").show(cx);
            Drawer::close_id("parent", cx);
            let drawers = &cx.global::<liora_core::ActiveDrawer>().0;
            assert_eq!(drawers.len(), 1);
            assert_eq!(drawers[0].id.as_ref(), "child");
        });
        app.quit();
    }

    #[test]
    fn stale_close_does_not_clear_same_id_replacement() {
        let mut app = gpui::TestAppContext::single();
        let window = app.add_empty_window();
        let (old_close, current_close, replacement_id) = window.update(|_, cx| {
            liora_core::init_liora(cx, liora_theme::Theme::light());
            Drawer::new().id("shared-drawer").show(cx);
            let old_view = cx
                .global::<liora_core::ActiveDrawer>()
                .0
                .last()
                .unwrap()
                .view
                .clone()
                .downcast::<DrawerView>()
                .unwrap();
            let old_close = old_view.read(cx).on_close.clone();

            Drawer::new()
                .id("shared-drawer")
                .title("Replacement")
                .show(cx);
            let replacement_view = cx
                .global::<liora_core::ActiveDrawer>()
                .0
                .last()
                .unwrap()
                .view
                .clone()
                .downcast::<DrawerView>()
                .unwrap();
            let replacement_id = replacement_view.entity_id();
            let current_close = replacement_view.read(cx).on_close.clone();
            (old_close, current_close, replacement_id)
        });

        window.update(|_, cx| old_close(cx));
        window.update(|_, cx| {
            let active = cx.global::<liora_core::ActiveDrawer>().0.last().unwrap();
            assert_eq!(active.view.entity_id(), replacement_id);
        });
        window.update(|_, cx| current_close(cx));
        window.update(|_, cx| current_close(cx));
        window.update(|_, cx| assert!(cx.global::<liora_core::ActiveDrawer>().0.is_empty()));
        app.quit();
    }

    #[test]
    fn drawer_size_helpers_set_demo_sizes() {
        assert_eq!(Drawer::new().width_lg().width, px(480.0));
        assert_eq!(Drawer::new().height_sm().height, px(200.0));
        assert_eq!(Drawer::new().height_lg().height, px(360.0));
    }

    #[test]
    fn drawer_includes_lightweight_sheet_presets() {
        let sheet = Drawer::sheet().left().sheet_width_lg().sheet_height_sm();
        assert_eq!(sheet.placement, DrawerPlacement::Left);
        assert_eq!(sheet.width, px(440.0));
        assert_eq!(sheet.height, px(220.0));
    }
}
