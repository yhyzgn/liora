use aura_core::{Placement, TooltipData, clear_tooltip, set_active_tooltip};
use gpui::{
    AnyElement, App, Bounds, Component, ElementId, GlobalElementId, InspectorElementId,
    IntoElement, LayoutId, Pixels, RenderOnce, SharedString, Window, div, prelude::*, px,
};
use std::cell::Cell;
use std::rc::Rc;

pub struct Tooltip {
    trigger: AnyElement,
    content: SharedString,
    placement: Placement,
    offset: Pixels,
    id: SharedString,
}

impl Tooltip {
    #[track_caller]
    pub fn new(trigger: impl IntoElement) -> Self {
        let caller = std::panic::Location::caller();
        Self {
            trigger: trigger.into_any_element(),
            content: SharedString::default(),
            placement: Placement::Top,
            offset: px(8.0),
            id: format!("tooltip-{}", caller).into(),
        }
    }

    pub fn content(mut self, content: impl Into<SharedString>) -> Self {
        self.content = content.into();
        self
    }

    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }

    pub fn offset(mut self, offset: impl Into<Pixels>) -> Self {
        self.offset = offset.into();
        self
    }

    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }
}

impl RenderOnce for Tooltip {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let content = self.content.clone();
        let placement = self.placement;
        let offset = self.offset;
        let id = self.id.clone();

        let bounds_cell = Rc::new(Cell::new(Bounds::default()));
        let bounds_cell_clone = bounds_cell.clone();
        let hover_id = id.clone();
        let move_id = id.clone();

        div()
            .id(id.clone())
            .child(TooltipBoundsTracker {
                trigger: self.trigger,
                bounds: bounds_cell,
            })
            .on_hover(move |hovered, _window, cx| {
                if !hovered {
                    clear_tooltip(&hover_id, cx);
                }
            })
            .on_mouse_move(move |_event, _window, cx| {
                let anchor_bounds = bounds_cell_clone.get();
                if anchor_bounds.size.width > px(0.0) {
                    set_active_tooltip(
                        TooltipData {
                            id: move_id.clone(),
                            content: content.clone(),
                            anchor_bounds,
                            placement,
                            offset,
                        },
                        cx,
                    );
                }
            })
    }
}

impl IntoElement for Tooltip {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

struct TooltipBoundsTracker {
    trigger: AnyElement,
    bounds: Rc<Cell<Bounds<Pixels>>>,
}

impl IntoElement for TooltipBoundsTracker {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}

impl gpui::Element for TooltipBoundsTracker {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _id2: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        (self.trigger.request_layout(window, cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _id2: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _rl: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) -> () {
        self.trigger.prepaint(window, cx);
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _id2: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _rl: &mut (),
        _ps: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        self.bounds.set(bounds);
        self.trigger.paint(window, cx);
    }
}
