//! Native command palette with stable command identities and GPUI action dispatch.

use crate::{
    Input, MenuItem,
    input::{InputDown, InputUp},
};
use gpui::{
    Action, AnyElement, App, Context, Entity, FocusHandle, Focusable, KeyBinding, MouseButton,
    Render, ScrollStrategy, SharedString, UniformListScrollHandle, Window, actions, div,
    prelude::*, px, uniform_list,
};
use liora_core::{Config, LocalizedText, push_portal};
use liora_icons_lucide::IconName;
use std::rc::Rc;

actions!(
    command_palette,
    [
        #[doc = "GPUI action used to close an open command palette with Escape."]
        CommandPaletteEscape
    ]
);

type Availability = dyn Fn(&App) -> bool + 'static;
type ExecuteCallback = dyn Fn(&CommandPaletteId, &mut Window, &mut App) + 'static;

/// Stable application-owned command identity shared with menu descriptors and dispatchers.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CommandPaletteId(SharedString);

impl CommandPaletteId {
    /// Creates a command id. Callers should use a stable, unique value such as `document.save`.
    pub fn new(id: impl Into<SharedString>) -> Self {
        Self(id.into())
    }

    /// Returns the stable id string.
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<&str> for CommandPaletteId {
    fn from(id: &str) -> Self {
        Self::new(id)
    }
}

impl From<String> for CommandPaletteId {
    fn from(id: String) -> Self {
        Self::new(id)
    }
}

impl From<SharedString> for CommandPaletteId {
    fn from(id: SharedString) -> Self {
        Self::new(id)
    }
}

/// Metadata and dispatch target for one command palette entry.
pub struct CommandPaletteCommand {
    /// Stable id used as the command's identity in palettes and menus.
    pub id: CommandPaletteId,
    /// Localized label shown as the command title.
    pub label: LocalizedText,
    /// Optional localized explanation shown below the label.
    pub description: Option<LocalizedText>,
    /// Optional keyboard shortcut displayed beside the command.
    pub shortcut: Option<SharedString>,
    /// Whether the command is explicitly disabled.
    pub disabled: bool,
    availability: Option<Rc<Availability>>,
    action: Option<Box<dyn Action>>,
    on_execute: Option<Rc<ExecuteCallback>>,
}

impl Clone for CommandPaletteCommand {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            label: self.label.clone(),
            description: self.description.clone(),
            shortcut: self.shortcut.clone(),
            disabled: self.disabled,
            availability: self.availability.clone(),
            action: self.action.as_ref().map(|action| action.boxed_clone()),
            on_execute: self.on_execute.clone(),
        }
    }
}

