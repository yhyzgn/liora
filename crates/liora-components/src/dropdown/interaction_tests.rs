use crate::{Dropdown, DropdownButton, DropdownButtonItem, PortalLayer};
use gpui::{
    App, Context, KeyDownEvent, KeyUpEvent, Keystroke, MouseButton, MouseDownEvent, MouseMoveEvent,
    MouseUpEvent, Pixels, PlatformInput, Point, Render, TestAppContext, Window, div, point,
    prelude::*, px, size,
};
use liora_core::{is_popover_active, render_active_popover_in_window, set_active_popover};
use std::{cell::RefCell, rc::Rc};

const ID: &str = "dropdown-interaction-test";

#[derive(Clone, Copy)]
enum Menu {
    Dropdown,
    Button,
}

#[derive(Clone, Copy, Default)]
enum Availability {
    #[default]
    Enabled,
    FirstDisabled,
    AllDisabled,
    TriggerDisabled,
    Empty,
    Replacement,
    DisabledMiddle,
}

#[derive(Default)]
struct Commands(RefCell<Vec<usize>>, RefCell<Option<gpui::EntityId>>);

impl Commands {
    fn callback(
        self: &Rc<Self>,
        index: usize,
        replace: bool,
    ) -> impl Fn(&mut Window, &mut App) + 'static {
        let commands = self.clone();
        move |_, cx| {
            commands.0.borrow_mut().push(index);
            if replace {
                let replacement = cx.new(|_| gpui::EmptyView);
                *commands.1.borrow_mut() = Some(replacement.entity_id());
                set_active_popover(ID.into(), replacement.into(), cx);
            }
        }
    }
}

struct Surface {
    menu: Menu,
    availability: Availability,
    commands: Rc<Commands>,
    close_on_escape: bool,
    revision: usize,
}

impl Render for Surface {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        render_active_popover_in_window(window, cx);
        let menu = match self.menu {
            Menu::Dropdown => {
                let trigger = div()
                    .id("plain-trigger")
                    .debug_selector(|| "dropdown-interaction-trigger".to_string())
                    .size(px(80.))
                    .child("Actions");
                let mut dropdown = Dropdown::new(trigger)
                    .id(ID)
                    .close_on_escape(self.close_on_escape)
                    .close_on_click_outside(false);
                if !matches!(self.availability, Availability::Empty) {
                    for (index, label) in ["First", "Middle", "Last"].into_iter().enumerate() {
                        if index == 1 && matches!(self.availability, Availability::DisabledMiddle) {
                            dropdown = dropdown.disabled_item(label);
                            continue;
                        }
                        dropdown = dropdown.item(
                            label,
                            self.commands.callback(
                                index + self.revision * 100,
                                index == 0
                                    && matches!(self.availability, Availability::Replacement),
                            ),
                        );
                    }
                }
                dropdown.into_any_element()
            }
            Menu::Button => {
                let mut dropdown = DropdownButton::new("Actions")
                    .id(ID)
                    .close_on_escape(self.close_on_escape)
                    .close_on_click_outside(false)
                    .disabled(matches!(self.availability, Availability::TriggerDisabled));
                for (index, label) in ["First", "Middle", "Last"].into_iter().enumerate() {
                    let disabled = index == 1
                        || (index == 0 && matches!(self.availability, Availability::FirstDisabled))
                        || matches!(self.availability, Availability::AllDisabled);
                    dropdown = dropdown.menu_item(
                        DropdownButtonItem::new(
                            label,
                            self.commands.callback(
                                index + self.revision * 100,
                                index == 0
                                    && matches!(self.availability, Availability::Replacement),
                            ),
                        )
                        .disabled(disabled),
                    );
                }
                dropdown.into_any_element()
            }
        };
        div()
            .size_full()
            .relative()
            .child(div().absolute().top(px(24.)).left(px(24.)).child(menu))
            .child(PortalLayer)
    }
}

fn draw(window: &mut Window, cx: &mut App) {
    window.simulate_next_frame(cx);
    window.draw(cx).clear(cx);
}

fn active(cx: &App) -> bool {
    is_popover_active(&ID.into(), cx)
}

