//! Selectable Text module.
//!
//! This public module implements the Liora native selectable text rendering and key bindings. It keeps the reusable
//! component logic inside `liora-components` rather than host applications so
//! downstream GPUI applications can compose the same behavior with their own
//! app state, assets, and release policy.
//!
//! ## Usage model
//!
//! Components in this module render native GPUI element trees. Stateless builder
//! values can be constructed inline, while controls with focus, selection,
//! popup, drag, or editing state should be stored as `gpui::Entity<T>` fields in
//! the parent view so state survives GPUI render passes.
//!
//! ## Design contract
//!
//! The implementation should use Liora theme tokens from `liora-core` and
//! `liora-theme`, keep accessibility-oriented keyboard/pointer behavior close to
//! the component, and avoid app-specific host-application resources in this SDK
//! crate.

use crate::gpui_compat::element_id;
use gpui::{
    App, AvailableSpace, Bounds, ClipboardItem, Context, Element, ElementId, Entity, FocusHandle,
    Focusable, GlobalElementId, InspectorElementId, IntoElement, KeyDownEvent, LayoutId,
    MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, PaintQuad, Pixels, Point, Render,
    RenderOnce, SharedString, Style, TextRun, TextStyle, ViewElement as Component, WhiteSpace,
    Window, actions, div, fill, point, prelude::*, px, relative, size,
};
use liora_core::Config;
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    ops::Range,
    rc::Rc,
    sync::{Arc, Mutex, MutexGuard, OnceLock},
};

actions!(
    selectable_text_actions,
    [
        #[doc = "Keyboard action that selects the full selectable text content."]
        SelectableTextSelectAll,
        #[doc = "Keyboard action that copies the active selectable text range."]
        SelectableTextCopy,
    ]
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Options that control selectable text wrap behavior.
pub enum SelectableTextWrap {
    /// Allows selectable text to wrap normally.
    Normal,
    /// Keeps selectable text on one line.
    NoWrap,
}

impl SelectableTextWrap {
    fn white_space(self) -> WhiteSpace {
        match self {
            Self::Normal => WhiteSpace::Normal,
            Self::NoWrap => WhiteSpace::Nowrap,
        }
    }
}

#[derive(Clone)]
/// Configuration options for selectable text options rendering and interaction behavior.
pub struct SelectableTextOptions {
    /// Stable identifier used for GPUI state, callbacks, and automation.
    pub id: ElementId,
    /// Foreground text color used in the normal state.
    pub text: SharedString,
    /// Text runs with per-range styling metadata.
    pub runs: Vec<TextRun>,
    /// Font-size token group used throughout Liora components.
    pub font_size: Pixels,
    /// Line height used for wrapped text layout.
    pub line_height: Pixels,
    /// Foreground text color for the normal state.
    pub text_color: gpui::Hsla,
    /// Wrapping policy applied to selectable text.
    pub wrap: SelectableTextWrap,
    /// Keyboard context name used for GPUI key bindings.
    pub key_context: &'static str,
    /// Whether text should occupy the full available width.
    pub fill_width: bool,
    /// Optional font family override inherited from Liora app typography.
    pub font_family: Option<SharedString>,
    /// Horizontal alignment applied to every visual line within the element width.
    pub text_align: gpui::TextAlign,
}

impl SelectableTextOptions {
    /// Creates `SelectableTextOptions` initialized from the supplied id, and text.
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            runs: Vec::new(),
            font_size: px(14.0),
            line_height: px(22.0),
            text_color: gpui::black(),
            wrap: SelectableTextWrap::Normal,
            key_context: "SelectableText",
            fill_width: true,
            font_family: None,
            text_align: gpui::TextAlign::Left,
        }
    }
}

/// Fluent native GPUI component for rendering Liora selectable text.
pub struct SelectableText;

impl SelectableText {
    /// Registers GPUI key bindings required for keyboard interaction.
    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([
            gpui::KeyBinding::new("cmd-a", SelectableTextSelectAll, Some("SelectableText")),
            gpui::KeyBinding::new("ctrl-a", SelectableTextSelectAll, Some("SelectableText")),
            gpui::KeyBinding::new("cmd-c", SelectableTextCopy, Some("SelectableText")),
            gpui::KeyBinding::new("ctrl-c", SelectableTextCopy, Some("SelectableText")),
        ]);
    }

    /// Performs the view operation used by this component.
    pub fn view(
        options: SelectableTextOptions,
        window: &mut Window,
        cx: &mut App,
    ) -> gpui::AnyElement {
        Self::view_with_pointer_handler(options, None, window, cx)
    }

    pub(crate) fn view_with_pointer_handler(
        options: SelectableTextOptions,
        pointer_handler: Option<Rc<dyn Fn(usize, &mut Window, &mut App)>>,
        window: &mut Window,
        cx: &mut App,
    ) -> gpui::AnyElement {
        let input = window.use_keyed_state(options.id.clone(), cx, {
            let initial = options.clone();
            move |_, cx| SelectableTextState::new(cx, initial)
        });
        input.update(cx, |state, cx| {
            state.update_options(options, cx);
            state.scope = None;
            state.pointer_handler = pointer_handler;
        });
        SelectableTextView { input }.into_any_element()
    }

    /// Renders a leaf in a window-local selection scope, preserving its own text layout.
    pub(crate) fn view_in_scope(
        options: SelectableTextOptions,
        scope: SharedString,
        order: usize,
        separator: SharedString,
        window: &mut Window,
        cx: &mut App,
    ) -> gpui::AnyElement {
        let shared = window.use_keyed_state(
            element_id(format!("selectable-scope:{scope}")),
            cx,
            |_, _| ScopedSelection::default(),
        );
        let input = window.use_keyed_state(options.id.clone(), cx, {
            let initial = options.clone();
            move |_, cx| SelectableTextState::new(cx, initial)
        });
        input.update(cx, |state, cx| {
            state.scope = Some((shared.clone(), order));
            state.update_options(options, cx);
        });
        let leaf = input.read(cx);
        let text = leaf.text.clone();
        let geometry = leaf.scoped_geometry.clone();
        shared.update(cx, |state, cx| {
            state.members.retain(|&member_order, member| {
                member.input.upgrade().is_some_and(|input| {
                    input
                        .read(cx)
                        .scope
                        .as_ref()
                        .is_some_and(|(member_scope, order)| {
                            member_scope == &shared && *order == member_order
                        })
                })
            });
            if state
                .anchor
                .is_some_and(|pos| !state.members.contains_key(&pos.order))
                || state
                    .head
                    .is_some_and(|pos| !state.members.contains_key(&pos.order))
            {
                state.anchor = None;
                state.head = None;
                state.selecting = false;
            }
            state.members.insert(
                order,
                ScopedMember {
                    input: input.downgrade(),
                    text,
                    separator,
                    geometry,
                },
            );
            for position in [&mut state.anchor, &mut state.head] {
                if let Some(position) = position.as_mut().filter(|position| position.order == order)
                {
                    let text = &state.members[&order].text;
                    position.offset = position.offset.min(text.len());
                    while position.offset > 0 && !text.is_char_boundary(position.offset) {
                        position.offset -= 1;
                    }
                }
            }
        });
        SelectableTextView { input }.into_any_element()
    }
}