impl CommandPaletteCommand {
    /// Creates a command with a caller-supplied stable id and user-facing label.
    pub fn new(id: impl Into<CommandPaletteId>, label: impl Into<LocalizedText>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            shortcut: None,
            disabled: false,
            availability: None,
            action: None,
            on_execute: None,
        }
    }

    /// Copies a menu item's identity and display metadata for use in a palette.
    ///
    /// Attach the same GPUI action with [`Self::with_action`] or use the same host
    /// callback as the menu. Separators and submenu parents are not commands.
    pub fn from_menu_item(item: &MenuItem) -> Self {
        Self {
            id: CommandPaletteId::new(item.id.clone()),
            label: item.label.clone(),
            description: None,
            shortcut: item.shortcut.clone(),
            disabled: item.disabled || item.separator || item.has_children(),
            availability: None,
            action: None,
            on_execute: None,
        }
    }

    /// Sets localized secondary command text.
    pub fn description(mut self, description: impl Into<LocalizedText>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the shortcut text shown beside this command.
    pub fn shortcut(mut self, shortcut: impl Into<SharedString>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Explicitly disables this command while keeping it visible in search results.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Enables the command only while the supplied host-state predicate returns true.
    pub fn available_when(mut self, predicate: impl Fn(&App) -> bool + 'static) -> Self {
        self.availability = Some(Rc::new(predicate));
        self
    }

    /// Dispatches the same GPUI action type used by the host's menus and key bindings.
    pub fn with_action<A: Action + 'static>(mut self, action: A) -> Self {
        self.action = Some(Box::new(action));
        self.on_execute = None;
        self
    }

    /// Registers a direct host callback for this command instead of an action.
    pub fn on_execute(
        mut self,
        callback: impl Fn(&CommandPaletteId, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.action = None;
        self.on_execute = Some(Rc::new(callback));
        self
    }

    /// Returns whether the command has a dispatch target and can currently be invoked.
    pub fn is_available(&self, cx: &App) -> bool {
        !self.disabled
            && (self.action.is_some() || self.on_execute.is_some())
            && self
                .availability
                .as_ref()
                .is_none_or(|available| available(cx))
    }
}

struct RankedCommand {
    command: Rc<CommandPaletteCommand>,
    label: SharedString,
    description: Option<SharedString>,
    enabled: bool,
    score: i64,
    source_index: usize,
}

/// Stateful, native GPUI command search overlay.
pub struct CommandPalette {
    id: SharedString,
    commands: Vec<Rc<CommandPaletteCommand>>,
    input: Entity<Input>,
    query: SharedString,
    active_id: Option<CommandPaletteId>,
    is_open: bool,
    previous_focus: Option<FocusHandle>,
    scroll_handle: UniformListScrollHandle,
}

impl CommandPalette {
    /// Creates a palette for the supplied commands.
    pub fn new(commands: Vec<CommandPaletteCommand>, cx: &mut Context<Self>) -> Self {
        let owner = cx.entity().downgrade();
        let input = cx.new(move |cx| {
            let on_change_owner = owner.clone();
            let on_enter_owner = owner.clone();
            Input::new("", cx)
                .id(liora_core::unique_id("command-palette-query"))
                .placeholder("Search commands...")
                .icon_prefix(IconName::Search)
                .on_change(move |query, cx| {
                    let owner = on_change_owner.clone();
                    let query = SharedString::from(query.to_owned());
                    cx.defer(move |cx| {
                        let _ = owner.update(cx, |palette, cx| palette.update_query(query, cx));
                    });
                })
                .on_enter(move |_, _, window, cx| {
                    let owner = on_enter_owner.clone();
                    window.defer(cx, move |window, cx| {
                        let _ = owner.update(cx, |palette, cx| palette.execute_active(window, cx));
                    });
                })
        });

        let mut palette = Self {
            id: liora_core::unique_id("command-palette"),
            commands: commands.into_iter().map(Rc::new).collect(),
            input,
            query: SharedString::default(),
            active_id: None,
            is_open: false,
            previous_focus: None,
            scroll_handle: UniformListScrollHandle::new(),
        };
        palette.active_id = palette
            .commands
            .iter()
            .find(|command| command.is_available(cx))
            .map(|command| command.id.clone());
        palette
    }

    /// Creates a GPUI entity that keeps palette state between render passes.
    pub fn entity(commands: Vec<CommandPaletteCommand>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(commands, cx))
    }

    /// Registers the context-scoped Escape binding used to dismiss this palette.
    /// Call once during app initialization, normally alongside `init_liora`.
    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([KeyBinding::new(
            "escape",
            CommandPaletteEscape,
            Some("CommandPalette"),
        )]);
    }

    /// Replaces the available commands and makes the best available match active.
    pub fn set_commands(&mut self, commands: Vec<CommandPaletteCommand>, cx: &mut Context<Self>) {
        debug_assert!(
            commands
                .iter()
                .map(|c| &c.id)
                .collect::<std::collections::HashSet<_>>()
                .len()
                == commands.len(),
            "duplicate CommandPaletteId within one palette"
        );
        self.commands = commands.into_iter().map(Rc::new).collect();
        self.select_first_available(cx);
        cx.notify();
    }

    /// Refreshes availability predicates after host state changes.
    pub fn refresh(&mut self, cx: &mut Context<Self>) {
        let still_active = self.active_id.as_ref().is_some_and(|active_id| {
            self.ranked_commands(cx)
                .iter()
                .any(|candidate| candidate.enabled && &candidate.command.id == active_id)
        });
        if !still_active {
            self.select_first_available(cx);
        }
        cx.notify();
    }

    /// Sets the current query and selects the best available result.
    pub fn set_query(&mut self, query: impl Into<SharedString>, cx: &mut Context<Self>) {
        let query = query.into();
        if self.query == query {
            return;
        }
        self.input
            .update(cx, |input, cx| input.set_value(query.clone(), cx));
        self.update_query(query, cx);
    }

    /// Returns the active command id, if an available match is selected.
    pub fn active_command_id(&self) -> Option<&CommandPaletteId> {
        self.active_id.as_ref()
    }

    /// Returns whether the overlay is currently open.
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    /// Opens the palette, focuses its search input, and remembers the prior focus target.
    pub fn open(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let input_focus = self.input.focus_handle(cx);
        if let Some(focused) = window.focused(cx).filter(|focused| focused != &input_focus) {
            self.previous_focus = Some(focused);
        }
        self.is_open = true;
        self.query = SharedString::default();
        self.input.update(cx, |input, cx| input.set_value("", cx));
        self.scroll_handle.scroll_to_item(0, ScrollStrategy::Top);
        self.select_first_available(cx);
        let input_focus = self.input.focus_handle(cx);
        window.defer(cx, move |window, cx| window.focus(&input_focus, cx));
        cx.notify();
    }

    /// Closes the palette and restores the focus target captured by [`Self::open`].
    pub fn close(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.dismiss(None, window, cx);
    }

    fn update_query(&mut self, query: SharedString, cx: &mut Context<Self>) {
        if self.query == query {
            return;
        }
        self.query = query;
        self.select_first_available(cx);
        cx.notify();
    }

    fn ranked_commands(&self, cx: &App) -> Vec<RankedCommand> {
        let mut ranked = self
            .commands
            .iter()
            .enumerate()
            .filter_map(|(source_index, command)| {
                let label = command.label.resolve(cx);
                let description = command.description.as_ref().map(|text| text.resolve(cx));
                let score = command_score(
                    self.query.as_ref(),
                    &label,
                    description.as_deref(),
                    command.id.as_str(),
                    command.shortcut.as_deref(),
                )?;
                Some(RankedCommand {
                    command: command.clone(),
                    label,
                    description,
                    enabled: command.is_available(cx),
                    score,
                    source_index,
                })
            })
            .collect::<Vec<_>>();
        ranked.sort_by(|left, right| {
            right
                .score
                .cmp(&left.score)
                .then_with(|| left.source_index.cmp(&right.source_index))
        });
        ranked
    }

    fn select_first_available(&mut self, cx: &App) {
        let ranked = self.ranked_commands(cx);
        let active = ranked.iter().position(|candidate| candidate.enabled);
        self.active_id = active.map(|index| ranked[index].command.id.clone());
        if let Some(index) = active {
            self.scroll_handle
                .scroll_to_item(index, ScrollStrategy::Nearest);
        }
    }

    fn move_active(&mut self, delta: isize, cx: &App) {
        let ranked = self.ranked_commands(cx);
        let enabled = ranked
            .iter()
            .enumerate()
            .filter_map(|(index, candidate)| candidate.enabled.then_some(index))
            .collect::<Vec<_>>();
        if enabled.is_empty() {
            self.active_id = None;
            return;
        }

        let current = enabled.iter().position(|index| {
            self.active_id
                .as_ref()
                .is_some_and(|active| ranked[*index].command.id == *active)
        });
        let next = current
            .map(|index| (index as isize + delta).rem_euclid(enabled.len() as isize) as usize)
            .unwrap_or_else(|| if delta < 0 { enabled.len() - 1 } else { 0 });
        let result_index = enabled[next];
        self.active_id = Some(ranked[result_index].command.id.clone());
        self.scroll_handle
            .scroll_to_item(result_index, ScrollStrategy::Nearest);
    }

    fn execute_active(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(active_id) = self.active_id.clone() else {
            return;
        };
        self.execute_command(&active_id, window, cx);
    }

    fn execute_command(
        &mut self,
        id: &CommandPaletteId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let command = self
            .ranked_commands(cx)
            .into_iter()
            .find(|candidate| candidate.enabled && &candidate.command.id == id)
            .map(|candidate| candidate.command);
        let Some(command) = command else {
            self.select_first_available(cx);
            cx.notify();
            return;
        };
        self.dismiss(Some(command.as_ref().clone()), window, cx);
    }

    fn dismiss(
        &mut self,
        command: Option<CommandPaletteCommand>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.is_open {
            return;
        }
        self.is_open = false;
        self.query = SharedString::default();
        self.active_id = None;
        self.input.update(cx, |input, cx| input.set_value("", cx));
        let previous_focus = self.previous_focus.take();
        let dispatch = command.map(|command| (command.id, command.action, command.on_execute));
        cx.notify();
        window.defer(cx, move |window, cx| {
            if let Some(previous_focus) = previous_focus {
                window.focus(&previous_focus, cx);
            }
            match dispatch {
                Some((_, Some(action), _)) => window.dispatch_action(action, cx),
                Some((id, None, Some(callback))) => callback(&id, window, cx),
                _ => {}
            }
        });
    }
}

