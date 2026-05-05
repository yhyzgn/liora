use gpui::{prelude::*, AnyElement, Global, Bounds, Pixels, Point, App, Window};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Placement {
    Top, TopStart, TopEnd,
    Bottom, BottomStart, BottomEnd,
    Left, LeftStart, LeftEnd,
    Right, RightStart, RightEnd,
}

pub type PortalRender = Box<dyn Fn(&mut Window, &mut App) -> AnyElement>;

pub struct Portal(pub Vec<PortalRender>);
impl Global for Portal {}

pub fn push_portal(render: impl Fn(&mut Window, &mut App) -> AnyElement + 'static, cx: &mut App) {
    if !cx.has_global::<Portal>() {
        cx.set_global(Portal(vec![]));
    }
    cx.global_mut::<Portal>().0.push(Box::new(render));
}

pub fn clear_portals(cx: &mut App) {
    if cx.has_global::<Portal>() {
        cx.global_mut::<Portal>().0.clear();
    }
}

pub struct Popper {
    pub anchor_bounds: Bounds<Pixels>,
    pub placement: Placement,
    pub offset: Pixels,
}

impl Popper {
    pub fn calculate_position(&self, content_size: gpui::Size<Pixels>) -> Point<Pixels> {
        let anchor = self.anchor_bounds;
        let (x, y) = match self.placement {
            Placement::BottomStart => {
                (anchor.left(), anchor.bottom() + self.offset)
            }
            Placement::Bottom => {
                (anchor.left() + (anchor.size.width - content_size.width) / 2.0, anchor.bottom() + self.offset)
            }
            Placement::Top => {
                (anchor.left() + (anchor.size.width - content_size.width) / 2.0, anchor.top() - content_size.height - self.offset)
            }
            // Add more placements as needed
            _ => {
                (anchor.left(), anchor.bottom() + self.offset)
            }
        };

        Point { x, y }
    }
}
