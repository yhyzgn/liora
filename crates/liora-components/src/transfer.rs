use gpui::{
    App, Context, Entity, IntoElement, MouseButton, Render, SharedString, Window, div, prelude::*,
    px,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferItem {
    pub key: SharedString,
    pub label: SharedString,
    pub description: Option<SharedString>,
    pub disabled: bool,
}

pub struct Transfer {
    id: SharedString,
    items: Vec<TransferItem>,
    target_keys: Vec<SharedString>,
    checked_source_keys: Vec<SharedString>,
    checked_target_keys: Vec<SharedString>,
    source_title: SharedString,
    target_title: SharedString,
    source_filter: SharedString,
    target_filter: SharedString,
    filterable: bool,
    empty_text: SharedString,
    width: gpui::Pixels,
    height: gpui::Pixels,
    on_change: Option<Arc<dyn Fn(Vec<SharedString>, &mut Window, &mut App) + 'static>>,
}

impl TransferItem {
    pub fn new(key: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            description: None,
            disabled: false,
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Transfer {
    pub fn new(items: Vec<TransferItem>) -> Self {
        Self {
            id: liora_core::unique_id("transfer"),
            items,
            target_keys: vec![],
            checked_source_keys: vec![],
            checked_target_keys: vec![],
            source_title: "Source".into(),
            target_title: "Target".into(),
            source_filter: SharedString::default(),
            target_filter: SharedString::default(),
            filterable: false,
            empty_text: "暂无数据".into(),
            width: px(620.0),
            height: px(300.0),
            on_change: None,
        }
    }

    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    pub fn target_keys(mut self, keys: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        self.target_keys = keys.into_iter().map(Into::into).collect();
        self
    }

    pub fn checked_source_keys(
        mut self,
        keys: impl IntoIterator<Item = impl Into<SharedString>>,
    ) -> Self {
        self.checked_source_keys = keys.into_iter().map(Into::into).collect();
        self
    }

    pub fn checked_target_keys(
        mut self,
        keys: impl IntoIterator<Item = impl Into<SharedString>>,
    ) -> Self {
        self.checked_target_keys = keys.into_iter().map(Into::into).collect();
        self
    }

    pub fn titles(
        mut self,
        source: impl Into<SharedString>,
        target: impl Into<SharedString>,
    ) -> Self {
        self.source_title = source.into();
        self.target_title = target.into();
        self
    }

    pub fn filterable(mut self, filterable: bool) -> Self {
        self.filterable = filterable;
        self
    }

    pub fn source_filter(mut self, query: impl Into<SharedString>) -> Self {
        self.source_filter = query.into();
        self
    }

    pub fn target_filter(mut self, query: impl Into<SharedString>) -> Self {
        self.target_filter = query.into();
        self
    }

    pub fn empty_text(mut self, text: impl Into<SharedString>) -> Self {
        self.empty_text = text.into();
        self
    }

    pub fn width(mut self, width: impl Into<gpui::Pixels>) -> Self {
        self.width = width.into();
        self
    }

    pub fn width_lg(self) -> Self {
        self.width(px(680.0))
    }

    pub fn height(mut self, height: impl Into<gpui::Pixels>) -> Self {
        self.height = height.into();
        self
    }

    pub fn on_change(
        mut self,
        f: impl Fn(Vec<SharedString>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Arc::new(f));
        self
    }

    pub fn set_target_keys(&mut self, keys: impl IntoIterator<Item = impl Into<SharedString>>) {
        self.target_keys = keys.into_iter().map(Into::into).collect();
    }

    pub fn filter_items(items: &[TransferItem], query: &str) -> Vec<SharedString> {
        let query = query.trim().to_lowercase();
        items
            .iter()
            .filter(|item| {
                query.is_empty()
                    || item.key.to_string().to_lowercase().contains(&query)
                    || item.label.to_string().to_lowercase().contains(&query)
                    || item
                        .description
                        .as_ref()
                        .is_some_and(|desc| desc.to_string().to_lowercase().contains(&query))
            })
            .map(|item| item.key.clone())
            .collect()
    }

    pub fn move_to_target(
        items: &[TransferItem],
        target_keys: &mut Vec<SharedString>,
        checked_source_keys: &mut Vec<SharedString>,
    ) -> Vec<SharedString> {
        let disabled = disabled_keys(items);
        let mut moved = Vec::new();
        for key in checked_source_keys.iter() {
            if disabled.contains(key) || target_keys.contains(key) {
                continue;
            }
            target_keys.push(key.clone());
            moved.push(key.clone());
        }
        checked_source_keys.clear();
        moved
    }

    pub fn move_to_source(
        items: &[TransferItem],
        target_keys: &mut Vec<SharedString>,
        checked_target_keys: &mut Vec<SharedString>,
    ) -> Vec<SharedString> {
        let disabled = disabled_keys(items);
        let mut moved = Vec::new();
        target_keys.retain(|key| {
            let should_move = checked_target_keys.contains(key) && !disabled.contains(key);
            if should_move {
                moved.push(key.clone());
            }
            !should_move
        });
        checked_target_keys.clear();
        moved
    }

    pub fn move_to_target_with_checked(
        items: &[TransferItem],
        target_keys: &mut Vec<SharedString>,
        checked_source_keys: &mut Vec<SharedString>,
        checked_target_keys: &mut Vec<SharedString>,
    ) -> Vec<SharedString> {
        let moved = Self::move_to_target(items, target_keys, checked_source_keys);
        for key in &moved {
            if !checked_target_keys.contains(key) {
                checked_target_keys.push(key.clone());
            }
        }
        moved
    }

    pub fn move_to_source_with_checked(
        items: &[TransferItem],
        target_keys: &mut Vec<SharedString>,
        checked_target_keys: &mut Vec<SharedString>,
        checked_source_keys: &mut Vec<SharedString>,
    ) -> Vec<SharedString> {
        let moved = Self::move_to_source(items, target_keys, checked_target_keys);
        for key in &moved {
            if !checked_source_keys.contains(key) {
                checked_source_keys.push(key.clone());
            }
        }
        moved
    }

    fn source_items(&self) -> Vec<TransferItem> {
        self.items
            .iter()
            .filter(|item| !self.target_keys.contains(&item.key))
            .cloned()
            .collect()
    }

    fn target_items(&self) -> Vec<TransferItem> {
        self.target_keys
            .iter()
            .filter_map(|key| self.items.iter().find(|item| &item.key == key).cloned())
            .collect()
    }

    fn toggle_source_key(&mut self, key: SharedString) {
        toggle_key(&mut self.checked_source_keys, key);
    }

    fn toggle_target_key(&mut self, key: SharedString) {
        toggle_key(&mut self.checked_target_keys, key);
    }

    fn move_checked_to_target(&mut self, window: &mut Window, cx: &mut App) {
        let moved = Self::move_to_target_with_checked(
            &self.items,
            &mut self.target_keys,
            &mut self.checked_source_keys,
            &mut self.checked_target_keys,
        );
        if !moved.is_empty() {
            if let Some(on_change) = &self.on_change {
                on_change(self.target_keys.clone(), window, cx);
            }
        }
    }

    fn move_checked_to_source(&mut self, window: &mut Window, cx: &mut App) {
        let moved = Self::move_to_source_with_checked(
            &self.items,
            &mut self.target_keys,
            &mut self.checked_target_keys,
            &mut self.checked_source_keys,
        );
        if !moved.is_empty() {
            if let Some(on_change) = &self.on_change {
                on_change(self.target_keys.clone(), window, cx);
            }
        }
    }
}

impl Render for Transfer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let source_items = self.source_items();
        let target_items = self.target_items();
        let can_move_right = has_enabled_checked(&source_items, &self.checked_source_keys);
        let can_move_left = has_enabled_checked(&target_items, &self.checked_target_keys);
        let entity = cx.entity().clone();
        let id = self.id.clone();
        let width = self.width;
        let height = self.height;

        div()
            .flex()
            .items_center()
            .gap_4()
            .w(width)
            .child(render_panel(
                format!("{}-source", id),
                self.source_title.clone(),
                source_items,
                self.checked_source_keys.clone(),
                self.source_filter.clone(),
                self.filterable,
                self.empty_text.clone(),
                height,
                true,
                &theme,
                entity.clone(),
            ))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(action_button(
                        format!("{}-to-target", id),
                        IconName::ChevronRight,
                        can_move_right,
                        theme.clone(),
                        entity.clone(),
                        true,
                    ))
                    .child(action_button(
                        format!("{}-to-source", id),
                        IconName::ChevronLeft,
                        can_move_left,
                        theme.clone(),
                        entity.clone(),
                        false,
                    )),
            )
            .child(render_panel(
                format!("{}-target", id),
                self.target_title.clone(),
                target_items,
                self.checked_target_keys.clone(),
                self.target_filter.clone(),
                self.filterable,
                self.empty_text.clone(),
                height,
                false,
                &theme,
                entity,
            ))
    }
}

