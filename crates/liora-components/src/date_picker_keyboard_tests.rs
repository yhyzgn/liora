use super::*;
use gpui::{FocusHandle, Render, VisualTestContext};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
struct Page {
    before: FocusHandle,
    after: FocusHandle,
    pickers: [Entity<DatePicker>; 2],
    modified: Rc<Cell<usize>>,
}
impl Render for Page {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let modified = self.modified.clone();
        div()
            .size_full()
            .flex()
            .flex_col()
            .on_key_down(move |event, window, cx| {
                let m = event.keystroke.modifiers;
                if m.control
                    || m.alt
                    || m.platform
                    || m.function
                    || (m.shift && event.keystroke.key != "tab")
                {
                    modified.set(modified.get() + 1);
                } else if event.keystroke.key == "tab" {
                    if m.shift {
                        window.focus_prev(cx)
                    } else {
                        window.focus_next(cx)
                    }
                }
            })
            .child(div().id("before").size(px(20.0)).track_focus(&self.before))
            .children(self.pickers.iter().cloned())
            .child(
                div()
                    .id("after")
                    .debug_selector(|| "after".into())
                    .size(px(30.0))
                    .track_focus(&self.after)
                    .on_mouse_down(MouseButton::Left, {
                        let focus = self.after.clone();
                        move |_, window, cx| focus.focus(window, cx)
                    }),
            )
            .child(crate::PortalLayer)
    }
}
fn date(day: u32) -> DateValue {
    DateValue::new(2026, 6, day).unwrap()
}
fn key(w: &mut VisualTestContext, key: &str) {
    w.simulate_keystrokes(key);
    w.update(|w, cx| {
        let keystroke = gpui::Keystroke::parse_with_os(key, cx.platform_info().os).unwrap();
        w.dispatch_event(
            gpui::PlatformInput::KeyUp(gpui::KeyUpEvent { keystroke }),
            cx,
        );
    });
    draw(w);
}
fn draw(w: &mut VisualTestContext) {
    w.update(|w, cx| {
        w.refresh();
        w.draw(cx).clear(cx);
        w.draw(cx).clear(cx);
    });
}
fn fixture() -> (
    gpui::TestAppContext,
    VisualTestContext,
    [Entity<DatePicker>; 2],
    Rc<RefCell<Vec<DateValue>>>,
    Rc<Cell<usize>>,
    FocusHandle,
    FocusHandle,
) {
    let mut app = gpui::TestAppContext::single();
    app.update(crate::init_liora);
    let before = app.update(|cx| cx.focus_handle().tab_stop(true));
    let after = app.update(|cx| cx.focus_handle().tab_stop(true));
    let calls = Rc::new(RefCell::new(Vec::new()));
    let modified = Rc::new(Cell::new(0));
    let pickers = app.update(|cx| {
        std::array::from_fn(|i| {
            let calls = calls.clone();
            cx.new(|_| {
                DatePicker::new()
                    .id(format!("picker-{i}"))
                    .value(date(15))
                    .on_change(move |v, _, _| calls.borrow_mut().push(v.unwrap()))
            })
        })
    });
    let handle = app.add_window(|_, _| Page {
        before: before.clone(),
        after: after.clone(),
        pickers: pickers.clone(),
        modified: modified.clone(),
    });
    let mut w = VisualTestContext::from_window(handle.into(), &app);
    draw(&mut w);
    w.update(|w, cx| before.focus(w, cx));
    (app, w, pickers, calls, modified, before, after)
}
#[test]
fn dispatched_entry_navigation_commit_and_sibling_identity() {
    let (app, mut w, p, calls, modified, before, after) = fixture();
    key(&mut w, "tab");
    let trigger = w.update(|w, cx| w.focused(cx)).unwrap();
    assert_ne!(trigger, before);
    assert_ne!(trigger, after);
    key(&mut w, "enter");
    w.update(|_, cx| assert!(p[0].read(cx).is_open));
    for k in ["right", "down", "home", "end", "left", "up"] {
        key(&mut w, k)
    }
    assert!(calls.borrow().is_empty());
    w.update(|_, cx| assert_eq!(p[0].read(cx).value_ref(), Some(date(15))));
    for k in [
        "ctrl-right",
        "shift-left",
        "alt-enter",
        "cmd-space",
        "fn-home",
    ] {
        key(&mut w, k)
    }
    assert_eq!(modified.get(), 5);
    draw(&mut w);
    key(&mut w, "space");
    assert_eq!(*calls.borrow(), [date(20)]);
    w.update(|w, cx| {
        assert!(!p[0].read(cx).is_open);
        assert_eq!(w.focused(cx), Some(trigger.clone()));
    });
    key(&mut w, "tab");
    key(&mut w, "space");
    w.update(|_, cx| {
        assert!(!p[0].read(cx).is_open);
        assert!(p[1].read(cx).is_open)
    });
    key(&mut w, "enter");
    assert_eq!(*calls.borrow(), [date(20), date(15)]);
    app.quit();
}
#[test]
fn header_reanchor_policies_tab_exit_and_retirement() {
    let (app, mut w, p, calls, _, before, after) = fixture();
    key(&mut w, "tab");
    key(&mut w, "enter");
    key(&mut w, "shift-tab"); // next-year header
    key(&mut w, "space");
    w.update(|_, cx| assert_eq!(p[0].read(cx).view_year, 2027));
    key(&mut w, "tab");
    key(&mut w, "enter");
    assert_eq!(*calls.borrow(), [DateValue::new(2027, 6, 1).unwrap()]);
    key(&mut w, "enter");
    key(&mut w, "escape");
    w.update(|_, cx| assert!(!p[0].read(cx).is_open));
    w.update(|_, cx| p[0].update(cx, |p, _| p.close_on_escape = false));
    key(&mut w, "space");
    key(&mut w, "escape");
    w.update(|_, cx| assert!(p[0].read(cx).is_open));
    key(&mut w, "tab");
    w.update(|_, cx| assert!(!p[0].read(cx).is_open)); // next sibling trigger
    key(&mut w, "space");
    key(&mut w, "tab");
    w.update(|w, _| assert!(after.is_focused(w)));
    key(&mut w, "shift-tab");
    key(&mut w, "enter");
    for _ in 0..5 {
        key(&mut w, "shift-tab");
    } // through four header stops to trigger
    key(&mut w, "shift-tab"); // first picker trigger
    key(&mut w, "space");
    w.update(|_, cx| {
        p[0].update(cx, |p, cx| {
            p.disabled = true;
            cx.notify();
        })
    });
    draw(&mut w);
    w.update(|_, cx| assert!(!p[0].read(cx).is_open));
    assert_eq!(calls.borrow().len(), 1);
    w.update(|w, cx| before.focus(w, cx));
    key(&mut w, "tab");
    key(&mut w, "space");
    w.update(|_, cx| assert!(p[1].read(cx).is_open));
    app.quit();
}

