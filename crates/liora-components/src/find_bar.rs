//! Native GPUI find bar inspired by desktop find-in-page controls.
//!
//! The component owns query input and navigation events only. Match calculation,
//! text indexing, and scrolling remain with the host surface.
//!
//! Focus with [`FindBar::focus`] or [`Focusable`]. Enter requests the next match,
//! Shift-Enter the previous match, and Escape closes. Native marked composition
//! and modified shortcuts remain with the input. Cursor-only notifications do
//! not invalidate host-provided match status or emit a query change.
//! Programmatic `set_query` invalidates status synchronously, so a subsequent
//! `set_match_status` in the same update is retained. For native edits, provide
//! status in response to `QueryChanged`, after the input notification is observed.

use crate::{Button, Input};
use gpui::{
    App, Context, Entity, EntityInputHandler, EventEmitter, FocusHandle, Focusable, IntoElement,
    KeyDownEvent, Render, SharedString, Window, div, prelude::*,
};
use liora_core::Config;
use liora_icons_lucide::IconName;

gpui::actions!(
    find_bar,
    [
        /// Keyboard action that closes the find bar.
        CloseFind
    ]
);

/// Events emitted by [`FindBar`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FindBarEvent {
    /// The host should recompute matches for this query.
    QueryChanged {
        /// Updated query text.
        query: SharedString,
    },
    /// The host should reveal the previous match.
    Previous,
    /// The host should reveal the next match.
    Next,
    /// The host should close or hide the find surface.
    Closed,
}

/// Compact native query bar for document, editor, and PDF surfaces.
pub struct FindBar {
    id: SharedString,
    input: Entity<Input>,
    match_status: SharedString,
    last_emitted_query: SharedString,
    _input_subscription: gpui::Subscription,
}

impl EventEmitter<FindBarEvent> for FindBar {}

impl FindBar {
    /// Registers the scoped Escape binding; called by `init_liora`.
    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([gpui::KeyBinding::new("escape", CloseFind, Some("FindBar"))]);
    }

    /// Creates an empty find bar with a native GPUI input.
    pub fn new(cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| {
            Input::new("", cx)
                .id("liora-find-query")
                .placeholder("Find in page")
                .clearable(true)
                .icon_suffix(IconName::Search)
                .min_rows(1)
        });
        let input_subscription = cx.observe(&input, |bar, _, cx| bar.sync_query(cx));
        Self {
            id: liora_core::unique_id("find-bar"),
            input,
            match_status: "".into(),
            last_emitted_query: "".into(),
            _input_subscription: input_subscription,
        }
    }

    /// Returns the native query input entity for host-side customization.
    pub fn input(&self) -> Entity<Input> {
        self.input.clone()
    }
    /// Returns the current query.
    pub fn query(&self, cx: &App) -> SharedString {
        self.input.read(cx).value()
    }

    /// Replaces the query and emits `QueryChanged` before returning if it changed.
    pub fn set_query(&mut self, query: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.input
            .update(cx, |input, cx| input.set_value(query.into(), cx));
        self.sync_query(cx);
    }

    /// Focuses the native query field on the next window turn.
    pub fn focus(&self, window: &mut Window, cx: &mut Context<Self>) {
        let handle = self.input.read(cx).focus_handle(cx);
        window.defer(cx, move |window, cx| window.focus(&handle, cx));
    }

    /// Shows host-provided match information, such as `2 of 9` or `No results`.
    pub fn set_match_status(&mut self, status: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.match_status = status.into();
        cx.notify();
    }

    /// Requests the previous host-side match.
    pub fn previous(&self, cx: &mut Context<Self>) {
        cx.emit(FindBarEvent::Previous);
    }
    /// Requests the next host-side match.
    pub fn next(&self, cx: &mut Context<Self>) {
        cx.emit(FindBarEvent::Next);
    }
    /// Requests that the host hide the find surface.
    pub fn close(&self, cx: &mut Context<Self>) {
        cx.emit(FindBarEvent::Closed);
    }
    fn sync_query(&mut self, cx: &mut Context<Self>) {
        let query = self.query(cx);
        if update_query(&mut self.last_emitted_query, &query) {
            self.match_status = "".into();
            cx.emit(FindBarEvent::QueryChanged { query });
            cx.notify();
        }
    }
    fn handle_key(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        self.handle_command(
            event.keystroke.key.as_str(),
            event.keystroke.modifiers,
            window,
            cx,
        );
    }
    fn handle_command(
        &mut self,
        key: &str,
        modifiers: gpui::Modifiers,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(command) = find_command(
            key,
            modifiers.shift,
            modifiers.control || modifiers.alt || modifiers.platform,
        ) else {
            return;
        };
        if !self.focus_handle(cx).is_focused(window) {
            return;
        }
        let composing = self.input.update(cx, |input, cx| {
            input.marked_text_range(window, cx).is_some()
        });
        if composing {
            if key == "enter" {
                cx.stop_propagation();
            }
            return;
        }
        cx.emit(command);
        cx.stop_propagation();
    }
}

