//! Native, display-free pointer regression for HoverCard and Watermark.
//! Run with `cargo run -p liora-gallery --example hover_watermark_smoke --locked`.

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
mod smoke {
    use gpui::{
        App, AppContext, Context, IntoElement, MouseButton, MouseDownEvent, MouseMoveEvent,
        MouseUpEvent, PlatformInput, Render, SharedString, Window, WindowOptions, div, point,
        prelude::*, px,
    };
    use liora_components::{HoverCard, Popover, Watermark, init_liora};
    use liora_core::{Placement, Portal, is_popover_active, render_active_popover_in_window};
    use std::time::Duration;

    struct SmokeView;

    impl Render for SmokeView {
        fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            render_active_popover_in_window(window, cx);
            let portals = if cx.has_global::<Portal>() {
                std::mem::take(&mut cx.global_mut::<Portal>().entries)
            } else {
                Vec::new()
            };
            div()
                .size_full()
                .child(
                    div().absolute().left(px(50.0)).top(px(80.0)).child(
                        HoverCard::new(div().w(px(160.0)).h(px(32.0)).child("Hover me"))
                            .id("hover-smoke")
                            .placement(Placement::BottomStart)
                            .content(|_, _| div().w(px(160.0)).h(px(64.0)).child("Preview")),
                    ),
                )
                .child(
                    div().absolute().left(px(350.0)).top(px(80.0)).child(
                        Popover::new(div().w(px(160.0)).h(px(32.0)).child("Click me"))
                            .id("click-smoke"),
                    ),
                )
                .child(
                    div().absolute().left(px(50.0)).top(px(250.0)).child(
                        Watermark::new(
                            div()
                                .id("watermark-body")
                                .w(px(400.0))
                                .h(px(180.0))
                                .child("Underlying content / 水印测试"),
                            "CONFIDENTIAL / 保密",
                        )
                        .rotate(-32.0)
                        .skew(16.0, 0.0)
                        .rotation_center(0.0, 0.5),
                    ),
                )
                .child(
                    div()
                        .absolute()
                        .size_full()
                        .children(portals.into_iter().map(|entry| (entry.render)(window, cx))),
                )
        }
    }

    fn redraw(window: &mut Window, cx: &mut App) {
        window.refresh();
        window.draw(cx).clear(cx);
    }

    fn move_to(x: f32, y: f32, window: &mut Window, cx: &mut App) {
        window.dispatch_event(
            PlatformInput::MouseMove(MouseMoveEvent {
                position: point(px(x), px(y)),
                ..Default::default()
            }),
            cx,
        );
        redraw(window, cx);
    }

    fn click_at(x: f32, y: f32, window: &mut Window, cx: &mut App) {
        move_to(x, y, window, cx);
        let position = point(px(x), px(y));
        window.dispatch_event(
            PlatformInput::MouseDown(MouseDownEvent {
                position,
                button: MouseButton::Left,
                ..Default::default()
            }),
            cx,
        );
        window.dispatch_event(
            PlatformInput::MouseUp(MouseUpEvent {
                position,
                button: MouseButton::Left,
                ..Default::default()
            }),
            cx,
        );
        redraw(window, cx);
    }

    pub fn run() {
        gpui_platform::headless().run(|cx| {
            init_liora(cx);
            liora_core::push_portal(|_, _| div().into_any_element(), cx);
            liora_core::clear_portals(cx);
            cx.set_reduce_motion(true);
            let handle = cx
                .open_window(
                    WindowOptions {
                        show: false,
                        focus: false,
                        ..Default::default()
                    },
                    |_, cx| cx.new(|_| SmokeView),
                )
                .expect("headless window should open");
            cx.update_window(handle.into(), |_, window, cx| {
                window.resize(gpui::size(px(640.0), px(480.0)));
                redraw(window, cx);
                let hover_id = SharedString::from("hover-smoke");
                assert!(!is_popover_active(&hover_id, cx));
                move_to(60.0, 96.0, window, cx);
                assert!(
                    is_popover_active(&hover_id, cx),
                    "hover without click must open"
                );
                move_to(60.0, 96.0, window, cx);
                assert!(
                    is_popover_active(&hover_id, cx),
                    "overlay must not close on trigger"
                );
                move_to(60.0, 116.0, window, cx);
                assert!(
                    is_popover_active(&hover_id, cx),
                    "offset gap must remain traversable"
                );
                move_to(90.0, 150.0, window, cx);
                assert!(
                    is_popover_active(&hover_id, cx),
                    "preview must remain readable"
                );
                move_to(550.0, 220.0, window, cx);
                let click_id = SharedString::from("click-smoke");
                move_to(360.0, 96.0, window, cx);
                assert!(
                    !is_popover_active(&click_id, cx),
                    "ordinary Popover must remain click-only"
                );
                click_at(360.0, 96.0, window, cx);
                assert!(
                    is_popover_active(&click_id, cx),
                    "ordinary Popover must still open on click"
                );
                click_at(550.0, 220.0, window, cx);
                assert!(
                    !is_popover_active(&click_id, cx),
                    "outside click must close"
                );
            })
            .expect("native pointer regression should run");
            println!("PASS: hover open/bridge/card, click Popover, transformed Unicode watermark");
            let executor = cx.background_executor().clone();
            cx.spawn(async move |cx| {
                executor.timer(Duration::from_millis(10)).await;
                cx.update(|cx| cx.quit());
            })
            .detach();
        });
    }
}

fn main() {
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    smoke::run();
    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
    {
        eprintln!("This smoke requires the Linux/FreeBSD headless backend.");
        std::process::exit(1);
    }
}
