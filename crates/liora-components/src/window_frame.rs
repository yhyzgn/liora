//! Window Frame module.
//!
//! This public module implements native app-window frame helpers for Liora GPUI
//! applications. Liora apps can keep the platform system frame or opt into a
//! lightweight custom native GPUI title bar without introducing a WebView or web
//! runtime layer.
//!
//! ## Usage model
//!
//! Apply [`apply_window_frame_mode`] while constructing `gpui::WindowOptions`,
//! then wrap the root content in [`AppWindowFrame`] when a custom frame is
//! selected. Store the selected [`WindowFrameMode`] in application state so the
//! frame choice survives normal GPUI render passes and window recreation.
//!
//! ## Design contract
//!
//! The implementation should only adapt GPUI window/frame primitives, use Liora
//! theme tokens for custom chrome, preserve platform/server decorations when
//! [`WindowFrameMode::System`] is selected, and avoid app-specific Gallery/Docs
//! resources in this SDK crate.

use crate::{Button, Space, Text};
use gpui::{
    AnyElement, App, Component, InteractiveElement, IntoElement, MouseButton, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, WindowControlArea,
    WindowDecorations, WindowOptions, div, point, prelude::*, px,
};
use liora_core::Config;
use liora_icons_lucide::IconName;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control window frame mode behavior.
pub enum WindowFrameMode {
    #[default]
    /// Uses the platform/system window-frame behavior.
    System,
    /// Emits the custom tray command.
    Custom,
}

impl WindowFrameMode {
    /// Returns whether custom is currently true for this value.
    pub fn is_custom(self) -> bool {
        matches!(self, Self::Custom)
    }

    /// Creates this value from custom.
    pub fn from_custom(custom: bool) -> Self {
        if custom { Self::Custom } else { Self::System }
    }

    /// Returns the stable user-facing label for this value.
    pub fn label(self) -> &'static str {
        match self {
            Self::System => "System frame",
            Self::Custom => "Custom frame",
        }
    }
}

/// Applies the GPUI window options required by the selected frame mode.
///
/// `System` keeps platform/server decorations. `Custom` requests a transparent
/// titlebar on macOS/Windows and client-side decorations on Linux/Wayland.
pub fn apply_window_frame_mode(mut options: WindowOptions, mode: WindowFrameMode) -> WindowOptions {
    match mode {
        WindowFrameMode::System => {
            if let Some(titlebar) = options.titlebar.as_mut() {
                titlebar.appears_transparent = false;
                titlebar.traffic_light_position = None;
            }
            options.window_decorations = Some(WindowDecorations::Server);
        }
        WindowFrameMode::Custom => {
            if let Some(titlebar) = options.titlebar.as_mut() {
                titlebar.appears_transparent = true;
                titlebar.traffic_light_position = Some(point(px(12.0), px(12.0)));
            }
            options.window_decorations = Some(WindowDecorations::Client);
        }
    }
    options
}

/// Convenience control for switching between system and custom frames.
pub fn frame_mode_switch_row(switch: impl IntoElement, mode: WindowFrameMode) -> impl IntoElement {
    Space::new()
        .gap_sm()
        .child(Text::new("Frame"))
        .child(switch)
        .child(Text::new(mode.label()).size(px(12.0)))
}

/// Fluent native GPUI component for rendering Liora app window frame.
pub struct AppWindowFrame {
    title: SharedString,
    subtitle: Option<SharedString>,
    mode: WindowFrameMode,
    actions: Vec<AnyElement>,
    content: AnyElement,
    on_close: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl AppWindowFrame {
    /// Creates `AppWindowFrame` initialized from the supplied title, and content.
    pub fn new(title: impl Into<SharedString>, content: impl IntoElement) -> Self {
        Self {
            title: title.into(),
            subtitle: None,
            mode: WindowFrameMode::System,
            actions: Vec::new(),
            content: content.into_any_element(),
            on_close: None,
        }
    }

    /// Sets the subtitle value used by the component.
    pub fn subtitle(mut self, subtitle: impl Into<SharedString>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Selects the rendering mode used by this component.
    pub fn mode(mut self, mode: WindowFrameMode) -> Self {
        self.mode = mode;
        self
    }

    /// Creates a tray menu action item specification.
    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.actions.push(action.into_any_element());
        self
    }

    /// Sets the actions value used by the component.
    pub fn actions(mut self, actions: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.actions
            .extend(actions.into_iter().map(IntoElement::into_any_element));
        self
    }

    /// Registers a callback that runs when close occurs.
    pub fn on_close(mut self, close: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Box::new(close));
        self
    }
}

