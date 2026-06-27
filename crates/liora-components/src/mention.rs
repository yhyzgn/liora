//! Mention module.
//!
//! This public module implements the Liora mention input helpers for trigger-based suggestions. It keeps the reusable
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

use crate::Input;
use crate::gpui_compat::element_id;
use gpui::{
    App, Context, Entity, KeyBinding, MouseButton, Render, SharedString, Window, actions, div,
    prelude::*, px,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

actions!(
    mention,
    [
        #[doc = "Keyboard action that moves mention selection upward."]
        MentionUp,
        #[doc = "Keyboard action that moves mention selection downward."]
        MentionDown,
        #[doc = "Keyboard action that selects the active mention option."]
        MentionEnter
    ]
);

#[derive(Clone, Debug, PartialEq, Eq)]
/// Data model used by mention item rendering.
pub struct MentionItem {
    /// Machine-readable value represented by this item.
    pub value: SharedString,
    /// User-facing label rendered for this item.
    pub label: SharedString,
    /// Supporting descriptive text shown near the primary label.
    pub description: Option<SharedString>,
}

impl MentionItem {
    /// Creates `MentionItem` initialized from the supplied value, and label.
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            description: None,
        }
    }
    /// Sets secondary descriptive text shown below the primary label.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// Fluent native GPUI component for rendering Liora mention.
pub struct Mention {
    input: Entity<Input>,
    trigger: char,
    suggestions: Vec<MentionItem>,
    max_suggestions: usize,
    placeholder: SharedString,
    disabled: bool,
    selected_index: usize,
    on_select: Option<std::sync::Arc<dyn Fn(MentionItem, &mut Window, &mut App) + 'static>>,
}

impl Mention {
    /// Registers GPUI key bindings required for keyboard interaction.
    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([
            KeyBinding::new("up", MentionUp, None),
            KeyBinding::new("down", MentionDown, None),
            KeyBinding::new("enter", MentionEnter, None),
        ]);
    }

    /// Creates `Mention` with default theme-driven styling and no optional callbacks attached.
    pub fn new(suggestions: Vec<MentionItem>, cx: &mut Context<Self>) -> Self {
        Self {
            input: cx.new(|cx| Input::new("", cx)),
            trigger: '@',
            suggestions,
            max_suggestions: 6,
            placeholder: "Type @ to mention".into(),
            disabled: false,
            selected_index: 0,
            on_select: None,
        }
    }
    /// Creates a GPUI entity that owns this component state across render passes.
    pub fn entity(suggestions: Vec<MentionItem>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(suggestions, cx))
    }
    /// Sets the trigger value used by the component.
    pub fn trigger(mut self, trigger: char) -> Self {
        self.trigger = trigger;
        self
    }
    /// Uses the supplied placeholder text when the value is empty.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }
    /// Limits how many suggestions are displayed in the popup.
    pub fn max_suggestions(mut self, max: usize) -> Self {
        self.max_suggestions = max.max(1);
        self
    }
    /// Toggles the disabled state and suppresses user interaction when enabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    /// Registers a callback that runs when select occurs.
    pub fn on_select(mut self, cb: impl Fn(MentionItem, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(std::sync::Arc::new(cb));
        self
    }
    /// Performs the input operation used by this component.
    pub fn input(&self) -> Entity<Input> {
        self.input.clone()
    }
    /// Performs the query for text operation used by this component.
    pub fn query_for_text(text: &str, trigger: char) -> Option<&str> {
        mention_query(text, trigger)
    }
    /// Performs the filtered suggestions operation used by this component.
    pub fn filtered_suggestions(&self, query: &str) -> Vec<MentionItem> {
        filter_suggestions(&self.suggestions, query, self.max_suggestions)
    }

    fn current_suggestions(&self, cx: &App) -> Vec<MentionItem> {
        let text = self.input.read(cx).value().to_string();
        let Some(query) = mention_query(&text, self.trigger) else {
            return Vec::new();
        };
        self.filtered_suggestions(query)
    }

    fn move_selection(&mut self, delta: isize, cx: &mut Context<Self>) {
        let suggestions = self.current_suggestions(cx);
        if suggestions.is_empty() {
            self.selected_index = 0;
            return;
        }
        let len = suggestions.len();
        let current = self.selected_index.min(len - 1);
        self.selected_index = if delta < 0 {
            current.checked_sub(1).unwrap_or(len - 1)
        } else {
            (current + 1) % len
        };
        cx.notify();
    }

    fn select_active(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let suggestions = self.current_suggestions(cx);
        let Some(item) = suggestions
            .get(self.selected_index.min(suggestions.len().saturating_sub(1)))
            .cloned()
        else {
            return;
        };
        self.select_item(item, window, cx);
    }

    fn hover_option(&mut self, index: usize, cx: &mut Context<Self>) {
        if self.selected_index != index {
            self.selected_index = index;
            cx.notify();
        }
    }

    fn select_item(&mut self, item: MentionItem, window: &mut Window, cx: &mut Context<Self>) {
        let input = self.input.clone();
        let trigger = self.trigger;
        let value = item.value.clone();
        cx.update_entity(&input, |input, cx| {
            let next = apply_mention_selection(input.value().as_ref(), trigger, value.as_ref());
            input.set_value(next, cx);
        });
        self.selected_index = 0;
        if let Some(cb) = self.on_select.clone() {
            cb(item, window, cx);
        }
        cx.notify();
    }

    fn move_up_action(&mut self, _: &MentionUp, _: &mut Window, cx: &mut Context<Self>) {
        self.move_selection(-1, cx);
    }

    fn move_down_action(&mut self, _: &MentionDown, _: &mut Window, cx: &mut Context<Self>) {
        self.move_selection(1, cx);
    }

    fn enter_action(&mut self, _: &MentionEnter, window: &mut Window, cx: &mut Context<Self>) {
        self.select_active(window, cx);
    }
}