impl Render for CommandPalette {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_open {
            let owner = cx.entity().downgrade();
            let input = self.input.clone();
            let commands = self.ranked_commands(cx);
            let active_id = self.active_id.clone();
            let scroll_handle = self.scroll_handle.clone();
            let id = self.id.clone();
            push_portal(
                move |window, cx| {
                    let theme = cx.global::<Config>().theme.clone();
                    let viewport = window.viewport_size();
                    let max_width = (viewport.width - px(32.0)).max(px(0.0));
                    let list_height = px((commands.len().min(6) as f32 * 58.0).max(58.0));
                    let mut panel = div()
                        .w_full()
                        .max_w(px(560.0).min(max_width))
                        .overflow_hidden()
                        .rounded(px(theme.radius.md))
                        .border_1()
                        .border_color(theme.neutral.border)
                        .bg(theme.neutral.card)
                        .shadow_lg()
                        .occlude()
                        .on_mouse_down_out({
                            let owner = owner.clone();
                            move |_, window, cx| {
                                let _ = owner.update(cx, |palette, cx| palette.close(window, cx));
                            }
                        })
                        .child(div().p_3().child(input));

                    if commands.is_empty() {
                        panel = panel.child(
                            div()
                                .px_3()
                                .py_4()
                                .text_sm()
                                .text_color(theme.neutral.text_3)
                                .child(
                                    if cx.global::<Config>().locales.locale.as_str() == "zh-CN" {
                                        "没有匹配的命令"
                                    } else {
                                        "No matching commands"
                                    },
                                ),
                        );
                    } else {
                        let row_owner = owner.clone();
                        let row_active_id = active_id.clone();
                        let row_id = id.clone();
                        let list = uniform_list(
                            format!("{id}-results"),
                            commands.len(),
                            move |range, _, _| {
                                range
                                    .map(|index| {
                                        let candidate = &commands[index];
                                        let command = candidate.command.clone();
                                        let command_id = command.id.clone();
                                        let enabled = candidate.enabled;
                                        let selected = row_active_id.as_ref() == Some(&command_id);
                                        let label = candidate.label.clone();
                                        let description = candidate.description.clone();
                                        let shortcut = command.shortcut.clone();
                                        let hover = theme.neutral.hover;
                                        let row_owner = row_owner.clone();
                                        let mut row = div()
                                            .id(format!("{row_id}-{}", command_id.as_str()))
                                            .w_full()
                                            .h(px(58.0))
                                            .px_3()
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .gap_3()
                                            .bg(if selected {
                                                theme.primary.base.opacity(0.10)
                                            } else {
                                                theme.neutral.card
                                            })
                                            .text_color(if enabled {
                                                theme.neutral.text_1
                                            } else {
                                                theme.neutral.text_disabled
                                            })
                                            .child(
                                                div()
                                                    .flex_1()
                                                    .min_w(px(0.0))
                                                    .flex()
                                                    .flex_col()
                                                    .gap_1()
                                                    .child(div().text_sm().child(label))
                                                    .when_some(description, |el, description| {
                                                        el.child(
                                                            div()
                                                                .text_xs()
                                                                .text_color(theme.neutral.text_3)
                                                                .child(description),
                                                        )
                                                    }),
                                            )
                                            .when_some(shortcut, |el, shortcut| {
                                                el.child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(theme.neutral.text_3)
                                                        .child(shortcut),
                                                )
                                            });
                                        if enabled {
                                            row = row
                                                .cursor_pointer()
                                                .hover(move |el| el.bg(hover))
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    move |_, window, cx| {
                                                        let command_id = command_id.clone();
                                                        let _ =
                                                            row_owner.update(cx, |palette, cx| {
                                                                palette.execute_command(
                                                                    &command_id,
                                                                    window,
                                                                    cx,
                                                                );
                                                            });
                                                        cx.stop_propagation();
                                                    },
                                                );
                                        } else {
                                            row = row.cursor_not_allowed();
                                        }
                                        row.into_any_element()
                                    })
                                    .collect::<Vec<AnyElement>>()
                            },
                        )
                        .track_scroll(&scroll_handle)
                        .h(list_height);
                        panel = panel.child(list);
                    }

