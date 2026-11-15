//! Native, source-backed rich editing actions; source remains canonical.
use super::{Block, Kind, MarkdownEditor, find_block, map_display_offset};
use gpui::{
    AppContext, Context, Focusable, InteractiveElement, IntoElement, ParentElement, SharedString,
    StatefulInteractiveElement, Styled, Window,
};
use std::ops::Range;

#[derive(Clone, Copy)]
enum Action {
    Inline(&'static str),
    Block(&'static str),
    Split,
}

impl MarkdownEditor {
    pub(super) fn editing_controls(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> gpui::AnyElement {
        if let Some((&id, _)) = self
            .block_inputs
            .iter()
            .find(|(_, input)| input.focus_handle(cx).is_focused(window))
        {
            self.active_block = Some(id);
        }
        let id = self
            .active_block
            .filter(|id| self.block_inputs.contains_key(id) && !self.is_composing(cx));
        let mut toolbar = gpui::div().flex().flex_row().flex_wrap().gap_1();
        for (label, action) in [
            ("Bold", Action::Inline("**")),
            ("Italic", Action::Inline("*")),
            ("Strike", Action::Inline("~~")),
            ("Code", Action::Inline("`")),
            ("Highlight", Action::Inline("==")),
            ("Paragraph", Action::Block("")),
            ("H1", Action::Block("# ")),
            ("H2", Action::Block("## ")),
            ("Quote", Action::Block("> ")),
            ("List", Action::Block("- ")),
            ("Split", Action::Split),
        ] {
            let weak = cx.weak_entity();
            let keyboard = weak.clone();
            toolbar = toolbar.child(
                gpui::div()
                    .id(SharedString::from(format!("{}-format-{label}", self.id)))
                    .focusable()
                    .tab_index(0)
                    .tab_stop(id.is_some())
                    .aria_label(label)
                    .on_key_down(move |event, window, cx| {
                        if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                            if let Some(id) = id {
                                let _ = keyboard.update(cx, |editor, cx| {
                                    editor.apply_rich_action(id, action, window, cx)
                                });
                                cx.stop_propagation();
                            }
                        }
                    })
                    .child(
                        crate::Button::new(label)
                            .small()
                            .text()
                            .disabled(id.is_none())
                            .on_click(move |_, window, cx| {
                                if let Some(id) = id {
                                    let _ = weak.update(cx, |editor, cx| {
                                        editor.apply_rich_action(id, action, window, cx)
                                    });
                                }
                            }),
                    ),
            );
        }
        toolbar.into_any_element()
    }

    fn apply_rich_action(
        &mut self,
        id: u64,
        action: Action,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.is_composing(cx) {
            return;
        }
        let Some(input) = self.block_inputs.get(&id).cloned() else {
            return;
        };
        let selection = input.read(cx).selected_range();
        let result = cx.update_entity(&self.preview, |preview, _| {
            preview.document.flush();
            let block = find_block(&preview.document.blocks, id)?;
            edit(self.source.as_ref(), block, selection.clone(), action)
        });
        let Some((source, offset)) = result else {
            self.edit_error = Some("Select text within one Markdown style to format it; use Source mode for ambiguous syntax.".into());
            cx.notify();
            return;
        };
        self.set_source(SharedString::from(source), cx);
        if let Some(input) = self.rich_input_at(offset, cx) {
            let selection = if matches!(action, Action::Split) {
                0..0
            } else {
                selection
            };
            input.update(cx, |input, cx| input.set_selection(selection, cx));
            window.focus(&input.focus_handle(cx), cx);
        }
    }
}

fn edit(
    source: &str,
    block: &Block,
    selection: Range<usize>,
    action: Action,
) -> Option<(String, usize)> {
    let mut next = source.to_owned();
    match action {
        Action::Inline(marker) => {
            if selection.is_empty() {
                return None;
            }
            let span = block.spans.iter().find(|span| {
                span.range.start <= selection.start && selection.end <= span.range.end
            })?;
            if span.style.math.is_some()
                || span.style.footnote.is_some()
                || (span.style.code && marker != "`")
            {
                return None;
            }
            let styled = match marker {
                "**" => span.style.strong,
                "*" => span.style.emphasis,
                "~~" => span.style.strike,
                "`" => span.style.code,
                "==" => span.style.highlight,
                _ => return None,
            };
            let spans = [(span.range.clone(), span.source_range.clone())];
            let start = map_display_offset(source, &block.text, selection.start, &spans)?;
            let end = map_display_offset(source, &block.text, selection.end, &spans)?;
            if source.get(start..end)? != block.text.get(selection.clone())? {
                return None;
            }
            let selected = source.get(start..end)?;
            if selected.contains(marker) || selected.contains('\n') {
                return None;
            }
            if styled {
                if start < marker.len()
                    || source.get(start - marker.len()..start) != Some(marker)
                    || source.get(end..end + marker.len()) != Some(marker)
                {
                    return None;
                }
                if marker == "`"
                    && (source.as_bytes().get(start.wrapping_sub(2)) == Some(&b'`')
                        || source.as_bytes().get(end + 1) == Some(&b'`'))
                {
                    return None;
                }
                next.replace_range(end..end + marker.len(), "");
                next.replace_range(start - marker.len()..start, "");
                Some((next, start - marker.len()))
            } else {
                next.insert_str(end, marker);
                next.insert_str(start, marker);
                Some((next, start + marker.len()))
            }
        }
        Action::Block(prefix) => {
            let start = block.source_range.start;
            let mut end = start;
            if matches!(block.kind, Kind::Heading(_)) {
                let text = source.get(start..block.source_range.end)?;
                let hashes = text.bytes().take_while(|byte| *byte == b'#').count();
                if hashes == 0 || hashes > 6 {
                    return None;
                }
                end += hashes;
                while matches!(source.as_bytes().get(end), Some(b' ' | b'\t')) {
                    end += 1;
                }
            }
            next.replace_range(start..end, prefix);
            Some((next, start + prefix.len()))
        }
        Action::Split => {
            if !selection.is_empty() {
                return None;
            }
            let spans = block
                .spans
                .iter()
                .map(|span| (span.range.clone(), span.source_range.clone()))
                .collect::<Vec<_>>();
            let offset = map_display_offset(source, &block.text, selection.start, &spans)?;
            // A split must not cut any hidden inline delimiter or escaped syntax.
            if block.spans.iter().any(|span| {
                let style = &span.style;
                style.strong
                    || style.emphasis
                    || style.strike
                    || style.code
                    || style.link.is_some()
                    || style.superscript
                    || style.subscript
                    || style.highlight
                    || style.math.is_some()
                    || style.footnote.is_some()
                    || style.image.is_some()
                    || source.get(span.source_range.clone()) != block.text.get(span.range.clone())
            }) {
                return None;
            }
            let line_start = source[..block.source_range.start]
                .rfind('\n')
                .map_or(0, |index| index + 1);
            let prefix = source.get(line_start..block.source_range.start)?;
            let separator = if prefix.is_empty() {
                "\n\n".to_owned()
            } else {
                format!("\n{prefix}")
            };
            next.insert_str(offset, &separator);
            Some((next, offset + separator.len()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::MarkdownDocument;
    use super::*;

    #[test]
    fn inline_formatting_roundtrips_unicode_and_rejects_cross_style_ranges() {
        let document = MarkdownDocument::new("猫 text\n");
        let (source, _) = edit(
            document.source(),
            &document.blocks[0],
            0..3,
            Action::Inline("**"),
        )
        .unwrap();
        assert_eq!(source, "**猫** text\n");
        let document = MarkdownDocument::new(source);
        let (source, _) = edit(
            document.source(),
            &document.blocks[0],
            0..3,
            Action::Inline("**"),
        )
        .unwrap();
        assert_eq!(source, "猫 text\n");
        let document = MarkdownDocument::new("**bold** plain\n");
        assert!(
            edit(
                document.source(),
                &document.blocks[0],
                0..10,
                Action::Inline("*")
            )
            .is_none()
        );
    }

    #[test]
    fn heading_conversion_and_plain_block_split_preserve_neighbors() {
        let document = MarkdownDocument::new("## Title\n\nnext\n");
        let (source, _) = edit(
            document.source(),
            &document.blocks[0],
            0..0,
            Action::Block("# "),
        )
        .unwrap();
        assert_eq!(source, "# Title\n\nnext\n");
        let document = MarkdownDocument::new("first last\n\nnext\n");
        let (source, _) =
            edit(document.source(), &document.blocks[0], 5..5, Action::Split).unwrap();
        assert_eq!(source, "first\n\n last\n\nnext\n");
    }
    #[test]
    fn italic_preserves_bold_and_split_rejects_hidden_delimiters() {
        let document = MarkdownDocument::new("**bold**\n");
        let (source, _) = edit(
            document.source(),
            &document.blocks[0],
            0..4,
            Action::Inline("*"),
        )
        .unwrap();
        assert_eq!(source, "***bold***\n");
        let document = MarkdownDocument::new(source);
        let (source, _) = edit(
            document.source(),
            &document.blocks[0],
            0..4,
            Action::Inline("*"),
        )
        .unwrap();
        assert_eq!(source, "**bold**\n");
        for source in [
            "**hello world**\n",
            "~~hello world~~\n",
            "==hello world==\n",
            "`hello world`\n",
        ] {
            let document = MarkdownDocument::new(source);
            assert!(
                edit(source, &document.blocks[0], 5..5, Action::Split).is_none(),
                "{source}"
            );
        }
    }
}
