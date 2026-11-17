use gpui::{
    App, AppContext, Context, Entity, IntoElement, Render, Subscription, Window, div, prelude::*,
    px,
};
use liora_components::{MarkdownEditor, MarkdownEditorChanged};

/// Retain the editor and subscription; the host owns storage of committed Markdown.
pub struct EditingExample {
    editor: Entity<MarkdownEditor>,
    source: String,
    _changes: Subscription,
}

impl EditingExample {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let source = "# Editable document\n\nNative **styled** text.\n".to_owned();
        let editor = cx.new(|cx| MarkdownEditor::new(source.clone(), cx));
        let changes = cx.subscribe(&editor, |host, _, event: &MarkdownEditorChanged, cx| {
            host.source = event.source.to_string();
            cx.notify();
        });
        Self {
            editor,
            source,
            _changes: changes,
        }
    }
}

impl Render for EditingExample {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        // The editor owns virtual scrolling; its host supplies a bounded height.
        div().w_full().h(px(420.)).child(self.editor.clone())
    }
}

pub fn select_word(editor: &Entity<MarkdownEditor>, window: &mut Window, cx: &mut App) {
    editor.update(cx, |editor, cx| {
        if let Some(start) = editor.source().find("styled") {
            editor.focus_source_offset(start, window, cx);
            editor.set_selection(start..start + "styled".len(), cx);
        }
    });
}
