use super::*;

impl CodeEditor {
    /// Captures the document revision and selection for asynchronous host results.
    pub fn request_token(&self) -> CodeEditorRequestToken {
        CodeEditorRequestToken {
            revision: self.text_revision,
            cursor: self.selection.cursor(),
            selection: self.selection.range.clone(),
        }
    }

    /// Whether a host result still targets this revision and caret/selection.
    pub fn is_request_current(&self, token: &CodeEditorRequestToken) -> bool {
        self.marked_range.is_none() && *token == self.request_token()
    }

    /// Installs legacy provider candidates only if revision and caret still match.
    pub fn set_completions_for_request(
        &mut self,
        token: &CodeEditorRequestToken,
        items: impl IntoIterator<Item = CodeCompletionItem>,
        cx: &mut Context<Self>,
    ) -> bool {
        if !self.is_request_current(token) {
            return false;
        }
        self.completion_items = items.into_iter().collect();
        self.completion_edits.clear();
        self.completion_selection = (!self.completion_items.is_empty()).then_some(0);
        self.completion_request = Some(token.clone());
        cx.notify();
        true
    }

    /// Installs explicit text edits atomically; invalid UTF-8 boundaries reject the batch.
    pub fn set_completion_edits_for_request(
        &mut self,
        token: &CodeEditorRequestToken,
        edits: impl IntoIterator<Item = CodeCompletionEdit>,
        cx: &mut Context<Self>,
    ) -> bool {
        if !self.is_request_current(token) {
            return false;
        }
        let edits: Vec<_> = edits.into_iter().collect();
        if edits.iter().any(|edit| {
            edit.range.start > edit.range.end
                || edit.range.end > self.buffer.len()
                || !self.buffer.as_str().is_char_boundary(edit.range.start)
                || !self.buffer.as_str().is_char_boundary(edit.range.end)
        }) {
            return false;
        }
        self.completion_items = edits.iter().map(|edit| edit.item.clone()).collect();
        self.completion_edits = edits;
        self.completion_selection = (!self.completion_items.is_empty()).then_some(0);
        self.completion_request = Some(token.clone());
        cx.notify();
        true
    }

    /// Accepts a candidate as one undoable edit, never during IME composition.
    pub fn accept_completion(&mut self, index: usize, cx: &mut Context<Self>) -> bool {
        if self.options.read_only
            || self.marked_range.is_some()
            || !self
                .completion_request
                .as_ref()
                .is_some_and(|token| self.is_request_current(token))
        {
            return false;
        }
        if !self.completion_edits.is_empty() && index >= self.completion_edits.len() {
            return false;
        }
        let Some(item) = self.completion_items.get(index).cloned() else {
            return false;
        };
        let text_end = self.buffer.len();
        let (range, inserted) = self
            .completion_edits
            .get(index)
            .map(|edit| (edit.range.clone(), edit.new_text.clone()))
            .unwrap_or_else(|| {
                let range = if !self.selection.is_empty() {
                    self.selection.range.clone()
                } else {
                    let cursor = self.selection.cursor();
                    let mut start = cursor;
                    while start > 0 {
                        let previous = self.buffer.prev_char(start);
                        if !self.buffer.as_str()[previous..start]
                            .chars()
                            .all(is_code_word_char)
                        {
                            break;
                        }
                        start = previous;
                    }
                    let mut end = cursor;
                    while end < text_end {
                        let next = self.buffer.next_char(end);
                        if !self.buffer.as_str()[end..next]
                            .chars()
                            .all(is_code_word_char)
                        {
                            break;
                        }
                        end = next;
                    }
                    start..end
                };
                (range, item.label.clone())
            });
        let text = self.buffer.as_str();
        if range.start > range.end
            || range.end > text.len()
            || !text.is_char_boundary(range.start)
            || !text.is_char_boundary(range.end)
        {
            return false;
        }
        let mut transaction = CodeTransaction::new(self.selection.clone());
        self.record_edit(&mut transaction, range.clone(), inserted.as_ref());
        self.selection.set_cursor(range.start + inserted.len());
        self.dismiss_completions(cx);
        self.commit_transaction(transaction, cx);
        true
    }

    /// Hides suggestions without changing the document.
    pub fn dismiss_completions(&mut self, cx: &mut Context<Self>) {
        self.completion_items.clear();
        self.completion_edits.clear();
        self.completion_selection = None;
        self.completion_request = None;
        cx.notify();
    }

    pub(super) fn completion_active(&self) -> bool {
        self.options.completions_panel
            && !self.options.read_only
            && self.marked_range.is_none()
            && !self.completion_items.is_empty()
            && self
                .completion_request
                .as_ref()
                .is_some_and(|token| self.is_request_current(token))
    }

    pub(super) fn cycle_completion(&mut self, delta: isize, cx: &mut Context<Self>) -> bool {
        if !self.completion_active() {
            return false;
        }
        let count = self
            .completion_items
            .len()
            .min(self.options.completion_limit.max(1));
        let selected = self.completion_selection.unwrap_or(0) as isize;
        self.completion_selection = Some((selected + delta).rem_euclid(count as isize) as usize);
        cx.notify();
        true
    }

    /// Updates a literal, case-sensitive query; an empty query clears matches.
    pub fn set_search_query(&mut self, query: impl Into<SharedString>, cx: &mut Context<Self>) {
        let query = query.into();
        self.search_query = (!query.is_empty()).then_some(query);
        self.search_matches_cache = None;
        self.active_search_match = None;
        cx.notify();
    }

