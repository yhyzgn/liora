//! Geometry regression plus the actual Gallery/Docs Markdown demo for native inspection.
use gpui::{
    AnyView, AppContext, Bounds, Context, Entity, IntoElement, Render, ScrollHandle, Window,
    WindowBounds, WindowOptions, div, point, prelude::*, px, size,
};
use liora_components::{Anchor, AnchorTarget, init_liora};
use std::{cell::Cell, rc::Rc, time::Duration};

#[path = "../src/demos/markdown_demo.rs"]
mod markdown_demo;
#[path = "../src/demos/pdf_reader_demo.rs"]
mod pdf_reader_demo;

struct Surface(AnyView);
impl Render for Surface {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .p_4()
            .bg(cx.global::<liora_core::Config>().theme.neutral.body)
            .text_color(cx.global::<liora_core::Config>().theme.neutral.text_1)
            .child(self.0.clone())
    }
}
struct AnchorProbe {
    anchor: Entity<Anchor>,
    bounds: Rc<Cell<Bounds<gpui::Pixels>>>,
}
impl Render for AnchorProbe {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let bounds = self.bounds.clone();
        div()
            .flex()
            .flex_col()
            .p(px(32.))
            .child(div().h(px(80.)))
            .child(AnchorTarget::new(
                "target",
                self.anchor.clone(),
                div()
                    .w_full()
                    .h(px(60.))
                    .on_children_prepainted(move |children, _, _| bounds.set(children[0]))
                    .child(div().size_full()),
            ))
    }
}
fn assert_anchor_geometry() {
    let mut app = gpui::TestAppContext::single();
    app.update(init_liora);
    let anchor = app.new(|_| Anchor::new(ScrollHandle::new()));
    let bounds = Rc::new(Cell::new(Bounds::default()));
    let measured = bounds.clone();
    let handle = app.open_window(size(px(800.), px(600.)), |_, _| AnchorProbe {
        anchor,
        bounds: measured,
    });
    app.update_window(handle.into(), |_, window, cx| window.draw(cx).clear(cx))
        .unwrap();
    let bounds = bounds.get();
    assert_eq!(
        bounds.origin,
        point(px(32.), px(112.)),
        "anchor must paint at its assigned layout origin, not apply its offset twice"
    );
    assert_eq!(bounds.size, size(px(736.), px(60.)));
    println!("PASS: padded native AnchorTarget geometry {bounds:?}");
}
fn main() {
    assert_anchor_geometry();
    let native = std::env::var_os("LIORA_NATIVE_SMOKE").is_some();
    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
    assert!(
        native,
        "Headless smoke requires Linux/FreeBSD; set LIORA_NATIVE_SMOKE=1 for a real window"
    );
    let app = if native {
        gpui_platform::application()
    } else {
        gpui_platform::headless()
    };
    app.with_assets(liora_icons::IconAssetSource)
        .run(move |cx| {
            init_liora(cx);
            let demo = if std::env::args().any(|arg| arg == "--pdf") {
                pdf_reader_demo::render(cx)
            } else {
                markdown_demo::render(cx).into()
            };
            let window = cx
                .open_window(
                    WindowOptions {
                        titlebar: Some(gpui::TitlebarOptions {
                            title: Some("Liora Document Layout Smoke".into()),
                            ..Default::default()
                        }),
                        window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                            point(px(0.), px(0.)),
                            size(px(1568.), px(830.)),
                        ))),
                        show: native,
                        focus: native,
                        ..Default::default()
                    },
                    |_, cx| cx.new(|_| Surface(demo)),
                )
                .unwrap();
            cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx))
                .unwrap();
            println!("READY: document layout smoke");
            let executor = cx.background_executor().clone();
            cx.spawn(async move |cx| {
                executor
                    .timer(Duration::from_millis(if native { 20000 } else { 10 }))
                    .await;
                cx.update(|cx| cx.quit());
            })
            .detach();
        });
}
