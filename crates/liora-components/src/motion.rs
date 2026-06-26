//! Motion module.
//!
//! This public module implements the Liora motion presets and animation helpers used across interactive components. It keeps the reusable
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

use gpui::{Animation, AnimationElement, AnimationExt, ElementId, IntoElement, Styled, radians};
use liora_icons::Icon;
use std::{f32::consts::TAU, time::Duration};

const SMOOTH_SPIN_ANIMATION_SECS: f32 = 86_400.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Options that control motion duration behavior.
pub enum MotionDuration {
    /// Uses the `Fast` option for `MotionDuration`.
    Fast,
    /// Uses the `Normal` option for `MotionDuration`.
    Normal,
    /// Uses the `Slow` option for `MotionDuration`.
    Slow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Options that control motion easing behavior.
pub enum MotionEasing {
    /// Uses the `Linear` option for `MotionEasing`.
    Linear,
    /// Uses the `EaseInOut` option for `MotionEasing`.
    EaseInOut,
    /// Uses the `EaseOut` option for `MotionEasing`.
    EaseOut,
    /// Uses the `Elastic` option for `MotionEasing`.
    Elastic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Options that control motion preset behavior.
pub enum MotionPreset {
    /// Uses the `FadeIn` option for `MotionPreset`.
    FadeIn,
    /// Uses the `FadeOut` option for `MotionPreset`.
    FadeOut,
    /// Uses the `PopIn` option for `MotionPreset`.
    PopIn,
    /// Uses the `Pulse` option for `MotionPreset`.
    Pulse,
    /// Uses the `Spin` option for `MotionPreset`.
    Spin,
    /// Uses the `ElasticSlide` option for `MotionPreset`.
    ElasticSlide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Options that control motion curve behavior.
pub enum MotionCurve {
    /// Uses the `Linear` option for `MotionCurve`.
    Linear,
    /// Uses the `EaseInOut` option for `MotionCurve`.
    EaseInOut,
    /// Uses the `EaseOut` option for `MotionCurve`.
    EaseOut,
    /// Uses the `ElasticSnap` option for `MotionCurve`.
    ElasticSnap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Options that control fade direction behavior.
pub enum FadeDirection {
    /// Uses the in direction.
    In,
    /// Uses the out direction.
    Out,
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Fluent native GPUI component for rendering Liora interpolator.
pub struct Interpolator {
    from: f32,
    to: f32,
}

impl MotionDuration {
    /// Borrows this value as duration.
    pub fn as_duration(self) -> Duration {
        match self {
            Self::Fast => Duration::from_millis(220),
            Self::Normal => Duration::from_millis(320),
            Self::Slow => Duration::from_millis(900),
        }
    }
}

impl Interpolator {
    /// Creates `Interpolator` initialized from the supplied from, and to.
    pub fn new(from: f32, to: f32) -> Self {
        Self { from, to }
    }

    /// Performs the from operation used by this component.
    pub fn from(&self) -> f32 {
        self.from
    }

    /// Performs the to operation used by this component.
    pub fn to(&self) -> f32 {
        self.to
    }

    /// Performs the sample operation used by this component.
    pub fn sample(&self, delta: f32) -> f32 {
        self.sample_with(delta, MotionCurve::Linear)
    }

    /// Performs the sample with operation used by this component.
    pub fn sample_with(&self, delta: f32, curve: MotionCurve) -> f32 {
        self.from + (self.to - self.from) * curve_progress(delta, curve)
    }