fn mouse_down(w: &mut VisualTestContext, position: gpui::Point<Pixels>) {
    w.update(|w, cx| {
        w.dispatch_event(
            gpui::PlatformInput::MouseDown(gpui::MouseDownEvent {
                position,
                button: MouseButton::Left,
                modifiers: gpui::Modifiers::none(),
                click_count: 1,
                first_mouse: false,
            }),
            cx,
        );
    });
    draw(w);
}

#[test]
fn outside_dismissal_optout_and_pointer_adjacent_semantics() {
    let (app, mut w, p, calls, _, _, after) = fixture();
    key(&mut w, "tab");
    key(&mut w, "enter");
    // A real destination already holding focus must not be replaced by trigger restoration.
    w.update(|w, cx| after.focus(w, cx));
    mouse_down(&mut w, gpui::point(px(700.0), px(500.0)));
    w.update(|w, cx| {
        assert!(!p[0].read(cx).is_open);
        assert!(after.is_focused(w));
    });
    assert!(calls.borrow().is_empty());
    w.update(|_, cx| p[0].update(cx, |p, _| p.close_on_click_outside = false));
    key(&mut w, "shift-tab");
    key(&mut w, "shift-tab");
    key(&mut w, "space");
    mouse_down(&mut w, gpui::point(px(700.0), px(500.0)));
    w.update(|_, cx| assert!(p[0].read(cx).is_open));
    // June 2026's fifth week, third cell is adjacent July 1: pointer selection remains legal.
    let position = w
        .debug_bounds("picker-0-day-4-2")
        .expect("adjacent July date")
        .center();
    mouse_down(&mut w, position);
    assert_eq!(*calls.borrow(), [DateValue::new(2026, 7, 1).unwrap()]);
    w.update(|_, cx| assert!(!p[0].read(cx).is_open));
    app.quit();
}
