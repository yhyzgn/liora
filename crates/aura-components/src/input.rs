use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    AnyElement, App, Bounds, Context, Element, ElementId, ElementInputHandler, Entity,
    EntityInputHandler, FocusHandle, Focusable, GlobalElementId, InspectorElementId, IntoElement,
    KeyBinding, LayoutId, MouseButton, MouseDownEvent, MouseMoveEvent, Pixels, Point, Render,
    ShapedLine, SharedString, Style, TextRun, UTF16Selection, Window, actions, fill, point,
    prelude::*, px, size,
};
use std::ops::{Add, Range};

actions!(
    input,
    [
        Backspace,
        Delete,
        Left,
        Right,
        Home,
        End,
        SelectAll,
        Enter,
        InputUp,
        InputDown,
        Copy,
        Paste,
        Cut,
        SelectLeft,
        SelectRight,
        SelectUp,
        SelectDown,
        SelectHome,
        SelectEnd,
        TogglePassword
    ]
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputType {
    Text,
    Password,
}

pub struct Input {
    value: SharedString,
    placeholder: SharedString,
    disabled: bool,
    clearable: bool,
    icon_prefix: Option<IconName>,
    icon_suffix: Option<IconName>,
    focus_handle: FocusHandle,
    selected_range: Range<usize>,
    selection_reversed: bool,
    marked_range: Option<Range<usize>>,
    last_line_layouts: Vec<(ShapedLine, Pixels)>,
    last_bounds: Option<Bounds<Pixels>>,
    last_layout_is_masked: bool,
    cursor_visible: bool,
    blink_task: Option<gpui::Task<()>>,
    filter: Option<Box<dyn Fn(&str) -> bool + 'static>>,
    max_length: Option<usize>,
    input_type: InputType,
    password_visible: bool,
    mask_char: char,
    prepend: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>>,
    append: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>>,
    height: Option<Pixels>,
    pub min_rows: usize,
    text_align: gpui::TextAlign,
    on_enter: Option<Box<dyn Fn(&mut Self, &str, &mut Window, &mut Context<Self>) + 'static>>,
    on_change: Option<Box<dyn Fn(&str, &mut Context<Self>) + 'static>>,
}

impl Input {
    pub fn new(value: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        Self {
            value: value.into(),
            placeholder: SharedString::default(),
            disabled: false,
            clearable: false,
            icon_prefix: None,
            icon_suffix: None,
            focus_handle: cx.focus_handle(),
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            last_line_layouts: Vec::new(),
            last_bounds: None,
            last_layout_is_masked: false,
            cursor_visible: true,
            blink_task: None,
            filter: None,
            max_length: None,
            input_type: InputType::Text,
            password_visible: false,
            mask_char: '•',
            prepend: None,
            append: None,
            height: None,
            min_rows: 1,
            text_align: gpui::TextAlign::Left,
            on_enter: None,
            on_change: None,
        }
    }
    pub fn placeholder(mut self, p: impl Into<SharedString>) -> Self {
        self.placeholder = p.into();
        self
    }
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }
    pub fn clearable(mut self, c: bool) -> Self {
        self.clearable = c;
        self
    }
    pub fn icon_prefix(mut self, icon: IconName) -> Self {
        self.icon_prefix = Some(icon);
        self
    }
    pub fn icon_suffix(mut self, icon: IconName) -> Self {
        self.icon_suffix = Some(icon);
        self
    }
    pub fn set_icon_suffix(&mut self, icon: Option<IconName>, cx: &mut Context<Self>) {
        if self.icon_suffix == icon {
            return;
        }
        self.icon_suffix = icon;
        cx.notify();
    }
    pub fn set_clearable(&mut self, clearable: bool, cx: &mut Context<Self>) {
        if self.clearable == clearable {
            return;
        }
        self.clearable = clearable;
        cx.notify();
    }
    pub fn filter(mut self, f: impl Fn(&str) -> bool + 'static) -> Self {
        self.filter = Some(Box::new(f));
        self
    }
    pub fn max_length(mut self, max: usize) -> Self {
        self.max_length = Some(max);
        self
    }
    pub fn password(mut self) -> Self {
        self.input_type = InputType::Password;
        self
    }
    pub fn mask_char(mut self, c: char) -> Self {
        self.mask_char = c;
        self
    }
    pub fn min_rows(mut self, rows: usize) -> Self {
        self.min_rows = rows;
        self
    }
    pub fn height(mut self, h: impl Into<Pixels>) -> Self {
        self.height = Some(h.into());
        self
    }
    pub fn text_align(mut self, align: gpui::TextAlign) -> Self {
        self.text_align = align;
        self
    }
    pub fn on_enter(
        mut self,
        f: impl Fn(&mut Self, &str, &mut Window, &mut Context<Self>) + 'static,
    ) -> Self {
        self.on_enter = Some(Box::new(f));
        self
    }

    pub fn set_on_enter(
        &mut self,
        f: impl Fn(&mut Self, &str, &mut Window, &mut Context<Self>) + 'static,
        cx: &mut Context<Self>,
    ) {
        self.on_enter = Some(Box::new(f));
        cx.notify();
    }

    pub fn on_change(mut self, f: impl Fn(&str, &mut Context<Self>) + 'static) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    pub fn set_on_change(&mut self, f: impl Fn(&str, &mut Context<Self>) + 'static) {
        self.on_change = Some(Box::new(f));
    }

    pub fn clear_on_change(&mut self) {
        self.on_change = None;
    }

    fn emit_change(&mut self, cx: &mut Context<Self>) {
        if let Some(on_change) = self.on_change.take() {
            let value = self.value.to_string();
            on_change(&value, cx);
            self.on_change = Some(on_change);
        }
    }

    pub fn set_placeholder(&mut self, p: impl Into<SharedString>, cx: &mut Context<Self>) {
        let p = p.into();
        if self.placeholder == p {
            return;
        }
        self.placeholder = p;
        cx.notify();
    }

    pub fn set_disabled(&mut self, d: bool, cx: &mut Context<Self>) {
        if self.disabled == d {
            return;
        }
        self.disabled = d;
        cx.notify();
    }

    pub fn set_value(&mut self, value: impl Into<SharedString>, cx: &mut Context<Self>) {
        let value = value.into();
        if self.value == value {
            return;
        }
        self.value = value;
        self.selected_range = self.value.len()..self.value.len();
        cx.notify();
    }

    pub fn set_min_rows(&mut self, rows: usize, cx: &mut Context<Self>) {
        self.min_rows = rows;
        cx.notify();
    }

    pub fn value(&self) -> SharedString {
        self.value.clone()
    }

    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([
            KeyBinding::new("backspace", Backspace, None),
            KeyBinding::new("delete", Delete, None),
            KeyBinding::new("left", Left, None),
            KeyBinding::new("shift-left", SelectLeft, None),
            KeyBinding::new("right", Right, None),
            KeyBinding::new("shift-right", SelectRight, None),
            KeyBinding::new("home", Home, None),
            KeyBinding::new("shift-home", SelectHome, None),
            KeyBinding::new("end", End, None),
            KeyBinding::new("shift-end", SelectEnd, None),
            KeyBinding::new("cmd-a", SelectAll, None),
            KeyBinding::new("ctrl-a", SelectAll, None),
            KeyBinding::new("cmd-c", Copy, None),
            KeyBinding::new("ctrl-c", Copy, None),
            KeyBinding::new("cmd-v", Paste, None),
            KeyBinding::new("ctrl-v", Paste, None),
            KeyBinding::new("cmd-x", Cut, None),
            KeyBinding::new("ctrl-x", Cut, None),
            KeyBinding::new("enter", Enter, None),
            KeyBinding::new("up", InputUp, None),
            KeyBinding::new("shift-up", SelectUp, None),
            KeyBinding::new("down", InputDown, None),
            KeyBinding::new("shift-down", SelectDown, None),
        ]);
    }

    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.value = SharedString::default();
        self.selected_range = 0..0;
        self.emit_change(cx);
        cx.notify();
    }

    fn cursor_offset(&self) -> usize {
        if self.selection_reversed {
            self.selected_range.start
        } else {
            self.selected_range.end
        }
    }

    fn prev_char(&self, offset: usize) -> usize {
        if offset == 0 {
            return 0;
        }
        let mut p = offset - 1;
        while p > 0 && !self.value.is_char_boundary(p) {
            p -= 1;
        }
        p
    }
    fn next_char(&self, offset: usize) -> usize {
        if offset >= self.value.len() {
            return self.value.len();
        }
        let mut n = offset + 1;
        while n < self.value.len() && !self.value.is_char_boundary(n) {
            n += 1;
        }
        n
    }
    fn move_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        self.selected_range = offset..offset;
        self.reset_blink(cx);
    }
    fn select_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        if self.selection_reversed {
            self.selected_range.start = offset
        } else {
            self.selected_range.end = offset
        }
        if self.selected_range.end < self.selected_range.start {
            self.selection_reversed = !self.selection_reversed;
            self.selected_range = self.selected_range.end..self.selected_range.start;
        }
        self.reset_blink(cx);
    }

    fn backspace(&mut self, _: &Backspace, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            let p = self.prev_char(self.cursor_offset());
            if p == self.cursor_offset() {
                return;
            }
            self.select_to(p, cx);
        }
        self.internal_replace("", cx);
    }
    fn delete(&mut self, _: &Delete, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            let n = self.next_char(self.cursor_offset());
            if n == self.cursor_offset() {
                return;
            }
            self.select_to(n, cx);
        }
        self.internal_replace("", cx);
    }
    fn left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.prev_char(self.cursor_offset()), cx);
    }
    fn select_left(&mut self, _: &SelectLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.prev_char(self.cursor_offset()), cx);
    }
    fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.next_char(self.cursor_offset()), cx);
    }
    fn select_right(&mut self, _: &SelectRight, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.next_char(self.cursor_offset()), cx);
    }
    fn home(&mut self, _: &Home, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(0, cx);
    }
    fn select_home(&mut self, _: &SelectHome, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(0, cx);
    }
    fn end(&mut self, _: &End, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.value.len(), cx);
    }
    fn select_end(&mut self, _: &SelectEnd, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.value.len(), cx);
    }
    fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.selected_range = 0..self.value.len();
        self.reset_blink(cx);
    }

    fn copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty()
            && (self.input_type != InputType::Password || self.password_visible)
        {
            let selected_text = self.value[self.selected_range.clone()].to_string();
            cx.write_to_clipboard(gpui::ClipboardItem::new_string(selected_text));
        }
    }

    fn paste(&mut self, _: &Paste, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(clipboard) = cx.read_from_clipboard() {
            if let Some(text) = clipboard.text() {
                self.internal_replace(&text, cx);
            }
        }
    }

    fn cut(&mut self, _: &Cut, window: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() {
            self.copy(&Copy, window, cx);
            self.internal_replace("", cx);
        }
    }

    fn enter(&mut self, _: &Enter, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(on_enter) = self.on_enter.take() {
            let value = self.value.to_string();
            on_enter(self, &value, window, cx);
            self.on_enter = Some(on_enter);
        } else {
            self.internal_replace("\n", cx);
        }
    }

    fn up(&mut self, _: &InputUp, _: &mut Window, cx: &mut Context<Self>) {
        self.move_vertical(-1, false, cx);
    }
    fn select_up(&mut self, _: &SelectUp, _: &mut Window, cx: &mut Context<Self>) {
        self.move_vertical(-1, true, cx);
    }
    fn down(&mut self, _: &InputDown, _: &mut Window, cx: &mut Context<Self>) {
        self.move_vertical(1, false, cx);
    }
    fn select_down(&mut self, _: &SelectDown, _: &mut Window, cx: &mut Context<Self>) {
        self.move_vertical(1, true, cx);
    }

    fn toggle_password(&mut self, _: &TogglePassword, _: &mut Window, cx: &mut Context<Self>) {
        self.password_visible = !self.password_visible;
        cx.notify();
    }

    fn move_vertical(&mut self, delta: isize, select: bool, cx: &mut Context<Self>) {
        let text = self.value.clone();
        let offset = self.cursor_offset();
        let lines: Vec<&str> = text.split('\n').collect();
        let mut current_line = 0;
        let mut line_start = 0;
        for (i, line) in lines.iter().enumerate() {
            if offset >= line_start && offset <= line_start + line.len() {
                current_line = i;
                break;
            }
            line_start += line.len() + 1;
        }

        let col = offset - line_start;
        let target_line = (current_line as isize + delta).max(0) as usize;
        if target_line >= lines.len() {
            return;
        }

        let mut target_start = 0;
        for i in 0..target_line {
            target_start += lines[i].len() + 1;
        }
        let target_len = lines[target_line].len();
        let new_col = col.min(target_len);
        let new_offset = target_start + new_col;
        if select {
            self.select_to(new_offset, cx);
        } else {
            self.move_to(new_offset, cx);
        }
    }

    fn index_for_point(&self, pt: Point<Pixels>, window: &Window) -> usize {
        if let (Some(bounds), layouts) = (self.last_bounds.as_ref(), &self.last_line_layouts) {
            if layouts.is_empty() {
                return 0;
            }
            let line_height = window.line_height();
            let mut best_line = 0;
            let mut final_original_byte_offset = 0;
            let mut current_original_byte_offset = 0;
            for (i, (_layout, y_offset)) in layouts.iter().enumerate() {
                if pt.y >= *y_offset && pt.y < *y_offset + line_height {
                    best_line = i;
                    final_original_byte_offset = current_original_byte_offset;
                    break;
                }
                if pt.y >= *y_offset {
                    best_line = i;
                    final_original_byte_offset = current_original_byte_offset;
                }
                current_original_byte_offset += self
                    .value
                    .split('\n')
                    .nth(i)
                    .map(|l| l.len() + 1)
                    .unwrap_or(0);
            }
            let x = pt.x - bounds.left();
            let display_index = layouts[best_line]
                .0
                .index_for_x(x)
                .unwrap_or(layouts[best_line].0.len);

            if self.last_layout_is_masked {
                let char_count = display_index / self.mask_char.len_utf8();
                let original_line = self.value.split('\n').nth(best_line).unwrap_or("");
                let mut byte_idx = 0;
                for _ in 0..char_count {
                    if byte_idx >= original_line.len() {
                        break;
                    }
                    byte_idx += original_line[byte_idx..].chars().next().unwrap().len_utf8();
                }
                final_original_byte_offset + byte_idx
            } else {
                final_original_byte_offset + display_index
            }
        } else {
            self.value.len()
        }
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus(&self.focus_handle, cx);
        if self.value.is_empty() {
            self.move_to(0, cx);
            return;
        }
        let idx = self.index_for_point(event.position, window);
        match event.click_count {
            1 => {
                if event.modifiers.shift {
                    self.select_to(idx, cx);
                } else {
                    self.move_to(idx, cx);
                }
            }
            2 => {
                let range = self.word_range_at(idx);
                self.selected_range = range;
                self.selection_reversed = false;
                self.reset_blink(cx);
            }
            3 => {
                self.selected_range = 0..self.value.len();
                self.selection_reversed = false;
                self.reset_blink(cx);
            }
            _ => {}
        }
    }

    fn word_range_at(&self, idx: usize) -> Range<usize> {
        let text = self.value.as_ref();
        if text.is_empty() {
            return 0..0;
        }
        let idx = idx.min(text.len());
        let mut start = idx;
        while start > 0 {
            let prev = self.prev_char(start);
            let c = text[prev..start].chars().next().unwrap();
            if !c.is_alphanumeric() && c != '_' {
                break;
            }
            start = prev;
        }
        let mut end = idx;
        while end < text.len() {
            let next = self.next_char(end);
            let c = text[end..next].chars().next().unwrap();
            if !c.is_alphanumeric() && c != '_' {
                break;
            }
            end = next;
        }
        start..end
    }

    fn on_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if event.pressed_button == Some(MouseButton::Left) {
            let idx = self.index_for_point(event.position, window);
            self.select_to(idx, cx);
        }
    }

    fn start_blink(&mut self, cx: &mut Context<Self>) {
        self.cursor_visible = true;
        let executor = cx.background_executor().clone();
        self.blink_task = Some(cx.spawn(async move |this, cx| {
            loop {
                executor.timer(std::time::Duration::from_millis(500)).await;
                let res = this.update(cx, |this, cx| {
                    this.cursor_visible = !this.cursor_visible;
                    cx.notify();
                });
                if res.is_err() {
                    break;
                }
            }
        }));
    }

    fn reset_blink(&mut self, cx: &mut Context<Self>) {
        self.cursor_visible = true;
        self.start_blink(cx);
        cx.notify();
    }

    fn internal_replace(&mut self, new_text: &str, cx: &mut Context<Self>) {
        let mut v = self.value.to_string();
        let range = self.selected_range.clone();
        let potential_v = {
            let mut temp = v.clone();
            temp.replace_range(range.clone(), new_text);
            temp
        };
        if let Some(ref filter) = self.filter {
            if !filter(&potential_v) {
                return;
            }
        }
        if let Some(max) = self.max_length {
            if potential_v.chars().count() > max {
                return;
            }
        }
        v.replace_range(range, new_text);
        self.value = SharedString::from(v);
        let pos = self.selected_range.start + new_text.len();
        self.selected_range = pos..pos;
        self.emit_change(cx);
        self.reset_blink(cx);
    }

    fn is_password(&self) -> bool {
        self.input_type == InputType::Password && !self.password_visible
    }

    pub fn prepend(
        mut self,
        render: impl Fn(&mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        self.prepend = Some(Box::new(render));
        self
    }
    pub fn append(
        mut self,
        render: impl Fn(&mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        self.append = Some(Box::new(render));
        self
    }
}

impl Focusable for Input {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EntityInputHandler for Input {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        _: &mut Option<Range<usize>>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<String> {
        let start = self.offset_from_utf16(range_utf16.start);
        let end = self.offset_from_utf16(range_utf16.end);
        if start <= self.value.len() && end <= self.value.len() {
            Some(self.value[start..end].to_string())
        } else {
            None
        }
    }

    fn selected_text_range(
        &mut self,
        _: bool,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        Some(UTF16Selection {
            range: self.offset_to_utf16(self.selected_range.start)
                ..self.offset_to_utf16(self.selected_range.end),
            reversed: self.selection_reversed,
        })
    }

    fn marked_text_range(&self, _: &mut Window, _: &mut Context<Self>) -> Option<Range<usize>> {
        self.marked_range
            .as_ref()
            .map(|r| self.offset_to_utf16(r.start)..self.offset_to_utf16(r.end))
    }

    fn unmark_text(&mut self, _: &mut Window, _: &mut Context<Self>) {
        self.marked_range = None;
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range_utf16
            .map(|r| self.offset_from_utf16(r.start)..self.offset_from_utf16(r.end))
            .or_else(|| self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());
        let potential_v = {
            let mut temp = self.value.to_string();
            temp.replace_range(range.clone(), new_text);
            temp
        };
        if let Some(ref filter) = self.filter {
            if !filter(&potential_v) {
                return;
            }
        }
        if let Some(max) = self.max_length {
            if potential_v.chars().count() > max {
                return;
            }
        }
        let mut v = self.value.to_string();
        v.replace_range(range.clone(), new_text);
        self.value = SharedString::from(v);
        self.selected_range = range.start + new_text.len()..range.start + new_text.len();
        self.marked_range = None;
        self.emit_change(cx);
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected: Option<Range<usize>>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range_utf16
            .map(|r| self.offset_from_utf16(r.start)..self.offset_from_utf16(r.end))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());
        let potential_v = {
            let mut temp = self.value.to_string();
            temp.replace_range(range.clone(), new_text);
            temp
        };
        if let Some(ref filter) = self.filter {
            if !filter(&potential_v) {
                return;
            }
        }
        if let Some(max) = self.max_length {
            if potential_v.chars().count() > max {
                return;
            }
        }
        let mut v = self.value.to_string();
        v.replace_range(range.clone(), new_text);
        self.value = SharedString::from(v);
        if !new_text.is_empty() {
            self.marked_range = Some(range.start..range.start + new_text.len());
        } else {
            self.marked_range = None;
        }
        if let Some(sel) = new_selected {
            self.selected_range = range.start + sel.start..range.start + sel.end;
        } else {
            self.selected_range = range.start + new_text.len()..range.start + new_text.len();
        }
        self.emit_change(cx);
        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        bounds: Bounds<Pixels>,
        window: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let layouts = &self.last_line_layouts;
        if layouts.is_empty() {
            return None;
        }
        let start = self.offset_from_utf16(range_utf16.start);
        let end = self.offset_from_utf16(range_utf16.end);
        let line_height = window.line_height();
        let mut original_byte_offset = 0;
        for (idx, (layout, y_offset)) in layouts.iter().enumerate() {
            let line_text = self.value.split('\n').nth(idx).unwrap_or("");
            let line_len = line_text.len();
            if start >= original_byte_offset && start <= original_byte_offset + line_len {
                let x_start = layout.x_for_index(
                    self.safe_display_offset_in_line(start - original_byte_offset, line_text),
                );
                let x_end = layout.x_for_index(self.safe_display_offset_in_line(
                    end.min(original_byte_offset + line_len) - original_byte_offset,
                    line_text,
                ));
                return Some(Bounds::from_corners(
                    point(bounds.left() + x_start, *y_offset),
                    point(bounds.left() + x_end, *y_offset + line_height),
                ));
            }
            original_byte_offset += line_len + 1;
        }
        None
    }

    fn character_index_for_point(
        &mut self,
        pt: Point<Pixels>,
        window: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<usize> {
        Some(self.offset_to_utf16(self.index_for_point(pt, window)))
    }
}

impl Input {
    fn offset_to_utf16(&self, offset: usize) -> usize {
        if self.value.is_empty() {
            return 0;
        }
        self.value[..offset.min(self.value.len())]
            .chars()
            .map(|c| c.len_utf16())
            .sum()
    }
    fn offset_from_utf16(&self, target: usize) -> usize {
        let mut utf8 = 0;
        let mut utf16 = 0;
        for c in self.value.chars() {
            if utf16 >= target {
                break;
            }
            utf16 += c.len_utf16();
            utf8 += c.len_utf8();
        }
        utf8
    }
    fn text_for_display(&self) -> SharedString {
        if self.value.is_empty() {
            self.placeholder.clone()
        } else if self.is_password() {
            let masked = self
                .value
                .chars()
                .map(|c| if c == '\n' { '\n' } else { self.mask_char })
                .collect::<String>();
            SharedString::from(masked)
        } else {
            self.value.clone()
        }
    }
    fn safe_display_offset_in_line(&self, line_offset: usize, line_text: &str) -> usize {
        if !self.last_layout_is_masked {
            return line_offset;
        }
        let mut count = 0;
        let mut bytes = 0;
        for c in line_text.chars() {
            if bytes >= line_offset {
                break;
            }
            bytes += c.len_utf8();
            count += 1;
        }
        count * self.mask_char.len_utf8()
    }
}

struct InputElement {
    input: Entity<Input>,
    disabled: bool,
}

struct InputPrepaint {
    lines: Vec<(ShapedLine, Pixels)>,
    cursor: Option<gpui::PaintQuad>,
    selection: Vec<gpui::PaintQuad>,
    is_masked: bool,
    text_align: gpui::TextAlign,
}

impl IntoElement for InputElement {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for InputElement {
    type RequestLayoutState = ();
    type PrepaintState = InputPrepaint;
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
    ) -> (LayoutId, ()) {
        let input = self.input.read(cx);
        let line_count = input
            .text_for_display()
            .split('\n')
            .count()
            .max(input.min_rows) as f32;
        let mut style = Style::default();
        style.size.width = gpui::relative(1.).into();
        style.size.height = (window.line_height() * line_count).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) -> InputPrepaint {
        let input = self.input.read(cx);
        let style = window.text_style();
        let theme = &cx.global::<Config>().theme;
        let text_c = if self.disabled {
            theme.neutral.text_disabled
        } else {
            style.color
        };
        let font_size = style.font_size.to_pixels(window.rem_size());
        let line_height = window.line_height();
        let cursor_offset = input.cursor_offset();
        let text = input.text_for_display();
        let is_masked = input.is_password();
        let text_align = input.text_align;
        let text_lines: Vec<&str> = text.split('\n').collect();

        let original_cursor_line = if input.value.is_empty() {
            0
        } else {
            let mut line = 0;
            let mut start = 0;
            for l in input.value.split('\n') {
                if cursor_offset >= start && cursor_offset <= start + l.len() {
                    break;
                }
                start += l.len() + 1;
                line += 1;
            }
            line
        };

        let mut lines = Vec::new();
        let mut y = bounds.top();
        let mut cursor_quad = None;
        let mut selection_quads = Vec::new();
        let mut original_byte_offset = 0;

        for (i, line_text) in text_lines.iter().enumerate() {
            let (display, color) = if input.value.is_empty() {
                (input.placeholder.clone(), theme.neutral.text_3)
            } else {
                (SharedString::from(*line_text), text_c)
            };
            let run = TextRun {
                len: display.len(),
                font: style.font(),
                color,
                background_color: None,
                underline: None,
                strikethrough: None,
            };
            let shaped = window
                .text_system()
                .shape_line(display, font_size, &[run], None);

            let x_offset = match text_align {
                gpui::TextAlign::Left => px(0.0),
                gpui::TextAlign::Center => (bounds.size.width - shaped.width) / 2.0,
                gpui::TextAlign::Right => bounds.size.width - shaped.width,
            };

            if !input.selected_range.is_empty() && !input.value.is_empty() {
                let range = input.selected_range.clone();
                let original_line = input.value.split('\n').nth(i).unwrap_or("");
                let line_start = original_byte_offset;
                let line_end = original_byte_offset + original_line.len();
                let start = range.start.max(line_start);
                let end = range.end.min(line_end);
                if start < end {
                    let d_start =
                        input.safe_display_offset_in_line(start - line_start, original_line);
                    let d_end = input.safe_display_offset_in_line(end - line_start, original_line);
                    let x_start = shaped.x_for_index(d_start);
                    let x_end = shaped.x_for_index(d_end);
                    selection_quads.push(fill(
                        Bounds::new(
                            point(bounds.left() + x_offset + x_start, y),
                            size(x_end - x_start, line_height),
                        ),
                        theme.primary.base.opacity(0.3),
                    ));
                }
            }
            if i == original_cursor_line
                && input.selected_range.is_empty()
                && input.cursor_visible
                && !input.value.is_empty()
            {
                let original_line = input.value.split('\n').nth(i).unwrap_or("");
                let line_start = original_byte_offset;
                let col = cursor_offset - line_start;
                let d_col = if is_masked {
                    let mut count = 0;
                    let mut bytes = 0;
                    for c in original_line.chars() {
                        if bytes >= col {
                            break;
                        }
                        bytes += c.len_utf8();
                        count += 1;
                    }
                    count * input.mask_char.len_utf8()
                } else {
                    col
                };

                let x = shaped.x_for_index(d_col);
                let ch = font_size.add(px(6.0));
                let ct = y + (line_height - ch) / 2.0;
                cursor_quad = Some(fill(
                    Bounds::new(point(bounds.left() + x_offset + x, ct), size(px(2.), ch)),
                    theme.primary.base,
                ));
            } else if i == 0 && input.value.is_empty() && input.cursor_visible {
                let x = match text_align {
                    gpui::TextAlign::Left => px(0.0),
                    gpui::TextAlign::Center => (bounds.size.width - shaped.width) / 2.0,
                    gpui::TextAlign::Right => bounds.size.width - shaped.width,
                };
                let ch = font_size.add(px(6.0));
                let ct = y + (line_height - ch) / 2.0;
                cursor_quad = Some(fill(
                    Bounds::new(point(bounds.left() + x, ct), size(px(2.), ch)),
                    theme.primary.base,
                ));
            }

            lines.push((shaped, y));
            y = y + line_height;
            original_byte_offset += input
                .value
                .split('\n')
                .nth(i)
                .map(|l| l.len() + 1)
                .unwrap_or(0);
        }
        InputPrepaint {
            lines,
            cursor: cursor_quad,
            selection: selection_quads,
            is_masked,
            text_align,
        }
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        prepaint: &mut InputPrepaint,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.input.read(cx).focus_handle.clone();
        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.input.clone()),
            cx,
        );
        for s in prepaint.selection.drain(..) {
            window.paint_quad(s);
        }
        let text_align = prepaint.text_align;
        for (line, y) in &prepaint.lines {
            line.paint(
                point(bounds.left(), *y),
                window.line_height(),
                text_align,
                None,
                window,
                cx,
            )
            .unwrap();
        }
        if focus_handle.is_focused(window) {
            if let Some(c) = prepaint.cursor.take() {
                window.paint_quad(c);
            }
        }
        let line_layouts = prepaint.lines.clone();
        let is_masked = prepaint.is_masked;
        self.input.update(cx, |input, _| {
            input.last_line_layouts = line_layouts;
            input.last_bounds = Some(bounds);
            input.last_layout_is_masked = is_masked;
        });
    }
}

