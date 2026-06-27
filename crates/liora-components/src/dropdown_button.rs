//! Dropdown Button module.
//!
//! This public module implements a Liora split-capable dropdown button composed from native GPUI Button/Popover
//! behavior. It keeps the reusable component logic inside `liora-components` rather than Gallery or Docs so
//! downstream GPUI applications can compose command menus without app-specific resources.
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

use crate::{Button, Popover, gpui_compat::element_id};
use gpui::{
    AnyElement, App, Component, Hsla, IntoElement, MouseButton, RenderOnce, SharedString, Window,
    div, prelude::*, px,
};
use liora_core::{Config, Placement, clear_popover, stable_unique_id};
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use liora_theme::{ButtonSize, ButtonVariant};
use std::sync::Arc;

type DropdownButtonCallback = Arc<dyn Fn(&mut Window, &mut App) + 'static>;

/// Data model used by dropdown button item rendering.
#[derive(Clone)]
pub struct DropdownButtonItem {
    /// User-facing label rendered for this item.
    pub label: SharedString,
    /// Optional leading Lucide icon.
    pub icon: Option<IconName>,
    /// Whether the item should be inert and styled as disabled.
    pub disabled: bool,
    /// Whether the item should use danger semantics.
    pub danger: bool,
    /// Callback invoked when the item is activated.
    pub on_click: DropdownButtonCallback,
}

impl DropdownButtonItem {
    /// Creates `DropdownButtonItem` initialized from the supplied label and callback.
    pub fn new(
        label: impl Into<SharedString>,
        on_click: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            label: label.into(),
            icon: None,
            disabled: false,
            danger: false,
            on_click: Arc::new(on_click),
        }
    }

    /// Sets the leading icon used by the item.
    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Toggles the disabled state and suppresses activation when enabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Applies the danger semantic treatment.
    pub fn danger(mut self) -> Self {
        self.danger = true;
        self
    }
}

/// Fluent native GPUI component for rendering Liora dropdown button.
pub struct DropdownButton {
    label: SharedString,
    items: Vec<DropdownButtonItem>,
    placement: Placement,
    close_on_click_outside: bool,
    close_on_escape: bool,
    id: Option<SharedString>,
    variant: ButtonVariant,
    size: ButtonSize,
    secondary: bool,
    disabled: bool,
    split: bool,
    leading_icon: Option<IconName>,
    on_click: Option<DropdownButtonCallback>,
}

