use super::*;
use gpui::{InputEvent, MouseMoveEvent, TestAppContext, VisualTestContext, point, size};

fn drain(app: &TestAppContext) {
    for _ in 0..100 {
        if !app.executor().tick() {
            return;
        }
    }
    panic!("notification tasks did not park");
}

fn advance(app: &TestAppContext, millis: u64) {
    app.executor().advance_clock(Duration::from_millis(millis));
    drain(app);
}

#[test]
fn replaced_manager_does_not_expire_its_successor_notifications() {
    let app = TestAppContext::single();
    app.update(crate::init_liora);
    app.update(|cx| show_notification("old", None, NotificationType::Info, cx));
    drain(&app);
    advance(&app, 1000);
    let current = app.update(|cx| {
        let current = cx.new(|_| NotificationManager::new());
        cx.set_global(NotificationManagerGlobal(current.clone()));
        show_notification("new", None, NotificationType::Success, cx);
        current
    });
    drain(&app);
    advance(&app, 3000);
    assert_eq!(
        current.read_with(&app, |value, _| value.notifications.len()),
        1
    );
    advance(&app, 1000);
    assert_eq!(
        current.read_with(&app, |value, _| value.notifications.len()),
        0
    );
    app.quit();
}

fn show(app: &mut TestAppContext, title: &'static str) -> Entity<NotificationManager> {
    app.update(|cx| {
        show_notification(title, None, NotificationType::Info, cx);
        cx.global::<NotificationManagerGlobal>().0.clone()
    })
}

fn titles(manager: &Entity<NotificationManager>, app: &TestAppContext) -> Vec<SharedString> {
    manager.read_with(app, |value, _| {
        value
            .notifications
            .iter()
            .map(|entry| entry.item.title.clone())
            .collect()
    })
}

#[test]
fn stacked_notifications_expire_independently() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let manager = show(&mut app, "first");
    drain(&app);
    advance(&app, 1000);
    show(&mut app, "second");
    drain(&app);
    advance(&app, 2999);
    assert_eq!(titles(&manager, &app), ["first", "second"]);
    advance(&app, 1);
    assert_eq!(titles(&manager, &app), ["second"]);
    advance(&app, 1000);
    assert!(titles(&manager, &app).is_empty());
    app.quit();
}

struct Surface(Entity<NotificationManager>);

impl Render for Surface {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.0.clone())
    }
}

fn draw(window: &mut VisualTestContext) {
    window.update(|window, cx| window.draw(cx).clear(cx));
}

fn move_mouse(window: &mut VisualTestContext, x: f32, y: f32) {
    window.update(|window, cx| {
        window.dispatch_event(
            MouseMoveEvent {
                position: point(px(x), px(y)),
                ..Default::default()
            }
            .to_platform_input(),
            cx,
        );
    });
}

#[test]
fn actual_hover_preserves_remaining_time_without_pausing_siblings() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let manager = show(&mut app, "first");
    show(&mut app, "second");
    let handle = app.open_window(size(px(800.), px(600.)), |_, _| Surface(manager.clone()));
    let mut window = VisualTestContext::from_window(handle.into(), &app);
    drain(&app);
    advance(&app, 1000);
    draw(&mut window);
    let bounds = window.debug_bounds("notification-0").unwrap();
    move_mouse(
        &mut window,
        f32::from(bounds.left()) + 10.,
        f32::from(bounds.top()) + 10.,
    );
    advance(&app, 6000);
    assert_eq!(titles(&manager, &app), ["first"]);
    draw(&mut window);
    move_mouse(&mut window, 10., 580.);
    drain(&app);
    advance(&app, 2999);
    assert_eq!(titles(&manager, &app), ["first"]);
    advance(&app, 1);
    assert!(titles(&manager, &app).is_empty());
    app.quit();
}

#[test]
fn closing_hovered_window_resumes_notification_lifetime() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let manager = show(&mut app, "first");
    let handle = app.open_window(size(px(800.), px(600.)), |_, _| Surface(manager.clone()));
    let window_id = handle.window_id();
    drain(&app);
    advance(&app, 1000);
    manager.update(&mut app, |value, cx| {
        value.set_hovered(0, window_id, true, cx)
    });
    advance(&app, 6000);
    handle
        .update(&mut app, |_, window, _| window.remove_window())
        .unwrap();
    drain(&app);
    advance(&app, 2999);
    assert_eq!(titles(&manager, &app), ["first"]);
    advance(&app, 1);
    assert!(titles(&manager, &app).is_empty());
    app.quit();
}

#[test]
fn dismissing_one_notification_cancels_only_its_deadline() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let manager = show(&mut app, "first");
    show(&mut app, "second");
    drain(&app);
    advance(&app, 1000);
    manager.update(&mut app, |value, cx| value.dismiss(0, cx));
    manager.update(&mut app, |value, cx| value.dismiss(0, cx));
    assert_eq!(titles(&manager, &app), ["second"]);
    advance(&app, 3000);
    assert!(titles(&manager, &app).is_empty());
    app.quit();
}

#[test]
fn dropping_the_manager_cancels_its_pending_deadlines() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let manager = show(&mut app, "first");
    let weak = manager.downgrade();
    app.update(|cx| {
        cx.remove_global::<NotificationManagerGlobal>();
    });
    drop(manager);
    drain(&app);
    assert!(weak.upgrade().is_none());
    advance(&app, 5000);
    app.quit();
}

#[test]
fn hover_in_another_window_keeps_the_shared_notification_paused() {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    let manager = show(&mut app, "first");
    let first = app.open_window(size(px(800.), px(600.)), |_, _| Surface(manager.clone()));
    let second = app.open_window(size(px(800.), px(600.)), |_, _| Surface(manager.clone()));
    drain(&app);
    advance(&app, 1000);
    manager.update(&mut app, |value, cx| {
        value.set_hovered(0, first.window_id(), true, cx);
        value.set_hovered(0, first.window_id(), true, cx);
        value.set_hovered(0, second.window_id(), true, cx);
        value.set_hovered(0, first.window_id(), false, cx);
    });
    advance(&app, 5000);
    assert_eq!(titles(&manager, &app), ["first"]);
    manager.update(&mut app, |value, cx| {
        value.set_hovered(0, second.window_id(), false, cx)
    });
    drain(&app);
    advance(&app, 2999);
    assert_eq!(titles(&manager, &app), ["first"]);
    advance(&app, 1);
    assert!(titles(&manager, &app).is_empty());
    app.quit();
}

#[test]
fn native_close_button_accepts_enter_and_space_without_closing_siblings() {
    for key in ["enter", "space"] {
        let mut app = TestAppContext::single();
        app.update(crate::init_liora);
        let manager = show(&mut app, "first");
        show(&mut app, "second");
        let handle = app.open_window(size(px(800.), px(600.)), |_, _| Surface(manager.clone()));
        let mut window = VisualTestContext::from_window(handle.into(), &app);
        drain(&app);
        draw(&mut window);
        window.update(|window, cx| window.focus_next(cx));
        draw(&mut window);
        window.simulate_keystrokes(key);
        window.update(|window, cx| {
            let keystroke = gpui::Keystroke::parse_with_os(key, cx.platform_info().os).unwrap();
            window.dispatch_event(
                gpui::PlatformInput::KeyUp(gpui::KeyUpEvent { keystroke }),
                cx,
            );
        });
        assert_eq!(titles(&manager, &app), ["second"], "{key}");
        app.quit();
    }
}
