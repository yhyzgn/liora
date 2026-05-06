use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    AnyElement, App, Hsla, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimelineMode {
    #[default]
    Left,
    Right,
    Alternate,
}

pub struct TimelineItem {
    pub timestamp: Option<SharedString>,
    pub content: AnyElement,
    pub color: Option<Hsla>,
    pub icon: Option<IconName>,
    pub hollow: bool,
    pub hide_timestamp: bool,
    pub placement: TimelinePlacement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimelinePlacement {
    #[default]
    Top,
    Bottom,
}

pub struct Timeline {
    items: Vec<TimelineItem>,
    reverse: bool,
    mode: TimelineMode,
}

impl TimelineItem {
    pub fn new() -> Self {
        Self {
            timestamp: None,
            content: div().into_any_element(),
            color: None,
            icon: None,
            hollow: false,
            hide_timestamp: false,
            placement: TimelinePlacement::Bottom,
        }
    }

    pub fn timestamp(mut self, t: impl Into<SharedString>) -> Self {
        self.timestamp = Some(t.into());
        self
    }

    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = content.into_any_element();
        self
    }

    pub fn color(mut self, c: Hsla) -> Self {
        self.color = Some(c);
        self
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn hollow(mut self, h: bool) -> Self {
        self.hollow = h;
        self
    }

    pub fn placement(mut self, p: TimelinePlacement) -> Self {
        self.placement = p;
        self
    }

    pub fn hide_timestamp(mut self, hide: bool) -> Self {
        self.hide_timestamp = hide;
        self
    }
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            items: vec![],
            reverse: false,
            mode: TimelineMode::Left,
        }
    }

    pub fn reverse(mut self, r: bool) -> Self {
        self.reverse = r;
        self
    }

    pub fn mode(mut self, m: TimelineMode) -> Self {
        self.mode = m;
        self
    }

    pub fn item(mut self, item: TimelineItem) -> Self {
        self.items.push(item);
        self
    }
}

impl RenderOnce for Timeline {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let mut items = self.items;
        if self.reverse {
            items.reverse();
        }
        let items_count = items.len();

        div()
            .flex()
            .flex_col()
            .w_full()
            .children(items.into_iter().enumerate().map(|(i, item)| {
                let is_last = i == items_count - 1;
                let dot_color = item.color.unwrap_or(theme.neutral.border);
                let text_color = theme.neutral.text_2;
                let timestamp_color = theme.neutral.text_3;

                div()
                    .flex()
                    .flex_row()
                    .gap_3()
                    .relative()
                    .child(
                        // Left: Axis & Node
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .w(px(20.0))
                            .child(
                                // Node
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .w(px(12.0))
                                    .h(px(12.0))
                                    .mt(px(4.0))
                                    .rounded_full()
                                    .bg(if item.hollow {
                                        theme.neutral.card
                                    } else {
                                        dot_color
                                    })
                                    .border_2()
                                    .border_color(dot_color)
                                    .when_some(item.icon, |s, icon| {
                                        // If icon, use icon instead of dot
                                        s.size(px(20.0))
                                            .mt(px(0.0))
                                            .bg(gpui::transparent_black())
                                            .border_0()
                                            .child(Icon::new(icon).size(px(14.0)).color(dot_color))
                                    }),
                            )
                            .when(!is_last, |s| {
                                s.child(
                                    // Vertical Line
                                    div().flex_1().w(px(2.0)).bg(theme.neutral.border),
                                )
                            }),
                    )
                    .child(
                        // Right: Content & Timestamp
                        div()
                            .flex()
                            .flex_col()
                            .pb_6()
                            .flex_1()
                            .when(
                                item.placement == TimelinePlacement::Top && !item.hide_timestamp,
                                |s| {
                                    s.when_some(item.timestamp.clone(), |s, t| {
                                        s.child(
                                            div()
                                                .text_xs()
                                                .text_color(timestamp_color)
                                                .mb_1()
                                                .child(t),
                                        )
                                    })
                                },
                            )
                            .child(div().text_sm().text_color(text_color).child(item.content))
                            .when(
                                item.placement == TimelinePlacement::Bottom && !item.hide_timestamp,
                                |s| {
                                    s.when_some(item.timestamp, |s, t| {
                                        s.child(
                                            div()
                                                .text_xs()
                                                .text_color(timestamp_color)
                                                .mt_2()
                                                .child(t),
                                        )
                                    })
                                },
                            ),
                    )
            }))
    }
}

impl IntoElement for Timeline {
    type Element = gpui::Component<Self>;
    fn into_element(self) -> Self::Element {
        gpui::Component::new(self)
    }
}
