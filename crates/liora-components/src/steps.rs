use gpui::{App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StepsDirection {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepStatus {
    Wait,
    Process,
    Finish,
    Error,
}

pub struct StepItem {
    pub title: SharedString,
    pub description: Option<SharedString>,
    pub icon: Option<IconName>,
    pub status: Option<StepStatus>,
}

pub struct Steps {
    active: usize,
    direction: StepsDirection,
    items: Vec<StepItem>,
}

impl StepItem {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            description: None,
            icon: None,
            status: None,
        }
    }

    pub fn description(mut self, d: impl Into<SharedString>) -> Self {
        self.description = Some(d.into());
        self
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn status(mut self, s: StepStatus) -> Self {
        self.status = Some(s);
        self
    }
}

impl Steps {
    pub fn new() -> Self {
        Self {
            active: 0,
            direction: StepsDirection::Horizontal,
            items: vec![],
        }
    }

    pub fn active(mut self, active: usize) -> Self {
        self.active = active;
        self
    }

    pub fn direction(mut self, d: StepsDirection) -> Self {
        self.direction = d;
        self
    }

    pub fn step(mut self, item: StepItem) -> Self {
        self.items.push(item);
        self
    }
}

impl RenderOnce for Steps {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let items_count = self.items.len();
        let active = self.active;
        let direction = self.direction;
        let is_vertical = direction == StepsDirection::Vertical;

        div()
            .flex()
            .when(!is_vertical, |s| s.flex_row().w_full())
            .when(is_vertical, |s| s.flex_col().h_full())
            .children(self.items.into_iter().enumerate().map(|(i, item)| {
                let is_last = i == items_count - 1;
                let status = item.status.unwrap_or_else(|| {
                    if i < active {
                        StepStatus::Finish
                    } else if i == active {
                        StepStatus::Process
                    } else {
                        StepStatus::Wait
                    }
                });

                let color = match status {
                    StepStatus::Finish => theme.primary.base,
                    StepStatus::Process => theme.neutral.text_1,
                    StepStatus::Wait => theme.neutral.text_3,
                    StepStatus::Error => theme.danger.base,
                };

                let icon_bg = match status {
                    StepStatus::Finish => gpui::transparent_black(),
                    StepStatus::Process => theme.primary.base,
                    StepStatus::Wait => gpui::transparent_black(),
                    StepStatus::Error => theme.danger.base,
                };

                let icon_border = match status {
                    StepStatus::Finish => theme.primary.base,
                    StepStatus::Process => theme.primary.base,
                    StepStatus::Wait => theme.neutral.border,
                    StepStatus::Error => theme.danger.base,
                };

                let icon_color = match status {
                    StepStatus::Finish => theme.primary.base,
                    StepStatus::Process => theme.neutral.card,
                    StepStatus::Wait => theme.neutral.text_3,
                    StepStatus::Error => theme.neutral.card,
                };

                div()
                    .flex()
                    .when(!is_vertical, |s| s.flex_1().flex_row().items_center())
                    .when(is_vertical, |s| s.flex_col())
                    .child(
                        div()
                            .flex()
                            .when(!is_vertical, |s| s.flex_row().items_center().gap_2())
                            .when(is_vertical, |s| s.flex_col().items_start().gap_2())
                            .child(
                                // Icon/Number container
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .w(px(24.0))
                                    .h(px(24.0))
                                    .rounded_full()
                                    .border_1()
                                    .border_color(icon_border)
                                    .bg(icon_bg)
                                    .child(match item.icon {
                                        Some(icon) => Icon::new(icon)
                                            .size(px(14.0))
                                            .color(icon_color)
                                            .into_any_element(),
                                        None => {
                                            if status == StepStatus::Finish {
                                                Icon::new(IconName::Check)
                                                    .size(px(14.0))
                                                    .color(icon_color)
                                                    .into_any_element()
                                            } else {
                                                div()
                                                    .text_xs()
                                                    .text_color(icon_color)
                                                    .child((i + 1).to_string())
                                                    .into_any_element()
                                            }
                                        }
                                    }),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(gpui::FontWeight::BOLD)
                                            .text_color(color)
                                            .child(item.title),
                                    )
                                    .when_some(item.description, |s, d| {
                                        s.child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.neutral.text_3)
                                                .child(d),
                                        )
                                    }),
                            ),
                    )
                    .when(!is_last, |s| {
                        s.child(
                            // Line
                            div()
                                .flex_1()
                                .when(!is_vertical, |s| {
                                    s.mx_4().h(px(1.0)).bg(if i < active {
                                        theme.primary.base
                                    } else {
                                        theme.neutral.border
                                    })
                                })
                                .when(is_vertical, |s| {
                                    s.ml(px(12.0)).my_2().w(px(1.0)).min_h(px(40.0)).bg(
                                        if i < active {
                                            theme.primary.base
                                        } else {
                                            theme.neutral.border
                                        },
                                    )
                                }),
                        )
                    })
            }))
    }
}

impl IntoElement for Steps {
    type Element = gpui::Component<Self>;
    fn into_element(self) -> Self::Element {
        gpui::Component::new(self)
    }
}
