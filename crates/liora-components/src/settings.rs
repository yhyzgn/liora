//! Settings UI module.
//!
//! This module provides layout primitives for native settings screens. It does
//! not replace `Form`; instead it standardizes common desktop settings page
//! structure: pages, groups, explanatory copy, rows, controls, extra content,
//! and semantic item states.

use crate::{Text, Title};
use gpui::{
    AnyElement, App, Component, Hsla, IntoElement, Pixels, RenderOnce, SharedString, Window, div,
    prelude::*, px,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

/// Semantic treatment for one settings row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SettingsItemTone {
    /// Neutral setting row.
    #[default]
    Neutral,
    /// Destructive or security-sensitive setting row.
    Danger,
    /// Highlighted recommendation or active setting row.
    Primary,
}

/// One row inside a [`SettingsGroup`].
pub struct SettingsItem {
    label: SharedString,
    description: Option<SharedString>,
    icon: Option<IconName>,
    control: Option<AnyElement>,
    extra: Option<AnyElement>,
    tone: SettingsItemTone,
    disabled: bool,
    compact: bool,
}

impl SettingsItem {
    /// Creates a settings item with a primary label.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            description: None,
            icon: None,
            control: None,
            extra: None,
            tone: SettingsItemTone::Neutral,
            disabled: false,
            compact: false,
        }
    }

    /// Adds explanatory copy below the label.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Adds a leading icon.
    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Adds the primary trailing control, for example Switch, Select, or Button.
    pub fn control(mut self, control: impl IntoElement) -> Self {
        self.control = Some(control.into_any_element());
        self
    }

    /// Adds secondary content below the main row, for example validation text or inline fields.
    pub fn extra(mut self, extra: impl IntoElement) -> Self {
        self.extra = Some(extra.into_any_element());
        self
    }

    /// Applies danger styling for destructive or sensitive settings.
    pub fn danger(mut self) -> Self {
        self.tone = SettingsItemTone::Danger;
        self
    }

    /// Applies primary styling for recommended or active settings.
    pub fn primary(mut self) -> Self {
        self.tone = SettingsItemTone::Primary;
        self
    }

    /// Toggles disabled visual state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Uses denser vertical padding.
    pub fn compact(mut self) -> Self {
        self.compact = true;
        self
    }

    fn text_color(&self, theme: &liora_theme::Theme) -> Hsla {
        if self.disabled {
            theme.neutral.text_disabled
        } else {
            match self.tone {
                SettingsItemTone::Neutral => theme.neutral.text_1,
                SettingsItemTone::Danger => theme.danger.base,
                SettingsItemTone::Primary => theme.primary.base,
            }
        }
    }

    fn render(self, theme: &liora_theme::Theme) -> AnyElement {
        let fg = self.text_color(theme);
        let py = if self.compact { px(10.0) } else { px(14.0) };
        div()
            .flex()
            .flex_col()
            .gap_2()
            .px_4()
            .py(py)
            .border_b_1()
            .border_color(theme.neutral.border.opacity(0.72))
            .when(self.disabled, |s| s.opacity(0.64))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap_4()
                    .child(
                        div()
                            .flex()
                            .items_start()
                            .gap_3()
                            .min_w(px(0.0))
                            .when_some(self.icon, |s, icon| {
                                s.child(Icon::new(icon).size(px(16.0)).color(fg))
                            })
                            .child(
                                div()
                                    .flex_1()
                                    .min_w(px(0.0))
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .child(Text::new(self.label).sm().bold().text_color(fg).wrap())
                                    .when_some(self.description, |s, description| {
                                        s.child(
                                            Text::new(description)
                                                .xs()
                                                .text_color(theme.neutral.text_3)
                                                .wrap(),
                                        )
                                    }),
                            ),
                    )
                    .when_some(self.control, |s, control| {
                        s.child(div().flex_none().child(control))
                    }),
            )
            .when_some(self.extra, |s, extra| {
                s.child(div().pl(px(28.0)).child(extra))
            })
            .into_any_element()
    }
}

