use gpui::{AnyElement, App, Bounds, Global, Pixels, Point, SharedString, Window};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Placement {
    Top,
    TopStart,
    TopEnd,
    Bottom,
    BottomStart,
    BottomEnd,
    Left,
    LeftStart,
    LeftEnd,
    Right,
    RightStart,
    RightEnd,
}

impl Placement {
    pub fn flip(&self) -> Self {
        match self {
            Placement::Top => Placement::Bottom,
            Placement::TopStart => Placement::BottomStart,
            Placement::TopEnd => Placement::BottomEnd,
            Placement::Bottom => Placement::Top,
            Placement::BottomStart => Placement::TopStart,
            Placement::BottomEnd => Placement::TopEnd,
            Placement::Left => Placement::Right,
            Placement::LeftStart => Placement::RightStart,
            Placement::LeftEnd => Placement::RightEnd,
            Placement::Right => Placement::Left,
            Placement::RightStart => Placement::LeftStart,
            Placement::RightEnd => Placement::LeftEnd,
        }
    }
}

pub type PortalRender = Box<dyn FnOnce(&mut Window, &mut App) -> AnyElement>;

pub struct PortalEntry {
    pub id: u64,
    pub render: PortalRender,
}

pub struct Portal {
    pub entries: Vec<PortalEntry>,
    next_id: u64,
}

impl Global for Portal {}

pub fn push_portal(
    render: impl FnOnce(&mut Window, &mut App) -> AnyElement + 'static,
    cx: &mut App,
) -> u64 {
    if !cx.has_global::<Portal>() {
        cx.set_global(Portal {
            entries: vec![],
            next_id: 1,
        });
    }
    let portal = cx.global_mut::<Portal>();
    let id = portal.next_id;
    portal.next_id += 1;
    portal.entries.push(PortalEntry {
        id,
        render: Box::new(render),
    });
    id
}

pub fn remove_portal(id: u64, cx: &mut App) {
    if cx.has_global::<Portal>() {
        cx.global_mut::<Portal>().entries.retain(|e| e.id != id);
    }
}

pub fn clear_portals(cx: &mut App) {
    if cx.has_global::<Portal>() {
        cx.global_mut::<Portal>().entries.clear();
    }
}

pub struct ZIndexStack {
    pub base: u32,
    pub popup: u32,
    pub modal: u32,
    pub notification: u32,
    pub tooltip: u32,
}

impl Default for ZIndexStack {
    fn default() -> Self {
        Self {
            base: 1000,
            popup: 1100,
            modal: 1200,
            notification: 1300,
            tooltip: 1400,
        }
    }
}

impl Global for ZIndexStack {}

#[derive(Clone)]
pub struct TooltipData {
    pub content: SharedString,
    pub anchor_bounds: Bounds<Pixels>,
    pub placement: Placement,
    pub offset: Pixels,
}

pub struct ActiveTooltip(pub Option<TooltipData>);
impl Global for ActiveTooltip {}

pub fn set_active_tooltip(data: TooltipData, cx: &mut App) {
    cx.set_global(ActiveTooltip(Some(data)));
}

pub fn clear_active_tooltip(cx: &mut App) {
    cx.set_global(ActiveTooltip(None));
}

pub struct ActivePopover(pub Option<gpui::AnyView>);
impl Global for ActivePopover {}

pub fn set_active_popover(view: gpui::AnyView, cx: &mut App) {
    cx.set_global(ActivePopover(Some(view)));
}

pub fn clear_active_popover(cx: &mut App) {
    cx.set_global(ActivePopover(None));
}

pub struct ActiveModal(pub Option<gpui::AnyView>);
impl Global for ActiveModal {}

pub fn set_active_modal(view: gpui::AnyView, cx: &mut App) {
    cx.set_global(ActiveModal(Some(view)));
}

pub fn clear_active_modal(cx: &mut App) {
    cx.set_global(ActiveModal(None));
}

pub struct ActiveDrawer(pub Option<gpui::AnyView>);
impl Global for ActiveDrawer {}

pub fn set_active_drawer(view: gpui::AnyView, cx: &mut App) {
    cx.set_global(ActiveDrawer(Some(view)));
}

pub fn clear_active_drawer(cx: &mut App) {
    cx.set_global(ActiveDrawer(None));
}

pub struct Popper {
    pub anchor_bounds: Bounds<Pixels>,
    pub placement: Placement,
    pub offset: Pixels,
}

impl Popper {
    pub fn calculate_position(&self, content_size: gpui::Size<Pixels>) -> Point<Pixels> {
        self.calculate_position_with_placement(self.placement, content_size)
    }