struct SelectableTextView {
    input: Entity<SelectableTextState>,
}

impl IntoElement for SelectableTextView {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl RenderOnce for SelectableTextView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.input.into_any_element()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ScopedPosition {
    order: usize,
    offset: usize,
}

struct ScopedMember {
    input: gpui::WeakEntity<SelectableTextState>,
    text: SharedString,
    separator: SharedString,
    geometry: Rc<RefCell<SelectableTextSelectionState>>,
}

#[derive(Default)]
struct ScopedSelection {
    members: BTreeMap<usize, ScopedMember>,
    anchor: Option<ScopedPosition>,
    head: Option<ScopedPosition>,
    selecting: bool,
}

impl ScopedSelection {
    fn endpoints(&self) -> Option<(ScopedPosition, ScopedPosition)> {
        let (anchor, head) = (self.anchor?, self.head?);
        self.members.get(&anchor.order)?.input.upgrade()?;
        self.members.get(&head.order)?.input.upgrade()?;
        Some((anchor, head))
    }

    fn range_for(&self, order: usize) -> Range<usize> {
        let (Some((anchor, head)), Some(member)) = (self.endpoints(), self.members.get(&order))
        else {
            return 0..0;
        };
        scoped_leaf_range(order, member.text.len(), anchor, head)
    }

    fn point_at(
        &self,
        point: Point<Pixels>,
        active: ScopedPosition,
        cx: &App,
    ) -> Option<ScopedPosition> {
        let mut nearest = None;
        for (&order, member) in &self.members {
            if member.input.upgrade().is_none() {
                continue;
            }
            let geometry = member.geometry.borrow();
            let Some(bounds) = geometry.bounds else {
                continue;
            };
            let dx =
                (bounds.left() - point.x).max(px(0.0)) + (point.x - bounds.right()).max(px(0.0));
            let dy =
                (bounds.top() - point.y).max(px(0.0)) + (point.y - bounds.bottom()).max(px(0.0));
            let distance = dx + dy;
            if nearest.as_ref().is_none_or(|(best, _)| distance < *best) {
                nearest = Some((distance, order));
            }
        }
        let (_, order) = nearest?;
        if order == active.order {
            return Some(active);
        }
        let member = self.members.get(&order)?;
        let input = member.input.upgrade()?;
        let offset = input
            .read(cx)
            .index_for_point_in(&member.geometry.borrow(), point);
        Some(ScopedPosition { order, offset })
    }

    fn selected_text(&self) -> String {
        let Some((anchor, head)) = self.endpoints() else {
            return String::new();
        };
        joined_scoped_text(
            self.members
                .iter()
                .filter(|(_, member)| member.input.upgrade().is_some())
                .map(|(&order, member)| (order, member.text.as_ref(), member.separator.as_ref())),
            anchor,
            head,
        )
    }

    fn select_all(&mut self) {
        let Some((&first, _)) = self
            .members
            .iter()
            .find(|(_, member)| member.input.upgrade().is_some())
        else {
            return;
        };
        let Some((&last, member)) = self
            .members
            .iter()
            .rev()
            .find(|(_, member)| member.input.upgrade().is_some())
        else {
            return;
        };
        self.anchor = Some(ScopedPosition {
            order: first,
            offset: 0,
        });
        self.head = Some(ScopedPosition {
            order: last,
            offset: member.text.len(),
        });
        self.selecting = false;
    }
}

fn scoped_leaf_range(
    order: usize,
    len: usize,
    anchor: ScopedPosition,
    head: ScopedPosition,
) -> Range<usize> {
    let (start, end) = if anchor <= head {
        (anchor, head)
    } else {
        (head, anchor)
    };
    if order < start.order || order > end.order {
        return 0..0;
    }
    (if order == start.order {
        start.offset.min(len)
    } else {
        0
    })..(if order == end.order {
        end.offset.min(len)
    } else {
        len
    })
}

fn joined_scoped_text<'a>(
    members: impl IntoIterator<Item = (usize, &'a str, &'a str)>,
    anchor: ScopedPosition,
    head: ScopedPosition,
) -> String {
    if anchor == head {
        return String::new();
    }
    let (start, end) = if anchor <= head {
        (anchor, head)
    } else {
        (head, anchor)
    };
    let mut output = String::new();
    let mut separator = None;
    for (order, text, next_separator) in members {
        if order < start.order || order > end.order {
            continue;
        }
        if let Some(separator) = separator {
            output.push_str(separator);
        }
        output.push_str(&text[scoped_leaf_range(order, text.len(), anchor, head)]);
        separator = Some(next_separator);
    }
    output
}

struct SelectableTextSelectionState {
    selected_range: Range<usize>,
    selection_reversed: bool,
    selecting: bool,
    layout: Option<Arc<SelectableTextLayout>>,
    line_starts: Vec<(Pixels, usize)>,
    bounds: Option<Bounds<Pixels>>,
}

impl Default for SelectableTextSelectionState {
    fn default() -> Self {
        Self {
            selected_range: 0..0,
            selection_reversed: false,
            selecting: false,
            layout: None,
            line_starts: Vec::new(),
            bounds: None,
        }
    }
}

type SelectableTextLayoutCacheStore = Rc<RefCell<Vec<SelectableTextLayoutCache>>>;

struct SelectableTextLayoutCache {
    text: SharedString,
    runs: Vec<TextRun>,
    font_size: Pixels,
    line_height: Pixels,
    wrap_width: Option<Pixels>,
    layout: Arc<SelectableTextLayout>,
}

impl SelectableTextLayoutCache {
    fn matches(
        &self,
        text: &SharedString,
        runs: &[TextRun],
        font_size: Pixels,
        line_height: Pixels,
        wrap_width: Option<Pixels>,
    ) -> bool {
        self.text == *text
            && self.runs == runs
            && self.font_size == font_size
            && self.line_height == line_height
            && self.wrap_width == wrap_width
    }
}

fn selection_state_map() -> &'static Mutex<HashMap<String, SelectableTextSelectionState>> {
    static STATES: OnceLock<Mutex<HashMap<String, SelectableTextSelectionState>>> = OnceLock::new();
    STATES.get_or_init(|| Mutex::new(HashMap::new()))
}

