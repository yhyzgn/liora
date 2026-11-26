use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::Space;

use liora_components::layout_helpers::page;

#[path = "../../../liora-docs/content/snippets/loading/basic.rs"]
mod loading_basic_example;
#[path = "../../../liora-docs/content/snippets/loading/fullscreen.rs"]
mod loading_fullscreen_example;

pub fn render(cx: &mut App) -> AnyView {
    let fullscreen = render_fullscreen(cx);
    cx.new(|_| LoadingDemo(fullscreen)).into()
}

pub(super) fn render_fullscreen(cx: &mut App) -> AnyView {
    loading_fullscreen_example::fullscreen_loading(cx)
}

struct LoadingDemo(AnyView);

impl Render for LoadingDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Loading 加载",
            "加载数据时显示。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(loading_basic_example::basic_loading())
                .child(self.0.clone()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{TestAppContext, VisualTestContext, div, point, px, size};
    use liora_components::{Button, PortalLayer};
    use std::{cell::Cell, rc::Rc, time::Duration};

    struct Surface(Option<AnyView>, Rc<Cell<usize>>);

    impl Render for Surface {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let clicks = self.1.clone();
            div()
                .relative()
                .size_full()
                .children(self.0.clone())
                .child(
                    div()
                        .absolute()
                        .bottom_4()
                        .right_4()
                        .debug_selector(|| "loading-background".into())
                        .child(Button::new("Background").on_click(move |_, _, _| {
                            clicks.set(clicks.get() + 1);
                        })),
                )
                .child(PortalLayer)
        }
    }

    fn draw(window: &mut VisualTestContext) {
        window.update(|window, cx| window.draw(cx).clear(cx));
    }

    fn click(window: &mut VisualTestContext, selector: &'static str) {
        let position = window
            .debug_bounds(selector)
            .unwrap_or_else(|| panic!("missing {selector}"))
            .center();
        window.simulate_mouse_move(position, None, Default::default());
        window.simulate_click(position, Default::default());
        draw(window);
    }

    fn exercise_preview(factory: fn(&mut App) -> AnyView) {
        let mut app = TestAppContext::single();
        app.update(liora_components::init_liora);
        let demo = app.update(factory);
        let clicks = Rc::new(Cell::new(0));
        let surface = app.open_window(size(px(640.0), px(480.0)), |_, _| {
            Surface(Some(demo), clicks.clone())
        });
        let mut window = VisualTestContext::from_window(surface.into(), &app);
        let advance = |window: &mut VisualTestContext| {
            app.executor().advance_clock(Duration::from_millis(1500));
            window.run_until_parked();
            draw(window);
        };

        draw(&mut window);
        click(&mut window, "loading-preview-start");
        let bounds = window
            .debug_bounds("loading-preview-overlay")
            .expect("overlay");
        assert_eq!(bounds.origin, point(px(0.0), px(0.0)));
        assert_eq!(
            bounds.size,
            window.update(|window, _| window.viewport_size())
        );
        click(&mut window, "loading-background");
        assert_eq!(clicks.get(), 0, "overlay leaked a background click");
        advance(&mut window);
        click(&mut window, "loading-preview-cancel");
        assert!(window.debug_bounds("loading-preview-overlay").is_none());
        click(&mut window, "loading-background");
        assert_eq!(clicks.get(), 1);

        click(&mut window, "loading-preview-start");
        advance(&mut window);
        assert!(
            window.debug_bounds("loading-preview-overlay").is_some(),
            "old deadline closed a new preview"
        );
        advance(&mut window);
        assert!(window.debug_bounds("loading-preview-overlay").is_none());
        click(&mut window, "loading-background");
        assert_eq!(clicks.get(), 2);

        click(&mut window, "loading-preview-start");
        app.update(|cx| {
            surface
                .update(cx, |surface, _, cx| {
                    surface.0 = None;
                    cx.notify();
                })
                .expect("surface window must remain available");
        });
        draw(&mut window);
        assert!(window.debug_bounds("loading-preview-overlay").is_none());
        advance(&mut window);
        advance(&mut window);
        click(&mut window, "loading-background");
        assert_eq!(
            clicks.get(),
            3,
            "unmounted preview left an occluding portal"
        );
    }

    #[test]
    fn gallery_fullscreen_blocks_pointer_and_releases_on_cancel_timeout_and_unmount() {
        exercise_preview(render);
    }

    #[test]
    fn docs_fullscreen_blocks_pointer_and_releases_on_cancel_timeout_and_unmount() {
        exercise_preview(|cx| super::super::render_doc_demo("LoadingFullscreen", cx).unwrap());
    }
}