    /// Returns UTF-8 byte ranges of non-overlapping literal matches.
    pub fn search_matches(&mut self) -> Arc<Vec<Range<usize>>> {
        let Some(query) = self.search_query.clone() else {
            return Arc::new(Vec::new());
        };
        if let Some((revision, cached_query, matches)) = &self.search_matches_cache {
            if *revision == self.text_revision && *cached_query == query {
                return matches.clone();
            }
        }
        let matches: Arc<Vec<Range<usize>>> = Arc::new(
            self.buffer
                .as_str()
                .match_indices(query.as_ref())
                .map(|(start, matched)| start..start + matched.len())
                .collect(),
        );
        self.search_matches_cache = Some((self.text_revision, query, matches.clone()));
        matches
    }

    /// Selects and reveals the next match, wrapping at document end.
    pub fn find_next(&mut self, cx: &mut Context<Self>) -> Option<Range<usize>> {
        self.navigate_search(false, cx)
    }

    /// Selects and reveals the previous match, wrapping at document start.
    pub fn find_previous(&mut self, cx: &mut Context<Self>) -> Option<Range<usize>> {
        self.navigate_search(true, cx)
    }

    fn navigate_search(&mut self, previous: bool, cx: &mut Context<Self>) -> Option<Range<usize>> {
        if self.marked_range.is_some() {
            return None;
        }
        let matches = self.search_matches();
        if matches.is_empty() {
            return None;
        }
        let index = if previous {
            matches
                .iter()
                .rposition(|range| range.end <= self.selection.range.start)
                .unwrap_or(matches.len() - 1)
        } else {
            matches
                .iter()
                .position(|range| range.start >= self.selection.range.end)
                .unwrap_or(0)
        };
        let range = matches[index].clone();
        self.selection.range = range.clone();
        self.selection.reversed = false;
        self.selection.preferred_column = None;
        self.active_search_match = Some(index);
        // A located match must not remain hidden inside a collapsed fold.
        let start_row = self.buffer.offset_to_point(range.start).row;
        let end_row = self.buffer.offset_to_point(range.end).row;
        for fold in &mut self.folds {
            if fold.start_line < end_row && fold.end_line >= start_row {
                fold.collapsed = false;
            }
        }
        self.sync_list_state();
        self.reveal_cursor();
        self.reset_blink(cx);
        Some(range)
    }

    /// Replaces the selected exact match, then locates the next one.
    pub fn replace_current_match(&mut self, replacement: &str, cx: &mut Context<Self>) -> bool {
        if self.options.read_only || self.marked_range.is_some() {
            return false;
        }
        if !self
            .search_matches()
            .iter()
            .any(|range| *range == self.selection.range)
        {
            return false;
        }
        self.replace_selection(replacement, cx);
        self.find_next(cx);
        true
    }

    /// Replaces all current matches in one undo transaction, without rescanning inserted text.
    pub fn replace_all_matches(&mut self, replacement: &str, cx: &mut Context<Self>) -> usize {
        if self.options.read_only || self.marked_range.is_some() {
            return 0;
        }
        let matches = self.search_matches();
        if matches.is_empty() {
            return 0;
        }
        let mut transaction = CodeTransaction::new(self.selection.clone());
        for range in matches.iter().rev() {
            self.record_edit(&mut transaction, range.clone(), replacement);
        }
        self.selection
            .set_cursor(matches[0].start + replacement.len());
        self.commit_transaction(transaction, cx);
        matches.len()
    }

    pub(super) fn record_edit(
        &mut self,
        transaction: &mut CodeTransaction,
        range: Range<usize>,
        inserted: &str,
    ) {
        let edit = CodeTextEdit::new(self.buffer.as_str(), range, inserted);
        if edit.removed.as_ref() == edit.inserted.as_ref() {
            return;
        }
        transaction.edits.push(edit);
    }

    pub(super) fn restore_transaction(
        &mut self,
        transaction: &CodeTransaction,
        undo: bool,
        cx: &mut Context<Self>,
    ) {
        transaction.apply(&mut self.buffer, undo);
        self.selection = if undo {
            transaction.before_selection.clone()
        } else {
            transaction.after_selection.clone()
        };
        self.marked_range = None;
        self.bump_text_revision();
        self.invalidate_row_layouts();
        self.handle_buffer_change(cx);
    }
}

impl CodeTransaction {
    pub(super) fn apply(&self, buffer: &mut CodeBuffer, undo: bool) {
        if let [edit] = self.edits.as_slice() {
            if undo {
                buffer.replace_range(
                    edit.range.start..edit.range.start + edit.inserted.len(),
                    &edit.removed,
                );
            } else {
                buffer.replace_range(edit.range.clone(), &edit.inserted);
            }
            return;
        }
        if self.edits.is_empty() {
            return;
        }
        // Delta coordinates are sequential; publish one line index after the batch.
        if undo {
            for edit in self.edits.iter().rev() {
                buffer.text.replace_range(
                    edit.range.start..edit.range.start + edit.inserted.len(),
                    &edit.removed,
                );
            }
        } else {
            for edit in &self.edits {
                buffer
                    .text
                    .replace_range(edit.range.clone(), &edit.inserted);
            }
        }
        buffer.rebuild_lines();
    }
}