fn lock_selection_state_map() -> MutexGuard<'static, HashMap<String, SelectableTextSelectionState>>
{
    selection_state_map()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn selection_key(id: &ElementId) -> String {
    format!("{id:?}")
}

fn with_selection_state<R>(
    id: &ElementId,
    f: impl FnOnce(&mut SelectableTextSelectionState) -> R,
) -> R {
    let mut states = lock_selection_state_map();
    f(states.entry(selection_key(id)).or_default())
}

fn selected_range_snapshot(id: &ElementId) -> Range<usize> {
    lock_selection_state_map()
        .get(&selection_key(id))
        .map(|state| state.selected_range.clone())
        .unwrap_or(0..0)
}

fn clear_other_selection_states(active_id: &ElementId) -> bool {
    let active_key = selection_key(active_id);
    clear_selection_states(|key| key != active_key)
}

fn clear_selection_state_for(id: &ElementId) -> bool {
    let target_key = selection_key(id);
    clear_selection_states(|key| key == target_key)
}

fn clear_selection_states(mut should_clear: impl FnMut(&str) -> bool) -> bool {
    let mut changed = false;
    let mut states = lock_selection_state_map();
    for (key, state) in states.iter_mut() {
        if should_clear(key.as_str()) && (!state.selected_range.is_empty() || state.selecting) {
            state.selected_range = 0..0;
            state.selection_reversed = false;
            state.selecting = false;
            changed = true;
        }
    }
    changed
}

fn set_layout_state(
    id: &ElementId,
    layout: Arc<SelectableTextLayout>,
    line_starts: Vec<(Pixels, usize)>,
    bounds: Bounds<Pixels>,
) {
    with_selection_state(id, |state| {
        state.layout = Some(layout);
        state.line_starts = line_starts;
        state.bounds = Some(bounds);
    });
}

struct SelectableTextState {
    id: ElementId,
    text: SharedString,
    runs: Vec<TextRun>,
    font_size: Pixels,
    line_height: Pixels,
    text_color: gpui::Hsla,
    wrap: SelectableTextWrap,
    key_context: &'static str,
    fill_width: bool,
    font_family: Option<SharedString>,
    text_align: gpui::TextAlign,
    layout_caches: SelectableTextLayoutCacheStore,
    focus_handle: FocusHandle,
    scope: Option<(Entity<ScopedSelection>, usize)>,
    scoped_geometry: Rc<RefCell<SelectableTextSelectionState>>,
    pointer_handler: Option<Rc<dyn Fn(usize, &mut Window, &mut App)>>,
}

impl SelectableTextState {
    fn new(cx: &mut Context<Self>, options: SelectableTextOptions) -> Self {
        Self {
            pointer_handler: None,
            id: options.id,
            runs: normalize_runs(options.runs, options.text.len(), options.text_color),
            text: options.text,
            font_size: options.font_size,
            line_height: options.line_height,
            text_color: options.text_color,
            wrap: options.wrap,
            key_context: options.key_context,
            fill_width: options.fill_width,
            font_family: options.font_family,
            text_align: options.text_align,
            layout_caches: Rc::new(RefCell::new(Vec::new())),
            focus_handle: cx.focus_handle(),
            scope: None,
            scoped_geometry: Rc::new(RefCell::new(SelectableTextSelectionState::default())),
        }
    }

    fn update_options(&mut self, options: SelectableTextOptions, cx: &mut Context<Self>) {
        let runs = normalize_runs(options.runs, options.text.len(), options.text_color);
        let changed = self.id != options.id
            || self.text != options.text
            || self.runs != runs
            || self.font_size != options.font_size
            || self.line_height != options.line_height
            || self.text_color != options.text_color
            || self.wrap != options.wrap
            || self.key_context != options.key_context
            || self.fill_width != options.fill_width
            || self.font_family != options.font_family
            || self.text_align != options.text_align;
        if !changed {
            return;
        }

        let old_id = self.id.clone();
        self.id = options.id;
        self.text = options.text;
        self.runs = runs;
        self.font_size = options.font_size;
        self.line_height = options.line_height;
        self.text_color = options.text_color;
        self.wrap = options.wrap;
        self.key_context = options.key_context;
        self.fill_width = options.fill_width;
        self.font_family = options.font_family;
        self.text_align = options.text_align;
        self.layout_caches.borrow_mut().clear();

        if self.scope.is_none() {
            if old_id != self.id {
                let old_range = selected_range_snapshot(&old_id);
                with_selection_state(&self.id, |state| state.selected_range = old_range);
            }
            with_selection_state(&self.id, |state| {
                state.selected_range.start = self.clamp_boundary(state.selected_range.start);
                state.selected_range.end = self.clamp_boundary(state.selected_range.end);
                if state.selected_range.end < state.selected_range.start {
                    state.selected_range = state.selected_range.end..state.selected_range.start;
                    state.selection_reversed = !state.selection_reversed;
                }
            });
        }
        cx.notify();
    }

    fn text_style(&self, window: &Window) -> TextStyle {
        let mut style = window.text_style();
        if let Some(family) = self.font_family.clone() {
            style.font_family = family;
        }
        style.color = self.text_color;
        style.font_size = self.font_size.into();
        style.line_height = self.line_height.into();
        style.white_space = self.wrap.white_space();
        style.text_overflow = None;
        style.line_clamp = None;
        style.text_align = self.text_align;
        style
    }

    fn move_to(&self, state: &mut SelectableTextSelectionState, offset: usize) -> bool {
        let offset = self.clamp_boundary(offset);
        if state.selected_range == (offset..offset) && !state.selection_reversed {
            return false;
        }
        state.selected_range = offset..offset;
        state.selection_reversed = false;
        true
    }

    fn select_to(&self, state: &mut SelectableTextSelectionState, offset: usize) -> bool {
        let offset = self.clamp_boundary(offset);
        let previous_range = state.selected_range.clone();
        let previous_reversed = state.selection_reversed;
        if state.selection_reversed {
            state.selected_range.start = offset;
        } else {
            state.selected_range.end = offset;
        }
        if state.selected_range.end < state.selected_range.start {
            state.selection_reversed = !state.selection_reversed;
            state.selected_range = state.selected_range.end..state.selected_range.start;
        }
        state.selected_range != previous_range || state.selection_reversed != previous_reversed
    }

    fn clamp_boundary(&self, mut offset: usize) -> usize {
        offset = offset.min(self.text.len());
        while offset > 0 && !self.text.is_char_boundary(offset) {
            offset -= 1;
        }
        offset
    }

    fn index_for_point(&self, pt: Point<Pixels>) -> usize {
        if self.scope.is_some() {
            return self.index_for_point_in(&self.scoped_geometry.borrow(), pt);
        }
        let states = lock_selection_state_map();
        let Some(state) = states.get(&selection_key(&self.id)) else {
            return self.text.len();
        };
        self.index_for_point_in(state, pt)
    }

    fn index_for_point_in(&self, state: &SelectableTextSelectionState, pt: Point<Pixels>) -> usize {
        let Some(bounds) = state.bounds.as_ref() else {
            return self.text.len();
        };
        let Some(layout) = state.layout.as_ref() else {
            return self.text.len();
        };
        if layout.lines.is_empty() {
            return self.text.len();
        }

        let mut chosen = 0;
        for (ix, line) in layout.lines.iter().enumerate() {
            let y = state
                .line_starts
                .get(ix)
                .map(|(y, _)| *y)
                .unwrap_or(bounds.top());
            let line_bottom = y + line.size(self.line_height).height;
            if pt.y <= line_bottom {
                chosen = ix;
                break;
            }
            if pt.y >= y {
                chosen = ix;
            }
        }

        let line = &layout.lines[chosen];
        let (y, start) = state
            .line_starts
            .get(chosen)
            .copied()
            .unwrap_or((bounds.top(), 0));
        let line_size = line.size(self.line_height);
        let local_y = (pt.y - y)
            .max(px(0.0))
            .min((line_size.height - px(0.5)).max(px(0.0)));
        let visual_row =
            ((local_y / self.line_height).floor() as usize).min(line.wrap_boundaries().len());
        let local_x = (pt.x
            - bounds.left()
            - visual_row_alignment_offset(line, visual_row, bounds.size.width, self.text_align))
        .max(px(0.0));
        let position = point(
            local_x,
            self.line_height * visual_row as f32 + self.line_height * 0.5,
        );
        let line_index = line
            .closest_index_for_position(position, self.line_height)
            .unwrap_or_else(|idx| idx);
        self.clamp_boundary(start + line_index)
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(handler) = &self.pointer_handler {
            handler(self.index_for_point(event.position), window, cx);
            return;
        }
        if let Some((scope, order)) = &self.scope {
            window.focus(&self.focus_handle, cx);
            let idx = self.index_for_point(event.position);
            let position = ScopedPosition {
                order: *order,
                offset: idx,
            };
            let word = (event.click_count == 2).then(|| self.word_range_at(idx));
            scope.update(cx, |state, _| {
                if event.modifiers.shift && state.anchor.is_some() {
                    state.head = Some(position);
                } else if event.click_count >= 3 {
                    state.anchor = Some(ScopedPosition {
                        order: *order,
                        offset: 0,
                    });
                    state.head = Some(ScopedPosition {
                        order: *order,
                        offset: self.text.len(),
                    });
                } else if let Some(range) = word {
                    state.anchor = Some(ScopedPosition {
                        order: *order,
                        offset: range.start,
                    });
                    state.head = Some(ScopedPosition {
                        order: *order,
                        offset: range.end,
                    });
                } else {
                    state.anchor = Some(position);
                    state.head = Some(position);
                }
                state.selecting = true;
            });
            window.refresh();
            return;
        }
        window.focus(&self.focus_handle, cx);
        let cleared_other_selection = clear_other_selection_states(&self.id);
        let idx = self.index_for_point(event.position);
        let changed = with_selection_state(&self.id, |state| {
            let was_selecting = state.selecting;
            state.selecting = true;
            if event.modifiers.shift {
                self.select_to(state, idx) || !was_selecting
            } else if event.click_count >= 3 {
                let changed = state.selected_range != (0..self.text.len())
                    || state.selection_reversed
                    || !was_selecting;
                state.selected_range = 0..self.text.len();
                state.selection_reversed = false;
                changed
            } else if event.click_count == 2 {
                let range = self.word_range_at(idx);
                let changed =
                    state.selected_range != range || state.selection_reversed || !was_selecting;
                state.selected_range = range;
                state.selection_reversed = false;
                changed
            } else {
                self.move_to(state, idx) || !was_selecting
            }
        });
        if cleared_other_selection {
            window.refresh();
        }
        if changed || cleared_other_selection {
            window.refresh();
        }
    }

    fn on_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some((scope, order)) = &self.scope {
            if !scope.read(cx).selecting {
                return;
            }
            let active = ScopedPosition {
                order: *order,
                offset: self.index_for_point(event.position),
            };
            let changed = scope.update(cx, |state, cx| {
                if !state.selecting || state.anchor.is_none_or(|anchor| anchor.order != *order) {
                    return false;
                }
                if event.pressed_button != Some(MouseButton::Left) {
                    state.selecting = false;
                    return false;
                }
                let next = state.point_at(event.position, active, cx);
                let changed = next.is_some() && next != state.head;
                if changed {
                    state.head = next;
                }
                changed
            });
            if changed {
                window.refresh();
            }
            return;
        }
        let dragging = event.pressed_button == Some(MouseButton::Left);
        let idx = dragging.then(|| self.index_for_point(event.position));
        let changed = with_selection_state(&self.id, |state| {
            if !dragging {
                let changed = state.selecting;
                state.selecting = false;
                changed
            } else if state.selecting {
                self.select_to(state, idx.unwrap_or(self.text.len()))
            } else {
                false
            }
        });
        if changed {
            // Selection is stored outside the entity so a drag only needs a
            // paint refresh; invalidating the whole entity tree here makes
            // fast mouse moves visibly lag behind the pointer.
            window.refresh();
        }
    }

    fn is_selecting(&self, cx: &App) -> bool {
        if let Some((scope, order)) = &self.scope {
            let state = scope.read(cx);
            return state.selecting && state.anchor.is_some_and(|anchor| anchor.order == *order);
        }
        lock_selection_state_map()
            .get(&selection_key(&self.id))
            .is_some_and(|state| state.selecting)
    }

    fn on_mouse_up(&mut self, event: &MouseUpEvent, window: &mut Window, cx: &mut Context<Self>) {
        if let Some((scope, order)) = &self.scope {
            if !scope.read(cx).selecting {
                return;
            }
            let active = ScopedPosition {
                order: *order,
                offset: self.index_for_point(event.position),
            };
            let changed = scope.update(cx, |state, cx| {
                if !state.selecting || state.anchor.is_none_or(|anchor| anchor.order != *order) {
                    return false;
                }
                let next = state.point_at(event.position, active, cx);
                let changed = next.is_some() && next != state.head;
                if changed {
                    state.head = next;
                }
                state.selecting = false;
                changed
            });
            if changed {
                window.refresh();
            }
            return;
        }
        let changed = with_selection_state(&self.id, |state| {
            let changed = state.selecting;
            state.selecting = false;
            changed
        });
        if changed {
            cx.notify();
        }
    }

    fn clear_selection(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some((scope, _)) = &self.scope {
            let current = cx.entity();
            if scope.read(cx).members.values().any(|member| {
                member.input.upgrade().is_some_and(|input| {
                    input != current && input.read(cx).focus_handle.is_focused(window)
                })
            }) {
                return;
            }
            scope.update(cx, |state, _| {
                state.anchor = None;
                state.head = None;
                state.selecting = false;
            });
            window.refresh();
            return;
        }
        let changed = with_selection_state(&self.id, |state| {
            let changed = !state.selected_range.is_empty() || state.selecting;
            state.selected_range = 0..0;
            state.selection_reversed = false;
            state.selecting = false;
            changed
        });
        if changed {
            cx.notify();
        }
    }

    fn set_select_all(&mut self, cx: &mut Context<Self>) {
        if let Some((scope, _)) = &self.scope {
            scope.update(cx, |state, _| state.select_all());
            cx.notify();
            return;
        }
        let changed = with_selection_state(&self.id, |state| {
            let changed = state.selected_range != (0..self.text.len())
                || state.selection_reversed
                || state.selecting;
            state.selected_range = 0..self.text.len();
            state.selection_reversed = false;
            state.selecting = false;
            changed
        });
        if changed {
            cx.notify();
        }
    }

    fn select_all(&mut self, _: &SelectableTextSelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.set_select_all(cx);
    }

    fn on_key_down(&mut self, event: &KeyDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        if event.keystroke.key.eq_ignore_ascii_case("a")
            && (event.keystroke.modifiers.control || event.keystroke.modifiers.platform)
            && !event.keystroke.modifiers.alt
            && !event.keystroke.modifiers.shift
            && !event.keystroke.modifiers.function
        {
            self.set_select_all(cx);
            cx.stop_propagation();
        }
    }

    fn copy(&mut self, _: &SelectableTextCopy, _: &mut Window, cx: &mut Context<Self>) {
        if let Some((scope, _)) = &self.scope {
            let text = scope.read(cx).selected_text();
            if !text.is_empty() {
                cx.write_to_clipboard(ClipboardItem::new_string(text));
            }
            return;
        }
        let selected_range = selected_range_snapshot(&self.id);
        if !selected_range.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.text[selected_range].to_string(),
            ));
        }
    }

    fn word_range_at(&self, idx: usize) -> Range<usize> {
        let text = self.text.as_ref();
        if text.is_empty() {
            return 0..0;
        }
        let idx = self.clamp_boundary(idx);
        let mut start = idx;
        while start > 0 {
            let prev = self.prev_char(start);
            let ch = text[prev..start].chars().next().unwrap_or(' ');
            if !is_word_char(ch) {
                break;
            }
            start = prev;
        }
        let mut end = idx;
        while end < text.len() {
            let next = self.next_char(end);
            let ch = text[end..next].chars().next().unwrap_or(' ');
            if !is_word_char(ch) {
                break;
            }
            end = next;
        }
        start..end
    }

    fn prev_char(&self, offset: usize) -> usize {
        if offset == 0 {
            return 0;
        }
        let mut prev = offset - 1;
        while prev > 0 && !self.text.is_char_boundary(prev) {
            prev -= 1;
        }
        prev
    }

    fn next_char(&self, offset: usize) -> usize {
        if offset >= self.text.len() {
            return self.text.len();
        }
        let mut next = offset + 1;
        while next < self.text.len() && !self.text.is_char_boundary(next) {
            next += 1;
        }
        next
    }
}

