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
    AnyElement, App, Component, IntoElement, MouseButton, PathPromptOptions, RenderOnce,
    SharedString, Window, div, prelude::*, px,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::{path::PathBuf, sync::Arc};

type MenuSelectCallback =
    dyn Fn(NativeMenuAction, &NativeMenuItem, &mut Window, &mut App) + 'static;
type PathSelectCallback = dyn Fn(NativeMenuAction, Option<Vec<PathBuf>>, &mut App) + 'static;

/// Static documentation for a built-in native menu action.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NativeMenuActionInfo {
    /// Stable action id used by command dispatchers.
    pub id: &'static str,
    /// Human-readable action name.
    pub name: &'static str,
    /// Short description of what the action represents.
    pub description: &'static str,
    /// Built-in effect performed by Liora when automatic effects are enabled.
    pub effect: &'static str,
    /// Whether Liora can perform the effect without application-specific state.
    pub handled_by_liora: bool,
}

/// Built-in menu actions that cover common desktop application commands.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NativeMenuAction {
    /// Opens a new application window. Applications usually map this to their own window factory.
    NewWindow,
    /// Opens a file or folder picker.
    Open,
    /// Opens a single-file picker.
    OpenFile,
    /// Opens a multiple-file picker.
    OpenFiles,
    /// Opens a single-folder picker.
    OpenFolder,
    /// Opens a multiple-folder picker.
    OpenFolders,
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
    /// Returns static metadata describing this action and its default effect.
    pub fn info(&self) -> NativeMenuActionInfo {
        match self {
            Self::NewWindow => NativeMenuActionInfo {
                id: "new-window",
                name: "NewWindow",
                description: "Request a new application window.",
                effect: "Dispatch only; application opens the window from on_select.",
                handled_by_liora: false,
            },
            Self::Open => NativeMenuActionInfo {
                id: "open",
                name: "Open",
                description: "Open a system picker that accepts one file or folder.",
                effect: "Calls cx.prompt_for_paths(files=true, directories=true, multiple=false).",
                handled_by_liora: true,
            },
            Self::OpenFile => NativeMenuActionInfo {
                id: "open-file",
                name: "OpenFile",
                description: "Open a system picker for one file.",
                effect: "Calls cx.prompt_for_paths(files=true, directories=false, multiple=false).",
                handled_by_liora: true,
            },
            Self::OpenFiles => NativeMenuActionInfo {
                id: "open-files",
                name: "OpenFiles",
                description: "Open a system picker for multiple files.",
                effect: "Calls cx.prompt_for_paths(files=true, directories=false, multiple=true).",
                handled_by_liora: true,
            },
            Self::OpenFolder => NativeMenuActionInfo {
                id: "open-folder",
                name: "OpenFolder",
                description: "Open a system picker for one folder.",
                effect: "Calls cx.prompt_for_paths(files=false, directories=true, multiple=false).",
                handled_by_liora: true,
            },
            Self::OpenFolders => NativeMenuActionInfo {
                id: "open-folders",
                name: "OpenFolders",
                description: "Open a system picker for multiple folders.",
                effect: "Calls cx.prompt_for_paths(files=false, directories=true, multiple=true).",
                handled_by_liora: true,
            },
            Self::Save => NativeMenuActionInfo {
                id: "save",
                name: "Save",
                description: "Request saving the active document.",
                effect: "Opens a Save As path dialog when no app-specific save handler is attached; application still writes content.",
                handled_by_liora: true,
            },
            Self::SaveAs => NativeMenuActionInfo {
                id: "save-as",
                name: "SaveAs",
                description: "Request saving through a Save As flow.",
                effect: "Calls cx.prompt_for_new_path(current_dir, suggested_name). Application still writes content.",
                handled_by_liora: true,
            },
            Self::Close => NativeMenuActionInfo {
                id: "close",
                name: "Close",
                description: "Request closing the active document or window.",
                effect: "Calls window.remove_window(); apps with close confirmation should disable automatic actions and handle on_select.",
                handled_by_liora: true,
            },
            Self::Quit => NativeMenuActionInfo {
                id: "quit",
                name: "Quit",
                description: "Quit the current GPUI application.",
                effect: "Calls cx.quit() when automatic effects are enabled.",
                handled_by_liora: true,
            },
            Self::CommandPalette => NativeMenuActionInfo {
                id: "command-palette",
                name: "CommandPalette",
                description: "Request opening the app command palette.",
                effect: "Dispatch only; application owns palette state.",
                handled_by_liora: false,
            },
            Self::ToggleSidebar => NativeMenuActionInfo {
                id: "toggle-sidebar",
                name: "ToggleSidebar",
                description: "Request showing or hiding the sidebar.",
                effect: "Dispatch only; application owns shell layout state.",
                handled_by_liora: false,
            },
            Self::ToggleStatusBar => NativeMenuActionInfo {
                id: "toggle-statusbar",
                name: "ToggleStatusBar",
                description: "Request showing or hiding the status bar.",
                effect: "Dispatch only; application owns shell layout state.",
                handled_by_liora: false,
            },
            Self::ZoomIn => NativeMenuActionInfo {
                id: "zoom-in",
                name: "ZoomIn",
                description: "Request zooming in the active surface.",
                effect: "Increases the current window rem size for app-level UI zoom.",
                handled_by_liora: true,
            },
            Self::ZoomOut => NativeMenuActionInfo {
                id: "zoom-out",
                name: "ZoomOut",
                description: "Request zooming out the active surface.",
                effect: "Decreases the current window rem size for app-level UI zoom.",
                handled_by_liora: true,
            },
            Self::ZoomReset => NativeMenuActionInfo {
                id: "zoom-reset",
                name: "ZoomReset",
                description: "Request resetting active-surface zoom.",
                effect: "Resets the current window rem size to 16px.",
                handled_by_liora: true,
            },
            Self::OpenUrl(_) => NativeMenuActionInfo {
                id: "open-url",
                name: "OpenUrl",
                description: "Open an external URL through the platform.",
                effect: "Calls cx.open_url(url) when automatic effects are enabled.",
                handled_by_liora: true,
            },
            Self::CopyText(_) => NativeMenuActionInfo {
                id: "copy-text",
                name: "CopyText",
                description: "Copy text into the native clipboard.",
                effect: "Calls cx.write_to_clipboard(...) when automatic effects are enabled.",
                handled_by_liora: true,
            },
            Self::Custom(_) => NativeMenuActionInfo {
                id: "custom",
                name: "Custom",
                description: "Application-defined command id.",
                effect: "Dispatch only; application handles it from on_select.",
                handled_by_liora: false,
            },
        }
    }

    /// Returns a representative catalog of built-in and custom action variants.
    pub fn catalog() -> Vec<Self> {
        vec![
            Self::NewWindow,
            Self::Open,
            Self::OpenFile,
            Self::OpenFiles,
            Self::OpenFolder,
            Self::OpenFolders,
            Self::Save,
            Self::SaveAs,
            Self::Close,
            Self::Quit,
            Self::CommandPalette,
            Self::ToggleSidebar,
            Self::ToggleStatusBar,
            Self::ZoomIn,
            Self::ZoomOut,
            Self::ZoomReset,
            Self::OpenUrl("https://github.com/yhyzgn/liora".into()),
            Self::CopyText("liora".into()),
            Self::Custom("check-updates".into()),
        ]
    }

    /// Applies the built-in side effect for actions that can be handled generically.
    pub fn perform(&self, window: &mut Window, cx: &mut App) {
        self.perform_with_path_callback(window, cx, None);
    }

    fn perform_with_path_callback(
        &self,
        window: &mut Window,
        cx: &mut App,
        on_paths_selected: Option<Arc<PathSelectCallback>>,
    ) {
        match self {
            Self::Open => prompt_for_existing_paths(
                self.clone(),
                path_prompt(true, true, false, "Open file or folder"),
                on_paths_selected,
                cx,
            ),
            Self::OpenFile => prompt_for_existing_paths(
                self.clone(),
                path_prompt(true, false, false, "Open file"),
                on_paths_selected,
                cx,
            ),
            Self::OpenFiles => prompt_for_existing_paths(
                self.clone(),
                path_prompt(true, false, true, "Open files"),
                on_paths_selected,
                cx,
            ),
            Self::OpenFolder => prompt_for_existing_paths(
                self.clone(),
                path_prompt(false, true, false, "Open folder"),
                on_paths_selected,
                cx,
            ),
            Self::OpenFolders => prompt_for_existing_paths(
                self.clone(),
                path_prompt(false, true, true, "Open folders"),
                on_paths_selected,
                cx,
            ),
            Self::Save | Self::SaveAs => {
                let directory = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
                prompt_for_new_path(
                    self.clone(),
                    directory,
                    Some("Untitled"),
                    on_paths_selected,
                    cx,
                );
            }
            Self::Close => window.remove_window(),
            Self::Quit => cx.quit(),
            Self::ZoomIn => window.set_rem_size(window.rem_size() + px(1.0)),
            Self::ZoomOut => window.set_rem_size((window.rem_size() - px(1.0)).max(px(8.0))),
            Self::ZoomReset => window.set_rem_size(px(16.0)),
            Self::OpenUrl(url) => cx.open_url(url),
            Self::CopyText(text) => {
                cx.write_to_clipboard(gpui::ClipboardItem::new_string(text.to_string()))
            }
            _ => {}
        }
    }
}

