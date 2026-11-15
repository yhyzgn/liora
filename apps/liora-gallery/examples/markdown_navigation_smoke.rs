//! Headless native Markdown TOC-to-scroll-target interaction.
//! Run: cargo run -p liora-gallery --example markdown_navigation_smoke --locked
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn main() {
    use gpui::{
        AppContext, Context, Entity, EntityInputHandler, IntoElement, Keystroke, Render,
        ScrollHandle, Window, WindowOptions,
    };
    use liora_components::{
        Flex, MarkdownEditor, MarkdownNavigationRequested, MarkdownView, init_liora,
    };
    use std::{cell::RefCell, rc::Rc};

    struct Surface {
        view: Entity<MarkdownView>,
        scroll: ScrollHandle,
    }
    impl Render for Surface {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            Flex::new()
                .height_units(140.0)
                .overflow_y_scroll()
                .track_scroll(&self.scroll)
                .child(self.view.clone())
        }
    }

    gpui_platform::headless().run(|cx| {
        init_liora(cx);
        let scroll = ScrollHandle::new();
        let mut markdown = String::from("# First\n\n");
        for _ in 0..30 {
            markdown.push_str("Paragraph padding keeps the second heading below the viewport.\n\n");
        }
        markdown.push_str("# Target\n\nDestination.\n");
        let view = cx.new(|cx| {
            let mut view = MarkdownView::new(markdown, cx);
            view.set_show_toc(true, cx);
            view.set_navigation_scroll(scroll.clone(), cx);
            view
        });
        let events = Rc::new(RefCell::new(Vec::new()));
        let _subscription = cx.subscribe(&view, {
            let events = events.clone();
            move |_, event: &MarkdownNavigationRequested, _| {
                events.borrow_mut().push(event.fragment.clone())
            }
        });
        let window = cx
            .open_window(
                WindowOptions {
                    show: false,
                    focus: false,
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|_| Surface {
                        view: view.clone(),
                        scroll: scroll.clone(),
                    })
                },
            )
            .unwrap();
        cx.update_window(window.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
            window.blur(cx);
            window.focus_next(cx); // selection toggle
            window.focus_next(cx); // first TOC entry
            window.focus_next(cx); // second TOC entry
            window.dispatch_keystroke(Keystroke::parse("enter").unwrap(), cx);
            assert!(
                scroll.offset().y < gpui::px(0.0),
                "TOC activation must scroll to the painted target"
            );
        })
        .unwrap();
        assert_eq!(
            events.borrow().as_slice(),
            &[gpui::SharedString::from("target")]
        );
        let editor = cx.new(|cx| MarkdownEditor::new("old", cx));
        let input = editor.read(cx).input();
        cx.update_window(window.into(), |_, window, cx| {
            input.update(cx, |input, cx| {
                input.replace_and_mark_text_in_range(Some(0..3), "候选", None, window, cx);
                assert!(input.is_composing());
            });
        })
        .unwrap();
        assert_eq!(editor.read(cx).source(), "old");
        cx.update_window(window.into(), |_, window, cx| {
            input.update(cx, |input, cx| input.unmark_text(window, cx));
        })
        .unwrap();
        assert_eq!(editor.read(cx).source(), "候选");
        editor.update(cx, |editor, cx| {
            editor.begin_undo_group();
            editor.set_source("候选!", cx);
            editor.set_source("候选!!", cx);
            editor.end_undo_group();
            assert!(editor.undo(cx));
            assert_eq!(editor.source(), "候选");
            assert!(editor.redo(cx));
            assert_eq!(editor.source(), "候选!!");
        });
        println!("PASS: native Markdown TOC scrolls to target and emits host event");
        let executor = cx.background_executor().clone();
        cx.spawn(async move |cx| {
            executor.timer(std::time::Duration::from_millis(10)).await;
            cx.update(|cx| cx.quit());
        })
        .detach();
    });
}

#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
fn main() {
    eprintln!("markdown_navigation_smoke requires the GPUI Linux/FreeBSD headless backend");
}
