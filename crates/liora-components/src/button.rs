//! Button module.
//!
//! This public module implements the Liora button component, custom color model, gradient model, and icon placement API. It keeps the reusable
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

use crate::gpui_compat::element_id;
use crate::motion::spin_icon;
use gpui::{
    AbsoluteLength, AnyElement, App, Background, Component, ElementId, Hsla, IntoElement,
    RenderOnce, Rgba, SharedString, Window, linear_color_stop, linear_gradient, prelude::*, px,
};
use liora_core::{Config, stable_unique_id};
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use liora_theme::{ButtonSize, ButtonVariant, ButtonVariantColors, Theme};

fn rgba(r: u8, g: u8, b: u8, a: f32) -> Hsla {
    Rgba {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a,
    }
    .into()
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// Fluent native GPUI component for rendering Liora button colors.
pub struct ButtonColors {
    /// Base background color used in the normal state.
    pub bg: Hsla,
    /// Background color used while the control is hovered.
    pub hover_bg: Hsla,
    /// Background color used while the control is pressed or active.
    pub active_bg: Hsla,
    /// Foreground text color used in the normal state.
    pub text: Hsla,
    /// Border color used in the normal state.
    pub border: Hsla,
    /// Foreground text color used while the control is hovered.
    pub text_hover: Hsla,
    /// Border color used while the control is hovered.
    pub border_hover: Hsla,
    /// Background color used when the control is disabled.
    pub disabled_bg: Hsla,
    /// Foreground text color used when the control is disabled.
    pub disabled_text: Hsla,
    /// Border color used when the control is disabled.
    pub disabled_border: Hsla,
}

impl ButtonColors {
    /// Creates a filled custom button color set from background and foreground colors.
    pub fn filled(bg: Hsla, text: Hsla) -> Self {
        Self {
            bg,
            hover_bg: derive_hover_bg(bg),
            active_bg: derive_active_bg(bg),
            text,
            border: bg,
            text_hover: text,
            border_hover: derive_hover_bg(bg),
            disabled_bg: derive_disabled_bg(bg),
            disabled_text: text.opacity(0.58),
            disabled_border: derive_disabled_bg(bg),
        }
    }

    /// Creates an outline custom button color set from accent, text, and background colors.
    pub fn outline(accent: Hsla, text: Hsla, bg: Hsla) -> Self {
        Self {
            bg,
            hover_bg: accent.opacity(0.10),
            active_bg: accent.opacity(0.18),
            text,
            border: accent,
            text_hover: accent,
            border_hover: derive_hover_bg(accent),
            disabled_bg: bg.opacity(0.35),
            disabled_text: text.opacity(0.45),
            disabled_border: accent.opacity(0.30),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
/// Fluent native GPUI component for rendering Liora button gradient.
pub struct ButtonGradient {
    /// Starting color for the gradient.
    pub from: Hsla,
    /// Ending color for the gradient.
    pub to: Hsla,
    /// Gradient angle in degrees.
    pub angle: f32,
    /// Starting gradient color used while hovered.
    pub hover_from: Hsla,
    /// Ending gradient color used while hovered.
    pub hover_to: Hsla,
    /// Starting gradient color used while pressed or active.
    pub active_from: Hsla,
    /// Ending gradient color used while pressed or active.
    pub active_to: Hsla,
    /// Starting gradient color used while disabled.
    pub disabled_from: Hsla,
    /// Ending gradient color used while disabled.
    pub disabled_to: Hsla,
}

impl ButtonGradient {
    /// Creates `ButtonGradient` initialized from the supplied from, and to.
    pub fn new(from: Hsla, to: Hsla) -> Self {
        Self::with_angle(from, to, 90.0)
    }

    /// Sets the angle value used by the component.
    pub fn with_angle(from: Hsla, to: Hsla, angle: f32) -> Self {
        Self {
            from,
            to,
            angle,
            hover_from: derive_hover_bg(from),
            hover_to: derive_hover_bg(to),
            active_from: derive_active_bg(from),
            active_to: derive_active_bg(to),
            disabled_from: derive_disabled_bg(from),
            disabled_to: derive_disabled_bg(to),
        }
    }

    fn background(&self) -> Background {
        gradient_background(self.angle, self.from, self.to)
    }

    fn hover_background(&self) -> Background {
        gradient_background(self.angle, self.hover_from, self.hover_to)
    }

    fn active_background(&self) -> Background {
        gradient_background(self.angle, self.active_from, self.active_to)
    }

    fn disabled_background(&self) -> Background {
        gradient_background(self.angle, self.disabled_from, self.disabled_to)
    }
}

fn derive_hover_bg(color: Hsla) -> Hsla {
    color.blend(gpui::white().opacity(0.14))
}

fn derive_active_bg(color: Hsla) -> Hsla {
    color.blend(gpui::black().opacity(0.18))
}

fn derive_disabled_bg(color: Hsla) -> Hsla {
    // Keep the original hue so disabled custom buttons still look related to
    // the caller-provided color. Only soften saturation/contrast and opacity.
    Hsla {
        h: color.h,
        s: (color.s * 0.45).clamp(0.0, 1.0),
        l: (color.l + (1.0 - color.l) * 0.38).clamp(0.0, 1.0),
        a: (color.a * 0.62).clamp(0.0, 1.0),
    }
}

fn gradient_background(angle: f32, from: Hsla, to: Hsla) -> Background {
    linear_gradient(
        angle,
        linear_color_stop(from, 0.0),
        linear_color_stop(to, 1.0),
    )
}

/// Options that control button icon behavior.
pub enum ButtonIcon {
    /// Stores a Lucide icon name for deferred icon construction.
    IconName(IconName),
    /// Stores the separator or button content as an icon.
    Icon(Icon),
    /// Stores a caller-provided GPUI element.
    Element(AnyElement),
}

impl From<IconName> for ButtonIcon {
    fn from(name: IconName) -> Self {
        ButtonIcon::IconName(name)
    }
}

impl From<AnyElement> for ButtonIcon {
    fn from(el: AnyElement) -> Self {
        ButtonIcon::Element(el)
    }
}

impl From<Icon> for ButtonIcon {
    fn from(icon: Icon) -> Self {
        ButtonIcon::Icon(icon)
    }
}

/// Fluent native GPUI component for rendering Liora button.
pub struct Button {
    label: SharedString,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
    loading: bool,
    secondary: bool,
    background: bool,
    border: bool,
    rounded: Option<AbsoluteLength>,
    id: Option<ElementId>,
    icon_start: Option<ButtonIcon>,
    icon_end: Option<ButtonIcon>,
    icon_top: Option<IconName>,
    icon_bottom: Option<IconName>,
    icon_only: Option<IconName>,
    custom_colors: Option<ButtonColors>,
    gradient: Option<ButtonGradient>,
    on_click: Option<Box<dyn Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl Button {
    /// Creates `Button` initialized from the supplied label.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            disabled: false,
            loading: false,
            secondary: false,
            background: true,
            border: true,
            rounded: None,
            id: None,
            icon_start: None,
            icon_end: None,
            icon_top: None,
            icon_bottom: None,
            icon_only: None,
            custom_colors: None,
            gradient: None,
            on_click: None,
        }
    }
    /// Selects the visual variant used for styling.
    pub fn variant(mut self, v: ButtonVariant) -> Self {
        self.variant = v;
        self
    }
    /// Applies the primary semantic visual variant.
    pub fn primary(mut self) -> Self {
        self.variant = ButtonVariant::Primary;
        self
    }
    /// Applies the tertiary low-emphasis visual variant.
    pub fn tertiary(mut self) -> Self {
        self.variant = ButtonVariant::Tertiary;
        self
    }
    /// Applies the text-only visual variant.
    pub fn text(mut self) -> Self {
        self.variant = ButtonVariant::Text;
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
    /// Sets an explicit icon size while preserving the default color behavior.
    pub fn size(mut self, s: ButtonSize) -> Self {
        self.size = s;
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
    /// Toggles the disabled state and suppresses user interaction when enabled.
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }
    /// Toggles the loading state and associated spinner treatment.
    pub fn loading(mut self, l: bool) -> Self {
        self.loading = l;
        self
    }
    /// Sets the secondary value used by the component.
    pub fn secondary(mut self) -> Self {
        self.secondary = true;
        self
    }
    /// Toggles or applies the component background treatment.
    pub fn background(mut self, show: bool) -> Self {
        self.background = show;
        self
    }
    /// Toggles or applies the component border treatment.
    pub fn border(mut self, show: bool) -> Self {
        self.border = show;
        self
    }
    /// Applies rounded-corner styling.
    pub fn rounded(mut self, r: impl Into<AbsoluteLength>) -> Self {
        self.rounded = Some(r.into());
        self
    }

    /// Applies the predefined rounded sm sizing preset.
    pub fn rounded_sm(self) -> Self {
        self.rounded(px(4.0))
    }

    /// Applies the predefined rounded md sizing preset.
    pub fn rounded_md(self) -> Self {
        self.rounded(px(12.0))
    }

    /// Applies the predefined rounded lg sizing preset.
    pub fn rounded_lg(self) -> Self {
        self.rounded(px(20.0))
    }

    /// Applies fully rounded pill styling.
    pub fn pill(self) -> Self {
        self.rounded(px(9999.0))
    }
    /// Assigns a stable element id used by GPUI state, hit testing, and automated interaction tests.
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }
    /// Sets the leading icon rendered before the button label.
    pub fn icon_start(mut self, icon: impl Into<ButtonIcon>) -> Self {
        self.icon_start = Some(icon.into());
        self
    }
    /// Sets the trailing icon rendered after the button label.
    pub fn icon_end(mut self, icon: impl Into<ButtonIcon>) -> Self {
        self.icon_end = Some(icon.into());
        self
    }
    /// Sets the icon rendered above the button label.
    pub fn icon_top(mut self, icon: IconName) -> Self {
        self.icon_top = Some(icon);
        self
    }
    /// Sets the icon rendered below the button label.
    pub fn icon_bottom(mut self, icon: IconName) -> Self {
        self.icon_bottom = Some(icon);
        self
    }
    /// Renders the button as an icon-only control.
    pub fn icon_only(mut self, icon: IconName) -> Self {
        self.icon_only = Some(icon);
        self
    }
    /// Sets the colors value used by the component.
    pub fn colors(mut self, colors: ButtonColors) -> Self {
        self.custom_colors = Some(colors);
        self.gradient = None;
        self
    }

    /// Sets the custom colors value used by the component.
    pub fn custom_colors(self, colors: ButtonColors) -> Self {
        self.colors(colors)
    }

    /// Sets the custom color used by the rendered component.
    pub fn custom_color(mut self, bg: Hsla, text: Hsla) -> Self {
        self.custom_colors = Some(ButtonColors::filled(bg, text));
        self.gradient = None;
        self
    }

    /// Sets the gradient value used by the component.
    pub fn gradient(mut self, from: Hsla, to: Hsla) -> Self {
        self.gradient = Some(ButtonGradient::new(from, to));
        self.custom_colors = None;
        self
    }

    /// Sets the gradient with angle value used by the component.
    pub fn gradient_with_angle(mut self, angle: f32, from: Hsla, to: Hsla) -> Self {
        self.gradient = Some(ButtonGradient::with_angle(from, to, angle));
        self.custom_colors = None;
        self
    }

    /// Registers a callback that runs when click occurs.
    pub fn on_click(
        mut self,
        cb: impl Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(cb));
        self
    }

    fn resolved_colors(&self, theme: &Theme) -> ButtonVariantColors {
        if let Some(colors) = self.custom_colors {
            if self.disabled {
                return ButtonVariantColors {
                    bg: colors.disabled_bg,
                    hover_bg: colors.disabled_bg,
                    active_bg: colors.disabled_bg,
                    text: colors.disabled_text,
                    border: colors.disabled_border,
                    text_hover: colors.disabled_text,
                    border_hover: colors.disabled_border,
                };
            }

            return ButtonVariantColors {
                bg: colors.bg,
                hover_bg: colors.hover_bg,
                active_bg: colors.active_bg,
                text: colors.text,
                border: colors.border,
                text_hover: colors.text_hover,
                border_hover: colors.border_hover,
            };
        }

        if self.gradient.is_some() {
            let text = theme.neutral.inverted;
            return ButtonVariantColors {
                bg: rgba(0, 0, 0, 0.0),
                hover_bg: rgba(0, 0, 0, 0.0),
                active_bg: rgba(0, 0, 0, 0.0),
                text: if self.disabled {
                    text.opacity(0.58)
                } else {
                    text
                },
                border: rgba(0, 0, 0, 0.0),
                text_hover: if self.disabled {
                    text.opacity(0.58)
                } else {
                    text
                },
                border_hover: rgba(0, 0, 0, 0.0),
            };
        }

        if self.disabled {
            ButtonVariantColors {
                bg: rgba(0, 0, 0, 0.0),
                hover_bg: rgba(0, 0, 0, 0.0),
                active_bg: rgba(0, 0, 0, 0.0),
                text: theme.neutral.text_disabled,
                border: theme.neutral.border,
                text_hover: theme.neutral.text_disabled,
                border_hover: theme.neutral.border,
            }
        } else {
            theme.color_by_variant(self.variant, self.secondary, self.background, self.border)
        }
    }

    fn icon_size(&self) -> f32 {
        match self.size {
            ButtonSize::Small => 12.0,
            ButtonSize::Default => 14.0,
            ButtonSize::Large => 16.0,
        }
    }

    fn render_with_theme(
        self,
        theme: Theme,
        window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        let c = self.resolved_colors(&theme);
        let h = self.size.height();
        let px_h = self.size.padding_x();
        let fs = match self.size {
            ButtonSize::Small => theme.font_size.xs,
            ButtonSize::Default => theme.font_size.md,
            ButtonSize::Large => theme.font_size.lg,
        };
        let r = self.rounded.unwrap_or_else(|| px(theme.radius.md).into());
        let id = self.id.clone().unwrap_or_else(|| {
            stable_unique_id(
                format!(
                    "liora-button:{}:{:?}:{:?}:secondary={}:background={}:border={}:rounded={:?}",
                    self.label,
                    self.variant,
                    self.size,
                    self.secondary,
                    self.background,
                    self.border,
                    self.rounded
                ),
                "liora-button",
                window,
                cx,
            )
            .into()
        });
        let icon_sz = self.icon_size();

        let icon_only = self.icon_only.is_some();
        let vertical = self.icon_top.is_some() || self.icon_bottom.is_some() || icon_only;

        let label = self.label.clone();
        let hover_group = SharedString::from(format!("{}:hover", id));

        let gradient = self.gradient.clone();
        let mut div = gpui::div()
            .flex()
            .justify_center()
            .items_center()
            .gap_1()
            .h(px(if vertical { h + icon_sz + 6.0 } else { h }))
            .rounded(r)
            .text_color(c.text)
            .text_size(px(fs));

        div = if let Some(gradient) = gradient.as_ref() {
            if self.disabled {
                div.bg(gradient.disabled_background())
            } else {
                div.bg(gradient.background())
            }
        } else {
            div.bg(c.bg)
        };

        if vertical {
            div = div.flex_col();
            if !icon_only {
                div = div.px(px(px_h));
            }
        } else {
            div = div.flex_row().px(px(px_h));
        }

        if icon_only {
            div = div.size(px(h)).w(px(h)); // square button
        }

        if !self.disabled {
            div = div.cursor_pointer();
        } else {
            div = div.cursor_not_allowed();
        }
        if !c.border.is_transparent() {
            div = div.border_1().border_color(c.border);
        }
        if self.disabled {
            if let Some(icon) = self.icon_only {
                let sz = icon_sz * 2.0;
                let group = hover_group.clone();
                return div
                    .child(
                        Icon::new(icon)
                            .size(px(sz))
                            .color(c.text)
                            .group_hover_color(group, c.text_hover),
                    )
                    .into_any_element();
            }
            return div.child(label.clone()).into_any_element();
        }

        // Build children: icons + label
        let mut children: Vec<AnyElement> = Vec::new();

        if let Some(icon) = self.icon_only {
            let group = hover_group.clone();
            children.push(
                Icon::new(icon)
                    .size(px(icon_sz))
                    .color(c.text)
                    .group_hover_color(group, c.text_hover)
                    .into_any_element(),
            );
        } else if self.loading {
            let sz = icon_sz;
            let group = hover_group.clone();
            children.push(
                spin_icon(
                    element_id(format!("{id}:loading-spinner-motion")),
                    Icon::new(IconName::LoaderCircle)
                        .size(px(sz))
                        .color(c.text)
                        .group_hover_color(group, c.text_hover),
                )
                .into_any_element(),
            );
            children.push(gpui::div().child(label.clone()).into_any_element());
        } else {
            let lbl = label.clone();
            // icon_top
            if let Some(icon) = self.icon_top {
                let sz = icon_sz;
                let group = hover_group.clone();
                children.push(
                    Icon::new(icon)
                        .size(px(sz))
                        .color(c.text)
                        .group_hover_color(group, c.text_hover)
                        .into_any_element(),
                );
            }
            // icon_start
            if let Some(icon) = self.icon_start {
                match icon {
                    ButtonIcon::IconName(name) => {
                        let group = hover_group.clone();
                        children.push(
                            Icon::new(name)
                                .size(px(icon_sz))
                                .color(c.text)
                                .group_hover_color(group, c.text_hover)
                                .into_any_element(),
                        );
                    }
                    ButtonIcon::Icon(icon) => {
                        let group = hover_group.clone();
                        children.push(
                            icon.size(px(icon_sz))
                                .color(c.text)
                                .group_hover_color(group, c.text_hover)
                                .into_any_element(),
                        );
                    }
                    ButtonIcon::Element(el) => children.push(el),
                }
            }
            // label
            children.push(gpui::div().child(lbl).into_any_element());
            // icon_end
            if let Some(icon) = self.icon_end {
                match icon {
                    ButtonIcon::IconName(name) => {
                        let group = hover_group.clone();
                        children.push(
                            Icon::new(name)
                                .size(px(icon_sz))
                                .color(c.text)
                                .group_hover_color(group, c.text_hover)
                                .into_any_element(),
                        );
                    }
                    ButtonIcon::Icon(icon) => {
                        let group = hover_group.clone();
                        children.push(
                            icon.size(px(icon_sz))
                                .color(c.text)
                                .group_hover_color(group, c.text_hover)
                                .into_any_element(),
                        );
                    }
                    ButtonIcon::Element(el) => children.push(el),
                }
            }
            // icon_bottom
            if let Some(icon) = self.icon_bottom {
                let sz = icon_sz;
                let group = hover_group.clone();
                children.push(
                    Icon::new(icon)
                        .size(px(sz))
                        .color(c.text)
                        .group_hover_color(group, c.text_hover)
                        .into_any_element(),
                );
            }
        }

        let click_handler = self.on_click;
        let hover_gradient = gradient.clone();
        let active_gradient = gradient.clone();

        div.id(id)
            .group(hover_group)
            .hover(move |style| {
                let hover_bg: gpui::Fill = hover_gradient
                    .as_ref()
                    .map(ButtonGradient::hover_background)
                    .map_or_else(|| c.hover_bg.into(), Into::into);
                let mut s = style.bg(hover_bg).text_color(c.text_hover);
                if !c.border_hover.is_transparent() {
                    s = s.border_color(c.border_hover);
                }
                s
            })
            .active(move |style| {
                let active_bg: gpui::Fill = active_gradient
                    .as_ref()
                    .map(ButtonGradient::active_background)
                    .map_or_else(|| c.active_bg.into(), Into::into);
                style.bg(active_bg)
            })
            .on_click(move |event, window, cx| {
                if let Some(ref handler) = click_handler {
                    handler(event, window, cx);
                }
            })
            .children(children)
            .into_any_element()
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        self.render_with_theme(theme, _window, cx)
    }
}

impl IntoElement for Button {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_rounded_helpers_set_custom_radius() {
        assert!(Button::new("small").rounded_sm().rounded.is_some());
        assert!(Button::new("pill").pill().rounded.is_some());
    }

    #[test]
    fn button_loading_icon_uses_spin_motion() {
        let source = include_str!("button.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains("spin_icon("));
        assert!(source.contains("loading-spinner-motion"));
    }

    #[test]
    fn custom_color_derives_interaction_states() {
        let bg = rgba(99, 102, 241, 1.0);
        let colors = ButtonColors::filled(bg, gpui::white());

        assert_ne!(colors.bg, colors.hover_bg);
        assert_ne!(colors.bg, colors.active_bg);
        assert_eq!(colors.disabled_bg.h, colors.bg.h);
        assert!(colors.disabled_bg.s < colors.bg.s);
        assert!(colors.disabled_bg.a < colors.bg.a);
    }

    #[test]
    fn gradient_builder_derives_state_gradients() {
        let button = Button::new("gradient").gradient(gpui::blue(), gpui::green());
        let gradient = button.gradient.unwrap();

        assert_eq!(gradient.angle, 90.0);
        assert_ne!(gradient.from, gradient.hover_from);
        assert_ne!(gradient.to, gradient.active_to);
    }
}