#[allow(clippy::too_many_arguments)]
fn render_panel(
    id: String,
    title: SharedString,
    items: Vec<TransferItem>,
    checked_keys: Vec<SharedString>,
    filter_query: SharedString,
    filterable: bool,
    empty_text: SharedString,
    height: gpui::Pixels,
    is_source: bool,
    theme: &liora_theme::Theme,
    transfer: Entity<Transfer>,
) -> impl IntoElement {
    let visible_keys = Transfer::filter_items(&items, filter_query.as_ref());
    let visible_key_set = visible_keys.iter().cloned().collect::<HashSet<_>>();
    let enabled_count = items.iter().filter(|item| !item.disabled).count();
    let checked_count = checked_keys.len();

    div()
        .id(id.clone())
        .flex()
        .flex_col()
        .w(px(260.0))
        .h(height)
        .bg(theme.neutral.card)
        .border_1()
        .border_color(theme.neutral.border)
        .rounded(px(theme.radius.md))
        .overflow_hidden()
        .child(
            div()
                .h(px(42.0))
                .px_3()
                .flex()
                .items_center()
                .justify_between()
                .bg(theme.neutral.hover)
                .border_b_1()
                .border_color(theme.neutral.border)
                .child(
                    div()
                        .font_weight(gpui::FontWeight::BOLD)
                        .text_color(theme.neutral.text_1)
                        .child(title),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.neutral.text_3)
                        .child(format!("{checked_count}/{enabled_count}")),
                ),
        )
        .when(filterable, |panel| {
            panel.child(
                div()
                    .px_3()
                    .py_2()
                    .border_b_1()
                    .border_color(theme.neutral.divider)
                    .child(
                        div()
                            .h(px(30.0))
                            .px_2()
                            .flex()
                            .items_center()
                            .gap_2()
                            .rounded(px(theme.radius.sm))
                            .border_1()
                            .border_color(theme.neutral.border)
                            .text_sm()
                            .text_color(if filter_query.is_empty() {
                                theme.neutral.placeholder
                            } else {
                                theme.neutral.text_1
                            })
                            .child(
                                Icon::new(IconName::Search)
                                    .size(px(14.0))
                                    .color(theme.neutral.icon),
                            )
                            .child(if filter_query.is_empty() {
                                "Filter...".into()
                            } else {
                                filter_query.clone()
                            }),
                    ),
            )
        })
        .child(
            div()
                .id(format!("{}-list", id))
                .flex_1()
                .overflow_y_scroll()
                .children(
                    items
                        .into_iter()
                        .filter(move |item| visible_key_set.contains(&item.key))
                        .map(move |item| {
                            render_item(
                                id.clone(),
                                item,
                                checked_keys.clone(),
                                is_source,
                                theme.clone(),
                                transfer.clone(),
                            )
                        }),
                )
                .when(visible_keys.is_empty(), |list| {
                    list.child(
                        div()
                            .h_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child(empty_text),
                    )
                }),
        )
}

