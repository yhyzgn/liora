//! Native GPUI todo-list controls.
//!
//! [`TodoItem`] owns one task's interactive state, while [`TodoList`] wires
//! several items together and reports changes with stable item IDs. The
//! renderer hooks intentionally receive a value context instead of an
//! `AnyElement` stored across renders, so custom content remains safe when
//! GPUI re-renders an entity after a state change.

use std::sync::Arc;

use gpui::{
    AnyElement, App, Context, EventEmitter, FocusHandle, Focusable, KeyBinding, MouseButton,
    Render, SharedString, Window, prelude::*, px,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

gpui::actions!(
    todo,
    [
        #[doc = "Keyboard action that toggles the focused todo item."]
        TodoItemToggle
    ]
);

/// Snapshot passed to [`TodoItem`] custom renderers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TodoItemRenderContext {
    /// Stable item identifier.
    pub id: SharedString,
    /// Main task title.
    pub title: SharedString,
    /// Optional supporting description.
    pub description: Option<SharedString>,
    /// Current completion state.
    pub checked: bool,
    /// Whether user interaction is disabled.
    pub disabled: bool,
}

/// Data used to create a stateful [`TodoItem`] entity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TodoItemData {
    /// Stable item identifier.
    pub id: SharedString,
    /// Main task title.
    pub title: SharedString,
    /// Optional supporting description.
    pub description: Option<SharedString>,
    /// Initial completion state.
    pub checked: bool,
    /// Initial disabled state.
    pub disabled: bool,
}

impl TodoItemData {
    /// Creates an enabled, incomplete task.
    pub fn new(id: impl Into<SharedString>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: None,
            checked: false,
            disabled: false,
        }
    }

    /// Sets supporting text shown below the title by the default renderer.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the initial completion state.
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Sets the initial disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Change event emitted by an interactive [`TodoItem`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TodoItemChanged {
    /// Stable ID of the changed item.
    pub id: SharedString,
    /// New completion state.
    pub checked: bool,
}

/// Stateful native GPUI todo item.
pub struct TodoItem {
    id: SharedString,
    title: SharedString,
    description: Option<SharedString>,
    checked: bool,
    disabled: bool,
    focus_handle: FocusHandle,
    leading_renderer: Option<Arc<dyn Fn(TodoItemRenderContext) -> AnyElement + 'static>>,
    check_renderer: Option<Arc<dyn Fn(TodoItemRenderContext) -> AnyElement + 'static>>,
    content_renderer: Option<Arc<dyn Fn(TodoItemRenderContext) -> AnyElement + 'static>>,
    trailing_renderer: Option<Arc<dyn Fn(TodoItemRenderContext) -> AnyElement + 'static>>,
    on_change: Option<Box<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
}

impl EventEmitter<TodoItemChanged> for TodoItem {}

impl TodoItem {
    /// Creates a todo item with a stable ID and title.
    pub fn new(
        id: impl Into<SharedString>,
        title: impl Into<SharedString>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::from_data(TodoItemData::new(id, title), cx)
    }

    /// Creates an item from reusable data.
    pub fn from_data(data: TodoItemData, cx: &mut Context<Self>) -> Self {
        Self {
            id: data.id,
            title: data.title,
            description: data.description,
            checked: data.checked,
            disabled: data.disabled,
            focus_handle: cx.focus_handle(),
            leading_renderer: None,
            check_renderer: None,
            content_renderer: None,
            trailing_renderer: None,
            on_change: None,
        }
    }

    /// Sets supporting text shown by the default content renderer.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the initial/current completion state.
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Sets whether user interaction is disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Replaces the optional leading slot with a render callback.
    pub fn leading_renderer(
        mut self,
        renderer: impl Fn(TodoItemRenderContext) -> AnyElement + 'static,
    ) -> Self {
        self.leading_renderer = Some(Arc::new(renderer));
        self
    }

    /// Replaces the checkbox visual with a render callback.
    pub fn check_renderer(
        mut self,
        renderer: impl Fn(TodoItemRenderContext) -> AnyElement + 'static,
    ) -> Self {
        self.check_renderer = Some(Arc::new(renderer));
        self
    }

    /// Replaces the default title/description block with a render callback.
    pub fn content_renderer(
        mut self,
        renderer: impl Fn(TodoItemRenderContext) -> AnyElement + 'static,
    ) -> Self {
        self.content_renderer = Some(Arc::new(renderer));
        self
    }

    /// Replaces the optional trailing slot with a render callback.
    pub fn trailing_renderer(
        mut self,
        renderer: impl Fn(TodoItemRenderContext) -> AnyElement + 'static,
    ) -> Self {
        self.trailing_renderer = Some(Arc::new(renderer));
        self
    }

    /// Registers a callback that runs after the item changes.
    pub fn on_change(mut self, callback: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(callback));
        self
    }

    /// Returns the stable item identifier.
    pub fn id(&self) -> SharedString {
        self.id.clone()
    }

    /// Returns the current title.
    pub fn title(&self) -> SharedString {
        self.title.clone()
    }

    /// Replaces the title while preserving the item entity identity.
    pub fn set_title(&mut self, title: impl Into<SharedString>, cx: &mut Context<Self>) {
        let title = title.into();
        if self.title != title {
            self.title = title;
            cx.notify();
        }
    }

    /// Replaces the supporting description while preserving the item entity identity.
    pub fn set_description(
        &mut self,
        description: Option<impl Into<SharedString>>,
        cx: &mut Context<Self>,
    ) {
        let description = description.map(Into::into);
        if self.description != description {
            self.description = description;
            cx.notify();
        }
    }

    /// Returns the current checked state.
    pub fn is_checked(&self) -> bool {
        self.checked
    }

    /// Returns whether interaction is disabled.
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// Updates completion state without emitting a user-change event.
    pub fn set_checked(&mut self, checked: bool, cx: &mut Context<Self>) {
        if self.checked != checked {
            self.checked = checked;
            cx.notify();
        }
    }

    /// Updates disabled state while preserving entity identity.
    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        if self.disabled != disabled {
            self.disabled = disabled;
            cx.notify();
        }
    }

    /// Registers keyboard bindings required by todo item interaction.
    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([
            KeyBinding::new("space", TodoItemToggle, None),
            KeyBinding::new("enter", TodoItemToggle, None),
        ]);
    }

    fn render_context(&self) -> TodoItemRenderContext {
        TodoItemRenderContext {
            id: self.id.clone(),
            title: self.title.clone(),
            description: self.description.clone(),
            checked: self.checked,
            disabled: self.disabled,
        }
    }

    fn toggle(&mut self, _: &TodoItemToggle, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        self.checked = !self.checked;
        let checked = self.checked;
        cx.emit(TodoItemChanged {
            id: self.id.clone(),
            checked,
        });
        cx.notify();
        if let Some(callback) = &self.on_change {
            callback(checked, window, cx);
        }
    }

    fn default_check(&self, theme: &liora_theme::Theme) -> AnyElement {
        let checked = self.checked;
        let (background, border, check_color) = if self.disabled {
            (
                theme.neutral.hover,
                theme.neutral.border,
                theme.neutral.text_disabled,
            )
        } else if checked {
            (
                theme.primary.base,
                theme.primary.base,
                theme.neutral.inverted,
            )
        } else {
            (
                gpui::transparent_black(),
                theme.neutral.border,
                gpui::transparent_black(),
            )
        };

        let mut indicator = gpui::div()
            .flex_none()
            .w(px(16.0))
            .h(px(16.0))
            .rounded(px(3.0))
            .bg(background)
            .border_1()
            .border_color(border)
            .flex()
            .items_center()
            .justify_center();
        if checked {
            indicator =
                indicator.child(Icon::new(IconName::Check).size(px(12.0)).color(check_color));
        }
        indicator.into_any_element()
    }
}

