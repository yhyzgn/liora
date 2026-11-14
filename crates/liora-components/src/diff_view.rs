//! Native, read-only UTF-8 diff and review controls.
//!
//! Diffing uses Myers' shortest-edit-script algorithm with an explicit work
//! budget. Inputs outside that budget return [`DiffError`] rather than a
//! partial or guessed diff.

use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use gpui::{
    App, Context, EventEmitter, IntoElement, Render, SharedString, Window, div, prelude::*, px,
};
use liora_core::Config;

const MAX_LINES: usize = 100_000;
const MAX_FRONTIER_WORK: usize = 2_000_000;
const HUNK_CONTEXT_LINES: usize = 3;
static NEXT_HUNK_ID: AtomicU64 = AtomicU64::new(1);

/// Before/after text and optional display paths. Inputs are UTF-8 strings, so
/// binary data cannot be passed to this API accidentally.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiffDocument {
    /// Original UTF-8 source.
    pub before: SharedString,
    /// Updated UTF-8 source.
    pub after: SharedString,
    /// Optional path shown for the original side.
    pub before_path: Option<SharedString>,
    /// Optional path shown for the updated side.
    pub after_path: Option<SharedString>,
}

impl DiffDocument {
    /// Creates a diff document from UTF-8 text.
    pub fn new(before: impl Into<SharedString>, after: impl Into<SharedString>) -> Self {
        Self {
            before: before.into(),
            after: after.into(),
            before_path: None,
            after_path: None,
        }
    }

    /// Adds optional path metadata for the two displayed sides.
    pub fn paths(mut self, before: Option<SharedString>, after: Option<SharedString>) -> Self {
        self.before_path = before;
        self.after_path = after;
        self
    }
}

/// Why a document could not be diffed. No partial or fallback diff is returned.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiffError {
    /// One side exceeded the configured line limit.
    TooManyLines {
        /// Input side that exceeded the limit: `"before"` or `"after"`.
        side: &'static str,
        /// Number of lines in that input.
        count: usize,
        /// Maximum accepted line count per input side.
        limit: usize,
    },
    /// Myers' frontier exceeded the deterministic work budget.
    WorkLimitExceeded {
        /// Maximum allowed cumulative Myers frontier work.
        limit: usize,
    },
    /// The process-wide monotonic hunk identifier space was exhausted.
    HunkIdExhausted,
}

impl std::fmt::Display for DiffError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooManyLines { side, count, limit } => {
                write!(f, "{side} input has {count} lines; limit is {limit}")
            }
            Self::WorkLimitExceeded { limit } => {
                write!(f, "diff exceeded the frontier work limit of {limit}")
            }
            Self::HunkIdExhausted => f.write_str("diff hunk identifier space exhausted"),
        }
    }
}

impl std::error::Error for DiffError {}

/// Display layout for a [`DiffView`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DiffLayout {
    /// Render each changed line in a unified stream.
    #[default]
    Inline,
    /// Render old and new line columns next to each other.
    SideBySide,
}

/// Classification of a rendered diff line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiffLineKind {
    /// Unchanged context included around edits.
    Context,
    /// Line exists only in the updated text.
    Added,
    /// Line exists only in the original text.
    Removed,
}

/// One rendered row with one-based source line mappings. `None` represents an
/// empty side (for example, an inserted line has no original line number).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiffLine {
    /// Row classification.
    pub kind: DiffLineKind,
    /// Original text for this row, if present.
    pub before: Option<SharedString>,
    /// Updated text for this row, if present.
    pub after: Option<SharedString>,
    /// One-based original line number.
    pub before_line: Option<usize>,
    /// One-based updated line number.
    pub after_line: Option<usize>,
    /// Whether the original line has a trailing LF terminator.
    pub before_terminated: Option<bool>,
    /// Whether the updated line has a trailing LF terminator.
    pub after_terminated: Option<bool>,
    /// Whether the original line uses CRLF rather than LF.
    pub before_crlf: Option<bool>,
    /// Whether the updated line uses CRLF rather than LF.
    pub after_crlf: Option<bool>,
}

/// Old/new line coordinates for a location callback. Line numbers are one-based.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DiffLineNumbers {
    /// Original-side source line, when the row has one.
    pub before_line: Option<usize>,
    /// Updated-side source line, when the row has one.
    pub after_line: Option<usize>,
}

