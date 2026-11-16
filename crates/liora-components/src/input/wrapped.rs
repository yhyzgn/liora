//! Opt-in native wrapped input; all geometry uses the same GPUI shaped lines.
use super::*;
use gpui::{AvailableSpace, WrappedLine};
use std::{cell::RefCell, rc::Rc};

pub(super) struct WrappedText {
    lines: Vec<WrappedLine>,
    starts: Vec<usize>,
    height: Pixels,
}

pub(super) struct WrappedInputLayout {
    text: Rc<WrappedText>,
    pub(super) bounds: Bounds<Pixels>,
    pub(super) line_height: Pixels,
    cursor: usize,
    source: SharedString,
}

impl WrappedInputLayout {
    pub(super) fn position(&self, offset: usize) -> Point<Pixels> {
        let mut y = self.bounds.top();
        for (line, start) in self.text.lines.iter().zip(&self.text.starts) {
            if offset <= start + line.len() {
                let index = offset.saturating_sub(*start).min(line.len());
                let mut local = line
                    .position_for_index(index, self.line_height)
                    .unwrap_or_default();
                // Native position_for_index favors the previous row at a shared boundary.
                for (row, boundary) in line.wrap_boundaries.iter().enumerate() {
                    if line.unwrapped_layout.runs[boundary.run_ix].glyphs[boundary.glyph_ix].index
                        == index
                    {
                        local = point(px(0.), self.line_height * (row + 1) as f32);
                        break;
                    }
                }
                return point(self.bounds.left() + local.x, y + local.y);
            }
            y += line.size(self.line_height).height;
        }
        point(self.bounds.left(), y - self.line_height)
    }

    pub(super) fn index(&self, position: Point<Pixels>) -> usize {
        let mut y = self.bounds.top();
        for (line, start) in self.text.lines.iter().zip(&self.text.starts) {
            let height = line.size(self.line_height).height;
            if position.y < y + height {
                let local = point(
                    position.x - self.bounds.left(),
                    (position.y - y).max(px(0.)),
                );
                return start
                    + line
                        .closest_index_for_position(local, self.line_height)
                        .unwrap_or_else(|index| index);
            }
            y += height;
        }
        self.source.len()
    }

    pub(super) fn range_bounds(&self, range: Range<usize>) -> Bounds<Pixels> {
        let start = self.position(range.start);
        let end = self.position(range.end);
        let right = if end.y == start.y {
            end.x.max(start.x + px(1.))
        } else {
            self.bounds.right()
        };
        Bounds::new(start, size((right - start.x).max(px(1.)), self.line_height))
    }
}

pub(super) struct WrappedInputElement {
    input: Entity<Input>,
}

impl WrappedInputElement {
    pub(super) fn new(input: Entity<Input>) -> Self {
        Self { input }
    }
}

type MeasuredText = Rc<RefCell<Option<(Pixels, Rc<WrappedText>)>>>;

fn shape(
    text: SharedString,
    font_size: Pixels,
    line_height: Pixels,
    run: &TextRun,
    width: Pixels,
    window: &Window,
) -> Rc<WrappedText> {
    let lines = window
        .text_system()
        .shape_text(
            text,
            font_size,
            std::slice::from_ref(run),
            Some(width),
            None,
        )
        .expect("native wrapped input text shaping failed")
        .into_vec();
    let mut offset = 0;
    let mut height = px(0.);
    let starts = lines
        .iter()
        .map(|line| {
            let start = offset;
            offset += line.len() + 1;
            height += line.size(line_height).height;
            start
        })
        .collect();
    Rc::new(WrappedText {
        lines,
        starts,
        height,
    })
}

