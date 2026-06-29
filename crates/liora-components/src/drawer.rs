//! Drawer module.
//!
//! This public module implements the Liora side drawer overlay component for secondary panels. It keeps the reusable
//! component logic inside `liora-components` rather than host applications so
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
//! the component, and avoid app-specific host-application resources in this SDK
//! crate.

use crate::gpui_compat::element_id;
use crate::motion::{fade_in, pop_in};
use gpui::{
    AnyElement, App, Context, IntoElement, KeyBinding, MouseButton, Pixels, Render, SharedString,
    Window, actions, div, prelude::*, px,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::sync::Arc;

actions!(
    drawer,
    [
        #[doc = "Keyboard action that closes the active drawer when dismissal is allowed."]
        DrawerClose
    ]
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control drawer placement behavior.
pub enum DrawerPlacement {
    #[default]
    /// Places the overlay to the right of the anchor.
    Right,
    /// Places the overlay to the left of the anchor.
    Left,
    /// Places the overlay above the anchor.
    Top,
    /// Places the overlay below the anchor.
    Bottom,
}

/// Fluent native GPUI component for rendering Liora drawer.
pub struct Drawer {
    id: SharedString,
    title: SharedString,
    content: Arc<dyn Fn(&mut Window, &mut Context<DrawerView>) -> AnyElement + 'static>,
    placement: DrawerPlacement,
    width: Pixels,
    height: Pixels,
    close_on_click_outside: bool,
    close_on_escape: bool,
}

/// Fluent native GPUI component for rendering Liora drawer view.
pub struct DrawerView {
    id: SharedString,
    title: SharedString,
    content: Arc<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement + 'static>,
    placement: DrawerPlacement,
    width: Pixels,
    height: Pixels,
    close_on_click_outside: bool,
    close_on_escape: bool,
    on_close: Arc<dyn Fn(&mut Window, &mut App) + 'static>,
}

impl DrawerView {
    fn new(
        id: SharedString,
        title: SharedString,
        content: Arc<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement + 'static>,
        placement: DrawerPlacement,
        width: Pixels,
        height: Pixels,
        close_on_click_outside: bool,
        close_on_escape: bool,
        on_close: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            id,
            title,
            content,
            placement,
            width,
            height,
            close_on_click_outside,
            close_on_escape,
            on_close: Arc::new(on_close),
        }
    }
}

impl Render for DrawerView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let id = self.id.clone();
        let title = self.title.clone();
        let content_fn = self.content.clone();
        let on_close = self.on_close.clone();
        let placement = self.placement;
        let width = self.width;
        let height = self.height;
        let close_on_click_outside = self.close_on_click_outside;
        let close_on_escape = self.close_on_escape;

        let mut container = div()
            .id(id.clone())
            .absolute()
            .size_full()
            .cursor_default()
            .bg(theme.neutral.overlay)
            .on_mouse_move(|_, _, cx| {
                cx.stop_propagation();
            })
            .when(close_on_click_outside, |s| {
                s.on_mouse_down(MouseButton::Left, {
                    let on_close = on_close.clone();
                    move |_, window, cx| {
                        on_close(window, cx);
                    }
                })
            })
            .when(close_on_escape, |s| {
                s.on_action(cx.listener({
                    let on_close = on_close.clone();
                    move |_, _action: &DrawerClose, window, cx| {
                        on_close(window, cx);
                    }
                }))
            });

        let mut panel = div()
            .bg(theme.neutral.card)
            .cursor_default()
            .shadow_xl()
            // CONSUME mouse down inside the panel so it doesn't trigger the overlay close
            .on_mouse_move(|_, _, cx| {
                cx.stop_propagation();
            })
            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                cx.stop_propagation();
            });

        match placement {
            DrawerPlacement::Left => {
                container = container.flex().flex_row().justify_start();
                panel = panel
                    .h_full()
                    .w(width)
                    .border_r_1()
                    .border_color(theme.neutral.border);
            }
            DrawerPlacement::Right => {
                container = container.flex().flex_row().justify_end();
                panel = panel
                    .h_full()
                    .w(width)
                    .border_l_1()
                    .border_color(theme.neutral.border);
            }
            DrawerPlacement::Top => {
                container = container.flex().flex_col().justify_start();
                panel = panel
                    .w_full()
                    .h(height)
                    .border_b_1()
                    .border_color(theme.neutral.border);
            }
            DrawerPlacement::Bottom => {
                container = container.flex().flex_col().justify_end();
                panel = panel
                    .w_full()
                    .h(height)
                    .border_t_1()
                    .border_color(theme.neutral.border);
            }
        }

        fade_in(
            element_id(format!("{id}-overlay-motion")),
            container.child(pop_in(
                element_id(format!("{id}-panel-motion")),
                panel
                    .child(
                        div()
                            .p_4()
                            .border_b_1()
                            .border_color(theme.neutral.border)
                            .flex()
                            .justify_between()
                            .items_center()
                            .child(div().font_weight(gpui::FontWeight::BOLD).child(title))
                            .child(
                                div()
                                    .id(element_id(format!("{id}-close-btn")))
                                    .cursor_pointer()
                                    .child(
                                        Icon::new(IconName::X)
                                            .size(px(16.0))
                                            .color(theme.neutral.icon),
                                    )
                                    .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                        on_close(window, cx);
                                    }),
                            ),
                    )
                    .child(div().flex_1().p_4().child(content_fn(_window, cx))),
            )),
        )
    }
}

