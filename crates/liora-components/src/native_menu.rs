//! Native Menu module.
//!
//! `NativeMenu` is a platform-neutral app menu model plus a native GPUI preview
//! renderer. Applications can use the same descriptor for custom titlebar menus,
//! settings previews, and future platform menu adapters without coupling menu
//! content to Gallery or Docs.
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
//! the component, and avoid app-specific Gallery/Docs resources in this SDK
//! crate.

use crate::Text;
use gpui::{
    AnyElement, App, Component, IntoElement, MouseButton, RenderOnce, SharedString, Window, div,
    prelude::*, px,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::sync::Arc;

type MenuSelectCallback =
    dyn Fn(NativeMenuAction, &NativeMenuItem, &mut Window, &mut App) + 'static;

/// Built-in menu actions that cover common desktop application commands.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NativeMenuAction {
    /// Opens a new application window. Applications usually map this to their own window factory.
    NewWindow,
    /// Opens a file picker or project picker.
    Open,
    /// Saves the active document.
    Save,
    /// Saves the active document through a save-as flow.
    SaveAs,
    /// Closes the active window or document.
    Close,
    /// Quits the current GPUI application.
    Quit,
    /// Opens the command palette.
    CommandPalette,
    /// Toggles a sidebar region.
    ToggleSidebar,
    /// Toggles a status bar region.
    ToggleStatusBar,
    /// Zooms in the active surface.
    ZoomIn,
    /// Zooms out the active surface.
    ZoomOut,
    /// Resets zoom for the active surface.
    ZoomReset,
    /// Opens an external URL through GPUI's platform integration.
    OpenUrl(SharedString),
    /// Copies text to the native clipboard.
    CopyText(SharedString),
    /// App-defined command id for custom dispatchers.
    Custom(SharedString),
}

impl NativeMenuAction {
    /// Applies the built-in side effect for actions that can be handled generically.
    pub fn perform(&self, cx: &mut App) {
        match self {
            Self::Quit => cx.quit(),
            Self::OpenUrl(url) => cx.open_url(url),
            Self::CopyText(text) => {
                cx.write_to_clipboard(gpui::ClipboardItem::new_string(text.to_string()))
            }
            _ => {}
        }
    }
}

/// Platform-neutral native menu item description.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativeMenuItem {
    /// Stable command id emitted by application menu dispatchers.
    pub id: SharedString,
    /// User-facing menu label.
    pub label: SharedString,
    /// Optional keyboard shortcut shown at the row end.
    pub shortcut: Option<SharedString>,
    /// Whether this item is visible but disabled.
    pub disabled: bool,
    /// Optional nested submenu items.
    pub children: Vec<NativeMenuItem>,
    /// Whether this row is a separator.
    pub separator: bool,
    /// Optional built-in or custom command action.
    pub action: Option<NativeMenuAction>,
}

