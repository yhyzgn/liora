//! Otp Input module.
//!
//! This public module implements the Liora one-time-passcode display/input scaffold component. It keeps the reusable
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

use gpui::{
    App, Component, Hsla, IntoElement, Pixels, RenderOnce, SharedString, Window, div, prelude::*,
    px,
};
use liora_core::Config;

/// Visual state for one-time-passcode cells.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OtpInputStatus {
    /// Neutral/default state.
    Default,
    /// Success/verified state.
    Success,
    /// Warning state.
    Warning,
    /// Error/invalid state.
    Error,
}

/// Fluent native GPUI component for rendering OTP/PIN entry cells.
pub struct OtpInput {
    value: SharedString,
    length: usize,
    gap: Pixels,
    cell_size: Pixels,
    masked: bool,
    disabled: bool,
    status: OtpInputStatus,
    active_index: Option<usize>,
}

impl OtpInput {
    /// Creates `OtpInput` with a six-cell default length.
    pub fn new(value: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            length: 6,
            gap: px(8.0),
            cell_size: px(40.0),
            masked: false,
            disabled: false,
            status: OtpInputStatus::Default,
            active_index: None,
        }
    }

    /// Sets the number of rendered OTP cells.
    pub fn length(mut self, length: usize) -> Self {
        self.length = length.clamp(1, 12);
        self
    }

    /// Sets spacing between cells.
    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = gap.into().max(px(0.0));
        self
    }

    /// Sets square cell size.
    pub fn cell_size(mut self, size: impl Into<Pixels>) -> Self {
        self.cell_size = size.into().max(px(28.0));
        self
    }

    /// Masks filled cells with a bullet.
    pub fn masked(mut self, masked: bool) -> Self {
        self.masked = masked;
        self
    }

    /// Toggles disabled visual state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Applies success status styling.
    pub fn success(mut self) -> Self {
        self.status = OtpInputStatus::Success;
        self
    }

    /// Applies warning status styling.
    pub fn warning(mut self) -> Self {
        self.status = OtpInputStatus::Warning;
        self
    }

    /// Applies error status styling.
    pub fn error(mut self) -> Self {
        self.status = OtpInputStatus::Error;
        self
    }

    /// Marks a cell as active/focused for controlled parent input flows.
    pub fn active_index(mut self, index: usize) -> Self {
        self.active_index = Some(index.min(self.length.saturating_sub(1)));
        self
    }

    /// Returns normalized OTP characters up to the configured length.
    pub fn cells(&self) -> Vec<Option<char>> {
        otp_cells(self.value.as_ref(), self.length)
    }

    fn status_color(&self, theme: &liora_theme::Theme) -> Hsla {
        match self.status {
            OtpInputStatus::Default => theme.neutral.border,
            OtpInputStatus::Success => theme.success.base,
            OtpInputStatus::Warning => theme.warning.base,
            OtpInputStatus::Error => theme.danger.base,
        }
    }
}

impl RenderOnce for OtpInput {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let status_color = self.status_color(&theme);
        let active_index = self.active_index;
        let disabled = self.disabled;
        let masked = self.masked;
        let cell_size = self.cell_size;
        let cells = self.cells();

        div()
            .flex()
            .items_center()
            .gap(self.gap)
            .children(cells.into_iter().enumerate().map(move |(index, value)| {
                let filled = value.is_some();
                let active = active_index == Some(index);
                let border_color = if disabled {
                    theme.neutral.border.opacity(0.5)
                } else if active || self.status != OtpInputStatus::Default {
                    status_color
                } else {
                    theme.neutral.border
                };
                let text = value.map(|ch| if masked { '•' } else { ch });

                div()
                    .w(cell_size)
                    .h(cell_size)
                    .rounded_md()
                    .border_1()
                    .border_color(border_color)
                    .bg(if disabled {
                        theme.neutral.hover
                    } else {
                        theme.neutral.card
                    })
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_lg()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .text_color(if disabled {
                        theme.neutral.text_disabled
                    } else {
                        theme.neutral.text_1
                    })
                    .when(active, |s| s.shadow_sm())
                    .child(text.map(|ch| ch.to_string()).unwrap_or_else(|| {
                        if filled {
                            String::new()
                        } else {
                            "·".to_string()
                        }
                    }))
            }))
    }
}

impl IntoElement for OtpInput {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

/// Normalizes an OTP string into fixed-length display cells.
pub fn otp_cells(value: &str, length: usize) -> Vec<Option<char>> {
    let mut chars = value.chars().filter(|ch| !ch.is_whitespace());
    (0..length.clamp(1, 12)).map(|_| chars.next()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn otp_cells_trim_whitespace_and_cap_length() {
        assert_eq!(
            otp_cells("1 2 3", 4),
            vec![Some('1'), Some('2'), Some('3'), None]
        );
        assert_eq!(otp_cells("1234567890123", 20).len(), 12);
    }

    #[test]
    fn otp_builders_track_state() {
        let otp = OtpInput::new("12")
            .length(4)
            .masked(true)
            .error()
            .active_index(9);

        assert_eq!(otp.length, 4);
        assert!(otp.masked);
        assert_eq!(otp.status, OtpInputStatus::Error);
        assert_eq!(otp.active_index, Some(3));
    }
}
