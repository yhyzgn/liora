//! Signal Meter module.
//!
//! This public module implements the Liora signal strength meter component with threshold coloring. It keeps the reusable
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
    App, Background, BorderStyle, Bounds, Component, Corners, Edges, Hsla, IntoElement, Pixels,
    RenderOnce, Window, point, prelude::*, px, quad, size,
};
use liora_core::Config;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control signal meter kind behavior.
pub enum SignalMeterKind {
    #[default]
    /// Renders cellular signal-style bars.
    Mobile,
    /// Renders Wi-Fi signal-style arcs.
    Wifi,
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// Fluent native GPUI component for rendering Liora signal level color.
pub struct SignalLevelColor {
    /// Signal or progress level represented by this item.
    pub level: usize,
    /// Color token or explicit color applied to the visual element.
    pub color: Hsla,
}

impl SignalLevelColor {
    /// Creates `SignalLevelColor` initialized from the supplied level, and color.
    pub fn new(level: usize, color: Hsla) -> Self {
        Self { level, color }
    }
}

#[derive(Clone)]
/// Fluent native GPUI component for rendering Liora signal meter.
pub struct SignalMeter {
    level: usize,
    max_level: usize,
    kind: SignalMeterKind,
    active_color: Option<Hsla>,
    inactive_color: Option<Hsla>,
    level_colors: Vec<Hsla>,
    threshold_colors: Vec<SignalLevelColor>,
    bar_width: Pixels,
    gap: Pixels,
    height: Pixels,
}

impl SignalMeter {
    /// Creates `SignalMeter` initialized from the supplied level.
    pub fn new(level: usize) -> Self {
        Self {
            level,
            max_level: 4,
            kind: SignalMeterKind::Mobile,
            active_color: None,
            inactive_color: None,
            level_colors: Vec::new(),
            threshold_colors: Vec::new(),
            bar_width: px(6.0),
            gap: px(4.0),
            height: px(32.0),
        }
    }
    /// Sets the maximum level limit.
    pub fn max_level(mut self, max_level: usize) -> Self {
        self.max_level = max_level.max(1);
        self.level = self.level.min(self.max_level);
        self
    }

    /// Sets the total signals value used by the component.
    pub fn total_signals(self, total: usize) -> Self {
        self.max_level(total)
    }

    /// Sets the signal count value used by the component.
    pub fn signal_count(self, count: usize) -> Self {
        self.max_level(count)
    }
    /// Sets the wifi value used by the component.
    pub fn wifi(mut self) -> Self {
        self.kind = SignalMeterKind::Wifi;
        self
    }
    /// Sets the mobile value used by the component.
    pub fn mobile(mut self) -> Self {
        self.kind = SignalMeterKind::Mobile;
        self
    }
    /// Sets the active color used by the rendered component.
    pub fn active_color(mut self, color: Hsla) -> Self {
        self.active_color = Some(color);
        self
    }
    /// Sets the inactive color used by the rendered component.
    pub fn inactive_color(mut self, color: Hsla) -> Self {
        self.inactive_color = Some(color);
        self
    }

    /// Sets the level colors value used by the component.
    pub fn level_colors(mut self, colors: impl IntoIterator<Item = Hsla>) -> Self {
        self.level_colors = colors.into_iter().collect();
        self
    }

    /// Sets the signal colors value used by the component.
    pub fn signal_colors(self, colors: impl IntoIterator<Item = Hsla>) -> Self {
        self.level_colors(colors)
    }

    /// Sets the threshold colors value used by the component.
    pub fn threshold_colors(mut self, colors: impl IntoIterator<Item = SignalLevelColor>) -> Self {
        self.threshold_colors = colors.into_iter().collect();
        self.threshold_colors
            .sort_by_key(|threshold| threshold.level);
        self
    }

    /// Performs the level threshold colors operation used by this component.
    pub fn level_threshold_colors(
        self,
        colors: impl IntoIterator<Item = SignalLevelColor>,
    ) -> Self {
        self.threshold_colors(colors)
    }

