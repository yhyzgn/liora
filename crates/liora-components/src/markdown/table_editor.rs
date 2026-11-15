//! Retained native controls for editing one source-backed Markdown table.

use super::model::{MarkdownTable, MarkdownTableEdit};
use crate::{Button, Input};
use gpui::{
    App, Context, Entity, IntoElement, KeyDownEvent, Render, SharedString, Window, div, prelude::*,
    px,
};
use liora_core::Config;
use std::rc::Rc;

type EditCallback = Rc<dyn Fn(MarkdownTableEdit, &mut App)>;
const MIN_WIDTH: f32 = 120.0;
const MAX_WIDTH: f32 = 640.0;
const WIDTH_STEP: f32 = 24.0;
const INITIAL_WIDTH: f32 = 216.0;

/// Retain one entity per table ID. Widths are visual state, never Markdown data.
/// Call `sync` with each fresh document snapshot before rendering the entity.
pub(super) struct TableEditor {
    table: MarkdownTable,
    inputs: Vec<Vec<Entity<Input>>>,
    widths: Vec<f32>,
    on_edit: EditCallback,
}

impl TableEditor {
    pub(super) fn input_at(&self, offset: usize) -> Option<Entity<Input>> {
        self.table
            .rows
            .iter()
            .enumerate()
            .find_map(|(row, snapshot)| {
                snapshot
                    .cells
                    .iter()
                    .position(|cell| cell.source_range.contains(&offset))
                    .and_then(|column| self.inputs.get(row)?.get(column).cloned())
            })
    }

    pub(super) fn is_composing(&self, cx: &App) -> bool {
        self.inputs
            .iter()
            .flatten()
            .any(|input| input.read(cx).is_composing())
    }

    pub(super) fn new(table: MarkdownTable, on_edit: EditCallback, cx: &mut Context<Self>) -> Self {
        let mut editor = Self {
            widths: vec![INITIAL_WIDTH; table.alignments.len()],
            table,
            inputs: Vec::new(),
            on_edit,
        };
        editor.sync_inputs(cx);
        editor
    }

    /// Reconciles external edits/undo without recreating unchanged input entities.
    pub(super) fn sync(
        &mut self,
        table: MarkdownTable,
        on_edit: EditCallback,
        cx: &mut Context<Self>,
    ) {
        self.on_edit = on_edit;
        if self.table == table {
            return;
        }
        self.table = table;
        self.widths
            .resize(self.table.alignments.len(), INITIAL_WIDTH);
        self.sync_inputs(cx);
        cx.notify();
    }

    fn sync_inputs(&mut self, cx: &mut Context<Self>) {
        self.inputs.truncate(self.table.rows.len());
        for (row, snapshot) in self.table.rows.iter().enumerate() {
            if row == self.inputs.len() {
                self.inputs.push(Vec::new());
            }
            self.inputs[row].truncate(snapshot.cells.len());
            for (column, cell) in snapshot.cells.iter().enumerate() {
                if column < self.inputs[row].len() {
                    self.inputs[row][column].update(cx, |input, cx| {
                        // Identical text must not cancel ongoing native IME composition.
                        if !input.is_composing() && input.value() != cell.text {
                            input.set_value(cell.text.clone(), cx);
                        }
                    });
                } else {
                    let owner = cx.weak_entity();
                    let id = format!("markdown-table-{}-cell-{row}-{column}", self.table.id);
                    let width = self.widths.get(column).copied().unwrap_or(INITIAL_WIDTH);
                    let input = cx.new(|cx| {
                        Input::new(cell.text.clone(), cx)
                            .id(id)
                            .width(px(width))
                            .placeholder(format!("Row {}, column {}", row + 1, column + 1))
                            .on_change(move |text, cx| {
                                let owner = owner.clone();
                                let text = text.to_owned();
                                // Source reconciliation may update this very input entity.
                                cx.defer(move |cx| {
                                    if let Some(owner) = owner.upgrade() {
                                        let editor = owner.read(cx);
                                        let Some(input) = editor
                                            .inputs
                                            .get(row)
                                            .and_then(|cells| cells.get(column))
                                        else {
                                            return;
                                        };
                                        if input.read(cx).is_composing()
                                            || input.read(cx).value().as_ref() != text
                                        {
                                            return;
                                        }
                                        if editor
                                            .table
                                            .rows
                                            .get(row)
                                            .and_then(|row| row.cells.get(column))
                                            .is_some_and(|cell| cell.text.as_ref() == text)
                                        {
                                            return;
                                        }
                                        let callback = editor.on_edit.clone();
                                        if editor.is_composing(cx) {
                                            return;
                                        }
                                        callback(
                                            MarkdownTableEdit::SetCell { row, column, text },
                                            cx,
                                        );
                                    }
                                });
                            })
                    });
                    self.inputs[row].push(input);
                }
            }
        }
    }

    fn resize(&mut self, column: usize, delta: f32, cx: &mut Context<Self>) {
        let Some(width) = self.widths.get_mut(column) else {
            return;
        };
        *width = (*width + delta).clamp(MIN_WIDTH, MAX_WIDTH);
        for row in &self.inputs {
            if let Some(input) = row.get(column) {
                input.update(cx, |input, cx| input.set_width(px(*width), cx));
            }
        }
        cx.notify();
    }

    fn action(
        &self,
        id: String,
        label: String,
        edit: MarkdownTableEdit,
        enabled: bool,
    ) -> gpui::AnyElement {
        let click = self.on_edit.clone();
        let keyboard = self.on_edit.clone();
        let key_edit = edit.clone();
        div()
            .id(SharedString::from(id))
            .focusable()
            .tab_index(0)
            .tab_stop(enabled)
            .aria_label(label.clone())
            .on_key_down(move |event: &KeyDownEvent, _, cx| {
                if enabled && matches!(event.keystroke.key.as_str(), "enter" | "space") {
                    keyboard(key_edit.clone(), cx);
                    cx.stop_propagation();
                }
            })
            .child(
                Button::new(label)
                    .tertiary()
                    .disabled(!enabled)
                    .on_click(move |_, _, cx| {
                        click(edit.clone(), cx);
                    }),
            )
            .into_any_element()
    }

