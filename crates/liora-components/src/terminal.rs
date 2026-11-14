//! Stateful native GPUI terminal viewport backed by the VT100 parser.
//!
//! Hosts own their process and byte streams. Feed output with [`Terminal::process_output`],
//! subscribe to [`TerminalEvent::InputBytes`] for user input, and update dimensions through
//! [`Terminal::resize`].

#[path = "terminal_model.rs"]
mod model;

use gpui::{
    App, Bounds, ClipboardItem, Context, Element, ElementId, ElementInputHandler, Entity,
    EntityInputHandler, EventEmitter, FocusHandle, Focusable, FontWeight, GlobalElementId, Hsla,
    InspectorElementId, IntoElement, KeyDownEvent, LayoutId, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, Pixels, Point, Render, ScrollEvent, SharedString, Style, TextRun,
    UTF16Selection, UnderlineStyle, Window, canvas, div, font, point, prelude::*, px, relative,
};
use liora_core::{Config, code_font_family, code_font_weight, unique_id};
use model::{CellPosition, TerminalModel};
use std::ops::Range;

const CELL_WIDTH: f32 = 8.4;
const CELL_HEIGHT: f32 = 18.0;
const FONT_SIZE: f32 = 14.0;

/// Terminal grid dimensions. Rows and columns must each be in `1..=1024`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TerminalSize {
    rows: u16,
    cols: u16,
}

impl TerminalSize {
    /// The greatest number of rows or columns accepted by [`Self::new`].
    pub const MAX_DIMENSION: u16 = 1024;

    /// Creates a terminal size, rejecting zero or oversized dimensions.
    pub const fn new(rows: u16, cols: u16) -> Option<Self> {
        if rows == 0 || cols == 0 || rows > Self::MAX_DIMENSION || cols > Self::MAX_DIMENSION {
            None
        } else {
            Some(Self { rows, cols })
        }
    }

    /// Returns the row count.
    pub const fn rows(self) -> u16 {
        self.rows
    }

    /// Returns the column count.
    pub const fn cols(self) -> u16 {
        self.cols
    }

    const fn rows_nonzero(self) -> std::num::NonZeroU16 {
        // Construction validates non-zero dimensions.
        match std::num::NonZeroU16::new(self.rows) {
            Some(rows) => rows,
            None => unreachable!(),
        }
    }

    const fn cols_nonzero(self) -> std::num::NonZeroU16 {
        // Construction validates non-zero dimensions.
        match std::num::NonZeroU16::new(self.cols) {
            Some(cols) => cols,
            None => unreachable!(),
        }
    }
}

/// Events emitted by [`Terminal`] for the host-owned process and terminal size.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalEvent {
    /// Bytes to write to the host's terminal input stream.
    InputBytes(Vec<u8>),
    /// A size change requested by the host through [`Terminal::resize`].
    Resized(TerminalSize),
}

#[derive(Clone, Copy, Debug)]
struct Selection {
    anchor: CellPosition,
    head: CellPosition,
}

/// Native VT/ANSI terminal surface. The host owns all process, PTY, and I/O lifecycles.
pub struct Terminal {
    id: SharedString,
    model: TerminalModel,
    focus_handle: FocusHandle,
    bounds: Option<Bounds<Pixels>>,
    selecting: bool,
    selection: Option<Selection>,
    marked_text: String,
    marked_selection: Range<usize>,
}

impl Terminal {
    /// Creates a terminal entity state for use with `cx.new(|cx| Terminal::new(..., cx))`.
    pub fn new(size: TerminalSize, scrollback_capacity: usize, cx: &mut Context<Self>) -> Self {
        Self {
            id: unique_id("terminal"),
            model: TerminalModel::new(size, scrollback_capacity),
            focus_handle: cx.focus_handle(),
            bounds: None,
            selecting: false,
            selection: None,
            marked_text: String::new(),
            marked_selection: 0..0,
        }
    }

    /// Incrementally parses bytes received from the host's terminal output stream.
    pub fn process_output(&mut self, bytes: &[u8], cx: &mut Context<Self>) {
        self.model.process_output(bytes);
        let max_scrollback = self.model.scrollback_len();
        self.selection = None;
        self.selecting = false;
        let current_scrollback = self.model.scrollback();
        if current_scrollback > max_scrollback {
            self.model.set_scrollback(max_scrollback);
        }
        cx.notify();
    }