/// A contiguous changed region with surrounding context and independent review state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiffHunk {
    /// Unique identifier; IDs are not reused when the document is replaced.
    pub id: u64,
    /// Zero-based original line index, or insertion boundary when count is zero.
    pub before_start: usize,
    /// Number of original lines represented by this hunk.
    pub before_count: usize,
    /// Zero-based updated line index, or insertion boundary when count is zero.
    pub after_start: usize,
    /// Number of updated lines represented by this hunk.
    pub after_count: usize,
    /// Context and changed rows belonging to this hunk.
    pub lines: Vec<DiffLine>,
    /// `None` while awaiting review; otherwise the recorded decision.
    pub decision: Option<DiffDecision>,
}

/// Review decision for one hunk.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiffDecision {
    /// The reviewer accepts this hunk.
    Accepted,
    /// The reviewer rejects this hunk.
    Rejected,
}

/// Event emitted after a review action succeeds or is rejected.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiffEvent {
    /// A hunk was accepted.
    HunkAccepted {
        /// Identity of the hunk whose decision became accepted.
        hunk_id: u64,
    },
    /// A hunk was rejected.
    HunkRejected {
        /// Identity of the hunk whose decision became rejected.
        hunk_id: u64,
    },
    /// A delayed or repeated review action did not match a pending hunk.
    ActionRejected {
        /// Hunk identity supplied by the rejected review action.
        hunk_id: u64,
        /// Reason the review action could not be applied.
        error: DiffActionError,
    },
}

/// Invalid review action; stale, unknown, and already-decided IDs are rejected.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiffActionError {
    /// The ID does not belong to this current document.
    UnknownHunk {
        /// Unknown or expired identity supplied by the caller.
        hunk_id: u64,
    },
    /// Review decisions are terminal and cannot be silently overwritten.
    AlreadyDecided {
        /// Identity of the hunk that already has a terminal decision.
        hunk_id: u64,
        /// Previously recorded decision, which the rejected action leaves unchanged.
        decision: DiffDecision,
    },
}

impl std::fmt::Display for DiffActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownHunk { hunk_id } => write!(f, "unknown or expired diff hunk {hunk_id}"),
            Self::AlreadyDecided { hunk_id, decision } => {
                write!(f, "diff hunk {hunk_id} is already {decision:?}")
            }
        }
    }
}

impl std::error::Error for DiffActionError {}

/// Availability of the diff review surface.
///
/// `Binary` and `NotApplicable` are host-provided states: this component never
/// decodes bytes, reads paths, or invents text differences for them.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DiffStatus {
    /// UTF-8 text is available for review.
    #[default]
    Ready,
    /// Both supplied text sides are empty.
    Empty,
    /// The host identified the resource as binary.
    Binary,
    /// The host identified the resource as outside diff review scope.
    NotApplicable,
}

/// Read-only native GPUI diff viewer with per-hunk review actions.
pub struct DiffView {
    document: DiffDocument,
    hunks: Vec<DiffHunk>,
    status: DiffStatus,
    layout: DiffLayout,
    on_locate: Option<Arc<dyn Fn(DiffLineNumbers, &mut Window, &mut App) + 'static>>,
}

impl EventEmitter<DiffEvent> for DiffView {}

impl DiffView {
    /// Computes the line diff and creates a view. Limits are reported explicitly.
    pub fn new(document: DiffDocument, _cx: &mut Context<Self>) -> Result<Self, DiffError> {
        let hunks = build_hunks(&document)?;
        let status = if document.before.is_empty() && document.after.is_empty() {
            DiffStatus::Empty
        } else {
            DiffStatus::Ready
        };
        Ok(Self {
            document,
            hunks,
            status,
            layout: DiffLayout::Inline,
            on_locate: None,
        })
    }

    /// Creates a view with host-declared availability; Ready/Empty follow the document.
    /// Binary and not-applicable states contain no hunks or text interpretation.
    pub fn with_status(
        document: DiffDocument,
        status: DiffStatus,
        cx: &mut Context<Self>,
    ) -> Result<Self, DiffError> {
        if matches!(status, DiffStatus::Ready | DiffStatus::Empty) {
            Self::new(document, cx)
        } else {
            Ok(Self {
                document,
                hunks: Vec::new(),
                status,
                layout: DiffLayout::Inline,
                on_locate: None,
            })
        }
    }

    /// Replaces both inputs and invalidates all prior hunk IDs.
    pub fn set_document(
        &mut self,
        document: DiffDocument,
        cx: &mut Context<Self>,
    ) -> Result<(), DiffError> {
        let hunks = build_hunks(&document)?;
        let status = if document.before.is_empty() && document.after.is_empty() {
            DiffStatus::Empty
        } else {
            DiffStatus::Ready
        };
        self.document = document;
        self.hunks = hunks;
        self.status = status;
        cx.notify();
        Ok(())
    }

