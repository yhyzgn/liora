use super::*;
use gpui::{Context, FocusHandle, Render, VisualTestContext};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

struct KeyboardPage {
    before: FocusHandle,
    after: FocusHandle,
    selections: Rc<RefCell<Vec<(usize, CalendarDate)>>>,
    shortcuts: Rc<Cell<usize>>,
    adjacent: bool,
    selected: CalendarDate,
}

fn date(month: u32, day: u32) -> CalendarDate {
    CalendarDate::new(2026, month, day).unwrap()
}

impl Render for KeyboardPage {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let shortcuts = self.shortcuts.clone();
        div()
            .flex()
            .flex_col()
            .w(px(800.0))
            .on_key_down(move |event, window, cx| {
                let modifiers = event.keystroke.modifiers;
                if modifiers.control
                    || modifiers.alt
                    || modifiers.platform
                    || modifiers.function
                    || (modifiers.shift && event.keystroke.key != "tab")
                {
                    shortcuts.set(shortcuts.get() + 1);
                } else if event.keystroke.key == "tab" {
                    if modifiers.shift {
                        window.focus_prev(cx);
                    } else {
                        window.focus_next(cx);
                    }
                }
            })
            .child(div().id("before").size(px(20.0)).track_focus(&self.before))
            .children((0..2).map(|index| {
                let selections = self.selections.clone();
                let calendar = Calendar::new(2026, 6)
                    .id(("keyboard-calendar", index))
                    .show_adjacent_months(self.adjacent)
                    .disabled_dates([date(6, 1), date(6, 16)])
                    .on_select(move |date, _, _| selections.borrow_mut().push((index, date)));
                if index == 0 {
                    calendar.selected(self.selected)
                } else {
                    calendar
                }
            }))
            .child(div().id("after").size(px(20.0)).track_focus(&self.after))
    }
}

fn setup(
    adjacent: bool,
    selected: CalendarDate,
) -> (
    gpui::TestAppContext,
    VisualTestContext,
    gpui::WindowHandle<KeyboardPage>,
    Rc<RefCell<Vec<(usize, CalendarDate)>>>,
    Rc<Cell<usize>>,
    FocusHandle,
    FocusHandle,
) {
    let mut app = gpui::TestAppContext::single();
    app.update(crate::init_liora);
    let before = app.update(|cx| cx.focus_handle().tab_stop(true));
    let after = app.update(|cx| cx.focus_handle().tab_stop(true));
    let selections = Rc::new(RefCell::new(Vec::new()));
    let shortcuts = Rc::new(Cell::new(0));
    let handle = app.add_window(|_, _| KeyboardPage {
        before: before.clone(),
        after: after.clone(),
        selections: selections.clone(),
        shortcuts: shortcuts.clone(),
        adjacent,
        selected,
    });
    let mut window = VisualTestContext::from_window(handle.into(), &app);
    window.update(|window, cx| {
        window.draw(cx).clear(cx);
        before.focus(window, cx);
        window.draw(cx).clear(cx);
    });
    (app, window, handle, selections, shortcuts, before, after)
}

fn key(window: &mut VisualTestContext, key: &str) {
    window.simulate_keystrokes(key);
    window.update(|window, cx| window.draw(cx).clear(cx));
}