impl Focusable for SelectableTextState {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for SelectableTextState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        cx.on_blur(&self.focus_handle, window, |this, window, cx| {
            this.clear_selection(window, cx);
        })
        .detach();

        let mut wrapper = div()
            .id(element_id(format!("{:?}-selectable", self.id)))
            .key_context(self.key_context)
            .track_focus(&self.focus_handle(cx))
            .cursor_text()
            .min_w(px(0.0));
        if self.fill_width {
            wrapper = wrapper.w_full().flex_shrink(1.0);
        }

        wrapper
            .on_key_down(cx.listener(Self::on_key_down))
            .on_action(cx.listener(Self::select_all))
            .on_action(cx.listener(Self::copy))
            .child(SelectableTextElement {
                id: element_id(format!("{:?}-text", self.id)),
                input: cx.entity(),
                layout_caches: self.layout_caches.clone(),
            })
    }
}

struct SelectableTextLayout {
    lines: Vec<gpui::WrappedLine>,
    line_offsets: Vec<usize>,
    width: Pixels,
    height: Pixels,
}

struct SelectableTextElement {
    id: ElementId,
    input: Entity<SelectableTextState>,
    layout_caches: SelectableTextLayoutCacheStore,
}

struct SelectableTextPrepaint {
    layout: Arc<SelectableTextLayout>,
    selection: Vec<PaintQuad>,
    hitbox: gpui::Hitbox,
}

