//! Native cache, image policy, streaming and continuous-selection smoke.
//! Run: cargo run -p liora-gallery --example markdown_smoke --locked
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn main() {
    use gpui::{
        AppContext, Keystroke, MouseButton, MouseDownEvent, MouseUpEvent, PlatformInput,
        WindowOptions, point, px,
    };
    use liora_components::{MarkdownImage, MarkdownView, init_liora};
    use std::{cell::Cell, rc::Rc};

    gpui_platform::headless().run(|cx| {
        init_liora(cx);
        let resolutions = Rc::new(Cell::new(0));
        let view = cx.new(|cx| MarkdownView::new("# Title\n\nFirst **paragraph**.\n\n![first](private-path) ![second](private-path)\n\n| Left | Center | Right |\n| :--- | :---: | ---: |\n| L | C | R |\n\n```rust\nlet value", cx));
        let ids = view.read(cx).document().block_ids().collect::<Vec<_>>();
        let window = cx.open_window(WindowOptions { show: false, focus: false, ..Default::default() }, |_, _| view.clone()).unwrap();
        cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx)).unwrap();
        view.update(cx, |view, cx| {
            view.append(" = ", cx);
            view.append("42;", cx);
            view.append("\n```\n\nFinal paragraph.", cx);
            view.append("\n\n[Safe](https://example.com) [Blocked](javascript:alert%281%29)", cx);
            assert_eq!(view.document().parse_count(), 1, "append must not parse per token");
        });
        cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx)).unwrap();
        assert_eq!(view.read(cx).document().parse_count(), 2);
        assert_eq!(view.read(cx).document().block_ids().take(ids.len()).collect::<Vec<_>>(), ids);
        cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx)).unwrap();
        assert_eq!(view.read(cx).document().parse_count(), 2, "clean renders reuse the parse cache");

        view.update(cx, |view, cx| {
            let resolutions = resolutions.clone();
            view.set_image_resolver(move |destination| {
                resolutions.set(resolutions.get() + 1);
                assert_eq!(destination, "private-path");
                let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/local.jpeg");
                assert!(fixture.is_file(), "native image smoke requires the real Gallery fixture");
                Some(MarkdownImage::File(fixture))
            }, cx);
        });
        for _ in 0..2 {
            cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx)).unwrap();
        }
        assert_eq!(resolutions.get(), 1, "policy decisions are cached across renders");
        view.update(cx, |view, cx| view.clear_image_resolver(cx));
        cx.update_window(window.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
            window.blur(cx);
            window.focus_next(cx);
            let toggle_focus = window.focused(cx).expect("selection control is a native tab stop");
            window.dispatch_keystroke(Keystroke::parse("enter").unwrap(), cx);
            assert!(view.read(cx).continuous_selection(), "Enter must activate the focused toggle");
            window.draw(cx).clear(cx);
            assert!(toggle_focus.is_focused(window), "toggle focus survives rerender");
            window.dispatch_keystroke(Keystroke::parse("space").unwrap(), cx);
            assert!(!view.read(cx).continuous_selection(), "Space must activate the focused toggle");
            window.draw(cx).clear(cx);
            window.focus_next(cx);
            let link_focus = window.focused(cx).expect("safe Open URL control is a native tab stop");
            assert!(!toggle_focus.is_focused(window));
            window.dispatch_keystroke(Keystroke::parse("enter").unwrap(), cx);
            window.dispatch_keystroke(Keystroke::parse("space").unwrap(), cx);
            window.draw(cx).clear(cx);
            assert!(link_focus.is_focused(window), "Open URL focus survives rerender");
            window.focus_next(cx);
            assert!(toggle_focus.is_focused(window), "unsafe links must not install activation controls");
            // Headless cannot observe external URL launching; only the safe URL is activated.
            // The first native control switches to one continuous selectable document.
            let position = point(px(24.0), px(12.0));
            window.dispatch_event(PlatformInput::MouseDown(MouseDownEvent { button: MouseButton::Left, position, click_count: 1, ..Default::default() }), cx);
            window.dispatch_event(PlatformInput::MouseUp(MouseUpEvent { button: MouseButton::Left, position, click_count: 1, ..Default::default() }), cx);
            assert!(view.read(cx).continuous_selection(), "the visible control must change selection mode");
            window.draw(cx).clear(cx);
            let position = point(px(24.0), px(52.0));
            window.dispatch_event(PlatformInput::MouseDown(MouseDownEvent { button: MouseButton::Left, position, click_count: 1, ..Default::default() }), cx);
            window.dispatch_event(PlatformInput::MouseUp(MouseUpEvent { button: MouseButton::Left, position, click_count: 1, ..Default::default() }), cx);
            window.dispatch_keystroke(Keystroke::parse("ctrl-a").unwrap(), cx);
            window.dispatch_keystroke(Keystroke::parse("ctrl-c").unwrap(), cx);
            // Dispatch only: actual cross-block selection/copy payload is covered by the real-display smoke.
            // Official headless clipboard is a no-op; system copy requires a real display backend.
        }).unwrap();
        assert_eq!(resolutions.get(), 1, "revoking policy must not invoke it again");
        println!("PASS: Markdown native render, batched parse, stable IDs, deny-by-default images, keyboard toggle/focus and selection key dispatch (clipboard unverified)");
        let executor = cx.background_executor().clone();
        cx.spawn(async move |cx| {
            executor.timer(std::time::Duration::from_millis(10)).await;
            cx.update(|cx| cx.quit());
        }).detach();
    });
}

#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
fn main() {
    eprintln!("markdown_smoke requires the GPUI Linux/FreeBSD headless backend");
}
