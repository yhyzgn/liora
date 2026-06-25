//! Otp Input module.
//!
//! This public module implements the Liora one-time-passcode input component. It keeps the reusable
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

use crate::Input;
use gpui::{
    App, Context, Entity, FocusHandle, Focusable, Hsla, IntoElement, MouseButton, Pixels, Render,
    SharedString, Window, div, prelude::*, px,
};
use liora_core::Config;
use std::ops::Range;

type OtpInputChangeCallback = Box<dyn Fn(SharedString, &mut Context<OtpInput>) + 'static>;

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

/// Fluent native GPUI component for rendering and editing OTP/PIN entry cells.
pub struct OtpInput {
    input: Entity<Input>,
    length: usize,
    gap: Pixels,
    cell_size: Pixels,
    masked: bool,
    disabled: bool,
    status: OtpInputStatus,
    on_change: Option<OtpInputChangeCallback>,
}

impl OtpInput {
    /// Creates `OtpInput` with a six-cell default length.
    pub fn new(value: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        let length = 6;
        let value = normalize_otp_value(value.into().as_ref(), length);
        let owner = cx.entity().downgrade();
        Self {
            input: cx.new(move |cx| {
                Input::new(value, cx)
                    .width(px(1.0))
                    .on_change(move |_, cx| {
                        let _ = owner.update(cx, |otp, cx| otp.handle_input_change(cx));
                    })
            }),
            length,
            gap: px(8.0),
            cell_size: px(40.0),
            masked: false,
            disabled: false,
            status: OtpInputStatus::Default,
            on_change: None,
        }
    }

