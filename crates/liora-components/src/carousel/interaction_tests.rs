use super::*;
use gpui::{
    Context, InputEvent, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Point, Render,
    TestAppContext, VisualTestContext, WindowHandle, point, size,
};
use std::rc::Weak;

const SLIDES: [&str; 3] = ["slide-a", "slide-b", "slide-c"];

struct Surface {
    mounted: bool,
    count: usize,
    autoplay: bool,
    interval_ms: u64,
    pause_on_hover: bool,
    active_index: Option<usize>,
    accept_changes: bool,
    listen: bool,
    clip_height: Option<Pixels>,
    callback_update: Option<fn(&mut Surface)>,
    changes: Rc<RefCell<Vec<usize>>>,
    callback_lifetime: Weak<()>,
}

impl Render for Surface {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .p_8()
            .when_some(self.clip_height, |root, height| {
                root.h(height).overflow_hidden()
            })
            .when(self.mounted, |root| {
                let view = cx.entity().downgrade();
                let changes = self.changes.clone();
                let lifetime = self.callback_lifetime.upgrade();
                let items = SLIDES[..self.count]
                    .iter()
                    .map(|&slide| {
                        CarouselItem::new(slide)
                            .content(div().h(px(1.)).debug_selector(|| slide.into()))
                    })
                    .collect();
                root.child(
                    Carousel::new(items)
                        .autoplay(self.autoplay)
                        .interval_ms(self.interval_ms)
                        .pause_on_hover(self.pause_on_hover)
                        .indicators_outside()
                        .when_some(self.active_index, Carousel::active_index)
                        .when(self.listen, |carousel| {
                            carousel.on_change(move |index, window, cx| {
                                let _ = &lifetime;
                                changes.borrow_mut().push(index);
                                let redraw = view
                                    .update(cx, |surface, cx| {
                                        if surface.accept_changes {
                                            surface.active_index = Some(index);
                                            cx.notify();
                                        }
                                        if let Some(update) = surface.callback_update {
                                            update(surface);
                                            cx.notify();
                                            true
                                        } else {
                                            false
                                        }
                                    })
                                    .unwrap_or(false);
                                if redraw {
                                    window.draw(cx).clear(cx);
                                }
                            })
                        }),
                )
            })
    }
}

struct Mounted {
    app: TestAppContext,
    handle: WindowHandle<Surface>,
    window: VisualTestContext,
    changes: Rc<RefCell<Vec<usize>>>,
    callback_lifetime: Weak<()>,
}

impl Mounted {
    fn new(configure: impl FnOnce(&mut Surface)) -> Self {
        let mut app = TestAppContext::single();
        app.update(crate::init_liora);
        let changes = Rc::new(RefCell::new(Vec::new()));
        let lifetime = Rc::new(());
        let callback_lifetime = Rc::downgrade(&lifetime);
        let handle = app.open_window(size(px(480.), px(400.)), |_, _| {
            let mut surface = Surface {
                mounted: true,
                count: 3,
                autoplay: true,
                interval_ms: 500,
                pause_on_hover: true,
                active_index: None,
                accept_changes: false,
                listen: true,
                clip_height: None,
                callback_update: None,
                changes: changes.clone(),
                callback_lifetime: callback_lifetime.clone(),
            };
            configure(&mut surface);
            surface
        });
        let window = VisualTestContext::from_window(handle.into(), &app);
        let mut mounted = Self {
            app,
            handle,
            window,
            changes,
            callback_lifetime,
        };
        mounted.draw();
        mounted
    }

    fn draw(&mut self) {
        self.window.update(|window, cx| window.draw(cx).clear(cx));
        self.drain_ready();
    }

    fn advance(&mut self, ms: u64) {
        self.app.executor().advance_clock(Duration::from_millis(ms));
        self.drain_ready();
    }