#[test]
fn dispatched_tab_enters_one_composite_and_navigation_never_selects() {
    let (mut app, mut window, handle, selections, shortcuts, before, after) =
        setup(false, date(6, 15));
    key(&mut window, "tab");
    let first_focus = window
        .update(|window, cx| window.focused(cx))
        .expect("Calendar is a Tab stop");
    assert_ne!(first_focus, before);
    assert_ne!(
        first_focus, after,
        "Tab must enter Calendar, not skip to the following stop"
    );
    key(&mut window, "enter");
    assert_eq!(*selections.borrow(), [(0, date(6, 15))]);
    key(&mut window, "right");
    key(&mut window, "enter");
    key(&mut window, "space");
    assert_eq!(
        selections.borrow().len(),
        1,
        "disabled dates can focus, never activate"
    );
    key(&mut window, "down");
    assert_eq!(selections.borrow().len(), 1, "navigation must not select");
    key(&mut window, "space");
    assert_eq!(selections.borrow().last(), Some(&(0, date(6, 23))));
    assert_eq!(selections.borrow().len(), 2, "Space activates exactly once");
    key(&mut window, "home");
    key(&mut window, "enter");
    assert_eq!(selections.borrow().last(), Some(&(0, date(6, 22))));
    key(&mut window, "end");
    key(&mut window, "left");
    key(&mut window, "up");
    key(&mut window, "enter");
    assert_eq!(selections.borrow().last(), Some(&(0, date(6, 20))));
    let count = selections.borrow().len();
    for shortcut in [
        "ctrl-right",
        "shift-left",
        "alt-down",
        "cmd-up",
        "fn-home",
        "ctrl-enter",
        "shift-space",
        "ctrl-tab",
    ] {
        key(&mut window, shortcut);
    }
    assert_eq!(shortcuts.get(), 8, "modified keys must reach the parent");
    assert_eq!(selections.borrow().len(), count);
    key(&mut window, "enter");
    assert_eq!(selections.borrow().last(), Some(&(0, date(6, 20))));
    handle.update(&mut app, |_, _, cx| cx.notify()).unwrap();
    window.update(|window, cx| {
        window.refresh();
        window.draw(cx).clear(cx);
    });
    key(&mut window, "tab");
    let second_focus = window
        .update(|window, cx| window.focused(cx))
        .expect("second Calendar focus");
    assert_ne!(second_focus, first_focus);
    assert_ne!(second_focus, after);
    key(&mut window, "enter");
    assert_eq!(
        selections.borrow().last(),
        Some(&(1, date(6, 2))),
        "initial fallback skips disabled in-month date"
    );
    key(&mut window, "tab");
    window.update(|window, _| assert!(after.is_focused(window)));
    key(&mut window, "shift-tab");
    key(&mut window, "shift-tab");
    window.update(|window, _| assert!(first_focus.is_focused(window)));
    key(&mut window, "enter");
    assert_eq!(
        selections.borrow().last(),
        Some(&(0, date(6, 20))),
        "redraw and sibling traversal retain active date independently of host selected"
    );
    key(&mut window, "shift-tab");
    window.update(|window, _| assert!(before.is_focused(window)));
    app.quit();
}

#[test]
fn rendered_boundaries_skip_hidden_adjacent_without_changing_month() {
    for adjacent in [false, true] {
        let (app, mut window, _, selections, _, _, _) = setup(adjacent, date(6, 30));
        key(&mut window, "tab");
        key(&mut window, "right");
        key(&mut window, "enter");
        assert_eq!(
            selections.borrow().last(),
            Some(&(0, if adjacent { date(7, 1) } else { date(6, 30) }))
        );
        assert!(
            window.debug_bounds("calendar-day-2026-06-01").is_some(),
            "fixed month must remain June"
        );
        key(&mut window, "home");
        key(&mut window, "enter");
        assert_eq!(selections.borrow().last(), Some(&(0, date(6, 29))));
        key(&mut window, "end");
        key(&mut window, "enter");
        assert_eq!(
            selections.borrow().last(),
            Some(&(0, if adjacent { date(7, 5) } else { date(6, 30) }))
        );
        app.quit();
    }
}

#[test]
fn full_date_names_follow_current_locale() {
    for (locale, expected) in [
        ("en-US", "Monday, June 15, 2026"),
        ("zh-CN", "2026年六月15日 星期一"),
        ("en-US", "Monday, June 15, 2026"),
    ] {
        let cx = liora_core::LocalesConfig::system().with_locale(locale);
        assert_eq!(date_name(date(6, 15), &cx).as_ref(), expected);
    }
}
