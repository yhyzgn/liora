//! cargo run -p liora-gallery --example diff_smoke --locked
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn main() {
    use gpui::{
        AppContext, MouseButton, MouseDownEvent, MouseUpEvent, PlatformInput, WindowOptions, point,
        px,
    };
    use liora_components::{
        DiffActionError, DiffDecision, DiffDocument, DiffEvent, DiffLayout, DiffStatus, DiffView,
        init_liora,
    };
    use std::{cell::RefCell, rc::Rc, time::Duration};
    gpui_platform::headless().run(|cx| {
        init_liora(cx);
        let normalized = cx.new(|cx| DiffView::with_status(
            DiffDocument::new("left", "right"), DiffStatus::Empty, cx,
        ).unwrap());
        assert_eq!(normalized.read(cx).status(), DiffStatus::Ready);
        assert!(!normalized.read(cx).hunks().is_empty());
        let empty = cx.new(|cx| DiffView::with_status(
            DiffDocument::new("", ""), DiffStatus::Ready, cx,
        ).unwrap());
        assert_eq!(empty.read(cx).status(), DiffStatus::Empty);
        let view = cx.new(|cx| DiffView::new(DiffDocument::new("old\n", "新\n"), cx).unwrap());
        let events = Rc::new(RefCell::new(Vec::<DiffEvent>::new()));
        let subscription = cx.subscribe(&view, {
            let events = events.clone();
            move |_, event, _| events.borrow_mut().push(event.clone())
        });
        let handle = cx.open_window(WindowOptions { show: false, ..Default::default() }, |_, _| view.clone()).unwrap();
        let executor = cx.background_executor().clone();
        cx.spawn(async move |cx| {
            executor.timer(Duration::from_millis(10)).await;
            cx.update(|cx| {
                cx.update_window(handle.into(), |_, window, cx| {
                    window.draw(cx).clear(cx);
                    let position = point(px(120.), px(14.));
                    window.dispatch_event(PlatformInput::MouseDown(MouseDownEvent { button: MouseButton::Left, position, click_count: 1, ..Default::default() }), cx);
                    window.dispatch_event(PlatformInput::MouseUp(MouseUpEvent { button: MouseButton::Left, position, click_count: 1, ..Default::default() }), cx);
                    assert_eq!(view.read(cx).layout(), DiffLayout::SideBySide, "native layout button must switch the rendered mode");
                    let id = view.read(cx).hunks()[0].id;
                    view.update(cx, |view, cx| {
                        assert_eq!(view.decide_hunk(id, DiffDecision::Accepted, cx), Ok(DiffEvent::HunkAccepted { hunk_id: id }));
                        assert_eq!(view.hunks()[0].decision, Some(DiffDecision::Accepted));
                        view.set_status(DiffStatus::Empty, cx).unwrap();
                        assert_eq!(view.status(), DiffStatus::Ready, "nonempty text cannot be forced empty");
                        view.set_status(DiffStatus::Ready, cx).unwrap();
                        assert_eq!(view.hunks()[0].id, id, "unchanged availability keeps hunk identity");
                        assert_eq!(view.hunks()[0].decision, Some(DiffDecision::Accepted));
                        assert!(matches!(view.decide_hunk(id, DiffDecision::Rejected, cx), Err(DiffActionError::AlreadyDecided { .. })));
                        assert!(matches!(view.decide_hunk(0, DiffDecision::Accepted, cx), Err(DiffActionError::UnknownHunk { .. })));
                        assert_eq!(view.document().after.as_ref(), "新\n");
                        let too_many = "x\n".repeat(100_001);
                        assert!(view.set_document(DiffDocument::new(too_many, ""), cx).is_err());
                        assert_eq!(view.document().after.as_ref(), "新\n");
                        assert_eq!(view.hunks()[0].id, id);
                        view.set_document(DiffDocument::new("x", "y"), cx).unwrap();
                        assert!(matches!(view.decide_hunk(id, DiffDecision::Accepted, cx), Err(DiffActionError::UnknownHunk { .. })));
                        assert_eq!(view.hunks()[0].decision, None);
                        let fresh = view.hunks()[0].id;
                        assert_eq!(view.decide_hunk(fresh, DiffDecision::Rejected, cx), Ok(DiffEvent::HunkRejected { hunk_id: fresh }));
                        view.set_document(DiffDocument::new("", ""), cx).unwrap();
                        assert_eq!(view.status(), DiffStatus::Empty);
                        assert!(view.hunks().is_empty());
                        let source = view.document().clone();
                        view.set_status(DiffStatus::Binary, cx).unwrap();
                        assert_eq!(view.status(), DiffStatus::Binary);
                        assert!(view.hunks().is_empty());
                        assert_eq!(view.document(), &source);
                        view.set_status(DiffStatus::NotApplicable, cx).unwrap();
                        assert_eq!(view.status(), DiffStatus::NotApplicable);
                        assert!(view.hunks().is_empty());
                        view.set_status(DiffStatus::Ready, cx).unwrap();
                        assert_eq!(view.status(), DiffStatus::Empty);
                        assert!(view.hunks().is_empty());
                    });
                    window.draw(cx).clear(cx);
                }).unwrap();
            });
            cx.update(|cx| {
                assert_eq!(events.borrow().len(), 2, "only successful decisions emit acceptance/rejection events");
                drop(subscription);
                println!("PASS: diff native layout, accepted/rejected events, stale IDs, terminal decisions and immutable source");
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