impl Render for Input {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focused = self.focus_handle(cx).is_focused(window);
        if focused && self.blink_task.is_none() {
            self.start_blink(cx);
        } else if !focused && self.blink_task.is_some() {
            self.blink_task = None;
        }

        let theme = cx.global::<Config>().theme.clone();
        let icon_sz = 16.0;
        let (bg, border_c) = if self.disabled {
            (theme.neutral.hover, theme.neutral.border)
        } else if focused {
            (theme.neutral.card, theme.primary.base)
        } else {
            (theme.neutral.card, theme.neutral.border)
        };
        let fh = self.focus_handle(cx);
        let line_height = window.line_height();

        let mut row = gpui::div()
            .flex()
            .flex_row()
            .when_some(self.height, |s, h| s.h(h))
            .when(self.height.is_none(), |s| {
                if self.min_rows > 1 {
                    s.h_auto()
                        .min_h(line_height * self.min_rows as f32 + px(16.0))
                } else {
                    s.min_h(px(34.0))
                }
            })
            .rounded(px(theme.radius.md))
            .bg(bg)
            .border_1()
            .border_color(border_c)
            .text_size(px(theme.font_size.md))
            .overflow_hidden();

        if self.min_rows > 1 {
            row = row.items_start();
        } else {
            row = row.items_center();
        }

