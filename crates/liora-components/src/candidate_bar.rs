//! Native GPUI candidate bar inspired by keyboard-first input method panels.
//!
//! Candidate data is supplied by the host. This component does not contain an
//! input method engine, dictionary, language model, or platform IME integration.
//!
//! Explicitly focus this surface for keyboard selection. Arrows move within the
//! current page, PageUp/PageDown change pages, 1–9 commit visible slots, Space
//! commits the selected candidate, Enter commits the literal, and Escape cancels.
//! Modified keys and keys focused in descendant inputs are left untouched.

use crate::Button;
use gpui::{
    App, Context, EventEmitter, FocusHandle, Focusable, IntoElement, KeyDownEvent, Render,
    SharedString, Window, div, prelude::*,
};
use liora_core::Config;

gpui::actions!(candidate_bar, [CancelCandidates]);

/// One host-provided candidate shown in [`CandidateBar`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CandidateItem {
    /// Text committed when this candidate is selected.
    pub value: SharedString,
    /// Optional annotation, such as pronunciation or source.
    pub annotation: Option<SharedString>,
}

impl CandidateItem {
    /// Creates a candidate without an annotation.
    pub fn new(value: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            annotation: None,
        }
    }

    /// Adds an annotation displayed beside the candidate.
    pub fn annotated(mut self, annotation: impl Into<SharedString>) -> Self {
        self.annotation = Some(annotation.into());
        self
    }
}

/// Events emitted by [`CandidateBar`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CandidateBarEvent {
    /// The host should commit the selected candidate.
    Selected { index: usize, value: SharedString },
    /// The host should commit the current literal composition.
    CommittedLiteral { composition: SharedString },
    /// The host should clear the current composition.
    Cancelled,
    /// The host should load or expose another candidate page.
    PageChanged { page: usize },
}

/// Keyboard-first candidate panel for IME, command completion, and mention flows.
pub struct CandidateBar {
    id: SharedString,
    focus_handle: FocusHandle,
    composition: SharedString,
    items: Vec<CandidateItem>,
    page: usize,
    page_size: usize,
    selected: usize,
}

impl EventEmitter<CandidateBarEvent> for CandidateBar {}