    /// Resizes the VT screen and emits [`TerminalEvent::Resized`] when dimensions change.
    pub fn resize(&mut self, size: TerminalSize, cx: &mut Context<Self>) {
        if self.model.size() == size {
            return;
        }
        self.model.resize(size);
        self.selection = None;
        self.selecting = false;
        self.model.set_scrollback(self.model.scrollback());
        cx.emit(TerminalEvent::Resized(size));
        cx.notify();
    }

    /// Returns current grid dimensions.
    pub fn size(&self) -> TerminalSize {
        self.model.size()
    }

    /// Returns the number of retained scrollback rows.
    pub fn scrollback_len(&self) -> usize {
        self.model.scrollback_len()
    }

    /// Returns the current user scrollback offset; zero tracks the live screen.
    pub fn scrollback_offset(&self) -> usize {
        self.model.scrollback()
    }

    /// Returns plain text for the currently visible screen or scrollback viewport.
    pub fn visible_text(&self) -> String {
        self.model.visible_text()
    }

    /// Returns the plain text currently covered by a non-empty pointer selection.
    pub fn selected_text(&self) -> Option<String> {
        self.selection
            .and_then(|range| self.model.selection_text(range.anchor, range.head))
    }

    /// Copies the current selection to the native clipboard, if it is non-empty.
    pub fn copy_selection(&self, cx: &mut Context<Self>) -> Option<String> {
        let text = self.selected_text()?;
        cx.write_to_clipboard(ClipboardItem::new_string(text.clone()));
        Some(text)
    }

    /// Returns whether VT output has switched to the alternate screen.
    pub fn is_alternate_screen(&self) -> bool {
        self.model.screen().alternate_screen()
    }

