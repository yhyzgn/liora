use super::*;
use crate::{Button, DropdownButton, Input, PortalLayer};
use gpui::{Entity, Focusable, TestAppContext, VisualTestContext, WindowHandle};
use std::cell::RefCell;

struct Surface {
    outside: Entity<Input>,
    inside: Entity<Input>,
    trigger_clicks: Rc<Cell<usize>>,
    content_clicks: Rc<Cell<usize>>,
    close_on_escape: bool,
    close_on_click_outside: bool,
    move_focus: bool,
    split: bool,
    dropdown: bool,
    split_has_action: bool,
    replace_on_trigger: bool,
    replacement_entity: Rc<Cell<Option<gpui::EntityId>>>,
    confirm_focus: Rc<RefCell<Option<gpui::FocusHandle>>>,
    confirm_had_focus: Rc<Cell<bool>>,
    editable_trigger: bool,
    show_popup: bool,
    popup_offset: Pixels,
}

impl Render for Surface {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        liora_core::render_active_popover_in_window(window, cx);
        let trigger_clicks = self.trigger_clicks.clone();
        let outside = self.outside.clone();
        let move_focus = self.move_focus;
        let replace_on_trigger = self.replace_on_trigger;
        let replacement_entity = self.replacement_entity.clone();
        let trigger = Button::new("Open")
            .id("keyboard-trigger")
            .on_click(move |_, window, cx| {
                trigger_clicks.set(trigger_clicks.get() + 1);
                if move_focus {
                    outside.focus_handle(cx).focus(window, cx);
                }
                if replace_on_trigger {
                    let content: Arc<dyn Fn(&mut Window, &mut Context<PopoverView>) -> AnyElement> =
                        Arc::new(|_, _| {
                            div()
                                .debug_selector(|| "replacement-content".into())
                                .into_any_element()
                        });
                    let replacement = cx.new(|_| {
                        PopoverView::new(
                            content,
                            Bounds {
                                origin: point(px(10.0), px(10.0)),
                                size: gpui::size(px(80.0), px(24.0)),
                            },
                            Placement::Bottom,
                            px(8.0),
                            true,
                            true,
                            "keyboard-popover".into(),
                            None,
                            |_, cx| clear_popover(&"keyboard-popover".into(), cx),
                        )
                    });
                    replacement_entity.set(Some(replacement.entity_id()));
                    set_active_popover("keyboard-popover".into(), replacement.into(), cx);
                }
            });
        let content_clicks = self.content_clicks.clone();
        let inside = self.inside.clone();
        let close_on_click_outside = self.close_on_click_outside;
        let confirm_focus = self.confirm_focus.clone();
        let confirm_had_focus = self.confirm_had_focus.clone();
        let popup = if self.dropdown {
            let split = DropdownButton::new("Split")
                .id("keyboard-popover")
                .split(self.split);
            let split = if self.split_has_action {
                let clicks = self.trigger_clicks.clone();
                let outside = self.outside.clone();
                split.on_click(move |window, cx| {
                    clicks.set(clicks.get() + 1);
                    outside.focus_handle(cx).focus(window, cx);
                })
            } else {
                split
            };
            split.item("Item", |_, _| {}).into_any_element()
        } else {
            let trigger = if self.editable_trigger {
                self.outside.clone().into_any_element()
            } else {
                trigger.into_any_element()
            };
            Popover::new(trigger)
                .id("keyboard-popover")
                .close_on_escape(self.close_on_escape)
                .close_on_click_outside(close_on_click_outside)
                .content(move |_, _| {
                    let clicks = content_clicks.clone();
                    let confirm_focus = confirm_focus.clone();
                    let confirm_had_focus = confirm_had_focus.clone();
                    div()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child(
                            Button::new("Content action")
                                .id("content-action")
                                .on_click(move |_, _, _| clicks.set(clicks.get() + 1)),
                        )
                        .child(
                            div()
                                .debug_selector(|| "inside-input".into())
                                .child(inside.clone()),
                        )
                        .child(Button::new("Confirm").id("docs-confirm").on_click(
                            move |_, window, cx| {
                                confirm_had_focus.set(
                                    confirm_focus
                                        .borrow()
                                        .as_ref()
                                        .is_some_and(|confirm| confirm.is_focused(window)),
                                );
                                clear_popover(&"keyboard-popover".into(), cx);
                            },
                        ))
                        .child(
                            Button::new("Close")
                                .id("content-close")
                                .on_click(|_, _, cx| clear_popover(&"keyboard-popover".into(), cx)),
                        )
                })
                .into_any_element()
        };
        div()
            .relative()
            .size_full()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .when(self.show_popup, |s| {
                        s.child(
                            div().flex().mt(self.popup_offset).child(
                                div().debug_selector(|| "fixture-popup".into()).child(popup),
                            ),
                        )
                    })
                    .when(!self.editable_trigger, |s| {
                        s.child(
                            div()
                                .debug_selector(|| "outside-input-host".into())
                                .child(self.outside.clone()),
                        )
                    }),
            )
            .child(PortalLayer)
    }
}

