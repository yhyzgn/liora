use super::*;
use gpui::{AppContext, Context, FocusHandle, Focusable, IntoElement, Render, Window, div};
use std::{cell::Cell, cell::RefCell, rc::Rc};

struct RateKeyboardSurface {
    rates: Vec<gpui::Entity<Rate>>,
    before: FocusHandle,
    after: FocusHandle,
    parent_keys: Rc<Cell<usize>>,
}

impl Render for RateKeyboardSurface {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let parent_keys = self.parent_keys.clone();
        div()
            .id("rate-test-parent")
            .size_full()
            .flex()
            .flex_col()
            .on_key_down(move |event, window, cx| {
                parent_keys.set(parent_keys.get() + 1);
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
                }
            })
            .child(
                div()
                    .id("rate-test-before")
                    .h(gpui::px(24.0))
                    .w(gpui::px(120.0))
                    .track_focus(&self.before)
                    .tab_index(0)
                    .child("Before"),
            )
            .children(self.rates.iter().cloned())
            .child(
                div()
                    .id("rate-test-after")
                    .h(gpui::px(24.0))
                    .w(gpui::px(120.0))
                    .track_focus(&self.after)
                    .tab_index(0)
                    .child("After"),
            )
    }
}

fn mount(
    app: &mut gpui::TestAppContext,
    rates: Vec<gpui::Entity<Rate>>,
    parent_keys: Rc<Cell<usize>>,
) -> (gpui::VisualTestContext, FocusHandle, FocusHandle) {
    let (before, after) = app.update(|cx| {
        (
            cx.focus_handle().tab_stop(true).tab_index(0),
            cx.focus_handle().tab_stop(true).tab_index(0),
        )
    });
    let initial_focus = before.clone();
    let root = app.add_window({
        let before = before.clone();
        let after = after.clone();
        move |_, _| RateKeyboardSurface {
            rates,
            before,
            after,
            parent_keys,
        }
    });
    let mut window = gpui::VisualTestContext::from_window(root.into(), app);
    window.update(|window, cx| {
        window.draw(cx).clear(cx);
        window.focus(&initial_focus, cx);
    });
    (window, before, after)
}

fn make_rate(
    app: &mut gpui::TestAppContext,
    value: f32,
    max: usize,
    disabled: bool,
    changes: Rc<RefCell<Vec<f32>>>,
) -> gpui::Entity<Rate> {
    app.new(|cx| {
        Rate::new(value, cx)
            .max(max)
            .disabled(disabled)
            .on_change(move |value, _, _| changes.borrow_mut().push(value))
    })
}

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

fn focused(window: &mut gpui::VisualTestContext) -> FocusHandle {
    window.update(|window, cx| {
        window
            .focused(cx)
            .expect("a rendered control must hold focus")
    })
}

fn rate_focus(app: &gpui::TestAppContext, rate: &gpui::Entity<Rate>) -> FocusHandle {
    app.update(|cx| rate.read(cx).focus_handle(cx))
}

fn value(app: &gpui::TestAppContext, rate: &gpui::Entity<Rate>) -> f32 {
    app.update(|cx| rate.read(cx).value)
}