    /// Returns the focus handle used by this terminal.
    pub fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }

    fn emit_input(&mut self, bytes: Vec<u8>, cx: &mut Context<Self>) {
        if !bytes.is_empty() {
            cx.emit(TerminalEvent::InputBytes(bytes));
        }
    }

    fn scroll(&mut self, event: &ScrollEvent, _: &mut Window, cx: &mut Context<Self>) {
        let delta = event.delta.pixel_delta(px(CELL_HEIGHT)).y;
        let rows = if delta > px(0.0) {
            (f32::from(delta) / CELL_HEIGHT).ceil().max(1.0) as i32
        } else if delta < px(0.0) {
            -((f32::from(-delta) / CELL_HEIGHT).ceil().max(1.0) as i32)
        } else {
            return;
        };
        self.model.scroll_by(rows);
        self.selection = None;
        cx.notify();
    }

    fn position_for_point(&self, position: Point<Pixels>) -> CellPosition {
        let size = self.model.size();
        let origin = self
            .bounds
            .map_or(point(px(0.0), px(0.0)), |bounds| bounds.origin);
        let col = ((f32::from(position.x - origin.x) / CELL_WIDTH).floor() as i32)
            .clamp(0, i32::from(size.cols() - 1)) as u16;
        let row = ((f32::from(position.y - origin.y) / CELL_HEIGHT).floor() as i32)
            .clamp(0, i32::from(size.rows() - 1)) as u16;
        CellPosition { row, col }
    }

    fn select_at(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
        let cell = self.position_for_point(position);
        if let Some(selection) = &mut self.selection {
            selection.head = cell;
        } else {
            self.selection = Some(Selection {
                anchor: cell,
                head: cell,
            });
        }
        cx.notify();
    }

    fn mouse_down(&mut self, event: &MouseDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        if event.button != MouseButton::Left {
            return;
        }
        window.focus(&self.focus_handle, cx);
        self.selection = Some(Selection {
            anchor: self.position_for_point(event.position),
            head: self.position_for_point(event.position),
        });
        self.selecting = true;
        cx.notify();
    }

    fn mouse_move(&mut self, event: &MouseMoveEvent, _: &mut Window, cx: &mut Context<Self>) {
        if self.selecting && event.pressed_button == Some(MouseButton::Left) {
            self.select_at(event.position, cx);
        }
    }

    fn mouse_up(&mut self, event: &MouseUpEvent, _: &mut Window, cx: &mut Context<Self>) {
        if event.button == MouseButton::Left && self.selecting {
            self.selecting = false;
            cx.notify();
        }
    }

    fn key_down(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        if !self.marked_text.is_empty() {
            return;
        }
        let keystroke = &event.keystroke;
        let modifiers = keystroke.modifiers;
        let key = keystroke.key.as_str();
        let is_copy = (modifiers.platform && key == "c")
            || (modifiers.control && modifiers.shift && key == "c");
        if is_copy {
            self.copy_selection(cx);
            cx.stop_propagation();
            return;
        }

        let is_paste = (modifiers.platform && key == "v")
            || (modifiers.control && modifiers.shift && key == "v");
        if is_paste {
            if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
                let mut bytes = Vec::with_capacity(text.len() + 12);
                if self.model.screen().bracketed_paste() {
                    bytes.extend_from_slice(b"\x1b[200~");
                    bytes.extend_from_slice(text.as_bytes());
                    bytes.extend_from_slice(b"\x1b[201~");
                } else {
                    bytes.extend_from_slice(text.as_bytes());
                }
                self.emit_input(bytes, cx);
            }
            cx.stop_propagation();
            return;
        }

        let encoded =
            encode_key(key, modifiers, self.model.screen().application_cursor()).or_else(|| {
                (modifiers.alt && !modifiers.control)
                    .then(|| keystroke.key_char.as_ref())
                    .flatten()
                    .map(|text| [b"\x1b".as_slice(), text.as_bytes()].concat())
            });
        if let Some(bytes) = encoded {
            self.emit_input(bytes, cx);
            cx.stop_propagation();
            return;
        }

        if modifiers.control && !modifiers.alt && key == "c" {
            self.emit_input(vec![0x03], cx);
            cx.stop_propagation();
        } else if modifiers.control && !modifiers.alt && key == "v" {
            self.emit_input(vec![0x16], cx);
            cx.stop_propagation();
        } else if modifiers.control && !modifiers.alt && key.len() == 1 {
            if let Some(byte) = key.as_bytes().first().copied() {
                let control = byte.to_ascii_uppercase();
                if (b'@'..=b'_').contains(&control) {
                    self.emit_input(vec![control & 0x1f], cx);
                    cx.stop_propagation();
                }
            }
        }
        let _ = window;
    }

    fn render_row(
        &self,
        row: u16,
        code_family: &SharedString,
        code_weight: Option<FontWeight>,
        default_fg: Hsla,
        default_bg: Hsla,
    ) -> impl IntoElement + use<> {
        let size = self.model.size();
        let screen = self.model.screen();
        let cursor = screen.cursor_position();
        let cursor_visible = !screen.hide_cursor() && screen.scrollback() == 0;
        let selection = self.selection;
        let mut text = String::with_capacity(usize::from(size.cols()));
        let mut runs = Vec::with_capacity(usize::from(size.cols()));

        for col in 0..size.cols() {
            let cell = screen.cell(row, col);
            let mut fg = cell.map_or(default_fg, |cell| vt_color(cell.fgcolor(), default_fg));
            let mut bg = cell.map_or(default_bg, |cell| vt_color(cell.bgcolor(), default_bg));
            let bold = cell.is_some_and(vt100::Cell::bold);
            let italic = cell.is_some_and(vt100::Cell::italic);
            let underline = cell.is_some_and(vt100::Cell::underline);
            let inverse = cell.is_some_and(vt100::Cell::inverse);
            if inverse {
                std::mem::swap(&mut fg, &mut bg);
            }
            if cell.is_some_and(vt100::Cell::dim) {
                fg = fg.opacity(0.6);
            }
            let selected = selection.is_some_and(|selection| {
                let pos = CellPosition { row, col };
                let start = selection.anchor.min(selection.head);
                let end = selection.anchor.max(selection.head);
                pos >= start && pos <= end
            });
            if selected {
                fg = gpui::white();
                bg = gpui::blue();
            }
            if cursor_visible && cursor == (row, col) {
                std::mem::swap(&mut fg, &mut bg);
            }

            let content = render_cell_content(cell);
            text.push_str(content);

            let mut run_font = font(code_family.clone());
            if bold {
                run_font.weight = FontWeight::BOLD;
            } else if let Some(weight) = code_weight {
                run_font.weight = weight;
            }
            if italic {
                run_font.style = gpui::FontStyle::Italic;
            }
            runs.push(TextRun {
                len: content.len(),
                font: run_font,
                color: fg,
                background_color: Some(bg),
                underline: underline.then_some(UnderlineStyle {
                    thickness: px(1.0),
                    color: Some(fg),
                    wavy: false,
                }),
                strikethrough: None,
            });
        }

        canvas(
            move |_, window, _| {
                window.text_system().shape_line(
                    text.into(),
                    px(FONT_SIZE),
                    &runs,
                    Some(px(CELL_WIDTH)),
                )
            },
            move |bounds, line, window, cx| {
                let result = line
                    .paint_background(
                        bounds.origin,
                        px(CELL_HEIGHT),
                        gpui::TextAlign::Left,
                        None,
                        window,
                        cx,
                    )
                    .and_then(|_| {
                        line.paint(
                            bounds.origin,
                            px(CELL_HEIGHT),
                            gpui::TextAlign::Left,
                            None,
                            window,
                            cx,
                        )
                    });
                if let Err(error) = result {
                    eprintln!("Terminal row paint failed: {error}");
                }
            },
        )
        .flex_shrink_0()
        .h(px(CELL_HEIGHT))
        .w(px(f32::from(size.cols()) * CELL_WIDTH))
    }
}