struct Harness {
    app: TestAppContext,
    handle: WindowHandle<Surface>,
    window: VisualTestContext,
    trigger_clicks: Rc<Cell<usize>>,
    content_clicks: Rc<Cell<usize>>,
}

impl Harness {
    fn new(close_on_escape: bool, move_focus: bool, split: bool, editable_trigger: bool) -> Self {
        let mut app = TestAppContext::single();
        app.update(crate::init_liora);
        let trigger_clicks = Rc::new(Cell::new(0));
        let content_clicks = Rc::new(Cell::new(0));
        let handle = app.open_window(gpui::size(px(600.0), px(400.0)), {
            let trigger_clicks = trigger_clicks.clone();
            let content_clicks = content_clicks.clone();
            move |_, cx| Surface {
                outside: cx.new(|cx| Input::new("", cx).id("outside-input")),
                inside: cx.new(|cx| Input::new("", cx).id("inside-input")),
                trigger_clicks,
                content_clicks,
                close_on_escape,
                close_on_click_outside: true,
                split_has_action: true,
                replace_on_trigger: false,
                replacement_entity: Rc::new(Cell::new(None)),
                confirm_focus: Rc::new(RefCell::new(None)),
                confirm_had_focus: Rc::new(Cell::new(false)),
                move_focus,
                split,
                dropdown: split,
                editable_trigger,
                show_popup: true,
                popup_offset: px(0.0),
            }
        });
        let window = VisualTestContext::from_window(handle.into(), &app);
        let mut harness = Self {
            app,
            handle,
            window,
            trigger_clicks,
            content_clicks,
        };
        harness.draw();
        harness
    }

    fn draw(&mut self) {
        self.window.update(|window, cx| {
            window.refresh();
            window.draw(cx).clear(cx);
        });
    }

    fn press(&mut self, key: &str) {
        self.window.simulate_keystrokes(key);
        self.window.update(|window, cx| {
            let keystroke = gpui::Keystroke::parse_with_os(key, cx.platform_info().os).unwrap();
            window.dispatch_event(
                gpui::PlatformInput::KeyUp(gpui::KeyUpEvent { keystroke }),
                cx,
            );
        });
        self.draw();
    }

    fn click(&mut self, id: &'static str) {
        let bounds = self
            .window
            .debug_bounds(id)
            .expect("visible native control");
        self.window
            .simulate_click(bounds.center(), gpui::Modifiers::none());
        self.draw();
    }

    fn focus_first(&mut self) -> gpui::FocusHandle {
        let focus = self.window.update(|window, cx| {
            window.focus_next(cx);
            window
                .focused(cx)
                .expect("first real control must be focusable")
        });
        self.draw();
        focus
    }

    fn is_open(&mut self) -> bool {
        self.window
            .update(|_, cx| is_popover_active(&"keyboard-popover".into(), cx))
    }

    fn assert_focus(&mut self, expected: &gpui::FocusHandle) {
        self.window.update(|window, _| {
            assert!(
                expected.is_focused(window),
                "original native focus owner was not restored"
            )
        });
    }
}

#[test]
fn open_popover_follows_anchor_movement_and_preserves_escape_focus() {
    let mut h = Harness::new(true, false, false, false);
    let trigger = h.focus_first();
    h.press("enter");
    let before = h.window.debug_bounds("inside-input").unwrap();
    h.handle
        .update(&mut h.app, |view, _, cx| {
            view.popup_offset = px(60.0);
            cx.notify();
        })
        .unwrap();
    h.draw();
    h.draw();
    let after = h.window.debug_bounds("inside-input").unwrap();
    assert_eq!(after.origin.y - before.origin.y, px(60.0));
    h.assert_focus(&trigger);
    h.click("inside-input");
    h.press("escape");
    assert!(!h.is_open());
    h.assert_focus(&trigger);
    h.app.quit();
}