impl Focusable for TodoItem {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for TodoItem {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let context = self.render_context();
        let mut row = gpui::div()
            .id(self.id.clone())
            .flex()
            .flex_row()
            .items_center()
            .gap_3()
            .py_2()
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::toggle));

        if self.disabled {
            row = row.cursor_not_allowed().opacity(0.64);
        } else {
            row = row.cursor_pointer().on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| {
                    window.focus(&this.focus_handle, cx);
                }),
            );
            row = row.on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| {
                    this.toggle(&TodoItemToggle, window, cx);
                }),
            );
        }

        if let Some(renderer) = &self.leading_renderer {
            row = row.child(renderer(context.clone()));
        }

        let check = self
            .check_renderer
            .as_ref()
            .map(|renderer| renderer(context.clone()))
            .unwrap_or_else(|| self.default_check(&theme));
        row = row.child(check);

        if let Some(renderer) = &self.content_renderer {
            row = row.child(renderer(context.clone()));
        } else {
            let title_color = if self.checked {
                theme.neutral.text_3
            } else {
                theme.neutral.text_1
            };
            let mut title = gpui::div()
                .text_size(px(theme.font_size.md))
                .text_color(title_color)
                .child(self.title.clone());
            if self.checked {
                title = title.opacity(0.72);
            }
            let content = gpui::div()
                .flex()
                .flex_col()
                .gap_1()
                .flex_1()
                .child(title)
                .when_some(self.description.clone(), |element, description| {
                    element.child(
                        gpui::div()
                            .text_size(px(theme.font_size.sm))
                            .text_color(theme.neutral.text_3)
                            .child(description),
                    )
                });
            row = row.child(content);
        }

        if let Some(renderer) = &self.trailing_renderer {
            row = row.child(renderer(context));
        }
        row
    }
}