impl NativeMenuItem {
    /// Creates a clickable menu command item.
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            shortcut: None,
            disabled: false,
            children: Vec::new(),
            separator: false,
            action: None,
        }
    }

    /// Creates a visual separator row.
    pub fn separator() -> Self {
        Self {
            id: "separator".into(),
            label: SharedString::default(),
            shortcut: None,
            disabled: true,
            children: Vec::new(),
            separator: true,
            action: None,
        }
    }

    /// Creates a menu item with a built-in action and conventional id.
    pub fn action(action: NativeMenuAction, label: impl Into<SharedString>) -> Self {
        Self::new(action_id(&action), label).with_action(action)
    }

    /// Creates a built-in New Window item.
    pub fn new_window() -> Self {
        Self::action(NativeMenuAction::NewWindow, "New Window").shortcut("Ctrl+Shift+N")
    }

    /// Creates a built-in Open item.
    pub fn open() -> Self {
        Self::action(NativeMenuAction::Open, "Open...").shortcut("Ctrl+O")
    }

    /// Creates a built-in Save item.
    pub fn save() -> Self {
        Self::action(NativeMenuAction::Save, "Save").shortcut("Ctrl+S")
    }

    /// Creates a built-in Quit item.
    pub fn quit() -> Self {
        Self::action(NativeMenuAction::Quit, "Quit").shortcut("Ctrl+Q")
    }

    /// Creates a built-in Command Palette item.
    pub fn command_palette() -> Self {
        Self::action(NativeMenuAction::CommandPalette, "Command Palette").shortcut("Ctrl+K")
    }

    /// Creates a built-in Toggle Sidebar item.
    pub fn toggle_sidebar() -> Self {
        Self::action(NativeMenuAction::ToggleSidebar, "Toggle Sidebar").shortcut("Ctrl+B")
    }

    /// Creates a built-in Toggle StatusBar item.
    pub fn toggle_statusbar() -> Self {
        Self::action(NativeMenuAction::ToggleStatusBar, "Toggle StatusBar")
    }

    /// Creates a built-in Open URL item.
    pub fn open_url(label: impl Into<SharedString>, url: impl Into<SharedString>) -> Self {
        Self::action(NativeMenuAction::OpenUrl(url.into()), label)
    }

    /// Creates a built-in Copy Text item.
    pub fn copy_text(label: impl Into<SharedString>, text: impl Into<SharedString>) -> Self {
        Self::action(NativeMenuAction::CopyText(text.into()), label)
    }

    /// Sets the shortcut text shown for this command.
    pub fn shortcut(mut self, shortcut: impl Into<SharedString>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Assigns a built-in or custom action to this item.
    pub fn with_action(mut self, action: NativeMenuAction) -> Self {
        self.action = Some(action);
        self
    }

    /// Toggles disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Adds a child item for submenu descriptors.
    pub fn child(mut self, child: NativeMenuItem) -> Self {
        self.children.push(child);
        self
    }

    /// Adds several child items for submenu descriptors.
    pub fn children(mut self, children: impl IntoIterator<Item = NativeMenuItem>) -> Self {
        self.children.extend(children);
        self
    }

    /// Returns true when this item owns submenu children.
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
}

/// Platform-neutral native menu description.
#[derive(Clone)]
pub struct NativeMenu {
    /// Menu bar title, such as File/Edit/View.
    pub title: SharedString,
    /// Top-level menu rows.
    pub items: Vec<NativeMenuItem>,
    preview_width: gpui::Pixels,
    on_select: Option<Arc<MenuSelectCallback>>,
}

impl NativeMenu {
    /// Creates an empty menu descriptor.
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            items: Vec::new(),
            preview_width: px(280.0),
            on_select: None,
        }
    }

    /// Adds one menu item.
    pub fn item(mut self, item: NativeMenuItem) -> Self {
        self.items.push(item);
        self
    }

    /// Adds several menu items.
    pub fn items(mut self, items: impl IntoIterator<Item = NativeMenuItem>) -> Self {
        self.items.extend(items);
        self
    }

    /// Sets the preview width used by Gallery/Docs/custom titlebar previews.
    pub fn preview_width(mut self, width: impl Into<gpui::Pixels>) -> Self {
        self.preview_width = width.into().max(px(180.0));
        self
    }

    /// Registers a callback that receives built-in and custom actions.
    pub fn on_select(
        mut self,
        callback: impl Fn(NativeMenuAction, &NativeMenuItem, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Arc::new(callback));
        self
    }

    /// Returns total command rows including nested submenu commands, excluding separators.
    pub fn command_count(&self) -> usize {
        self.items.iter().map(count_commands).sum()
    }
}

impl RenderOnce for NativeMenu {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let command_count = self.command_count();
        let on_select = self.on_select.clone();
        div()
            .w(self.preview_width)
            .rounded(px(theme.radius.md))
            .border_1()
            .border_color(theme.neutral.border)
            .bg(theme.neutral.card)
            .overflow_hidden()
            .child(
                div()
                    .px_3()
                    .py_2()
                    .bg(theme.neutral.hover)
                    .border_b_1()
                    .border_color(theme.neutral.border)
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(Text::new(self.title).bold())
                    .child(Text::new(format!("{} commands", command_count)).xs()),
            )
            .children(
                self.items
                    .into_iter()
                    .map(|item| render_menu_item(item, 0, &theme, on_select.clone())),
            )
    }
}