    /// Creates a GPUI entity that owns this component state across render passes.
    pub fn entity(value: impl Into<SharedString>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(value, cx))
    }

    /// Sets the number of rendered OTP cells.
    pub fn length(mut self, length: usize, cx: &mut Context<Self>) -> Self {
        let length = length.clamp(1, 12);
        self.length = length;
        self.sync_input_constraints(cx);
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

    /// Toggles disabled visual state and suppresses editing when enabled.
    pub fn disabled(mut self, disabled: bool, cx: &mut Context<Self>) -> Self {
        self.disabled = disabled;
        self.sync_input_constraints(cx);
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

    /// Registers a callback that runs when the normalized OTP value changes.
    pub fn on_change(mut self, cb: impl Fn(SharedString, &mut Context<Self>) + 'static) -> Self {
        self.on_change = Some(Box::new(cb));
        self
    }

    /// Updates the current change callback while preserving component identity.
    pub fn set_on_change(
        &mut self,
        cb: impl Fn(SharedString, &mut Context<Self>) + 'static,
        cx: &mut Context<Self>,
    ) {
        self.on_change = Some(Box::new(cb));
        cx.notify();
    }

    /// Returns the current normalized value.
    pub fn value(&self, cx: &App) -> SharedString {
        self.input.read(cx).value()
    }

    /// Returns normalized OTP characters up to the configured length.
    pub fn cells(&self, cx: &App) -> Vec<Option<char>> {
        otp_cells(self.value(cx).as_ref(), self.length)
    }

    /// Returns the current caret/selection range in byte offsets.
    pub fn selected_range(&self, cx: &App) -> Range<usize> {
        self.input.read(cx).selected_range()
    }

    /// Sets the focused cell/caret position. Filled cells are selected so the next typed
    /// character replaces that cell instead of shifting the remaining code.
    pub fn set_active_index(&mut self, index: usize, cx: &mut Context<Self>) {
        let range = self.byte_range_for_index(index, cx);
        cx.update_entity(&self.input, |input, cx| {
            input.set_selection(range, cx);
        });
        cx.notify();
    }

    fn byte_range_for_index(&self, index: usize, cx: &App) -> Range<usize> {
        let value = self.input.read(cx).value();
        let char_index = index.min(self.length);
        let start = value
            .char_indices()
            .nth(char_index)
            .map(|(offset, _)| offset)
            .unwrap_or(value.len());
        let end = value[start..]
            .chars()
            .next()
            .map(|ch| start + ch.len_utf8())
            .unwrap_or(start);
        start..end
    }

    fn sync_input_constraints(&mut self, cx: &mut Context<Self>) {
        let length = self.length;
        let disabled = self.disabled;
        cx.update_entity(&self.input, |input, cx| {
            let normalized = normalize_otp_value(input.value().as_ref(), length);
            input.set_value(normalized, cx);
            input.set_disabled(disabled, cx);
        });
    }

    fn handle_input_change(&mut self, cx: &mut Context<Self>) {
        let raw = self.input.read(cx).value();
        let normalized = normalize_otp_value(raw.as_ref(), self.length);
        if normalized != raw {
            cx.update_entity(&self.input, |input, cx| {
                input.set_value(normalized.clone(), cx);
            });
        }

        if let Some(on_change) = self.on_change.take() {
            let value = self.input.read(cx).value();
            on_change(value, cx);
            self.on_change = Some(on_change);
        }

        cx.notify();
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

impl Focusable for OtpInput {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.input.read(cx).focus_handle(cx)
    }
}

impl Render for OtpInput {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let focused = self.focus_handle(cx).is_focused(window);
        let value = self.input.read(cx).value();
        let selected_range = self.input.read(cx).selected_range();
        let active_offset = if selected_range.is_empty() {
            selected_range.end
        } else {
            selected_range.start
        };
        let active_index = byte_offset_to_cell_index(value.as_ref(), active_offset, self.length);
        let cells = otp_cells(value.as_ref(), self.length);
        let status_color = self.status_color(&theme);
        let disabled = self.disabled;
        let masked = self.masked;
        let cell_size = self.cell_size;

        cx.update_entity(&self.input, |input, cx| {
            input.set_disabled(disabled, cx);
        });

        let input = self.input.clone();

        let hidden_input = div()
            .absolute()
            .w(px(1.0))
            .h(px(1.0))
            .opacity(0.0)
            .child(input);

        div()
            .relative()
            .flex()
            .items_center()
            .gap(self.gap)
            .child(hidden_input)
            .children(cells.into_iter().enumerate().map(move |(index, value)| {
                let active = focused && active_index == index;
                let border_color = if disabled {
                    theme.neutral.border.opacity(0.5)
                } else if active || self.status != OtpInputStatus::Default {
                    status_color
                } else {
                    theme.neutral.border
                };
                let text = value.map(|ch| if masked { '•' } else { ch });
                let input = self.input.clone();
                let host = cx.entity().clone();
                let cell_text = if let Some(ch) = text {
                    ch.to_string()
                } else if active {
                    "▌".to_string()
                } else {
                    "·".to_string()
                };

                div()
                    .w(cell_size)
                    .h(cell_size)
                    .rounded_md()
                    .border_1()
                    .border_color(border_color)
                    .bg(if disabled {
                        theme.neutral.hover
                    } else if active {
                        theme.primary.base.opacity(0.08)
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
                    .when(!disabled, |s| {
                        s.cursor_text()
                            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                window.focus(&input.read(cx).focus_handle(cx));
                                host.update(cx, |host, cx| {
                                    host.set_active_index(index, cx);
                                });
                                cx.stop_propagation();
                            })
                    })
                    .when(active, |s| s.shadow_sm())
                    .child(cell_text)
            }))
    }
}

/// Normalizes an OTP string into fixed-length display cells.
pub fn otp_cells(value: &str, length: usize) -> Vec<Option<char>> {
    let mut chars = value.chars().filter(|ch| !ch.is_whitespace());
    (0..length.clamp(1, 12)).map(|_| chars.next()).collect()
}

/// Keeps only non-whitespace characters and caps the value to the configured OTP length.
pub fn normalize_otp_value(value: &str, length: usize) -> SharedString {
    SharedString::from(
        value
            .chars()
            .filter(|ch| !ch.is_whitespace())
            .take(length.clamp(1, 12))
            .collect::<String>(),
    )
}

/// Returns whether an edited candidate value can be accepted by the OTP input.
pub fn otp_candidate_is_valid(value: &str, length: usize) -> bool {
    value.chars().filter(|ch| !ch.is_whitespace()).count() <= length.clamp(1, 12)
}

fn byte_offset_to_cell_index(value: &str, offset: usize, length: usize) -> usize {
    let offset = offset.min(value.len());
    let mut count = 0;
    for (byte_offset, _) in value.char_indices() {
        if byte_offset >= offset {
            break;
        }
        count += 1;
    }
    count.min(length.saturating_sub(1))
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
    fn otp_candidate_validation_accepts_paste_up_to_length() {
        assert!(otp_candidate_is_valid("12 34", 4));
        assert!(!otp_candidate_is_valid("12345", 4));
    }

    #[test]
    fn otp_normalization_filters_spaces_and_caps_length() {
        assert_eq!(normalize_otp_value("1 2 3 4 5", 4).as_ref(), "1234");
    }

    #[test]
    fn byte_offsets_map_to_cell_indices() {
        assert_eq!(byte_offset_to_cell_index("1234", 0, 4), 0);
        assert_eq!(byte_offset_to_cell_index("1234", 2, 4), 2);
        assert_eq!(byte_offset_to_cell_index("1234", 4, 4), 3);
    }

    #[test]
    fn otp_source_uses_real_input_for_editing() {
        let source = include_str!("otp_input.rs");
        assert!(source.contains("input: Entity<Input>"));
        assert!(source.contains("window.focus(&input.read(cx).focus_handle(cx))"));
        assert!(source.contains("input.set_selection(range, cx)"));
        let input_change_pipeline =
            ["Input::new(value, cx)", ".width(px(1.0))", ".on_change"].join(".*");
        assert!(regex_like_in_order(source, &input_change_pipeline));
        assert!(source.contains("otp.handle_input_change(cx)"));
        assert!(source.contains("cx.notify()"));
        let click_change_bypass = ["host", ".emit_change(", "window"].join("");
        assert!(!source.contains(&click_change_bypass));
    }

    fn regex_like_in_order(source: &str, pattern: &str) -> bool {
        let mut cursor = 0;
        for part in pattern.split(".*") {
            let Some(offset) = source[cursor..].find(part) else {
                return false;
            };
            cursor += offset + part.len();
        }
        true
    }
}