impl CandidateBar {
    /// Registers the scoped Escape binding; called by `init_liora`.
    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([gpui::KeyBinding::new(
            "escape",
            CancelCandidates,
            Some("CandidateBar"),
        )]);
    }

    /// Creates an empty candidate bar with eight visible candidates per page.
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            id: liora_core::unique_id("candidate-bar"),
            focus_handle: cx.focus_handle(),
            composition: SharedString::default(),
            items: Vec::new(),
            page: 0,
            page_size: 8,
            selected: 0,
        }
    }

    /// Sets the current composition shown above the candidates.
    pub fn set_composition(
        &mut self,
        composition: impl Into<SharedString>,
        cx: &mut Context<Self>,
    ) {
        self.composition = composition.into();
        cx.notify();
    }

    /// Replaces the host-provided candidates and resets selection to the first page.
    pub fn set_items(&mut self, items: Vec<CandidateItem>, cx: &mut Context<Self>) {
        self.items = items;
        self.page = 0;
        self.selected = 0;
        cx.notify();
    }

    /// Sets the visible page size, clamped to the nine numbered keyboard slots.
    pub fn page_size(&mut self, page_size: usize, cx: &mut Context<Self>) {
        self.page_size = bounded_page_size(page_size);
        self.page = self.page.min(self.page_count().saturating_sub(1));
        self.selected = self.selected.min(self.visible_count().saturating_sub(1));
        cx.notify();
    }

    /// Focuses the native candidate keyboard surface on the next window turn.
    pub fn focus(&self, window: &mut Window, cx: &mut Context<Self>) {
        let handle = self.focus_handle.clone();
        window.defer(cx, move |window, cx| window.focus(&handle, cx));
    }

    /// Returns the selected visible slot, or `None` for an empty page.
    pub fn selected(&self) -> Option<usize> {
        (self.selected < self.visible_count()).then_some(self.selected)
    }

    /// Returns the current candidate page.
    pub fn page(&self) -> usize {
        self.page
    }

    /// Returns the total number of candidate pages.
    pub fn page_count(&self) -> usize {
        self.items.len().div_ceil(self.page_size)
    }

    /// Selects a visible candidate by its zero-based position.
    pub fn select(&mut self, position: usize, cx: &mut Context<Self>) {
        if position < self.visible_count() {
            self.selected = position;
            cx.notify();
        }
    }

    /// Moves to the next candidate page and emits a page event.
    pub fn next_page(&mut self, cx: &mut Context<Self>) {
        let last = self.page_count().saturating_sub(1);
        if self.page < last {
            self.page += 1;
            self.selected = 0;
            cx.emit(CandidateBarEvent::PageChanged { page: self.page });
            cx.notify();
        }
    }

    /// Moves to the previous candidate page and emits a page event.
    pub fn previous_page(&mut self, cx: &mut Context<Self>) {
        if self.page > 0 {
            self.page -= 1;
            self.selected = 0;
            cx.emit(CandidateBarEvent::PageChanged { page: self.page });
            cx.notify();
        }
    }

    /// Commits a candidate by its visible position.
    pub fn commit(&mut self, position: usize, cx: &mut Context<Self>) {
        if let Some(index) = visible_index(self.items.len(), self.page, self.page_size, position) {
            if let Some(item) = self.items.get(index) {
                cx.emit(CandidateBarEvent::Selected {
                    index,
                    value: item.value.clone(),
                });
            }
        }
    }

    /// Commits the literal composition.
    pub fn commit_literal(&self, cx: &mut Context<Self>) {
        cx.emit(CandidateBarEvent::CommittedLiteral {
            composition: self.composition.clone(),
        });
    }

    /// Cancels the current composition.
    pub fn cancel(&self, cx: &mut Context<Self>) {
        cx.emit(CandidateBarEvent::Cancelled);
    }

    fn visible_count(&self) -> usize {
        let Some(start) = self.page.checked_mul(self.page_size) else {
            return 0;
        };
        self.items.len().saturating_sub(start).min(self.page_size)
    }

    fn handle_key(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        // Handle only our own focus, never a descendant input's IME or shortcuts.
        if !self.focus_handle.is_focused(window) {
            return;
        }
        let Some(command) = candidate_command(
            event.keystroke.key.as_str(),
            event.keystroke.modifiers.modified(),
        ) else {
            return;
        };
        match command {
            CandidateCommand::Previous => self.select(self.selected.saturating_sub(1), cx),
            CandidateCommand::Next => self.select(self.selected.saturating_add(1), cx),
            CandidateCommand::PreviousPage => self.previous_page(cx),
            CandidateCommand::NextPage => self.next_page(cx),
            CandidateCommand::Commit => self.commit(self.selected, cx),
            CandidateCommand::Slot(position) => self.commit(position, cx),
            CandidateCommand::Literal => self.commit_literal(cx),
            CandidateCommand::Cancel => self.cancel(cx),
        }
        cx.stop_propagation();
    }
}

impl Focusable for CandidateBar {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for CandidateBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let entity = cx.entity().clone();
        let start = self
            .page
            .checked_mul(self.page_size)
            .unwrap_or(self.items.len())
            .min(self.items.len());
        let end = start.saturating_add(self.page_size).min(self.items.len());
        let mut candidates = Vec::with_capacity(end.saturating_sub(start));
        for (position, item) in self.items[start..end].iter().enumerate() {
            let selected = position == self.selected;
            let entity = entity.clone();
            let label = match &item.annotation {
                Some(annotation) => format!("{}  {}", item.value, annotation),
                None => item.value.to_string(),
            };
            candidates.push(
                Button::new(format!("{}  {}", position + 1, label))
                    .small()
                    .when(selected, |button| button.primary())
                    .on_click(move |_, _, cx| {
                        let _ = entity.update(cx, |bar, cx| bar.commit(position, cx));
                    })
                    .into_any_element(),
            );
        }

