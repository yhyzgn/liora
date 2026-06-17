use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    AnyElement, App, Component, Hsla, IntoElement, Pixels, RenderOnce, SharedString, Window, div,
    prelude::*, px,
};

pub struct Label {
    text: SharedString,
    icon: Option<IconName>,
    custom_icon: Option<AnyElement>,
    gap: Pixels,
    color: Option<Hsla>,
    size: Pixels,
}

impl Label {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            icon: None,
            custom_icon: None,
            gap: px(6.0),
            color: None,
            size: px(13.0),
        }
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }
    pub fn custom_icon(mut self, icon: impl IntoElement) -> Self {
        self.custom_icon = Some(icon.into_any_element());
        self
    }
    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = gap.into().max(px(0.0));
        self
    }
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = size.into().max(px(8.0));
        self
    }
}

impl RenderOnce for Label {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let color = self.color.unwrap_or(theme.neutral.text_2);
        let has_custom_icon = self.custom_icon.is_some();
        div()
            .flex()
            .items_center()
            .gap(self.gap)
            .text_size(self.size)
            .text_color(color)
            .when_some(self.custom_icon, |s, icon| s.child(icon))
            .when(!has_custom_icon, |s| {
                if let Some(icon) = self.icon {
                    s.child(Icon::new(icon).size(self.size).color(color))
                } else {
                    s
                }
            })
            .child(self.text)
    }
}

impl IntoElement for Label {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn label_builders_track_state() {
        let label = Label::new("CPU")
            .icon(IconName::Activity)
            .gap(px(10.0))
            .size(px(15.0));
        assert_eq!(label.gap, px(10.0));
        assert_eq!(label.size, px(15.0));
        assert_eq!(label.icon, Some(IconName::Activity));
    }
}