    fn calculate_position_with_placement(
        &self,
        placement: Placement,
        content_size: gpui::Size<Pixels>,
    ) -> Point<Pixels> {
        let anchor = self.anchor_bounds;
        let (x, y) = match placement {
            Placement::Top => (
                anchor.left() + (anchor.size.width - content_size.width) / 2.0,
                anchor.top() - content_size.height - self.offset,
            ),
            Placement::TopStart => (
                anchor.left(),
                anchor.top() - content_size.height - self.offset,
            ),
            Placement::TopEnd => (
                anchor.right() - content_size.width,
                anchor.top() - content_size.height - self.offset,
            ),
            Placement::Bottom => (
                anchor.left() + (anchor.size.width - content_size.width) / 2.0,
                anchor.bottom() + self.offset,
            ),
            Placement::BottomStart => (anchor.left(), anchor.bottom() + self.offset),
            Placement::BottomEnd => (
                anchor.right() - content_size.width,
                anchor.bottom() + self.offset,
            ),
            Placement::Left => (
                anchor.left() - content_size.width - self.offset,
                anchor.top() + (anchor.size.height - content_size.height) / 2.0,
            ),
            Placement::LeftStart => (
                anchor.left() - content_size.width - self.offset,
                anchor.top(),
            ),
            Placement::LeftEnd => (
                anchor.left() - content_size.width - self.offset,
                anchor.bottom() - content_size.height,
            ),
            Placement::Right => (
                anchor.right() + self.offset,
                anchor.top() + (anchor.size.height - content_size.height) / 2.0,
            ),
            Placement::RightStart => (anchor.right() + self.offset, anchor.top()),
            Placement::RightEnd => (
                anchor.right() + self.offset,
                anchor.bottom() - content_size.height,
            ),
        };

        Point { x, y }
    }

    pub fn calculate_position_with_flip(
        &self,
        content_size: gpui::Size<Pixels>,
        viewport: Bounds<Pixels>,
    ) -> (Point<Pixels>, Placement) {
        let pos = self.calculate_position_with_placement(self.placement, content_size);
        let mut final_pos = pos;
        let mut final_placement = self.placement;

        let out_of_bounds = pos.x < viewport.left()
            || pos.x + content_size.width > viewport.right()
            || pos.y < viewport.top()
            || pos.y + content_size.height > viewport.bottom();

        if out_of_bounds {
            let flipped_placement = self.placement.flip();
            let flipped_pos =
                self.calculate_position_with_placement(flipped_placement, content_size);

            let flipped_out_of_bounds = flipped_pos.x < viewport.left()
                || flipped_pos.x + content_size.width > viewport.right()
                || flipped_pos.y < viewport.top()
                || flipped_pos.y + content_size.height > viewport.bottom();

            if !flipped_out_of_bounds {
                final_pos = flipped_pos;
                final_placement = flipped_placement;
            }
        }

        final_pos.x = final_pos
            .x
            .clamp(viewport.left(), viewport.right() - content_size.width);
        final_pos.y = final_pos
            .y
            .clamp(viewport.top(), viewport.bottom() - content_size.height);

        (final_pos, final_placement)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{point, px, size};

    fn viewport() -> Bounds<Pixels> {
        Bounds {
            origin: point(px(0.0), px(0.0)),
            size: size(px(800.0), px(600.0)),
        }
    }

    fn anchor(x: f32, width: f32) -> Bounds<Pixels> {
        Bounds {
            origin: point(px(x), px(200.0)),
            size: size(px(width), px(40.0)),
        }
    }

    #[test]
    fn centered_vertical_placements_align_content_center_with_anchor_center() {
        let content_size = size(px(180.0), px(80.0));
        let anchor_bounds = anchor(300.0, 80.0);
        let popper = Popper {
            anchor_bounds,
            placement: Placement::Bottom,
            offset: px(8.0),
        };

        let (pos, placement) = popper.calculate_position_with_flip(content_size, viewport());

        assert_eq!(placement, Placement::Bottom);
        assert_eq!(
            pos.x + content_size.width / 2.0,
            anchor_bounds.left() + anchor_bounds.size.width / 2.0
        );
        assert_eq!(pos.y, anchor_bounds.bottom() + px(8.0));
    }

    #[test]
    fn centered_vertical_placements_clamp_horizontally_to_viewport() {
        let content_size = size(px(220.0), px(80.0));
        let near_left = Popper {
            anchor_bounds: anchor(8.0, 40.0),
            placement: Placement::Bottom,
            offset: px(8.0),
        };
        let near_right = Popper {
            anchor_bounds: anchor(760.0, 32.0),
            placement: Placement::Bottom,
            offset: px(8.0),
        };

        let (left_pos, _) = near_left.calculate_position_with_flip(content_size, viewport());
        let (right_pos, _) = near_right.calculate_position_with_flip(content_size, viewport());

        assert_eq!(left_pos.x, px(0.0));
        assert_eq!(right_pos.x + content_size.width, viewport().right());
    }
}
