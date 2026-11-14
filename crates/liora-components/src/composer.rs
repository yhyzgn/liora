//! Native draft composer. The host owns uploads, command execution and request lifecycles.
use crate::{Button, Input};
use gpui::{
    App, ClipboardEntry, ClipboardItem, Context, Entity, EntityInputHandler, EventEmitter,
    FocusHandle, Focusable, KeyDownEvent, Render, SharedString, Subscription, Window, div,
    prelude::*, px,
};
use liora_core::{Config, unique_id};
use std::ops::Range;

/// Host-owned attachment identity. No file is opened or uploaded by the component.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComposerAttachment {
    /// Host-defined identity used to deduplicate attachments added to a draft.
    pub id: SharedString,
    /// Display label shown in the draft's attachment list.
    pub label: SharedString,
}
impl ComposerAttachment {
    /// Creates attachment metadata without opening or uploading a resource.
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
        }
    }
}
/// Structured reference independent of the draft's editable text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComposerReference {
    /// Host-defined identity used to deduplicate references added to a draft.
    pub id: SharedString,
    /// Display label, also inserted after `@` when accepted as a candidate.
    pub label: SharedString,
}
impl ComposerReference {
    /// Creates reference metadata independent of the editable draft text.
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
        }
    }
}
/// Editable text and structured context that the host can save or restore.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ComposerDraft {
    /// Current editable message text.
    pub text: SharedString,
    /// Structured references associated with the message.
    pub references: Vec<ComposerReference>,
    /// Host-owned attachments associated with the message.
    pub attachments: Vec<ComposerAttachment>,
}
/// Candidate insertion is local editing, never command execution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ComposerCandidate {
    /// An `@` candidate that inserts its label and adds a structured reference.
    Reference(ComposerReference),
    /// A `/` candidate that inserts text without executing a command.
    Command {
        /// Host-defined command identity.
        id: SharedString,
        /// Display label used for case-insensitive candidate matching.
        label: SharedString,
        /// Text that replaces the command query when accepted.
        text: SharedString,
    },
}
impl ComposerCandidate {
    fn label(&self) -> &SharedString {
        match self {
            Self::Reference(r) => &r.label,
            Self::Command { label, .. } => label,
        }
    }
    fn trigger(&self) -> char {
        match self {
            Self::Reference(_) => '@',
            Self::Command { .. } => '/',
        }
    }
}
/// Draft updates and requests for host-owned send, stop and paste handling.
#[derive(Clone, Debug)]
pub enum ComposerEvent {
    /// The editable text or structured draft context changed.
    DraftChanged(ComposerDraft),
    /// A nonempty draft was submitted and cleared locally; the host performs sending.
    Send(ComposerDraft),
    /// Stop was requested while busy; the host cancels its work and updates busy state.
    Stop,
    /// Original native clipboard payload, including image bytes and external paths.
    /// Text entries are also inserted into the current selection exactly once.
    Paste(ClipboardItem),
}

