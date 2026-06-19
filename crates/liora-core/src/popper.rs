use gpui::{AnyElement, App, Bounds, Global, Pixels, Point, SharedString, Window};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Options that control placement behavior.
pub enum Placement {
    /// Places the overlay above the anchor.
    Top,
    /// Places the overlay above the anchor aligned to the start edge.
    TopStart,
    /// Places the overlay above the anchor aligned to the end edge.
    TopEnd,
    /// Places the overlay below the anchor.
    Bottom,
    /// Places the overlay below the anchor aligned to the start edge.
    BottomStart,
    /// Places the overlay below the anchor aligned to the end edge.
    BottomEnd,
    /// Places the overlay to the left of the anchor.
    Left,
    /// Places the overlay to the left aligned to the start edge.
    LeftStart,
    /// Places the overlay to the left aligned to the end edge.
    LeftEnd,
    /// Places the overlay to the right of the anchor.
    Right,
    /// Places the overlay to the right aligned to the start edge.
    RightStart,
    /// Places the overlay to the right aligned to the end edge.
    RightEnd,
}

impl Placement {
    /// Returns the opposite placement used when a popper would overflow its viewport.
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

/// Type alias for portal render values used by the popper API.
pub type PortalRender = Box<dyn FnOnce(&mut Window, &mut App) -> AnyElement>;

/// Runtime state used by Liora portal entry behavior.
pub struct PortalEntry {
    /// Stable identifier used for GPUI state, callbacks, and automation.
    pub id: u64,
    /// One-shot closure that produces the portal element.
    pub render: PortalRender,
}

/// Runtime state used by Liora portal behavior.
pub struct Portal {
    /// Queued portal entries waiting to be rendered.
    pub entries: Vec<PortalEntry>,
    next_id: u64,
}

impl Global for Portal {}

/// Runtime state used by Liora passive portal behavior.
pub struct PassivePortal {
    /// Queued portal entries waiting to be rendered.
    pub entries: Vec<PortalEntry>,
    next_id: u64,
}

impl Global for PassivePortal {}

/// Adds a one-shot portal render entry and returns its id.
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

/// Adds a passive portal render entry and returns its id.
pub fn push_passive_portal(
    render: impl FnOnce(&mut Window, &mut App) -> AnyElement + 'static,
    cx: &mut App,
) -> u64 {
    if !cx.has_global::<PassivePortal>() {
        cx.set_global(PassivePortal {
            entries: vec![],
            next_id: 1,
        });
    }
    let portal = cx.global_mut::<PassivePortal>();
    let id = portal.next_id;
    portal.next_id += 1;
    portal.entries.push(PortalEntry {
        id,
        render: Box::new(render),
    });
    id
}

/// Removes the matching portal from the component state.
pub fn remove_portal(id: u64, cx: &mut App) {
    if cx.has_global::<Portal>() {
        cx.global_mut::<Portal>().entries.retain(|e| e.id != id);
    }
}

/// Clears the current portals state.
pub fn clear_portals(cx: &mut App) {
    if cx.has_global::<Portal>() {
        cx.global_mut::<Portal>().entries.clear();
    }
}

/// Runtime state used by Liora zindex stack behavior.
pub struct ZIndexStack {
    /// Base value for the z-index stack.
    pub base: u32,
    /// Z-index assigned to popup-style overlays.
    pub popup: u32,
    /// Modal panel background color.
    pub modal: u32,
    /// Z-index assigned to notification overlays.
    pub notification: u32,
    /// Tooltip text shown by the operating-system tray area.
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
/// Runtime state used by Liora tooltip data behavior.
pub struct TooltipData {
    /// Stable identifier used for GPUI state, callbacks, and automation.
    pub id: SharedString,
    /// Content rendered inside the component body.
    pub content: SharedString,
    /// Bounds of the trigger element used for popper positioning.
    pub anchor_bounds: Bounds<Pixels>,
    /// Preferred placement relative to the trigger or anchor.
    pub placement: Placement,
    /// Pixel offset applied after popper placement is resolved.
    pub offset: Pixels,
}

/// Runtime state used by Liora active tooltip behavior.
pub struct ActiveTooltip(pub Vec<TooltipData>);
impl Global for ActiveTooltip {}

/// Updates the stored active tooltip value and keeps the existing component identity.
pub fn set_active_tooltip(data: TooltipData, cx: &mut App) {
    let tooltips = &mut cx.global_mut::<ActiveTooltip>().0;
    if let Some(existing) = tooltips.iter_mut().find(|tooltip| tooltip.id == data.id) {
        *existing = data;
    } else {
        tooltips.push(data);
    }
}

/// Clears the current tooltip state.
pub fn clear_tooltip(id: &SharedString, cx: &mut App) {
    cx.global_mut::<ActiveTooltip>()
        .0
        .retain(|tooltip| &tooltip.id != id);
}

/// Clears the current active tooltip state.
pub fn clear_active_tooltip(cx: &mut App) {
    cx.global_mut::<ActiveTooltip>().0.clear();
}

#[derive(Clone)]
/// Runtime state used by Liora active overlay entry behavior.
pub struct ActiveOverlayEntry {
    /// Stable identifier used for GPUI state, callbacks, and automation.
    pub id: SharedString,
    /// Entity view rendered inside a portal.
    pub view: gpui::AnyView,
}

/// Runtime state used by Liora active popover behavior.
pub struct ActivePopover(pub Vec<ActiveOverlayEntry>);
impl Global for ActivePopover {}

/// Returns whether popover active is currently true for this value.
pub fn is_popover_active(id: &SharedString, cx: &App) -> bool {
    cx.global::<ActivePopover>()
        .0
        .iter()
        .any(|entry| &entry.id == id)
}

/// Updates the stored active popover value and keeps the existing component identity.
pub fn set_active_popover(id: SharedString, view: gpui::AnyView, cx: &mut App) {
    let popovers = &mut cx.global_mut::<ActivePopover>().0;
    if let Some(existing) = popovers.iter_mut().find(|entry| entry.id == id) {
        existing.view = view;
    } else {
        popovers.push(ActiveOverlayEntry { id, view });
    }
}

/// Clears the current popover state.
pub fn clear_popover(id: &SharedString, cx: &mut App) {
    cx.global_mut::<ActivePopover>()
        .0
        .retain(|entry| &entry.id != id);
}

/// Clears the current active popover state.
pub fn clear_active_popover(cx: &mut App) {
    cx.global_mut::<ActivePopover>().0.clear();
}

/// Runtime state used by Liora active modal behavior.
pub struct ActiveModal(pub Vec<ActiveOverlayEntry>);
impl Global for ActiveModal {}

/// Updates the stored active modal value and keeps the existing component identity.
pub fn set_active_modal(id: SharedString, view: gpui::AnyView, cx: &mut App) {
    let modals = &mut cx.global_mut::<ActiveModal>().0;
    if let Some(existing) = modals.iter_mut().find(|entry| entry.id == id) {
        existing.view = view;
    } else {
        modals.push(ActiveOverlayEntry { id, view });
    }
}

/// Clears the current modal state.
pub fn clear_modal(id: &SharedString, cx: &mut App) {
    cx.global_mut::<ActiveModal>()
        .0
        .retain(|entry| &entry.id != id);
}

/// Clears the current active modal state.
pub fn clear_active_modal(cx: &mut App) {
    cx.global_mut::<ActiveModal>().0.clear();
}

/// Runtime state used by Liora active drawer behavior.
pub struct ActiveDrawer(pub Vec<ActiveOverlayEntry>);
impl Global for ActiveDrawer {}

/// Updates the stored active drawer value and keeps the existing component identity.
pub fn set_active_drawer(id: SharedString, view: gpui::AnyView, cx: &mut App) {
    let drawers = &mut cx.global_mut::<ActiveDrawer>().0;
    if let Some(existing) = drawers.iter_mut().find(|entry| entry.id == id) {
        existing.view = view;
    } else {
        drawers.push(ActiveOverlayEntry { id, view });
    }
}

/// Clears the current drawer state.
pub fn clear_drawer(id: &SharedString, cx: &mut App) {
    cx.global_mut::<ActiveDrawer>()
        .0
        .retain(|entry| &entry.id != id);
}

/// Clears the current active drawer state.
pub fn clear_active_drawer(cx: &mut App) {
    cx.global_mut::<ActiveDrawer>().0.clear();
}

/// Runtime state used by Liora popper behavior.
pub struct Popper {
    /// Bounds of the trigger element used for popper positioning.
    pub anchor_bounds: Bounds<Pixels>,
    /// Preferred placement relative to the trigger or anchor.
    pub placement: Placement,
    /// Pixel offset applied after popper placement is resolved.
    pub offset: Pixels,
}

impl Popper {
    /// Calculates the floating element origin for the configured anchor and placement.
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

    /// Calculates the floating element origin and flips placement when needed to fit the viewport.
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