impl Element for WrappedInputElement {
    type RequestLayoutState = (MeasuredText, SharedString, Pixels, Pixels, TextRun);
    type PrepaintState = Rc<WrappedInputLayout>;

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
        let input = self.input.read(cx);
        let text = input.text_for_display(cx);
        let mut text_style = window.text_style();
        if let Some(family) = ui_font_family(cx) {
            text_style.font_family = family;
        }
        if let Some(weight) = ui_font_weight(cx) {
            text_style.font_weight = weight;
        }
        let font_size = text_style.font_size.to_pixels(window.rem_size());
        let line_height = window.line_height();
        let theme = &cx.global::<Config>().theme;
        let run = TextRun {
            len: text.len(),
            font: text_style.font(),
            color: if input.value.is_empty() {
                theme.neutral.placeholder
            } else if input.disabled {
                theme.neutral.text_disabled
            } else {
                theme.neutral.text_1
            },
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        let min_rows = input.min_rows;
        let cache: MeasuredText = Rc::default();
        let measured = cache.clone();
        let source = text.clone();
        let measured_run = run.clone();
        let mut style = Style::default();
        style.size.width = gpui::relative(1.).into();
        style.min_size.width = px(0.).into();
        let id = window.request_measured_layout(style, move |known, available, window, _| {
            let width = known
                .width
                .unwrap_or_else(|| match available.width {
                    AvailableSpace::Definite(width) => width,
                    _ => px(640.),
                })
                .max(px(1.));
            let cached = measured
                .borrow()
                .as_ref()
                .filter(|(previous, _)| *previous == width)
                .map(|(_, text)| text.clone());
            let layout = cached.unwrap_or_else(|| {
                shape(
                    source.clone(),
                    font_size,
                    line_height,
                    &measured_run,
                    width,
                    window,
                )
            });
            let height = layout.height.max(line_height * min_rows as f32);
            *measured.borrow_mut() = Some((width, layout));
            size(width, height)
        });
        (id, (cache, text, font_size, line_height, run))
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        state: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let (cache, text, font_size, line_height, run) = state;
        let width = bounds.size.width.max(px(1.));
        let cached = cache
            .borrow()
            .as_ref()
            .filter(|(previous, _)| *previous == width)
            .map(|(_, text)| text.clone());
        let text = cached
            .unwrap_or_else(|| shape(text.clone(), *font_size, *line_height, run, width, window));
        let input = self.input.read(cx);
        let layout = Rc::new(WrappedInputLayout {
            text,
            bounds,
            line_height: *line_height,
            cursor: input.cursor_offset(),
            source: input.value.clone(),
        });
        let moved = input.wrapped_layout.as_ref().is_none_or(|previous| {
            previous.cursor != layout.cursor
                || previous.source != layout.source
                || previous.bounds.size != bounds.size
        });
        if moved && input.focus_handle.is_focused(window) {
            window.request_autoscroll(layout.range_bounds(layout.cursor..layout.cursor));
        }
        self.input.update(cx, |input, _| {
            input.wrapped_layout = Some(layout.clone());
        });
        layout
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        layout: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let input = self.input.read(cx);
        let focus = input.focus_handle.clone();
        let selection = input.selected_range.clone();
        let cursor_visible = input.cursor_visible;
        let theme = cx.global::<Config>().theme.clone();
        window.handle_input(
            &focus,
            ElementInputHandler::new(bounds, self.input.clone()),
            cx,
        );
        let mut y = bounds.top();
        for (line, start) in layout.text.lines.iter().zip(&layout.text.starts) {
            let mut row_start = 0;
            let row_ends = line
                .wrap_boundaries
                .iter()
                .map(|boundary| {
                    line.unwrapped_layout.runs[boundary.run_ix].glyphs[boundary.glyph_ix].index
                })
                .chain(std::iter::once(line.len()));
            for (row, row_end) in row_ends.enumerate() {
                let from = selection.start.max(start + row_start);
                let to = selection.end.min(start + row_end);
                if from < to {
                    let origin_x = line.unwrapped_layout.x_for_index(row_start);
                    let left = line.unwrapped_layout.x_for_index(from - start) - origin_x;
                    let right = line.unwrapped_layout.x_for_index(to - start) - origin_x;
                    window.paint_quad(fill(
                        Bounds::new(
                            point(bounds.left() + left, y + layout.line_height * row as f32),
                            size(right - left, layout.line_height),
                        ),
                        theme.primary.base.opacity(0.3),
                    ));
                }
                row_start = row_end;
            }
            let _ = line.paint(
                point(bounds.left(), y),
                layout.line_height,
                gpui::TextAlign::Left,
                Some(bounds),
                window,
                cx,
            );
            y += line.size(layout.line_height).height;
        }
        if focus.is_focused(window) && selection.is_empty() && cursor_visible {
            window.paint_quad(fill(
                layout.range_bounds(layout.cursor..layout.cursor),
                theme.primary.base,
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{MouseMoveEvent, PlatformInput, TestAppContext};

    struct Surface(Entity<Input>);
    impl Render for Surface {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            gpui::div().w_full().child(self.0.clone())
        }
    }

    #[test]
    fn wrapped_input_keeps_end_caret_hit_testing_and_ime_on_visual_rows() {
        let mut app = TestAppContext::single();
        app.update(crate::init_liora);
        let text =
            "A long paragraph with 猫 and native text that wraps across many narrow visual lines. "
                .repeat(5);
        let input = app.new(|cx| Input::new(text.clone(), cx).soft_wrap(true));
        let handle = app.open_window(size(px(240.), px(900.)), |_, _| Surface(input.clone()));
        app.update_window(handle.into(), |_, window, cx| {
            window.focus(&input.focus_handle(cx), cx);
            window.draw(cx).clear(cx);
            input.update(cx, |input, cx| {
                input.end(&End, window, cx);
                input.insert_text("END", cx);
            });
        })
        .unwrap();
        app.update_window(handle.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
            let layout = input.read(cx).wrapped_layout.clone().unwrap();
            assert!(layout.bounds.size.height > layout.line_height * 3.);
            let end = layout.position(input.read(cx).value.len());
            assert!(end.x >= layout.bounds.left() && end.x <= layout.bounds.right());
            assert!(end.y >= layout.bounds.top() && end.y < layout.bounds.bottom());
            let row_position = point(
                layout.bounds.left() + px(20.),
                layout.bounds.top() + layout.line_height * 1.5,
            );
            let expected = layout.index(row_position);
            window.dispatch_event(
                PlatformInput::MouseMove(MouseMoveEvent {
                    position: row_position,
                    ..Default::default()
                }),
                cx,
            );
            window.dispatch_event(
                PlatformInput::MouseDown(MouseDownEvent {
                    position: row_position,
                    button: MouseButton::Left,
                    click_count: 1,
                    ..Default::default()
                }),
                cx,
            );
            assert_eq!(input.read(cx).cursor_offset(), expected);
            input.update(cx, |input, cx| {
                input.move_vertical(1, false, cx);
                assert!(input.cursor_offset() > expected);
                input.move_vertical(-1, true, cx);
                assert!(!input.selected_range.is_empty());
                input.set_selection(0..0, cx);
                input.replace_and_mark_text_in_range(Some(0..0), "候选", None, window, cx);
            });
        })
        .unwrap();
        app.update_window(handle.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
            input.update(cx, |input, cx| {
                let bounds = input
                    .bounds_for_range(0..2, Bounds::default(), window, cx)
                    .unwrap();
                let layout = input.wrapped_layout.as_ref().unwrap();
                assert!(bounds.origin.y >= layout.bounds.top());
                assert!(bounds.origin.x >= layout.bounds.left());
                input.unmark_text(window, cx);
                input.set_value("ab", cx);
            });
        })
        .unwrap();
        app.update_window(handle.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
            input.update(cx, |input, cx| {
                input.set_selection(0..0, cx);
                input.insert_text("猫", cx);
                input.move_vertical(1, false, cx);
                assert!(input.value.is_char_boundary(input.cursor_offset()));
                let _ = input.selected_text_range(false, window, cx);
                input.set_value("猫\nlonger second line", cx);
                input.set_selection(0..0, cx);
                input.move_vertical(1, false, cx);
                assert_eq!(input.cursor_offset(), "猫\n".len());
            });
        })
        .unwrap();
        app.quit();
    }
}