impl IntoElement for NativeMenu {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

fn count_commands(item: &NativeMenuItem) -> usize {
    if item.separator {
        0
    } else {
        1 + item.children.iter().map(count_commands).sum::<usize>()
    }
}

fn action_id(action: &NativeMenuAction) -> SharedString {
    match action {
        NativeMenuAction::NewWindow => "new-window".into(),
        NativeMenuAction::Open => "open".into(),
        NativeMenuAction::Save => "save".into(),
        NativeMenuAction::SaveAs => "save-as".into(),
        NativeMenuAction::Close => "close".into(),
        NativeMenuAction::Quit => "quit".into(),
        NativeMenuAction::CommandPalette => "command-palette".into(),
        NativeMenuAction::ToggleSidebar => "toggle-sidebar".into(),
        NativeMenuAction::ToggleStatusBar => "toggle-statusbar".into(),
        NativeMenuAction::ZoomIn => "zoom-in".into(),
        NativeMenuAction::ZoomOut => "zoom-out".into(),
        NativeMenuAction::ZoomReset => "zoom-reset".into(),
        NativeMenuAction::OpenUrl(_) => "open-url".into(),
        NativeMenuAction::CopyText(_) => "copy-text".into(),
        NativeMenuAction::Custom(id) => id.clone(),
    }
}

fn render_menu_item(
    item: NativeMenuItem,
    depth: usize,
    theme: &liora_theme::Theme,
    on_select: Option<Arc<MenuSelectCallback>>,
) -> AnyElement {
    if item.separator {
        return div()
            .h(px(1.0))
            .mx_2()
            .my_1()
            .bg(theme.neutral.divider)
            .into_any_element();
    }

    let disabled = item.disabled;
    let has_children = item.has_children();
    let children = item.children.clone();
    let action = item.action.clone();
    let click_item = item.clone();
    let click_callback = on_select.clone();
    div()
        .flex()
        .flex_col()
        .child(
            div()
                .min_h(px(32.0))
                .px_3()
                .pl(px(12.0 + depth as f32 * 16.0))
                .flex()
                .items_center()
                .justify_between()
                .text_color(if disabled {
                    theme.neutral.text_disabled
                } else {
                    theme.neutral.text_1
                })
                .when(!disabled, |s| {
                    s.cursor_pointer()
                        .hover(|s| s.bg(theme.neutral.hover))
                        .on_mouse_up(MouseButton::Left, move |_, window, cx| {
                            let selected_action = action
                                .clone()
                                .unwrap_or_else(|| NativeMenuAction::Custom(click_item.id.clone()));
                            selected_action.perform(cx);
                            if let Some(callback) = &click_callback {
                                callback(selected_action, &click_item, window, cx);
                            }
                        })
                })
                .child(Text::new(item.label).sm().text_color(if disabled {
                    theme.neutral.text_disabled
                } else {
                    theme.neutral.text_1
                }))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .when_some(item.shortcut, |s, shortcut| {
                            s.child(Text::new(shortcut).xs().text_color(theme.neutral.text_3))
                        })
                        .when(has_children, |s| {
                            s.child(
                                Icon::new(IconName::ChevronRight)
                                    .size(px(13.0))
                                    .color(theme.neutral.text_3),
                            )
                        }),
                ),
        )
        .children(
            children
                .into_iter()
                .map(|child| render_menu_item(child, depth + 1, theme, on_select.clone())),
        )
        .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_menu_tracks_items_and_submenus() {
        let menu = NativeMenu::new("File")
            .item(NativeMenuItem::open())
            .item(NativeMenuItem::separator())
            .item(
                NativeMenuItem::new("recent", "Open Recent")
                    .child(NativeMenuItem::new("recent-1", "liora")),
            );
        assert_eq!(menu.items.len(), 3);
        assert_eq!(menu.command_count(), 3);
        assert_eq!(
            menu.items[0].shortcut.as_ref().map(|s| s.as_ref()),
            Some("Ctrl+O")
        );
        assert_eq!(menu.items[0].action, Some(NativeMenuAction::Open));
    }

    #[test]
    fn native_menu_renders_preview_component() {
        let source = include_str!("native_menu.rs");
        assert!(source.contains("impl RenderOnce for NativeMenu"));
        assert!(source.contains("render_menu_item"));
        assert!(source.contains("NativeMenuItem::separator"));
        assert!(source.contains("command_count"));
        assert!(source.contains("cursor_pointer"));
        assert!(source.contains("on_mouse_up(MouseButton::Left"));
        assert!(source.contains("pub enum NativeMenuAction"));
        assert!(source.contains("pub fn open_url"));
    }
}