    fn drain_ready(&self) {
        let executor = self.app.executor();
        for _ in 0..100 {
            if !executor.tick() {
                return;
            }
        }
        panic!("GPUI tasks did not park within 100 ticks");
    }

    fn dispatch(&mut self, event: impl InputEvent) {
        self.window.update(|window, cx| {
            window.dispatch_event(event.to_platform_input(), cx);
        });
        self.drain_ready();
    }

    fn move_mouse(&mut self, position: Point<Pixels>) {
        self.dispatch(MouseMoveEvent {
            position,
            ..Default::default()
        });
    }

    fn update(&mut self, change: impl FnOnce(&mut Surface)) {
        self.handle
            .update(&mut self.app, |surface, _, cx| {
                change(surface);
                cx.notify();
            })
            .unwrap();
        self.draw();
    }

    fn click(&mut self, selector: &'static str) {
        let target = self.window.debug_bounds(selector).unwrap().center();
        self.dispatch(MouseDownEvent {
            position: target,
            button: MouseButton::Left,
            click_count: 1,
            ..Default::default()
        });
        self.dispatch(MouseUpEvent {
            position: target,
            button: MouseButton::Left,
            click_count: 1,
            ..Default::default()
        });
        self.draw();
    }

    fn expect(&mut self, active: Option<usize>, changes: &[usize]) {
        assert_eq!(self.changes.borrow().as_slice(), changes);
        for (index, slide) in SLIDES.into_iter().enumerate() {
            assert_eq!(
                self.window.debug_bounds(slide).is_some(),
                active == Some(index)
            );
        }
    }
}

impl Drop for Mounted {
    fn drop(&mut self) {
        self.app.quit();
    }
}

#[test]
fn content_stays_clear_of_both_navigation_arrows() {
    let mut carousel = Mounted::new(|surface| surface.autoplay = false);
    let content = carousel.window.debug_bounds(SLIDES[0]).unwrap();
    let previous = carousel.window.debug_bounds("carousel-previous").unwrap();
    let next = carousel.window.debug_bounds("carousel-next").unwrap();
    assert!(content.left() >= previous.right() + px(8.));
    assert!(content.right() <= next.left() - px(8.));
}

#[test]
fn autoplay_requests_one_change_at_each_interval_and_wraps() {
    let mut carousel = Mounted::new(|_| {});
    carousel.expect(Some(0), &[]);
    for (step, active) in [1, 2, 0, 1].into_iter().enumerate() {
        let before = [1, 2, 0, 1];
        carousel.advance(499);
        carousel.draw();
        assert_eq!(carousel.changes.borrow().as_slice(), &before[..step]);
        carousel.advance(1);
        carousel.draw();
        carousel.expect(Some(active), &before[..=step]);
    }
}

#[test]
fn minimum_interval_is_250_milliseconds() {
    let mut carousel = Mounted::new(|surface| surface.interval_ms = 0);
    carousel.advance(249);
    carousel.draw();
    carousel.expect(Some(0), &[]);
    carousel.advance(1);
    carousel.draw();
    carousel.expect(Some(1), &[1]);
}

#[test]
fn extreme_interval_stays_in_native_range_without_immediate_navigation() {
    let configured = Carousel::new(Vec::new()).interval_ms(u64::MAX);
    assert!(Duration::from_millis(configured.interval_ms).as_nanos() <= i64::MAX as u128);
    let mut carousel = Mounted::new(|surface| surface.interval_ms = u64::MAX);
    carousel.advance(10_000);
    carousel.draw();
    carousel.expect(Some(0), &[]);
}

#[test]
fn hovering_outside_indicators_pauses_and_leaving_restarts_the_interval() {
    let mut carousel = Mounted::new(|_| {});
    carousel.advance(200);
    let dot = carousel
        .window
        .debug_bounds("carousel-dot-1")
        .unwrap()
        .center();
    carousel.move_mouse(dot);
    carousel.advance(1000);
    carousel.draw();
    carousel.expect(Some(0), &[]);

    carousel.move_mouse(point(px(8.), px(8.)));
    carousel.draw();
    carousel.advance(499);
    carousel.expect(Some(0), &[]);
    carousel.advance(1);
    carousel.draw();
    carousel.expect(Some(1), &[1]);
}

