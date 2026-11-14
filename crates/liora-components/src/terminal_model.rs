use super::TerminalSize;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct CellPosition {
    pub row: u16,
    pub col: u16,
}

pub(super) struct TerminalModel {
    parser: vt100::Parser,
    size: TerminalSize,
}

impl TerminalModel {
    pub fn new(size: TerminalSize, scrollback_capacity: usize) -> Self {
        Self {
            parser: vt100::Parser::new(
                size.rows_nonzero(),
                size.cols_nonzero(),
                scrollback_capacity,
            ),
            size,
        }
    }

    pub fn process_output(&mut self, bytes: &[u8]) {
        self.parser.process(bytes);
    }

    pub fn resize(&mut self, size: TerminalSize) {
        if self.size != size {
            self.parser
                .set_size(size.rows_nonzero(), size.cols_nonzero());
            self.size = size;
        }
    }

    pub fn size(&self) -> TerminalSize {
        self.size
    }

    pub fn screen(&self) -> &vt100::Screen {
        self.parser.screen()
    }

    pub fn screen_mut(&mut self) -> &mut vt100::Screen {
        self.parser.screen_mut()
    }

    pub fn visible_text(&self) -> String {
        self.screen().contents()
    }

    pub fn scrollback_len(&self) -> usize {
        self.screen().scrollback_len()
    }

    pub fn scrollback(&self) -> usize {
        self.screen().scrollback()
    }

    pub fn set_scrollback(&mut self, rows: usize) {
        self.screen_mut().set_scrollback(rows);
    }

    pub fn scroll_by(&mut self, delta: i32) {
        let next = (self.scrollback() as i64 + i64::from(delta))
            .clamp(0, self.scrollback_len() as i64) as usize;
        self.set_scrollback(next);
    }

    pub fn selection_text(&self, anchor: CellPosition, head: CellPosition) -> Option<String> {
        if anchor == head {
            return None;
        }
        let (start, end) = if anchor < head {
            (anchor, head)
        } else {
            (head, anchor)
        };
        let end_col = end.col.saturating_add(1).min(self.size.cols());
        Some(
            self.screen()
                .contents_between(start.row, start.col, end.row, end_col),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn model(rows: u16, cols: u16, scrollback: usize) -> TerminalModel {
        TerminalModel::new(
            TerminalSize::new(rows, cols).expect("test size is valid"),
            scrollback,
        )
    }

    #[test]
    fn terminal_output_bytes_populate_cells_and_preserve_sgr_colors() {
        let mut terminal = model(3, 12, 0);
        terminal.process_output(b"plain \x1b[31mred\x1b[0m");

        assert_eq!(terminal.screen().cell(0, 0).unwrap().contents(), "p");
        assert_eq!(terminal.screen().cell(0, 6).unwrap().contents(), "r");
        assert_eq!(
            terminal.screen().cell(0, 6).unwrap().fgcolor(),
            vt100::Color::Idx(1)
        );
        assert_eq!(
            terminal.screen().cell(0, 9).unwrap().fgcolor(),
            vt100::Color::Default
        );
    }

    #[test]
    fn carriage_return_overwrites_from_the_first_column() {
        let mut terminal = model(2, 10, 0);
        terminal.process_output(b"abcdef\rXY");

        assert_eq!(
            terminal.screen().rows(0, 10).next().as_deref(),
            Some("XYcdef")
        );
    }

    #[test]
    fn clear_cursor_movement_and_sgr_change_screen_state() {
        let mut terminal = model(3, 8, 0);
        terminal.process_output(b"old text\x1b[2J\x1b[1;3H\x1b[32mX");
        let cell = terminal.screen().cell(0, 2).unwrap();

        assert!(!terminal.screen().cell(0, 0).unwrap().has_contents());
        assert_eq!(cell.contents(), "X");
        assert_eq!(cell.fgcolor(), vt100::Color::Idx(2));
        assert_eq!(terminal.screen().cursor_position(), (0, 3));
    }

    #[test]
    fn parser_keeps_utf8_codepoints_across_output_chunks() {
        let mut terminal = model(2, 8, 0);
        let bytes = "猫".as_bytes();
        terminal.process_output(&bytes[..1]);
        assert!(!terminal.screen().cell(0, 0).unwrap().has_contents());
        terminal.process_output(&bytes[1..]);

        assert_eq!(terminal.screen().cell(0, 0).unwrap().contents(), "猫");
    }

    #[test]
    fn alternate_screen_keeps_primary_screen_contents() {
        let mut terminal = model(3, 12, 8);
        terminal.process_output(b"primary\x1b[?1049halternate");

        assert!(terminal.screen().alternate_screen());
        assert_eq!(terminal.screen().cell(0, 0).unwrap().contents(), "a");
        terminal.process_output(b"\x1b[?1049l");
        assert!(!terminal.screen().alternate_screen());
        assert_eq!(terminal.screen().cell(0, 0).unwrap().contents(), "p");
    }

    #[test]
    fn scrollback_selection_returns_real_cells_and_resizes_the_parser() {
        let mut terminal = model(2, 10, 8);
        terminal.process_output(b"one\r\ntwo\r\nthree");
        assert_eq!(terminal.scrollback_len(), 1);
        terminal.set_scrollback(1);
        assert_eq!(
            terminal.selection_text(
                CellPosition { row: 0, col: 0 },
                CellPosition { row: 1, col: 2 },
            ),
            Some("one\ntwo".to_string()),
        );
        terminal.resize(TerminalSize::new(4, 20).unwrap());
        assert_eq!(terminal.size(), TerminalSize::new(4, 20).unwrap());
        assert_eq!(
            terminal.screen().size(),
            (
                std::num::NonZeroU16::new(4).unwrap(),
                std::num::NonZeroU16::new(20).unwrap()
            )
        );
    }

    #[test]
    fn scrollback_offset_is_clamped_to_available_history() {
        let mut terminal = model(2, 10, 8);
        terminal.process_output(b"a\r\nb\r\nc");
        terminal.set_scrollback(usize::MAX);

        assert_eq!(terminal.scrollback(), terminal.scrollback_len());
        terminal.scroll_by(i32::MIN);
        assert_eq!(terminal.scrollback(), 0);
    }
}