impl Focusable for Terminal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<TerminalEvent> for Terminal {}

impl Render for Terminal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let code_family = code_font_family(cx);
        let code_weight = code_font_weight(cx);
        let size = self.model.size();
        let entity = cx.entity().clone();
        let id = self.id.clone();
        let default_fg = theme.neutral.text_1;
        let default_bg = theme.neutral.card;
        let rows = (0..size.rows())
            .map(|row| self.render_row(row, &code_family, code_weight, default_fg, default_bg))
            .collect::<Vec<_>>();

        let mut root = div()
            .id(id)
            .relative()
            .text_size(px(FONT_SIZE))
            .line_height(px(CELL_HEIGHT))
            .w(px(f32::from(size.cols()) * CELL_WIDTH))
            .h(px(f32::from(size.rows()) * CELL_HEIGHT))
            .flex()
            .flex_col()
            .overflow_hidden()
            .bg(default_bg)
            .track_focus(&self.focus_handle)
            .capture_key_down(cx.listener(Self::key_down))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::mouse_down))
            .on_mouse_move(cx.listener(Self::mouse_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::mouse_up))
            .on_scroll(cx.listener(Self::scroll));
        for row in rows {
            root = root.child(row);
        }
        if !self.marked_text.is_empty() {
            let (row, col) = self.model.screen().cursor_position();
            root = root.child(
                div()
                    .absolute()
                    .left(px(f32::from(col) * CELL_WIDTH))
                    .top(px(f32::from(row) * CELL_HEIGHT))
                    .bg(default_bg)
                    .text_color(default_fg)
                    .font_family(code_family)
                    .child(self.marked_text.clone()),
            );
        }
        root.child(TerminalInputLayer { terminal: entity })
    }
}

impl EntityInputHandler for Terminal {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        _: &mut Option<Range<usize>>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<String> {
        let utf16 = self.marked_text.encode_utf16().collect::<Vec<_>>();
        let start = range_utf16.start.min(utf16.len());
        let end = range_utf16.end.max(start).min(utf16.len());
        Some(
            char::decode_utf16(utf16[start..end].iter().copied())
                .map(|unit| unit.unwrap_or(char::REPLACEMENT_CHARACTER))
                .collect(),
        )
    }

    fn selected_text_range(
        &mut self,
        _: bool,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        Some(UTF16Selection {
            range: self.marked_selection.clone(),
            reversed: false,
        })
    }

    fn marked_text_range(&self, _: &mut Window, _: &mut Context<Self>) -> Option<Range<usize>> {
        (!self.marked_text.is_empty()).then(|| 0..self.marked_text.encode_utf16().count())
    }

    fn unmark_text(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.marked_text.clear();
        self.marked_selection = 0..0;
        cx.notify();
    }

