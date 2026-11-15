//! Real-display CopyButton regression; run on a disposable X11/Wayland display.
//! cargo run -p liora-gallery --example copy_button_smoke --locked
//! Restores GPUI-readable clipboard formats even after an assertion failure.
//! Other clipboard formats require a disposable clipboard; never use headless.
use gpui::{
    App, AppContext, Bounds, ClipboardItem, Context, Entity, Focusable, IntoElement, Keystroke,
    MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, PlatformInput, Point,
    Render, Window, WindowBounds, WindowHandle, WindowOptions, canvas, div, point, prelude::*, px,
    size,
};
use liora_components::{CopyButton, init_liora, write_text_to_clipboard};
use std::{
    cell::Cell,
    panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
    rc::Rc,
    time::Duration,
};

struct Surface {
    buttons: [Entity<CopyButton>; 2],
    bounds: [Rc<Cell<Bounds<Pixels>>>; 2],
}

impl Render for Surface {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let mut root = div()
            .flex()
            .flex_col()
            .items_start()
            .gap(px(24.))
            .p(px(24.));
        for (button, bounds) in self.buttons.iter().zip(&self.bounds) {
            let bounds = bounds.clone();
            root = root.child(
                div().relative().flex_none().child(button.clone()).child(
                    canvas(move |value, _, _| bounds.set(value), |_, _, _, _| {})
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full(),
                ),
            );
        }
        root
    }
}

fn click(window: &mut Window, cx: &mut App, position: Point<Pixels>) {
    window.dispatch_event(
        PlatformInput::MouseMove(MouseMoveEvent {
            position,
            ..Default::default()
        }),
        cx,
    );
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

fn assert_clipboard(cx: &mut App, expected: &str) {
    assert_eq!(
        cx.read_from_clipboard()
            .and_then(|item| item.text())
            .as_deref(),
        Some(expected)
    );
}

fn restore_clipboard(cx: &mut App, previous: &mut Option<ClipboardItem>) {
    cx.write_to_clipboard(previous.take().unwrap_or(ClipboardItem {
        entries: Vec::new(),
    }));
}

fn step(
    cx: &mut App,
    handle: WindowHandle<Surface>,
    previous: &mut Option<ClipboardItem>,
    check: impl FnOnce(&mut Window, &mut App),
) {
    let result = catch_unwind(AssertUnwindSafe(|| {
        cx.update_window(handle.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
            check(window, cx);
        })
        .unwrap();
    }));
    if let Err(panic) = result {
        restore_clipboard(cx, previous);
        resume_unwind(panic);
    }
}

fn main() {
    gpui_platform::application().with_assets(liora_icons::IconAssetSource).run(|cx| {
        init_liora(cx);
        let enabled = cx.new(|cx| CopyButton::new("smoke-copy", "pointer 🦀\nsecond line", cx));
        let disabled = cx.new(|cx| CopyButton::new("smoke-disabled", "must not copy", cx).disabled(true));
        let bounds = [Rc::new(Cell::new(Bounds::default())), Rc::new(Cell::new(Bounds::default()))];
        let surface = cx.new(|_| Surface {
            buttons: [enabled.clone(), disabled.clone()],
            bounds: bounds.clone(),
        });
        let handle = cx.open_window(WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                point(px(0.), px(0.)), size(px(480.), px(240.)),
            ))),
            ..Default::default()
        }, |_, _| surface).unwrap();
        let executor = cx.background_executor().clone();
        cx.spawn(async move |cx| {
            executor.timer(Duration::from_millis(100)).await;
            let mut previous = cx.update(|cx| cx.read_from_clipboard());
            let mut idle_width = px(0.);
            cx.update(|cx| step(cx, handle, &mut previous, |window, cx| {
                let initial = bounds[0].get();
                assert!(initial.size.width > px(0.) && initial.size.height > px(0.));
                idle_width = initial.size.width;
                write_text_to_clipboard(cx, "pointer sentinel");
                click(window, cx, initial.center());
                assert_clipboard(cx, "pointer 🦀\nsecond line");
                assert!(enabled.focus_handle(cx).is_focused(window), "pointer activation focuses the control");
                window.draw(cx).clear(cx);
                // Observe native layout, not private feedback state: Copied is wider than Copy.
                assert!(bounds[0].get().size.width > idle_width, "copied feedback changes the rendered label width");
                enabled.update(cx, |button, cx| button.set_text("Enter payload", cx));
                window.dispatch_keystroke(Keystroke::parse("enter").unwrap(), cx);
                assert_clipboard(cx, "Enter payload");
            }));

            executor.timer(Duration::from_millis(1200)).await;
            cx.update(|cx| step(cx, handle, &mut previous, |window, cx| {
                enabled.update(cx, |button, cx| button.set_text("Space payload", cx));
                window.dispatch_keystroke(Keystroke::parse("space").unwrap(), cx);
                assert_clipboard(cx, "Space payload");
                window.draw(cx).clear(cx);
                assert!(bounds[0].get().size.width > idle_width);
            }));

            // The first activation's two-second deadline has passed, but the last one's has not.
            executor.timer(Duration::from_millis(1100)).await;
            cx.update(|cx| step(cx, handle, &mut previous, |window, cx| {
                assert!(bounds[0].get().size.width > idle_width, "an older expiry must not clear repeated feedback");
                write_text_to_clipboard(cx, "disabled sentinel");
                click(window, cx, bounds[1].get().center());
                assert_clipboard(cx, "disabled sentinel");
                window.focus(&disabled.focus_handle(cx), cx);
                window.draw(cx).clear(cx);
                for key in ["enter", "space"] {
                    window.dispatch_keystroke(Keystroke::parse(key).unwrap(), cx);
                    assert_clipboard(cx, "disabled sentinel");
                }
            }));

            executor.timer(Duration::from_millis(1100)).await;
            cx.update(|cx| {
                step(cx, handle, &mut previous, |_, cx| {
                    assert_eq!(bounds[0].get().size.width, idle_width, "feedback returns to the initial rendered Copy label after the latest deadline");
                    assert_clipboard(cx, "disabled sentinel");
                });
                restore_clipboard(cx, &mut previous);
                println!("PASS: native CopyButton pointer, Enter/Space, updated payload, disabled no-write, repeated feedback and expiry; prior GPUI clipboard restored");
                cx.quit();
            });
        }).detach();
    });
}