fn key(window: &mut Window, cx: &mut App, key: &str, down: bool, is_held: bool) {
    let keystroke = Keystroke::parse_with_os(key, cx.platform_info().os).unwrap();
    let event = if down {
        PlatformInput::KeyDown(KeyDownEvent {
            keystroke,
            is_held,
            prefer_character_input: false,
        })
    } else {
        PlatformInput::KeyUp(KeyUpEvent { keystroke })
    };
    window.dispatch_event(event, cx);
}

fn press(window: &mut Window, cx: &mut App, stroke: &str) {
    key(window, cx, stroke, true, false);
    key(window, cx, stroke, false, false);
}

fn pointer(window: &mut Window, cx: &mut App, position: Point<Pixels>, down: bool) {
    window.dispatch_event(
        PlatformInput::MouseMove(MouseMoveEvent {
            position,
            ..Default::default()
        }),
        cx,
    );
    let event = if down {
        PlatformInput::MouseDown(MouseDownEvent {
            position,
            button: MouseButton::Left,
            click_count: 1,
            ..Default::default()
        })
    } else {
        PlatformInput::MouseUp(MouseUpEvent {
            position,
            button: MouseButton::Left,
            click_count: 1,
            ..Default::default()
        })
    };
    window.dispatch_event(event, cx);
}

fn fixture(
    menu: Menu,
    availability: Availability,
    close_on_escape: bool,
) -> (TestAppContext, gpui::WindowHandle<Surface>, Rc<Commands>) {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let commands = Rc::new(Commands::default());
    let window = app.open_window(size(px(480.), px(360.)), |_, _| Surface {
        menu,
        availability,
        close_on_escape,
        commands: commands.clone(),
        revision: 0,
    });
    app.update_window(window.into(), |_, window, cx| draw(window, cx))
        .unwrap();
    match menu {
        Menu::Dropdown => {
            let trigger = {
                let mut visual = gpui::VisualTestContext::from_window(window.into(), &app);
                visual
                    .debug_bounds("dropdown-interaction-trigger")
                    .expect("painted production trigger")
                    .center()
            };
            app.update_window(window.into(), |_, window, cx| {
                pointer(window, cx, trigger, true);
                pointer(window, cx, trigger, false);
                draw(window, cx);
            })
            .unwrap();
        }
        Menu::Button => app
            .update_window(window.into(), |_, window, cx| {
                window.focus_next(cx);
                draw(window, cx);
                press(window, cx, "enter");
                draw(window, cx);
            })
            .unwrap(),
    }
    app.update_window(window.into(), |_, window, cx| {
        window.simulate_next_frame(cx);
    })
    .unwrap();
    app.update_window(window.into(), |_, _, cx| {
        assert!(active(cx), "precondition: the production trigger must open by its supported activation; failure here is not a row RED");
    }).unwrap();
    (app, window, commands)
}

#[test]
fn disabled_pointer_preserves_enabled_row_focus() {
    for menu in [Menu::Dropdown, Menu::Button] {
        let (mut app, window, commands) = fixture(menu, Availability::DisabledMiddle, true);
        let mut visual = gpui::VisualTestContext::from_window(window.into(), &app);
        let disabled = visual
            .debug_bounds("dropdown-interaction-test-item-1")
            .unwrap()
            .center();
        app.update_window(window.into(), |_, window, cx| {
            let first = window
                .focused(cx)
                .expect("first enabled menu row focused on open");
            pointer(window, cx, disabled, true);
            pointer(window, cx, disabled, false);
            draw(window, cx);
            assert!(active(cx), "disabled pointer must keep menu open");
            assert!(commands.0.borrow().is_empty());
            assert_eq!(
                window.focused(cx),
                Some(first),
                "disabled pointer preserves first enabled focus"
            );
            press(window, cx, "enter");
            assert_eq!(*commands.0.borrow(), [0]);
            assert!(!active(cx));
        })
        .unwrap();
    }
}