    /// Declares a host-owned availability state without changing either text side.
    pub fn set_status(
        &mut self,
        status: DiffStatus,
        cx: &mut Context<Self>,
    ) -> Result<(), DiffError> {
        let status = if matches!(status, DiffStatus::Ready | DiffStatus::Empty) {
            if self.document.before.is_empty() && self.document.after.is_empty() {
                DiffStatus::Empty
            } else {
                DiffStatus::Ready
            }
        } else {
            status
        };
        if self.status == status {
            return Ok(());
        }
        if status == DiffStatus::Ready {
            self.hunks = build_hunks(&self.document)?;
        } else {
            self.hunks.clear();
        }
        self.status = status;
        cx.notify();
        Ok(())
    }

    /// Changes unified/side-by-side presentation.
    pub fn set_layout(&mut self, layout: DiffLayout, cx: &mut Context<Self>) {
        if self.layout != layout {
            self.layout = layout;
            cx.notify();
        }
    }

    /// Sets a keyboard-accessible per-hunk location action.
    pub fn on_locate(
        mut self,
        callback: impl Fn(DiffLineNumbers, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_locate = Some(Arc::new(callback));
        self
    }

    /// Current immutable document snapshot.
    pub fn document(&self) -> &DiffDocument {
        &self.document
    }

    /// Current host/document availability state.
    pub fn status(&self) -> DiffStatus {
        self.status
    }

    /// Current review hunks, in source order.
    pub fn hunks(&self) -> &[DiffHunk] {
        &self.hunks
    }

    /// Current display layout.
    pub fn layout(&self) -> DiffLayout {
        self.layout
    }

    /// Records a single terminal decision and emits its matching event.
    pub fn decide_hunk(
        &mut self,
        hunk_id: u64,
        decision: DiffDecision,
        cx: &mut Context<Self>,
    ) -> Result<DiffEvent, DiffActionError> {
        let Some(hunk) = self.hunks.iter_mut().find(|hunk| hunk.id == hunk_id) else {
            return Err(DiffActionError::UnknownHunk { hunk_id });
        };
        if let Some(previous) = hunk.decision {
            return Err(DiffActionError::AlreadyDecided {
                hunk_id,
                decision: previous,
            });
        }
        hunk.decision = Some(decision);
        let event = match decision {
            DiffDecision::Accepted => DiffEvent::HunkAccepted { hunk_id },
            DiffDecision::Rejected => DiffEvent::HunkRejected { hunk_id },
        };
        cx.emit(event.clone());
        cx.notify();
        Ok(event)
    }

    fn decide_from_button(&mut self, hunk_id: u64, decision: DiffDecision, cx: &mut Context<Self>) {
        if let Err(error) = self.decide_hunk(hunk_id, decision, cx) {
            cx.emit(DiffEvent::ActionRejected { hunk_id, error });
        }
    }
}

impl Render for DiffView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let layout = self.layout;
        let before_label = self
            .document
            .before_path
            .clone()
            .unwrap_or_else(|| "Before".into());
        let after_label = self
            .document
            .after_path
            .clone()
            .unwrap_or_else(|| "After".into());
        let toolbar = div()
            .flex()
            .items_center()
            .gap_2()
            .pb_2()
            .child(review_control(
                crate::Button::new("Unified")
                    .variant(if layout == DiffLayout::Inline {
                        liora_theme::ButtonVariant::Primary
                    } else {
                        liora_theme::ButtonVariant::Tertiary
                    })
                    .small(),
                ReviewAction::Layout(DiffLayout::Inline),
                false,
                cx,
            ))
            .child(review_control(
                crate::Button::new("Side by side")
                    .variant(if layout == DiffLayout::SideBySide {
                        liora_theme::ButtonVariant::Primary
                    } else {
                        liora_theme::ButtonVariant::Tertiary
                    })
                    .small(),
                ReviewAction::Layout(DiffLayout::SideBySide),
                false,
                cx,
            ));

