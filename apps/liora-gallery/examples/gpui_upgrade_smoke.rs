//! Linux headless layout smoke for the pinned official GPUI upgrade.
//! Runs real native text/layout/spring paths without a display server or GPU.

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
mod smoke {
    use gpui::{
        AnimationExt, AppContext, Context, IntoElement, Pixels, Render, Window, WindowOptions, px,
    };
    use liora_components::{MotionSpring, Space, Text, init_liora};
    use std::{cell::Cell, rc::Rc, time::Duration};

    struct SmokeView {
        target: Pixels,
        observed: Rc<Cell<Option<Pixels>>>,
    }

    impl Render for SmokeView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            let observed = self.observed.clone();
            Space::new()
                .vertical()
                .child(Text::new("Native GPUI / 原生文字换行与布局").wrap())
                .child(Text::new("Spring target"))
                .with_spring(
                    "gpui-upgrade-smoke-gap",
                    MotionSpring::Snappy.animation_to(self.target).from(px(4.0)),
                    move |row, gap| {
                        observed.set(Some(gap));
                        row.gap(gap)
                    },
                )
        }
    }

    pub fn run() {
        gpui_platform::headless().run(|cx| {
            init_liora(cx);
            let observed = Rc::new(Cell::new(None));
            let handle = cx
                .open_window(
                    WindowOptions {
                        show: false,
                        focus: false,
                        ..WindowOptions::default()
                    },
                    |_, cx| {
                        cx.new(|_| SmokeView {
                            target: px(24.0),
                            observed: observed.clone(),
                        })
                    },
                )
                .expect("native headless window should open");
            cx.update_window(handle.into(), |_, window, cx| {
                window.draw(cx).clear(cx);
            })
            .expect("initial native layout should draw");
            assert!(
                observed.get().is_some(),
                "spring animator must run during layout"
            );

            cx.set_reduce_motion(true);
            handle
                .update(cx, |view, _, cx| {
                    view.target = px(48.0);
                    cx.notify();
                })
                .expect("spring target should update");
            cx.update_window(handle.into(), |_, window, cx| {
                window.refresh();
                window.draw(cx).clear(cx);
            })
            .expect("retargeted native layout should draw");
            assert_eq!(
                observed.get(),
                Some(px(48.0)),
                "reduce-motion must resolve to the target"
            );
            println!("PASS: native headless text/layout and retargeted reduced-motion spring");
            // Quit after the platform run loop starts; stopping it inside the
            // launch callback can be reset by the loop's initial run call.
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
        eprintln!("This display-free smoke uses the official Linux/FreeBSD headless backend.");
        std::process::exit(1);
    }
}
