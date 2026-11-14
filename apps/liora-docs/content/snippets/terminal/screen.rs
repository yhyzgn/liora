use gpui::{AnyView, App, prelude::*};
use liora_components::{Terminal, TerminalSize};

pub fn terminal_screen(cx: &mut App) -> AnyView {
    let terminal = cx.new(|cx| Terminal::new(TerminalSize::new(10, 64).unwrap(), 1000, cx));
    terminal.update(cx, |terminal, cx| {
        terminal.process_output(b"Host-provided output\r\n\x1b[32mANSI color\x1b[0m\r\nSelect text with the pointer; Ctrl-Shift-C copies.\r\n", cx);
    });
    terminal.into()
}