    fn replace_text_in_range(
        &mut self,
        replacement_range: Option<Range<usize>>,
        text: &str,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        replace_composition(&mut self.marked_text, replacement_range, text);
        let committed = std::mem::take(&mut self.marked_text);
        self.marked_selection = 0..0;
        self.emit_input(committed.into_bytes(), cx);
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        replacement_range: Option<Range<usize>>,
        new_text: &str,
        new_selected_range: Option<Range<usize>>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let inserted = replace_composition(&mut self.marked_text, replacement_range, new_text);
        let inserted_len = inserted.end - inserted.start;
        self.marked_selection = new_selected_range.map_or(inserted.end..inserted.end, |range| {
            let start = range.start.min(inserted_len);
            let end = range.end.max(start).min(inserted_len);
            inserted.start + start..inserted.start + end
        });
        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        _: Range<usize>,
        element_bounds: Bounds<Pixels>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let bounds = self.bounds.unwrap_or(element_bounds);
        let (row, col) = self.model.screen().cursor_position();
        let origin = point(
            bounds.origin.x + px(f32::from(col) * CELL_WIDTH),
            bounds.origin.y + px(f32::from(row) * CELL_HEIGHT),
        );
        Some(Bounds {
            origin,
            size: gpui::size(px(CELL_WIDTH), px(CELL_HEIGHT)),
        })
    }

    fn character_index_for_point(
        &mut self,
        _: Point<Pixels>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<usize> {
        Some(self.marked_selection.start)
    }
}

struct TerminalInputLayer {
    terminal: Entity<Terminal>,
}

impl Element for TerminalInputLayer {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.position = gpui::Position::Absolute;
        style.inset.top = px(0.0).into();
        style.inset.left = px(0.0).into();
        style.size.width = relative(1.0).into();
        style.size.height = relative(1.0).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _: &mut Window,
        _: &mut App,
    ) -> Self::PrepaintState {
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.terminal.read(cx).focus_handle.clone();
        self.terminal
            .update(cx, |terminal, _| terminal.bounds = Some(bounds));
        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.terminal.clone()),
            cx,
        );
    }
}

fn encode_key(key: &str, modifiers: gpui::Modifiers, application_cursor: bool) -> Option<Vec<u8>> {
    let has_modifiers = modifiers.shift || modifiers.alt || modifiers.control;
    let modifier_code = 1
        + u8::from(modifiers.shift)
        + 2 * u8::from(modifiers.alt)
        + 4 * u8::from(modifiers.control);
    let arrow = match key {
        "up" => Some(b'A'),
        "down" => Some(b'B'),
        "right" => Some(b'C'),
        "left" => Some(b'D'),
        _ => None,
    };
    if let Some(code) = arrow {
        return Some(if has_modifiers {
            format!("\x1b[1;{modifier_code}{}", char::from(code)).into_bytes()
        } else if application_cursor {
            vec![0x1b, b'O', code]
        } else {
            vec![0x1b, b'[', code]
        });
    }

    let bytes: &[u8] = match key {
        "enter" => b"\r",
        "tab" if modifiers.shift => b"\x1b[Z",
        "tab" => b"\t",
        "backspace" => b"\x7f",
        "escape" => b"\x1b",
        "home" => b"\x1b[H",
        "end" => b"\x1b[F",
        "insert" => b"\x1b[2~",
        "delete" => b"\x1b[3~",
        "pageup" => b"\x1b[5~",
        "pagedown" => b"\x1b[6~",
        _ => return None,
    };
    Some(bytes.to_vec())
}

fn vt_color(color: vt100::Color, default: Hsla) -> Hsla {
    let rgb = match color {
        vt100::Color::Default => return default,
        vt100::Color::Rgb(r, g, b) => (u32::from(r) << 16) | (u32::from(g) << 8) | u32::from(b),
        vt100::Color::Idx(index) => xterm_color(index),
    };
    gpui::rgb(rgb).into()
}

fn xterm_color(index: u8) -> u32 {
    const ANSI: [u32; 16] = [
        0x000000, 0xcd0000, 0x00cd00, 0xcdcd00, 0x0000ee, 0xcd00cd, 0x00cdcd, 0xe5e5e5, 0x7f7f7f,
        0xff0000, 0x00ff00, 0xffff00, 0x5c5cff, 0xff00ff, 0x00ffff, 0xffffff,
    ];
    match index {
        0..=15 => ANSI[usize::from(index)],
        16..=231 => {
            let value = u16::from(index - 16);
            let component = |shift: u16| match ((value / shift) % 6) as u8 {
                0 => 0,
                level => 55 + 40 * level,
            };
            (u32::from(component(36)) << 16)
                | (u32::from(component(6)) << 8)
                | u32::from(component(1))
        }
        _ => {
            let gray = 8 + 10 * u32::from(index - 232);
            (gray << 16) | (gray << 8) | gray
        }
    }
}

