//! Render the Gallery's native matched-text page without a display server.
//! cargo run -p liora-gallery --example highlighted_text_smoke --locked
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn main() {
    use gpui::{
        AnyView, AppContext, Context, IntoElement, Render, Window, WindowOptions, div, prelude::*,
    };
    use liora_components::init_liora;

    struct Surface(AnyView);
    impl Render for Surface {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().child(self.0.clone())
        }
    }

    gpui_platform::headless().run(|cx| {
        init_liora(cx);
        let demo = liora_gallery::demos::highlighted_text_demo::render(cx);
        let surface = cx.new(|_| Surface(demo));
        let handle = cx
            .open_window(
                WindowOptions {
                    show: false,
                    ..Default::default()
                },
                |_, _| surface,
            )
            .unwrap();
        cx.update_window(handle.into(), |_, window, cx| window.draw(cx).clear(cx))
            .unwrap();
        println!("PASS: HighlightedText Gallery demo rendered in a native GPUI window");
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
    eprintln!("This smoke requires the official Linux/FreeBSD headless backend.");
    std::process::exit(1);
}