        div()
            .id(self.id.clone())
            .key_context("CandidateBar")
            .on_action(cx.listener(|bar, _: &CancelCandidates, window, cx| {
                if bar.focus_handle.is_focused(window) && !window.modifiers().modified() {
                    bar.cancel(cx);
                    cx.stop_propagation();
                }
            }))
            .track_focus(&self.focus_handle)
            .tab_index(0)
            .on_mouse_down(
                gpui::MouseButton::Left,
                cx.listener(|bar, _, window, cx| window.focus(&bar.focus_handle, cx)),
            )
            .capture_key_down(cx.listener(Self::handle_key))
            .flex()
            .flex_col()
            .gap_2()
            .w_full()
            .p_2()
            .border_1()
            .border_color(theme.neutral.border)
            .bg(theme.neutral.card)
            .child(
                div()
                    .text_sm()
                    .text_color(theme.neutral.text_2)
                    .child(self.composition.clone()),
            )
            .child(div().flex().flex_wrap().gap_1().children(candidates))
            .child(
                div()
                    .flex()
                    .gap_1()
                    .child(Button::new("Previous page").small().text().on_click({
                        let entity = entity.clone();
                        move |_, _, cx| {
                            entity.update(cx, |bar, cx| bar.previous_page(cx));
                        }
                    }))
                    .child(Button::new("Next page").small().text().on_click({
                        let entity = entity.clone();
                        move |_, _, cx| {
                            entity.update(cx, |bar, cx| bar.next_page(cx));
                        }
                    }))
                    .child(Button::new("Commit selected").small().text().on_click({
                        let entity = entity.clone();
                        move |_, _, cx| {
                            entity.update(cx, |bar, cx| bar.commit(bar.selected, cx));
                        }
                    }))
                    .child(Button::new("Commit literal").small().text().on_click({
                        let entity = entity.clone();
                        move |_, _, cx| {
                            entity.update(cx, |bar, cx| bar.commit_literal(cx));
                        }
                    }))
                    .child(
                        Button::new("Cancel")
                            .small()
                            .text()
                            .on_click(move |_, _, cx| {
                                entity.update(cx, |bar, cx| bar.cancel(cx));
                            }),
                    ),
            )
            .child(
                div()
                    .flex()
                    .justify_between()
                    .text_xs()
                    .text_color(theme.neutral.text_3)
                    .child(format!(
                        "Page {} / {}",
                        self.page + 1,
                        self.page_count().max(1)
                    ))
                    .child("1-9 select · Space candidate · Enter literal · Esc cancel"),
            )
    }
}

fn bounded_page_size(size: usize) -> usize {
    size.clamp(1, 9)
}

fn visible_index(len: usize, page: usize, page_size: usize, position: usize) -> Option<usize> {
    if !(1..=9).contains(&page_size) || position >= page_size {
        return None;
    }
    page.checked_mul(page_size)?
        .checked_add(position)
        .filter(|index| *index < len)
}

#[derive(Debug, PartialEq, Eq)]
enum CandidateCommand {
    Previous,
    Next,
    PreviousPage,
    NextPage,
    Commit,
    Literal,
    Cancel,
    Slot(usize),
}

fn candidate_command(key: &str, modified: bool) -> Option<CandidateCommand> {
    if modified {
        return None;
    }
    Some(match key {
        "left" | "up" => CandidateCommand::Previous,
        "right" | "down" => CandidateCommand::Next,
        "pageup" => CandidateCommand::PreviousPage,
        "pagedown" => CandidateCommand::NextPage,
        "space" | " " => CandidateCommand::Commit,
        "enter" => CandidateCommand::Literal,
        "escape" => CandidateCommand::Cancel,
        key if key.len() == 1 && (b'1'..=b'9').contains(&key.as_bytes()[0]) => {
            CandidateCommand::Slot(usize::from(key.as_bytes()[0] - b'1'))
        }
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::{CandidateCommand, bounded_page_size, candidate_command, visible_index};
    #[test]
    fn page_size_cannot_exceed_numbered_slots_or_be_empty() {
        assert_eq!(bounded_page_size(0), 1);
        assert_eq!(bounded_page_size(8), 8);
        assert_eq!(bounded_page_size(usize::MAX), 9);
    }
    #[test]
    fn commits_only_visible_slots_and_handles_overflow() {
        assert_eq!(visible_index(20, 1, 8, 0), Some(8));
        assert_eq!(visible_index(20, 0, 8, 8), None);
        assert_eq!(visible_index(10, 1, 8, 2), None);
        assert_eq!(visible_index(0, 0, 8, 0), None);
        assert_eq!(visible_index(20, usize::MAX, 9, usize::MAX), None);
        assert_eq!(visible_index(20, usize::MAX, 9, 0), None);
        assert_eq!(visible_index(20, 0, 0, 0), None);
    }
    #[test]
    fn keyboard_contract_preserves_editing_and_shortcuts() {
        assert_eq!(
            candidate_command("left", false),
            Some(CandidateCommand::Previous)
        );
        assert_eq!(
            candidate_command("pagedown", false),
            Some(CandidateCommand::NextPage)
        );
        assert_eq!(
            candidate_command("9", false),
            Some(CandidateCommand::Slot(8))
        );
        assert_eq!(
            candidate_command("space", false),
            Some(CandidateCommand::Commit)
        );
        assert_eq!(
            candidate_command("enter", false),
            Some(CandidateCommand::Literal)
        );
        assert_eq!(
            candidate_command("escape", false),
            Some(CandidateCommand::Cancel)
        );
        for key in ["backspace", "0", "a", "tab"] {
            assert_eq!(candidate_command(key, false), None);
        }
        for key in ["1", "left", "enter", "escape", "space"] {
            assert_eq!(candidate_command(key, true), None);
        }
    }
}
