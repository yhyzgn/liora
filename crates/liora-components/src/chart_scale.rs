//! Chart Scale module.
//!
//! This public module implements the Liora chart scale helpers for linear, point, and band coordinate mapping. It keeps the reusable
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

use gpui::SharedString;

#[derive(Clone, Copy, Debug, PartialEq)]
/// Fluent native GPUI component for rendering Liora scale linear.
pub struct ScaleLinear {
    domain: (f64, f64),
    range: (f32, f32),
}

impl ScaleLinear {
    /// Creates `ScaleLinear` initialized from the supplied domain, and range.
    pub fn new(domain: (f64, f64), range: (f32, f32)) -> Self {
        Self { domain, range }
    }

    /// Maps a numeric scale value into the configured pixel range.
    pub fn tick(&self, value: f64) -> f32 {
        let span = self.domain.1 - self.domain.0;
        if !value.is_finite() || span.abs() < f64::EPSILON {
            return self.range.0;
        }
        let t = ((value - self.domain.0) / span) as f32;
        self.range.0 + (self.range.1 - self.range.0) * t
    }

    /// Performs the ticks operation used by this component.
    pub fn ticks(&self, count: usize) -> Vec<(f64, f32)> {
        let count = count.max(2);
        let step = (self.domain.1 - self.domain.0) / (count - 1) as f64;
        (0..count)
            .map(|index| {
                let value = self.domain.0 + step * index as f64;
                (value, self.tick(value))
            })
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq)]
/// Fluent native GPUI component for rendering Liora scale point.
pub struct ScalePoint {
    domain: Vec<SharedString>,
    domain_len: usize,
    range: (f32, f32),
}

impl ScalePoint {
    /// Creates `ScalePoint` initialized from the supplied domain, and range.
    pub fn new(domain: Vec<SharedString>, range: (f32, f32)) -> Self {
        let domain_len = domain.len();
        Self {
            domain,
            domain_len,
            range,
        }
    }

    /// Build an index-based point scale without cloning/storing every label.
    /// This is useful for dense native charts where label text is painted from
    /// a separate sparse axis-label list, while data points still need stable
    /// original-index positioning.
    pub fn from_len(domain_len: usize, range: (f32, f32)) -> Self {
        Self {
            domain: Vec::new(),
            domain_len,
            range,
        }
    }

    /// Returns the visual index for a point-scale domain value.
    pub fn tick_index(&self, index: usize) -> Option<f32> {
        if self.domain_len == 0 || index >= self.domain_len {
            return None;
        }
        if self.domain_len == 1 {
            return Some((self.range.0 + self.range.1) / 2.0);
        }
        let step = (self.range.1 - self.range.0) / (self.domain_len - 1) as f32;
        Some(self.range.0 + step * index as f32)
    }

    /// Maps a numeric scale value into the configured pixel range.
    pub fn tick(&self, value: &SharedString) -> Option<f32> {
        self.domain
            .iter()
            .position(|label| label == value)
            .and_then(|index| self.tick_index(index))
    }
}

#[derive(Clone, Debug, PartialEq)]
/// Fluent native GPUI component for rendering Liora scale band.
pub struct ScaleBand {
    domain: Vec<SharedString>,
    range: (f32, f32),
    padding_inner: f32,
    padding_outer: f32,
}

impl ScaleBand {
    /// Creates `ScaleBand` initialized from the supplied domain, and range.
    pub fn new(domain: Vec<SharedString>, range: (f32, f32)) -> Self {
        Self {
            domain,
            range,
            padding_inner: 0.2,
            padding_outer: 0.1,
        }
    }

    /// Sets the padding inner value used by the component.
    pub fn padding_inner(mut self, padding: f32) -> Self {
        self.padding_inner = padding.clamp(0.0, 0.95);
        self
    }

    /// Sets the padding outer value used by the component.
    pub fn padding_outer(mut self, padding: f32) -> Self {
        self.padding_outer = padding.max(0.0);
        self
    }

    /// Performs the step operation used by this component.
    pub fn step(&self) -> f32 {
        if self.domain.is_empty() {
            return 0.0;
        }
        let slots = self.domain.len() as f32 - self.padding_inner + self.padding_outer * 2.0;
        if slots <= 0.0 {
            0.0
        } else {
            (self.range.1 - self.range.0) / slots
        }
    }

    /// Performs the band width operation used by this component.
    pub fn band_width(&self) -> f32 {
        self.step() * (1.0 - self.padding_inner)
    }

    /// Returns the visual index for a point-scale domain value.
    pub fn tick_index(&self, index: usize) -> Option<f32> {
        if self.domain.is_empty() || index >= self.domain.len() {
            return None;
        }
        Some(self.range.0 + self.step() * (self.padding_outer + index as f32))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_scale_maps_domain_to_range() {
        let scale = ScaleLinear::new((0.0, 100.0), (200.0, 0.0));
        assert_eq!(scale.tick(0.0), 200.0);
        assert_eq!(scale.tick(50.0), 100.0);
        assert_eq!(scale.tick(100.0), 0.0);
    }

    #[test]
    fn point_scale_handles_single_and_multiple_labels() {
        let one = ScalePoint::new(vec!["A".into()], (0.0, 100.0));
        assert_eq!(one.tick_index(0), Some(50.0));

        let many = ScalePoint::new(vec!["A".into(), "B".into(), "C".into()], (0.0, 100.0));
        assert_eq!(many.tick_index(1), Some(50.0));
        assert_eq!(many.tick(&"C".into()), Some(100.0));

        let index_only = ScalePoint::from_len(3, (0.0, 100.0));
        assert_eq!(index_only.tick_index(2), Some(100.0));
        assert_eq!(index_only.tick(&"C".into()), None);
    }

    #[test]
    fn band_scale_allocates_padded_bands() {
        let scale = ScaleBand::new(vec!["A".into(), "B".into()], (0.0, 120.0))
            .padding_inner(0.2)
            .padding_outer(0.1);
        assert!(scale.band_width() > 0.0);
        assert!(scale.tick_index(1).unwrap() > scale.tick_index(0).unwrap());
    }
}
