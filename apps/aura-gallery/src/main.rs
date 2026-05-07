mod category;
mod demos;

use aura_components::{Checkbox, Dialog, Drawer, Input, Radio, RadioGroup, Switch};
use aura_core::{ContextExt, Portal, init_aura};
use aura_theme::Theme;
use gpui::{
    AnyView, App, Bounds, Component, Context, MouseButton, Render, Window, WindowBounds,
    WindowOptions, div, prelude::*, px, size,
};

pub struct Gallery {
    entries: Vec<demos::DemoEntry>,
    demos: Vec<AnyView>,
    selected: usize,
}

fn run_gallery() {
    gpui_platform::application().run(|cx: &mut App| {
        init_aura(cx, Theme::light());

        // Register all key bindings
        Input::register_key_bindings(cx);
        Checkbox::register_key_bindings(cx);
        Radio::register_key_bindings(cx);
        RadioGroup::register_key_bindings(cx);
        Switch::register_key_bindings(cx);
        Dialog::register_key_bindings(cx);
        Drawer::register_key_bindings(cx);

        let _ = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Maximized(Bounds {
                    origin: gpui::Point::default(),
                    size: size(px(1920.0), px(1080.0)),
                })),
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("Aura UI Gallery".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| {
                let entries = demos::registry();
                let demos = entries.iter().map(|entry| (entry.render)(cx)).collect();
                cx.new(|_| Gallery {
                    entries,
                    demos,
                    selected: 0,
                })
            },
        );
    });
}

impl Render for Gallery {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.aura();
        let selected = self.selected.min(self.entries.len().saturating_sub(1));
        self.selected = selected;

        let header = div()
            .flex()
            .flex_col()
            .gap_1()
            .px_6()
            .py_5()
            .border_b_1()
            .border_color(theme.neutral.border)
            .child(
                div()
                    .text_2xl()
                    .text_color(theme.neutral.text_1)
                    .font_weight(gpui::FontWeight::BOLD)
                    .child("Aura UI"),
            )
            .child(
                div()
                    .text_size(px(theme.font_size.md))
                    .text_color(theme.neutral.text_3)
                    .child(format!(
                        "Native Component Library · {} demos · rendering one demo at a time",
                        self.entries.len()
                    )),
            );

        let nav = div()
            .flex()
            .flex_col()
            .w(px(280.0))
            .h_full()
            .flex_shrink_0()
            .id("gallery-nav")
            .overflow_y_scroll()
            .border_r_1()
            .border_color(theme.neutral.border)
            .bg(theme.neutral.card)
            .children(self.entries.iter().enumerate().map(|(i, entry)| {
                let active = i == selected;
                let bg = if active {
                    theme.primary.light_9
                } else {
                    theme.neutral.card
                };
                let title_color = if active {
                    theme.primary.base
                } else {
                    theme.neutral.text_1
                };
                let desc_color = if active {
                    theme.primary.base.opacity(0.75)
                } else {
                    theme.neutral.text_3
                };

                div()
                    .id(format!("gallery-demo-nav-{}", i))
                    .flex()
                    .flex_col()
                    .gap_1()
                    .px_4()
                    .py_3()
                    .border_b_1()
                    .border_color(theme.neutral.divider)
                    .bg(bg)
                    .cursor_pointer()
                    .hover(|style| style.bg(theme.neutral.hover))
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            this.selected = i;
                            cx.notify();
                        }),
                    )
                    .child(
                        div()
                            .text_size(px(theme.font_size.md))
                            .text_color(title_color)
                            .font_weight(gpui::FontWeight::BOLD)
                            .child(entry.name),
                    )
                    .child(
                        div()
                            .text_size(px(theme.font_size.sm))
                            .text_color(desc_color)
                            .child(entry.description),
                    )
            }));

        let selected_entry = &self.entries[selected];
        let selected_demo = self.demos[selected].clone();

        let content = div()
            .flex()
            .flex_col()
            .flex_1()
            .h_full()
            .id("gallery-content")
            .overflow_y_scroll()
            .p_8()
            .bg(theme.neutral.body)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .p_4()
                    .border_1()
                    .border_color(theme.neutral.divider)
                    .rounded(px(theme.radius.lg))
                    .bg(theme.neutral.card)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_size(px(theme.font_size.lg))
                                    .text_color(theme.neutral.text_1)
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .child(selected_entry.name),
                            )
                            .child(
                                div()
                                    .text_size(px(theme.font_size.sm))
                                    .text_color(theme.neutral.text_3)
                                    .child(selected_entry.description),
                            ),
                    )
                    .child(selected_demo),
            );

        let body = div()
            .flex()
            .flex_row()
            .flex_1()
            .min_h_0()
            .child(nav)
            .child(content);

        let container = div()
            .size_full()
            .relative()
            .flex()
            .flex_col()
            .bg(theme.neutral.body)
            .child(header)
            .child(body);

        aura_components::message::render_messages(cx);
        aura_components::notification::render_notifications(cx);
        aura_core::render_active_tooltip_in_window(_window, cx);
        aura_core::render_active_popover_in_window(_window, cx);
        aura_core::render_active_modal_in_window(_window, cx);
        aura_core::render_active_drawer_in_window(_window, cx);

        container.child(PortalLayer)
    }
}

struct PortalLayer;

impl IntoElement for PortalLayer {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl gpui::RenderOnce for PortalLayer {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let mut container = div().absolute().top_0().left_0().size_full();

        if cx.has_global::<Portal>() {
            let portals = std::mem::take(&mut cx.global_mut::<Portal>().entries);
            for entry in portals {
                container = container.child((entry.render)(window, cx));
            }
        }

        container
    }
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    run_gallery();
}
#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    gpui_platform::web_init();
    run_gallery();
}