#[test]
fn clipped_indicators_do_not_pause_but_revealing_them_under_the_pointer_does() {
    let mut carousel = Mounted::new(|_| {});
    let dot = carousel
        .window
        .debug_bounds("carousel-dot-1")
        .unwrap()
        .center();
    carousel.update(|surface| surface.clip_height = Some(px(120.)));
    carousel.move_mouse(dot);
    carousel.advance(500);
    carousel.draw();
    carousel.expect(Some(1), &[1]);

    carousel.update(|surface| surface.clip_height = None);
    carousel.advance(1000);
    carousel.draw();
    carousel.expect(Some(1), &[1]);

    carousel.move_mouse(point(px(8.), px(8.)));
    carousel.draw();
    carousel.advance(499);
    carousel.expect(Some(1), &[1]);
    carousel.advance(1);
    carousel.draw();
    carousel.expect(Some(2), &[1, 2]);
}

#[test]
fn carousel_controls_wire_pointer_events_to_on_change() {
    let mut carousel = Mounted::new(|surface| surface.pause_on_hover = false);
    carousel.advance(400);
    carousel.click("carousel-next");
    carousel.expect(Some(1), &[1]);
    carousel.click("carousel-previous");
    carousel.expect(Some(0), &[1, 0]);
    carousel.click("carousel-dot-2");
    carousel.expect(Some(2), &[1, 0, 2]);
    let dot = carousel
        .window
        .debug_bounds("carousel-dot-2")
        .unwrap()
        .center();
    carousel.move_mouse(dot);
    carousel.advance(499);
    carousel.draw();
    carousel.expect(Some(2), &[1, 0, 2]);
    carousel.advance(1);
    carousel.draw();
    carousel.expect(Some(0), &[1, 0, 2, 0]);
}

#[test]
fn uncontrolled_navigation_does_not_require_a_callback() {
    let mut carousel = Mounted::new(|surface| {
        surface.listen = false;
        surface.pause_on_hover = false;
    });
    carousel.click("carousel-next");
    carousel.expect(Some(1), &[]);
    carousel.advance(500);
    carousel.draw();
    carousel.expect(Some(2), &[]);
}

#[test]
fn explicit_index_stays_authoritative_and_sync_does_not_emit() {
    let mut carousel = Mounted::new(|surface| {
        surface.active_index = Some(1);
        surface.pause_on_hover = false;
    });
    carousel.click("carousel-next");
    carousel.expect(Some(1), &[2]);
    carousel.advance(500);
    carousel.draw();
    carousel.expect(Some(1), &[2, 2]);
    carousel.update(|surface| {
        surface.active_index = Some(2);
        surface.accept_changes = true;
    });
    carousel.expect(Some(2), &[2, 2]);
    carousel.advance(500);
    carousel.draw();
    carousel.expect(Some(0), &[2, 2, 0]);
    carousel.click("carousel-dot-2");
    carousel.expect(Some(2), &[2, 2, 0, 2]);
}

#[test]
fn leaving_controlled_mode_retains_the_last_synchronized_index() {
    let mut carousel = Mounted::new(|surface| {
        surface.autoplay = false;
        surface.active_index = Some(2);
    });
    carousel.click("carousel-next");
    carousel.expect(Some(2), &[0]);
    carousel.update(|surface| surface.active_index = None);
    carousel.expect(Some(2), &[0]);
    carousel.click("carousel-previous");
    carousel.expect(Some(1), &[0, 1]);

    carousel.update(|surface| surface.active_index = Some(0));
    carousel.expect(Some(0), &[0, 1]);
    carousel.update(|surface| surface.active_index = None);
    carousel.click("carousel-previous");
    carousel.expect(Some(2), &[0, 1, 2]);
}