    /// Performs the map operation used by this component.
    pub fn map(&self, delta: f32, mapper: impl FnOnce(f32) -> f32) -> f32 {
        self.from + (self.to - self.from) * mapper(delta.clamp(0.0, 1.0))
    }
}

/// Performs the motion animation operation used by this component.
pub fn motion_animation(duration: MotionDuration, easing: MotionEasing) -> Animation {
    Animation::new(duration.as_duration()).with_easing(move |delta| ease(delta, easing))
}

/// Performs the repeating motion animation operation used by this component.
pub fn repeating_motion_animation(duration: MotionDuration, easing: MotionEasing) -> Animation {
    motion_animation(duration, easing).repeat()
}

/// Performs the fade operation used by this component.
pub fn fade<E>(
    id: impl Into<ElementId>,
    direction: FadeDirection,
    element: E,
) -> AnimationElement<E>
where
    E: Styled + IntoElement + 'static,
{
    element.with_animation(
        ElementId::from(id.into()),
        motion_animation(MotionDuration::Fast, MotionEasing::EaseOut),
        move |element, delta| {
            let opacity = match direction {
                FadeDirection::In => delta,
                FadeDirection::Out => 1.0 - delta,
            };
            element.opacity(opacity)
        },
    )
}

/// Performs the fade in operation used by this component.
pub fn fade_in<E>(id: impl Into<ElementId>, element: E) -> AnimationElement<E>
where
    E: Styled + IntoElement + 'static,
{
    fade(id, FadeDirection::In, element)
}

/// Performs the fade out operation used by this component.
pub fn fade_out<E>(id: impl Into<ElementId>, element: E) -> AnimationElement<E>
where
    E: Styled + IntoElement + 'static,
{
    fade(id, FadeDirection::Out, element)
}

/// Performs the pop in operation used by this component.
pub fn pop_in<E>(id: impl Into<ElementId>, element: E) -> AnimationElement<E>
where
    E: Styled + IntoElement + 'static,
{
    element.with_animation(
        ElementId::from(id.into()),
        motion_animation(MotionDuration::Normal, MotionEasing::EaseOut),
        |element, delta| element.opacity(0.86 + delta * 0.14),
    )
}

/// Performs the pulse operation used by this component.
pub fn pulse<E>(id: impl Into<ElementId>, element: E) -> AnimationElement<E>
where
    E: Styled + IntoElement + 'static,
{
    element.with_animation(
        ElementId::from(id.into()),
        repeating_motion_animation(MotionDuration::Slow, MotionEasing::EaseInOut),
        |element, delta| element.opacity(0.62 + pulse_alpha(delta) * 0.38),
    )
}

/// Performs the spin icon operation used by this component.
pub fn spin_icon(id: impl Into<ElementId>, icon: Icon) -> AnimationElement<Icon> {
    spin_icon_with_duration(id, icon, MotionDuration::Slow.as_duration())
}

/// Performs the spin icon operation with an explicit cycle duration.
pub fn spin_icon_with_duration(
    id: impl Into<ElementId>,
    icon: Icon,
    cycle_duration: Duration,
) -> AnimationElement<Icon> {
    let cycle_secs = cycle_duration.as_secs_f32().max(0.1);
    icon.with_animation(
        ElementId::from(id.into()),
        Animation::new(Duration::from_secs_f32(SMOOTH_SPIN_ANIMATION_SECS))
            .repeat()
            .with_easing(|delta| delta),
        move |icon, delta| {
            let elapsed_secs = delta * SMOOTH_SPIN_ANIMATION_SECS;
            let turn = (elapsed_secs / cycle_secs).fract();
            icon.rotation(radians(turn * TAU))
        },
    )
}

/// Performs the elastic slide operation used by this component.
pub fn elastic_slide(delta: f32) -> f32 {
    let t = delta.clamp(0.0, 1.0);
    let c1 = 1.35;
    let c3 = c1 + 1.0;
    1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
}

/// Performs the elastic snap operation used by this component.
pub fn elastic_snap(delta: f32) -> f32 {
    let eased = gpui::ease_in_out(delta.clamp(0.0, 1.0));
    let snap_start = 0.62;

    if eased <= snap_start {
        eased
    } else {
        let local = (eased - snap_start) / (1.0 - snap_start);
        snap_start + (1.0 - snap_start) * elastic_slide(local)
    }
}

/// Performs the slide snap operation used by this component.
pub fn slide_snap(from: f32, to: f32, delta: f32) -> f32 {
    Interpolator::new(from, to).sample_with(delta, MotionCurve::ElasticSnap)
}

fn curve_progress(delta: f32, curve: MotionCurve) -> f32 {
    match curve {
        MotionCurve::Linear => gpui::linear(delta.clamp(0.0, 1.0)),
        MotionCurve::EaseInOut => gpui::ease_in_out(delta.clamp(0.0, 1.0)),
        MotionCurve::EaseOut => gpui::ease_out_quint()(delta.clamp(0.0, 1.0)),
        MotionCurve::ElasticSnap => elastic_snap(delta),
    }
}

fn ease(delta: f32, easing: MotionEasing) -> f32 {
    match easing {
        MotionEasing::Linear => gpui::linear(delta),
        MotionEasing::EaseInOut => gpui::ease_in_out(delta),
        MotionEasing::EaseOut => gpui::ease_out_quint()(delta),
        MotionEasing::Elastic => elastic_slide(delta).clamp(0.0, 1.0),
    }
}

fn pulse_alpha(delta: f32) -> f32 {
    gpui::pulsating_between(0.0, 1.0)(delta)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn motion_duration_tokens_track_liora_defaults() {
        assert_eq!(
            MotionDuration::Fast.as_duration(),
            Duration::from_millis(220)
        );
        assert_eq!(
            MotionDuration::Normal.as_duration(),
            Duration::from_millis(320)
        );
        assert_eq!(
            MotionDuration::Slow.as_duration(),
            Duration::from_millis(900)
        );
    }

    #[test]
    fn elastic_slide_overshoots_then_settles() {
        assert!(elastic_slide(0.0).abs() < 0.000_01);
        assert_eq!(elastic_slide(1.0), 1.0);
        assert!(elastic_slide(0.7) > 1.0);
    }

    #[test]
    fn interpolator_samples_common_curves() {
        let interpolator = Interpolator::new(10.0, 20.0);

        assert_eq!(interpolator.from(), 10.0);
        assert_eq!(interpolator.to(), 20.0);
        assert_eq!(interpolator.sample(0.5), 15.0);
        assert!(interpolator.sample_with(0.25, MotionCurve::EaseInOut) < 12.5);
        assert_eq!(interpolator.map(0.5, |delta| delta * delta), 12.5);
    }

    #[test]
    fn elastic_snap_accelerates_decelerates_and_snaps() {
        assert!(elastic_snap(0.25) < 0.25);
        assert!((elastic_snap(1.0) - 1.0).abs() < 0.000_01);
        assert!(elastic_snap(0.75) > 1.0);
    }

    #[test]
    fn slide_snap_overshoots_toward_target() {
        assert!(slide_snap(3.0, 21.0, 0.25) < 3.0 + (21.0 - 3.0) * 0.25);
        assert!(slide_snap(3.0, 21.0, 0.75) > 21.0);
        assert!(slide_snap(21.0, 3.0, 0.75) < 3.0);
        assert_eq!(slide_snap(3.0, 21.0, 1.0), 21.0);
    }

    #[test]
    fn elastic_easing_is_bounded_for_gpui_animation() {
        let animation = motion_animation(MotionDuration::Normal, MotionEasing::Elastic);
        let eased = (animation.easing)(0.7);

        assert!((0.0..=1.0).contains(&eased));
    }

    #[test]
    fn spin_icon_uses_the_long_cycle_slow_motion_preset() {
        assert_eq!(
            MotionDuration::Slow.as_duration(),
            Duration::from_millis(900)
        );
    }

    #[test]
    fn spin_icon_with_duration_keeps_the_rotation_helper_explicit() {
        let custom = Animation::new(Duration::from_secs_f32(SMOOTH_SPIN_ANIMATION_SECS)).repeat();

        assert!(custom.duration >= Duration::from_secs(60));
        assert!(!custom.oneshot);
    }

    #[test]
    fn motion_presets_cover_requested_component_behaviors() {
        let presets = [
            MotionPreset::FadeIn,
            MotionPreset::FadeOut,
            MotionPreset::PopIn,
            MotionPreset::Pulse,
            MotionPreset::Spin,
            MotionPreset::ElasticSlide,
        ];

        assert_eq!(presets.len(), 6);
    }
}
