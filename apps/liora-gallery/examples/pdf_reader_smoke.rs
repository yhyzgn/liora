//! cargo run -p liora-gallery --example pdf_reader_smoke --locked

#[cfg(all(debug_assertions, any(target_os = "linux", target_os = "freebsd")))]
fn check_layout_bounds() {
    use gpui::{AppContext, Bounds, Pixels, TestAppContext, VisualTestContext, px, size};
    use liora_components::{
        PdfDocument, PdfPageLayout, PdfPageSize, PdfReader, PdfZoom, init_liora,
    };

    fn settle(cx: &mut VisualTestContext) {
        for _ in 0..4 {
            cx.update(|window, cx| {
                window.refresh();
                window.draw(cx).clear(cx);
            });
            cx.run_until_parked();
        }
    }

    fn bounds(cx: &mut VisualTestContext, selector: &'static str) -> Bounds<Pixels> {
        cx.debug_bounds(selector).expect(selector)
    }

    fn assert_fit(cx: &mut VisualTestContext, sidebar: bool, fit_width: bool, rotated: bool) {
        let root = bounds(cx, "pdf-reader-root");
        let toolbar = bounds(cx, "pdf-reader-toolbar");
        let viewport = bounds(cx, "pdf-reader-viewport");
        let pages = bounds(cx, "pdf-reader-pages");
        let page = bounds(cx, "pdf-reader-page-1");
        let sidebar_width = if sidebar {
            let sidebar = bounds(cx, "pdf-reader-sidebar");
            assert!((f32::from(sidebar.size.width) - 140.0).abs() < 1.0);
            assert!((sidebar.origin.x - root.origin.x).abs() < px(1.0));
            assert!((sidebar.origin.y - viewport.origin.y).abs() < px(1.0));
            140.0
        } else {
            0.0
        };
        assert!((f32::from(viewport.origin.x - root.origin.x) - sidebar_width).abs() < 1.0);
        assert!((f32::from(viewport.size.width - root.size.width) + sidebar_width).abs() < 1.0);
        assert!((viewport.origin.y - toolbar.bottom()).abs() < px(1.0));
        assert!((viewport.bottom() - root.bottom()).abs() < px(1.0));
        assert!(
            viewport.size.height > root.size.height / 2.0,
            "toolbar must not consume the reader body"
        );
        assert_eq!(
            pages, viewport,
            "scroll viewport must fill the body, not size to its page"
        );
        let width = f32::from(page.size.width) - 2.0;
        let height = f32::from(page.size.height) - 2.0;
        let ratio = if rotated {
            900.0 / 600.0
        } else {
            600.0 / 900.0
        };
        assert!(
            (width / height - ratio).abs() < 0.01,
            "fit must preserve page aspect ratio"
        );
        assert!(
            (page.center().x - viewport.center().x).abs() < px(1.0),
            "page must center in remaining viewport, not at window right"
        );
        assert!(page.origin.x >= viewport.origin.x);
        assert!(page.right() <= viewport.right() + px(1.0));
        if fit_width {
            assert!((page.size.width - (viewport.size.width - px(24.0))).abs() < px(1.0));
        } else {
            assert!(page.bottom() <= viewport.bottom() + px(1.0));
            assert!(
                ((page.size.width + px(24.0)) - viewport.size.width).abs() < px(1.0)
                    || ((page.size.height + px(24.0)) - viewport.size.height).abs() < px(1.0),
                "fit-page must use the largest proportional size that fits"
            );
        }
    }

    let mut app = TestAppContext::single();
    app.update(init_liora);
    let reader = app.new(|cx| {
        PdfReader::new(
            PdfDocument::with_page_count("Layout bounds", 2, PdfPageSize::new(600.0, 900.0)),
            cx,
        )
    });
    let handle = app.open_window(size(px(1568.0), px(718.0)), |_, _| {
        LayoutView(reader.clone())
    });
    let mut cx = VisualTestContext::from_window(handle.into(), &app);
    for (width, height) in [
        (1568.0, 718.0),
        (640.0, 718.0),
        (1568.0, 640.0),
        (1568.0, 900.0),
    ] {
        cx.update(|window, cx| {
            window.resize(size(px(width), px(height)));
            window.bounds_changed(cx);
        });
        settle(&mut cx);
        assert_fit(&mut cx, true, false, false);
        reader.update(&mut cx, |reader, cx| reader.set_zoom(PdfZoom::FitWidth, cx));
        settle(&mut cx);
        assert_fit(&mut cx, true, true, false);
        reader.update(&mut cx, |reader, cx| {
            reader.toggle_sidebar(cx);
            reader.set_zoom(PdfZoom::FitPage, cx);
        });
        settle(&mut cx);
        assert_fit(&mut cx, false, false, false);
        reader.update(&mut cx, |reader, cx| reader.toggle_sidebar(cx));
    }
    reader.update(&mut cx, |reader, cx| reader.rotate(cx));
    settle(&mut cx);
    assert_fit(&mut cx, true, false, true);
    reader.update(&mut cx, |reader, cx| {
        reader.set_zoom(PdfZoom::Percent(200), cx);
        reader.set_layout(PdfPageLayout::Continuous, cx);
    });
    settle(&mut cx);
    let viewport = bounds(&mut cx, "pdf-reader-viewport");
    let first = bounds(&mut cx, "pdf-reader-page-1");
    let first_row = bounds(&mut cx, "pdf-reader-page-row-1");
    let second_row = bounds(&mut cx, "pdf-reader-page-row-2");
    assert!((first.size.width - px(1802.0)).abs() < px(1.0));
    assert!((first.size.height - px(1202.0)).abs() < px(1.0));
    assert!(
        first.origin.x >= viewport.origin.x,
        "zoom overflow must remain scroll-reachable on the left"
    );
    assert!(first_row.size.width >= first.size.width + px(24.0));
    assert!(
        second_row.origin.y >= first_row.bottom() - px(1.0),
        "continuous pages must stack without shrinking"
    );
    reader.update(&mut cx, |reader, cx| reader.set_zoom(PdfZoom::FitWidth, cx));
    settle(&mut cx);
    assert_fit(&mut cx, true, true, true);
    println!(
        "PASS: actual native bounds at wide/narrow resize, sidebar, fit page/width, proportional rotation, manual zoom overflow and continuous stacking"
    );
    app.quit();

    struct LayoutView(gpui::Entity<PdfReader>);
    impl gpui::Render for LayoutView {
        fn render(
            &mut self,
            _: &mut gpui::Window,
            _: &mut gpui::Context<Self>,
        ) -> impl gpui::IntoElement {
            self.0.clone()
        }
    }
}

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

    #[cfg(debug_assertions)]
    check_layout_bounds();

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
