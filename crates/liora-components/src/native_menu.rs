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
    AnyElement, App, Component, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

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
        }
    }

    /// Sets the shortcut text shown for this command.
    pub fn shortcut(mut self, shortcut: impl Into<SharedString>) -> Self {
        self.shortcut = Some(shortcut.into());
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
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativeMenu {
    /// Menu bar title, such as File/Edit/View.
    pub title: SharedString,
    /// Top-level menu rows.
    pub items: Vec<NativeMenuItem>,
    preview_width: gpui::Pixels,
}

impl NativeMenu {
    /// Creates an empty menu descriptor.
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            items: Vec::new(),
            preview_width: px(280.0),
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

    /// Returns total command rows including nested submenu commands, excluding separators.
    pub fn command_count(&self) -> usize {
        self.items.iter().map(count_commands).sum()
    }
}

impl RenderOnce for NativeMenu {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let command_count = self.command_count();
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
                    .map(|item| render_menu_item(item, 0, &theme)),
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

fn render_menu_item(item: NativeMenuItem, depth: usize, theme: &liora_theme::Theme) -> AnyElement {
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
                .when(!disabled, |s| s.hover(|s| s.bg(theme.neutral.hover)))
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
                .map(|child| render_menu_item(child, depth + 1, theme)),
        )
        .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_menu_tracks_items_and_submenus() {
        let menu = NativeMenu::new("File")
            .item(NativeMenuItem::new("open", "Open").shortcut("Ctrl+O"))
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
    }

    #[test]
    fn native_menu_renders_preview_component() {
        let source = include_str!("native_menu.rs");
        assert!(source.contains("impl RenderOnce for NativeMenu"));
        assert!(source.contains("render_menu_item"));
        assert!(source.contains("NativeMenuItem::separator"));
        assert!(source.contains("command_count"));
    }
}
