//! Native terminal event regression; run with the official Linux/FreeBSD headless backend.
//! cargo run -p liora-gallery --example terminal_smoke --locked
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn main() {
    use gpui::{
        AppContext, EntityInputHandler, Keystroke, MouseButton, MouseDownEvent, MouseMoveEvent,
        MouseUpEvent, PlatformInput, ScrollDelta, ScrollEvent, WindowOptions, point, px,
    };
    use liora_components::{Terminal, TerminalEvent, TerminalSize, init_liora};
    use std::{cell::RefCell, rc::Rc, time::Duration};

    gpui_platform::headless().run(|cx| {
        init_liora(cx);
        let terminal = cx.new(|cx| Terminal::new(TerminalSize::new(3, 20).unwrap(), 20, cx));
        let events = Rc::new(RefCell::new(Vec::<TerminalEvent>::new()));
        let subscription = cx.subscribe(&terminal, {
            let events = events.clone();
            move |_, event, _| events.borrow_mut().push(event.clone())
        });
        let handle = cx.open_window(WindowOptions { show: false, ..Default::default() }, |_, _| terminal.clone()).unwrap();
        let executor = cx.background_executor().clone();
        cx.spawn(async move |cx| {
            executor.timer(Duration::from_millis(10)).await;
            cx.update(|cx| {
                cx.update_window(handle.into(), |_, window, cx| {
                    terminal.update(cx, |terminal, cx| {
                        terminal.process_output(b"abcdef\rXY", cx);
                        assert_eq!(terminal.visible_text(), "XYcdef");
                        terminal.process_output(b"\x1b[2J\x1b[Hhello", cx);
                        terminal.process_output(b"\x1b[?1049halternate", cx);
                        assert!(terminal.is_alternate_screen());
                        terminal.process_output(b"\x1b[?1049l", cx);
                        assert!(!terminal.is_alternate_screen());
                        assert_eq!(terminal.visible_text(), "hello");
                    });
                    window.draw(cx).clear(cx);
                    let start = point(px(1.), px(5.));
                    let end = point(px(35.), px(5.));
                    window.dispatch_event(PlatformInput::MouseDown(MouseDownEvent {
                        button: MouseButton::Left, position: start, click_count: 1, ..Default::default()
                    }), cx);
                    assert!(terminal.read(cx).focus_handle(cx).is_focused(window), "clicking the terminal focuses its native text input handler");
                    window.draw(cx).clear(cx);
                    terminal.update(cx, |terminal, cx| {
                        let caret = terminal.bounds_for_range(0..0, gpui::Bounds::default(), window, cx).unwrap();
                        assert_eq!(caret.origin, point(px(42.), px(0.)), "native input geometry starts at the visible terminal viewport");
                    });
                    window.dispatch_event(PlatformInput::MouseMove(MouseMoveEvent {
                        position: end, pressed_button: Some(MouseButton::Left), ..Default::default()
                    }), cx);
                    window.dispatch_event(PlatformInput::MouseUp(MouseUpEvent {
                        button: MouseButton::Left, position: end, click_count: 1, ..Default::default()
                    }), cx);
                    assert_eq!(terminal.read(cx).selected_text().as_deref(), Some("hello"));
                    // Headless clipboard writes are no-ops; assert the actual selected/copy payload, not OS readback.
                    terminal.update(cx, |terminal, cx| assert_eq!(terminal.copy_selection(cx).as_deref(), Some("hello")));
                    window.dispatch_keystroke(Keystroke::parse("ctrl-c").unwrap(), cx);
                    window.dispatch_keystroke(Keystroke::parse("left").unwrap(), cx);
                    terminal.update(cx, |terminal, cx| {
                        terminal.replace_and_mark_text_in_range(None, "a😀z", Some(3..3), window, cx);
                        terminal.replace_and_mark_text_in_range(Some(1..3), "候选", Some(1..1), window, cx);
                        assert_eq!(terminal.text_for_range(0..4, &mut None, window, cx).as_deref(), Some("a候选z"));
                        assert_eq!(terminal.selected_text_range(false, window, cx).unwrap().range, 2..2);
                        terminal.replace_text_in_range(Some(2..3), "定", window, cx);
                        assert!(terminal.marked_text_range(window, cx).is_none());
                        terminal.replace_text_in_range(None, "猫", window, cx);
                        terminal.process_output(b"\x1b[?1h", cx);
                    });
                    window.dispatch_keystroke(Keystroke::parse("up").unwrap(), cx);
                    terminal.update(cx, |terminal, cx| {
                        terminal.process_output(b"\x1b[2J\x1b[Hone\r\ntwo\r\nthree\r\nfour", cx);
                        assert_eq!(terminal.scrollback_len(), 1);
                    });
                    window.draw(cx).clear(cx);
                    window.dispatch_event(PlatformInput::Scroll(ScrollEvent {
                        position: start, delta: ScrollDelta::Lines(point(0., 1.)), ..Default::default()
                    }), cx);
                    assert_eq!(terminal.read(cx).scrollback_offset(), 1);
                    assert!(terminal.read(cx).visible_text().starts_with("one"));
                    terminal.update(cx, |terminal, cx| {
                        let next = TerminalSize::new(4, 24).unwrap();
                        terminal.resize(next, cx);
                        terminal.resize(next, cx);
                        assert_eq!(terminal.size(), next);
                    });
                    window.draw(cx).clear(cx);
                    terminal.update(cx, |terminal, cx| {
                        terminal.resize(TerminalSize::new(24, 80).unwrap(), cx);
                        terminal.process_output("\x1b[2J\x1b[HASCII 猫 e\u{301} 😀\r\n\x1b[31mred\x1b[0m".as_bytes(), cx);
                    });
                    window.draw(cx).clear(cx);
                }).unwrap();
            });
            cx.update(|cx| {
                let events = events.borrow();
                let inputs: Vec<_> = events.iter().filter_map(|event| match event {
                    TerminalEvent::InputBytes(bytes) => Some(bytes.as_slice()), _ => None,
                }).collect();
                assert_eq!(inputs, vec![b"\x03".as_slice(), b"\x1b[D".as_slice(), "a候定z".as_bytes(), "猫".as_bytes(), b"\x1bOA".as_slice()]);
                assert_eq!(events.iter().filter(|event| matches!(event, TerminalEvent::Resized(_))).count(), 2);
                drop(subscription);
                println!("PASS: native terminal VT state, pointer selection/copy payload, keyboard bytes, scrollback and resize (OS clipboard not tested)");
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