#[test]
fn enabled_rate_is_one_tab_stop_and_arrows_preview_until_enter_or_space_commits() {
    let mut app = gpui::TestAppContext::single();
    app.update(crate::init_liora);
    let changes = Rc::new(RefCell::new(Vec::new()));
    let rate = make_rate(&mut app, 2.0, 5, false, changes.clone());
    let (mut window, _, after) = mount(&mut app, vec![rate.clone()], Rc::new(Cell::new(0)));
    let rate_focus = rate_focus(&app, &rate);

    press(&mut window, "tab");
    assert_eq!(
        focused(&mut window),
        rate_focus,
        "Tab enters the Rate between neighboring stops"
    );
    press(&mut window, "tab");
    assert_eq!(
        focused(&mut window),
        after,
        "Rate contributes exactly one Tab stop"
    );
    press(&mut window, "shift-tab");
    assert_eq!(focused(&mut window), rate_focus);

    press(&mut window, "right");
    press(&mut window, "up");
    assert_eq!(value(&app, &rate), 2.0, "arrows preview without committing");
    assert!(changes.borrow().is_empty());
    press(&mut window, "enter");
    assert_eq!(value(&app, &rate), 4.0);
    assert_eq!(changes.borrow().as_slice(), &[4.0], "Enter commits once");

    press(&mut window, "home");
    press(&mut window, "enter");
    assert_eq!(value(&app, &rate), 1.0);
    assert_eq!(changes.borrow().as_slice(), &[4.0, 1.0]);

    press(&mut window, "end");
    press(&mut window, "space");
    assert_eq!(value(&app, &rate), 5.0);
    assert_eq!(changes.borrow().as_slice(), &[4.0, 1.0, 5.0]);

    press(&mut window, "right");
    assert_eq!(value(&app, &rate), 5.0, "Right clamps at the maximum");
    assert_eq!(changes.borrow().as_slice(), &[4.0, 1.0, 5.0]);
    press(&mut window, "left");
    press(&mut window, "down");
    press(&mut window, "enter");
    assert_eq!(value(&app, &rate), 3.0);
    assert_eq!(changes.borrow().as_slice(), &[4.0, 1.0, 5.0, 3.0]);
}

#[test]
fn unrated_rate_arrows_preview_and_commit_the_first_star() {
    let mut app = gpui::TestAppContext::single();
    app.update(crate::init_liora);
    let right_changes = Rc::new(RefCell::new(Vec::new()));
    let up_changes = Rc::new(RefCell::new(Vec::new()));
    let right_rate = make_rate(&mut app, 0.0, 5, false, right_changes.clone());
    let up_rate = make_rate(&mut app, 0.0, 5, false, up_changes.clone());
    let (mut window, _, _) = mount(
        &mut app,
        vec![right_rate.clone(), up_rate.clone()],
        Rc::new(Cell::new(0)),
    );

    press(&mut window, "tab");
    press(&mut window, "right");
    assert_eq!(
        window.update(|window, cx| right_rate.read(cx).active_value(window)),
        1.0
    );
    press(&mut window, "space");
    assert_eq!(value(&app, &right_rate), 1.0);
    assert_eq!(right_changes.borrow().as_slice(), &[1.0]);

    press(&mut window, "tab");
    press(&mut window, "up");
    assert_eq!(
        window.update(|window, cx| up_rate.read(cx).active_value(window)),
        1.0
    );
    press(&mut window, "enter");
    assert_eq!(value(&app, &up_rate), 1.0);
    assert_eq!(up_changes.borrow().as_slice(), &[1.0]);
}

#[test]
fn leaving_rate_discards_uncommitted_keyboard_preview() {
    let mut app = gpui::TestAppContext::single();
    app.update(crate::init_liora);
    let changes = Rc::new(RefCell::new(Vec::new()));
    let rate = make_rate(&mut app, 2.0, 5, false, changes.clone());
    let (mut window, _, after) = mount(&mut app, vec![rate.clone()], Rc::new(Cell::new(0)));

    press(&mut window, "tab");
    press(&mut window, "right");
    press(&mut window, "tab");
    assert_eq!(focused(&mut window), after);
    assert_eq!(value(&app, &rate), 2.0);
    assert_eq!(
        window.update(|window, cx| rate.read(cx).active_value(window)),
        2.0
    );

    press(&mut window, "shift-tab");
    press(&mut window, "enter");
    assert!(changes.borrow().is_empty());
}

