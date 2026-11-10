//! Native GPUI todo-list controls.
//!
//! [`TodoItem`] owns one task's interactive state, while [`TodoList`] wires
//! several items together and reports changes with stable item IDs. The
//! renderer hooks intentionally receive a value context instead of an
//! `AnyElement` stored across renders, so custom content remains safe when
//! GPUI re-renders an entity after a state change.

use std::cmp::Ordering;
use std::collections::HashMap;
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
    /// Whether the item is currently rendered in its inline editor state.
    pub editing: bool,
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
    editing: bool,
    focus_handle: FocusHandle,
    leading_renderer: Option<Arc<dyn Fn(TodoItemRenderContext) -> AnyElement + 'static>>,
    check_renderer: Option<Arc<dyn Fn(TodoItemRenderContext) -> AnyElement + 'static>>,
    content_renderer: Option<Arc<dyn Fn(TodoItemRenderContext) -> AnyElement + 'static>>,
    editor_renderer: Option<Arc<dyn Fn(TodoItemRenderContext) -> AnyElement + 'static>>,
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
            editing: false,
            focus_handle: cx.focus_handle(),
            leading_renderer: None,
            check_renderer: None,
            content_renderer: None,
            editor_renderer: None,
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

    /// Replaces the default content with an inline editor while the item is editing.
    pub fn editor_renderer(
        mut self,
        renderer: impl Fn(TodoItemRenderContext) -> AnyElement + 'static,
    ) -> Self {
        self.editor_renderer = Some(Arc::new(renderer));
        self
    }

    /// Replaces the optional trailing slot with a render callback.
    /// This slot is outside the item's completion toggle area.
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

    /// Returns the current supporting description.
    pub fn description_value(&self) -> Option<SharedString> {
        self.description.clone()
    }

    /// Returns whether the item is currently being edited.
    pub fn is_editing(&self) -> bool {
        self.editing
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

    /// Switches the item between display and inline editor states.
    pub fn set_editing(&mut self, editing: bool, cx: &mut Context<Self>) {
        if self.editing != editing {
            self.editing = editing;
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
            editing: self.editing,
        }
    }

    fn toggle(&mut self, _: &TodoItemToggle, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled || self.editing {
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
            .flex_1()
            .min_w_0()
            .py_2()
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::toggle));

        if !self.disabled && !self.editing {
            row = row
                .cursor_pointer()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, window, cx| {
                        window.focus(&this.focus_handle, cx);
                        this.toggle(&TodoItemToggle, window, cx);
                    }),
                )
                .on_click(cx.listener(|this, event: &gpui::ClickEvent, window, cx| {
                    if event.is_keyboard() {
                        this.toggle(&TodoItemToggle, window, cx);
                    }
                }));
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

        if self.editing {
            if let Some(renderer) = &self.editor_renderer {
                row = row.child(renderer(context.clone()));
            }
        } else if let Some(renderer) = &self.content_renderer {
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

        gpui::div()
            .flex()
            .items_center()
            .gap_3()
            .when(self.disabled, |row| row.cursor_not_allowed().opacity(0.64))
            .child(row)
            .when_some(self.trailing_renderer.as_ref(), |row, renderer| {
                row.child(renderer(context))
            })
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

/// Counts passed to list-level composition hooks; filtering never changes totals.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TodoListRenderContext {
    /// Number of retained items, including hidden items.
    pub total: usize,
    /// Number of completed items, including hidden items.
    pub checked: usize,
    /// Number of items currently displayed.
    pub visible: usize,
}

