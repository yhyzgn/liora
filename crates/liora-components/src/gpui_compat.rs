//! Gpui Compat module.
//!
//! This crate-internal module implements the Liora small compatibility helpers that isolate GPUI API differences. It keeps the reusable
//! component logic inside `liora-components` rather than Gallery or Docs so
//! downstream GPUI applications can compose the same behavior with their own
//! app state, assets, and release policy.
//!
//! ## Usage model
//!
//! Components in this module render native GPUI element trees. Stateless builder
//! values can be constructed inline, while controls with focus, selection,
//! popup, drag, or editing state should be stored as `gpui::Entity<T>` fields in
//! the parent view so state survives GPUI render passes.
//!
//! ## Design contract
//!
//! The implementation should use Liora theme tokens from `liora-core` and
//! `liora-theme`, keep accessibility-oriented keyboard/pointer behavior close to
//! the component, and avoid app-specific Gallery/Docs resources in this SDK
//! crate.

use gpui::{App, ElementId, FocusHandle, Hsla, Pixels, Point, SharedString, Window};

/// Corner/anchor type used by `deferred`, named differently across GPUI releases.
#[cfg(liora_gpui_latest_api)]
pub type AnchorCorner = gpui::Anchor;

/// Corner/anchor type used by `deferred`, named differently across GPUI releases.
#[cfg(not(liora_gpui_latest_api))]
pub type AnchorCorner = gpui::Corner;

/// Returns a stable GPUI element id from a string-like value.
pub fn element_id(id: impl Into<SharedString>) -> ElementId {
    ElementId::from(id.into())
}

/// Extension trait retained so chart code can call `as_f32()` when it is
/// compiled against older official crates.io GPUI versions that did not expose
/// an inherent `Pixels::as_f32` method yet. Newer official Zed GPUI resolves to
/// the inherent method first.
#[allow(dead_code)]
pub trait PixelsExt {
    /// Returns the raw scalar value represented by this pixel unit.
    fn as_f32(self) -> f32;
}

impl PixelsExt for Pixels {
    fn as_f32(self) -> f32 {
        f32::from(self)
    }
}

/// Returns the raw pixel value across GPUI releases.
#[allow(dead_code)]
pub fn px_f32(value: Pixels) -> f32 {
    #[cfg(liora_gpui_latest_api)]
    {
        value.as_f32()
    }
    #[cfg(not(liora_gpui_latest_api))]
    {
        f32::from(value)
    }
}

/// Focuses a GPUI focus handle across the old and current official signatures.
pub fn focus_window(window: &mut Window, focus_handle: &FocusHandle, cx: &mut App) {
    #[cfg(liora_gpui_latest_api)]
    {
        window.focus(focus_handle, cx);
    }
    #[cfg(not(liora_gpui_latest_api))]
    {
        let _ = cx;
        window.focus(focus_handle);
    }
}

/// Creates a child element id across GPUI's Box-to-Arc storage change.
pub fn named_child_id(id: ElementId, child: impl Into<SharedString>) -> ElementId {
    #[cfg(liora_gpui_latest_api)]
    {
        ElementId::NamedChild(std::sync::Arc::new(id), child.into())
    }
    #[cfg(not(liora_gpui_latest_api))]
    {
        ElementId::NamedChild(Box::new(id), child.into())
    }
}

/// Creates a box shadow without leaking GPUI's `inset` field addition.
pub fn box_shadow(
    color: Hsla,
    offset: Point<Pixels>,
    blur_radius: Pixels,
    spread_radius: Pixels,
) -> gpui::BoxShadow {
    #[cfg(liora_gpui_latest_api)]
    {
        gpui::BoxShadow {
            color,
            offset,
            blur_radius,
            spread_radius,
            inset: false,
        }
    }
    #[cfg(not(liora_gpui_latest_api))]
    {
        gpui::BoxShadow {
            color,
            offset,
            blur_radius,
            spread_radius,
        }
    }
}

/// Paints a shaped line while preserving alignment on newer GPUI and using the
/// older default-alignment API when publishing against crates.io GPUI 0.2.2.
pub fn paint_shaped_line(
    line: &gpui::ShapedLine,
    origin: Point<Pixels>,
    line_height: Pixels,
    text_align: gpui::TextAlign,
    align_width: Option<Pixels>,
    window: &mut Window,
    cx: &mut App,
) -> gpui::Result<()> {
    #[cfg(liora_gpui_latest_api)]
    {
        line.paint(origin, line_height, text_align, align_width, window, cx)
    }
    #[cfg(not(liora_gpui_latest_api))]
    {
        let _ = text_align;
        let _ = align_width;
        line.paint(origin, line_height, window, cx)
    }
}

/// Returns the vertical maximum scrollbar offset across GPUI's Size-to-Point API change.
pub fn scrollbar_max_offset_y(list_state: &gpui::ListState) -> Pixels {
    let max_offset = list_state.max_offset_for_scrollbar();
    #[cfg(liora_gpui_latest_api)]
    {
        max_offset.y
    }
    #[cfg(not(liora_gpui_latest_api))]
    {
        max_offset.height
    }
}

/// Extension trait that preserves the old no-argument GPUI `flex_shrink` API
/// and the current numeric-factor API behind one call site.
pub trait FlexShrinkCompat: Sized {
    /// Applies a normal shrink factor of `1.0`.
    fn shrink_one(self) -> Self;
}

impl<T> FlexShrinkCompat for T
where
    T: gpui::Styled,
{
    fn shrink_one(self) -> Self {
        #[cfg(liora_gpui_latest_api)]
        {
            self.flex_shrink(1.0)
        }
        #[cfg(not(liora_gpui_latest_api))]
        {
            self.flex_shrink()
        }
    }
}

/// Returns a vertical offset from GPUI's Size-to-Point scroll handle API change.
pub fn scroll_handle_max_offset_y(scroll_handle: &gpui::ScrollHandle) -> Pixels {
    let max_offset = scroll_handle.max_offset();
    #[cfg(liora_gpui_latest_api)]
    {
        max_offset.y
    }
    #[cfg(not(liora_gpui_latest_api))]
    {
        max_offset.height
    }
}

/// Paints a wrapped line across GPUI's aligned wrapped-line API shape.
pub fn paint_wrapped_line(
    line: &gpui::WrappedLine,
    origin: Point<Pixels>,
    line_height: Pixels,
    text_align: gpui::TextAlign,
    bounds: gpui::Bounds<Pixels>,
    window: &mut Window,
    cx: &mut App,
) -> gpui::Result<()> {
    #[cfg(liora_gpui_latest_api)]
    {
        line.paint(origin, line_height, text_align, Some(bounds), window, cx)
    }
    #[cfg(not(liora_gpui_latest_api))]
    {
        line.paint(origin, line_height, text_align, Some(bounds), window, cx)
    }
}