impl Drawer {
    /// Registers GPUI key bindings required for keyboard interaction.
    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([KeyBinding::new("escape", DrawerClose, None)]);
    }

    /// Creates `Drawer` with default theme-driven styling and no optional callbacks attached.
    pub fn new() -> Self {
        Self {
            id: liora_core::unique_id("drawer"),
            title: SharedString::default(),
            content: Arc::new(|_, _| div().child("Drawer Content").into_any_element()),
            placement: DrawerPlacement::Right,
            width: px(300.0),
            height: px(300.0),
            close_on_click_outside: true,
            close_on_escape: true,
        }
    }

    /// Creates a lightweight sheet-style drawer for short contextual flows.
    ///
    /// This replaces the previous separate `Sheet` component: a sheet is just a
    /// Drawer with lighter default dimensions and the same overlay lifecycle.
    pub fn sheet() -> Self {
        Self::new()
            .id(liora_core::unique_id("drawer-sheet"))
            .width(px(360.0))
            .height(px(260.0))
    }

    /// Places the drawer on the left edge.
    pub fn left(self) -> Self {
        self.placement(DrawerPlacement::Left)
    }

    /// Places the drawer on the right edge.
    pub fn right(self) -> Self {
        self.placement(DrawerPlacement::Right)
    }

    /// Places the drawer on the top edge.
    pub fn top(self) -> Self {
        self.placement(DrawerPlacement::Top)
    }

    /// Places the drawer on the bottom edge.
    pub fn bottom(self) -> Self {
        self.placement(DrawerPlacement::Bottom)
    }

    /// Applies the former sheet wide-inspector preset.
    pub fn sheet_width_lg(self) -> Self {
        self.width(px(440.0))
    }

    /// Applies the former compact bottom-sheet height preset.
    pub fn sheet_height_sm(self) -> Self {
        self.height(px(220.0))
    }

    /// Sets content that only needs the current window, matching lightweight sheet use cases.
    pub fn content_view<F, E>(mut self, f: F) -> Self
    where
        F: Fn(&mut Window) -> E + 'static,
        E: IntoElement,
    {
        self.content = Arc::new(move |window, _cx| f(window).into_any_element());
        self
    }

    /// Assigns a stable element id used by GPUI state, hit testing, and automated interaction tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Sets the primary title text displayed by the component.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = title.into();
        self
    }

    /// Selects the popup, label, or overlay placement.
    pub fn placement(mut self, p: DrawerPlacement) -> Self {
        self.placement = p;
        self
    }

    /// Sets the component width token used during GPUI layout.
    pub fn width(mut self, w: impl Into<Pixels>) -> Self {
        self.width = w.into();
        self
    }

    /// Applies the predefined width lg sizing preset.
    pub fn width_lg(self) -> Self {
        self.width(px(480.0))
    }

    /// Sets the component height token used during GPUI layout.
    pub fn height(mut self, h: impl Into<Pixels>) -> Self {
        self.height = h.into();
        self
    }

    /// Applies the predefined height sm sizing preset.
    pub fn height_sm(self) -> Self {
        self.height(px(200.0))
    }

    /// Applies the predefined height lg sizing preset.
    pub fn height_lg(self) -> Self {
        self.height(px(360.0))
    }

    /// Toggles whether the popup closes when click outside occurs.
    pub fn close_on_click_outside(mut self, c: bool) -> Self {
        self.close_on_click_outside = c;
        self
    }

    /// Toggles whether the popup closes when escape occurs.
    pub fn close_on_escape(mut self, c: bool) -> Self {
        self.close_on_escape = c;
        self
    }

    /// Sets the rendered content element or text for this component.
    pub fn content<F, E>(mut self, f: F) -> Self
    where
        F: Fn(&mut Window, &mut Context<DrawerView>) -> E + 'static,
        E: IntoElement,
    {
        self.content = Arc::new(move |window, cx| f(window, cx).into_any_element());
        self
    }

    /// Performs the show operation used by this component.
    pub fn show(self, cx: &mut App) {
        let id = self.id;
        let title = self.title;
        let content = self.content;
        let placement = self.placement;
        let width = self.width;
        let height = self.height;
        let close_on_click_outside = self.close_on_click_outside;
        let close_on_escape = self.close_on_escape;

        let id_for_close = id.clone();
        let view = cx.new(|_cx| {
            DrawerView::new(
                id.clone(),
                title,
                content,
                placement,
                width,
                height,
                close_on_click_outside,
                close_on_escape,
                move |_window, _cx| {
                    liora_core::clear_drawer(&id_for_close, _cx);
                },
            )
        });

        liora_core::set_active_drawer(id, view.into(), cx);
    }

    /// Performs the close operation used by this component.
    pub fn close(cx: &mut App) {
        liora_core::clear_active_drawer(cx);
    }

    /// Performs the close id operation used by this component.
    pub fn close_id(id: impl Into<SharedString>, cx: &mut App) {
        let id = id.into();
        liora_core::clear_drawer(&id, cx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drawer_size_helpers_set_demo_sizes() {
        assert_eq!(Drawer::new().width_lg().width, px(480.0));
        assert_eq!(Drawer::new().height_sm().height, px(200.0));
        assert_eq!(Drawer::new().height_lg().height, px(360.0));
    }

    #[test]
    fn drawer_includes_lightweight_sheet_presets() {
        let sheet = Drawer::sheet().left().sheet_width_lg().sheet_height_sm();
        assert_eq!(sheet.placement, DrawerPlacement::Left);
        assert_eq!(sheet.width, px(440.0));
        assert_eq!(sheet.height, px(220.0));
    }

    #[test]
    fn drawer_uses_liora_motion_on_overlay_and_panel() {
        let source = include_str!("drawer.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains("fade_in("));
        assert!(source.contains("pop_in("));
        assert!(source.contains("panel-motion"));
    }
}