#[test]
fn hover_does_not_replace_keyboard_active_value_and_modified_keys_bubble() {
    let mut app = gpui::TestAppContext::single();
    app.update(crate::init_liora);
    let changes = Rc::new(RefCell::new(Vec::new()));
    let parent_keys = Rc::new(Cell::new(0));
    let rate = make_rate(&mut app, 2.0, 5, false, changes.clone());
    let (mut window, _, _) = mount(&mut app, vec![rate.clone()], parent_keys.clone());

    press(&mut window, "tab");
    press(&mut window, "right");
    window.simulate_mouse_move(
        gpui::point(gpui::px(10.0), gpui::px(34.0)),
        None,
        gpui::Modifiers::none(),
    );
    window.update(|window, cx| window.draw(cx).clear(cx));
    press(&mut window, "right");
    let active_preview = window.update(|window, cx| rate.read(cx).active_value(window));
    assert_eq!(
        active_preview, 4.0,
        "keyboard preview must override stale hover preview"
    );
    assert_eq!(value(&app, &rate), 2.0);
    assert!(changes.borrow().is_empty());

    let parent_before_modified_key = parent_keys.get();
    press(&mut window, "ctrl-right");
    assert_eq!(parent_keys.get(), parent_before_modified_key + 1);
    assert!(
        changes.borrow().is_empty(),
        "modified arrows are not Rate commands"
    );
    press(&mut window, "enter");
    assert_eq!(changes.borrow().as_slice(), &[4.0]);
    app.quit();
}

#[test]
fn disabled_rate_is_skipped_by_tab_and_ignores_pointer_and_keyboard_activation() {
    let mut app = gpui::TestAppContext::single();
    app.update(crate::init_liora);
    let changes = Rc::new(RefCell::new(Vec::new()));
    let rate = make_rate(&mut app, 2.0, 5, true, changes.clone());
    let (mut window, _, after) = mount(&mut app, vec![rate.clone()], Rc::new(Cell::new(0)));

    press(&mut window, "tab");
    assert_eq!(
        focused(&mut window),
        after,
        "disabled Rate is not a Tab stop"
    );
    for key in ["right", "enter", "space"] {
        press(&mut window, key);
    }
    window.simulate_click(
        gpui::point(gpui::px(106.0), gpui::px(34.0)),
        gpui::Modifiers::none(),
    );
    window.update(|window, cx| window.draw(cx).clear(cx));
    assert_eq!(value(&app, &rate), 2.0);
    assert!(changes.borrow().is_empty());
}

#[test]
fn sibling_rates_keep_independent_keyboard_previews_and_callbacks() {
    let mut app = gpui::TestAppContext::single();
    app.update(crate::init_liora);
    let first_changes = Rc::new(RefCell::new(Vec::new()));
    let second_changes = Rc::new(RefCell::new(Vec::new()));
    let first = make_rate(&mut app, 2.0, 5, false, first_changes.clone());
    let second = make_rate(&mut app, 4.0, 5, false, second_changes.clone());
    let (mut window, _, _) = mount(
        &mut app,
        vec![first.clone(), second.clone()],
        Rc::new(Cell::new(0)),
    );
    let first_focus = rate_focus(&app, &first);
    let second_focus = rate_focus(&app, &second);

    press(&mut window, "tab");
    assert_eq!(focused(&mut window), first_focus);
    press(&mut window, "right");
    press(&mut window, "tab");
    assert_eq!(focused(&mut window), second_focus);
    press(&mut window, "right");
    press(&mut window, "enter");
    assert_eq!(first_changes.borrow().as_slice(), &[] as &[f32]);
    assert_eq!(second_changes.borrow().as_slice(), &[5.0]);

    press(&mut window, "shift-tab");
    assert_eq!(focused(&mut window), first_focus);
    assert!(first_changes.borrow().is_empty());
    press(&mut window, "right");
    press(&mut window, "enter");
    assert_eq!(first_changes.borrow().as_slice(), &[3.0]);
    assert_eq!(second_changes.borrow().as_slice(), &[5.0]);
}

#[test]
fn pointer_selection_still_uses_the_existing_change_callback() {
    let mut app = gpui::TestAppContext::single();
    app.update(crate::init_liora);
    let changes = Rc::new(RefCell::new(Vec::new()));
    let rate = make_rate(&mut app, 2.0, 5, false, changes.clone());
    let (mut window, _, _) = mount(&mut app, vec![rate.clone()], Rc::new(Cell::new(0)));

    window.simulate_click(
        gpui::point(gpui::px(106.0), gpui::px(34.0)),
        gpui::Modifiers::none(),
    );
    window.update(|window, cx| window.draw(cx).clear(cx));

    assert_eq!(value(&app, &rate), 5.0);
    assert_eq!(changes.borrow().as_slice(), &[5.0]);
}
