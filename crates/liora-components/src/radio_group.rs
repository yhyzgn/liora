use gpui::{
    AnyElement, App, Context, FocusHandle, Focusable, Hsla, KeyBinding, MouseButton, MouseUpEvent,
    Pixels, Render, Rgba, SharedString, Window, prelude::*, px,
};
use liora_core::Config;

fn rgba(r: u8, g: u8, b: u8, a: f32) -> Hsla {
    Rgba {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a,
    }
    .into()
}

gpui::actions!(radio_group, [RadioGroupUp, RadioGroupDown]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RadioGroupLayout {
    #[default]
    Vertical,
    Horizontal,
    Button,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RadioGroupSize {
    Large,
    #[default]
    Default,
    Small,
}

impl RadioGroupSize {
    fn height(self) -> Pixels {
        match self {
            RadioGroupSize::Large => px(38.0),
            RadioGroupSize::Default => px(32.0),
            RadioGroupSize::Small => px(24.0),
        }
    }

    fn padding_x(self) -> Pixels {
        match self {
            RadioGroupSize::Large => px(18.0),
            RadioGroupSize::Default => px(14.0),
            RadioGroupSize::Small => px(10.0),
        }
    }

    fn text_size(self, theme: &liora_theme::Theme) -> Pixels {
        match self {
            RadioGroupSize::Large => px(theme.font_size.md),
            RadioGroupSize::Default => px(theme.font_size.md),
            RadioGroupSize::Small => px(theme.font_size.sm),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct RadioOptionStyle {
    pub bg: Option<Hsla>,
    pub selected_bg: Option<Hsla>,
    pub hover_bg: Option<Hsla>,
    pub text_color: Option<Hsla>,
    pub selected_text_color: Option<Hsla>,
    pub border_color: Option<Hsla>,
    pub selected_border_color: Option<Hsla>,
    pub radius: Option<Pixels>,
    pub padding_x: Option<Pixels>,
    pub padding_y: Option<Pixels>,
    pub gap: Option<Pixels>,
    pub show_indicator: Option<bool>,
    pub show_selected_icon: Option<bool>,
}

impl RadioOptionStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bg(mut self, color: Hsla) -> Self {
        self.bg = Some(color);
        self
    }

    pub fn selected_bg(mut self, color: Hsla) -> Self {
        self.selected_bg = Some(color);
        self
    }

    pub fn hover_bg(mut self, color: Hsla) -> Self {
        self.hover_bg = Some(color);
        self
    }

    pub fn text_color(mut self, color: Hsla) -> Self {
        self.text_color = Some(color);
        self
    }

    pub fn selected_text_color(mut self, color: Hsla) -> Self {
        self.selected_text_color = Some(color);
        self
    }

    pub fn border_color(mut self, color: Hsla) -> Self {
        self.border_color = Some(color);
        self
    }

    pub fn selected_border_color(mut self, color: Hsla) -> Self {
        self.selected_border_color = Some(color);
        self
    }

    pub fn radius(mut self, radius: impl Into<Pixels>) -> Self {
        self.radius = Some(radius.into());
        self
    }

    pub fn radius_px(self, radius: f32) -> Self {
        self.radius(px(radius))
    }

    pub fn radius_units(self, radius: f32) -> Self {
        self.radius_px(radius)
    }

    pub fn padding(mut self, x: impl Into<Pixels>, y: impl Into<Pixels>) -> Self {
        self.padding_x = Some(x.into());
        self.padding_y = Some(y.into());
        self
    }

    pub fn padding_px(self, x: f32, y: f32) -> Self {
        self.padding(px(x), px(y))
    }

    pub fn padding_units(self, x: f32, y: f32) -> Self {
        self.padding_px(x, y)
    }

    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = Some(gap.into());
        self
    }

    pub fn gap_px(self, gap: f32) -> Self {
        self.gap(px(gap))
    }

    pub fn gap_units(self, gap: f32) -> Self {
        self.gap_px(gap)
    }

    pub fn show_indicator(mut self, show: bool) -> Self {
        self.show_indicator = Some(show);
        self
    }

    pub fn show_selected_icon(mut self, show: bool) -> Self {
        self.show_selected_icon = Some(show);
        self
    }
}

pub struct RadioGroup {
    selected: usize,
    disabled: bool,
    options: Vec<SharedString>,
    layout: RadioGroupLayout,
    size: RadioGroupSize,
    stretch: bool,
    option_style: Option<RadioOptionStyle>,
    option_renderer: Option<Box<dyn Fn(RadioOptionRenderContext) -> AnyElement + 'static>>,
    focus_handle: FocusHandle,
    on_change: Option<Box<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
}

#[derive(Clone, Debug)]
pub struct RadioOptionRenderContext {
    pub index: usize,
    pub label: SharedString,
    pub selected: bool,
    pub disabled: bool,
    pub focused: bool,
}

impl RadioGroup {
    pub fn new(
        options: Vec<impl Into<SharedString>>,
        selected: usize,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            selected,
            disabled: false,
            options: options.into_iter().map(|o| o.into()).collect(),
            layout: RadioGroupLayout::Vertical,
            size: RadioGroupSize::Default,
            stretch: false,
            option_style: None,
            option_renderer: None,
            focus_handle: cx.focus_handle(),
            on_change: None,
        }
    }

    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }

    pub fn layout(mut self, layout: RadioGroupLayout) -> Self {
        self.layout = layout;
        self
    }

    pub fn vertical(mut self) -> Self {
        self.layout = RadioGroupLayout::Vertical;
        self
    }

    pub fn horizontal(mut self) -> Self {
        self.layout = RadioGroupLayout::Horizontal;
        self
    }

    pub fn button(mut self) -> Self {
        self.layout = RadioGroupLayout::Button;
        self
    }

    pub fn size(mut self, size: RadioGroupSize) -> Self {
        self.size = size;
        self
    }

    pub fn large(mut self) -> Self {
        self.size = RadioGroupSize::Large;
        self
    }

    pub fn small(mut self) -> Self {
        self.size = RadioGroupSize::Small;
        self
    }

    pub fn stretch(mut self, stretch: bool) -> Self {
        self.stretch = stretch;
        self
    }

    pub fn block(self, block: bool) -> Self {
        self.stretch(block)
    }

    pub fn option_style(mut self, style: RadioOptionStyle) -> Self {
        self.option_style = Some(style);
        self
    }

    pub fn option_renderer(
        mut self,
        renderer: impl Fn(RadioOptionRenderContext) -> AnyElement + 'static,
    ) -> Self {
        self.option_renderer = Some(Box::new(renderer));
        self
    }

    pub fn card_options(mut self) -> Self {
        self.option_style = Some(
            RadioOptionStyle::new()
                .radius(px(10.0))
                .padding(px(12.0), px(8.0)),
        );
        self
    }

    pub fn is_stretched(&self) -> bool {
        self.stretch
    }

    pub fn layout_kind(&self) -> RadioGroupLayout {
        self.layout
    }

    pub fn size_kind(&self) -> RadioGroupSize {
        self.size
    }

    pub fn on_change(mut self, cb: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(cb));
        self
    }

    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([
            KeyBinding::new("up", RadioGroupUp, None),
            KeyBinding::new("down", RadioGroupDown, None),
            KeyBinding::new("left", RadioGroupUp, None),
            KeyBinding::new("right", RadioGroupDown, None),
        ]);
    }

    fn up(&mut self, _: &RadioGroupUp, window: &mut Window, cx: &mut Context<Self>) {
        if !self.disabled && self.selected > 0 {
            self.select(self.selected - 1, window, cx);
        }
    }

    fn down(&mut self, _: &RadioGroupDown, window: &mut Window, cx: &mut Context<Self>) {
        if !self.disabled && self.selected + 1 < self.options.len() {
            self.select(self.selected + 1, window, cx);
        }
    }

    fn select(&mut self, idx: usize, window: &mut Window, cx: &mut Context<Self>) {
        if idx != self.selected {
            self.selected = idx;
            cx.notify();
            if let Some(ref cb) = self.on_change {
                cb(idx, window, cx);
            }
        }
    }

    fn render_radio_indicator(
        &self,
        checked: bool,
        border: Hsla,
        dot_color: Hsla,
        show_selected_icon: bool,
    ) -> impl IntoElement {
        let mut circle = gpui::div()
            .flex_none()
            .w(px(16.0))
            .h(px(16.0))
            .rounded(px(8.0))
            .border_1()
            .border_color(border)
            .flex()
            .items_center()
            .justify_center();

        if checked && show_selected_icon {
            circle = circle.child(
                gpui::div()
                    .w(px(8.0))
                    .h(px(8.0))
                    .rounded(px(4.0))
                    .bg(dot_color),
            );
        }

        circle
    }

    fn render_option_content(
        &self,
        idx: usize,
        label: SharedString,
        checked: bool,
        focused: bool,
    ) -> AnyElement {
        if let Some(renderer) = &self.option_renderer {
            renderer(RadioOptionRenderContext {
                index: idx,
                label,
                selected: checked,
                disabled: self.disabled,
                focused,
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
        focused: bool,
        style: RadioOptionStyle,
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
        let border = if checked || (focused && checked) {
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
                          window: &mut Window,
                          cx: &mut Context<Self>| {
                        this.select(idx, window, cx);
                    },
                ),
            );
        } else {
            item = item.cursor_not_allowed();
        }

        if show_indicator {
            item = item.child(self.render_radio_indicator(
                checked,
                border,
                theme.primary.base,
                show_selected_icon,
            ));
        }

        item.child(self.render_option_content(idx, label, checked, focused))
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
            .when(!self.stretch, |s| s.self_start())
            .on_action(cx.listener(Self::up))
            .on_action(cx.listener(Self::down));

        if !self.disabled {
            group = group.track_focus(&self.focus_handle);
        }

        for (idx, label) in self.options.iter().enumerate() {
            let checked = idx == self.selected;
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

            if checked && style.show_selected_icon.unwrap_or(false) {
                item = item.child(
                    gpui::div()
                        .w(px(7.0))
                        .h(px(7.0))
                        .rounded(px(4.0))
                        .bg(text_color),
                );
            }
            item = item.child(self.render_option_content(idx, label, checked, false));

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
                              window: &mut Window,
                              cx: &mut Context<Self>| {
                            this.select(idx, window, cx);
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

impl Focusable for RadioGroup {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for RadioGroup {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.layout == RadioGroupLayout::Button {
            return self.render_button_group(cx).into_any_element();
        }

        let theme = cx.global::<Config>().theme.clone();
        let focused = self.focus_handle.is_focused(_window);
        let option_style = self.option_style.clone();
        let sz = 16.0;
        let inner_sz = 8.0;

        let mut col = gpui::div()
            .flex()
            .when(self.layout == RadioGroupLayout::Vertical, |s| {
                s.flex_col().gap_2()
            })
            .when(self.layout == RadioGroupLayout::Horizontal, |s| {
                s.flex_row().gap_4().items_center()
            })
            .on_action(cx.listener(Self::up))
            .on_action(cx.listener(Self::down));

        if !self.disabled {
            col = col.track_focus(&self.focus_handle);
            col = col.on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| {
                    window.focus(&this.focus_handle, cx);
                }),
            );
        }

        for (idx, label) in self.options.iter().enumerate() {
            let checked = idx == self.selected;
            if let Some(style) = option_style.clone() {
                col = col.child(self.render_styled_option(
                    idx,
                    label.clone(),
                    checked,
                    focused,
                    style,
                    cx,
                ));
                continue;
            }

            if self.option_renderer.is_some() {
                col = col.child(self.render_styled_option(
                    idx,
                    label.clone(),
                    checked,
                    focused,
                    RadioOptionStyle::default(),
                    cx,
                ));
                continue;
            }

            let (border_color, dot_color) = if self.disabled {
                (theme.neutral.border, theme.neutral.text_disabled)
            } else if checked {
                (theme.primary.base, theme.primary.base)
            } else {
                (
                    if focused && checked {
                        theme.primary.base
                    } else {
                        theme.neutral.border
                    },
                    rgba(0, 0, 0, 0.0),
                )
            };

            let label_text = label.clone();
            let mut row = gpui::div().flex().flex_row().items_center().gap_2();

            if !self.disabled {
                row = row.cursor_pointer();
            } else {
                row = row.cursor_not_allowed();
            }

            let circle = gpui::div()
                .flex_none()
                .w(px(sz))
                .h(px(sz))
                .rounded(px(sz / 2.0))
                .border_1()
                .border_color(border_color)
                .flex()
                .items_center()
                .justify_center()
                .child(
                    gpui::div()
                        .w(px(inner_sz))
                        .h(px(inner_sz))
                        .rounded(px(inner_sz / 2.0))
                        .bg(dot_color),
                );

            row = row.child(circle);
            row = row.child(
                gpui::div()
                    .text_size(px(theme.font_size.md))
                    .text_color(theme.neutral.text_1)
                    .child(label_text),
            );

            if !self.disabled {
                row = row.on_mouse_up(
                    MouseButton::Left,
                    cx.listener(
                        move |this: &mut Self,
                              _: &MouseUpEvent,
                              window: &mut Window,
                              cx: &mut Context<Self>| {
                            this.select(idx, window, cx);
                        },
                    ),
                );
            }

            col = col.child(row);
        }

        col.into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn radio_option_style_supports_layout_and_selected_style() {
        let style = RadioOptionStyle::new()
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
    fn radio_group_accepts_custom_option_renderer() {
        let source = include_str!("radio_group.rs");
        assert!(source.contains("pub struct RadioOptionRenderContext"));
        assert!(source.contains("pub fn option_renderer"));
    }
}
