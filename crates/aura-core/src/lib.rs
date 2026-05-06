use gpui::{App, Bounds, Context, Global, Hsla, TextRun, prelude::*, px};

pub mod popper;

pub use popper::*;

pub use aura_theme::Theme;

pub struct Config {
    pub theme: Theme,
    pub z_index_base: u32,
}

impl Global for Config {}

pub fn init_aura(cx: &mut App, theme: Theme) {
    cx.set_global(Config {
        theme,
        z_index_base: 1000,
    });
    cx.set_global(crate::popper::ZIndexStack::default());
    cx.set_global(crate::popper::ActiveTooltip(None));
    cx.set_global(crate::popper::ActivePopover(None));
    cx.set_global(crate::popper::ActiveModal(None));
    cx.set_global(crate::popper::ActiveDrawer(None));
}

pub fn render_active_popover_in_window(_window: &mut gpui::Window, cx: &mut App) {
    if let Some(popover_view) = cx.global::<crate::popper::ActivePopover>().0.clone() {
        push_portal(
            move |_window, _cx| popover_view.clone().into_any_element(),
            cx,
        );
    }
}

pub fn render_active_modal_in_window(_window: &mut gpui::Window, cx: &mut App) {
    if let Some(modal_view) = cx.global::<crate::popper::ActiveModal>().0.clone() {
        push_portal(
            move |_window, _cx| modal_view.clone().into_any_element(),
            cx,
        );
    }
}

pub fn render_active_drawer_in_window(_window: &mut gpui::Window, cx: &mut App) {
    if let Some(drawer_view) = cx.global::<crate::popper::ActiveDrawer>().0.clone() {
        push_portal(
            move |_window, _cx| drawer_view.clone().into_any_element(),
            cx,
        );
    }
}

pub fn render_active_tooltip_in_window(window: &mut gpui::Window, cx: &mut App) {
    let active = cx.global::<crate::popper::ActiveTooltip>().0.clone();
    if let Some(data) = active {
        let mouse_pos = window.mouse_position();
        if !data.anchor_bounds.contains(&mouse_pos) {
            cx.set_global(crate::popper::ActiveTooltip(None));
            return;
        }

        let theme = cx.global::<Config>().theme.clone();

        // Measure text accurately
        let font_size = px(theme.font_size.sm);
        let text_style = window.text_style();
        let run = TextRun {
            len: data.content.len(),
            font: text_style.font(),
            color: theme.neutral.card,
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        let shaped_line =
            window
                .text_system()
                .shape_line(data.content.clone(), font_size, &[run], None);

        let padding_h = px(12.0);
        let padding_v = px(4.0);
        let line_height = window.line_height();
        let content_size = gpui::Size {
            width: shaped_line.width + padding_h * 2.0,
            height: line_height + padding_v * 2.0,
        };

        push_portal(
            move |window, _cx| {
                let viewport = Bounds {
                    origin: gpui::Point::default(),
                    size: window.viewport_size(),
                };

                let popper = Popper {
                    anchor_bounds: data.anchor_bounds,
                    placement: data.placement,
                    offset: data.offset,
                };

                let (pos, _final_placement) =
                    popper.calculate_position_with_flip(content_size, viewport);

                gpui::div()
                    .absolute()
                    .top(pos.y)
                    .left(pos.x)
                    .w(content_size.width)
                    .h(content_size.height)
                    .bg(theme.neutral.text_1)
                    .text_color(theme.neutral.card)
                    .px(padding_h)
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(px(theme.radius.sm))
                    .shadow_lg()
                    .text_size(font_size)
                    .child(data.content.clone())
                    .into_any_element()
            },
            cx,
        );
    }
}

pub fn aura_theme<'a, V>(cx: &'a Context<'a, V>) -> &'a Theme {
    &cx.global::<Config>().theme
}

pub trait ContextExt {
    fn aura(&self) -> &Theme;
}

impl<'a, V> ContextExt for Context<'a, V> {
    fn aura(&self) -> &Theme {
        aura_theme(self)
    }
}

pub trait ElementExt {
    fn aura(self, cx: &mut App) -> Self;
}

impl ElementExt for gpui::Div {
    fn aura(self, _cx: &mut App) -> Self {
        self
    }
}

pub fn z_index_popup<V>(cx: &Context<'_, V>) -> u32 {
    cx.global::<Config>().z_index_base + 100
}

pub fn z_index_modal<V>(cx: &Context<'_, V>) -> u32 {
    cx.global::<Config>().z_index_base + 200
}

pub fn z_index_notification<V>(cx: &Context<'_, V>) -> u32 {
    cx.global::<Config>().z_index_base + 300
}

pub fn z_index_tooltip<V>(cx: &Context<'_, V>) -> u32 {
    cx.global::<Config>().z_index_base + 400
}

pub fn hex_color(hex: u32) -> Hsla {
    gpui::rgb(hex).into()
}
