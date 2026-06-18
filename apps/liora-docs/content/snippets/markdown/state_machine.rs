//! Simplified Markdown parser state-machine dispatch.

use pulldown_cmark::{Event, Tag, TagEnd};

#[derive(Default)]
struct ParserState;

#[derive(Clone, Copy)]
struct InlineStyle;

enum Block {
    Rule,
}

impl ParserState {
    fn start_tag(&mut self, _tag: Tag<'_>) {}
    fn end_tag(&mut self, _tag: TagEnd) {}
    fn push_text_with_live_demos(&mut self, _text: &str, _style: InlineStyle) {}
    fn push_inline_code(&mut self, _text: &str) {}
    fn push_block(&mut self, _block: Block) {}
}

fn handle_event(event: Event<'_>, state: &mut ParserState, style: InlineStyle) {
    match event {
        Event::Start(tag) => state.start_tag(tag),
        Event::End(tag) => state.end_tag(tag),
        Event::Text(text) => state.push_text_with_live_demos(text.as_ref(), style),
        Event::Code(text) => state.push_inline_code(text.as_ref()),
        Event::Rule => state.push_block(Block::Rule),
        _ => {}
    }
}

fn main() {}