/// Change event emitted by a [`TodoList`] when one of its items changes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TodoListChanged {
    /// Stable ID of the changed item.
    pub id: SharedString,
    /// New completion state.
    pub checked: bool,
}

/// Kind of structural or content mutation emitted by a [`TodoList`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TodoListMutationKind {
    /// A new item was inserted.
    Added,
    /// An item was removed.
    Removed,
    /// An item's title, description, or disabled state changed.
    Updated,
    /// An item's completion state changed.
    Checked,
}

/// Mutation event emitted by a [`TodoList`] for dynamic CRUD operations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TodoListMutation {
    /// Stable ID of the affected item.
    pub id: SharedString,
    /// Mutation category.
    pub kind: TodoListMutationKind,
    /// Completion state when the mutation concerns a checked item.
    pub checked: Option<bool>,
}

/// Native GPUI list that groups interactive [`TodoItem`] entities.
pub struct TodoList {
    items: Vec<gpui::Entity<TodoItem>>,
    gap: gpui::Pixels,
    on_change: Option<Box<dyn Fn(TodoListChanged, &mut Context<Self>) + 'static>>,
    on_mutation: Option<Box<dyn Fn(TodoListMutation, &mut Context<Self>) + 'static>>,
}

impl EventEmitter<TodoListChanged> for TodoList {}
impl EventEmitter<TodoListMutation> for TodoList {}