#[test]
fn unmounting_anchor_clears_retained_popover_and_its_backdrop() {
    let mut h = Harness::new(false, false, false, false);
    h.handle
        .update(&mut h.app, |view, _, _| view.close_on_click_outside = false)
        .unwrap();
    h.draw();
    h.click("keyboard-trigger");
    h.click("inside-input");
    let retained_view = h
        .window
        .update(|_, cx| cx.global::<liora_core::ActivePopover>().0[0].view.clone());
    h.handle
        .update(&mut h.app, |view, _, cx| {
            view.show_popup = false;
            cx.notify();
        })
        .unwrap();
    h.draw();
    h.draw();
    assert!(!h.is_open(), "unmounted anchor left its popover registered");
    assert!(h.window.debug_bounds("inside-input").is_none());
    h.click("outside-input-host");
    let outside = h
        .handle
        .read_with(&h.app, |view, cx| view.outside.focus_handle(cx))
        .unwrap();
    h.assert_focus(&outside);
    drop(retained_view);
    h.app.quit();
}

#[test]
fn keyboard_button_opens_once_and_escape_reaches_trigger_and_content() {
    let mut h = Harness::new(true, false, false, false);
    let trigger = h.focus_first();
    h.press("ctrl-enter");
    assert!(!h.is_open());
    assert_eq!(h.trigger_clicks.get(), 0);
    for key in ["enter", "space"] {
        h.press(key);
        assert!(
            h.is_open(),
            "{key} on the actual Button did not open Popover"
        );
        h.assert_focus(&trigger);
        h.press("escape");
        assert!(!h.is_open(), "Escape from trigger did not close Popover");
        h.assert_focus(&trigger);
    }
    assert_eq!(h.trigger_clicks.get(), 2);
    h.press("enter");
    h.click("content-action");
    assert_eq!(h.content_clicks.get(), 1);
    h.press("escape");
    assert!(
        !h.is_open(),
        "Escape from interactive content did not close Popover"
    );
    h.assert_focus(&trigger);
    h.press("tab");
    let outside = h
        .handle
        .read_with(&h.app, |view, cx| view.outside.focus_handle(cx))
        .unwrap();
    h.assert_focus(&outside);
    h.app.quit();
}

#[test]
fn escape_policy_applies_from_trigger_button_and_input_content() {
    let mut h = Harness::new(false, false, false, false);
    let trigger = h.focus_first();
    h.press("enter");
    assert!(h.is_open());
    h.press("escape");
    assert!(h.is_open());
    h.assert_focus(&trigger);
    h.click("content-action");
    h.press("escape");
    assert!(h.is_open());
    h.click("inside-input");
    h.press("escape");
    assert!(h.is_open());
    h.click("content-close");
    assert!(!h.is_open());
    h.assert_focus(&trigger);
    assert_eq!(h.trigger_clicks.get(), 1);
    assert_eq!(h.content_clicks.get(), 1);
    h.app.quit();
}

#[test]
fn pointer_and_manual_clear_restore_focus_without_stealing_unrelated_focus() {
    let mut h = Harness::new(true, false, false, false);
    h.click("keyboard-trigger");
    assert!(h.is_open());
    let trigger = h.window.update(|window, cx| window.focused(cx).unwrap());
    h.click("content-close");
    assert!(!h.is_open());
    h.assert_focus(&trigger);
    h.click("keyboard-trigger");
    h.click("inside-input");
    h.press("escape");
    assert!(!h.is_open());
    h.assert_focus(&trigger);
    h.click("keyboard-trigger");
    let outside = h
        .handle
        .read_with(&h.app, |view, cx| view.outside.focus_handle(cx))
        .unwrap();
    h.window.update(|window, cx| {
        outside.focus(window, cx);
        clear_popover(&"keyboard-popover".into(), cx);
    });
    h.draw();
    h.assert_focus(&outside);
    h.click("keyboard-trigger");
    h.window
        .simulate_click(point(px(580.0), px(380.0)), gpui::Modifiers::none());
    h.draw();
    assert!(!h.is_open(), "outside pointer click must still dismiss");
    assert_eq!(h.trigger_clicks.get(), 4);
    h.app.quit();
}