    /// Sets the level color used by the rendered component.
    pub fn level_color(mut self, level: usize, color: Hsla) -> Self {
        self.threshold_colors
            .push(SignalLevelColor::new(level, color));
        self.threshold_colors
            .sort_by_key(|threshold| threshold.level);
        self
    }
    /// Sets a fixed bar width instead of automatic band sizing.
    pub fn bar_width(mut self, width: impl Into<Pixels>) -> Self {
        self.bar_width = width.into().max(px(2.0));
        self
    }
    /// Sets the spacing between child elements.
    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = gap.into().max(px(0.0));
        self
    }
    /// Sets the component height token used during GPUI layout.
    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = height.into().max(px(12.0));
        self
    }
}

impl RenderOnce for SignalMeter {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let active = self.active_color.unwrap_or(theme.success.base);
        let inactive = self
            .inactive_color
            .unwrap_or(theme.neutral.border.opacity(0.55));
        let total_width = self.bar_width * self.max_level as f32
            + self.gap * self.max_level.saturating_sub(1) as f32;
        let max_level = self.max_level;
        let level = self.level.min(max_level);
        let kind = self.kind;
        let bar_width = self.bar_width;
        let gap = self.gap;
        let height = self.height;
        let level_colors = self.level_colors.clone();
        let threshold_color = self
            .threshold_colors
            .iter()
            .filter(|threshold| level >= threshold.level)
            .map(|threshold| threshold.color)
            .last();
        gpui::canvas(
            |_, _, _| (),
            move |bounds, _, window, _| {
                for index in 0..max_level {
                    let ratio = (index + 1) as f32 / max_level as f32;
                    let bar_h = match kind {
                        SignalMeterKind::Mobile => height.as_f32() * (0.28 + ratio * 0.72),
                        SignalMeterKind::Wifi => height.as_f32() * ratio,
                    };
                    let x = bounds.left() + (bar_width + gap) * index as f32;
                    let y = bounds.bottom() - px(bar_h);
                    let color = if index < level {
                        threshold_color
                            .or_else(|| level_colors.get(index).copied())
                            .unwrap_or(active)
                    } else {
                        inactive
                    };
                    let rect = Bounds::new(point(x, y), size(bar_width, px(bar_h)));
                    window.paint_quad(quad(
                        rect,
                        Corners::all(bar_width / 2.0).clamp_radii_for_quad_size(rect.size),
                        Background::from(color),
                        Edges::all(px(0.0)),
                        gpui::transparent_black(),
                        BorderStyle::Solid,
                    ));
                }
            },
        )
        .w(total_width)
        .h(self.height)
    }
}

impl IntoElement for SignalMeter {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn signal_meter_clamps_levels() {
        let meter = SignalMeter::new(9)
            .max_level(5)
            .total_signals(6)
            .wifi()
            .bar_width(px(8.0))
            .gap(px(3.0))
            .height(px(24.0))
            .level_colors([gpui::red(), gpui::yellow(), gpui::green()])
            .threshold_colors([
                SignalLevelColor::new(2, gpui::red()),
                SignalLevelColor::new(3, gpui::yellow()),
                SignalLevelColor::new(5, gpui::green()),
            ]);
        assert_eq!(meter.level, 5);
        assert_eq!(meter.max_level, 6);
        assert_eq!(meter.kind, SignalMeterKind::Wifi);
        assert_eq!(meter.bar_width, px(8.0));
        assert_eq!(meter.level_colors.len(), 3);
        assert_eq!(meter.threshold_colors.len(), 3);
    }

    #[test]
    fn signal_meter_threshold_colors_sort_by_level() {
        let meter = SignalMeter::new(4)
            .total_signals(5)
            .level_color(5, gpui::green())
            .level_color(2, gpui::red())
            .level_color(3, gpui::yellow());
        let levels = meter
            .threshold_colors
            .iter()
            .map(|threshold| threshold.level)
            .collect::<Vec<_>>();
        assert_eq!(levels, vec![2, 3, 5]);
    }
}