impl Element for SelectableTextElement {
    type RequestLayoutState = Arc<SelectableTextLayout>;
    type PrepaintState = SelectableTextPrepaint;

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
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
    ) -> (LayoutId, Arc<SelectableTextLayout>) {
        let layout_caches = self.layout_caches.clone();
        let (text, runs, font_size, line_height, wrap, fill_width) = {
            let input = self.input.read(cx);
            (
                input.text.clone(),
                input.runs.clone(),
                input.font_size,
                input.line_height,
                input.wrap,
                input.fill_width,
            )
        };

        let mut style = Style::default();
        if fill_width {
            style.size.width = relative(1.).into();
            style.min_size.width = px(0.0).into();
        }

        let text_for_measure = text.clone();
        let runs_for_measure = runs.clone();
        let layout_id =
            window.request_measured_layout(style, move |known, available, window, _cx| {
                let wrap_width = selectable_wrap_width(wrap, known.width, available.width);
                let layout = cached_selectable_layout(
                    &layout_caches,
                    text_for_measure.clone(),
                    font_size,
                    line_height,
                    runs_for_measure.clone(),
                    wrap_width,
                    window,
                );
                size(
                    if fill_width {
                        known
                            .width
                            .or(wrap_width)
                            .unwrap_or(layout.width)
                            .max(px(0.0))
                    } else {
                        layout.width
                    },
                    layout.height,
                )
            });

        let layout = cached_selectable_layout(
            &self.layout_caches,
            text,
            font_size,
            line_height,
            runs,
            None,
            window,
        );
        (layout_id, layout)
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        layout: &mut Arc<SelectableTextLayout>,
        window: &mut Window,
        cx: &mut App,
    ) -> SelectableTextPrepaint {
        let wrap = self.input.read(cx).wrap;
        let actual_wrap_width =
            (wrap == SelectableTextWrap::Normal).then_some(bounds.size.width.max(px(1.0)));
        let (text, runs, font_size, line_height) = {
            let input = self.input.read(cx);
            (
                input.text.clone(),
                input.runs.clone(),
                input.font_size,
                input.line_height,
            )
        };
        let actual_layout = cached_selectable_layout(
            &self.layout_caches,
            text,
            font_size,
            line_height,
            runs,
            actual_wrap_width,
            window,
        );
        *layout = actual_layout;
        let input = self.input.read(cx);
        let mut selection_quads = Vec::new();
        let selected_range = if let Some((scope, order)) = &input.scope {
            scope.read(cx).range_for(*order)
        } else {
            selected_range_snapshot(&input.id)
        };
        let mut y = bounds.top();
        let mut line_starts = Vec::new();
        let selection_color = cx.global::<Config>().theme.primary.base.opacity(0.28);

        for (line, line_start) in layout.lines.iter().zip(layout.line_offsets.iter().copied()) {
            if !selected_range.is_empty() {
                let line_end = line_start + line.len();
                let start = selected_range.start.max(line_start);
                let end = selected_range.end.min(line_end);
                if start < end {
                    add_wrapped_selection_quads(
                        line,
                        start - line_start,
                        end - line_start,
                        y,
                        input.line_height,
                        bounds,
                        input.text_align,
                        selection_color,
                        &mut selection_quads,
                    );
                }
            }
            line_starts.push((y, line_start));
            y += line.size(input.line_height).height;
        }

        let hitbox = window.insert_hitbox(bounds, gpui::HitboxBehavior::Normal);
        if input.scope.is_some() {
            let mut geometry = input.scoped_geometry.borrow_mut();
            geometry.layout = Some(layout.clone());
            geometry.line_starts = line_starts;
            geometry.bounds = Some(bounds);
        } else {
            set_layout_state(&input.id, layout.clone(), line_starts, bounds);
        }

        SelectableTextPrepaint {
            layout: layout.clone(),
            selection: selection_quads,
            hitbox,
        }
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Arc<SelectableTextLayout>,
        prepaint: &mut SelectableTextPrepaint,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.input.read(cx).focus_handle.clone();
        window.set_cursor_style(gpui::CursorStyle::IBeam, &prepaint.hitbox);

        let input = self.input.clone();
        let focus_handle_for_down = focus_handle.clone();
        let hitbox = prepaint.hitbox.clone();
        window.on_mouse_event(move |event: &MouseDownEvent, phase, window, cx| {
            if phase.capture()
                && event.button == MouseButton::Left
                && !bounds.contains(&event.position)
            {
                let leaf = input.read(cx);
                if let Some((scope, _)) = leaf.scope.clone() {
                    let outside = scope.read(cx).members.values().all(|member| {
                        member.input.upgrade().is_none_or(|_| {
                            !member
                                .geometry
                                .borrow()
                                .bounds
                                .is_some_and(|bounds| bounds.contains(&event.position))
                        })
                    });
                    if outside {
                        scope.update(cx, |state, _| {
                            state.anchor = None;
                            state.head = None;
                            state.selecting = false;
                        });
                        window.refresh();
                    }
                } else {
                    let input_id = leaf.id.clone();
                    if clear_selection_state_for(&input_id) {
                        window.refresh();
                    }
                }
            }
            if phase.bubble() && event.button == MouseButton::Left && hitbox.is_hovered(window) {
                window.focus(&focus_handle_for_down, cx);
                window.capture_pointer(hitbox.id);
                input.update(cx, |input, cx| input.on_mouse_down(event, window, cx));
                cx.stop_propagation();
            }
        });

        let input = self.input.clone();
        let hitbox = prepaint.hitbox.clone();
        window.on_mouse_event(move |event: &MouseMoveEvent, phase, event_window, cx| {
            let selecting = input.read(cx).is_selecting(cx);
            if phase.capture() && (hitbox.is_hovered(event_window) || selecting) {
                input.update(cx, |input, cx| input.on_mouse_move(event, event_window, cx));
            }
        });

        let input = self.input.clone();
        let hitbox = prepaint.hitbox.clone();
        window.on_mouse_event(move |event: &MouseUpEvent, phase, window, cx| {
            let selecting = input.read(cx).is_selecting(cx);
            if phase.capture()
                && event.button == MouseButton::Left
                && (hitbox.is_hovered(window) || selecting)
            {
                input.update(cx, |input, cx| input.on_mouse_up(event, window, cx));
            }
        });

        let (line_height, text_align) = {
            let input = self.input.read(cx);
            (input.line_height, input.text_style(window).text_align)
        };

        let mut origin = bounds.origin;
        for line in &prepaint.layout.lines {
            let _ =
                line.paint_background(origin, line_height, text_align, Some(bounds), window, cx);
            origin.y += line.size(line_height).height;
        }

        for selection in prepaint.selection.drain(..) {
            window.paint_quad(selection);
        }

        let mut origin = bounds.origin;
        for line in &prepaint.layout.lines {
            let _ = line.paint(origin, line_height, text_align, Some(bounds), window, cx);
            origin.y += line.size(line_height).height;
        }
    }
}