#[test]
fn keyboard_activation_keeps_origin_when_user_callback_moves_focus() {
    let mut h = Harness::new(true, true, false, false);
    let trigger = h.focus_first();
    h.press("enter");
    assert!(
        h.is_open(),
        "deferred activation lost the actual Button dispatch path"
    );
    assert_eq!(h.trigger_clicks.get(), 1);
    let outside = h
        .handle
        .read_with(&h.app, |view, cx| view.outside.focus_handle(cx))
        .unwrap();
    h.assert_focus(&outside);
    h.click("content-close");
    assert!(!h.is_open());
    h.assert_focus(&trigger);
    h.app.quit();
}

#[test]
fn split_primary_never_opens_and_arrow_opens_with_or_without_callback() {
    for has_action in [true, false] {
        let mut h = Harness::new(true, false, true, false);
        h.handle
            .update(&mut h.app, |view, _, cx| {
                view.split_has_action = has_action;
                cx.notify();
            })
            .unwrap();
        h.draw();
        let primary = h.focus_first();
        for key in ["enter", "space"] {
            h.click("keyboard-popover-main-action");
            assert!(
                !h.is_open(),
                "split primary pointer click opened menu; callback={has_action}"
            );
            h.window.update(|window, cx| primary.focus(window, cx));
            h.draw();
            h.press(key);
            assert!(
                !h.is_open(),
                "split primary {key} opened menu; callback={has_action}"
            );
            h.click("keyboard-popover-toggle");
            assert!(h.is_open(), "split arrow pointer click did not open menu");
            h.press("escape");
            assert!(!h.is_open());
            h.press(key);
            assert!(
                h.is_open(),
                "split arrow {key} did not open menu; callback={has_action}"
            );
            h.press("escape");
            assert!(!h.is_open());
        }
        assert_eq!(h.trigger_clicks.get(), if has_action { 4 } else { 0 });
        h.app.quit();
    }
}

#[test]
fn editable_trigger_does_not_acquire_button_keyboard_semantics() {
    let mut h = Harness::new(true, false, false, true);
    h.focus_first();
    h.press("space");
    assert_eq!(
        h.handle
            .read_with(&h.app, |view, cx| view.outside.read(cx).value())
            .unwrap()
            .as_ref(),
        " ",
        "Space must reach the editable trigger"
    );
    h.press("x");
    assert_eq!(
        h.handle
            .read_with(&h.app, |view, cx| view.outside.read(cx).value())
            .unwrap()
            .as_ref(),
        " x",
        "visible text must reach the editable trigger"
    );
    h.press("enter");
    assert!(
        !h.is_open(),
        "typing keys in editable trigger must not activate Popover"
    );
    h.app.quit();
}

#[test]
fn synchronous_activation_preserves_same_id_callback_replacement() {
    let mut h = Harness::new(true, false, false, false);
    h.handle
        .update(&mut h.app, |view, _, cx| {
            view.replace_on_trigger = true;
            cx.notify();
        })
        .unwrap();
    h.draw();
    h.focus_first();
    h.press("enter");
    assert_eq!(h.trigger_clicks.get(), 1);
    let expected = h
        .handle
        .read_with(&h.app, |view, _| view.replacement_entity.get().unwrap())
        .unwrap();
    h.window.update(|_, cx| {
        let active = cx
            .global::<liora_core::ActivePopover>()
            .0
            .iter()
            .find(|entry| entry.id == "keyboard-popover")
            .unwrap();
        assert_eq!(
            active.view.entity_id(),
            expected,
            "activation dispatched after the user callback replaced its Popover"
        );
    });
    h.app.quit();
}

#[test]
fn nonsplit_dropdown_opens_from_whole_button_by_keyboard_and_pointer() {
    let mut h = Harness::new(true, false, true, false);
    h.handle
        .update(&mut h.app, |view, _, cx| {
            view.split = false;
            cx.notify();
        })
        .unwrap();
    h.draw();
    h.focus_first();
    for key in ["enter", "space"] {
        h.press(key);
        assert!(h.is_open(), "non-split {key} did not open the menu");
        h.press("escape");
        assert!(!h.is_open());
    }
    h.click("fixture-popup");
    assert!(h.is_open(), "non-split pointer click did not open the menu");
    h.app.quit();
}