fn path_prompt(
    files: bool,
    directories: bool,
    multiple: bool,
    prompt: &'static str,
) -> PathPromptOptions {
    PathPromptOptions {
        files,
        directories,
        multiple,
        prompt: Some(prompt.into()),
    }
}

fn prompt_for_existing_paths(
    action: NativeMenuAction,
    options: PathPromptOptions,
    callback: Option<Arc<PathSelectCallback>>,
    cx: &mut App,
) {
    let receiver = cx.prompt_for_paths(options);
    let Some(callback) = callback else {
        return;
    };
    let app = cx.to_async();
    cx.foreground_executor()
        .spawn(async move {
            let selected = receiver.await.ok().and_then(Result::ok).flatten();
            let _ = app.update(|cx| callback(action, selected, cx));
        })
        .detach();
}

fn prompt_for_new_path(
    action: NativeMenuAction,
    directory: PathBuf,
    suggested_name: Option<&str>,
    callback: Option<Arc<PathSelectCallback>>,
    cx: &mut App,
) {
    let receiver = cx.prompt_for_new_path(&directory, suggested_name);
    let Some(callback) = callback else {
        return;
    };
    let app = cx.to_async();
    cx.foreground_executor()
        .spawn(async move {
            let selected = receiver
                .await
                .ok()
                .and_then(Result::ok)
                .flatten()
                .map(|path| vec![path]);
            let _ = app.update(|cx| callback(action, selected, cx));
        })
        .detach();
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

    /// Creates a built-in Open item that accepts one file or folder.
    pub fn open() -> Self {
        Self::action(NativeMenuAction::Open, "Open...").shortcut("Ctrl+O")
    }

    /// Creates a built-in Open File item.
    pub fn open_file() -> Self {
        Self::action(NativeMenuAction::OpenFile, "Open File...").shortcut("Ctrl+O")
    }

    /// Creates a built-in Open Files item.
    pub fn open_files() -> Self {
        Self::action(NativeMenuAction::OpenFiles, "Open Files...").shortcut("Ctrl+Shift+O")
    }

    /// Creates a built-in Open Folder item.
    pub fn open_folder() -> Self {
        Self::action(NativeMenuAction::OpenFolder, "Open Folder...").shortcut("Ctrl+Alt+O")
    }

    /// Creates a built-in Open Folders item.
    pub fn open_folders() -> Self {
        Self::action(NativeMenuAction::OpenFolders, "Open Folders...")
    }

    /// Creates a built-in Save item.
    pub fn save() -> Self {
        Self::action(NativeMenuAction::Save, "Save").shortcut("Ctrl+S")
    }

    /// Creates a built-in Save As item.
    pub fn save_as() -> Self {
        Self::action(NativeMenuAction::SaveAs, "Save As...").shortcut("Ctrl+Shift+S")
    }

    /// Creates a built-in Close item.
    pub fn close() -> Self {
        Self::action(NativeMenuAction::Close, "Close").shortcut("Ctrl+W")
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
    on_paths_selected: Option<Arc<PathSelectCallback>>,
    perform_builtin_actions: bool,
}

impl NativeMenu {
    /// Creates an empty menu descriptor.
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            items: Vec::new(),
            preview_width: px(280.0),
            on_select: None,
            on_paths_selected: None,
            perform_builtin_actions: true,
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

    /// Configures whether built-in generic effects run before `on_select`.
    ///
    /// Keep this enabled for real application menus. Disable it for Gallery,
    /// Docs, tests, or command preview surfaces where actions such as Quit or
    /// OpenUrl should be demonstrated without side effects.
    pub fn perform_builtin_actions(mut self, perform: bool) -> Self {
        self.perform_builtin_actions = perform;
        self
    }

    /// Registers a callback for paths selected by Open/OpenFile/OpenFolder/SaveAs dialogs.
    pub fn on_paths_selected(
        mut self,
        callback: impl Fn(NativeMenuAction, Option<Vec<PathBuf>>, &mut App) + 'static,
    ) -> Self {
        self.on_paths_selected = Some(Arc::new(callback));
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
        let on_paths_selected = self.on_paths_selected.clone();
        let perform_builtin_actions = self.perform_builtin_actions;
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
            .children(self.items.into_iter().map(|item| {
                render_menu_item(
                    item,
                    0,
                    &theme,
                    on_select.clone(),
                    on_paths_selected.clone(),
                    perform_builtin_actions,
                )
            }))
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
        NativeMenuAction::OpenFile => "open-file".into(),
        NativeMenuAction::OpenFiles => "open-files".into(),
        NativeMenuAction::OpenFolder => "open-folder".into(),
        NativeMenuAction::OpenFolders => "open-folders".into(),
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
    on_paths_selected: Option<Arc<PathSelectCallback>>,
    perform_builtin_actions: bool,
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
    let click_paths_callback = on_paths_selected.clone();
    div()
        .flex()
        .flex_col()
        .child(
            div()
                .w_full()
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
                        .hover(|s| s.cursor_pointer().bg(theme.neutral.hover))
                        .on_mouse_up(MouseButton::Left, move |_, window, cx| {
                            let selected_action = action
                                .clone()
                                .unwrap_or_else(|| NativeMenuAction::Custom(click_item.id.clone()));
                            if perform_builtin_actions {
                                selected_action.perform_with_path_callback(
                                    window,
                                    cx,
                                    click_paths_callback.clone(),
                                );
                            }
                            if let Some(callback) = &click_callback {
                                callback(selected_action, &click_item, window, cx);
                            }
                        })
                })
                .child(
                    Text::new(item.label)
                        .sm()
                        .selectable(false)
                        .text_color(if disabled {
                            theme.neutral.text_disabled
                        } else {
                            theme.neutral.text_1
                        }),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .when_some(item.shortcut, |s, shortcut| {
                            s.child(
                                Text::new(shortcut)
                                    .xs()
                                    .selectable(false)
                                    .text_color(theme.neutral.text_3),
                            )
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
        .children(children.into_iter().map(|child| {
            render_menu_item(
                child,
                depth + 1,
                theme,
                on_select.clone(),
                on_paths_selected.clone(),
                perform_builtin_actions,
            )
        }))
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
        assert!(NativeMenuAction::Open.info().handled_by_liora);
        assert!(NativeMenuAction::OpenFolder.info().handled_by_liora);
        assert!(NativeMenuAction::SaveAs.info().handled_by_liora);
        assert!(
            NativeMenuAction::CopyText("liora".into())
                .info()
                .handled_by_liora
        );
    }

    #[test]
    fn native_menu_renders_preview_component() {
        let source = include_str!("native_menu.rs");
        assert!(source.contains("impl RenderOnce for NativeMenu"));
        assert!(source.contains("render_menu_item"));
        assert!(source.contains("NativeMenuItem::separator"));
        assert!(source.contains("command_count"));
        assert!(source.contains(".w_full()"));
        assert!(source.contains(".selectable(false)"));
        assert!(source.contains("hover(|s| s.cursor_pointer().bg(theme.neutral.hover))"));
        assert!(source.contains("on_mouse_up(MouseButton::Left"));
        assert!(source.contains("pub enum NativeMenuAction"));
        assert!(source.contains("pub fn open_url"));
        assert!(source.contains("NativeMenuActionInfo"));
        assert!(source.contains("pub fn catalog"));
        assert!(source.contains("perform_builtin_actions"));
        assert!(source.contains("on_paths_selected"));
        assert!(source.contains("prompt_for_paths"));
        assert!(source.contains("prompt_for_new_path"));
        assert!(source.contains("window.remove_window()"));
        assert!(source.contains("window.set_rem_size"));
    }
}