#[test]
fn zero_or_one_item_cancels_autoplay_and_count_changes_clamp_silently() {
    let mut carousel = Mounted::new(|surface| surface.count = 0);
    carousel.advance(1000);
    carousel.expect(None, &[]);
    carousel.update(|surface| surface.count = 1);
    carousel.advance(1000);
    carousel.expect(Some(0), &[]);
    carousel.update(|surface| surface.count = 3);
    for _ in 0..2 {
        carousel.advance(500);
        carousel.draw();
    }
    carousel.expect(Some(2), &[1, 2]);
    carousel.update(|surface| surface.count = 2);
    carousel.expect(Some(1), &[1, 2]);
    carousel.update(|surface| surface.count = 1);
    carousel.advance(1000);
    carousel.draw();
    carousel.expect(Some(0), &[1, 2]);
}

#[test]
fn changing_interval_or_disabling_replaces_pending_wake() {
    let mut carousel = Mounted::new(|_| {});
    carousel.advance(400);
    carousel.update(|surface| surface.interval_ms = 1000);
    carousel.advance(999);
    carousel.draw();
    carousel.expect(Some(0), &[]);
    carousel.advance(1);
    carousel.draw();
    carousel.expect(Some(1), &[1]);
    carousel.update(|surface| surface.autoplay = false);
    carousel.advance(1500);
    carousel.draw();
    carousel.expect(Some(1), &[1]);
    carousel.update(|surface| surface.autoplay = true);
    carousel.advance(1000);
    carousel.draw();
    carousel.expect(Some(2), &[1, 2]);
}

#[test]
fn callback_can_redraw_with_a_new_count_and_interval() {
    let mut carousel = Mounted::new(|surface| {
        surface.callback_update = Some(|surface| {
            surface.count = 2;
            surface.interval_ms = 1000;
            surface.callback_update = None;
        });
    });
    carousel.advance(500);
    carousel.draw();
    carousel.expect(Some(1), &[1]);
    carousel.advance(999);
    carousel.draw();
    carousel.expect(Some(1), &[1]);
    carousel.advance(1);
    carousel.draw();
    carousel.expect(Some(0), &[1, 0]);
}

#[test]
fn callback_can_redraw_and_unmount_its_own_carousel() {
    let mut carousel = Mounted::new(|surface| {
        surface.callback_update = Some(|surface| {
            surface.mounted = false;
            surface.callback_update = None;
        });
    });
    carousel.advance(500);
    carousel.expect(None, &[1]);
    assert!(carousel.callback_lifetime.upgrade().is_none());
    carousel.advance(1000);
    carousel.expect(None, &[1]);
}

#[test]
fn unmount_drops_callback_and_remount_starts_fresh() {
    let mut carousel = Mounted::new(|_| {});
    carousel.advance(500);
    carousel.draw();
    carousel.expect(Some(1), &[1]);
    assert!(carousel.callback_lifetime.upgrade().is_some());
    carousel.update(|surface| surface.mounted = false);
    assert!(carousel.callback_lifetime.upgrade().is_none());
    carousel.advance(1000);
    carousel.expect(None, &[1]);
    carousel.update(|surface| surface.mounted = true);
    carousel.expect(Some(0), &[1]);
    carousel.advance(500);
    carousel.draw();
    carousel.expect(Some(1), &[1, 1]);
}

#[test]
fn closing_the_window_drops_callback_before_its_pending_wake() {
    let mut carousel = Mounted::new(|_| {});
    carousel.advance(499);
    assert!(carousel.callback_lifetime.upgrade().is_some());
    carousel.window.update(|window, _| window.remove_window());
    carousel.drain_ready();
    assert!(carousel.callback_lifetime.upgrade().is_none());
    carousel.advance(1000);
    assert!(carousel.changes.borrow().is_empty());
}