fn render_item(
    panel_id: String,
    item: TransferItem,
    checked_keys: Vec<SharedString>,
    is_source: bool,
    theme: liora_theme::Theme,
    transfer: Entity<Transfer>,
) -> impl IntoElement {
    let checked = checked_keys.contains(&item.key);
    let disabled = item.disabled;
    let key = item.key.clone();
    let item_id = format!("{}-item-{}", panel_id, key);

    div()
        .id(item_id)
        .min_h(px(38.0))
        .px_3()
        .py_2()
        .flex()
        .items_center()
        .gap_2()
        .text_color(if disabled {
            theme.neutral.text_disabled
        } else {
            theme.neutral.text_1
        })
        .when(!disabled, |s| {
            s.cursor_pointer()
                .hover(|s| s.bg(theme.neutral.hover).cursor_pointer())
        })
        .when(disabled, |s| s.cursor_not_allowed())
        .child(check_box_visual(checked, disabled, &theme))
        .child(
            div()
                .flex_1()
                .flex()
                .flex_col()
                .gap_1()
                .child(div().text_sm().child(item.label))
                .when_some(item.description, |s, desc| {
                    s.child(div().text_xs().text_color(theme.neutral.text_3).child(desc))
                }),
        )
        .when(!disabled, |s| {
            s.on_mouse_down(MouseButton::Left, move |_, _, cx| {
                transfer.update(cx, |transfer, cx| {
                    if is_source {
                        transfer.toggle_source_key(key.clone());
                    } else {
                        transfer.toggle_target_key(key.clone());
                    }
                    cx.notify();
                });
            })
        })
}