        let hunks = self.hunks.iter().map(|hunk| {
            let hunk_id = hunk.id;
            let pending = hunk.decision.is_none();
            let decision_text = match hunk.decision {
                None => "Pending",
                Some(DiffDecision::Accepted) => "Accepted",
                Some(DiffDecision::Rejected) => "Rejected",
            };
            let removed = hunk
                .lines
                .iter()
                .filter(|line| line.kind == DiffLineKind::Removed)
                .count();
            let added = hunk
                .lines
                .iter()
                .filter(|line| line.kind == DiffLineKind::Added)
                .count();
            let mut heading = div()
                .flex()
                .items_center()
                .gap_2()
                .px_3()
                .py_2()
                .bg(theme.neutral.card)
                .border_b_1()
                .border_color(theme.neutral.border)
                .child(format!(
                    "Hunk {} · −{} +{} · {}",
                    hunk.id, removed, added, decision_text
                ));
            if self.on_locate.is_some() {
                let location = hunk
                    .lines
                    .iter()
                    .find(|line| line.kind != DiffLineKind::Context)
                    .map(|line| DiffLineNumbers {
                        before_line: line.before_line,
                        after_line: line.after_line,
                    })
                    .unwrap_or_default();
                heading = heading.child(review_control(
                    crate::Button::new("Locate change").tertiary().small(),
                    ReviewAction::Locate(hunk_id, location),
                    false,
                    cx,
                ));
            }
            heading = heading
                .child(review_control(
                    crate::Button::new("Accept").success().small(),
                    ReviewAction::Decide(hunk_id, DiffDecision::Accepted),
                    !pending,
                    cx,
                ))
                .child(review_control(
                    crate::Button::new("Reject").danger().small(),
                    ReviewAction::Decide(hunk_id, DiffDecision::Rejected),
                    !pending,
                    cx,
                ));
            let rows = hunk.lines.iter().map(|line| match layout {
                DiffLayout::Inline => render_inline_line(line, theme.clone()),
                DiffLayout::SideBySide => render_side_line(line, theme.clone()),
            });
            div()
                .border_1()
                .border_color(theme.neutral.border)
                .rounded_sm()
                .mb_3()
                .child(heading)
                .children(rows)
        });

        let mut labels = div()
            .flex()
            .items_center()
            .gap_2()
            .pb_2()
            .child(div().flex_1().child(before_label));
        labels = match layout {
            DiffLayout::SideBySide => labels.child(div().flex_1().child(after_label)),
            DiffLayout::Inline => labels.child(after_label),
        };
        let mut root = div()
            .flex()
            .flex_col()
            .w_full()
            .gap_2()
            .child(toolbar)
            .child(labels);
        if self.hunks.is_empty() {
            let message = match self.status {
                DiffStatus::Ready => "No differences",
                DiffStatus::Empty => "Empty file — no differences",
                DiffStatus::Binary => "Binary file — diff not applicable",
                DiffStatus::NotApplicable => "Diff not applicable",
            };
            root = root.child(
                div()
                    .px_3()
                    .py_3()
                    .text_color(theme.neutral.text_2)
                    .child(message),
            );
        } else {
            root = root.children(hunks);
        }
        root
    }
}

#[derive(Clone, Copy, Debug)]
enum ReviewAction {
    Layout(DiffLayout),
    Locate(u64, DiffLineNumbers),
    Decide(u64, DiffDecision),
}

fn activate_review(
    this: &mut DiffView,
    action: ReviewAction,
    window: &mut Window,
    cx: &mut Context<DiffView>,
) {
    match action {
        ReviewAction::Layout(layout) => this.set_layout(layout, cx),
        ReviewAction::Locate(id, location) => {
            if this.hunks.iter().any(|hunk| hunk.id == id) {
                if let Some(callback) = &this.on_locate {
                    callback(location, window, cx);
                }
            }
        }
        ReviewAction::Decide(id, decision) => this.decide_from_button(id, decision, cx),
    }
}

fn review_control(
    button: crate::Button,
    action: ReviewAction,
    disabled: bool,
    cx: &Context<DiffView>,
) -> impl IntoElement {
    div()
        .id(SharedString::from(format!(
            "diff-{:?}-{action:?}",
            cx.entity_id()
        )))
        .tab_index(0)
        .tab_stop(!disabled)
        .on_key_down(
            cx.listener(move |this, event: &gpui::KeyDownEvent, window, cx| {
                if event.keystroke.key == "tab" {
                    if event.keystroke.modifiers.shift {
                        window.focus_prev(cx);
                    } else {
                        window.focus_next(cx);
                    }
                    cx.stop_propagation();
                } else if !disabled
                    && !event.is_held
                    && matches!(event.keystroke.key.as_str(), "enter" | "space")
                {
                    activate_review(this, action, window, cx);
                    cx.stop_propagation();
                }
            }),
        )
        .child(
            button
                .disabled(disabled)
                .on_click(cx.listener(move |this, _, window, cx| {
                    if !disabled {
                        activate_review(this, action, window, cx);
                    }
                })),
        )
}