fn shaped_line_start_offsets<'a>(
    source_text: &str,
    shaped_lines: impl IntoIterator<Item = &'a str>,
) -> Vec<usize> {
    let mut starts = Vec::new();
    let mut cursor = 0usize;

    for line_text in shaped_lines {
        let relative_start = if line_text.is_empty() {
            Some(0)
        } else {
            source_text
                .get(cursor..)
                .and_then(|remaining| remaining.find(line_text))
        };
        let start = relative_start
            .map(|relative| cursor + relative)
            .unwrap_or(cursor)
            .min(source_text.len());
        starts.push(start);
        cursor = (start + line_text.len()).min(source_text.len());
        cursor = consume_line_ending(source_text, cursor);
    }

    starts
}

fn consume_line_ending(source_text: &str, cursor: usize) -> usize {
    let Some(remaining) = source_text.get(cursor..) else {
        return source_text.len();
    };
    if remaining.starts_with("\r\n") {
        cursor + 2
    } else if remaining.starts_with('\n') || remaining.starts_with('\r') {
        cursor + 1
    } else {
        cursor
    }
}

fn cached_selectable_layout(
    layout_caches: &SelectableTextLayoutCacheStore,
    text: SharedString,
    font_size: Pixels,
    line_height: Pixels,
    runs: Vec<TextRun>,
    wrap_width: Option<Pixels>,
    window: &mut Window,
) -> Arc<SelectableTextLayout> {
    if let Some(layout) = layout_caches
        .borrow()
        .iter()
        .find(|cache| cache.matches(&text, &runs, font_size, line_height, wrap_width))
        .map(|cache| cache.layout.clone())
    {
        return layout;
    }

    let layout = Arc::new(build_selectable_layout_from_parts(
        text.clone(),
        font_size,
        line_height,
        runs.clone(),
        wrap_width,
        window,
    ));
    let mut caches = layout_caches.borrow_mut();
    if caches.len() >= 4 {
        caches.remove(0);
    }
    caches.push(SelectableTextLayoutCache {
        text,
        runs,
        font_size,
        line_height,
        wrap_width,
        layout: layout.clone(),
    });
    layout
}