/// Native GPUI list that groups interactive [`TodoItem`] entities.
pub struct TodoList {
    items: Vec<gpui::Entity<TodoItem>>,
    subscriptions: HashMap<
        SharedString,
        (
            gpui::Entity<TodoItem>,
            gpui::Subscription,
            gpui::Subscription,
        ),
    >,
    filter: Option<Box<dyn Fn(&TodoItem) -> bool + 'static>>,
    sort: Option<Box<dyn Fn(&TodoItem, &TodoItem) -> Ordering + 'static>>,
    header_renderer: Option<Box<dyn Fn(TodoListRenderContext) -> AnyElement + 'static>>,
    empty_renderer: Option<Box<dyn Fn(TodoListRenderContext) -> AnyElement + 'static>>,
    contextual_footer_renderer: Option<Box<dyn Fn(TodoListRenderContext) -> AnyElement + 'static>>,
    gap: gpui::Pixels,
    on_change: Option<Box<dyn Fn(TodoListChanged, &mut Context<Self>) + 'static>>,
    on_mutation: Option<Box<dyn Fn(TodoListMutation, &mut Context<Self>) + 'static>>,
    footer_renderer: Option<Arc<dyn Fn() -> AnyElement + 'static>>,
}

impl EventEmitter<TodoListChanged> for TodoList {}
impl EventEmitter<TodoListMutation> for TodoList {}

impl TodoList {
    /// Creates an empty todo list.
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            items: Vec::new(),
            subscriptions: HashMap::new(),
            filter: None,
            sort: None,
            header_renderer: None,
            empty_renderer: None,
            contextual_footer_renderer: None,
            gap: px(4.0),
            on_change: None,
            on_mutation: None,
            footer_renderer: None,
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

