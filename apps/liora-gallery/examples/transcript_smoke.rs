//! Native variable-height transcript layout and scroll regression; no display/GPU claim.
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn main() {
    use gpui::{AppContext, IntoElement, WindowOptions, div, prelude::*, px};
    use liora_components::{Transcript, TranscriptPosition, init_liora};
    use std::{cell::Cell, rc::Rc, time::Duration};

    gpui_platform::headless().run(|cx| {
        init_liora(cx);
        let growing_height = Rc::new(Cell::new(48.0));
        let anchor_height = Rc::new(Cell::new(48.0));
        let anchor = anchor_height.clone();
        let height = growing_height.clone();
        let transcript = cx.new(|_| {
            let mut view = Transcript::new(move |id, _, _| {
                let row_height = match id.as_ref() { "25" => anchor.get(), "79" => height.get(), _ => 48.0 };
                div().h(px(row_height)).w_full().child(id.clone()).into_any_element()
            });
            view.set_items((0..80).map(|index| index.to_string().into()).collect()).unwrap();
            view
        });
        let handle = cx.open_window(WindowOptions { show: false, ..Default::default() }, |_, _| transcript.clone()).unwrap();
        let draw = |cx: &mut gpui::App| {
            cx.update_window(handle.into(), |_, window, cx| {
                window.refresh();
                window.draw(cx).clear(cx);
            }).unwrap();
        };
        transcript.update(cx, |view, cx| {
            view.restore_position(&TranscriptPosition { item: Some("25".into()), offset: 800.0, following_tail: false }).unwrap();
            cx.notify();
        });
        draw(cx);
        let cold_state = transcript.read(cx).list_state();
        let cold_position = transcript.read(cx).position();
        assert_eq!(cold_position.item.as_deref(), Some("25"), "cold restore must not spill into later rows");
        assert_eq!(cold_position.offset, 48.0);
        let cold_bounds = cold_state.bounds_for_item(25).expect("cold restored anchor is measured");
        assert_eq!(cold_bounds.size.height, px(48.0));
        assert!((cold_bounds.top() + px(cold_position.offset) - cold_state.viewport_bounds().top()).abs() < px(1.0));
        transcript.update(cx, |view, cx| { view.reveal_item("50").unwrap(); cx.notify(); });
        draw(cx);
        assert_eq!(transcript.read(cx).position().item.as_deref(), Some("50"), "reveal before first layout must reach an unmeasured row");
        transcript.update(cx, |view, cx| { view.follow_tail(); cx.notify(); });
        draw(cx);
        let state = transcript.read(cx).list_state();
        assert!(transcript.read(cx).is_following_tail());
        let bounds = state.bounds_for_item(79).expect("tail row is laid out");
        assert!((bounds.bottom() - state.viewport_bounds().bottom()).abs() < px(1.0));

        transcript.update(cx, |view, cx| {
            view.restore_position(&TranscriptPosition { item: Some("25".into()), offset: 7.0, following_tail: false }).unwrap();
            cx.notify();
        });
        draw(cx);
        let saved = transcript.read(cx).position();
        assert_eq!(saved.item.as_deref(), Some("25"));
        assert_eq!(saved.offset, 7.0);
        anchor_height.set(96.0);
        transcript.update(cx, |view, cx| {
            view.notify_item_changed("25").unwrap();
            view.restore_position(&saved).unwrap();
            cx.notify();
        });
        draw(cx);
        assert_eq!(transcript.read(cx).position(), saved, "restored pixel offsets must not scale with row height");
        let changed_bounds = state.bounds_for_item(25).expect("changed-height restored anchor is measured");
        assert_eq!(changed_bounds.size.height, px(96.0));
        assert!((changed_bounds.top() + px(7.0) - state.viewport_bounds().top()).abs() < px(1.0));
        anchor_height.set(48.0);
        transcript.update(cx, |view, cx| { view.notify_item_changed("25").unwrap(); cx.notify(); });
        draw(cx);
        transcript.update(cx, |view, cx| {
            let mut ids = vec!["history-1".into(), "history-2".into()];
            ids.extend_from_slice(view.items());
            view.set_items(ids).unwrap();
            cx.notify();
        });
        draw(cx);
        assert_eq!(transcript.read(cx).position(), saved, "prepending history must preserve item and offset");
        assert_eq!(state.logical_scroll_top().item_ix, 27);

        growing_height.set(180.0);
        transcript.update(cx, |view, cx| { view.notify_item_changed("79").unwrap(); cx.notify(); });
        draw(cx);
        assert_eq!(transcript.read(cx).position(), saved, "offscreen growth must not steal the viewport");
        transcript.update(cx, |view, cx| { view.follow_tail(); cx.notify(); });
        draw(cx);
        let bounds = state.bounds_for_item(81).expect("growing tail row is laid out");
        assert_eq!(bounds.size.height, px(180.0));
        assert!((bounds.bottom() - state.viewport_bounds().bottom()).abs() < px(1.0));

        state.scroll_by(px(-100.0));
        draw(cx);
        assert!(!transcript.read(cx).is_following_tail(), "scrolling upward detaches tail following");
        transcript.update(cx, |view, cx| { view.restore_position(&saved).unwrap(); cx.notify(); });
        draw(cx);
        assert_eq!(transcript.read(cx).position(), saved);
        transcript.update(cx, |view, cx| {
            view.restore_position(&TranscriptPosition { item: Some("25".into()), offset: 800.0, following_tail: false }).unwrap();
            cx.notify();
        });
        draw(cx);
        assert!(transcript.read(cx).position().offset <= 48.0, "saved offsets must clamp to a shrunken anchor row");
        assert_eq!(transcript.read(cx).position().item.as_deref(), Some("25"));
        let restored_bounds = state.bounds_for_item(27).expect("shrunken anchor is measured");
        assert!((restored_bounds.top() + px(transcript.read(cx).position().offset) - state.viewport_bounds().top()).abs() < px(1.0));
        println!("PASS: native Transcript variable-height layout, tail following, detachment, prepend anchor and position restore");
        let executor = cx.background_executor().clone();
        cx.spawn(async move |cx| {
            executor.timer(Duration::from_millis(10)).await;
            cx.update(|cx| cx.quit());
        }).detach();
    });
}

#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
fn main() {
    eprintln!("This smoke requires the official Linux/FreeBSD headless backend.");
    std::process::exit(1);
}