fn build_selectable_layout_from_parts(
    text: SharedString,
    font_size: Pixels,
    line_height: Pixels,
    runs: Vec<TextRun>,
    wrap_width: Option<Pixels>,
    window: &mut Window,
) -> SelectableTextLayout {
    let source_text = text.clone();
    let lines: Vec<gpui::WrappedLine> = window
        .text_system()
        .shape_text(text, font_size, &runs, wrap_width, None)
        .map(|lines| lines.into_iter().collect())
        .unwrap_or_default();
    let line_offsets = shaped_line_start_offsets(
        source_text.as_ref(),
        lines.iter().map(|line| line.text.as_ref()),
    );

    let mut width = px(1.0);
    let mut height = px(0.0);
    for line in &lines {
        let line_size = line.size(line_height);
        width = width.max(line_size.width).ceil();
        height += line_size.height;
    }

    SelectableTextLayout {
        lines,
        line_offsets,
        width,
        height,
    }
}

fn selectable_wrap_width(
    wrap: SelectableTextWrap,
    known_width: Option<Pixels>,
    available_width: AvailableSpace,
) -> Option<Pixels> {
    if wrap != SelectableTextWrap::Normal {
        return None;
    }

    known_width.or(match available_width {
        AvailableSpace::Definite(width) => Some(width.max(px(1.0))),
        AvailableSpace::MinContent | AvailableSpace::MaxContent => None,
    })
}

fn add_wrapped_selection_quads(
    line: &gpui::WrappedLine,
    start: usize,
    end: usize,
    y: Pixels,
    line_height: Pixels,
    bounds: Bounds<Pixels>,
    align: gpui::TextAlign,
    color: gpui::Hsla,
    quads: &mut Vec<PaintQuad>,
) {
    for (row, row_start, row_end) in visual_row_ranges(line) {
        let segment_start = start.max(row_start);
        let segment_end = end.min(row_end);
        if segment_start >= segment_end {
            continue;
        }

        let start_x = if segment_start == row_start && row > 0 {
            px(0.0)
        } else {
            line.position_for_index(segment_start, line_height)
                .map(|pos| pos.x)
                .unwrap_or(px(0.0))
        };
        let end_x = if segment_end == row_start && row > 0 {
            px(0.0)
        } else {
            line.position_for_index(segment_end, line_height)
                .map(|pos| pos.x)
                .unwrap_or_else(|| line.size(line_height).width)
        };
        let row_y = line_height * row as f32;
        let alignment = visual_row_alignment_offset(line, row, bounds.size.width, align);
        let width = (end_x - start_x).max(px(1.0));
        quads.push(fill(
            Bounds::new(
                point(bounds.left() + alignment + start_x, y + row_y),
                size(width, line_height),
            ),
            color,
        ));
    }
}

fn visual_row_alignment_offset(
    line: &gpui::WrappedLine,
    row: usize,
    width: Pixels,
    align: gpui::TextAlign,
) -> Pixels {
    if align == gpui::TextAlign::Left {
        return px(0.0);
    }
    let boundary_x = |row: usize| {
        line.wrap_boundaries()
            .get(row)
            .map_or(line.unwrapped_layout.width, |boundary| {
                line.runs()[boundary.run_ix].glyphs[boundary.glyph_ix]
                    .position
                    .x
            })
    };
    let start = row.checked_sub(1).map_or(px(0.0), boundary_x);
    let remaining = width - (boundary_x(row) - start);
    match align {
        gpui::TextAlign::Left => px(0.0),
        gpui::TextAlign::Center => remaining / 2.0,
        gpui::TextAlign::Right => remaining,
    }
}

fn visual_row_ranges(line: &gpui::WrappedLine) -> Vec<(usize, usize, usize)> {
    let mut rows = Vec::new();
    let mut row_start = 0;

    for (row, boundary) in line.wrap_boundaries().iter().enumerate() {
        let row_end = line
            .runs()
            .get(boundary.run_ix)
            .and_then(|run| run.glyphs.get(boundary.glyph_ix))
            .map(|glyph| glyph.index)
            .unwrap_or(row_start);
        rows.push((row, row_start, row_end.max(row_start)));
        row_start = row_end.max(row_start);
    }

    rows.push((
        line.wrap_boundaries().len(),
        row_start,
        line.len().max(row_start),
    ));
    rows
}

fn normalize_runs(mut runs: Vec<TextRun>, text_len: usize, color: gpui::Hsla) -> Vec<TextRun> {
    if text_len == 0 {
        return Vec::new();
    }
    if runs.is_empty() {
        let mut run = TextStyle::default().to_run(text_len);
        run.color = color;
        return vec![run];
    }
    let mut total = 0;
    for run in &mut runs {
        if run.color == gpui::transparent_black() {
            run.color = color;
        }
        total += run.len;
    }
    if total < text_len {
        let mut run = runs
            .last()
            .cloned()
            .unwrap_or_else(|| TextStyle::default().to_run(0));
        run.len = text_len - total;
        runs.push(run);
    } else if total > text_len {
        let mut remaining = text_len;
        runs.retain_mut(|run| {
            if remaining == 0 {
                return false;
            }
            if run.len > remaining {
                run.len = remaining;
            }
            remaining = remaining.saturating_sub(run.len);
            true
        });
    }
    runs
}

