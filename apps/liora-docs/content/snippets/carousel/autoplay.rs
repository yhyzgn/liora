use gpui::{App, Context, Entity, IntoElement, Render, Window, prelude::*, px, rgb};
use liora_components::{Carousel, CarouselIndicatorPosition, CarouselItem};

pub fn autoplay_carousel(cx: &mut App) -> Entity<AutoplayCarousel> {
    cx.new(|_| AutoplayCarousel::default())
}

#[derive(Default)]
pub struct AutoplayCarousel {
    active_index: usize,
}

impl Render for AutoplayCarousel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.entity().downgrade();
        Carousel::new(vec![
            CarouselItem::new("Preview").accent(rgb(0x2563eb).into()),
            CarouselItem::new("Release").accent(rgb(0x9333ea).into()),
        ])
        .id("autoplay-carousel")
        .active_index(self.active_index)
        .autoplay(true)
        .interval_ms(1800)
        .pause_on_hover(true)
        .indicator_position(CarouselIndicatorPosition::Outside)
        .height(px(220.0))
        .on_change(move |index, _, cx| {
            let _ = view.update(cx, |demo, cx| {
                demo.active_index = index;
                cx.notify();
            });
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::Cell, rc::Rc, time::Duration};

    struct Surface {
        carousel: Entity<AutoplayCarousel>,
        renders: Rc<Cell<usize>>,
    }

    impl Render for Surface {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            self.renders.set(self.renders.get() + 1);
            liora_components::Card::new(self.carousel.clone()).no_shadow()
        }
    }

    fn drain_ready(app: &gpui::TestAppContext) {
        let settled = (0..100).any(|_| !app.executor().tick());
        assert!(settled, "Carousel must finish ready work within 100 ticks");
    }

    fn draw_frame(app: &gpui::TestAppContext, window: &mut gpui::VisualTestContext) {
        drain_ready(app);
        window.update(|window, cx| window.draw(cx).clear(cx));
        drain_ready(app);
    }

    #[test]
    fn mounted_autoplay_retains_selection_and_deadline_across_parent_redraw() {
        let mut app = gpui::TestAppContext::single();
        app.update(liora_components::init_liora);
        let demo = app.update(autoplay_carousel);
        let parent_renders = Rc::new(Cell::new(0));
        let handle = app.open_window(gpui::size(px(640.0), px(480.0)), |_, _| Surface {
            carousel: demo.clone(),
            renders: parent_renders.clone(),
        });
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        window.simulate_mouse_move(
            gpui::point(px(620.0), px(460.0)),
            None,
            gpui::Modifiers::none(),
        );
        draw_frame(&app, &mut window);
        assert_eq!(demo.read_with(&app, |demo, _| demo.active_index), 0);

        app.executor().advance_clock(Duration::from_millis(1800));
        draw_frame(&app, &mut window);
        assert_eq!(demo.read_with(&app, |demo, _| demo.active_index), 1);

        app.executor().advance_clock(Duration::from_millis(900));
        draw_frame(&app, &mut window);
        assert_eq!(demo.read_with(&app, |demo, _| demo.active_index), 1);
        let rendered_before = parent_renders.get();
        assert!(
            handle
                .update(&mut app, |surface, _, cx| {
                    assert_eq!(surface.carousel.entity_id(), demo.entity_id());
                    cx.notify();
                })
                .is_ok()
        );
        draw_frame(&app, &mut window);
        assert!(
            parent_renders.get() > rendered_before,
            "the retaining parent must actually redraw"
        );
        assert_eq!(demo.read_with(&app, |demo, _| demo.active_index), 1);

        app.executor().advance_clock(Duration::from_millis(899));
        draw_frame(&app, &mut window);
        assert_eq!(demo.read_with(&app, |demo, _| demo.active_index), 1);
        app.executor().advance_clock(Duration::from_millis(1));
        draw_frame(&app, &mut window);
        assert_eq!(demo.read_with(&app, |demo, _| demo.active_index), 0);
        app.quit();
    }
}