impl DropdownButton {
    /// Creates `DropdownButton` initialized from the supplied label.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            items: Vec::new(),
            placement: Placement::BottomStart,
            close_on_click_outside: true,
            close_on_escape: true,
            id: None,
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            secondary: false,
            disabled: false,
            split: false,
            leading_icon: None,
            on_click: None,
        }
    }

    /// Adds an enabled menu item.
    pub fn item(
        mut self,
        label: impl Into<SharedString>,
        on_click: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.items.push(DropdownButtonItem::new(label, on_click));
        self
    }

    /// Adds a preconfigured menu item.
    pub fn menu_item(mut self, item: DropdownButtonItem) -> Self {
        self.items.push(item);
        self
    }

    /// Adds a disabled menu item.
    pub fn disabled_item(mut self, label: impl Into<SharedString>) -> Self {
        self.items
            .push(DropdownButtonItem::new(label, |_, _| {}).disabled(true));
        self
    }

    /// Adds a danger menu item.
    pub fn danger_item(
        mut self,
        label: impl Into<SharedString>,
        on_click: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.items
            .push(DropdownButtonItem::new(label, on_click).danger());
        self
    }

    /// Assigns a stable element id used by GPUI state, hit testing, and automated interaction tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Selects the popup placement.
    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }

    /// Toggles whether the popup closes when escape occurs.
    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = close;
        self
    }

    /// Toggles whether the popup closes when click outside occurs.
    pub fn close_on_click_outside(mut self, close: bool) -> Self {
        self.close_on_click_outside = close;
        self
    }

    /// Applies the primary semantic visual variant.
    pub fn primary(mut self) -> Self {
        self.variant = ButtonVariant::Primary;
        self
    }

    /// Applies the informational semantic visual variant.
    pub fn info(mut self) -> Self {
        self.variant = ButtonVariant::Info;
        self
    }

    /// Applies the success semantic visual variant.
    pub fn success(mut self) -> Self {
        self.variant = ButtonVariant::Success;
        self
    }

    /// Applies the warning semantic visual variant.
    pub fn warning(mut self) -> Self {
        self.variant = ButtonVariant::Warning;
        self
    }

    /// Applies the danger semantic visual variant.
    pub fn danger(mut self) -> Self {
        self.variant = ButtonVariant::Danger;
        self
    }

    /// Uses the compact size preset.
    pub fn small(mut self) -> Self {
        self.size = ButtonSize::Small;
        self
    }

    /// Uses the large size preset.
    pub fn large(mut self) -> Self {
        self.size = ButtonSize::Large;
        self
    }

    /// Toggles the disabled state and suppresses trigger/menu interaction when enabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Applies the secondary visual treatment.
    pub fn secondary(mut self) -> Self {
        self.secondary = true;
        self
    }

    /// Sets the leading icon rendered before the button label.
    pub fn icon_start(mut self, icon: IconName) -> Self {
        self.leading_icon = Some(icon);
        self
    }

    /// Enables split button behavior: left side invokes `on_click`, right side opens the menu.
    pub fn split(mut self, split: bool) -> Self {
        self.split = split;
        self
    }

    /// Registers the primary action used by split button mode.
    pub fn on_click(mut self, cb: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Arc::new(cb));
        self
    }

    fn resolved_button(&self, label: impl Into<SharedString>) -> Button {
        let mut button = Button::new(label).variant(self.variant).size(self.size);
        if self.secondary {
            button = button.secondary();
        }
        if self.disabled {
            button = button.disabled(true);
        }
        button
    }

    fn menu_trigger(&self, id: &SharedString) -> AnyElement {
        if self.split {
            split_trigger(self, id)
        } else {
            let mut button = self
                .resolved_button(self.label.clone())
                .icon_end(IconName::ChevronDown);
            if let Some(icon) = self.leading_icon {
                button = button.icon_start(icon);
            }
            if self.disabled {
                button.into_any_element()
            } else {
                button.into_any_element()
            }
        }
    }
}

impl RenderOnce for DropdownButton {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let dropdown_id = self.id.clone().unwrap_or_else(|| {
            stable_unique_id(
                format!(
                    "dropdown-button:{}:{}:{}:{:?}:{:?}",
                    self.label,
                    self.items.len(),
                    self.split,
                    self.variant,
                    self.size
                ),
                "dropdown-button",
                window,
                cx,
            )
        });
        let trigger = self.menu_trigger(&dropdown_id);
        if self.disabled {
            return trigger;
        }

        let items = self.items;
        let disabled = self.disabled;

        Popover::new(trigger)
            .id(dropdown_id.clone())
            .placement(self.placement)
            .offset(px(4.0))
            .flush_content()
            .close_on_click_outside(self.close_on_click_outside)
            .close_on_escape(self.close_on_escape)
            .content(move |_window, _cx| {
                dropdown_button_menu(dropdown_id.clone(), items.clone(), theme.clone(), disabled)
            })
            .into_any_element()
    }
}