#[test]
fn docs_confirm_pointer_close_restores_focus_with_both_dismissals_disabled() {
    let mut h = Harness::new(false, false, false, false);
    h.handle
        .update(&mut h.app, |view, _, cx| {
            view.close_on_click_outside = false;
            cx.notify();
        })
        .unwrap();
    h.draw();
    let trigger = h.focus_first();
    h.press("enter");
    h.press("escape");
    assert!(h.is_open(), "Escape disabled by manual-close policy");
    h.window
        .simulate_click(point(px(580.0), px(380.0)), gpui::Modifiers::none());
    h.draw();
    assert!(
        h.is_open(),
        "outside dismissal disabled by manual-close policy"
    );
    h.assert_focus(&trigger);
    let bounds = h
        .window
        .debug_bounds("docs-confirm")
        .expect("Docs-style Confirm rendered");
    h.window.simulate_mouse_down(
        bounds.center(),
        gpui::MouseButton::Left,
        gpui::Modifiers::none(),
    );
    let confirm = h.window.update(|window, cx| {
        window
            .focused(cx)
            .expect("Confirm received native pointer focus")
    });
    assert_ne!(
        confirm, trigger,
        "pointer press must focus Confirm before its clear callback"
    );
    h.handle
        .update(&mut h.app, |view, _, _| {
            *view.confirm_focus.borrow_mut() = Some(confirm)
        })
        .unwrap();
    h.window.simulate_mouse_up(
        bounds.center(),
        gpui::MouseButton::Left,
        gpui::Modifiers::none(),
    );
    h.draw();
    assert!(
        !h.is_open(),
        "Confirm must call the public clear_popover path"
    );
    assert!(
        h.handle
            .read_with(&h.app, |view, _| view.confirm_had_focus.get())
            .unwrap(),
        "Confirm must own native focus before clearing"
    );
    h.assert_focus(&trigger);
    h.app.quit();
}

#[test]
fn disabled_outside_dismissal_blank_press_and_release_preserve_external_focus() {
    let mut h = Harness::new(true, false, false, false);
    h.handle
        .update(&mut h.app, |view, _, cx| {
            view.close_on_click_outside = false;
            cx.notify();
        })
        .unwrap();
    h.draw();
    h.focus_first();
    h.press("enter");
    let outside = h
        .handle
        .read_with(&h.app, |view, cx| view.outside.focus_handle(cx))
        .unwrap();
    h.window.update(|window, cx| outside.focus(window, cx));
    h.draw();
    let blank = point(px(580.0), px(380.0));
    h.window
        .simulate_mouse_down(blank, gpui::MouseButton::Left, gpui::Modifiers::none());
    h.assert_focus(&outside);
    h.window
        .simulate_mouse_up(blank, gpui::MouseButton::Left, gpui::Modifiers::none());
    h.draw();
    assert!(h.is_open(), "outside=false must leave the Popover open");
    h.assert_focus(&outside);
    h.window
        .update(|_, cx| clear_popover(&"keyboard-popover".into(), cx));
    h.draw();
    assert!(!h.is_open());
    h.assert_focus(&outside);
    h.app.quit();
}

