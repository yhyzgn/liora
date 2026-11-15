//! Native FindBar action-routing regression (Linux/FreeBSD headless).
//! cargo run -p liora-gallery --example find_bar_smoke --locked
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn main() {
    use gpui::{
        App, AppContext, Context, Entity, EntityInputHandler, Focusable, IntoElement, Keystroke,
        ModifiersChangedEvent, PlatformInput, Render, Window, WindowOptions, div, prelude::*,
    };
    use liora_components::{
        CandidateBar, CandidateBarEvent, CandidateItem, FindBar, FindBarEvent, Input, init_liora,
    };
    use std::{cell::RefCell, rc::Rc, time::Duration};

    struct Surface {
        find: Entity<FindBar>,
        ordinary: Entity<Input>,
        candidates: Entity<CandidateBar>,
    }
    impl Render for Surface {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .flex()
                .flex_col()
                .child(self.find.clone())
                .child(self.ordinary.clone())
                .child(self.candidates.clone())
        }
    }
    fn key(text: &'static str, window: &mut Window, cx: &mut App) {
        let stroke = Keystroke::parse(text).unwrap();
        window.dispatch_event(
            PlatformInput::ModifiersChanged(ModifiersChangedEvent {
                modifiers: stroke.modifiers,
                ..Default::default()
            }),
            cx,
        );
        window.dispatch_keystroke(stroke, cx);
    }

    gpui_platform::headless().run(|cx| {
        init_liora(cx);
        let find = cx.new(FindBar::new);
        let ordinary = cx.new(|cx| Input::new("ordinary", cx));
        let candidates = cx.new(CandidateBar::new);
        candidates.update(cx, |bar, cx| {
            bar.set_composition("literal", cx);
            bar.set_items((0..10).map(|index| CandidateItem::new(index.to_string())).collect(), cx);
        });
        let candidate_events = Rc::new(RefCell::new(Vec::<CandidateBarEvent>::new()));
        let candidate_subscription = cx.subscribe(&candidates, {
            let events = candidate_events.clone();
            move |_, event, _| events.borrow_mut().push(event.clone())
        });
        let events = Rc::new(RefCell::new(Vec::<FindBarEvent>::new()));
        let subscription = cx.subscribe(&find, {
            let events = events.clone();
            move |_, event, _| events.borrow_mut().push(event.clone())
        });
        let surface = cx.new(|_| Surface { find: find.clone(), ordinary: ordinary.clone(), candidates: candidates.clone() });
        let handle = cx.open_window(WindowOptions { show: false, ..Default::default() },
            |_, _| surface).unwrap();
        let executor = cx.background_executor().clone();
        cx.spawn(async move |cx| {
            executor.timer(Duration::from_millis(10)).await;
            cx.update(|cx| {
                find.update(cx, |find, cx| {
                    find.set_query("needle", cx);
                    find.set_match_status("1 of 2", cx);
                });
            });
            cx.update(|cx| {
                cx.update_window(handle.into(), |_, window, cx| {
                    window.focus(&find.read(cx).focus_handle(cx), cx);
                    window.draw(cx).clear(cx);
                    key("enter", window, cx);
                    assert_eq!(find.read(cx).query(cx).as_ref(), "needle", "Enter must not insert a newline");
                    key("shift-enter", window, cx);
                    assert_eq!(find.read(cx).query(cx).as_ref(), "needle", "Shift-Enter must not insert a newline");
                    key("ctrl-enter", window, cx);
                    key("alt-enter", window, cx);
                    key("escape", window, cx);
                }).unwrap();
            });
            cx.update(|cx| {
                assert_eq!(&*events.borrow(), &[
                    FindBarEvent::QueryChanged { query: "needle".into() },
                    FindBarEvent::Next,
                    FindBarEvent::Previous,
                    FindBarEvent::Closed,
                ], "actual key dispatch must navigate exactly once and ignore modified shortcuts");
                events.borrow_mut().clear();
                cx.update_window(handle.into(), |_, window, cx| {
                    let input = find.read(cx).input();
                    input.update(cx, |input, cx| {
                        input.replace_and_mark_text_in_range(Some(0..6), "候选", Some(2..2), window, cx);
                    });
                    let marked = input.update(cx, |input, cx| input.marked_text_range(window, cx));
                    key("enter", window, cx);
                    key("shift-enter", window, cx);
                    key("escape", window, cx);
                    assert_eq!(find.read(cx).query(cx).as_ref(), "候选", "composition Enter must not mutate the query");
                    assert_eq!(input.update(cx, |input, cx| input.marked_text_range(window, cx)), marked, "composition range remains owned by the native IME");
                    window.focus(&ordinary.read(cx).focus_handle(cx), cx);
                    window.draw(cx).clear(cx);
                    key("enter", window, cx);
                    assert_eq!(ordinary.read(cx).value().as_ref(), "\nordinary", "ordinary Input retains its global Enter action");
                    window.focus(&candidates.read(cx).focus_handle(cx), cx);
                    window.refresh();
                    window.draw(cx).clear(cx);
                    for key_name in ["pagedown", "2", "enter", "escape"] {
                        key(key_name, window, cx);
                    }
                }).unwrap();
            });
            cx.update(|cx| {
                assert!(events.borrow().iter().all(|event| matches!(event, FindBarEvent::QueryChanged { .. })),
                    "composition and another field's focus must not navigate or close FindBar");
                drop(subscription);
                assert_eq!(&*candidate_events.borrow(), &[
                    CandidateBarEvent::PageChanged { page: 1 },
                    CandidateBarEvent::Selected { index: 9, value: "9".into() },
                    CandidateBarEvent::CommittedLiteral { composition: "literal".into() },
                    CandidateBarEvent::Cancelled,
                ]);
                drop(candidate_subscription);
                println!("PASS: FindBar actual Enter/Shift-Enter navigation without newlines, Escape, modifier/composition/focus guards and ordinary Input Enter");
                cx.quit();
            });
        }).detach();
    });
}
#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
fn main() {
    eprintln!("This smoke requires the official Linux/FreeBSD headless backend.");
    std::process::exit(1);
}