fn render_inline_line(line: &DiffLine, theme: liora_theme::Theme) -> gpui::Div {
    let (marker, content, background) = match line.kind {
        DiffLineKind::Context => (
            " ",
            line.after
                .as_deref()
                .or(line.before.as_deref())
                .unwrap_or(""),
            theme.neutral.card,
        ),
        DiffLineKind::Added => (
            "+",
            line.after.as_deref().unwrap_or(""),
            theme.success.light_9,
        ),
        DiffLineKind::Removed => (
            "−",
            line.before.as_deref().unwrap_or(""),
            theme.danger.light_9,
        ),
    };
    let ending = match line.kind {
        DiffLineKind::Removed => ending_marker(line.before_terminated, line.before_crlf),
        DiffLineKind::Added => ending_marker(line.after_terminated, line.after_crlf),
        DiffLineKind::Context => "",
    };
    div()
        .flex()
        .items_start()
        .gap_2()
        .px_2()
        .py_1()
        .bg(background)
        .font_family("monospace")
        .child(
            div()
                .w(px(36.0))
                .text_right()
                .text_color(theme.neutral.text_3)
                .child(
                    line.before_line
                        .map_or_else(|| "·".to_owned(), |number| number.to_string()),
                ),
        )
        .child(
            div()
                .w(px(36.0))
                .text_right()
                .text_color(theme.neutral.text_3)
                .child(
                    line.after_line
                        .map_or_else(|| "·".to_owned(), |number| number.to_string()),
                ),
        )
        .child(div().w(px(16.0)).child(marker))
        .child(div().flex_1().min_w_0().child(format!("{content}{ending}")))
}

fn render_side_line(line: &DiffLine, theme: liora_theme::Theme) -> gpui::Div {
    let left_bg = if line.kind == DiffLineKind::Removed {
        theme.danger.light_9
    } else {
        theme.neutral.card
    };
    let right_bg = if line.kind == DiffLineKind::Added {
        theme.success.light_9
    } else {
        theme.neutral.card
    };
    let before_ending = ending_marker(line.before_terminated, line.before_crlf);
    let after_ending = ending_marker(line.after_terminated, line.after_crlf);
    let cell = |number: Option<usize>, text: Option<&str>, ending: &'static str, background| {
        div()
            .flex()
            .flex_1()
            .min_w_0()
            .gap_2()
            .px_2()
            .py_1()
            .bg(background)
            .font_family("monospace")
            .child(
                div()
                    .w(px(40.0))
                    .text_right()
                    .text_color(theme.neutral.text_3)
                    .child(number.map_or_else(|| "·".to_owned(), |number| number.to_string())),
            )
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .child(format!("{}{ending}", text.unwrap_or(""))),
            )
    };
    div()
        .flex()
        .items_start()
        .child(cell(
            line.before_line,
            line.before.as_deref(),
            before_ending,
            left_bg,
        ))
        .child(cell(
            line.after_line,
            line.after.as_deref(),
            after_ending,
            right_bg,
        ))
}

