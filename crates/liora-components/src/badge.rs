use gpui::{
    AnyElement, App, Component, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px,
};
use liora_core::Config;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeType {
    #[default]
    Danger,
    Primary,
    Success,
    Warning,
    Info,
}

pub struct Badge {
    child: AnyElement,
    value: Option<SharedString>,
    max: Option<i32>,
    is_dot: bool,
    hidden: bool,
    badge_type: BadgeType,
}

impl Badge {
    pub fn new(child: impl IntoElement) -> Self {
        Self {
            child: child.into_any_element(),
            value: None,
            max: None,
            is_dot: false,
            hidden: false,
            badge_type: BadgeType::Danger,
        }
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn max(mut self, max: i32) -> Self {
        self.max = Some(max);
        self
    }

    pub fn is_dot(mut self, is_dot: bool) -> Self {
        self.is_dot = is_dot;
        self
    }

    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    pub fn badge_type(mut self, t: BadgeType) -> Self {
        self.badge_type = t;
        self
    }
}

impl RenderOnce for Badge {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        let color = match self.badge_type {
            BadgeType::Danger => theme.danger.base,
            BadgeType::Primary => theme.primary.base,
            BadgeType::Success => theme.success.base,
            BadgeType::Warning => theme.warning.base,
            BadgeType::Info => theme.info.base,
        };

        let badge_content = if self.is_dot {
            div()
                .size(px(8.0))
                .bg(color)
                .rounded_full()
                .border_1()
                .border_color(theme.neutral.body)
        } else {
            let display_value = if let Some(val) = self.value {
                // Try to parse as i32 if max is set
                if let Some(max) = self.max {
                    if let Ok(num) = val.parse::<i32>() {
                        if num > max {
                            format!("{}+", max).into()
                        } else {
                            val
                        }
                    } else {
                        val
                    }
                } else {
                    val
                }
            } else {
                "".into()
            };

            div()
                .flex()
                .items_center()
                .justify_center()
                .h(px(18.0))
                .min_w(px(18.0))
                .px(px(6.0))
                .bg(color)
                .rounded_full()
                .border_1()
                .border_color(theme.neutral.body)
                .text_color(theme.neutral.inverted)
                .text_size(px(10.0))
                .font_weight(gpui::FontWeight::BOLD)
                .child(display_value)
        };

        div().relative().child(self.child).when(!self.hidden, |s| {
            s.child(
                div()
                    .absolute()
                    .top(px(-6.0))
                    .right(px(-6.0))
                    // We use a small negative offset to put it in the top-right corner
                    .child(badge_content),
            )
        })
    }
}

impl IntoElement for Badge {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