#[test]
fn same_id_popover_replacements_survive_both_menu_activations() {
    for menu in [Menu::Dropdown, Menu::Button] {
        let (mut app, window, commands) = fixture(menu, Availability::Replacement, true);
        app.update_window(window.into(), |_, window, cx| {
            press(window, cx, "enter");
            assert_eq!(
                *commands.0.borrow(),
                [0],
                "replacement callback runs exactly once"
            );
            let expected = commands
                .1
                .borrow()
                .as_ref()
                .cloned()
                .expect("replacement view id");
            let entries = &cx.global::<liora_core::ActivePopover>().0;
            let active: Vec<_> = entries.iter().filter(|entry| entry.id == ID).collect();
            assert_eq!(
                active.len(),
                1,
                "exactly one same-ID replacement remains active"
            );
            assert_eq!(
                active[0].view.entity_id(),
                expected,
                "the callback's replacement survives selection"
            );
        })
        .unwrap();
    }
}

#[test]
fn menu_initial_focus_waits_for_the_next_frame_and_ignores_closed_popovers() {
    for close_before_next_frame in [false, true] {
        let mut app = TestAppContext::single();
        app.update(crate::init_liora);
        let commands = Rc::new(Commands::default());
        let window = app.open_window(size(px(480.), px(360.)), |_, _| Surface {
            menu: Menu::Button,
            availability: Availability::Enabled,
            close_on_escape: true,
            commands: commands.clone(),
            revision: 0,
        });
        app.update_window(window.into(), |_, window, cx| {
            draw(window, cx);
            window.focus_next(cx);
            draw(window, cx);
            let trigger = window.focused(cx).expect("keyboard-focusable trigger");
            press(window, cx, "enter");
            draw(window, cx);
            assert!(active(cx));
            assert_eq!(
                window.focused(cx),
                Some(trigger.clone()),
                "opening must not change focus during portal prepaint"
            );
            if close_before_next_frame {
                press(window, cx, "escape");
                assert!(!active(cx));
            }
            window.simulate_next_frame(cx);
            if close_before_next_frame {
                assert_eq!(
                    window.focused(cx),
                    Some(trigger),
                    "a closed menu must not receive deferred focus"
                );
            } else {
                assert_ne!(
                    window.focused(cx),
                    Some(trigger),
                    "the enabled first row receives focus after the frame"
                );
            }
        })
        .unwrap();
    }
}

#[test]
fn dropdown_tab_dismisses_and_preserves_host_focus() {
    struct TabSurface {
        menu: Menu,
        close_on_escape: bool,
        before: gpui::FocusHandle,
        after: gpui::FocusHandle,
        commands: Rc<Commands>,
    }

    impl Render for TabSurface {
        fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            render_active_popover_in_window(window, cx);
            let menu = match self.menu {
                Menu::Dropdown => Dropdown::new(div().size(px(80.)).child("Actions"))
                    .id(ID)
                    .close_on_escape(self.close_on_escape)
                    .item("First", self.commands.callback(0, false))
                    .into_any_element(),
                Menu::Button => DropdownButton::new("Actions")
                    .id(ID)
                    .split(false)
                    .close_on_escape(self.close_on_escape)
                    .item("First", self.commands.callback(0, false))
                    .into_any_element(),
            };
            div()
                .size_full()
                .relative()
                .on_key_down(|event, window, cx| {
                    if event.keystroke.key == "tab" {
                        if event.keystroke.modifiers.shift {
                            window.focus_prev(cx);
                        } else {
                            window.focus_next(cx);
                        }
                        window.prevent_default();
                        cx.stop_propagation();
                    }
                })
                .child(
                    div()
                        .id("host-before")
                        .track_focus(&self.before)
                        .tab_index(0)
                        .size(px(12.)),
                )
                .child(div().absolute().top(px(24.)).left(px(24.)).child(menu))
                .child(
                    div()
                        .id("host-after")
                        .track_focus(&self.after)
                        .tab_index(0)
                        .size(px(12.)),
                )
                .child(PortalLayer)
        }
    }

    for menu in [Menu::Dropdown, Menu::Button] {
        for close_on_escape in [true, false] {
            for reverse in [false, true] {
                let mut app = TestAppContext::single();
                app.update(crate::init_liora);
                let commands = Rc::new(Commands::default());
                let hosts = Rc::new(RefCell::new(None));
                let window = app.open_window(size(px(480.), px(360.)), |_, cx| {
                    let before = cx.focus_handle().tab_stop(true).tab_index(0);
                    let after = cx.focus_handle().tab_stop(true).tab_index(0);
                    *hosts.borrow_mut() = Some([before.clone(), after.clone()]);
                    TabSurface {
                        menu,
                        close_on_escape,
                        before,
                        after,
                        commands: commands.clone(),
                    }
                });
                let expected = hosts.borrow().as_ref().unwrap()[usize::from(reverse)].clone();
                let other = hosts.borrow().as_ref().unwrap()[usize::from(!reverse)].clone();
                app.update_window(window.into(), |_, window, cx| {
                    draw(window, cx);
                    let trigger = point(px(42.), px(38.));
                    pointer(window, cx, trigger, true);
                    pointer(window, cx, trigger, false);
                    draw(window, cx);
                    window.simulate_next_frame(cx);
                    assert!(active(cx));
                    if !close_on_escape {
                        press(window, cx, "escape");
                        assert!(active(cx), "Escape-disabled menu must remain open");
                    }
                    press(window, cx, if reverse { "shift-tab" } else { "tab" });
                    assert!(
                        !active(cx),
                        "Tab must dismiss the exact active menu immediately"
                    );
                    assert!(
                        commands.0.borrow().is_empty(),
                        "Tab must not activate a command"
                    );
                    assert_eq!(
                        window.focused(cx),
                        Some(expected.clone()),
                        "host traversal target"
                    );
                    draw(window, cx);
                    press(window, cx, "escape");
                    assert_eq!(
                        window.focused(cx),
                        Some(expected.clone()),
                        "close and Escape must not steal host focus"
                    );
                    press(window, cx, if reverse { "tab" } else { "shift-tab" });
                    assert_eq!(
                        window.focused(cx),
                        Some(other),
                        "ordinary host traversal continues after dismissal"
                    );
                    press(window, cx, if reverse { "shift-tab" } else { "tab" });
                    assert_eq!(
                        window.focused(cx),
                        Some(expected),
                        "reverse host traversal restores the same host control"
                    );
                    assert!(!active(cx));
                    assert!(commands.0.borrow().is_empty());
                })
                .unwrap();
            }
        }
    }
}