    fn resize_control(&self, column: usize, delta: f32, cx: &Context<Self>) -> gpui::AnyElement {
        let width = self.widths[column];
        let enabled = if delta < 0.0 {
            width > MIN_WIDTH
        } else {
            width < MAX_WIDTH
        };
        let label = format!(
            "{} column {}",
            if delta < 0.0 { "Narrow" } else { "Widen" },
            column + 1
        );
        let owner = cx.entity();
        div()
            .id(SharedString::from(format!(
                "table-{}-resize-{column}-{delta}",
                self.table.id
            )))
            .focusable()
            .tab_index(0)
            .tab_stop(enabled)
            .aria_label(label.clone())
            .on_key_down(cx.listener(move |editor, event: &KeyDownEvent, _, cx| {
                if enabled && matches!(event.keystroke.key.as_str(), "enter" | "space") {
                    editor.resize(column, delta, cx);
                    cx.stop_propagation();
                }
            }))
            .child(
                Button::new(label)
                    .tertiary()
                    .disabled(!enabled)
                    .on_click(move |_, _, cx| {
                        owner.update(cx, |editor, cx| editor.resize(column, delta, cx));
                    }),
            )
            .into_any_element()
    }
}

impl Render for TableEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let columns = self.table.alignments.len();
        let body_rows = self.table.rows.iter().filter(|row| !row.header).count();
        let prefix = format!("table-{}", self.table.id);
        let mut column_controls = div().flex().gap_2();
        for column in 0..columns {
            column_controls = column_controls.child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .w(px(self.widths[column]))
                    .flex_shrink_0()
                    .child(format!(
                        "Column {} · {} px",
                        column + 1,
                        self.widths[column] as u32
                    ))
                    .child(self.action(
                        format!("{prefix}-insert-col-{column}"),
                        "Insert column before".into(),
                        MarkdownTableEdit::InsertColumn {
                            index: column,
                            cells: vec![String::new(); self.table.rows.len()],
                        },
                        true,
                    ))
                    .child(self.action(
                        format!("{prefix}-delete-col-{column}"),
                        "Delete column".into(),
                        MarkdownTableEdit::DeleteColumn { index: column },
                        columns > 1,
                    ))
                    .child(self.action(
                        format!("{prefix}-left-col-{column}"),
                        "Move column left".into(),
                        MarkdownTableEdit::MoveColumn {
                            from: column,
                            to: column.saturating_sub(1),
                        },
                        column > 0,
                    ))
                    .child(self.action(
                        format!("{prefix}-right-col-{column}"),
                        "Move column right".into(),
                        MarkdownTableEdit::MoveColumn {
                            from: column,
                            to: column + 1,
                        },
                        column + 1 < columns,
                    ))
                    .child(self.resize_control(column, -WIDTH_STEP, cx))
                    .child(self.resize_control(column, WIDTH_STEP, cx)),
            );
        }
        let mut table = div().flex().flex_col().gap_2().child(column_controls);
        let mut body_index = 0;
        for (row, snapshot) in self.table.rows.iter().enumerate() {
            let mut line = div().flex().gap_2().items_start();
            for input in &self.inputs[row] {
                line = line.child(input.clone());
            }
            if !snapshot.header {
                let index = body_index;
                body_index += 1;
                line = line.child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(self.action(
                            format!("{prefix}-insert-row-{row}"),
                            format!("Insert before row {}", index + 1),
                            MarkdownTableEdit::InsertRow {
                                index,
                                cells: vec![String::new(); columns],
                            },
                            true,
                        ))
                        .child(self.action(
                            format!("{prefix}-delete-row-{row}"),
                            format!("Delete row {}", index + 1),
                            MarkdownTableEdit::DeleteRow { index },
                            true,
                        ))
                        .child(self.action(
                            format!("{prefix}-up-row-{row}"),
                            format!("Move row {} up", index + 1),
                            MarkdownTableEdit::MoveRow {
                                from: index,
                                to: index.saturating_sub(1),
                            },
                            index > 0,
                        ))
                        .child(self.action(
                            format!("{prefix}-down-row-{row}"),
                            format!("Move row {} down", index + 1),
                            MarkdownTableEdit::MoveRow {
                                from: index,
                                to: index + 1,
                            },
                            index + 1 < body_rows,
                        )),
                );
            }
            table = table.child(line);
        }
        div()
            .id(SharedString::from(prefix.clone()))
            .flex()
            .flex_col()
            .gap_2()
            .p_2()
            .border_1()
            .border_color(theme.neutral.border)
            .child(div().text_xs().text_color(theme.neutral.text_3).child(
                "Cell edits replace inline formatting with plain text. Widths are visual only.",
            ))
            .child(
                div()
                    .id(SharedString::from(format!("{prefix}-scroll")))
                    .overflow_x_scroll()
                    .child(table),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(self.action(
                        format!("{prefix}-add-row"),
                        "Add row".into(),
                        MarkdownTableEdit::InsertRow {
                            index: body_rows,
                            cells: vec![String::new(); columns],
                        },
                        true,
                    ))
                    .child(self.action(
                        format!("{prefix}-add-column"),
                        "Add column".into(),
                        MarkdownTableEdit::InsertColumn {
                            index: columns,
                            cells: vec![String::new(); self.table.rows.len()],
                        },
                        true,
                    )),
            )
    }
}