impl TodoList {
    /// Creates an empty todo list.
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            items: Vec::new(),
            gap: px(4.0),
            on_change: None,
            on_mutation: None,
        }
    }

    /// Creates a list and inserts items from data.
    pub fn from_data(items: Vec<TodoItemData>, cx: &mut Context<Self>) -> Self {
        let mut list = Self::new(cx);
        for data in items {
            let item = cx.new(|cx| TodoItem::from_data(data, cx));
            list.push(item, cx);
        }
        list
    }

    /// Adds an existing todo item entity and subscribes to its changes.
    pub fn push(&mut self, item: gpui::Entity<TodoItem>, cx: &mut Context<Self>) {
        let id = item.read(cx).id();
        cx.subscribe(&item, |this, _item, event: &TodoItemChanged, cx| {
            this.emit_checked(event.id.as_ref(), event.checked, cx);
            cx.notify();
        })
        .detach();
        self.items.push(item);
        self.emit_mutation(
            TodoListMutation {
                id,
                kind: TodoListMutationKind::Added,
                checked: None,
            },
            cx,
        );
        cx.notify();
    }

    /// Creates and inserts an item from data, returning its entity handle.
    pub fn push_data(
        &mut self,
        data: TodoItemData,
        cx: &mut Context<Self>,
    ) -> gpui::Entity<TodoItem> {
        let item = cx.new(|cx| TodoItem::from_data(data, cx));
        self.push(item.clone(), cx);
        item
    }

    /// Returns the item with the supplied stable ID.
    pub fn item(&self, id: &str, cx: &App) -> Option<gpui::Entity<TodoItem>> {
        self.items
            .iter()
            .find(|item| item.read(cx).id().as_ref() == id)
            .cloned()
    }

    /// Alias for [`TodoList::item`] that reads naturally in lookup-heavy code.
    pub fn find(&self, id: &str, cx: &App) -> Option<gpui::Entity<TodoItem>> {
        self.item(id, cx)
    }

    /// Removes and returns an item by stable ID.
    pub fn remove(&mut self, id: &str, cx: &mut Context<Self>) -> Option<gpui::Entity<TodoItem>> {
        let index = self
            .items
            .iter()
            .position(|item| item.read(cx).id().as_ref() == id)?;
        let item = self.items.remove(index);
        self.emit_mutation(
            TodoListMutation {
                id: id.into(),
                kind: TodoListMutationKind::Removed,
                checked: None,
            },
            cx,
        );
        cx.notify();
        Some(item)
    }

    /// Removes every item and returns the removed entity handles.
    pub fn clear(&mut self, cx: &mut Context<Self>) -> Vec<gpui::Entity<TodoItem>> {
        let items = std::mem::take(&mut self.items);
        for item in &items {
            let id = item.read(cx).id();
            self.emit_mutation(
                TodoListMutation {
                    id,
                    kind: TodoListMutationKind::Removed,
                    checked: None,
                },
                cx,
            );
        }
        cx.notify();
        items
    }

    /// Replaces an item's title. Returns `false` when no item has that ID.
    pub fn set_title(
        &mut self,
        id: &str,
        title: impl Into<SharedString>,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(item) = self.item(id, cx) else {
            return false;
        };
        let title = title.into();
        if item.read(cx).title() == title {
            return true;
        }
        item.update(cx, |item, cx| item.set_title(title, cx));
        self.emit_updated(id, cx);
        true
    }

    /// Replaces an item's description. Pass `None` to clear it.
    pub fn set_description(
        &mut self,
        id: &str,
        description: Option<SharedString>,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(item) = self.item(id, cx) else {
            return false;
        };
        if item.read(cx).description.as_ref() == description.as_ref() {
            return true;
        }
        item.update(cx, |item, cx| item.set_description(description, cx));
        self.emit_updated(id, cx);
        true
    }

    /// Replaces an item's completion state without synthesizing a user toggle.
    pub fn set_checked(&mut self, id: &str, checked: bool, cx: &mut Context<Self>) -> bool {
        let Some(item) = self.item(id, cx) else {
            return false;
        };
        if item.read(cx).is_checked() == checked {
            return true;
        }
        item.update(cx, |item, cx| item.set_checked(checked, cx));
        self.emit_checked(id, checked, cx);
        true
    }

    /// Replaces an item's disabled state.
    pub fn set_disabled(&mut self, id: &str, disabled: bool, cx: &mut Context<Self>) -> bool {
        let Some(item) = self.item(id, cx) else {
            return false;
        };
        if item.read(cx).is_disabled() == disabled {
            return true;
        }
        item.update(cx, |item, cx| item.set_disabled(disabled, cx));
        self.emit_updated(id, cx);
        true
    }

    /// Sets vertical spacing between items.
    pub fn gap(mut self, gap: impl Into<gpui::Pixels>) -> Self {
        self.gap = gap.into();
        self
    }

    /// Registers a callback for changes from any list item.
    pub fn on_change(
        mut self,
        callback: impl Fn(TodoListChanged, &mut Context<Self>) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(callback));
        self
    }

    /// Registers a callback for insert, remove, update, and checked mutations.
    pub fn on_mutation(
        mut self,
        callback: impl Fn(TodoListMutation, &mut Context<Self>) + 'static,
    ) -> Self {
        self.on_mutation = Some(Box::new(callback));
        self
    }

    /// Returns the number of items in the list.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns whether the list has no items.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns a snapshot of the current item handles.
    pub fn items(&self) -> Vec<gpui::Entity<TodoItem>> {
        self.items.clone()
    }

    /// Registers keyboard bindings for all todo items.
    pub fn register_key_bindings(cx: &mut App) {
        TodoItem::register_key_bindings(cx);
    }

    fn emit_updated(&mut self, id: &str, cx: &mut Context<Self>) {
        self.emit_mutation(
            TodoListMutation {
                id: id.into(),
                kind: TodoListMutationKind::Updated,
                checked: None,
            },
            cx,
        );
    }

    fn emit_checked(&mut self, id: &str, checked: bool, cx: &mut Context<Self>) {
        let changed = TodoListChanged {
            id: id.into(),
            checked,
        };
        cx.emit(changed.clone());
        if let Some(callback) = &self.on_change {
            callback(changed, cx);
        }
        self.emit_mutation(
            TodoListMutation {
                id: id.into(),
                kind: TodoListMutationKind::Checked,
                checked: Some(checked),
            },
            cx,
        );
    }

    fn emit_mutation(&mut self, mutation: TodoListMutation, cx: &mut Context<Self>) {
        cx.emit(mutation.clone());
        if let Some(callback) = &self.on_mutation {
            callback(mutation, cx);
        }
    }
}

impl Render for TodoList {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        gpui::div()
            .flex()
            .flex_col()
            .gap(self.gap)
            .children(self.items.iter().cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_builder_preserves_stable_identity_and_state() {
        let data = TodoItemData::new("build", "Build the release")
            .description("Run the native checks")
            .checked(true)
            .disabled(true);

        assert_eq!(data.id, "build");
        assert_eq!(data.title, "Build the release");
        assert_eq!(data.description.as_deref(), Some("Run the native checks"));
        assert!(data.checked);
        assert!(data.disabled);
    }

    #[test]
    fn source_exposes_custom_slots_and_stateful_list_wiring() {
        let source = include_str!("todo.rs");
        for required in [
            "pub struct TodoItem",
            "pub struct TodoList",
            "pub fn leading_renderer",
            "pub fn check_renderer",
            "pub fn content_renderer",
            "pub fn trailing_renderer",
            "cx.subscribe(",
            "TodoItemChanged",
            "TodoItemToggle",
            "pub fn push_data",
            "pub fn remove",
            "pub fn clear",
            "pub fn set_title",
            "pub fn set_description",
            "pub fn set_checked",
            "pub fn set_disabled",
            "pub fn on_mutation",
            "TodoListMutationKind",
        ] {
            assert!(source.contains(required), "missing {required}");
        }
    }
}
