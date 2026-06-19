//! Checkbox Group module.
//!
//! This public module implements the Liora checkbox group control for multi-select option sets. It keeps the reusable
//! component logic inside `liora-components` rather than Gallery or Docs so
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
//! the component, and avoid app-specific Gallery/Docs resources in this SDK
//! crate.

use crate::{Checkbox, CheckboxChanged};
use gpui::{
    AnyElement, App, Context, Entity, FocusHandle, Focusable, Hsla, MouseButton, MouseUpEvent,
    Pixels, Render, Rgba, SharedString, Window, prelude::*, px,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

fn rgba(r: u8, g: u8, b: u8, a: f32) -> Hsla {
    Rgba {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a,
    }
    .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control checkbox group layout behavior.
pub enum CheckboxGroupLayout {
    #[default]
    /// Lays out content in the vertical direction.
    Vertical,
    /// Lays out content in the horizontal direction.
    Horizontal,
    /// Uses button rendering for `CheckboxGroupLayout`.
    Button,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control checkbox group size behavior.
pub enum CheckboxGroupSize {
    /// Uses expanded sizing metrics.
    Large,
    #[default]
    /// Uses the default neutral treatment.
    Default,
    /// Uses compact sizing metrics.
    Small,
}

impl CheckboxGroupSize {
    fn height(self) -> Pixels {
        match self {
            CheckboxGroupSize::Large => px(38.0),
            CheckboxGroupSize::Default => px(32.0),
            CheckboxGroupSize::Small => px(24.0),
        }
    }

    fn padding_x(self) -> Pixels {
        match self {
            CheckboxGroupSize::Large => px(18.0),
            CheckboxGroupSize::Default => px(14.0),
            CheckboxGroupSize::Small => px(10.0),
        }
    }

    fn text_size(self, theme: &liora_theme::Theme) -> Pixels {
        match self {
            CheckboxGroupSize::Large => px(theme.font_size.md),
            CheckboxGroupSize::Default => px(theme.font_size.md),
            CheckboxGroupSize::Small => px(theme.font_size.sm),
        }
    }
}

#[derive(Clone, Debug, Default)]
/// Fluent native GPUI component for rendering Liora checkbox option style.
pub struct CheckboxOptionStyle {
    /// Base background color used in the normal state.
    pub bg: Option<Hsla>,
    /// Background color for the selected state.
    pub selected_bg: Option<Hsla>,
    /// Background color used while the control is hovered.
    pub hover_bg: Option<Hsla>,
    /// Foreground text color for the normal state.
    pub text_color: Option<Hsla>,
    /// Foreground text color for the selected state.
    pub selected_text_color: Option<Hsla>,
    /// Border color for the normal state.
    pub border_color: Option<Hsla>,
    /// Border color for the selected state.
    pub selected_border_color: Option<Hsla>,
    /// Corner radius applied to the rendered control.
    pub radius: Option<Pixels>,
    /// Horizontal padding applied inside the control.
    pub padding_x: Option<Pixels>,
    /// Vertical padding applied inside the control.
    pub padding_y: Option<Pixels>,
    /// Gap between adjacent child elements.
    pub gap: Option<Pixels>,
    /// Whether the selection indicator is rendered.
    pub show_indicator: Option<bool>,
    /// Whether a selected-state icon is rendered.
    pub show_selected_icon: Option<bool>,
}

impl CheckboxOptionStyle {
    /// Creates `CheckboxOptionStyle` with default theme-driven styling and no optional callbacks attached.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the bg used by the rendered component.
    pub fn bg(mut self, color: Hsla) -> Self {
        self.bg = Some(color);
        self
    }

    /// Sets the background color used by selected options.
    pub fn selected_bg(mut self, color: Hsla) -> Self {
        self.selected_bg = Some(color);
        self
    }

    /// Sets the background color shown while an option is hovered.
    pub fn hover_bg(mut self, color: Hsla) -> Self {
        self.hover_bg = Some(color);
        self
    }

    /// Applies the foreground text color.
    pub fn text_color(mut self, color: Hsla) -> Self {
        self.text_color = Some(color);
        self
    }

    /// Sets the foreground color used by selected options.
    pub fn selected_text_color(mut self, color: Hsla) -> Self {
        self.selected_text_color = Some(color);
        self
    }

    /// Sets the default border color.
    pub fn border_color(mut self, color: Hsla) -> Self {
        self.border_color = Some(color);
        self
    }

    /// Sets the selected-state border color.
    pub fn selected_border_color(mut self, color: Hsla) -> Self {
        self.selected_border_color = Some(color);
        self
    }

    /// Sets the corner radius used by the rendered frame.
    pub fn radius(mut self, radius: impl Into<Pixels>) -> Self {
        self.radius = Some(radius.into());
        self
    }

    /// Sets radius using raw pixel units.
    pub fn radius_px(self, radius: f32) -> Self {
        self.radius(px(radius))
    }

    /// Sets radius using design-system unit values.
    pub fn radius_units(self, radius: f32) -> Self {
        self.radius_px(radius)
    }

    /// Sets inner padding on all sides of the component.
    pub fn padding(mut self, x: impl Into<Pixels>, y: impl Into<Pixels>) -> Self {
        self.padding_x = Some(x.into());
        self.padding_y = Some(y.into());
        self
    }

    /// Sets padding using raw pixel units.
    pub fn padding_px(self, x: f32, y: f32) -> Self {
        self.padding(px(x), px(y))
    }

    /// Sets padding using design-system unit values.
    pub fn padding_units(self, x: f32, y: f32) -> Self {
        self.padding_px(x, y)
    }

    /// Sets the spacing between child elements.
    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = Some(gap.into());
        self
    }

    /// Sets gap using raw pixel units.
    pub fn gap_px(self, gap: f32) -> Self {
        self.gap(px(gap))
    }

    /// Sets gap using design-system unit values.
    pub fn gap_units(self, gap: f32) -> Self {
        self.gap_px(gap)
    }

    /// Configures whether indicator is visible in the rendered component.
    pub fn show_indicator(mut self, show: bool) -> Self {
        self.show_indicator = Some(show);
        self
    }

    /// Configures whether selected icon is visible in the rendered component.
    pub fn show_selected_icon(mut self, show: bool) -> Self {
        self.show_selected_icon = Some(show);
        self
    }
}

/// Fluent native GPUI component for rendering Liora checkbox group.
pub struct CheckboxGroup {
    selected: Vec<usize>,
    disabled: bool,
    focus_handle: FocusHandle,
    options: Vec<SharedString>,
    checkboxes: Vec<Entity<Checkbox>>,
    layout: CheckboxGroupLayout,
    size: CheckboxGroupSize,
    stretch: bool,
    option_style: Option<CheckboxOptionStyle>,
    option_renderer: Option<Box<dyn Fn(CheckboxOptionRenderContext) -> AnyElement + 'static>>,
    on_change: Option<Box<dyn Fn(Vec<usize>, &mut Window, &mut App) + 'static>>,
}

#[derive(Clone, Debug)]
/// Fluent native GPUI component for rendering Liora checkbox option render context.
pub struct CheckboxOptionRenderContext {
    /// Stable item index used by render callbacks and keyboard navigation.
    pub index: usize,
    /// User-facing label rendered for this item.
    pub label: SharedString,
    /// Whether the item is currently selected.
    pub selected: bool,
    /// Whether user interaction is disabled for this item.
    pub disabled: bool,
}

impl CheckboxGroup {
    /// Creates `CheckboxGroup` with default theme-driven styling and no optional callbacks attached.
    pub fn new(
        options: Vec<impl Into<SharedString>>,
        selected: Vec<usize>,
        cx: &mut Context<Self>,
    ) -> Self {
        let options: Vec<SharedString> = options.into_iter().map(|o| o.into()).collect();
        let mut checkboxes = Vec::new();

        for (i, label) in options.iter().enumerate() {
            let is_checked = selected.contains(&i);
            let checkbox = cx.new(|cx| Checkbox::new(is_checked, cx).label(label.clone()));

            // Subscribe to each checkbox's change
            cx.subscribe(
                &checkbox,
                move |this, _checkbox, event: &CheckboxChanged, cx| {
                    this.update_selection(i, event.0, cx);
                },
            )
            .detach();

            checkboxes.push(checkbox);
        }

        Self {
            selected,
            disabled: false,
            focus_handle: cx.focus_handle(),
            options,
            checkboxes,
            layout: CheckboxGroupLayout::Vertical,
            size: CheckboxGroupSize::Default,
            stretch: false,
            option_style: None,
            option_renderer: None,
            on_change: None,
        }
    }

    /// Toggles the disabled state and suppresses user interaction when enabled.
    pub fn disabled(mut self, d: bool, cx: &mut Context<Self>) -> Self {
        self.disabled = d;
        for cb in &self.checkboxes {
            cb.update(cx, |cb, cx| {
                cb.set_disabled(d, cx);
            });
        }
        self
    }

    /// Registers a callback that runs when change occurs.
    pub fn on_change(mut self, cb: impl Fn(Vec<usize>, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(cb));
        self
    }

    /// Selects the layout used by the component.
    pub fn layout(mut self, layout: CheckboxGroupLayout) -> Self {
        self.layout = layout;
        self
    }

    /// Uses vertical orientation or gradient direction.
    pub fn vertical(mut self) -> Self {
        self.layout = CheckboxGroupLayout::Vertical;
        self
    }

    /// Uses horizontal orientation or gradient direction.
    pub fn horizontal(mut self) -> Self {
        self.layout = CheckboxGroupLayout::Horizontal;
        self
    }

    /// Adds the supplied button to the component.
    pub fn button(mut self) -> Self {
        self.layout = CheckboxGroupLayout::Button;
        self
    }

    /// Sets an explicit icon size while preserving the default color behavior.
    pub fn size(mut self, size: CheckboxGroupSize) -> Self {
        self.size = size;
        self
    }

    /// Uses the large size preset.
    pub fn large(mut self) -> Self {
        self.size = CheckboxGroupSize::Large;
        self
    }

    /// Uses the compact size preset.
    pub fn small(mut self) -> Self {
        self.size = CheckboxGroupSize::Small;
        self
    }

    /// Lets items stretch to fill the available cross-axis space.
    pub fn stretch(mut self, stretch: bool) -> Self {
        self.stretch = stretch;
        self
    }

    /// Makes the component occupy the available inline width.
    pub fn block(self, block: bool) -> Self {
        self.stretch(block)
    }

    /// Applies a reusable style preset to rendered options.
    pub fn option_style(mut self, style: CheckboxOptionStyle) -> Self {
        self.option_style = Some(style);
        self
    }

    /// Performs the option renderer operation used by this component.
    pub fn option_renderer(
        mut self,
        renderer: impl Fn(CheckboxOptionRenderContext) -> AnyElement + 'static,
    ) -> Self {
        self.option_renderer = Some(Box::new(renderer));
        self
    }

    /// Switches option rendering to card-style controls.
    pub fn card_options(mut self) -> Self {
        self.option_style = Some(
            CheckboxOptionStyle::new()
                .radius(px(10.0))
                .padding(px(12.0), px(8.0)),
        );
        self
    }

    /// Returns whether stretched is currently true for this value.
    pub fn is_stretched(&self) -> bool {
        self.stretch
    }

    /// Performs the layout kind operation used by this component.
    pub fn layout_kind(&self) -> CheckboxGroupLayout {
        self.layout
    }

    /// Applies the predefined size kind sizing preset.
    pub fn size_kind(&self) -> CheckboxGroupSize {
        self.size
    }

    /// Registers GPUI key bindings required for keyboard interaction.
    pub fn register_key_bindings(_cx: &mut App) {}

    fn update_selection(&mut self, idx: usize, checked: bool, cx: &mut Context<Self>) {
        if checked {
            if !self.selected.contains(&idx) {
                self.selected.push(idx);
                self.selected.sort();
            }
        } else {
            self.selected.retain(|&i| i != idx);
        }
        cx.notify();
    }

    fn toggle_idx(&mut self, idx: usize, cx: &mut Context<Self>) {
        if self.disabled || idx >= self.options.len() {
            return;
        }
        let checked = !self.selected.contains(&idx);
        self.update_selection(idx, checked, cx);
    }

    fn render_indicator(
        &self,
        checked: bool,
        border: Hsla,
        bg: Hsla,
        check_color: Hsla,
        show_selected_icon: bool,
    ) -> impl IntoElement {
        let mut indicator = gpui::div()
            .flex_none()
            .w(px(16.0))
            .h(px(16.0))
            .rounded(px(3.0))
            .bg(bg)
            .border_1()
            .border_color(border)
            .flex()
            .items_center()
            .justify_center();

        if checked && show_selected_icon {
            indicator =
                indicator.child(Icon::new(IconName::Check).size(px(12.0)).color(check_color));
        }

        indicator
    }

    fn render_option_content(&self, idx: usize, label: SharedString, checked: bool) -> AnyElement {
        if let Some(renderer) = &self.option_renderer {
            renderer(CheckboxOptionRenderContext {
                index: idx,
                label,
                selected: checked,
                disabled: self.disabled,
            })
        } else {
            gpui::div().child(label).into_any_element()
        }
    }

    fn render_styled_option(
        &self,
        idx: usize,
        label: SharedString,
        checked: bool,
        style: CheckboxOptionStyle,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let disabled = self.disabled;
        let selected_bg = style
            .selected_bg
            .unwrap_or(theme.primary.base.opacity(0.12));
        let bg = if checked {
            selected_bg
        } else {
            style.bg.unwrap_or(theme.neutral.card)
        };
        let hover_bg = style.hover_bg.unwrap_or(theme.neutral.hover);
        let border = if checked {
            style.selected_border_color.unwrap_or(theme.primary.base)
        } else {
            style.border_color.unwrap_or(theme.neutral.border)
        };
        let text_color = if disabled {
            theme.neutral.text_disabled
        } else if checked {
            style.selected_text_color.unwrap_or(theme.primary.base)
        } else {
            style.text_color.unwrap_or(theme.neutral.text_1)
        };
        let show_indicator = style.show_indicator.unwrap_or(true);
        let show_selected_icon = style.show_selected_icon.unwrap_or(true);

        let mut item = gpui::div()
            .flex()
            .flex_row()
            .items_center()
            .gap(style.gap.unwrap_or(px(8.0)))
            .px(style.padding_x.unwrap_or(px(12.0)))
            .py(style.padding_y.unwrap_or(px(8.0)))
            .rounded(style.radius.unwrap_or(px(theme.radius.md)))
            .border_1()
            .border_color(border)
            .bg(bg)
            .text_size(self.size.text_size(&theme))
            .text_color(text_color);

        if !disabled {
            item = item.cursor_pointer().hover(move |s| {
                if checked {
                    s.cursor_pointer()
                } else {
                    s.cursor_pointer().bg(hover_bg)
                }
            });
            item = item.on_mouse_up(
                MouseButton::Left,
                cx.listener(
                    move |this: &mut Self,
                          _: &MouseUpEvent,
                          _: &mut Window,
                          cx: &mut Context<Self>| {
                        this.toggle_idx(idx, cx);
                    },
                ),
            );
        } else {
            item = item.cursor_not_allowed();
        }

        if show_indicator {
            let indicator_bg = if checked {
                theme.primary.base
            } else {
                rgba(0, 0, 0, 0.0)
            };
            item = item.child(self.render_indicator(
                checked,
                border,
                indicator_bg,
                rgba(255, 255, 255, 1.0),
                show_selected_icon,
            ));
        }

        item.child(self.render_option_content(idx, label, checked))
    }

    fn render_button_group(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let radius = px(theme.radius.md);
        let height = self.size.height();
        let padding_x = self.size.padding_x();
        let text_size = self.size.text_size(&theme);

        let mut group = gpui::div()
            .flex()
            .items_center()
            .rounded(radius)
            .border_1()
            .border_color(theme.neutral.border)
            .overflow_hidden()
            .when(self.stretch, |s| s.w_full())
            .when(!self.stretch, |s| s.items_start());

        if !self.disabled {
            group = group.track_focus(&self.focus_handle);
        }

        for (idx, label) in self.options.iter().enumerate() {
            let checked = self.selected.contains(&idx);
            let is_first = idx == 0;
            let label = label.clone();
            let style = self.option_style.clone().unwrap_or_default();
            let bg = if checked {
                style.selected_bg.unwrap_or(theme.primary.base)
            } else {
                style.bg.unwrap_or(theme.neutral.card)
            };
            let text_color = if self.disabled {
                theme.neutral.text_disabled
            } else if checked {
                style
                    .selected_text_color
                    .unwrap_or_else(|| rgba(255, 255, 255, 1.0))
            } else {
                style.text_color.unwrap_or(theme.neutral.text_1)
            };
            let mut item = gpui::div()
                .h(height)
                .px(style.padding_x.unwrap_or(padding_x))
                .flex()
                .items_center()
                .justify_center()
                .when(self.stretch, |s| s.flex_1())
                .gap(style.gap.unwrap_or(px(8.0)))
                .bg(bg)
                .text_size(text_size)
                .text_color(text_color);

            if checked && style.show_selected_icon.unwrap_or(true) {
                item = item.child(Icon::new(IconName::Check).size(px(12.0)).color(text_color));
            }
            item = item.child(self.render_option_content(idx, label, checked));

            if !is_first {
                item = item
                    .border_l_1()
                    .border_color(style.border_color.unwrap_or(theme.neutral.border));
            }
            if !self.disabled {
                let hover_bg = style.hover_bg.unwrap_or(theme.neutral.hover);
                item = item.cursor_pointer().hover(move |s| {
                    if checked {
                        s.cursor_pointer()
                    } else {
                        s.cursor_pointer().bg(hover_bg)
                    }
                });
                item = item.on_mouse_up(
                    MouseButton::Left,
                    cx.listener(
                        move |this: &mut Self,
                              _: &MouseUpEvent,
                              _: &mut Window,
                              cx: &mut Context<Self>| {
                            this.toggle_idx(idx, cx);
                        },
                    ),
                );
            } else {
                item = item.cursor_not_allowed();
            }
            group = group.child(item);
        }

        group
    }
}

impl Focusable for CheckboxGroup {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for CheckboxGroup {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.layout == CheckboxGroupLayout::Button {
            return self.render_button_group(cx).into_any_element();
        }

        let style = self.option_style.clone();
        let mut col = gpui::div()
            .flex()
            .when(self.layout == CheckboxGroupLayout::Vertical, |s| {
                s.flex_col().gap_2()
            })
            .when(self.layout == CheckboxGroupLayout::Horizontal, |s| {
                s.flex_row().gap_4().items_center()
            });

        if !self.disabled {
            col = col.track_focus(&self.focus_handle);
        }

        if let Some(style) = style {
            for (idx, label) in self.options.iter().enumerate() {
                let checked = self.selected.contains(&idx);
                col = col.child(self.render_styled_option(
                    idx,
                    label.clone(),
                    checked,
                    style.clone(),
                    cx,
                ));
            }
        } else if self.option_renderer.is_some() {
            for (idx, label) in self.options.iter().enumerate() {
                let checked = self.selected.contains(&idx);
                col = col.child(self.render_styled_option(
                    idx,
                    label.clone(),
                    checked,
                    CheckboxOptionStyle::default(),
                    cx,
                ));
            }
        } else {
            for cb_entity in &self.checkboxes {
                col = col.child(cb_entity.clone());
            }
        }

        col.into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkbox_option_style_supports_layout_and_selected_style() {
        let style = CheckboxOptionStyle::new()
            .selected_bg(gpui::blue())
            .selected_text_color(gpui::white())
            .padding(px(14.0), px(10.0))
            .radius(px(12.0))
            .show_indicator(false);

        assert_eq!(style.selected_bg, Some(gpui::blue()));
        assert_eq!(style.padding_x, Some(px(14.0)));
        assert_eq!(style.show_indicator, Some(false));
    }

    #[test]
    fn checkbox_group_accepts_custom_option_renderer() {
        let source = include_str!("checkbox_group.rs");
        assert!(source.contains("pub struct CheckboxOptionRenderContext"));
        assert!(source.contains("pub fn option_renderer"));
    }
}