impl IntoElement for DropdownButton {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

fn split_trigger(button: &DropdownButton, id: &SharedString) -> AnyElement {
    let main_id = element_id(format!("{id}-main-action"));
    let toggle_id = element_id(format!("{id}-toggle"));
    let mut primary = button.resolved_button(button.label.clone()).id(main_id);
    if let Some(icon) = button.leading_icon {
        primary = primary.icon_start(icon);
    }
    if !button.disabled
        && let Some(on_click) = button.on_click.clone()
    {
        primary = primary.on_click(move |_, window, cx| {
            on_click(window, cx);
            cx.stop_propagation();
        });
    }

    let toggle = button
        .resolved_button("")
        .id(toggle_id)
        .icon_only(IconName::ChevronDown);

    div()
        .id(element_id(format!("{id}-split-trigger")))
        .flex()
        .flex_row()
        .items_center()
        .gap_1()
        .child(primary)
        .child(toggle)
        .into_any_element()
}

fn dropdown_button_menu(
    dropdown_id: SharedString,
    items: Vec<DropdownButtonItem>,
    theme: liora_theme::Theme,
    disabled: bool,
) -> AnyElement {
    div()
        .id(element_id(format!("{dropdown_id}-menu")))
        .cursor_default()
        .occlude()
        .flex()
        .flex_col()
        .py_1()
        .min_w(px(188.0))
        .max_h(px(240.0))
        .children(items.into_iter().enumerate().map(|(index, item)| {
            dropdown_button_item(dropdown_id.clone(), index, item, theme.clone(), disabled)
        }))
        .into_any_element()
}

fn dropdown_button_item(
    dropdown_id: SharedString,
    index: usize,
    item: DropdownButtonItem,
    theme: liora_theme::Theme,
    menu_disabled: bool,
) -> AnyElement {
    let disabled = menu_disabled || item.disabled;
    let item_id = element_id(format!("{dropdown_id}-item-{index}"));
    let label = item.label;
    let on_click = item.on_click;
    let icon = item.icon;
    let text_color = item_text_color(&theme, disabled, item.danger);
    let hover_color = if item.danger {
        theme.danger.base
    } else {
        theme.primary.base
    };

    div()
        .id(item_id)
        .flex()
        .items_center()
        .gap_2()
        .min_h(px(34.0))
        .px_3()
        .py_2()
        .text_size(px(theme.font_size.md))
        .text_color(text_color)
        .when(disabled, |s| s.cursor_not_allowed())
        .when(!disabled, |s| {
            s.cursor_pointer()
                .hover(move |s| s.bg(theme.neutral.hover).text_color(hover_color))
        })
        .when_some(icon, |s, icon| {
            s.child(Icon::new(icon).size(px(14.0)).color(text_color))
        })
        .child(div().text_sm().child(label))
        .when(!disabled, |s| {
            s.on_mouse_down(MouseButton::Left, move |_, window, cx| {
                on_click(window, cx);
                clear_popover(&dropdown_id, cx);
                cx.stop_propagation();
            })
        })
        .into_any_element()
}

fn item_text_color(theme: &liora_theme::Theme, disabled: bool, danger: bool) -> Hsla {
    if disabled {
        theme.neutral.text_disabled
    } else if danger {
        theme.danger.base
    } else {
        theme.neutral.text_1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn dropdown_button_flushes_shared_popover_padding_for_menu_layout() {
        let source = include_str!("dropdown_button.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains(".flush_content()"));
    }

    use super::*;

    #[test]
    fn dropdown_button_builders_track_state() {
        let button = DropdownButton::new("Deploy")
            .id("deploy-menu")
            .primary()
            .large()
            .secondary()
            .split(true)
            .icon_start(IconName::Rocket)
            .on_click(|_, _| {})
            .placement(Placement::TopEnd)
            .close_on_click_outside(false)
            .close_on_escape(false)
            .item("Preview", |_, _| {})
            .menu_item(DropdownButtonItem::new("Rollback", |_, _| {}).icon(IconName::Undo2))
            .disabled_item("Locked")
            .danger_item("Delete", |_, _| {});

        assert_eq!(button.id.as_ref().map(AsRef::as_ref), Some("deploy-menu"));
        assert_eq!(button.variant, ButtonVariant::Primary);
        assert_eq!(button.size, ButtonSize::Large);
        assert!(button.secondary);
        assert!(button.split);
        assert_eq!(button.leading_icon, Some(IconName::Rocket));
        assert_eq!(button.placement, Placement::TopEnd);
        assert!(!button.close_on_click_outside);
        assert!(!button.close_on_escape);
        assert_eq!(button.items.len(), 4);
        assert!(button.items[1].icon.is_some());
        assert!(button.items[2].disabled);
        assert!(button.items[3].danger);
    }

    #[test]
    fn dropdown_button_reuses_popover_and_exposes_split_trigger() {
        let source = include_str!("dropdown_button.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains("Popover::new(trigger)"));
        assert!(source.contains("fn split_trigger"));
        assert!(source.contains("IconName::ChevronDown"));
        assert!(source.contains("clear_popover(&dropdown_id, cx)"));
        assert!(source.contains("cx.stop_propagation();"));
    }
}
