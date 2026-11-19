//! Checkbox module.
//!
//! This public module implements the Liora single checkbox control with keyboard and pointer interaction. It keeps the reusable
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

use crate::motion::pop_in;
use gpui::{
    App, Context, EventEmitter, FocusHandle, Focusable, Hsla, KeyBinding, MouseButton, Render, Rgba,
    Window, prelude::*, px,
};
use liora_core::LocalizedText;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

fn rgba(r: u8, g: u8, b: u8, a: f32) -> Hsla {
    Rgba {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a,
    }
    .into()
}

gpui::actions!(
    checkbox,
    [
        #[doc = "Keyboard action that toggles the focused checkbox."]
        CheckboxToggle
    ]
);

/// Fluent native GPUI component for rendering Liora checkbox.
pub struct Checkbox {
    checked: bool,
    disabled: bool,
    label: Option<LocalizedText>,
    aria_label: Option<LocalizedText>,
    pointer_armed: bool,
    focus_handle: FocusHandle,
    on_change: Option<Box<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
}

#[derive(Clone, Copy)]
/// Fluent native GPUI component for rendering Liora checkbox changed.
pub struct CheckboxChanged(pub bool);

impl EventEmitter<CheckboxChanged> for Checkbox {}

impl Checkbox {
    /// Creates `Checkbox` initialized from the supplied checked.
    pub fn new(checked: bool, cx: &mut Context<Self>) -> Self {
        Self {
            checked,
            disabled: false,
            label: None,
            aria_label: None,
            pointer_armed: false,
            focus_handle: cx.focus_handle().tab_stop(true),
            on_change: None,
        }
    }

    /// Toggles the disabled state and suppresses user interaction when enabled.
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self.pointer_armed = false;
        self.focus_handle = self.focus_handle.tab_stop(!d);
        self
    }
    /// Returns the stable user-facing label for this value.
    pub fn label(mut self, text: impl Into<LocalizedText>) -> Self {
        self.label = Some(text.into());
        self
    }
    /// Sets the accessible name without changing the visible label.
    /// Without an override, the visible label supplies the accessible name.
    pub fn aria_label(mut self, label: impl Into<LocalizedText>) -> Self {
        self.aria_label = Some(label.into());
        self
    }
    /// Registers a callback that runs when change occurs.
    pub fn on_change(mut self, cb: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(cb));
        self
    }

    pub(crate) fn sync(
        &mut self,
        checked: bool,
        on_change: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) {
        self.checked = checked;
        self.on_change = Some(Box::new(on_change));
    }

    /// Updates the stored disabled value and keeps the existing component identity.
    pub fn set_disabled(&mut self, d: bool, cx: &mut Context<Self>) {
        self.disabled = d;
        self.pointer_armed = false;
        self.focus_handle = self.focus_handle.clone().tab_stop(!d);
        cx.notify();
    }

    /// Registers GPUI key bindings required for keyboard interaction.
    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([
            KeyBinding::new("space", CheckboxToggle, None),
            KeyBinding::new("enter", CheckboxToggle, None),
        ]);
    }

    fn toggle(&mut self, _: &CheckboxToggle, window: &mut Window, cx: &mut Context<Self>) {
        if !self.disabled {
            self.checked = !self.checked;
            cx.emit(CheckboxChanged(self.checked));
            cx.notify();
            if let Some(ref cb) = self.on_change {
                cb(self.checked, window, cx);
            }
        }
    }
}

impl Focusable for Checkbox {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Checkbox {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<liora_core::Config>().theme;
        let focused = self.focus_handle.is_focused(_window);
        let sz = 16.0;

        let (bg, border, check_color) = if self.disabled {
            (
                theme.neutral.hover,
                theme.neutral.border,
                theme.neutral.text_disabled,
            )
        } else if self.checked {
            (
                theme.primary.base,
                theme.primary.base,
                theme.neutral.inverted,
            )
        } else {
            (
                rgba(0, 0, 0, 0.0),
                if focused {
                    theme.primary.base
                } else {
                    theme.neutral.border
                },
                rgba(0, 0, 0, 0.0),
            )
        };