        if !self.disabled {
            row = row.track_focus(&fh).cursor_text();
        } else {
            row = row.cursor_not_allowed();
        }

        if !self.disabled {
            row = row
                .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
                .on_mouse_move(cx.listener(Self::on_mouse_move))
                .on_action(cx.listener(Self::backspace))
                .on_action(cx.listener(Self::delete))
                .on_action(cx.listener(Self::left))
                .on_action(cx.listener(Self::select_left))
                .on_action(cx.listener(Self::right))
                .on_action(cx.listener(Self::select_right))
                .on_action(cx.listener(Self::home))
                .on_action(cx.listener(Self::select_home))
                .on_action(cx.listener(Self::end))
                .on_action(cx.listener(Self::select_end))
                .on_action(cx.listener(Self::select_all))
                .on_action(cx.listener(Self::copy))
                .on_action(cx.listener(Self::paste))
                .on_action(cx.listener(Self::cut))
                .on_action(cx.listener(Self::enter))
                .on_action(cx.listener(Self::up))
                .on_action(cx.listener(Self::select_up))
                .on_action(cx.listener(Self::down))
                .on_action(cx.listener(Self::select_down));
        }

        if let Some(ref p_render) = self.prepend {
            row = row.child(
                gpui::div()
                    .flex_none()
                    .h_full()
                    .bg(theme.neutral.hover)
                    .border_r_1()
                    .border_color(theme.neutral.border)
                    .flex()
                    .items_center()
                    .text_color(theme.neutral.text_3)
                    .child(p_render(window, cx)),
            );
        }