fn check_box_visual(checked: bool, disabled: bool, theme: &liora_theme::Theme) -> impl IntoElement {
    div()
        .w(px(16.0))
        .h(px(16.0))
        .rounded(px(2.0))
        .border_1()
        .border_color(if checked {
            theme.primary.base
        } else {
            theme.neutral.border
        })
        .bg(if checked {
            theme.primary.base
        } else {
            theme.neutral.card
        })
        .opacity(if disabled { 0.45 } else { 1.0 })
        .flex()
        .items_center()
        .justify_center()
        .when(checked, |s| {
            s.child(
                Icon::new(IconName::Check)
                    .size(px(12.0))
                    .color(theme.neutral.card),
            )
        })
}

fn action_button(
    id: String,
    icon: IconName,
    enabled: bool,
    theme: liora_theme::Theme,
    transfer: Entity<Transfer>,
    to_target: bool,
) -> impl IntoElement {
    div()
        .id(id)
        .w(px(34.0))
        .h(px(30.0))
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(theme.radius.md))
        .bg(if enabled {
            theme.primary.base
        } else {
            theme.neutral.hover
        })
        .text_color(if enabled {
            theme.neutral.card
        } else {
            theme.neutral.text_3
        })
        .when(enabled, |s| {
            s.cursor_pointer()
                .hover(|s| s.bg(theme.primary.hover).cursor_pointer())
                .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                    transfer.update(cx, |transfer, cx| {
                        if to_target {
                            transfer.move_checked_to_target(window, cx);
                        } else {
                            transfer.move_checked_to_source(window, cx);
                        }
                        cx.notify();
                    });
                })
        })
        .when(!enabled, |s| s.cursor_not_allowed())
        .child(Icon::new(icon).size(px(16.0)).color(if enabled {
            theme.neutral.card
        } else {
            theme.neutral.text_3
        }))
}

fn toggle_key(keys: &mut Vec<SharedString>, key: SharedString) {
    if keys.contains(&key) {
        keys.retain(|existing| existing != &key);
    } else {
        keys.push(key);
    }
}

fn disabled_keys(items: &[TransferItem]) -> HashSet<SharedString> {
    items
        .iter()
        .filter(|item| item.disabled)
        .map(|item| item.key.clone())
        .collect()
}

fn has_enabled_checked(items: &[TransferItem], checked_keys: &[SharedString]) -> bool {
    items
        .iter()
        .any(|item| !item.disabled && checked_keys.contains(&item.key))
}

#[cfg(test)]
mod demo_width_tests {
    use super::*;

    #[test]
    fn transfer_width_lg_sets_demo_width() {
        assert_eq!(
            Transfer::new(vec![TransferItem::new("a", "A")])
                .width_lg()
                .width,
            px(680.0)
        );
    }
}