        let label = self.label.as_ref().map(|label| label.resolve(cx));
        let name = self.aria_label.as_ref().map(|label| label.resolve(cx)).or_else(|| label.clone());
        let disabled = self.disabled;
        let mut row = gpui::div()
            .id(("liora-checkbox", cx.entity().entity_id()))
            .role(gpui::Role::CheckBox)
            .aria_toggled(self.checked.into())
            .when_some(name, |row, name| row.aria_label(name))
            .when(disabled, |row| row.a11y_synthetic_children(|builder| builder.parent_node().set_disabled()))
            .track_focus(&self.focus_handle)
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .on_action(cx.listener(Self::toggle))
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, _| this.pointer_armed = !this.disabled))
            .on_click(cx.listener(|this, event, window, cx| {
                // Keep key-down activation; an interrupted pointer press must not survive disabling.
                let armed = std::mem::take(&mut this.pointer_armed);
                if matches!(event, gpui::ClickEvent::Mouse(_)) && armed {
                    this.toggle(&CheckboxToggle, window, cx);
                }
            }))
            .on_key_down(|event, window, cx| {
                let modifiers = event.keystroke.modifiers;
                if event.keystroke.key == "tab" && !modifiers.control && !modifiers.alt && !modifiers.platform && !modifiers.function {
                    if modifiers.shift { window.focus_prev(cx); } else { window.focus_next(cx); }
                    cx.stop_propagation();
                }
            });

        if !self.disabled {
            row = row.cursor_pointer();
        } else {
            row = row.cursor_not_allowed().capture_any_mouse_down(|_, window, _| window.prevent_default());
        }

        let mut box_el = gpui::div()
            .flex_none()
            .relative()
            .w(px(sz))
            .h(px(sz))
            .rounded(px(2.0))
            .bg(bg)
            .border_1()
            .border_color(border)
            .flex()
            .items_center()
            .justify_center();

        if focused && !self.disabled {
            box_el = box_el.child(
                gpui::div()
                    .absolute()
                    .top(px(-4.0))
                    .left(px(-4.0))
                    .size(px(sz + 6.0))
                    .rounded(px(4.0))
                    .border_2()
                    .border_color(theme.primary.base),
            );
        }

        if self.checked {
            box_el = box_el.child(pop_in(
                ("liora-checkbox-check-motion", cx.entity().entity_id()),
                gpui::div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(Icon::new(IconName::Check).size(px(12.0)).color(check_color)),
            ));
        }

        row = row.child(box_el);

        if let Some(label) = label {
            row = row.child(
                gpui::div()
                    .text_size(px(theme.font_size.md))
                    .text_color(theme.neutral.text_1)
                    .child(label),
            );
        }

        row
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Button, Switch};
    use gpui::{AppContext, Entity, Modifiers, VisualTestContext, point};
    use std::{cell::RefCell, rc::Rc};

    struct ToggleSurface {
        checkbox: Entity<Checkbox>,
        switch: Entity<Switch>,
        changes: Rc<RefCell<Vec<(&'static str, bool)>>>,
    }

    impl Render for ToggleSurface {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            gpui::div()
                .flex()
                .gap_4()
                .child(Button::new("Before").id("toggle-before"))
                .child(gpui::div().debug_selector(|| "checkbox".into()).child(self.checkbox.clone()))
                .child(gpui::div().debug_selector(|| "switch".into()).child(self.switch.clone()))
                .child(Button::new("After").id("toggle-after").on_click({
                    let changes = self.changes.clone();
                    move |_, _, _| changes.borrow_mut().push(("after", true))
                }))
        }
    }

    fn surface(app: &mut gpui::TestAppContext) -> (VisualTestContext, Entity<ToggleSurface>) {
        app.update(crate::init_liora);
        let changes = Rc::new(RefCell::new(Vec::new()));
        let handle = app.add_window(move |_, cx| ToggleSurface {
            checkbox: cx.new(|cx| Checkbox::new(false, cx).label("Remember choice").on_change({
                let changes = changes.clone();
                move |checked, _, _| changes.borrow_mut().push(("checkbox", checked))
            })),
            switch: cx.new(|cx| Switch::new(false, cx).aria_label("Background sync").on_change({
                let changes = changes.clone();
                move |checked, _, _| changes.borrow_mut().push(("switch", checked))
            })),
            changes,
        });
        let entity = handle.root(app).unwrap();
        let mut window = VisualTestContext::from_window(handle.into(), app);
        draw(&mut window);
        (window, entity)
    }

    fn draw(window: &mut VisualTestContext) {
        window.update(|window, cx| window.draw(cx).clear(cx));
    }

    fn press(window: &mut VisualTestContext, key: &str) {
        window.simulate_keystrokes(key);
        window.update(|window, cx| {
            let keystroke = gpui::Keystroke::parse_with_os(key, cx.platform_info().os).unwrap();
            window.dispatch_event(gpui::PlatformInput::KeyUp(gpui::KeyUpEvent { keystroke }), cx);
        });
        draw(window);
    }

    #[test]
    fn toggle_controls_require_an_inside_press_and_release_across_redraws() {
        let mut app = gpui::TestAppContext::single();
        let (mut window, root) = surface(&mut app);
        let outside = point(px(700.0), px(200.0));
        for selector in ["checkbox", "switch"] {
            let inside = window.debug_bounds(selector).unwrap().origin + point(px(8.0), px(8.0));
            window.simulate_mouse_down(outside, MouseButton::Left, Modifiers::none());
            window.simulate_mouse_move(inside, Some(MouseButton::Left), Modifiers::none());
            draw(&mut window);
            window.simulate_mouse_up(inside, MouseButton::Left, Modifiers::none());
            draw(&mut window);
            assert!(root.read_with(&app, |surface, _| surface.changes.borrow().is_empty()));
            window.simulate_mouse_down(inside, MouseButton::Left, Modifiers::none());
            draw(&mut window);
            window.simulate_mouse_up(outside, MouseButton::Left, Modifiers::none());
            draw(&mut window);
            assert!(root.read_with(&app, |surface, _| surface.changes.borrow().is_empty()));
        }
        for selector in ["checkbox", "switch"] {
            let inside = window.debug_bounds(selector).unwrap().origin + point(px(8.0), px(8.0));
            window.simulate_mouse_down(inside, MouseButton::Left, Modifiers::none());
            draw(&mut window);
            window.simulate_mouse_up(inside, MouseButton::Left, Modifiers::none());
            draw(&mut window);
        }
        root.read_with(&app, |surface, _| assert_eq!(*surface.changes.borrow(), [("checkbox", true), ("switch", true)]));
        app.quit();
    }

    #[test]
    fn toggle_controls_traverse_and_activate_once_with_full_application_init() {
        let mut app = gpui::TestAppContext::single();
        let (mut window, root) = surface(&mut app);
        window.update(|window, cx| window.focus_next(cx));
        draw(&mut window);
        let checkbox_bounds = window.debug_bounds("checkbox").unwrap();
        let switch_bounds = window.debug_bounds("switch").unwrap();
        press(&mut window, "tab");
        let checkbox_focus = root.read_with(&app, |surface, cx| surface.checkbox.focus_handle(cx));
        window.update(|window, _| assert!(checkbox_focus.is_focused(window)));
        press(&mut window, "space");
        assert_eq!(window.debug_bounds("checkbox"), Some(checkbox_bounds));
        press(&mut window, "enter");
        press(&mut window, "ctrl-enter");
        press(&mut window, "tab");
        let switch_focus = root.read_with(&app, |surface, cx| surface.switch.focus_handle(cx));
        window.update(|window, _| assert!(switch_focus.is_focused(window)));
        press(&mut window, "enter");
        assert_eq!(window.debug_bounds("switch"), Some(switch_bounds));
        press(&mut window, "space");
        press(&mut window, "tab");
        press(&mut window, "enter");
        press(&mut window, "shift-tab");
        press(&mut window, "shift-tab");
        press(&mut window, "space");
        root.read_with(&app, |surface, _| assert_eq!(*surface.changes.borrow(), [("checkbox", true), ("checkbox", false), ("switch", true), ("switch", false), ("after", true), ("checkbox", true)]));
        app.quit();
    }

    #[test]
    fn focused_disabled_toggle_controls_stay_inert_and_reenable_with_same_identity() {
        let mut app = gpui::TestAppContext::single();
        let (mut window, root) = surface(&mut app);
        let (checkbox, switch) = root.read_with(&app, |surface, _| (surface.checkbox.clone(), surface.switch.clone()));
        let checkbox_focus = checkbox.read_with(&app, |checkbox, cx| checkbox.focus_handle(cx));
        let switch_focus = switch.read_with(&app, |switch, cx| switch.focus_handle(cx));
        for selector in ["checkbox", "switch"] {
            let inside = window.debug_bounds(selector).unwrap().origin + point(px(8.0), px(8.0));
            window.simulate_mouse_down(inside, MouseButton::Left, Modifiers::none());
            window.update(|window, cx| {
                checkbox.update(cx, |checkbox, cx| checkbox.set_disabled(true, cx));
                switch.update(cx, |switch, cx| switch.set_disabled(true, cx));
                window.draw(cx).clear(cx);
            });
            window.simulate_mouse_up(inside, MouseButton::Left, Modifiers::none());
            press(&mut window, "enter");
            press(&mut window, "space");
            window.simulate_click(inside, Modifiers::none());
            draw(&mut window);
            assert!(root.read_with(&app, |surface, _| surface.changes.borrow().iter().all(|(name, _)| *name == "after")));
            press(&mut window, "tab");
            press(&mut window, "enter");
            window.update(|window, cx| {
                checkbox.update(cx, |checkbox, cx| checkbox.set_disabled(false, cx));
                switch.update(cx, |switch, cx| switch.set_disabled(false, cx));
                assert_eq!(checkbox.focus_handle(cx), checkbox_focus);
                assert_eq!(switch.focus_handle(cx), switch_focus);
                window.draw(cx).clear(cx);
            });
        }
        press(&mut window, "shift-tab");
        press(&mut window, "space");
        press(&mut window, "shift-tab");
        press(&mut window, "enter");
        root.read_with(&app, |surface, _| assert_eq!(*surface.changes.borrow(), [("after", true), ("after", true), ("switch", true), ("checkbox", true)]));
        app.quit();
    }

    #[test]
    fn toggle_controls_cancel_a_press_when_disabled_then_reenabled_before_release() {
        let mut app = gpui::TestAppContext::single();
        let (mut window, root) = surface(&mut app);
        let (checkbox, switch) = root.read_with(&app, |surface, _| (surface.checkbox.clone(), surface.switch.clone()));
        for selector in ["checkbox", "switch"] {
            let inside = window.debug_bounds(selector).unwrap().origin + point(px(8.0), px(8.0));
            window.simulate_mouse_down(inside, MouseButton::Left, Modifiers::none());
            for disabled in [true, false] {
                window.update(|window, cx| {
                    checkbox.update(cx, |checkbox, cx| checkbox.set_disabled(disabled, cx));
                    switch.update(cx, |switch, cx| switch.set_disabled(disabled, cx));
                    window.draw(cx).clear(cx);
                });
            }
            window.simulate_mouse_up(inside, MouseButton::Left, Modifiers::none());
            draw(&mut window);
            assert!(root.read_with(&app, |surface, _| surface.changes.borrow().is_empty()));
            window.simulate_click(inside, Modifiers::none());
            draw(&mut window);
            root.read_with(&app, |surface, _| assert_eq!(surface.changes.borrow().last(), Some(&(selector, true))));
            root.read_with(&app, |surface, _| surface.changes.borrow_mut().clear());
        }
        app.quit();
    }

    #[test]
    fn toggle_controls_write_role_checked_and_name_on_the_stable_native_root() {
        use gpui::{Element, accesskit::{Node, Toggled}};
        let mut app = gpui::TestAppContext::single();
        let (mut window, root) = surface(&mut app);
        let (checkbox, switch) = root.read_with(&app, |surface, _| (surface.checkbox.clone(), surface.switch.clone()));
        let initial = window.update(|window, cx| {
            let checkbox_id = checkbox.update(cx, |checkbox, cx| {
                let element = checkbox.render(window, cx).into_element();
                assert_eq!(element.a11y_role(), Some(gpui::Role::CheckBox));
                let mut node = Node::new(element.a11y_role().unwrap());
                element.write_a11y_info(&mut node);
                assert_eq!(node.label(), Some("Remember choice"));
                assert_eq!(node.toggled(), Some(Toggled::False));
                element.id()
            });
            let switch_id = switch.update(cx, |switch, cx| {
                let element = switch.render(window, cx).into_element();
                assert_eq!(element.a11y_role(), Some(gpui::Role::Switch));
                let mut node = Node::new(element.a11y_role().unwrap());
                element.write_a11y_info(&mut node);
                assert_eq!(node.label(), Some("Background sync"));
                assert_eq!(node.toggled(), Some(Toggled::False));
                element.id()
            });
            (checkbox_id, switch_id)
        });
        for selector in ["checkbox", "switch"] {
            let inside = window.debug_bounds(selector).unwrap().origin + point(px(8.0), px(8.0));
            window.simulate_click(inside, Modifiers::none());
            draw(&mut window);
        }
        window.update(|window, cx| {
            checkbox.update(cx, |checkbox, cx| {
                checkbox.aria_label = Some("Remember this account".into());
                let element = checkbox.render(window, cx).into_element();
                let mut node = Node::new(gpui::Role::CheckBox);
                element.write_a11y_info(&mut node);
                assert_eq!(node.label(), Some("Remember this account"));
                assert_eq!(node.toggled(), Some(Toggled::True));
                assert_eq!(element.id(), initial.0);
                assert_eq!(checkbox.label.as_ref().unwrap().resolve(cx).as_ref(), "Remember choice");
            });
            switch.update(cx, |switch, cx| {
                let element = switch.render(window, cx).into_element();
                let mut node = Node::new(gpui::Role::Switch);
                element.write_a11y_info(&mut node);
                assert_eq!(node.label(), Some("Background sync"));
                assert_eq!(node.toggled(), Some(Toggled::True));
                assert_eq!(element.id(), initial.1);
            });
        });
        app.quit();
    }
}