impl Focusable for FindBar {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.input.read(cx).focus_handle(cx)
    }
}

impl Render for FindBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let entity = cx.entity().clone();
        div()
            .id(self.id.clone())
            .key_context("FindBar")
            .on_action(cx.listener(|bar, _: &CloseFind, window, cx| {
                bar.handle_command("escape", window.modifiers(), window, cx);
            }))
            .capture_key_down(cx.listener(Self::handle_key))
            // Bound actions run before raw key capture in GPUI.
            .capture_action(cx.listener(|bar, _: &crate::input::Enter, window, cx| {
                bar.handle_command("enter", window.modifiers(), window, cx);
            }))
            .flex()
            .items_center()
            .gap_2()
            .w_full()
            .p_2()
            .border_1()
            .border_color(theme.neutral.border)
            .bg(theme.neutral.card)
            .child(self.input.clone())
            .child(
                div()
                    .flex_1()
                    .text_sm()
                    .text_color(theme.neutral.text_2)
                    .child(self.match_status.clone()),
            )
            .child(Button::new("Prev").small().text().on_click({
                let entity = entity.clone();
                move |_, _, cx| {
                    let _ = entity.update(cx, |bar, cx| bar.previous(cx));
                }
            }))
            .child(Button::new("Next").small().text().on_click({
                let entity = entity.clone();
                move |_, _, cx| {
                    let _ = entity.update(cx, |bar, cx| bar.next(cx));
                }
            }))
            .child(
                Button::new("Close")
                    .small()
                    .text()
                    .on_click(move |_, _, cx| {
                        let _ = entity.update(cx, |bar, cx| bar.close(cx));
                    }),
            )
    }
}

fn update_query(previous: &mut SharedString, query: &SharedString) -> bool {
    if previous == query {
        return false;
    }
    *previous = query.clone();
    true
}

fn find_command(key: &str, shift: bool, modified: bool) -> Option<FindBarEvent> {
    if modified {
        return None;
    }
    match key {
        "enter" if shift => Some(FindBarEvent::Previous),
        "enter" => Some(FindBarEvent::Next),
        "escape" => Some(FindBarEvent::Closed),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{FindBarEvent, find_command, update_query};
    #[test]
    fn query_notifications_ignore_cursor_and_unchanged_values() {
        let mut query = "".into();
        assert!(!update_query(&mut query, &"".into()));
        assert!(update_query(&mut query, &"needle".into()));
        assert!(!update_query(&mut query, &"needle".into()));
        assert!(update_query(&mut query, &"".into()));
    }
    #[test]
    fn find_navigation_keeps_modified_shortcuts_native() {
        assert_eq!(
            find_command("enter", false, false),
            Some(FindBarEvent::Next)
        );
        assert_eq!(
            find_command("enter", true, false),
            Some(FindBarEvent::Previous)
        );
        assert_eq!(
            find_command("escape", false, false),
            Some(FindBarEvent::Closed)
        );
        assert_eq!(find_command("enter", false, true), None);
        assert_eq!(find_command("a", false, false), None);
    }
}