// The input document contains only the uncommitted composition, never process output.
// GPUI replacement ranges are UTF-16; round a split surrogate down to its scalar boundary.
fn replace_composition(
    buffer: &mut String,
    range: Option<Range<usize>>,
    text: &str,
) -> Range<usize> {
    let length = buffer.encode_utf16().count();
    let range = range.unwrap_or(0..length);
    let start = range.start.min(length);
    let end = range.end.max(start).min(length);
    let byte_offset = |offset| {
        let mut units = 0;
        for (byte, ch) in buffer.char_indices() {
            if units + ch.len_utf16() > offset {
                return byte;
            }
            units += ch.len_utf16();
        }
        buffer.len()
    };
    let byte_start = byte_offset(start);
    let byte_end = byte_offset(end);
    let actual_start = buffer[..byte_start].encode_utf16().count();
    buffer.replace_range(byte_start..byte_end, text);
    actual_start..actual_start + text.encode_utf16().count()
}

fn render_cell_content(cell: Option<&vt100::Cell>) -> &str {
    if cell.is_some_and(vt100::Cell::is_wide_continuation) {
        " "
    } else {
        cell.filter(|cell| cell.has_contents())
            .map_or(" ", vt100::Cell::contents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_composition_replaces_utf16_ranges_without_losing_surrounding_text() {
        let mut pending = "a😀z".to_string();
        assert_eq!(replace_composition(&mut pending, Some(1..3), "候选"), 1..3);
        assert_eq!(pending, "a候选z");
        replace_composition(&mut pending, Some(2..3), "定");
        assert_eq!(pending, "a候定z");
        replace_composition(&mut pending, None, "确认");
        assert_eq!(pending, "确认");
        replace_composition(&mut pending, Some(usize::MAX..usize::MAX), "!");
        assert_eq!(pending, "确认!");
    }

    #[test]
    fn terminal_size_rejects_zero_and_oversized_dimensions() {
        assert!(TerminalSize::new(0, 80).is_none());
        assert!(TerminalSize::new(24, 0).is_none());
        assert!(TerminalSize::new(1025, 80).is_none());
        assert!(TerminalSize::new(24, 1025).is_none());
        assert_eq!(TerminalSize::new(24, 80).unwrap().rows(), 24);
        assert_eq!(TerminalSize::new(24, 80).unwrap().cols(), 80);
    }
    #[test]
    fn wide_cell_render_placeholder_preserves_following_vt_column() {
        let mut model = TerminalModel::new(TerminalSize::new(1, 8).unwrap(), 0);
        model.process_output("猫A".as_bytes());
        let row = (0..model.size().cols())
            .map(|col| render_cell_content(model.screen().cell(0, col)))
            .collect::<String>();
        assert_eq!(row, "猫 A     ");
        let a_byte = row.find('A').unwrap();
        assert_eq!(row[..a_byte].chars().count(), 2);
    }

    #[test]
    fn terminal_key_encoding_emits_ctrl_and_cursor_mode_bytes() {
        let ctrl = gpui::Modifiers {
            control: true,
            ..Default::default()
        };
        assert_eq!(encode_key("left", ctrl, false), Some(b"\x1b[1;5D".to_vec()));
        assert_eq!(
            encode_key("up", Default::default(), true),
            Some(b"\x1bOA".to_vec())
        );
        assert_eq!(
            encode_key("down", Default::default(), false),
            Some(b"\x1b[B".to_vec())
        );
        assert_eq!(
            encode_key("enter", Default::default(), false),
            Some(b"\r".to_vec())
        );
    }

    #[test]
    fn xterm_color_index_maps_256_color_ranges() {
        assert_eq!(xterm_color(1), 0xcd0000);
        assert_eq!(xterm_color(16), 0x000000);
        assert_eq!(xterm_color(231), 0xffffff);
        assert_eq!(xterm_color(232), 0x080808);
        assert_eq!(xterm_color(255), 0xeeeeee);
    }
}