        let mut inner = gpui::div().flex_1().flex().flex_row().gap_2().px(px(12.0));
        if self.min_rows > 1 {
            inner = inner.items_start().py_2();
        } else {
            inner = inner.items_center();
        }

        if let Some(icon) = self.icon_prefix {
            inner = inner.child(Icon::new(icon).size(px(icon_sz)).color(theme.neutral.icon));
        }

        inner = inner.child(InputElement {
            input: cx.entity().clone(),
            disabled: self.disabled,
        });

        if self.clearable && !self.value.is_empty() && !self.disabled {
            inner = inner.child(
                gpui::div()
                    .flex_none()
                    .cursor_pointer()
                    .hover(|s| s.cursor_pointer())
                    .child(
                        Icon::new(IconName::X)
                            .size(px(14.0))
                            .color(theme.neutral.icon),
                    )
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(
                            move |this: &mut Self,
                                  _: &MouseDownEvent,
                                  _: &mut Window,
                                  cx: &mut Context<Self>| {
                                this.clear(cx);
                                cx.stop_propagation();
                            },
                        ),
                    ),
            );
        }

        if self.input_type == InputType::Password && !self.disabled {
            let visible = self.password_visible;
            inner = inner.child(
                gpui::div()
                    .cursor_pointer()
                    .flex_none()
                    .child(
                        Icon::new(if visible {
                            IconName::EyeOff
                        } else {
                            IconName::Eye
                        })
                        .size(px(14.0))
                        .color(theme.neutral.icon),
                    )
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(
                            move |this: &mut Self,
                                  _: &MouseDownEvent,
                                  window: &mut Window,
                                  cx: &mut Context<Self>| {
                                this.toggle_password(&TogglePassword, window, cx);
                            },
                        ),
                    ),
            );
        }

        if let Some(icon) = self.icon_suffix {
            inner = inner.child(Icon::new(icon).size(px(icon_sz)).color(theme.neutral.icon));
        }

        row = row.child(inner);

        if let Some(ref a_render) = self.append {
            row = row.child(
                gpui::div()
                    .flex_none()
                    .h_full()
                    .bg(theme.neutral.hover)
                    .border_l_1()
                    .border_color(theme.neutral.border)
                    .flex()
                    .items_center()
                    .text_color(theme.neutral.text_3)
                    .child(a_render(window, cx)),
            );
        }

        row
    }
}
