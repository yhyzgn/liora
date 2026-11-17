use super::*;

#[derive(Clone, Copy)]
enum Control {
    Previous,
    Next,
    Replace,
    ReplaceAll,
    CloseFind,
    Slash(usize),
}
const BLOCKS: &[(&str, &str)] = &[
    ("Paragraph", "paragraph"),
    ("Heading 1", "heading1"),
    ("Heading 2", "heading2"),
    ("Heading 3", "heading3"),
    ("Quote", "quote"),
    ("Bullet list", "bullet"),
    ("Numbered list", "ordered"),
    ("Task list", "task"),
    ("Code block", "code"),
    ("Divider", "rule"),
];
impl MarkdownEditor {
    fn button(
        &self,
        label: impl Into<SharedString>,
        action: Control,
        cx: &Context<Self>,
    ) -> AnyElement {
        let label = label.into();
        let weak = cx.weak_entity();
        let keyboard = weak.clone();
        let disabled = self.is_composing(cx);
        div()
            .id(SharedString::from(format!("{}-{}", self.id, label)))
            .focusable()
            .tab_index(0)
            .tab_stop(!disabled)
            .aria_label(label.clone())
            .on_key_down(move |event, window, cx| {
                if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                    let _ = keyboard.update(cx, |editor, cx| editor.control(action, window, cx));
                    cx.stop_propagation();
                }
            })
            .child(
                crate::Button::new(label)
                    .small()
                    .text()
                    .disabled(disabled)
                    .on_click(move |_, window, cx| {
                        let _ = weak.update(cx, |editor, cx| editor.control(action, window, cx));
                    }),
            )
            .into_any_element()
    }
    fn control(&mut self, control: Control, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_composing(cx) {
            return;
        }
        let mut focus_editor = true;
        match control {
            Control::Previous => {
                self.find_next(true, cx);
                focus_editor = false;
            }
            Control::Next => {
                self.find_next(false, cx);
                focus_editor = false;
            }
            Control::Replace => {
                let needle = self.find_input.read(cx).value();
                if !needle.is_empty() && self.engine.selected_text() == needle.as_ref() {
                    let replacement = self.replacement_input.read(cx).value();
                    self.replace(&replacement, cx);
                }
                self.find_next(false, cx);
                focus_editor = false;
            }
            Control::ReplaceAll => {
                let needle = self.find_input.read(cx).value();
                let replacement = self.replacement_input.read(cx).value();
                if !needle.is_empty() {
                    match self.engine.replace_all(&needle, &replacement) {
                        Ok(true) => self.publish(cx),
                        Ok(false) => {
                            self.error = Some("No matches".into());
                            cx.notify();
                        }
                        Err(error) => {
                            self.error = Some(error.into());
                            cx.notify();
                        }
                    }
                }
                focus_editor = false;
            }
            Control::CloseFind => {
                self.find_open = false;
                cx.notify();
            }
            Control::Slash(index) => self.apply_slash(index, cx),
        }
        if focus_editor {
            window.focus(&self.focus_handle(cx), cx);
        }
    }
    fn active_top_block(&self) -> Option<u64> {
        let head = self.engine.selection().head;
        self.engine
            .document()
            .blocks
            .iter()
            .find(|block| block.source_range.start <= head && head <= block.source_range.end)
            .map(|block| block.id)
    }
    pub(super) fn move_block(&mut self, direction: isize, cx: &mut Context<Self>) {
        if let Some(id) = self.active_top_block() {
            self.execute(Command::MoveBlock { id, direction }, cx);
        }
    }
    pub(super) fn find_next(&mut self, backwards: bool, cx: &mut Context<Self>) {
        if self.is_composing(cx) {
            return;
        }
        let needle = self.find_input.read(cx).value();
        if needle.is_empty() {
            return;
        }
        if self.engine.find(&needle, backwards) {
            self.error = None;
            self.reveal();
        } else {
            self.error = Some("No matches".into());
        }
        cx.notify();
    }
    pub(super) fn find_controls(&self, cx: &Context<Self>) -> AnyElement {
        let weak = cx.weak_entity();
        div()
            .id(SharedString::from(format!("{}-find", self.id)))
            .key_context("FindBar")
            .on_action(
                cx.listener(|editor, _: &crate::find_bar::CloseFind, window, cx| {
                    editor.control(Control::CloseFind, window, cx);
                }),
            )
            .capture_action(cx.listener(|editor, _: &crate::input::Enter, window, cx| {
                let modifiers = window.modifiers();
                if (editor.find_input.focus_handle(cx).is_focused(window)
                    || editor.replacement_input.focus_handle(cx).is_focused(window))
                    && !modifiers.control
                    && !modifiers.alt
                    && !modifiers.platform
                {
                    editor.find_next(modifiers.shift, cx);
                    cx.stop_propagation();
                }
            }))
            .capture_key_down(move |event: &gpui::KeyDownEvent, window, cx| {
                let _ = weak.update(cx, |editor, cx| {
                    if editor.is_composing(cx) {
                        return;
                    }
                    let keys = &event.keystroke;
                    if keys.key == "escape" {
                        editor.control(Control::CloseFind, window, cx);
                        cx.stop_propagation();
                    } else if keys.key == "enter"
                        && !keys.modifiers.control
                        && !keys.modifiers.alt
                        && !keys.modifiers.platform
                        && (editor.find_input.focus_handle(cx).is_focused(window)
                            || editor.replacement_input.focus_handle(cx).is_focused(window))
                    {
                        editor.find_next(keys.modifiers.shift, cx);
                        cx.stop_propagation();
                    }
                });
            })
            .flex()
            .flex_row()
            .flex_wrap()
            .gap_1()
            .child(self.find_input.clone())
            .child(self.replacement_input.clone())
            .children(
                [
                    ("Previous", Control::Previous),
                    ("Next", Control::Next),
                    ("Replace", Control::Replace),
                    ("Replace all", Control::ReplaceAll),
                    ("Close find", Control::CloseFind),
                ]
                .into_iter()
                .map(|(label, action)| self.button(label, action, cx)),
            )
            .into_any_element()
    }
    pub(super) fn slash_range(&self) -> Option<Range<usize>> {
        if self.slash_dismissed || self.composition || !self.engine.selection().is_empty() {
            return None;
        }
        let end = self.engine.selection().head;
        let source = self.engine.source();
        let start = source[..end].rfind('\n').map_or(0, |index| index + 1);
        let query = source.get(start..end)?;
        (query.starts_with('/') && !query.contains(char::is_whitespace)).then_some(start..end)
    }
    pub(super) fn slash_choices(&self) -> Vec<(&'static str, &'static str)> {
        let Some(range) = self.slash_range() else {
            return Vec::new();
        };
        let query = self.engine.source()[range.start + 1..range.end].to_ascii_lowercase();
        BLOCKS
            .iter()
            .copied()
            .filter(|(label, command)| {
                label.to_ascii_lowercase().contains(&query) || command.starts_with(&query)
            })
            .collect()
    }
    pub(super) fn apply_slash(&mut self, index: usize, cx: &mut Context<Self>) {
        if self.is_composing(cx) {
            return;
        }
        let Some(range) = self.slash_range() else {
            return;
        };
        let Some((_, command)) = self.slash_choices().get(index).copied() else {
            return;
        };
        self.engine.begin_undo_group();
        self.engine.set_selection(Selection {
            anchor: range.start,
            head: range.end,
        });
        match self.engine.replace("") {
            Ok(_) => {
                self.execute(Command::Block(command), cx);
            }
            Err(error) => {
                self.error = Some(error.into());
                cx.notify();
            }
        }
        self.engine.end_undo_group();
        self.slash_dismissed = true;
    }
    pub(super) fn slash_controls(&self, cx: &Context<Self>) -> Option<AnyElement> {
        let choices = self.slash_choices();
        if choices.is_empty() {
            return None;
        }
        Some(
            div()
                .id(SharedString::from(format!("{}-slash", self.id)))
                .flex()
                .flex_row()
                .flex_wrap()
                .gap_1()
                .aria_label("Slash block commands; use arrows and Enter")
                .children(choices.iter().enumerate().map(|(index, (label, _))| {
                    self.button(
                        SharedString::from(format!(
                            "{}{}",
                            if index == self.slash_index {
                                "› "
                            } else {
                                ""
                            },
                            label
                        )),
                        Control::Slash(index),
                        cx,
                    )
                }))
                .into_any_element(),
        )
    }
}