/// Native message editor with structured context, candidates and host-owned execution.
pub struct Composer {
    id: SharedString,
    input: Entity<Input>,
    references: Vec<ComposerReference>,
    attachments: Vec<ComposerAttachment>,
    candidates: Vec<ComposerCandidate>,
    selected_candidate: usize,
    dismissed: Option<(SharedString, Range<usize>)>,
    busy: bool,
    last_text: SharedString,
    _input_subscription: Subscription,
}
impl EventEmitter<ComposerEvent> for Composer {}
impl Focusable for Composer {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.input.read(cx).focus_handle(cx)
    }
}
impl Composer {
    /// Creates an empty, idle composer backed by the native [`Input`] text engine.
    pub fn new(cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| {
            Input::new("", cx)
                .min_rows(3)
                .placeholder("Write a message; @ reference, / command")
        });
        let subscription = cx.observe(&input, |this, input, cx| {
            let text = input.read(cx).value();
            if text != this.last_text {
                this.last_text = text;
                this.selected_candidate = 0;
                this.dismissed = None;
                this.changed(cx);
            } else {
                cx.notify();
            }
        });
        Self {
            id: unique_id("composer"),
            input,
            references: Vec::new(),
            attachments: Vec::new(),
            candidates: Vec::new(),
            selected_candidate: 0,
            dismissed: None,
            busy: false,
            last_text: "".into(),
            _input_subscription: subscription,
        }
    }
    /// Allocates an empty composer entity in the application.
    pub fn entity(cx: &mut App) -> Entity<Self> {
        cx.new(Self::new)
    }
    /// Access native text selection, clipboard and IME APIs without a second text engine.
    pub fn input(&self) -> Entity<Input> {
        self.input.clone()
    }
    /// Returns a snapshot of editable text, references and attachments.
    pub fn draft(&self, cx: &App) -> ComposerDraft {
        ComposerDraft {
            text: self.input.read(cx).value(),
            references: self.references.clone(),
            attachments: self.attachments.clone(),
        }
    }
    /// Restores a host-owned draft; busy state is deliberately independent.
    /// Replaces native text and cancels any active IME composition, even for identical text.
    pub fn set_draft(&mut self, draft: ComposerDraft, cx: &mut Context<Self>) {
        self.input
            .update(cx, |input, cx| input.set_value(draft.text.clone(), cx));
        self.last_text = draft.text;
        self.references = draft.references;
        self.attachments = draft.attachments;
        self.dismissed = None;
        self.selected_candidate = 0;
        self.changed(cx);
    }
    /// Returns whether the host has marked a request as in progress.
    pub fn is_busy(&self) -> bool {
        self.busy
    }
    /// Sets request activity without clearing or disabling editing of the next draft.
    pub fn set_busy(&mut self, busy: bool, cx: &mut Context<Self>) {
        self.busy = busy;
        cx.notify();
    }
    /// Replaces available `@` and `/` candidates and resets the selected result.
    pub fn set_candidates(&mut self, candidates: Vec<ComposerCandidate>, cx: &mut Context<Self>) {
        self.candidates = candidates;
        self.selected_candidate = 0;
        cx.notify();
    }
    /// Adds a reference and emits a draft update unless its identity is already present.
    pub fn add_reference(&mut self, reference: ComposerReference, cx: &mut Context<Self>) {
        if !self.references.iter().any(|r| r.id == reference.id) {
            self.references.push(reference);
            self.changed(cx);
        }
    }
    /// Adds attachment metadata and emits a draft update unless its identity is present.
    pub fn add_attachment(&mut self, attachment: ComposerAttachment, cx: &mut Context<Self>) {
        if !self.attachments.iter().any(|a| a.id == attachment.id) {
            self.attachments.push(attachment);
            self.changed(cx);
        }
    }
    /// Removes references with this identity, emitting an update only if the draft changes.
    pub fn remove_reference(&mut self, id: &str, cx: &mut Context<Self>) {
        let len = self.references.len();
        self.references.retain(|r| r.id.as_ref() != id);
        if self.references.len() != len {
            self.changed(cx);
        }
    }
    /// Removes attachments with this identity, emitting an update only if the draft changes.
    pub fn remove_attachment(&mut self, id: &str, cx: &mut Context<Self>) {
        let len = self.attachments.len();
        self.attachments.retain(|a| a.id.as_ref() != id);
        if self.attachments.len() != len {
            self.changed(cx);
        }
    }
    fn changed(&mut self, cx: &mut Context<Self>) {
        cx.emit(ComposerEvent::DraftChanged(self.draft(cx)));
        cx.notify();
    }
    /// Sends a snapshot and clears only that draft. A busy composer preserves the next draft.
    /// Returns false for empty drafts, busy requests, or active marked composition.
    pub fn send(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        if self.busy || self.composing(window, cx) {
            return false;
        }
        let draft = self.draft(cx);
        if draft.text.trim().is_empty()
            && draft.references.is_empty()
            && draft.attachments.is_empty()
        {
            return false;
        }
        self.set_draft(ComposerDraft::default(), cx);
        cx.emit(ComposerEvent::Send(draft));
        true
    }
    /// Emits a host stop request only while busy; does not change busy state or the draft.
    pub fn stop(&mut self, cx: &mut Context<Self>) {
        if self.busy {
            cx.emit(ComposerEvent::Stop);
        }
    }
    fn composing(&self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        self.input.update(cx, |input, cx| {
            input.marked_text_range(window, cx).is_some()
        })
    }
    /// Text is inserted locally; image/file processing remains exclusively host-owned.
    pub fn paste(&mut self, clipboard: ClipboardItem, cx: &mut Context<Self>) {
        let text: String = clipboard
            .entries()
            .iter()
            .filter_map(|entry| match entry {
                ClipboardEntry::String(s) => Some(s.text.as_str()),
                _ => None,
            })
            .collect();
        if !text.is_empty() {
            self.input
                .update(cx, |input, cx| input.insert_text(&text, cx));
            self.last_text = self.input.read(cx).value();
            self.dismissed = None;
            self.selected_candidate = 0;
            self.changed(cx);
        }
        cx.emit(ComposerEvent::Paste(clipboard));
    }
    fn query(&self, cx: &App) -> Option<(char, Range<usize>, String)> {
        let input = self.input.read(cx);
        let text = input.value();
        let selection = input.selected_range();
        if self.dismissed.as_ref() == Some(&(text.clone(), selection.clone())) {
            return None;
        }
        candidate_query(&text, selection).map(|(trigger, range)| {
            let query = text[range.start + 1..range.end].to_lowercase();
            (trigger, range, query)
        })
    }
    /// Returns candidates matching the caret's active `@` or `/` query in supplied order.
    pub fn matching_candidates(&self, cx: &App) -> Vec<ComposerCandidate> {
        let Some((trigger, _, query)) = self.query(cx) else {
            return Vec::new();
        };
        self.candidates
            .iter()
            .filter(|c| c.trigger() == trigger && c.label().to_lowercase().contains(&query))
            .cloned()
            .collect()
    }
    /// Accepts a currently matching candidate at the caret, preserving the suffix and selection.
    pub fn accept_candidate(&mut self, index: usize, cx: &mut Context<Self>) -> bool {
        let Some(candidate) = self.matching_candidates(cx).get(index).cloned() else {
            return false;
        };
        let Some((_, range, _)) = self.query(cx) else {
            return false;
        };
        let insertion = match &candidate {
            ComposerCandidate::Reference(r) => format!("@{} ", r.label),
            ComposerCandidate::Command { text, .. } => text.to_string(),
        };
        self.input.update(cx, |input, cx| {
            input.set_selection(range, cx);
            input.insert_text(&insertion, cx);
        });
        if let ComposerCandidate::Reference(r) = candidate {
            if !self.references.iter().any(|existing| existing.id == r.id) {
                self.references.push(r);
            }
        }
        self.last_text = self.input.read(cx).value();
        self.selected_candidate = 0;
        self.dismissed = None;
        self.changed(cx);
        true
    }
    fn key_down(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        self.handle_key(&event.keystroke.key, event.keystroke.modifiers, window, cx);
    }
    fn handle_key(
        &mut self,
        key: &str,
        modifiers: gpui::Modifiers,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.focus_handle(cx).is_focused(window) {
            return;
        }
        if key == "enter" {
            if !self.composing(window, cx) {
                if modifiers.shift {
                    self.input
                        .update(cx, |input, cx| input.insert_text("\n", cx));
                } else if !modifiers.control && !modifiers.platform && !modifiers.alt {
                    let count = self.matching_candidates(cx).len();
                    if count == 0 {
                        self.send(window, cx);
                    } else {
                        self.accept_candidate(self.selected_candidate.min(count - 1), cx);
                    }
                } else {
                    return;
                }
            }
            cx.stop_propagation();
        } else if key == "v" && (modifiers.platform || modifiers.control) && !modifiers.alt {
            if let Some(item) = cx.read_from_clipboard() {
                self.paste(item, cx);
            }
            cx.stop_propagation();
        } else if !self.composing(window, cx) {
            let count = self.matching_candidates(cx).len();
            if count > 0 && (key == "up" || key == "down") {
                let current = self.selected_candidate.min(count - 1);
                self.selected_candidate = if key == "up" {
                    (current + count - 1) % count
                } else {
                    (current + 1) % count
                };
                cx.stop_propagation();
                cx.notify();
            } else if count > 0 && key == "escape" {
                self.dismissed = Some((
                    self.input.read(cx).value(),
                    self.input.read(cx).selected_range(),
                ));
                cx.stop_propagation();
                cx.notify();
            }
        }
    }
}

