//! Real native splitter and dock input regression; no display server required.
//! Run: cargo run -p liora-gallery --example layout_smoke --locked
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn main() {
    use gpui::{
        App, AppContext, Context, IntoElement, KeyDownEvent, Keystroke, MouseButton,
        MouseDownEvent, MouseMoveEvent, MouseUpEvent, PlatformInput, Render, Window, WindowOptions,
        div, point, prelude::*, px,
    };
    use liora_components::dock_layout::DockState;
    use liora_components::{DockEdge, DockLayout, DockPanel, DockTab, Splitter, init_liora};
    use std::{
        cell::{Cell, RefCell},
        rc::Rc,
    };

    type Measurements =
        Rc<RefCell<std::collections::HashMap<&'static str, gpui::Bounds<gpui::Pixels>>>>;

    fn measured(
        key: &'static str,
        bounds: &Measurements,
        content: impl IntoElement,
    ) -> impl IntoElement {
        let bounds = bounds.clone();
        div().relative().size_full().child(content).child(
            gpui::canvas(
                move |value, _, _| {
                    bounds.borrow_mut().insert(key, value);
                },
                |_, _, _, _| {},
            )
            .absolute()
            .top_0()
            .left_0()
            .size_full(),
        )
    }

    struct SplitCases {
        width: f32,
        sizes: [Rc<Cell<gpui::Pixels>>; 2],
        requests: [Rc<RefCell<Vec<gpui::Pixels>>>; 2],
        bounds: Measurements,
    }
    impl Render for SplitCases {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let mut root = div().flex().flex_col();
            for (index, (id, first, second)) in [
                ("split-case-a", "first-a", "second-a"),
                ("split-case-b", "first-b", "second-b"),
            ]
            .into_iter()
            .enumerate()
            {
                let value = self.sizes[index].clone();
                let requests = self.requests[index].clone();
                root = root.child(
                    div().w(px(self.width)).h(px(120.)).flex_none().child(
                        Splitter::new()
                            .id(id)
                            .size(value.get())
                            .left(measured(first, &self.bounds, div()))
                            .right(measured(second, &self.bounds, div()))
                            .on_change(move |size| {
                                value.set(size);
                                requests.borrow_mut().push(size);
                            }),
                    ),
                );
            }
            root
        }
    }

    struct SplitView {
        changes: Rc<Cell<gpui::Pixels>>,
    }
    impl Render for SplitView {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let changes = self.changes.clone();
            Splitter::new()
                .id("smoke-split")
                .default_size(px(200.))
                .min_size(px(100.))
                .max_size(px(350.))
                .left(div().child("First"))
                .right(div().child("Second"))
                .on_change(move |value| changes.set(value))
        }
    }
    struct DockView {
        changes: Rc<RefCell<Option<DockState>>>,
        center_painted: Rc<Cell<bool>>,
        body: gpui::Entity<Body>,
        id: &'static str,
        restored: Option<DockState>,
        bounds: Measurements,
    }
    struct Body {
        edits: usize,
        painted_edits: Rc<Cell<usize>>,
    }
    impl Render for Body {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            self.painted_edits.set(self.edits);
            div().child(format!("Host edits: {}", self.edits))
        }
    }
    impl Render for DockView {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let changes = self.changes.clone();
            let painted = self.center_painted.clone();
            let bounds = self.bounds.clone();
            let panel_bounds = bounds.clone();
            let mut layout = DockLayout::new().id(self.id);
            if let Some(restored) = &self.restored {
                layout = layout.layout_state(restored.clone());
            }
            layout
                .tab(DockTab::new(
                    "a",
                    "A",
                    measured("a", &bounds, self.body.clone()),
                ))
                .tab(DockTab::new(
                    "b",
                    "B",
                    measured("b", &bounds, div().child("Second document")),
                ))
                .panel(
                    DockPanel::new(
                        "inspector",
                        "Inspector",
                        DockEdge::Center,
                        gpui::canvas(
                            move |bounds, _, _| {
                                panel_bounds.borrow_mut().insert("panel", bounds);
                                painted
                                    .set(bounds.size.width > px(0.) && bounds.size.height > px(0.))
                            },
                            |_, _, _, _| {},
                        )
                        .size_full(),
                    )
                    .size(px(100.)),
                )
                .on_layout_change(move |state| *changes.borrow_mut() = Some(state))
        }
    }
    fn click(window: &mut Window, cx: &mut App, x: f32, y: f32) {
        let position = point(px(x), px(y));
        window.dispatch_event(
            PlatformInput::MouseDown(MouseDownEvent {
                button: MouseButton::Left,
                position,
                click_count: 1,
                ..Default::default()
            }),
            cx,
        );
        window.dispatch_event(
            PlatformInput::MouseUp(MouseUpEvent {
                button: MouseButton::Left,
                position,
                click_count: 1,
                ..Default::default()
            }),
            cx,
        );
    }
    fn drag(window: &mut Window, cx: &mut App, start: (f32, f32), end: (f32, f32)) {
        let position = point(px(start.0), px(start.1));
        window.dispatch_event(
            PlatformInput::MouseDown(MouseDownEvent {
                button: MouseButton::Left,
                position,
                click_count: 1,
                ..Default::default()
            }),
            cx,
        );
        for position in [
            point(px(start.0 + 12.), px(start.1)),
            point(px(end.0), px(end.1)),
        ] {
            window.dispatch_event(
                PlatformInput::MouseMove(MouseMoveEvent {
                    position,
                    pressed_button: Some(MouseButton::Left),
                    ..Default::default()
                }),
                cx,
            );
            window.draw(cx).clear(cx);
        }
        window.dispatch_event(
            PlatformInput::MouseUp(MouseUpEvent {
                button: MouseButton::Left,
                position: point(px(end.0), px(end.1)),
                click_count: 1,
                ..Default::default()
            }),
            cx,
        );
        window.draw(cx).clear(cx);
    }
    struct TwoDocks([gpui::Entity<DockView>; 2]);
    impl Render for TwoDocks {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .flex()
                .flex_col()
                .w_full()
                .child(
                    div()
                        .h(px(240.))
                        .w_full()
                        .flex_none()
                        .child(self.0[0].clone()),
                )
                .child(
                    div()
                        .h(px(240.))
                        .w_full()
                        .flex_none()
                        .child(self.0[1].clone()),
                )
        }
    }

    fn key(window: &mut Window, cx: &mut App, key: &'static str) {
        window.dispatch_event(
            PlatformInput::KeyDown(KeyDownEvent {
                keystroke: Keystroke::parse(key).unwrap(),
                is_held: false,
                prefer_character_input: false,
            }),
            cx,
        );
    }

    fn assert_split_geometry(bounds: &Measurements, first_width: f32, total: f32) {
        let bounds = bounds.borrow();
        let first = bounds["first-a"];
        let second = bounds["second-a"];
        assert_eq!(
            first.size.width,
            px(first_width),
            "rendered first pane converges to current container"
        );
        assert_eq!(
            second.origin.x - first.right(),
            px(6.),
            "actual divider occupies six pixels"
        );
        assert_eq!(
            second.size.width,
            px(total - first_width - 6.),
            "rendered second pane keeps its reserved space"
        );
        assert_eq!(
            second.right() - first.origin.x,
            px(total),
            "panes and divider fit current bounds"
        );
    }

    fn run_split_cases(cx: &mut App) {
        let bounds = Measurements::default();
        let sizes = [Rc::new(Cell::new(px(500.))), Rc::new(Cell::new(px(200.)))];
        let requests = [
            Rc::new(RefCell::new(Vec::new())),
            Rc::new(RefCell::new(Vec::new())),
        ];
        let view = cx.new(|_| SplitCases {
            width: 300.,
            sizes: sizes.clone(),
            requests: requests.clone(),
            bounds: bounds.clone(),
        });
        let handle = cx
            .open_window(
                WindowOptions {
                    show: false,
                    ..Default::default()
                },
                |_, _| view.clone(),
            )
            .unwrap();
        cx.update_window(handle.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
        })
        .unwrap();
        // Let the measurement correction run without pointer, keyboard or host invalidation.
        cx.defer(move |cx| {
            cx.update_window(handle.into(), |_, window, cx| { window.draw(cx).clear(cx); }).unwrap();
            assert_split_geometry(&bounds, 214., 300.);
            assert!(requests.iter().all(|requests| requests.borrow().is_empty()), "measurement never writes controlled host size");
            view.update(cx, |view, cx| { view.width = 260.; cx.notify(); });
            cx.update_window(handle.into(), |_, window, cx| { window.refresh(); window.draw(cx).clear(cx); }).unwrap();
            cx.defer(move |cx| {
                cx.update_window(handle.into(), |_, window, cx| { window.draw(cx).clear(cx); }).unwrap();
                assert_split_geometry(&bounds, 174., 260.);
                view.update(cx, |view, cx| { view.width = 600.; view.sizes[0].set(px(200.)); cx.notify(); });
                cx.update_window(handle.into(), |_, window, cx| { window.refresh(); window.draw(cx).clear(cx); }).unwrap();
                cx.defer(move |cx| {
                    cx.update_window(handle.into(), |_, window, cx| {
                        window.draw(cx).clear(cx);
                        assert_split_geometry(&bounds, 200., 600.);
                        click(window, cx, 203., 40.);
                        key(window, cx, "right");
                        window.draw(cx).clear(cx);
                        assert_eq!(&*requests[0].borrow(), &[px(210.)], "controlled host accepts keyboard request");
                    }).unwrap();
                    view.update(cx, |view, cx| { view.sizes[0].set(px(200.)); cx.notify(); });
                    cx.update_window(handle.into(), |_, window, cx| {
                        window.refresh();
                        window.draw(cx).clear(cx);
                        click(window, cx, 203., 40.);
                        key(window, cx, "right");
                        assert_eq!(&*requests[0].borrow(), &[px(210.), px(210.)], "external reset permits the same controlled request again");
                        window.draw(cx).clear(cx);
                        let before = bounds.borrow()["first-b"];
                        drag(window, cx, (213., 40.), (303., 40.));
                        assert_eq!(sizes[0].get(), px(300.));
                        assert_eq!(sizes[1].get(), px(200.), "foreign capture listener leaves second splitter state unchanged");
                        assert!(requests[1].borrow().is_empty(), "foreign capture listener never invokes other callback");
                        assert_eq!(bounds.borrow()["first-b"], before, "other rendered pane remains unchanged");
                    }).unwrap();
                    println!("layout_smoke: native resize isolation, measured bounds, controlled resets, dock isolation and fresh rendered restore completed");
                    let executor = cx.background_executor().clone();
                    cx.spawn(async move |cx| {
                        executor.timer(std::time::Duration::from_millis(10)).await;
                        cx.update(|cx| cx.quit());
                    }).detach();
                });
            });
        });
    }

    gpui_platform::headless().run(|cx| {
        init_liora(cx);
        let changes = Rc::new(Cell::new(px(200.)));
        let split = cx.new(|_| SplitView { changes: changes.clone() });
        let window = cx.open_window(WindowOptions { show: false, ..Default::default() }, |_, _| split).unwrap();
        cx.update_window(window.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
            click(window, cx, 203., 40.);
            window.dispatch_event(PlatformInput::KeyDown(KeyDownEvent { keystroke: Keystroke::parse("right").unwrap(), is_held: false, prefer_character_input: false }), cx);
            assert_eq!(changes.get(), px(210.), "divider keyboard resizes");
            window.draw(cx).clear(cx);
            drag(window, cx, (213., 40.), (303., 40.));
            assert_eq!(changes.get(), px(300.), "real native drag resizes");
            drag(window, cx, (303., 40.), (999., 40.));
            assert_eq!(changes.get(), px(350.), "native drag respects max bound");
            window.draw(cx).clear(cx);
            click(window, cx, 353., 40.);
            window.dispatch_event(PlatformInput::KeyDown(KeyDownEvent { keystroke: Keystroke::parse("home").unwrap(), is_held: false, prefer_character_input: false }), cx);
            assert_eq!(changes.get(), px(100.), "keyboard sees persistent post-drag size");
        }).unwrap();
        let changes = Rc::new(RefCell::new(None::<DockState>));
        let center_painted = Rc::new(Cell::new(false));
        let painted_edits = Rc::new(Cell::new(0));
        let body = cx.new(|_| Body { edits: 7, painted_edits: painted_edits.clone() });
        let bounds = Measurements::default();
        let saved_layout = Rc::new(RefCell::new(None::<DockState>));
        let saved_bounds = Measurements::default();
        let dock = cx.new(|_| DockView { changes: changes.clone(), center_painted: center_painted.clone(), body: body.clone(), id: "smoke-dock", restored: None, bounds: bounds.clone() });
        let window = cx.open_window(WindowOptions { show: false, ..Default::default() }, |_, _| dock).unwrap();
        cx.update_window(window.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
            assert!(center_painted.get(), "Center panels receive real layout bounds");
            click(window, cx, 165., 18.);
            assert_eq!(changes.borrow().as_ref().unwrap().active_tab(), Some("b"));
            window.draw(cx).clear(cx);
            drag(window, cx, (165., 18.), (35., 18.));
            let reordered = changes.borrow().as_ref().unwrap().to_json().unwrap();
            assert!(reordered.contains("\"tabs\":[\"b\",\"a\"]"));
            let right = window.viewport_size().width / px(1.) - 100.;
            drag(window, cx, (35., 18.), (right - 32., 18.));
            assert_eq!(changes.borrow().as_ref().unwrap().group_count(), 2, "native drop splits a group");
            let saved = changes.borrow().as_ref().unwrap().to_json().unwrap();
            assert_eq!(DockState::from_json(&saved).unwrap(), *changes.borrow().as_ref().unwrap());
            *saved_layout.borrow_mut() = Some(DockState::from_json(&saved).unwrap());
            *saved_bounds.borrow_mut() = bounds.borrow().clone();
            assert_eq!(body.read(cx).edits, 7, "host entity survives tab moves and redraws");
            let bottom = window.viewport_size().height / px(1.) * 0.75;
            drag(window, cx, (35., 18.), (40., bottom));
            assert_eq!(changes.borrow().as_ref().unwrap().group_count(), 1, "native cross-group move collapses emptied source");
            click(window, cx, 222., 18.);
            assert!(!changes.borrow().as_ref().unwrap().contains_tab("a"), "native close removes the tab");
        }).unwrap();
        let restored = saved_layout.borrow().clone().unwrap();
        assert_eq!(restored.group_count(), 2);
        let restored_changes = Rc::new(RefCell::new(None::<DockState>));
        let restored_bounds = Measurements::default();
        let restored_edits = Rc::new(Cell::new(0));
        let restored_body = cx.new(|_| Body { edits: 19, painted_edits: restored_edits.clone() });
        let restored_dock = cx.new(|_| DockView {
            id: "fresh-restored-dock", restored: Some(restored.clone()),
            changes: restored_changes.clone(), center_painted: Rc::new(Cell::new(false)),
            body: restored_body.clone(), bounds: restored_bounds.clone(),
        });
        let restored_window = cx.open_window(WindowOptions { show: false, ..Default::default() }, |_, _| restored_dock).unwrap();
        cx.update_window(restored_window.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
            assert_eq!(restored_edits.get(), 19, "fresh restored layout renders newly supplied host entity content");
            for key in ["a", "b", "panel"] {
                assert_eq!(restored_bounds.borrow()[key], saved_bounds.borrow()[key], "fresh restored group and panel geometry matches saved layout: {key}");
            }
            let a = restored_bounds.borrow()["a"];
            let b = restored_bounds.borrow()["b"];
            let panel = restored_bounds.borrow()["panel"];
            assert!(a.size.width > px(0.) && a.size.height > px(0.));
            assert!(b.size.width > px(0.) && b.size.height > px(0.));
            assert_ne!(a.origin, b.origin, "restored active host contents occupy distinct groups");
            assert_eq!(panel.size.width, px(74.), "restored 100px panel preserves border and content padding geometry");
            assert!(panel.origin.x > a.origin.x && panel.origin.x > b.origin.x);
            assert!(restored_changes.borrow().is_none(), "restoration does not emit a synthetic user change");
            drag(window, cx, (a.origin.x.as_f32() + 20., a.origin.y.as_f32() - 28.),
                (b.origin.x.as_f32() + 20., b.origin.y.as_f32() + 20.));
            {
                let snapshot = restored_changes.borrow();
                let changed = snapshot.as_ref().expect("restored native cross-group drop emits layout");
                assert_eq!(changed.group_count(), 1, "restored groups remain natively movable");
                assert!(changed.contains_tab("a") && changed.contains_tab("b"));
            }
            window.draw(cx).clear(cx);
            click(window, cx, 165., 18.);
            assert_eq!(restored_changes.borrow().as_ref().unwrap().active_tab(), Some("a"), "restored tabs remain selectable");
            window.draw(cx).clear(cx);
            click(window, cx, 222., 18.);
            assert!(!restored_changes.borrow().as_ref().unwrap().contains_tab("a"), "restored tabs remain closeable");
            assert_eq!(restored_body.read(cx).edits, 19, "native restored interactions preserve host-owned state");
        }).unwrap();

        let foreign_changes = [Rc::new(RefCell::new(None::<DockState>)), Rc::new(RefCell::new(None::<DockState>))];
        let foreign_bounds = [Measurements::default(), Measurements::default()];
        let docks = ["isolated-dock-a", "isolated-dock-b"].map(|id| {
            let index = usize::from(id == "isolated-dock-b");
            cx.new(|_| DockView {
                id, restored: None, changes: foreign_changes[index].clone(),
                center_painted: Rc::new(Cell::new(false)), body: body.clone(), bounds: foreign_bounds[index].clone(),
            })
        });
        let pair = cx.new(|_| TwoDocks(docks));
        let pair_window = cx.open_window(WindowOptions { show: false, ..Default::default() }, |_, _| pair).unwrap();
        cx.update_window(pair_window.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
            let before = [foreign_bounds[0].borrow().clone(), foreign_bounds[1].borrow().clone()];
            drag(window, cx, (35., 18.), (60., 400.));
            assert!(foreign_changes.iter().all(|changes| changes.borrow().is_none()), "foreign dock tab drop cannot mutate either layout or invoke callbacks despite shared tab keys");
            for index in 0..2 {
                assert_eq!(*foreign_bounds[index].borrow(), before[index], "foreign drop leaves rendered host geometry unchanged");
            }
            click(window, cx, 165., 258.);
            assert_eq!(foreign_changes[1].borrow().as_ref().unwrap().active_tab(), Some("b"), "target dock remains independently interactive");
            assert!(foreign_changes[0].borrow().is_none());
        }).unwrap();
        cx.spawn(async move |cx| cx.update(run_split_cases)).detach();
    });
}
#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
fn main() {
    eprintln!("layout_smoke requires the official Linux/FreeBSD headless backend");
    std::process::exit(1);
}
