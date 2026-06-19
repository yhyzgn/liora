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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionDuration {
    Fast,
    Normal,
    Slow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionEasing {
    Linear,
    EaseInOut,
    EaseOut,
    Elastic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionPreset {
    FadeIn,
    FadeOut,
    PopIn,
    Pulse,
    Spin,
    ElasticSlide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionCurve {
    Linear,
    EaseInOut,
    EaseOut,
    ElasticSnap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FadeDirection {
    In,
    Out,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interpolator {
    from: f32,
    to: f32,
}

impl MotionDuration {
    pub fn as_duration(self) -> Duration {
        match self {
            Self::Fast => Duration::from_millis(220),
            Self::Normal => Duration::from_millis(320),
            Self::Slow => Duration::from_millis(900),
        }
    }
}

impl Interpolator {
    pub fn new(from: f32, to: f32) -> Self {
        Self { from, to }
    }

    pub fn from(&self) -> f32 {
        self.from
    }

    pub fn to(&self) -> f32 {
        self.to
    }

    pub fn sample(&self, delta: f32) -> f32 {
        self.sample_with(delta, MotionCurve::Linear)
    }

    pub fn sample_with(&self, delta: f32, curve: MotionCurve) -> f32 {
        self.from + (self.to - self.from) * curve_progress(delta, curve)
    }

    pub fn map(&self, delta: f32, mapper: impl FnOnce(f32) -> f32) -> f32 {
        self.from + (self.to - self.from) * mapper(delta.clamp(0.0, 1.0))
    }
}

pub fn motion_animation(duration: MotionDuration, easing: MotionEasing) -> Animation {
    Animation::new(duration.as_duration()).with_easing(move |delta| ease(delta, easing))
}

pub fn repeating_motion_animation(duration: MotionDuration, easing: MotionEasing) -> Animation {
    motion_animation(duration, easing).repeat()
}

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

pub fn fade_in<E>(id: impl Into<ElementId>, element: E) -> AnimationElement<E>
where
    E: Styled + IntoElement + 'static,
{
    fade(id, FadeDirection::In, element)
}

pub fn fade_out<E>(id: impl Into<ElementId>, element: E) -> AnimationElement<E>
where
    E: Styled + IntoElement + 'static,
{
    fade(id, FadeDirection::Out, element)
}

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

pub fn spin_icon(id: impl Into<ElementId>, icon: Icon) -> AnimationElement<Icon> {
    icon.with_animation(
        ElementId::from(id.into()),
        repeating_motion_animation(MotionDuration::Slow, MotionEasing::Linear),
        |icon, delta| icon.rotation(radians(delta * TAU)),
    )
}

pub fn elastic_slide(delta: f32) -> f32 {
    let t = delta.clamp(0.0, 1.0);
    let c1 = 1.35;
    let c3 = c1 + 1.0;
    1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
}

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
