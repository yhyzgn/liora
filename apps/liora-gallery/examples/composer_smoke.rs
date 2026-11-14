//! Native assertion-driven composer interaction regression (Linux/FreeBSD headless).
//! cargo run -p liora-gallery --example composer_smoke --locked
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn main() {
    use gpui::{
        AppContext, ClipboardEntry, ClipboardItem, EntityInputHandler, ExternalPaths, Focusable,
        Image, ImageFormat, Keystroke, WindowOptions,
    };
    use liora_components::{
        Composer, ComposerAttachment, ComposerCandidate, ComposerDraft, ComposerEvent,
        ComposerReference, init_liora,
    };
    use std::{cell::RefCell, rc::Rc, time::Duration};
    gpui_platform::headless().run(|cx| {
        init_liora(cx);
        let composer = cx.new(Composer::new);
        let events = Rc::new(RefCell::new(Vec::<ComposerEvent>::new()));
        let subscription = cx.subscribe(&composer, {
            let events = events.clone();
            move |_, event, _| events.borrow_mut().push(event.clone())
        });
        let handle = cx.open_window(WindowOptions { show: false, ..Default::default() },
            |_, _| composer.clone()).unwrap();
        let executor = cx.background_executor().clone();
        cx.spawn(async move |cx| {
            executor.timer(Duration::from_millis(10)).await;
            cx.update(|cx| {
                cx.update_window(handle.into(), |_, window, cx| {
                    composer.update(cx, |composer, cx| {
                        composer.set_candidates(vec![
                            ComposerCandidate::Reference(ComposerReference::new("alice", "Alice")),
                            ComposerCandidate::Command { id: "sum".into(), label: "summarize".into(), text: "/summarize ".into() },
                        ], cx);
                        composer.set_draft(ComposerDraft { text: "你好 @al suffix".into(), ..Default::default() }, cx);
                    });
                    let input = composer.read(cx).input();
                    input.update(cx, |input, cx| input.set_selection(10..10, cx));
                    composer.update(cx, |composer, cx| assert!(composer.accept_candidate(0, cx)));
                    assert_eq!(composer.read(cx).draft(cx).text.as_ref(), "你好 @Alice  suffix");
                    assert_eq!(input.read(cx).selected_range(), 14..14);
                    assert_eq!(composer.read(cx).draft(cx).references[0].id.as_ref(), "alice");
                    composer.update(cx, |composer, cx| {
                        composer.add_reference(ComposerReference::new("alice", "Duplicate"), cx);
                        assert_eq!(composer.draft(cx).references.len(), 1);
                        composer.remove_reference("alice", cx);
                        composer.add_attachment(ComposerAttachment::new("a", "notes.txt"), cx);
                        composer.remove_attachment("a", cx);
                        assert!(composer.draft(cx).references.is_empty());
                        assert!(composer.draft(cx).attachments.is_empty());
                        composer.set_draft(ComposerDraft { text: "aXYZb".into(), ..Default::default() }, cx);
                    });
                    input.update(cx, |input, cx| input.set_selection(1..4, cx));
                    let payload = ClipboardItem { entries: vec![
                        ClipboardEntry::from("粘贴".to_string()),
                        ClipboardEntry::Image(Image { format: ImageFormat::Png, bytes: vec![1, 2, 3], id: 42 }),
                        ClipboardEntry::ExternalPaths(ExternalPaths(vec!["/tmp/native-file.txt".into()].into())),
                    ] };
                    window.focus(&composer.read(cx).focus_handle(cx), cx);
                    window.draw(cx).clear(cx);
                    // The official headless backend has no system clipboard; test the public payload boundary.
                    composer.update(cx, |composer, cx| composer.paste(payload, cx));
                    assert_eq!(composer.read(cx).draft(cx).text.as_ref(), "a粘贴b");
                    assert_eq!(input.read(cx).selected_range(), 7..7);
                    input.update(cx, |input, cx| {
                        input.replace_and_mark_text_in_range(Some(1..3), "候选", Some(2..2), window, cx);
                    });
                    composer.update(cx, |composer, cx| assert!(!composer.send(window, cx), "marked text cannot submit"));
                    window.focus(&composer.read(cx).focus_handle(cx), cx);
                    window.draw(cx).clear(cx);
                    let before = input.read(cx).value();
                    window.dispatch_keystroke(Keystroke::parse("enter").unwrap(), cx);
                    assert_eq!(input.read(cx).value(), before, "IME Enter must not insert or send");
                    input.update(cx, |input, cx| {
                        input.replace_text_in_range(None, "确认", window, cx);
                    });
                    window.draw(cx).clear(cx);
                    window.dispatch_keystroke(Keystroke::parse("shift-enter").unwrap(), cx);
                    assert!(input.read(cx).value().contains('\n'));
                    composer.update(cx, |composer, cx| {
                        composer.set_draft(ComposerDraft { text: "/su tail".into(), ..Default::default() }, cx);
                    });
                    input.update(cx, |input, cx| input.set_selection(3..3, cx));
                    window.draw(cx).clear(cx);
                    window.dispatch_keystroke(Keystroke::parse("enter").unwrap(), cx);
                    assert_eq!(input.read(cx).value().as_ref(), "/summarize  tail", "first Enter accepts candidate, not send");
                    composer.update(cx, |composer, cx| {
                        assert!(composer.send(window, cx));
                        assert!(composer.draft(cx).text.is_empty());
                        composer.set_busy(true, cx);
                    });
                    input.update(cx, |input, cx| input.insert_text("next draft", cx));
                    composer.update(cx, |composer, cx| {
                        assert!(!composer.send(window, cx));
                        composer.stop(cx);
                        assert!(composer.is_busy(), "only host completes stop");
                        assert_eq!(composer.draft(cx).text.as_ref(), "next draft");
                        composer.set_busy(false, cx);
                    });
                    window.draw(cx).clear(cx);
                    window.dispatch_keystroke(Keystroke::parse("enter").unwrap(), cx);
                    assert!(input.read(cx).value().is_empty(), "native Enter sends next draft");
                    composer.update(cx, |composer, cx| {
                        composer.set_draft(ComposerDraft { text: "ab".into(), ..Default::default() }, cx);
                    });
                    input.update(cx, |input, cx| {
                        input.replace_and_mark_text_in_range(Some(0..2), "ab", Some(0..2), window, cx);
                    });
                    composer.update(cx, |composer, cx| {
                        composer.set_draft(ComposerDraft { text: "你好 restored".into(), ..Default::default() }, cx);
                    });
                    input.update(cx, |input, cx| {
                        assert_eq!(input.marked_text_range(window, cx), None);
                        let selection = input.selected_text_range(false, window, cx).unwrap();
                        assert_eq!(selection.range, 11..11);
                        assert!(!selection.reversed);
                        input.replace_text_in_range(None, "!", window, cx);
                        assert_eq!(input.value().as_ref(), "你好 restored!");
                        input.replace_and_mark_text_in_range(Some(0..2), "你好", None, window, cx);
                    });
                    // Same-text restoration also cancels composition, not just changed text.
                    let same_draft = composer.read(cx).draft(cx);
                    composer.update(cx, |composer, cx| composer.set_draft(same_draft, cx));
                    input.update(cx, |input, cx| {
                        assert_eq!(input.marked_text_range(window, cx), None);
                        input.replace_text_in_range(None, "?", window, cx);
                        assert_eq!(input.value().as_ref(), "你好 restored!?");
                    });
                    composer.update(cx, |composer, cx| assert!(composer.send(window, cx)));
                }).unwrap();
            });
            cx.update(|cx| {
                let events = events.borrow();
                let sends: Vec<_> = events.iter().filter_map(|event| match event {
                    ComposerEvent::Send(draft) => Some(draft), _ => None,
                }).collect();
                assert_eq!(sends.len(), 3, "IME, candidate acceptance and busy must not send");
                assert_eq!(sends[0].text.as_ref(), "/summarize  tail");
                assert_eq!(sends[1].text.as_ref(), "next draft");
                assert_eq!(sends[2].text.as_ref(), "你好 restored!?");
                assert_eq!(events.iter().filter(|event| matches!(event, ComposerEvent::Stop)).count(), 1);
                let paste = events.iter().find_map(|event| match event { ComposerEvent::Paste(item) => Some(item), _ => None }).unwrap();
                assert_eq!(paste.entries().len(), 3, "host receives text, image and file without lossy conversion");
                assert!(matches!(&paste.entries()[1], ClipboardEntry::Image(image) if image.bytes == [1, 2, 3]));
                assert!(events.iter().any(|event| matches!(event, ComposerEvent::DraftChanged(draft) if draft.text.as_ref() == "/su tail")));
                drop(subscription);
                println!("PASS: native Composer drafts, selection, candidates, injected paste payload, marked composition, send and stop");
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
