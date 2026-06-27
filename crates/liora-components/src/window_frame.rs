//! Window Frame module.
//!
//! This public module implements native app-window frame helpers for Liora GPUI
//! applications. Liora apps can keep the platform system frame or opt into a
//! lightweight custom native GPUI titlebar without introducing a WebView or web
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

use crate::{Space, Text, TitleBar};
use gpui::{
    AnyElement, App, Component, IntoElement, ParentElement, RenderOnce, SharedString, Styled,
    Window, WindowDecorations, WindowOptions, div, point, px,
};
use liora_core::Config;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control window frame mode behavior.
pub enum WindowFrameMode {
    #[default]
    /// Uses the platform/system window-frame behavior.
    System,
    /// Uses Liora's custom native titlebar and client-side window controls.
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

/// Requests the runtime GPUI decoration mode for an already-open window.
///
/// This avoids destroying and recreating the current window from inside a UI
/// control event handler. Reopening during the same event dispatch can leave
/// GPUI with a removed window while the original event/render chain is still
/// unwinding. Apps should still call [`apply_window_frame_mode`] when opening
/// the first window so the initial platform window is created with matching
/// decorations.
pub fn request_window_frame_mode(window: &mut Window, mode: WindowFrameMode) {
    let decorations = match mode {
        WindowFrameMode::System => WindowDecorations::Server,
        WindowFrameMode::Custom => WindowDecorations::Client,
    };
    window.request_decorations(decorations);
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
    titlebar: TitleBar,
    mode: WindowFrameMode,
    content: AnyElement,
}

impl AppWindowFrame {
    /// Creates `AppWindowFrame` initialized from the supplied title and content.
    pub fn new(title: impl Into<SharedString>, content: impl IntoElement) -> Self {
        Self {
            titlebar: TitleBar::new().title(title),
            mode: WindowFrameMode::System,
            content: content.into_any_element(),
        }
    }

    /// Replaces the full titlebar configuration used in custom frame mode.
    pub fn titlebar(mut self, titlebar: TitleBar) -> Self {
        self.titlebar = titlebar;
        self
    }

    /// Sets the subtitle value used by the component.
    pub fn subtitle(mut self, subtitle: impl Into<SharedString>) -> Self {
        self.titlebar = self.titlebar.subtitle(subtitle);
        self
    }

    /// Selects the rendering mode used by this component.
    pub fn mode(mut self, mode: WindowFrameMode) -> Self {
        self.mode = mode;
        self
    }

    /// Adds a titlebar action item.
    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.titlebar = self.titlebar.action(action);
        self
    }

    /// Adds titlebar action items.
    pub fn actions(mut self, actions: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.titlebar = self.titlebar.actions(actions);
        self
    }

    /// Registers a callback that runs when the custom close control is clicked.
    pub fn on_close(mut self, close: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.titlebar = self.titlebar.on_close(close);
        self
    }
}

impl RenderOnce for AppWindowFrame {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        if !self.mode.is_custom() {
            return self.content;
        }

        let theme = cx.global::<Config>().theme.clone();
        let titlebar = self.titlebar.render(window, cx);

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.neutral.body)
            .child(titlebar)
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
        let production = include_str!("window_frame.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();
        assert!(production.contains("pub fn request_window_frame_mode"));
        assert!(production.contains("window.request_decorations(decorations)"));
    }

    #[test]
    fn custom_window_frame_renders_native_window_control_areas() {
        let frame_source = include_str!("window_frame.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();
        let titlebar_source = include_str!("titlebar.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(frame_source.contains("TitleBar::new"));
        assert!(frame_source.contains("titlebar.render"));
        assert!(titlebar_source.contains("WindowControlArea::Drag"));
        assert!(titlebar_source.contains("WindowControlArea::Min"));
        assert!(titlebar_source.contains("WindowControlArea::Max"));
        assert!(titlebar_source.contains("WindowControlArea::Close"));
        assert!(titlebar_source.contains("theme.danger.base"));
        assert!(titlebar_source.contains("theme.neutral.inverted"));
        assert!(titlebar_source.contains("start_window_move"));
        assert!(titlebar_source.contains("titlebar_double_click"));
    }
}
