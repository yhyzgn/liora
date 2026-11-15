//! cargo run -p liora-gallery --example pdf_reader_smoke --locked

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn main() {
    use gpui::{
        AppContext, Context, Entity, Focusable, IntoElement, Keystroke, PlatformInput, Render,
        ScrollDelta, ScrollEvent, TouchPhase, Window, WindowOptions, point, px,
    };
    use liora_components::{
        PdfDocument, PdfPageLayout, PdfReader, PdfReaderEvent, PdfTool, PdfZoom, init_liora,
    };
    use std::{cell::RefCell, rc::Rc, time::Duration};

    struct SmokeView {
        reader: Entity<PdfReader>,
    }

    impl Render for SmokeView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            self.reader.clone()
        }
    }

    gpui_platform::headless().run(|cx| {
        init_liora(cx);
        let document = PdfDocument::from_bytes("PDF smoke", include_bytes!("../../../assets/samples/reader.pdf").to_vec()).expect("real PDF decodes");
        assert_eq!(document.page_count(), 2);
        let first_image = document.pages[0].image.as_ref().unwrap().clone();
        let reader = cx.new(|cx| PdfReader::new(document, cx));
        let events = Rc::new(RefCell::new(Vec::<PdfReaderEvent>::new()));
        let subscription = cx.subscribe(&reader, {
            let events = events.clone();
            move |_, event, _| events.borrow_mut().push(event.clone())
        });
        let view = cx.new(|_| SmokeView {
            reader: reader.clone(),
        });
        let handle = cx
            .open_window(WindowOptions { show: false, ..Default::default() }, |_, _| view)
            .expect("native headless window should open");
        let executor = cx.background_executor().clone();
        cx.spawn(async move |cx| {
            for step in 0..8 {
                executor.timer(Duration::from_millis(20)).await;
                cx.update(|cx| {
                    cx.update_window(handle.into(), |_, window, cx| {
                        window.refresh();
                        window.draw(cx).clear(cx);
                        match step {
                            0 => {
                                window.resize(gpui::size(px(800.0), px(600.0)));
                                window.bounds_changed(cx);
                                let focus = reader.read(cx).focus_handle(cx);
                                window.focus(&focus, cx);
                                reader.update(cx, |reader, cx| {
                                    assert!(!reader.set_page_image(0, first_image.clone(), cx));
                                    reader.set_zoom(PdfZoom::Percent(0), cx);
                                    assert_eq!(reader.zoom(), PdfZoom::Percent(25));
                                    reader.set_zoom(PdfZoom::Percent(u16::MAX), cx);
                                    assert_eq!(reader.zoom(), PdfZoom::Percent(400));
                                    reader.set_zoom(PdfZoom::FitPage, cx);
                                });
                            }
                            1 => {
                                window.dispatch_keystroke(Keystroke::parse("right").unwrap(), cx);
                                assert_eq!(reader.read(cx).current_page(), 2);
                                reader.update(cx, |reader, cx| {
                                    reader.rotate(cx);
                                    reader.set_layout(PdfPageLayout::Continuous, cx);
                                    reader.set_zoom(PdfZoom::Percent(200), cx);
                                    reader.set_tool(PdfTool::Pen, cx);
                                });
                            }
                            2 => {
                                assert_eq!(reader.read(cx).current_page(), 2, "continuous navigation reveals page two");
                                reader.update(cx, |reader, cx| reader.set_page(1, cx));
                            }
                            3 => {
                                assert_eq!(reader.read(cx).current_page(), 1, "continuous navigation returns to page one");
                                window.dispatch_event(PlatformInput::Scroll(ScrollEvent {
                                    position: point(px(500.0), px(400.0)),
                                    delta: ScrollDelta::Pixels(point(px(0.0), px(-1000.0))),
                                    touch_phase: TouchPhase::Moved,
                                    ..Default::default()
                                }), cx);
                            }
                            4 => assert_eq!(reader.read(cx).current_page(), 2, "manual scrolling updates current page"),
                            5 => reader.update(cx, |reader, cx| reader.toggle_find(window, cx)),
                            6 => {
                                let bar = reader.read(cx).find_bar();
                                assert_eq!(window.focused(cx), Some(bar.read(cx).focus_handle(cx)));
                                window.dispatch_keystroke(Keystroke::parse("enter").unwrap(), cx);
                            }
                            7 => {
                                let events = events.borrow();
                                assert!(events.iter().any(|event| matches!(event, PdfReaderEvent::PageChanged { page: 2 })));
                                assert!(events.iter().any(|event| matches!(event, PdfReaderEvent::RotationChanged { .. })));
                                assert!(events.iter().any(|event| matches!(event, PdfReaderEvent::ToolChanged { tool: PdfTool::Pen })));
                                assert!(events.iter().any(|event| matches!(event, PdfReaderEvent::SearchNext)));
                                println!("PASS: real PDF decoding/drawing, zoom bounds, page keyboard navigation, continuous reveal/manual scroll, rotation draw and focused Find Enter routing");
                                cx.quit();
                            }
                            _ => unreachable!(),
                        }
                        window.refresh();
                        window.draw(cx).clear(cx);
                    }).expect("native PDF reader scenario");
                });
            }
            drop(subscription);
        })
        .detach();
    });
}

#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
fn main() {
    eprintln!("This smoke requires the official Linux/FreeBSD headless backend.");
    std::process::exit(1);
}