fn is_word_char(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_' || ('\u{4e00}'..='\u{9fff}').contains(&ch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selectable_text_wraps_against_parent_available_width() {
        let source = include_str!("selectable_text.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains("request_measured_layout"));
        assert!(source.contains("type SelectableTextLayoutCacheStore"));
        assert!(source.contains("cached_selectable_layout"));
        assert!(source.contains("cache.matches"));
        assert!(source.contains("caches.len() >= 4"));
        assert!(source.contains("line_offsets: Vec<usize>"));
        assert!(source.contains("selectable_wrap_width"));
        assert!(source.contains("AvailableSpace::Definite"));
        assert!(source.contains("wrapper = wrapper.w_full().flex_shrink(1.0)"));
        assert!(source.contains(".min_w(px(0.0))"));
        assert!(!source.contains("window.viewport_size().width.max(px(1.0))"));
    }

    #[test]
    fn selectable_text_actions_include_copy_shortcuts() {
        let source = include_str!("selectable_text.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();
        assert!(source.contains("SelectableTextSelectAll"));
        assert!(source.contains("SelectableTextCopy"));
        assert!(source.contains("KeyBinding::new(\"ctrl-a\""));
        assert!(source.contains("KeyBinding::new(\"cmd-a\""));
        assert!(source.contains("KeyBinding::new(\"ctrl-c\""));
        assert!(source.contains("KeyBinding::new(\"cmd-c\""));
        assert!(source.contains("fn set_select_all"));
        assert!(source.contains("fn select_all"));
        assert!(source.contains("fn on_key_down"));
        assert!(source.contains("event.keystroke.modifiers.control"));
        assert!(source.contains("event.keystroke.modifiers.platform"));
        assert!(source.contains("event.click_count == 2"));
        assert!(source.contains("window.capture_pointer(hitbox.id)"));
        assert!(source.contains("fn clear_other_selection_states"));
        assert!(source.contains("fn clear_selection_state_for"));
        assert!(source.contains("window.refresh()"));
        assert!(source.contains("hitbox.is_hovered(window) || selecting"));
        assert!(source.contains("fn is_selecting(&self, cx: &App) -> bool"));
        assert!(source.contains("phase.capture()"));
        assert!(source.contains("!bounds.contains(&event.position)"));
        assert!(source.contains("clear_selection_state_for(&input_id)"));
    }

    #[test]
    fn selectable_text_paints_selection_between_run_backgrounds_and_glyphs() {
        let source = include_str!("selectable_text.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();
        let background_ix = source
            .find(
                "line.paint_background(origin, line_height, text_align, Some(bounds), window, cx)",
            )
            .expect("selectable text should paint TextRun backgrounds");
        let selection_ix = source
            .find("for selection in prepaint.selection.drain(..)")
            .expect("selectable text should paint selection quads");
        let glyph_ix = source
            .find("line.paint(origin, line_height, text_align, Some(bounds), window, cx)")
            .expect("selectable text should paint glyphs");

        assert!(background_ix < selection_ix);
        assert!(selection_ix < glyph_ix);
    }

    #[test]
    fn selectable_text_tracks_visual_line_offsets_for_wrapped_and_newline_text() {
        let source = include_str!("selectable_text.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains("shaped_line_start_offsets"));
        assert!(source.contains("line_starts.push((y, line_start))"));
        assert!(!source.contains("line_start += line.len() + 1"));
        assert!(source.contains(".closest_index_for_position("));
        assert!(source.contains("position, self.line_height"));
        assert!(source.contains("start - line_start"));
        assert!(source.contains("end - line_start"));
        assert!(source.contains("fn visual_row_ranges"));
        assert!(source.contains("line.wrap_boundaries()"));
        assert!(source.contains("glyph.index"));
    }

    #[test]
    fn shaped_line_offsets_scan_original_text_instead_of_guessing_one_newline_byte() {
        assert_eq!(
            shaped_line_start_offsets("Title\nBody", ["Title", "Body"]),
            vec![0, 6]
        );
        assert_eq!(
            shaped_line_start_offsets("Title\r\nBody", ["Title", "Body"]),
            vec![0, 7]
        );
        assert_eq!(
            shaped_line_start_offsets("Title\n\nBody", ["Title", "Body"]),
            vec![0, 7]
        );
    }

    #[test]
    fn shaped_line_offsets_preserve_multibyte_boundaries() {
        assert_eq!(
            shaped_line_start_offsets("标题\n正文", ["标题", "正文"]),
            vec![0, "标题\n".len()]
        );
    }

    #[test]
    fn aligned_wrapped_rows_share_pointer_and_selection_geometry() {
        let mut line = gpui::WrappedLine::default();
        line.text = "abcdefg".into();
        let layout = Arc::get_mut(&mut *line).unwrap();
        let unwrapped = Arc::get_mut(&mut layout.unwrapped_layout).unwrap();
        unwrapped.width = px(70.0);
        unwrapped.len = 7;
        unwrapped.runs = vec![gpui::ShapedRun {
            font_id: gpui::FontId(0),
            glyphs: (0..7)
                .map(|index| gpui::ShapedGlyph {
                    id: gpui::GlyphId(0),
                    position: point(px(index as f32 * 10.0), px(0.0)),
                    index,
                    is_emoji: false,
                })
                .collect(),
        }];
        layout.wrap_boundaries.push(gpui::WrapBoundary {
            run_ix: 0,
            glyph_ix: 4,
        });
        assert_eq!(
            visual_row_alignment_offset(&line, 0, px(100.0), gpui::TextAlign::Center),
            px(30.0)
        );
        assert_eq!(
            visual_row_alignment_offset(&line, 1, px(100.0), gpui::TextAlign::Center),
            px(35.0)
        );
        assert_eq!(
            visual_row_alignment_offset(&line, 1, px(20.0), gpui::TextAlign::Right),
            px(-10.0)
        );
        let mut quads = Vec::new();
        add_wrapped_selection_quads(
            &line,
            4,
            6,
            px(10.0),
            px(20.0),
            Bounds::new(point(px(5.0), px(10.0)), size(px(100.0), px(40.0))),
            gpui::TextAlign::Right,
            gpui::black(),
            &mut quads,
        );
        assert_eq!(quads.len(), 1);
        assert_eq!(
            quads[0].bounds,
            Bounds::new(point(px(75.0), px(30.0)), size(px(20.0), px(20.0)))
        );
    }
    #[test]
    fn scoped_selection_spans_sparse_leaves_in_both_directions() {
        let a = ScopedPosition {
            order: 1,
            offset: 2,
        };
        let b = ScopedPosition {
            order: 5,
            offset: 3,
        };
        for (anchor, head) in [(a, b), (b, a)] {
            assert_eq!(scoped_leaf_range(1, 6, anchor, head), 2..6);
            assert_eq!(scoped_leaf_range(3, 4, anchor, head), 0..4);
            assert_eq!(scoped_leaf_range(5, 8, anchor, head), 0..3);
            assert_eq!(scoped_leaf_range(6, 8, anchor, head), 0..0);
        }
    }

    #[test]
    fn scoped_copy_joins_only_member_text_in_order_with_separators() {
        let members = [(0, "前言", "\n\n"), (4, "title", "\n"), (9, "结尾", "\n")];
        let start = ScopedPosition {
            order: 0,
            offset: "前".len(),
        };
        let end = ScopedPosition {
            order: 9,
            offset: "结".len(),
        };
        assert_eq!(joined_scoped_text(members, start, end), "言\n\ntitle\n结");
        assert_eq!(joined_scoped_text(members, end, start), "言\n\ntitle\n结");
        assert_eq!(joined_scoped_text(members, start, start), "");
    }
}

#[cfg(test)]
pub(crate) fn test_display_point(id: &str, offset: usize, line_height: Pixels) -> Point<Pixels> {
    let id: ElementId = SharedString::from(id.to_owned()).into();
    with_selection_state(&id, |state| {
        let bounds = state.bounds.expect("displayed selectable text bounds");
        let layout = state
            .layout
            .as_ref()
            .expect("displayed selectable text layout");
        for (line, (y, start)) in layout.lines.iter().zip(&state.line_starts) {
            if offset <= start + line.len() {
                let local = line
                    .position_for_index(offset - start, line_height)
                    .unwrap();
                return point(bounds.left() + local.x, *y + local.y + line_height * 0.5);
            }
        }
        panic!("display offset is outside text");
    })
}