/// A titled group of settings rows.
pub struct SettingsGroup {
    title: SharedString,
    description: Option<SharedString>,
    items: Vec<SettingsItem>,
    footer: Option<AnyElement>,
}

impl SettingsGroup {
    /// Creates an empty group with a title.
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            description: None,
            items: Vec::new(),
            footer: None,
        }
    }

    /// Adds group-level explanatory copy.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Appends an item row.
    pub fn item(mut self, item: SettingsItem) -> Self {
        self.items.push(item);
        self
    }

    /// Replaces all item rows.
    pub fn items(mut self, items: Vec<SettingsItem>) -> Self {
        self.items = items;
        self
    }

    /// Adds footer content below rows.
    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }

    /// Returns item count for tests and dynamic page summaries.
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    fn render(self, theme: &liora_theme::Theme) -> AnyElement {
        let mut card = div()
            .rounded_lg()
            .border_1()
            .border_color(theme.neutral.border)
            .bg(theme.neutral.card)
            .overflow_hidden()
            .child(
                div()
                    .p_4()
                    .border_b_1()
                    .border_color(theme.neutral.border)
                    .child(Title::new(self.title).h4())
                    .when_some(self.description, |s, description| {
                        s.child(
                            Text::new(description)
                                .sm()
                                .text_color(theme.neutral.text_3)
                                .wrap(),
                        )
                    }),
            );
        for item in self.items {
            card = card.child(item.render(theme));
        }
        if let Some(footer) = self.footer {
            card = card.child(
                div()
                    .p_4()
                    .bg(theme.neutral.hover.opacity(0.5))
                    .child(footer),
            );
        }
        card.into_any_element()
    }
}

/// Full settings page composed from groups.
pub struct SettingsPage {
    title: SharedString,
    description: Option<SharedString>,
    groups: Vec<SettingsGroup>,
    max_width: Pixels,
}

impl SettingsPage {
    /// Creates an empty settings page.
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            description: None,
            groups: Vec::new(),
            max_width: px(820.0),
        }
    }

    /// Adds page-level explanatory copy.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Appends a settings group.
    pub fn group(mut self, group: SettingsGroup) -> Self {
        self.groups.push(group);
        self
    }

    /// Replaces all settings groups.
    pub fn groups(mut self, groups: Vec<SettingsGroup>) -> Self {
        self.groups = groups;
        self
    }

    /// Sets maximum page width.
    pub fn max_width(mut self, width: impl Into<Pixels>) -> Self {
        self.max_width = width.into().max(px(360.0));
        self
    }

    /// Returns number of groups for tests and page summaries.
    pub fn group_count(&self) -> usize {
        self.groups.len()
    }
}

impl RenderOnce for SettingsPage {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        div().w_full().flex().justify_center().child(
            div()
                .w_full()
                .max_w(self.max_width)
                .flex()
                .flex_col()
                .gap_5()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(Title::new(self.title).h2())
                        .when_some(self.description, |s, description| {
                            s.child(
                                Text::new(description)
                                    .text_color(theme.neutral.text_3)
                                    .wrap(),
                            )
                        }),
                )
                .children(self.groups.into_iter().map(|group| group.render(&theme))),
        )
    }
}

impl IntoElement for SettingsPage {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_page_tracks_groups_and_items() {
        let group = SettingsGroup::new("Editor")
            .item(SettingsItem::new("Format on save"))
            .item(SettingsItem::new("Tab size"));
        assert_eq!(group.item_count(), 2);
        let page = SettingsPage::new("Settings").group(group);
        assert_eq!(page.group_count(), 1);
    }

    #[test]
    fn settings_item_tracks_tone_and_disabled_state() {
        let item = SettingsItem::new("Delete cache")
            .danger()
            .disabled(true)
            .compact();
        assert_eq!(item.tone, SettingsItemTone::Danger);
        assert!(item.disabled);
        assert!(item.compact);
    }
}