#[test]
fn dropdown_first_item_activates_on_complete_enter() {
    for menu in [Menu::Dropdown, Menu::Button] {
        let (mut app, window, commands) = fixture(menu, Availability::Enabled, true);
        app.update_window(window.into(), |_, window, cx| {
            key(window, cx, "enter", true, false);
            assert!(commands.0.borrow().is_empty());
            draw(window, cx);
            key(window, cx, "enter", false, false);
            assert_eq!(*commands.0.borrow(), [0]);
            assert!(!active(cx));
            key(window, cx, "enter", false, false);
            assert_eq!(*commands.0.borrow(), [0]);
        })
        .unwrap();
    }
}

#[test]
fn rebuilt_open_menus_activate_the_current_callback_for_each_control() {
    for menu in [Menu::Dropdown, Menu::Button] {
        let (mut app, window, commands) = fixture(menu, Availability::Enabled, true);
        let first = {
            let mut visual = gpui::VisualTestContext::from_window(window.into(), &app);
            visual
                .debug_bounds("dropdown-interaction-test-item-0")
                .expect("painted first menu row")
                .center()
        };
        window
            .update(&mut app, |surface, _, cx| {
                surface.revision = 1;
                cx.notify();
            })
            .unwrap();
        app.update_window(window.into(), |_, window, cx| draw(window, cx))
            .unwrap();
        let mut visual = gpui::VisualTestContext::from_window(window.into(), &app);
        assert_eq!(
            visual
                .debug_bounds("dropdown-interaction-test-item-0")
                .unwrap()
                .center(),
            first
        );
        app.update_window(window.into(), |_, window, cx| {
            pointer(window, cx, first, true);
            pointer(window, cx, first, false);
            assert_eq!(
                *commands.0.borrow(),
                [100],
                "callback B runs once and stale callback A never runs"
            );
            assert!(!active(cx));
        })
        .unwrap();
    }
}