/// UTF-8 byte ranges; only the token immediately before a collapsed caret is replaced.
fn candidate_query(text: &str, selection: Range<usize>) -> Option<(char, Range<usize>)> {
    if !selection.is_empty() || !text.is_char_boundary(selection.end) {
        return None;
    }
    let prefix = text.get(..selection.end)?;
    let start = prefix
        .rfind(char::is_whitespace)
        .map_or(0, |i| i + prefix[i..].chars().next().unwrap().len_utf8());
    let trigger = prefix.get(start..)?.chars().next()?;
    matches!(trigger, '@' | '/').then_some((trigger, start..selection.end))
}

impl Render for Composer {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let mut root = div()
            .id(self.id.clone())
            .flex()
            .flex_col()
            .gap_2()
            .w_full()
            .p_2()
            .border_1()
            .border_color(theme.neutral.border)
            .rounded(px(theme.radius.md))
            .bg(theme.neutral.card)
            .text_color(theme.neutral.text_1)
            // Bound actions precede raw key listeners in the pinned GPUI dispatcher.
            .capture_key_down(cx.listener(Self::key_down))
            .capture_action(cx.listener(|this, _: &crate::input::Enter, window, cx| {
                this.handle_key("enter", gpui::Modifiers::default(), window, cx);
            }))
            .capture_action(cx.listener(|this, _: &crate::input::Paste, window, cx| {
                let modifiers = gpui::Modifiers {
                    control: true,
                    ..Default::default()
                };
                this.handle_key("v", modifiers, window, cx);
            }))
            .capture_action(cx.listener(|this, _: &crate::input::InputUp, window, cx| {
                this.handle_key("up", gpui::Modifiers::default(), window, cx);
            }))
            .capture_action(
                cx.listener(|this, _: &crate::input::InputDown, window, cx| {
                    this.handle_key("down", gpui::Modifiers::default(), window, cx);
                }),
            );
        let mut chips = div().flex().flex_wrap().gap_2();
        for reference in &self.references {
            let id = reference.id.clone();
            let keyboard_id = id.clone();
            chips = chips.child(
                div()
                    .id(SharedString::from(format!("{}-reference-{}", self.id, id)))
                    .focusable()
                    .tab_index(0)
                    .on_key_down(cx.listener(move |this, event: &KeyDownEvent, _, cx| {
                        if matches!(
                            event.keystroke.key.as_str(),
                            "enter" | "space" | "delete" | "backspace"
                        ) {
                            this.remove_reference(&keyboard_id, cx);
                            cx.stop_propagation();
                        }
                    }))
                    .child(Button::new(format!("@{} ×", reference.label)).on_click(
                        cx.listener(move |this, _, _, cx| this.remove_reference(&id, cx)),
                    )),
            );
        }
        for attachment in &self.attachments {
            let id = attachment.id.clone();
            let keyboard_id = id.clone();
            chips = chips.child(
                div()
                    .id(SharedString::from(format!("{}-attachment-{}", self.id, id)))
                    .focusable()
                    .tab_index(0)
                    .on_key_down(cx.listener(move |this, event: &KeyDownEvent, _, cx| {
                        if matches!(
                            event.keystroke.key.as_str(),
                            "enter" | "space" | "delete" | "backspace"
                        ) {
                            this.remove_attachment(&keyboard_id, cx);
                            cx.stop_propagation();
                        }
                    }))
                    .child(
                        Button::new(format!("Attachment: {} ×", attachment.label)).on_click(
                            cx.listener(move |this, _, _, cx| this.remove_attachment(&id, cx)),
                        ),
                    ),
            );
        }
        root = root.child(chips).child(self.input.clone());
        for (index, candidate) in self.matching_candidates(cx).into_iter().enumerate() {
            let label = format!(
                "{} {}",
                if index == self.selected_candidate {
                    "›"
                } else {
                    " "
                },
                candidate.label()
            );
            root = root.child(Button::new(label).on_click(cx.listener(
                move |this, _, window, cx| {
                    this.accept_candidate(index, cx);
                    window.focus(&this.focus_handle(cx), cx);
                },
            )));
        }
        root.child(
            div()
                .id(SharedString::from(format!("{}-submit", self.id)))
                .focusable()
                .tab_index(0)
                .flex()
                .justify_end()
                .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
                    if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                        if this.busy {
                            this.stop(cx);
                        } else {
                            this.send(window, cx);
                        }
                        cx.stop_propagation();
                    }
                }))
                .child(if self.busy {
                    Button::new("Stop").on_click(cx.listener(|this, _, _, cx| this.stop(cx)))
                } else {
                    Button::new("Send").on_click(cx.listener(|this, _, window, cx| {
                        this.send(window, cx);
                    }))
                }),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::candidate_query;
    #[test]
    fn candidate_range_uses_caret_not_end_of_draft() {
        assert_eq!(
            candidate_query("你好 @al suffix", 10..10),
            Some(('@', 7..10))
        );
        assert_eq!(candidate_query("/su tail", 3..3), Some(('/', 0..3)));
        assert_eq!(candidate_query("email@al", 8..8), None);
        assert_eq!(candidate_query("@alice", 0..6), None);
        assert_eq!(candidate_query("你好", 1..1), None);
        assert_eq!(candidate_query("@a", 99..99), None);
        assert_eq!(
            candidate_query("hello\u{3000}@x", 10..10),
            Some(('@', 8..10))
        );
    }
}