impl Render for Mention {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let input = self.input.clone();
        let placeholder = self.placeholder.clone();
        let disabled = self.disabled;
        cx.update_entity(&input, |input, cx| {
            input.set_placeholder(placeholder, cx);
            input.set_disabled(disabled, cx);
        });
        let text = input.read(cx).value().to_string();
        let query = mention_query(&text, self.trigger).unwrap_or("");
        let open = !disabled
            && text.contains(self.trigger)
            && mention_query(&text, self.trigger).is_some();
        let suggestions = if open {
            self.filtered_suggestions(query)
        } else {
            Vec::new()
        };
        let entity = cx.entity().clone();

        div()
            .id(liora_core::unique_id("mention"))
            .relative()
            .flex()
            .flex_col()
            .gap_2()
            .child(input)
            .when(open, |s| {
                s.child(
                    div()
                        .rounded_md()
                        .border_1()
                        .border_color(theme.neutral.border)
                        .bg(theme.neutral.card)
                        .shadow_lg()
                        .overflow_hidden()
                        .children(suggestions.into_iter().enumerate().map(|(idx, item)| {
                            let active = idx
                                == self
                                    .selected_index
                                    .min(self.max_suggestions.saturating_sub(1));
                            let selected_item = item.clone();
                            let mut row = div()
                                .id(element_id(format!("mention-option-{}", idx)))
                                .flex()
                                .items_center()
                                .gap_2()
                                .px_3()
                                .py_2()
                                .cursor_pointer()
                                .bg(if active {
                                    theme.primary.base.opacity(0.08)
                                } else {
                                    gpui::transparent_black()
                                })
                                .hover(|s| s.bg(theme.neutral.hover))
                                .on_mouse_move({
                                    let entity = entity.clone();
                                    move |_, _, cx| {
                                        entity.update(cx, |this, cx| this.hover_option(idx, cx));
                                    }
                                })
                                .child(
                                    Icon::new(IconName::AtSign)
                                        .size(px(15.0))
                                        .color(theme.primary.base),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(theme.neutral.text_1)
                                                .child(item.label),
                                        )
                                        .when_some(item.description, |s, description| {
                                            s.child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.neutral.text_3)
                                                    .child(description),
                                            )
                                        }),
                                );
                            row = row.on_mouse_down(MouseButton::Left, {
                                let entity = entity.clone();
                                move |_, window, cx| {
                                    let selected_item = selected_item.clone();
                                    entity.update(cx, |this, cx| {
                                        this.selected_index = idx;
                                        this.select_item(selected_item, window, cx);
                                    });
                                    cx.stop_propagation();
                                }
                            });
                            row.into_any_element()
                        })),
                )
            })
            .on_action(cx.listener(Self::move_up_action))
            .on_action(cx.listener(Self::move_down_action))
            .on_action(cx.listener(Self::enter_action))
    }
}

