use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{App, Hsla, IntoElement, Pixels, RenderOnce, Window, div, prelude::*, px};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressType {
    #[default]
    Line,
    Circle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressStatus {
    Success,
    Warning,
    Exception,
}

pub struct Progress {
    percentage: f32,
    type_: ProgressType,
    stroke_width: Pixels,
    status: Option<ProgressStatus>,
    color: Option<Hsla>,
    show_text: bool,
    text_inside: bool,
    text_inside_center: bool,
}

impl Progress {
    pub fn new(percentage: f32) -> Self {
        Self {
            percentage: percentage.clamp(0.0, 100.0),
            type_: ProgressType::Line,
            stroke_width: px(6.0),
            status: None,
            color: None,
            show_text: true,
            text_inside: false,
            text_inside_center: false,
        }
    }

    pub fn type_(mut self, t: ProgressType) -> Self {
        self.type_ = t;
        self
    }

    pub fn stroke_width(mut self, w: impl Into<Pixels>) -> Self {
        self.stroke_width = w.into();
        self
    }

    pub fn status(mut self, s: ProgressStatus) -> Self {
        self.status = Some(s);
        self
    }

    pub fn color(mut self, c: Hsla) -> Self {
        self.color = Some(c);
        self
    }

    pub fn show_text(mut self, show: bool) -> Self {
        self.show_text = show;
        self
    }

    pub fn text_inside(mut self, inside: bool) -> Self {
        self.text_inside = inside;
        self
    }

    pub fn text_inside_center(mut self, center: bool) -> Self {
        self.text_inside_center = center;
        self
    }

    pub fn text_inside_centered(mut self) -> Self {
        self.text_inside = true;
        self.text_inside_center = true;
        self
    }
}

impl RenderOnce for Progress {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        let status_color = match self.status {
            Some(ProgressStatus::Success) => theme.success.base,
            Some(ProgressStatus::Warning) => theme.warning.base,
            Some(ProgressStatus::Exception) => theme.danger.base,
            None => self.color.unwrap_or(theme.primary.base),
        };

        if self.type_ == ProgressType::Line {
            let percent_text = format!("{}%", self.percentage.round() as i32);
            let inside_center = self.show_text && self.text_inside && self.text_inside_center;
            let center_text_color = if self.percentage >= 50.0 {
                gpui::white()
            } else {
                theme.neutral.text_2
            };
            let bar = div()
                .h_full()
                .w(gpui::relative(self.percentage / 100.0))
                .bg(status_color)
                .rounded_full()
                .when(
                    self.show_text
                        && self.text_inside
                        && !self.text_inside_center
                        && self.percentage > 0.0,
                    |s| {
                        s.min_w(px(36.0))
                            .flex()
                            .items_center()
                            .justify_end()
                            .px_2()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(gpui::white())
                                    .whitespace_nowrap()
                                    .child(percent_text.clone()),
                            )
                    },
                );

            let track = div()
                .relative()
                .flex_1()
                .h(self.stroke_width)
                .bg(theme.neutral.hover)
                .rounded_full()
                .overflow_hidden()
                .child(bar)
                .when(inside_center, |s| {
                    s.child(
                        div()
                            .absolute()
                            .top_0()
                            .left_0()
                            .size_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_xs()
                            .text_color(center_text_color)
                            .whitespace_nowrap()
                            .child(percent_text.clone()),
                    )
                });

            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_2()
                .w_full()
                .child(track)
                .when(self.show_text && !self.text_inside, |s| {
                    s.child(
                        div()
                            .flex()
                            .items_center()
                            .justify_start()
                            .w(px(40.0))
                            .child(match self.status {
                                Some(ProgressStatus::Success) => Icon::new(IconName::CircleCheck)
                                    .size(px(16.0))
                                    .color(theme.success.base)
                                    .into_any_element(),
                                Some(ProgressStatus::Exception) => Icon::new(IconName::CircleX)
                                    .size(px(16.0))
                                    .color(theme.danger.base)
                                    .into_any_element(),
                                _ => div()
                                    .text_xs()
                                    .text_color(theme.neutral.text_2)
                                    .child(percent_text)
                                    .into_any_element(),
                            }),
                    )
                })
        } else {
            // Circle placeholder
            div().child("Circle Progress Placeholder")
        }
    }
}

impl IntoElement for Progress {
    type Element = gpui::Component<Self>;
    fn into_element(self) -> Self::Element {
        gpui::Component::new(self)
    }
}
