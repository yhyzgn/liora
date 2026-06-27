//! Titlebar module.
//!
//! This public module implements reusable native GPUI titlebar chrome for Liora
//! applications. It keeps draggable regions, window-control areas, title text,
//! and app-provided actions in one SDK component so apps do not duplicate
//! platform-sensitive titlebar behavior.

use crate::{Button, Space, Text};
use gpui::{
    AnyElement, App, Component, InteractiveElement, IntoElement, MouseButton, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, WindowControlArea, div,
    prelude::*, px,
};
use liora_core::Config;
use liora_icons_lucide::IconName;

/// Visual density and border treatment used by [`TitleBar`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TitleBarVariant {
    /// Standard app titlebar with the normal height and bottom border.
    #[default]
    Custom,
    /// Compact app titlebar for dense utility windows.
    Compact,
    /// Titlebar without a visible bottom border.
    Borderless,
}

/// Fluent native GPUI component for rendering reusable app titlebars.
pub struct TitleBar {
    id: SharedString,
    title: Option<SharedString>,
    subtitle: Option<SharedString>,
    icon: Option<AnyElement>,
    leading: Vec<AnyElement>,
    center: Option<AnyElement>,
    actions: Vec<AnyElement>,
    show_window_controls: bool,
    draggable: bool,
    height: gpui::Pixels,
    variant: TitleBarVariant,
    on_close: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl TitleBar {
    /// Creates a standard draggable titlebar with native window controls enabled.
    pub fn new() -> Self {
        Self {
            id: "liora-titlebar".into(),
            title: None,
            subtitle: None,
            icon: None,
            leading: Vec::new(),
            center: None,
            actions: Vec::new(),
            show_window_controls: true,
            draggable: true,
            height: px(46.0),
            variant: TitleBarVariant::Custom,
            on_close: None,
        }
    }

    /// Assigns a stable element id used by GPUI state and automation tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Sets the primary title text.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the secondary title text.
    pub fn subtitle(mut self, subtitle: impl Into<SharedString>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Sets an app-provided icon rendered before the title text.
    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_any_element());
        self
    }

    /// Adds an element before the title group.
    pub fn leading(mut self, element: impl IntoElement) -> Self {
        self.leading.push(element.into_any_element());
        self
    }

    /// Sets custom center content. When absent, the title group occupies the drag region.
    pub fn center(mut self, element: impl IntoElement) -> Self {
        self.center = Some(element.into_any_element());
        self
    }

    /// Adds a right-side action before the window controls.
    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.actions.push(action.into_any_element());
        self
    }

    /// Adds multiple right-side actions before the window controls.
    pub fn actions(mut self, actions: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.actions
            .extend(actions.into_iter().map(IntoElement::into_any_element));
        self
    }

    /// Sets whether the titlebar renders minimize, maximize, and close controls.
    pub fn window_controls(mut self, show: bool) -> Self {
        self.show_window_controls = show;
        self
    }

    /// Sets whether the main titlebar surface starts native window move operations.
    pub fn draggable(mut self, draggable: bool) -> Self {
        self.draggable = draggable;
        self
    }

    /// Sets the titlebar height.
    pub fn height(mut self, height: impl Into<gpui::Pixels>) -> Self {
        self.height = height.into();
        self
    }

    /// Uses the compact titlebar preset.
    pub fn compact(mut self) -> Self {
        self.variant = TitleBarVariant::Compact;
        self.height = px(38.0);
        self
    }

    /// Uses the borderless titlebar preset.
    pub fn borderless(mut self) -> Self {
        self.variant = TitleBarVariant::Borderless;
        self
    }

    /// Registers a callback that runs when the close control is clicked.
    pub fn on_close(mut self, close: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Box::new(close));
        self
    }

    /// Renders the titlebar as a native GPUI element tree.
    pub fn render(self, window: &mut Window, cx: &mut App) -> AnyElement {
        let theme = cx.global::<Config>().theme.clone();
        let title_group = title_group(self.icon, self.title, self.subtitle);
        let center = self.center;
        let actions = self.actions;
        let on_close = self.on_close;
        let draggable = self.draggable;
        let compact = matches!(self.variant, TitleBarVariant::Compact);
        let borderless = matches!(self.variant, TitleBarVariant::Borderless);

        div()
            .id(self.id)
            .h(self.height)
            .w_full()
            .flex()
            .items_center()
            .justify_between()
            .when(!borderless, |s| {
                s.border_b_1().border_color(theme.neutral.border)
            })
            .bg(theme.neutral.card.opacity(0.96))
            .child(
                div()
                    .id("liora-titlebar-drag-region")
                    .when(draggable, |s| {
                        s.window_control_area(WindowControlArea::Drag)
                            .cursor_default()
                            .on_mouse_down(MouseButton::Left, |_, window, cx| {
                                window.start_window_move();
                                cx.stop_propagation();
                            })
                            .on_click(|event, window, _| {
                                if event.click_count() == 2 {
                                    window.titlebar_double_click();
                                }
                            })
                    })
                    .h_full()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .items_center()
                    .gap_2()
                    .when(compact, |s| s.px_3())
                    .when(!compact, |s| s.px_4())
                    .children(self.leading)
                    .child(center.unwrap_or(title_group)),
            )
            .child(
                Space::new()
                    .gap_sm()
                    .children(actions)
                    .when(self.show_window_controls, |s| {
                        s.child(window_controls(on_close, window, theme.clone()))
                    }),
            )
            .into_any_element()
    }
}