fn mention_query(text: &str, trigger: char) -> Option<&str> {
    let last = text.rfind(trigger)?;
    let query = &text[last + trigger.len_utf8()..];
    if query.chars().any(char::is_whitespace) {
        None
    } else {
        Some(query)
    }
}

fn apply_mention_selection(text: &str, trigger: char, value: &str) -> String {
    let mention = format!("{trigger}{value} ");
    let Some(last) = text.rfind(trigger) else {
        let separator = if text.is_empty() || text.ends_with(char::is_whitespace) {
            ""
        } else {
            " "
        };
        return format!("{text}{separator}{mention}");
    };
    let query_start = last + trigger.len_utf8();
    let query = &text[query_start..];
    if query.chars().any(char::is_whitespace) {
        let separator = if text.is_empty() || text.ends_with(char::is_whitespace) {
            ""
        } else {
            " "
        };
        format!("{text}{separator}{mention}")
    } else {
        format!("{}{}", &text[..last], mention)
    }
}

fn filter_suggestions(items: &[MentionItem], query: &str, limit: usize) -> Vec<MentionItem> {
    let query = query.to_lowercase();
    items
        .iter()
        .filter(|item| {
            query.is_empty()
                || item.value.to_lowercase().contains(&query)
                || item.label.to_lowercase().contains(&query)
        })
        .take(limit.max(1))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mention_query_reads_last_trigger_until_space() {
        assert_eq!(Mention::query_for_text("hello @al", '@'), Some("al"));
        assert_eq!(Mention::query_for_text("hello @al ice", '@'), None);
    }

    #[test]
    fn mention_selection_replaces_active_query_with_triggered_value() {
        assert_eq!(
            apply_mention_selection("hello @al", '@', "alice"),
            "hello @alice "
        );
        assert_eq!(apply_mention_selection("fix #1", '#', "128"), "fix #128 ");
        assert_eq!(
            apply_mention_selection("ping @al and @bo", '@', "bob"),
            "ping @al and @bob "
        );
        assert_eq!(
            apply_mention_selection("hello", '@', "alice"),
            "hello @alice "
        );
    }

    #[test]
    fn mention_filter_matches_value_or_label_and_caps() {
        let items = vec![
            MentionItem::new("alice", "Alice"),
            MentionItem::new("bob", "Bob"),
            MentionItem::new("ops", "Operations"),
        ];
        let out = filter_suggestions(&items, "o", 1);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].value.as_ref(), "bob");
    }

    #[test]
    fn mention_popup_supports_mouse_hover_click_and_keyboard_selection() {
        let source = include_str!("mention.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap_or_default();

        assert!(source.contains("MentionUp"));
        assert!(source.contains("MentionDown"));
        assert!(source.contains("MentionEnter"));
        assert!(source.contains("KeyBinding::new(\"up\", MentionUp"));
        assert!(source.contains("KeyBinding::new(\"down\", MentionDown"));
        assert!(source.contains("KeyBinding::new(\"enter\", MentionEnter"));
        assert!(source.contains("fn move_selection"));
        assert!(source.contains("fn select_active"));
        assert!(source.contains("fn hover_option"));
        assert!(source.contains("apply_mention_selection"));
        assert!(source.contains("input.set_value"));
        assert!(source.contains(".on_mouse_move("));
        assert!(source.contains(".on_mouse_down(MouseButton::Left"));
        assert!(source.contains(".on_action(cx.listener(Self::move_up_action))"));
        assert!(source.contains(".on_action(cx.listener(Self::move_down_action))"));
        assert!(source.contains(".on_action(cx.listener(Self::enter_action))"));
    }
}
