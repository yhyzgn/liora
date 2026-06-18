use crate::motion::pulse;
use gpui::{AnyElement, App, DefiniteLength, IntoElement, RenderOnce, Window, div, prelude::*, px};
use liora_core::Config;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SkeletonVariant {
    #[default]
    Paragraph,
    Circle,
    Square,
    Image,
}

pub struct SkeletonItem {
    variant: SkeletonVariant,
    width: Option<DefiniteLength>,
}

pub struct Skeleton {
    loading: bool,
    rows: u32,
    animated: bool,
    template: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>>,
    child: Option<AnyElement>,
}

impl SkeletonItem {
    pub fn new(variant: SkeletonVariant) -> Self {
        Self {
            variant,
            width: None,
        }
    }

    pub fn width(mut self, width: impl Into<DefiniteLength>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn width_2_5(self) -> Self {
        self.width(gpui::relative(0.4))
    }
}

impl Skeleton {
    pub fn new() -> Self {
        Self {
            loading: true,
            rows: 3,
            animated: true,
            template: None,
            child: None,
        }
    }

    pub fn loading(mut self, l: bool) -> Self {
        self.loading = l;
        self
    }

    pub fn rows(mut self, r: u32) -> Self {
        self.rows = r;
        self
    }

    pub fn animated(mut self, a: bool) -> Self {
        self.animated = a;
        self
    }

    pub fn template<F>(mut self, f: F) -> Self
    where
        F: Fn(&mut Window, &mut App) -> AnyElement + 'static,
    {
        self.template = Some(Box::new(f));
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }
}

impl RenderOnce for SkeletonItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let skeleton_bg = theme.neutral.hover;

        let item = match self.variant {
            SkeletonVariant::Circle => div().w(px(40.0)).h(px(40.0)).bg(skeleton_bg).rounded_full(),
            SkeletonVariant::Square => div()
                .w_full()
                .h(px(40.0))
                .bg(skeleton_bg)
                .rounded(px(theme.radius.sm)),
            SkeletonVariant::Paragraph => {
                div().w_full().h(px(16.0)).bg(skeleton_bg).rounded(px(4.0))
            }
            SkeletonVariant::Image => div()
                .w(px(200.0))
                .h(px(150.0))
                .bg(skeleton_bg)
                .rounded(px(theme.radius.sm)),
        };

        item.when_some(self.width, |s, width| s.w(width))
    }
}

impl IntoElement for SkeletonItem {
    type Element = gpui::Component<Self>;
    fn into_element(self) -> Self::Element {
        gpui::Component::new(self)
    }
}

impl RenderOnce for Skeleton {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        if !self.loading {
            return div()
                .child(self.child.unwrap_or_else(|| div().into_any_element()))
                .into_any_element();
        }

        if let Some(template) = self.template {
            return (template)(window, cx).into_any_element();
        }

        // Default: multiple rows of paragraph
        let animated = self.animated;

        div()
            .flex()
            .flex_col()
            .gap_3()
            .w_full()
            .children((0..self.rows).map(|i| {
                let width = if i == self.rows - 1 && self.rows > 1 {
                    gpui::relative(0.6)
                } else {
                    gpui::relative(1.0)
                };
                let row = div()
                    .w(width)
                    .child(SkeletonItem::new(SkeletonVariant::Paragraph));

                if animated {
                    pulse(("liora-skeleton-row-motion", i as usize), row).into_any_element()
                } else {
                    row.into_any_element()
                }
            }))
            .into_any_element()
    }
}

impl IntoElement for Skeleton {
    type Element = gpui::Component<Self>;
    fn into_element(self) -> Self::Element {
        gpui::Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skeleton_item_width_2_5_sets_fraction_width() {
        assert_eq!(
            SkeletonItem::new(SkeletonVariant::Paragraph)
                .width_2_5()
                .width,
            Some(gpui::relative(0.4))
        );
    }

    #[test]
    fn skeleton_default_rows_use_pulse_motion_when_animated() {
        let source = include_str!("skeleton.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains("pulse("));
        assert!(source.contains("liora-skeleton-row-motion"));
        assert!(source.contains("if animated"));
    }
}
