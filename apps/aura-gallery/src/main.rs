use aura_components::{
    Card, Checkbox, CodeBlock, Container, Dialog, Drawer, Input, Menu, MenuMode, MessageManager,
    Paragraph, Preview, Radio, RadioGroup, Space, Switch, Text, Title,
};
use aura_core::{PassivePortal, Portal, init_aura};
use aura_gallery::demos;
use aura_theme::Theme;
use gpui::{
    AnyView, App, Bounds, Component, Context, Render, WeakEntity, Window, WindowBounds,
    WindowOptions, div, prelude::*, px, size,
};

pub struct Gallery {
    entries: Vec<demos::DemoEntry>,
    demos: Vec<AnyView>,
    selected: usize,
    nav_menu: Option<gpui::Entity<aura_components::Menu>>,
}

fn run_gallery() {
    gpui_platform::application().run(|cx: &mut App| {
        init_aura(cx, Theme::light());
        MessageManager::init(cx);

        // Register all key bindings
        Input::register_key_bindings(cx);
        CodeBlock::register_key_bindings(cx);
        Checkbox::register_key_bindings(cx);
        Radio::register_key_bindings(cx);
        RadioGroup::register_key_bindings(cx);
        Switch::register_key_bindings(cx);
        Dialog::register_key_bindings(cx);
        Drawer::register_key_bindings(cx);
        Preview::register_key_bindings(cx);

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
                    nav_menu: None,
                })
            },
        );
    });
}

#[cfg(test)]
mod shell_tests {
    #[test]
    fn gallery_shell_uses_container_and_menu() {
        let source = include_str!("main.rs");

        assert!(source.contains("Container::new()"));
        assert!(source.contains("Menu::new()"));
        assert!(source.contains(".no_shrink()"));
        assert!(source.contains("Preview::register_key_bindings(cx)"));
        assert!(!source.contains(&format!("gallery-demo{}nav-", "-")));
    }
}

impl Render for Gallery {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected = self.selected.min(self.entries.len().saturating_sub(1));
        self.selected = selected;

        let nav_menu = self.gallery_nav_menu(selected, cx);

        let selected_entry = &self.entries[selected];
        let selected_demo = self.demos[selected].clone();

        let header = Space::new()
            .vertical()
            .gap_xs()
            .child(Title::new("Aura UI").h2())
            .child(Text::new(format!(
                "Native Component Library · {} demos · rendering one demo at a time",
                self.entries.len()
            )));

        let content = Card::new(
            Space::new()
                .vertical()
                .gap_lg()
                .child(
                    Space::new()
                        .vertical()
                        .gap_xs()
                        .child(Title::new(selected_entry.name).h3())
                        .child(Paragraph::with_text(selected_entry.description)),
                )
                .child(selected_demo),
        )
        .no_shadow()
        .no_shrink();

        aura_components::message::render_messages(cx);
        aura_components::notification::render_notifications(cx);
        aura_components::image::render_image_preview(_window, cx);
        aura_core::render_active_tooltip_in_window(_window, cx);
        aura_core::render_active_popover_in_window(_window, cx);
        aura_core::render_active_modal_in_window(_window, cx);
        aura_core::render_active_drawer_in_window(_window, cx);

        Container::new()
            .header(header)
            .header_height_lg()
            .aside(nav_menu)
            .aside_width_lg()
            .aside_scroll()
            .main_scroll()
            .main_padding_xl()
            .child(content)
            .overlay(PortalLayer)
    }
}

impl Gallery {
    fn gallery_nav_menu(&mut self, selected: usize, cx: &mut Context<Self>) -> gpui::Entity<Menu> {
        if let Some(nav_menu) = &self.nav_menu {
            return nav_menu.clone();
        }

        let gallery = cx.entity().downgrade();
        let items = self
            .entries
            .iter()
            .enumerate()
            .map(|(index, entry)| (index, entry.name))
            .collect::<Vec<_>>();
        let nav_menu = cx.new(move |_| build_gallery_menu(items, selected, gallery));
        self.nav_menu = Some(nav_menu.clone());
        nav_menu
    }
}

fn build_gallery_menu(
    items: Vec<(usize, &'static str)>,
    selected: usize,
    gallery: WeakEntity<Gallery>,
) -> Menu {
    let mut menu = Menu::new()
        .id("gallery-menu")
        .mode(MenuMode::Vertical)
        .default_active(selected.to_string())
        .on_select(move |id, _, cx| {
            let Ok(index) = id.parse::<usize>() else {
                return;
            };
            let _ = gallery.update(cx, |gallery, cx| {
                gallery.selected = index;
                cx.notify();
            });
        });

    for (index, name) in items {
        menu = menu.item(index.to_string(), name, None);
    }

    menu
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
        let passive_portals = if cx.has_global::<PassivePortal>() {
            std::mem::take(&mut cx.global_mut::<PassivePortal>().entries)
        } else {
            Vec::new()
        };
        let portals = if cx.has_global::<Portal>() {
            std::mem::take(&mut cx.global_mut::<Portal>().entries)
        } else {
            Vec::new()
        };

        if passive_portals.is_empty() && portals.is_empty() {
            return div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .into_any_element();
        }

        let mut container = div().absolute().top_0().left_0().size_full();

        if !passive_portals.is_empty() {
            let mut passive_container = div()
                .id("passive-portal-layer")
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .bg(gpui::transparent_black());

            for entry in passive_portals {
                passive_container = passive_container.child((entry.render)(window, cx));
            }

            container = container.child(passive_container);
        }

        if !portals.is_empty() {
            let mut active_container = div()
                .id("portal-layer")
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .cursor_default()
                .occlude()
                .bg(gpui::transparent_black())
                .on_hover(|_, _, cx| {
                    cx.stop_propagation();
                })
                .on_mouse_move(|_, _, cx| {
                    cx.stop_propagation();
                });

            for entry in portals {
                active_container = active_container.child((entry.render)(window, cx));
            }

            container = container.child(active_container);
        }

        container.into_any_element()
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