                    let overlay = div()
                        .id(format!("{id}-overlay"))
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .flex()
                        .items_start()
                        .justify_center()
                        .pt(px(72.0))
                        .key_context("CommandPalette")
                        .on_action({
                            let owner = owner.clone();
                            move |_: &CommandPaletteEscape, window, cx| {
                                let _ = owner.update(cx, |palette, cx| palette.close(window, cx));
                            }
                        })
                        .capture_action({
                            let owner = owner.clone();
                            move |_: &InputUp, _, cx| {
                                let _ = owner.update(cx, |palette, cx| {
                                    palette.move_active(-1, cx);
                                    cx.notify();
                                });
                                cx.stop_propagation();
                            }
                        })
                        .capture_action({
                            let owner = owner.clone();
                            move |_: &InputDown, _, cx| {
                                let _ = owner.update(cx, |palette, cx| {
                                    palette.move_active(1, cx);
                                    cx.notify();
                                });
                                cx.stop_propagation();
                            }
                        })
                        .child(panel);
                    overlay.into_any_element()
                },
                cx,
            );
        }
        div()
    }
}

fn command_score(
    query: &str,
    label: &str,
    description: Option<&str>,
    id: &str,
    shortcut: Option<&str>,
) -> Option<i64> {
    let mut tokens = query.split_whitespace().peekable();
    if tokens.peek().is_none() {
        return Some(0);
    }
    let mut total = 0;
    for token in tokens {
        let best = fuzzy_score(token, label)
            .map(|score| score * 4)
            .into_iter()
            .chain(fuzzy_score(token, description.unwrap_or_default()).map(|score| score * 2))
            .chain(fuzzy_score(token, id))
            .chain(fuzzy_score(token, shortcut.unwrap_or_default()))
            .max()?;
        total += best;
    }
    Some(total)
}