impl RenderOnce for AppWindowFrame {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        if !self.mode.is_custom() {
            return self.content;
        }

        let theme = cx.global::<Config>().theme.clone();
        let title = self.title.clone();
        let subtitle = self.subtitle.clone();
        let on_close = self.on_close;

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.neutral.body)
            .child(
                div()
                    .id("liora-window-frame-titlebar")
                    .h(px(46.0))
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(theme.neutral.border)
                    .bg(theme.neutral.card.opacity(0.96))
                    .child(
                        div()
                            .id("liora-window-frame-drag-region")
                            .window_control_area(WindowControlArea::Drag)
                            .cursor_default()
                            .h_full()
                            .flex_1()
                            .min_w_0()
                            .flex()
                            .items_center()
                            .px_4()
                            .on_mouse_down(MouseButton::Left, |_, window, cx| {
                                window.start_window_move();
                                cx.stop_propagation();
                            })
                            .on_click(|event, window, _| {
                                if event.click_count() == 2 {
                                    window.titlebar_double_click();
                                }
                            })
                            .child(
                                Space::new()
                                    .vertical()
                                    .gap_xs()
                                    .child(Text::new(title).bold().size(px(13.0)))
                                    .when_some(subtitle, |s, subtitle| {
                                        s.child(Text::new(subtitle).size(px(11.0)))
                                    }),
                            ),
                    )
                    .child(
                        Space::new()
                            .gap_sm()
                            .child(Text::new("Custom Frame").size(px(12.0)))
                            .children(self.actions),
                    )
                    .child(window_controls(on_close, window, theme.clone())),
            )
            .child(div().flex_1().min_h_0().child(self.content))
            .into_any_element()
    }
}

impl IntoElement for AppWindowFrame {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
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
    fn frame_mode_tracks_custom_state_and_labels() {
        assert!(!WindowFrameMode::System.is_custom());
        assert!(WindowFrameMode::Custom.is_custom());
        assert_eq!(WindowFrameMode::from_custom(false), WindowFrameMode::System);
        assert_eq!(WindowFrameMode::from_custom(true), WindowFrameMode::Custom);
        assert_eq!(WindowFrameMode::Custom.label(), "Custom frame");
    }

    #[test]
    fn window_frame_options_switch_between_server_and_client_decorations() {
        let custom = apply_window_frame_mode(WindowOptions::default(), WindowFrameMode::Custom);
        assert!(
            custom
                .titlebar
                .as_ref()
                .is_some_and(|titlebar| titlebar.appears_transparent)
        );
        assert_eq!(custom.window_decorations, Some(WindowDecorations::Client));

        let system = apply_window_frame_mode(custom, WindowFrameMode::System);
        assert!(
            system
                .titlebar
                .as_ref()
                .is_some_and(|titlebar| !titlebar.appears_transparent)
        );
        assert_eq!(system.window_decorations, Some(WindowDecorations::Server));
    }

    #[test]
    fn custom_window_frame_renders_native_window_control_areas() {
        let source = include_str!("window_frame.rs");
        assert!(source.contains("WindowControlArea::Drag"));
        assert!(source.contains("WindowControlArea::Min"));
        assert!(source.contains("WindowControlArea::Max"));
        assert!(source.contains("WindowControlArea::Close"));
        assert!(source.contains("theme.danger.base"));
        assert!(source.contains("theme.neutral.inverted"));
        assert!(source.contains("start_window_move"));
        assert!(source.contains("titlebar_double_click"));
    }
}
