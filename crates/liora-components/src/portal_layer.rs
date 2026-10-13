//! Top-level portal host for Drawer, Modal, Popover, and other overlays.

use gpui::{App, IntoElement, RenderOnce, ViewElement as Component, Window, div, prelude::*};
use liora_core::{PassivePortal, Portal};

/// Consumes Liora's global portal queues and paints them above application content.
pub struct PortalLayer;

impl IntoElement for PortalLayer {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl RenderOnce for PortalLayer {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let passive = if cx.has_global::<PassivePortal>() {
            std::mem::take(&mut cx.global_mut::<PassivePortal>().entries)
        } else {
            Vec::new()
        };
        let active = if cx.has_global::<Portal>() {
            std::mem::take(&mut cx.global_mut::<Portal>().entries)
        } else {
            Vec::new()
        };
        let mut root = div().absolute().top_0().left_0().size_full();
        if !passive.is_empty() {
            let mut layer = div()
                .id("liora-passive-portal-layer")
                .absolute()
                .top_0()
                .left_0()
                .size_full();
            for entry in passive {
                layer = layer.child((entry.render)(window, cx));
            }
            root = root.child(layer);
        }
        if !active.is_empty() {
            let mut layer = div()
                .id("liora-portal-layer")
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .occlude();
            for entry in active {
                layer = layer.child((entry.render)(window, cx));
            }
            root = root.child(layer);
        }
        root
    }
}