fn fuzzy_score(query: &str, candidate: &str) -> Option<i64> {
    let query = query.to_lowercase();
    let candidate = candidate.to_lowercase().chars().collect::<Vec<_>>();
    let mut cursor = 0;
    let mut previous = None;
    let mut score = 0i64;
    for character in query.chars() {
        let relative = candidate[cursor..]
            .iter()
            .position(|item| *item == character)?;
        let index = cursor + relative;
        score += 10 - relative as i64;
        if index == 0 {
            score += 12;
        } else if !candidate[index - 1].is_alphanumeric() {
            score += 8;
        }
        if previous.is_some_and(|previous| previous + 1 == index) {
            score += 10;
        }
        previous = Some(index);
        cursor = index + 1;
    }
    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuzzy_ranking_prefers_label_prefix_and_contiguous_matches() {
        let exact_prefix = command_score("op", "Open File", None, "open-file", None).unwrap();
        let subsequence = command_score("op", "File Operations", None, "operations", None).unwrap();
        assert!(exact_prefix > subsequence);
        assert!(fuzzy_score("sve", "Save Document").is_some());
        assert!(fuzzy_score("xyz", "Save Document").is_none());
    }

    #[test]
    fn every_query_token_must_match_command_metadata() {
        assert!(command_score("save doc", "Save Document", None, "save", None).is_some());
        assert!(command_score("save absent", "Save Document", None, "save", None).is_none());
    }

    #[test]
    fn menu_mapping_preserves_stable_identity_and_shortcut() {
        let item = MenuItem::new("document.save", "Save").shortcut("Ctrl+S");
        let command = CommandPaletteCommand::from_menu_item(&item);
        assert_eq!(command.id.as_str(), "document.save");
        assert_eq!(command.shortcut.as_deref(), Some("Ctrl+S"));
        assert!(!command.disabled);
    }
}