impl Default for TitleBar {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for TitleBar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.render(window, cx)
    }
}

impl IntoElement for TitleBar {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

fn title_group(
    icon: Option<AnyElement>,
    title: Option<SharedString>,
    subtitle: Option<SharedString>,
) -> AnyElement {
    Space::new()
        .gap_sm()
        .child(icon.unwrap_or_else(|| div().into_any_element()))
        .child(
            Space::new()
                .vertical()
                .gap_xs()
                .when_some(title, |s, title| {
                    s.child(Text::new(title).bold().size(px(13.0)))
                })
                .when_some(subtitle, |s, subtitle| {
                    s.child(Text::new(subtitle).size(px(11.0)))
                }),
        )
        .into_any_element()
}

fn window_controls(
    on_close: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
    window: &mut Window,
    theme: liora_theme::Theme,
) -> impl IntoElement {
    Space::new()
        .gap_xs()
        .child(frame_control_button(
            "liora-window-frame-minimize",
            IconName::Minus,
            WindowControlArea::Min,
            false,
            theme.clone(),
            |window, _| window.minimize_window(),
        ))
        .child(frame_control_button(
            "liora-window-frame-maximize",
            if window.is_maximized() {
                IconName::Minimize2
            } else {
                IconName::Maximize2
            },
            WindowControlArea::Max,
            false,
            theme.clone(),
            |window, _| window.zoom_window(),
        ))
        .child(frame_control_button(
            "liora-window-frame-close",
            IconName::X,
            WindowControlArea::Close,
            true,
            theme.clone(),
            move |window, cx| {
                if let Some(close) = on_close.as_ref() {
                    close(window, cx);
                } else {
                    window.remove_window();
                }
            },
        ))
        .into_any_element()
}

fn frame_control_button(
    id: &'static str,
    icon: IconName,
    control_area: WindowControlArea,
    danger: bool,
    theme: liora_theme::Theme,
    on_click: impl Fn(&mut Window, &mut App) + 'static,
) -> impl IntoElement {
    Button::new("")
        .id(id)
        .text()
        .small()
        .icon_only(icon)
        .on_click(move |_, window, cx| on_click(window, cx))
        .into_element()
        .map(move |button| {
            div()
                .window_control_area(control_area)
                .rounded(px(8.0))
                .when(danger, |s| {
                    s.hover(move |s| s.bg(theme.danger.base).text_color(theme.neutral.inverted))
                })
                .child(button)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn titlebar_builder_tracks_core_options() {
        let titlebar = TitleBar::new()
            .id("test-titlebar")
            .title("Liora")
            .subtitle("Shell")
            .window_controls(false)
            .draggable(false)
            .compact();

        assert_eq!(titlebar.id.as_ref(), "test-titlebar");
        assert_eq!(titlebar.title.as_deref(), Some("Liora"));
        assert_eq!(titlebar.subtitle.as_deref(), Some("Shell"));
        assert!(!titlebar.show_window_controls);
        assert!(!titlebar.draggable);
        assert_eq!(titlebar.height, px(38.0));
        assert_eq!(titlebar.variant, TitleBarVariant::Compact);
    }

    #[test]
    fn titlebar_source_owns_native_window_control_areas() {
        let production = include_str!("titlebar.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(production.contains("WindowControlArea::Drag"));
        assert!(production.contains("WindowControlArea::Min"));
        assert!(production.contains("WindowControlArea::Max"));
        assert!(production.contains("WindowControlArea::Close"));
        assert!(production.contains("start_window_move"));
        assert!(production.contains("titlebar_double_click"));
        assert!(production.contains("theme.danger.base"));
        assert!(production.contains("theme.neutral.inverted"));
    }
}