#[test]
fn dropdown_navigation_skips_disabled_wraps_and_survives_redraw() {
    // Distinct expected commands make Down/Up, wrap and Home/End observable.
    for (keys, command) in [
        (&["down"][..], 2),
        (&["up"][..], 2),
        (&["end", "down"][..], 0),
        (&["end", "home"][..], 0),
        (&["end", "up"][..], 0),
        (&["end"][..], 2),
    ] {
        let (mut app, window, commands) = fixture(Menu::Button, Availability::Enabled, false);
        app.update_window(window.into(), |_, window, cx| {
            press(window, cx, "escape");
            assert!(active(cx), "manual Escape policy must be preserved");
            for stroke in keys {
                press(window, cx, stroke);
                draw(window, cx);
            }
            key(window, cx, "space", true, false);
            draw(window, cx);
            key(window, cx, "space", true, true);
            assert!(commands.0.borrow().is_empty());
            key(window, cx, "space", false, false);
            assert_eq!(*commands.0.borrow(), [command]);
            assert!(!active(cx));
        })
        .unwrap();
    }
}

#[test]
fn dropdown_empty_and_all_disabled_keep_escape_path_without_commands() {
    for (menu, availability) in [
        (Menu::Dropdown, Availability::Empty),
        (Menu::Button, Availability::AllDisabled),
    ] {
        let (mut app, window, commands) = fixture(menu, availability, true);
        app.update_window(window.into(), |_, window, cx| {
            for stroke in ["up", "end", "enter", "space"] {
                press(window, cx, stroke);
            }
            assert!(active(cx));
            assert!(commands.0.borrow().is_empty());
            press(window, cx, "escape");
            assert!(!active(cx));
        })
        .unwrap();
    }
}

#[test]
fn dropdown_button_disabling_open_menu_or_armed_row_blocks_old_command() {
    for availability in [Availability::FirstDisabled, Availability::TriggerDisabled] {
        let (mut app, window, commands) = fixture(Menu::Button, Availability::Enabled, true);
        app.update_window(window.into(), |_, window, cx| {
            key(window, cx, "enter", true, false);
        })
        .unwrap();
        window
            .update(&mut app, |surface, _, cx| {
                surface.availability = availability;
                cx.notify();
            })
            .unwrap();
        app.update_window(window.into(), |_, window, cx| {
            draw(window, cx);
        })
        .unwrap();
        app.update_window(window.into(), |_, window, cx| {
            key(window, cx, "enter", false, false);
            assert!(commands.0.borrow().is_empty());
            if matches!(availability, Availability::TriggerDisabled) {
                assert!(
                    !active(cx),
                    "the disabled trigger cannot leave an actionable popup"
                );
            } else {
                assert!(active(cx));
                press(window, cx, "enter");
                assert_eq!(
                    *commands.0.borrow(),
                    [2],
                    "focus must recover to the enabled row"
                );
                assert!(!active(cx));
            }
        })
        .unwrap();
    }
}

#[test]
fn dropdown_button_pointer_requires_release_on_pressed_row() {
    let (mut app, window, commands) = fixture(Menu::Button, Availability::Enabled, true);
    let mut visual = gpui::VisualTestContext::from_window(window.into(), &app);
    let first = visual
        .debug_bounds("dropdown-interaction-test-item-0")
        .expect("painted first row")
        .center();
    let disabled = visual
        .debug_bounds("dropdown-interaction-test-item-1")
        .expect("painted disabled row")
        .center();
    let last = visual
        .debug_bounds("dropdown-interaction-test-item-2")
        .expect("painted last enabled row")
        .center();
    app.update_window(window.into(), |_, window, cx| {
        let outside = point(px(410.), px(300.));
        for target in [disabled, outside] {
            pointer(window, cx, first, true);
            assert!(
                commands.0.borrow().is_empty(),
                "mouse down is not activation"
            );
            draw(window, cx);
            pointer(window, cx, target, false);
            assert!(commands.0.borrow().is_empty());
            assert!(active(cx));
        }
        pointer(window, cx, disabled, true);
        pointer(window, cx, disabled, false);
        assert!(
            commands.0.borrow().is_empty(),
            "disabled row owns a real, rejected command"
        );
        pointer(window, cx, first, true);
        draw(window, cx);
        pointer(window, cx, last, false);
        assert!(
            commands.0.borrow().is_empty(),
            "release on another enabled row is not a click"
        );
        assert!(active(cx));
        pointer(window, cx, first, true);
        pointer(window, cx, first, false);
        assert_eq!(
            *commands.0.borrow(),
            [0],
            "same-row release is the positive control"
        );
        assert!(!active(cx));
    })
    .unwrap();
}