    /// Adds an existing item. Duplicate IDs are ignored (first entity wins).
    pub fn push(&mut self, item: gpui::Entity<TodoItem>, cx: &mut Context<Self>) {
        let id = item.read(cx).id();
        if self.subscriptions.contains_key(id.as_ref()) {
            return;
        }
        let changed = cx.subscribe(&item, |this, item, event: &TodoItemChanged, cx| {
            if this
                .subscriptions
                .get(event.id.as_ref())
                .is_some_and(|(current, _, _)| current == &item)
            {
                this.emit_checked(event.id.as_ref(), event.checked, cx);
            }
        });
        let observed = cx.observe(&item, |_, _, cx| cx.notify());
        self.subscriptions
            .insert(id.clone(), (item.clone(), changed, observed));
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

    /// Creates and inserts an item, or returns the retained entity for a duplicate ID.
    pub fn push_data(
        &mut self,
        data: TodoItemData,
        cx: &mut Context<Self>,
    ) -> gpui::Entity<TodoItem> {
        if let Some(item) = self.item(data.id.as_ref(), cx) {
            return item;
        }
        let item = cx.new(|cx| TodoItem::from_data(data, cx));
        self.push(item.clone(), cx);
        item
    }

    /// Returns the item with the supplied stable ID.
    pub fn item(&self, id: &str, _cx: &App) -> Option<gpui::Entity<TodoItem>> {
        self.subscriptions.get(id).map(|(item, _, _)| item.clone())
    }

    /// Alias for [`TodoList::item`] that reads naturally in lookup-heavy code.
    pub fn find(&self, id: &str, cx: &App) -> Option<gpui::Entity<TodoItem>> {
        self.item(id, cx)
    }

    /// Removes and returns an item by stable ID.
    pub fn remove(&mut self, id: &str, cx: &mut Context<Self>) -> Option<gpui::Entity<TodoItem>> {
        let (item, _, _) = self.subscriptions.remove(id)?;
        self.items.retain(|current| current != &item);
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
        self.subscriptions.clear();
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

    /// Switches an item between display and inline editor states.
    pub fn set_editing(&mut self, id: &str, editing: bool, cx: &mut Context<Self>) -> bool {
        let Some(item) = self.item(id, cx) else {
            return false;
        };
        if item.read(cx).is_editing() == editing {
            return true;
        }
        item.update(cx, |item, cx| item.set_editing(editing, cx));
        self.emit_updated(id, cx);
        true
    }

    /// Filters the displayed rows without removing entities or changing CRUD lookups.
    /// Call again when captured business metadata changes.
    pub fn set_filter(
        &mut self,
        filter: impl Fn(&TodoItem) -> bool + 'static,
        cx: &mut Context<Self>,
    ) {
        self.filter = Some(Box::new(filter));
        cx.notify();
    }

    /// Shows every retained item again.
    pub fn clear_filter(&mut self, cx: &mut Context<Self>) {
        self.filter = None;
        cx.notify();
    }

    /// Sets a stable display-order comparator; equal items retain insertion order.
    /// Sorting is presentation-only and emits no data mutation events.
    pub fn set_sort(
        &mut self,
        compare: impl Fn(&TodoItem, &TodoItem) -> Ordering + 'static,
        cx: &mut Context<Self>,
    ) {
        self.sort = Some(Box::new(compare));
        cx.notify();
    }

    /// Restores insertion order without rebuilding item entities.
    pub fn clear_sort(&mut self, cx: &mut Context<Self>) {
        self.sort = None;
        cx.notify();
    }

    /// Returns displayed handles in display order; [`Self::items`] stays unfiltered.
    /// Query outside synchronous item callbacks, since projection reads the child entities.
    pub fn visible_items(&self, cx: &App) -> Vec<gpui::Entity<TodoItem>> {
        let mut items: Vec<_> = self
            .items
            .iter()
            .filter(|item| {
                self.filter
                    .as_ref()
                    .is_none_or(|filter| filter(item.read(cx)))
            })
            .cloned()
            .collect();
        if let Some(compare) = &self.sort {
            items.sort_by(|left, right| compare(left.read(cx), right.read(cx)));
        }
        items
    }

    /// Renders a contextual header before the rows (also when no rows match).
    pub fn header_renderer(
        mut self,
        renderer: impl Fn(TodoListRenderContext) -> AnyElement + 'static,
    ) -> Self {
        self.header_renderer = Some(Box::new(renderer));
        self
    }

    /// Renders when no rows match, including an empty underlying list.
    pub fn empty_renderer(
        mut self,
        renderer: impl Fn(TodoListRenderContext) -> AnyElement + 'static,
    ) -> Self {
        self.empty_renderer = Some(Box::new(renderer));
        self
    }

    /// Contextual alternative to [`Self::footer_renderer`]; the last setter wins.
    pub fn contextual_footer_renderer(
        mut self,
        renderer: impl Fn(TodoListRenderContext) -> AnyElement + 'static,
    ) -> Self {
        self.footer_renderer = None;
        self.contextual_footer_renderer = Some(Box::new(renderer));
        self
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

    /// Renders an element at the bottom of the list, such as an inline add button.
    pub fn footer_renderer(mut self, renderer: impl Fn() -> AnyElement + 'static) -> Self {
        self.contextual_footer_renderer = None;
        self.footer_renderer = Some(Arc::new(renderer));
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
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let items = (self.filter.is_some() || self.sort.is_some()).then(|| self.visible_items(cx));
        let visible = items.as_ref().map_or(self.items.len(), Vec::len);
        let context = TodoListRenderContext {
            total: self.len(),
            checked: if self.header_renderer.is_some()
                || self.contextual_footer_renderer.is_some()
                || (visible == 0 && self.empty_renderer.is_some())
            {
                self.items
                    .iter()
                    .filter(|item| item.read(cx).is_checked())
                    .count()
            } else {
                0
            },
            visible,
        };
        let mut list = gpui::div().flex().flex_col().gap(self.gap);
        if let Some(renderer) = &self.header_renderer {
            list = list.child(renderer(context));
        }
        if visible == 0 {
            if let Some(renderer) = &self.empty_renderer {
                list = list.child(renderer(context));
            }
        }
        list = match items {
            Some(items) => list.children(items),
            None => list.children(self.items.iter().cloned()),
        };
        if let Some(renderer) = &self.contextual_footer_renderer {
            list = list.child(renderer(context));
        } else if let Some(renderer) = &self.footer_renderer {
            list = list.child(renderer());
        }
        list
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
}