#[test]
fn same_id_replacement_does_not_restore_old_trigger_from_focused_content() {
    let mut h = Harness::new(true, false, false, false);
    let trigger = h.focus_first();
    h.press("enter");
    h.click("inside-input");
    let old_content = h
        .handle
        .read_with(&h.app, |view, cx| view.inside.focus_handle(cx))
        .unwrap();
    h.assert_focus(&old_content);
    let (replacement_input, replacement_focus) = h.window.update(|_, cx| {
        let input = cx.new(|cx| Input::new("replacement", cx).id("replacement-input"));
        let focus = input.read(cx).focus_handle(cx);
        (input, focus)
    });
    let replacement_id = h.window.update(|_, cx| {
        let content: Arc<dyn Fn(&mut Window, &mut Context<PopoverView>) -> AnyElement> =
            Arc::new(move |_, _| {
                div()
                    .debug_selector(|| "replacement-input-host".into())
                    .child(replacement_input.clone())
                    .into_any_element()
            });
        let replacement = cx.new(|_| {
            PopoverView::new(
                content,
                Bounds {
                    origin: point(px(10.0), px(10.0)),
                    size: gpui::size(px(80.0), px(24.0)),
                },
                Placement::Bottom,
                px(8.0),
                true,
                true,
                "keyboard-popover".into(),
                None,
                |_, cx| clear_popover(&"keyboard-popover".into(), cx),
            )
        });
        let id = replacement.entity_id();
        set_active_popover("keyboard-popover".into(), replacement.into(), cx);
        id
    });
    h.assert_focus(&old_content);
    h.window.update(|window, _| {
        assert!(
            !trigger.is_focused(window),
            "replacement must not restore the retired trigger"
        )
    });
    h.draw();
    h.click("replacement-input-host");
    h.assert_focus(&replacement_focus);
    h.window.update(|_, cx| {
        let active = cx
            .global::<liora_core::ActivePopover>()
            .0
            .iter()
            .find(|entry| entry.id == "keyboard-popover")
            .unwrap();
        assert_eq!(active.view.entity_id(), replacement_id);
    });
    h.handle
        .update(&mut h.app, |view, _, cx| {
            view.show_popup = false;
            cx.notify();
        })
        .unwrap();
    h.draw();
    h.draw();
    assert!(
        h.is_open(),
        "retired anchor cleared its same-id replacement"
    );
    assert!(h.window.debug_bounds("replacement-input-host").is_some());
    h.assert_focus(&replacement_focus);
    h.app.quit();
}

#[gpui::test]
fn shared_popover_mutations_repaint_without_extra_input(cx: &mut TestAppContext) {
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
            liora_core::render_active_popover_in_window(window, cx);
            div().size_full().child(PortalLayer)
        }
    }
    cx.update(crate::init_liora);
    let renders = Rc::new(Cell::new(0));
    let root = cx.add_window(|_, _| Root(renders.clone()));
    let mut window = VisualTestContext::from_window(root.into(), cx);
    window.update(|window, cx| window.draw(cx).clear(cx));
    let notifications = Rc::new(Cell::new(0));
    let observed = notifications.clone();
    let _subscription = cx.update(|cx| {
        cx.observe_global::<liora_core::ActivePopover>(move |_| {
            observed.set(observed.get() + 1);
        })
    });
    let first = cx.new(|_| Probe("mutation-first"));
    let replacement = cx.new(|_| Probe("mutation-replacement"));
    let second = cx.new(|_| Probe("mutation-second"));
    let id: SharedString = "mutation-popover".into();
    cx.update(|cx| set_active_popover(id.clone(), first.into(), cx));
    cx.run_until_parked();
    // Observe the naturally painted frame: no draw, refresh, or input follows mutations.
    assert!(window.debug_bounds("mutation-first").is_some());
    let before_noop = (renders.get(), notifications.get());
    cx.update(|cx| clear_popover(&"missing-popover".into(), cx));
    cx.run_until_parked();
    assert_eq!(
        (renders.get(), notifications.get()),
        before_noop,
        "missing-ID clear dirtied or notified"
    );
    cx.update(|cx| set_active_popover(id.clone(), replacement.into(), cx));
    cx.run_until_parked();
    assert!(window.debug_bounds("mutation-first").is_none());
    assert!(window.debug_bounds("mutation-replacement").is_some());
    cx.update(|cx| clear_popover(&id, cx));
    cx.run_until_parked();
    assert!(window.debug_bounds("mutation-replacement").is_none());
    cx.update(|cx| set_active_popover(id.clone(), second.into(), cx));
    cx.run_until_parked();
    assert!(window.debug_bounds("mutation-second").is_some());
    cx.update(liora_core::clear_active_popover);
    cx.run_until_parked();
    assert!(window.debug_bounds("mutation-second").is_none());
    let before_noop = (renders.get(), notifications.get());
    cx.update(|cx| {
        clear_popover(&id, cx);
        liora_core::clear_active_popover(cx);
    });
    cx.run_until_parked();
    assert_eq!(
        (renders.get(), notifications.get()),
        before_noop,
        "empty clears dirtied or notified"
    );
}