fn ending_marker(terminated: Option<bool>, crlf: Option<bool>) -> &'static str {
    match (terminated, crlf) {
        (Some(false), _) => "  [no final newline]",
        (Some(true), Some(true)) => "  [CRLF]",
        _ => "",
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct SourceLine {
    text: String,
    terminated: bool,
    crlf: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Edit {
    Equal(usize, usize),
    Delete(usize),
    Insert(usize),
}

fn split_lines(text: &str, side: &'static str) -> Result<Vec<SourceLine>, DiffError> {
    if text.is_empty() {
        return Ok(Vec::new());
    }
    let count = text.split_inclusive('\n').count();
    if count > MAX_LINES {
        return Err(DiffError::TooManyLines {
            side,
            count,
            limit: MAX_LINES,
        });
    }
    let lines = text
        .split_inclusive('\n')
        .map(|raw| {
            let terminated = raw.ends_with('\n');
            let body = if terminated {
                &raw[..raw.len() - 1]
            } else {
                raw
            };
            let crlf = terminated && body.ends_with('\r');
            let body = if crlf { &body[..body.len() - 1] } else { body };
            SourceLine {
                text: body.to_owned(),
                terminated,
                crlf,
            }
        })
        .collect::<Vec<_>>();
    Ok(lines)
}

fn build_hunks(document: &DiffDocument) -> Result<Vec<DiffHunk>, DiffError> {
    let before = split_lines(&document.before, "before")?;
    let after = split_lines(&document.after, "after")?;
    let edits = myers(&before, &after)?;
    let rows = edits
        .into_iter()
        .map(|edit| match edit {
            Edit::Equal(old, new) => DiffLine {
                kind: DiffLineKind::Context,
                before: Some(before[old].text.as_str().into()),
                after: Some(after[new].text.as_str().into()),
                before_line: Some(old + 1),
                after_line: Some(new + 1),
                before_terminated: Some(before[old].terminated),
                after_terminated: Some(after[new].terminated),
                before_crlf: Some(before[old].crlf),
                after_crlf: Some(after[new].crlf),
            },
            Edit::Delete(old) => DiffLine {
                kind: DiffLineKind::Removed,
                before: Some(before[old].text.as_str().into()),
                after: None,
                before_line: Some(old + 1),
                after_line: None,
                before_terminated: Some(before[old].terminated),
                after_terminated: None,
                before_crlf: Some(before[old].crlf),
                after_crlf: None,
            },
            Edit::Insert(new) => DiffLine {
                kind: DiffLineKind::Added,
                before: None,
                after: Some(after[new].text.as_str().into()),
                before_line: None,
                after_line: Some(new + 1),
                before_terminated: None,
                after_terminated: Some(after[new].terminated),
                before_crlf: None,
                after_crlf: Some(after[new].crlf),
            },
        })
        .collect::<Vec<_>>();

    let changed = rows
        .iter()
        .enumerate()
        .filter_map(|(index, row)| (row.kind != DiffLineKind::Context).then_some(index));
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    for index in changed {
        let start = index.saturating_sub(HUNK_CONTEXT_LINES);
        let end = (index + HUNK_CONTEXT_LINES + 1).min(rows.len());
        if let Some(last) = ranges.last_mut().filter(|(_, last_end)| start <= *last_end) {
            last.1 = end.max(last.1);
        } else {
            ranges.push((start, end));
        }
    }

    let mut before_prefix = Vec::with_capacity(rows.len() + 1);
    let mut after_prefix = Vec::with_capacity(rows.len() + 1);
    before_prefix.push(0);
    after_prefix.push(0);
    for row in &rows {
        before_prefix.push(
            before_prefix.last().copied().unwrap_or(0) + usize::from(row.before_line.is_some()),
        );
        after_prefix.push(
            after_prefix.last().copied().unwrap_or(0) + usize::from(row.after_line.is_some()),
        );
    }

    ranges
        .into_iter()
        .map(|(start, end)| {
            let lines = rows[start..end].to_vec();
            let before_count = lines
                .iter()
                .filter(|line| line.before_line.is_some())
                .count();
            let after_count = lines
                .iter()
                .filter(|line| line.after_line.is_some())
                .count();
            let before_start = lines
                .iter()
                .find_map(|line| line.before_line.map(|number| number - 1))
                .unwrap_or(before_prefix[start]);
            let after_start = lines
                .iter()
                .find_map(|line| line.after_line.map(|number| number - 1))
                .unwrap_or(after_prefix[start]);
            Ok(DiffHunk {
                id: next_hunk_id()?,
                before_start,
                before_count,
                after_start,
                after_count,
                lines,
                decision: None,
            })
        })
        .collect()
}

fn next_hunk_id() -> Result<u64, DiffError> {
    let mut current = NEXT_HUNK_ID.load(Ordering::Relaxed);
    loop {
        let next = current.checked_add(1).ok_or(DiffError::HunkIdExhausted)?;
        match NEXT_HUNK_ID.compare_exchange_weak(
            current,
            next,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(_) => return Ok(current),
            Err(observed) => current = observed,
        }
    }
}

fn myers(before: &[SourceLine], after: &[SourceLine]) -> Result<Vec<Edit>, DiffError> {
    let n = before.len();
    let m = after.len();
    let max = n + m;
    if max == 0 {
        return Ok(Vec::new());
    }
    let offset = max as isize + 1;
    let mut frontier = vec![0_isize; 2 * max + 3];
    frontier[(offset + 1) as usize] = 0;
    let mut trace: Vec<Vec<isize>> = Vec::new();
    let mut work = 0_usize;
    let mut found_distance = None;

    'search: for distance in 0..=max {
        let width = distance + 1;
        work = work.saturating_add(width);
        if work > MAX_FRONTIER_WORK {
            return Err(DiffError::WorkLimitExceeded {
                limit: MAX_FRONTIER_WORK,
            });
        }
        let d = distance as isize;
        for k in (-d..=d).step_by(2) {
            let index = (offset + k) as usize;
            let mut x = if k == -d || (k != d && frontier[index - 1] < frontier[index + 1]) {
                frontier[index + 1]
            } else {
                frontier[index - 1] + 1
            };
            let mut y = x - k;
            while x < n as isize && y < m as isize && before[x as usize] == after[y as usize] {
                x += 1;
                y += 1;
            }
            frontier[index] = x;
            if x >= n as isize && y >= m as isize {
                found_distance = Some(distance);
                break 'search;
            }
        }
        trace.push(
            (-d..=d)
                .step_by(2)
                .map(|k| frontier[(offset + k) as usize])
                .collect(),
        );
    }

    let distance = found_distance.ok_or(DiffError::WorkLimitExceeded {
        limit: MAX_FRONTIER_WORK,
    })?;
    let mut x = n as isize;
    let mut y = m as isize;
    let mut reversed = Vec::with_capacity(n + m);
    for d_usize in (1..=distance).rev() {
        let d = d_usize as isize;
        let previous = &trace[d_usize - 1];
        let previous_distance = d - 1;
        let k = x - y;
        let at = |key: isize| previous[((key + previous_distance) / 2) as usize];
        let previous_k = if k == -d || (k != d && at(k - 1) < at(k + 1)) {
            k + 1
        } else {
            k - 1
        };
        let previous_x = at(previous_k);
        let previous_y = previous_x - previous_k;
        while x > previous_x && y > previous_y {
            reversed.push(Edit::Equal((x - 1) as usize, (y - 1) as usize));
            x -= 1;
            y -= 1;
        }
        if x == previous_x {
            reversed.push(Edit::Insert((y - 1) as usize));
            y -= 1;
        } else {
            reversed.push(Edit::Delete((x - 1) as usize));
            x -= 1;
        }
    }
    while x > 0 && y > 0 {
        reversed.push(Edit::Equal((x - 1) as usize, (y - 1) as usize));
        x -= 1;
        y -= 1;
    }
    while x > 0 {
        reversed.push(Edit::Delete((x - 1) as usize));
        x -= 1;
    }
    while y > 0 {
        reversed.push(Edit::Insert((y - 1) as usize));
        y -= 1;
    }
    reversed.reverse();
    Ok(reversed)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn document(before: &str, after: &str) -> DiffDocument {
        DiffDocument::new(before, after)
    }

    #[test]
    fn empty_unchanged_and_line_terminator_boundaries_are_real_diffs() {
        assert!(build_hunks(&document("", "")).unwrap().is_empty());
        let inserted = build_hunks(&document("", "added")).unwrap();
        assert_eq!(inserted.len(), 1);
        assert_eq!(inserted[0].before_count, 0);
        assert_eq!(inserted[0].after_count, 1);
        assert_eq!(
            inserted[0].before_start, 0,
            "insertion into empty source is boundary zero"
        );
        assert_eq!(build_hunks(&document("same\n", "same\n")).unwrap().len(), 0);
        let eof_change = build_hunks(&document("same\n", "same")).unwrap();
        assert_eq!(eof_change.len(), 1);
        assert_eq!(
            eof_change[0]
                .lines
                .iter()
                .filter(|line| line.kind == DiffLineKind::Removed)
                .count(),
            1
        );
        assert_eq!(
            eof_change[0]
                .lines
                .iter()
                .filter(|line| line.kind == DiffLineKind::Added)
                .count(),
            1
        );
    }

    #[test]
    fn unicode_and_crlf_rows_keep_one_based_old_new_line_mapping() {
        let hunks = build_hunks(&document(
            "α\r\nkeep\r\n古い\r\n",
            "α\r\nkeep\r\n新しい\r\n",
        ))
        .unwrap();
        assert_eq!(hunks.len(), 1);
        let removed = hunks[0]
            .lines
            .iter()
            .find(|line| line.kind == DiffLineKind::Removed)
            .unwrap();
        let added = hunks[0]
            .lines
            .iter()
            .find(|line| line.kind == DiffLineKind::Added)
            .unwrap();
        assert_eq!(removed.before.as_deref(), Some("古い"));
        assert_eq!(removed.before_line, Some(3));
        assert_eq!(added.after.as_deref(), Some("新しい"));
        assert_eq!(added.after_line, Some(3));
    }

    #[test]
    fn full_insert_delete_and_independent_hunks_preserve_order() {
        let inserted = build_hunks(&document("", "一\n二\n")).unwrap();
        assert_eq!(
            inserted[0]
                .lines
                .iter()
                .map(|line| line.after.as_deref().unwrap_or(""))
                .collect::<Vec<_>>(),
            vec!["一", "二"]
        );
        let deleted = build_hunks(&document("一\n二\n", "")).unwrap();
        assert_eq!(deleted[0].before_count, 2);
        let separate = build_hunks(&document(
            "a\nb\nc\nd\ne\nf\ng\nh\ni\n",
            "A\nb\nc\nd\ne\nf\ng\nh\nI\n",
        ))
        .unwrap();
        assert_eq!(separate.len(), 2);
        assert_eq!(separate[0].lines.first().unwrap().before_line, Some(1));
        assert_eq!(separate[1].lines.last().unwrap().after_line, Some(9));
    }

    #[test]
    fn complexity_limit_returns_error_instead_of_fabricating_a_diff() {
        let before = (0..2_100)
            .map(|line| format!("old-{line}\n"))
            .collect::<String>();
        let after = (0..2_100)
            .map(|line| format!("new-{line}\n"))
            .collect::<String>();
        assert_eq!(
            build_hunks(&document(&before, &after)),
            Err(DiffError::WorkLimitExceeded {
                limit: MAX_FRONTIER_WORK
            })
        );
    }

    #[test]
    fn line_limit_rejects_input_explicitly() {
        let too_many = "x\n".repeat(MAX_LINES + 1);
        assert!(matches!(
            build_hunks(&document(&too_many, "")),
            Err(DiffError::TooManyLines { side: "before", .. })
        ));
    }

    #[test]
    fn myers_produces_a_valid_minimal_script_on_repeated_lines() {
        let before = split_lines("a\nb\na\nc\n", "before").unwrap();
        let after = split_lines("a\na\nb\nc\n", "after").unwrap();
        let edits = myers(&before, &after).unwrap();
        let mut old = 0;
        let mut new = 0;
        for edit in edits {
            match edit {
                Edit::Equal(i, j) => {
                    assert_eq!(i, old);
                    assert_eq!(j, new);
                    assert_eq!(before[i], after[j]);
                    old += 1;
                    new += 1;
                }
                Edit::Delete(i) => {
                    assert_eq!(i, old);
                    old += 1;
                }
                Edit::Insert(j) => {
                    assert_eq!(j, new);
                    new += 1;
                }
            }
        }
        assert_eq!((old, new), (before.len(), after.len()));
    }

    #[test]
    fn carriage_returns_and_newline_style_are_not_discarded() {
        assert!(!build_hunks(&document("x\r", "x")).unwrap().is_empty());
        let hunks = build_hunks(&document("x\r\n", "x\n")).unwrap();
        assert_eq!(hunks[0].lines[0].before_crlf, Some(true));
        assert!(
            hunks[0]
                .lines
                .iter()
                .any(|line| line.after_crlf == Some(false))
        );
        assert_eq!(split_lines("", "before").unwrap().len(), 0);
    }

    #[test]
    fn myers_small_repeated_inputs_reconstruct_both_sides_with_minimal_edits() {
        let sequences = (0..=4)
            .flat_map(|len| {
                (0..1usize << len).map(move |bits| {
                    (0..len)
                        .map(|index| {
                            if bits & (1 << index) == 0 {
                                "a\n"
                            } else {
                                "b\n"
                            }
                        })
                        .collect::<String>()
                })
            })
            .collect::<Vec<_>>();
        for old_text in &sequences {
            for new_text in &sequences {
                let old = split_lines(old_text, "before").unwrap();
                let new = split_lines(new_text, "after").unwrap();
                let script = myers(&old, &new).unwrap();
                let mut costs = (0..=new.len()).collect::<Vec<_>>();
                for (i, old_line) in old.iter().enumerate() {
                    let mut diagonal = costs[0];
                    costs[0] = i + 1;
                    for (j, new_line) in new.iter().enumerate() {
                        let above = costs[j + 1];
                        costs[j + 1] = if old_line == new_line {
                            diagonal
                        } else {
                            1 + above.min(costs[j])
                        };
                        diagonal = above;
                    }
                }
                assert_eq!(
                    script
                        .iter()
                        .filter(|edit| !matches!(edit, Edit::Equal(_, _)))
                        .count(),
                    costs[new.len()]
                );
                let (mut i, mut j) = (0, 0);
                for edit in script {
                    match edit {
                        Edit::Equal(a, b) => {
                            assert_eq!((a, b), (i, j));
                            assert_eq!(old[a], new[b]);
                            i += 1;
                            j += 1;
                        }
                        Edit::Delete(a) => {
                            assert_eq!(a, i);
                            i += 1;
                        }
                        Edit::Insert(b) => {
                            assert_eq!(b, j);
                            j += 1;
                        }
                    }
                }
                assert_eq!((i, j), (old.len(), new.len()));
            }
        }
    }
}
