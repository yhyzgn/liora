//! Link module.
//!
//! This public module implements the Liora interactive text link component with optional icon placement. It keeps the reusable
//! component logic inside `liora-components` rather than host applications so
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
//! the component, and avoid app-specific host-application resources in this SDK
//! crate.

use gpui::{
    App, Hsla, IntoElement, MouseButton, RenderOnce, SharedString, ViewElement as Component,
    Window, prelude::*, px,
};
use liora_core::{Config, stable_unique_id};
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use liora_theme::{ButtonVariant, Theme};

/// Fluent native GPUI component for rendering Liora link.
pub struct Link {
    label: SharedString,
    href: Option<SharedString>,
    variant: ButtonVariant,
    disabled: bool,
    underline: bool,
    icon_start: Option<IconName>,
    icon_end: Option<IconName>,
    id: Option<SharedString>,
}

impl Link {
    /// Creates `Link` initialized from the supplied label.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            href: None,
            variant: ButtonVariant::Default,
            disabled: false,
            underline: true,
            icon_start: None,
            icon_end: None,
            id: None,
        }
    }
    /// Sets the destination URL opened by the platform's default handler.
    pub fn href(mut self, url: impl Into<SharedString>) -> Self {
        self.href = Some(url.into());
        self
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
    /// Applies the informational semantic visual variant.
    pub fn info(mut self) -> Self {
        self.variant = ButtonVariant::Info;
        self
    }
    /// Toggles the disabled state and suppresses user interaction when enabled.
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }
    /// Sets the underline value used by the component.
    pub fn underline(mut self, u: bool) -> Self {
        self.underline = u;
        self
    }
    /// Sets the leading icon rendered before the button label.
    pub fn icon_start(mut self, icon: IconName) -> Self {
        self.icon_start = Some(icon);
        self
    }
    /// Sets the trailing icon rendered after the button label.
    pub fn icon_end(mut self, icon: IconName) -> Self {
        self.icon_end = Some(icon);
        self
    }

    /// Assigns a stable element id used by GPUI state, hit testing, and automated interaction tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = Some(id.into());
        self
    }

    fn color_for(&self, theme: &Theme) -> (Hsla, Hsla) {
        if self.disabled {
            return (theme.neutral.text_disabled, theme.neutral.text_disabled);
        }
        let family = match self.variant {
            ButtonVariant::Default | ButtonVariant::Tertiary | ButtonVariant::Text => {
                &theme.primary
            }
            ButtonVariant::Primary => &theme.primary,
            ButtonVariant::Success => &theme.success,
            ButtonVariant::Warning => &theme.warning,
            ButtonVariant::Danger => &theme.danger,
            ButtonVariant::Info => &theme.info,
        };
        (family.base, family.hover)
    }

    fn render_with_theme(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        let (color, hover_color) = self.color_for(theme);
        let fs = theme.font_size.md;
        let focus_color = theme.primary.base;
        let icon_sz = 14.0;
        let id = self.id.unwrap_or_else(|| {
            stable_unique_id(
                format!(
                    "link:{}:{:?}:disabled={}:underline={}",
                    self.label, self.variant, self.disabled, self.underline
                ),
                "link",
                window,
                cx,
            )
        });
        if self.disabled || self.href.is_none() {
            // Discard native presses before an inert frame can retain them across re-enabling.
            window.with_global_id(id.clone().into(), |global_id, window| {
                window.with_element_state::<gpui::InteractiveElementState, _>(global_id, |_, _| {
                    ((), Default::default())
                });
            });
        }

        let mut div = gpui::div()
            .flex()
            .flex_row()
            .items_center()
            .gap_1()
            .text_size(px(fs))
            .text_color(color)
            .id(id.clone());

        if self.disabled {
            div = div.cursor_not_allowed();
        } else if self.href.is_some() {
            div = div.cursor_pointer();
        }
        if let Some(url) = self.href.as_ref() {
            let url = url.clone();
            let disabled = self.disabled;
            div = div
                .role(gpui::Role::Link)
                .aria_label(self.label.clone())
                .a11y_synthetic_children(move |builder| {
                    let node = builder.parent_node();
                    node.set_url(url.as_ref());
                    if disabled {
                        node.set_disabled();
                    }
                });
        }
        if self.underline {
            div = div.underline();
        }

        if let Some(icon) = self.icon_start {
            div = div.child(Icon::new(icon).size(px(icon_sz)).color(color));
        }
        div = div.child(gpui::div().child(self.label));
        if let Some(icon) = self.icon_end {
            div = div.child(Icon::new(icon).size(px(icon_sz)).color(color));
        }

        if !self.disabled {
            if let Some(url) = self.href {
                let focus = window.with_global_id(id.into(), |global_id, window| {
                    window.with_element_state::<gpui::FocusHandle, _>(global_id, |previous, _| {
                        let handle = previous
                            .unwrap_or_else(|| cx.focus_handle().tab_stop(true).tab_index(0));
                        (handle.clone(), handle)
                    })
                });
                let paint_focus = focus.clone();
                let outline = gpui::canvas(
                    |_, _, _| (),
                    move |bounds, (), window, _| {
                        if paint_focus.is_focused(window) && window.last_input_was_keyboard() {
                            window.paint_quad(
                                gpui::outline(
                                    bounds.dilate(px(2.0)),
                                    focus_color,
                                    gpui::BorderStyle::default(),
                                )
                                .border_widths(px(2.0))
                                .corner_radii(px(2.0)),
                            );
                        }
                    },
                )
                .absolute()
                .inset(px(0.0));
                div = div
                    .track_focus(&focus)
                    .tab_index(0)
                    .capture_key_down(|event, window, _| {
                        // Keep Space from arming GPUI's shared Enter/Space click marker.
                        if event.keystroke.key == "space" && !event.keystroke.modifiers.modified() {
                            window.prevent_default();
                        }
                    })
                    .on_key_down(|event, window, cx| {
                        let modifiers = event.keystroke.modifiers;
                        if event.keystroke.key == "tab"
                            && !modifiers.control
                            && !modifiers.alt
                            && !modifiers.platform
                            && !modifiers.function
                        {
                            if modifiers.shift {
                                window.focus_prev(cx);
                            } else {
                                window.focus_next(cx);
                            }
                            cx.stop_propagation();
                        } else if event.keystroke.key == "enter" && !modifiers.modified() {
                            // Preserve GPUI's release-time click while containing parent shortcuts.
                            cx.stop_propagation();
                        }
                    })
                    .on_click(move |event, _, cx| {
                        let activate = match event {
                            gpui::ClickEvent::Mouse(click) => click.up.button == MouseButton::Left,
                            gpui::ClickEvent::Keyboard(click) => {
                                let enter = click.button == gpui::KeyboardButton::Enter;
                                if enter {
                                    cx.stop_propagation();
                                }
                                enter
                            }
                            gpui::ClickEvent::Touch(_) => true,
                        };
                        if activate {
                            cx.open_url(&url);
                        }
                    })
                    .child(outline);
            }
            div = div.hover(move |style| style.text_color(hover_color));
        }

        div
    }
}

impl RenderOnce for Link {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.render_with_theme(_window, cx)
    }
}

impl IntoElement for Link {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::Cell, rc::Rc};

    struct LinkUrlTest {
        href: Option<&'static str>,
        disabled: bool,
        bounds: Rc<Cell<Option<gpui::Bounds<gpui::Pixels>>>>,
        parent_enters: Rc<Cell<usize>>,
    }

    impl gpui::Render for LinkUrlTest {
        fn render(&mut self, _: &mut Window, _: &mut gpui::Context<Self>) -> impl IntoElement {
            let measured = self.bounds.clone();
            let parent_enters = self.parent_enters.clone();
            let mut link = Link::new("Native URL")
                .id("link-url")
                .disabled(self.disabled);
            if let Some(href) = self.href {
                link = link.href(href);
            }
            gpui::div()
                .w(px(240.0))
                .h(px(40.0))
                .on_key_up(move |event, _, _| {
                    if event.keystroke.key == "enter" {
                        parent_enters.set(parent_enters.get() + 1);
                    }
                })
                .child(link)
                .child(
                    gpui::canvas(
                        move |bounds, _, _| measured.set(Some(bounds)),
                        |_, (), _, _| {},
                    )
                    .absolute()
                    .inset(px(0.0)),
                )
        }
    }

    fn click_link(
        href: Option<&'static str>,
        disabled: bool,
        input: impl FnOnce(&mut gpui::VisualTestContext, gpui::Point<gpui::Pixels>),
    ) -> Option<String> {
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let bounds = Rc::new(Cell::new(None));
        let measured = bounds.clone();
        let handle = app.add_window(move |_, _| LinkUrlTest {
            href,
            disabled,
            bounds: measured,
            parent_enters: Rc::new(Cell::new(0)),
        });
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        window.update(|window, cx| window.draw(cx).clear(cx));
        let origin = bounds.get().expect("Link frame must be laid out").origin;
        input(&mut window, origin + gpui::point(px(4.0), px(4.0)));
        let opened = app.opened_url();
        app.quit();
        opened
    }

    fn release_key(window: &mut gpui::VisualTestContext, key: &str) {
        window.update(|window, cx| {
            let keystroke = gpui::Keystroke::parse_with_os(key, cx.platform_info().os).unwrap();
            window.dispatch_event(
                gpui::PlatformInput::KeyUp(gpui::KeyUpEvent { keystroke }),
                cx,
            );
        });
    }

    #[test]
    fn pointer_activation_dispatches_exact_url_through_native_platform() {
        for href in [
            "https://example.invalid/path?a=1&b=2#section",
            "https://example.invalid/%22quoted%22?q=hello%20world",
            "https://example.invalid/文档?语言=中文#标题",
        ] {
            assert_eq!(
                click_link(Some(href), false, |window, inside| {
                    window.simulate_click(inside, gpui::Modifiers::none());
                })
                .as_deref(),
                Some(href)
            );
        }
    }

    #[test]
    fn disabled_and_destinationless_links_do_not_open_urls() {
        for (href, disabled) in [(Some("https://example.invalid/"), true), (None, false)] {
            assert_eq!(
                click_link(href, disabled, |window, inside| {
                    window.simulate_click(inside, gpui::Modifiers::none());
                }),
                None
            );
        }
    }

    #[test]
    fn release_without_a_matching_press_does_not_open_url() {
        assert_eq!(
            click_link(Some("https://example.invalid/"), false, |window, inside| {
                window.update(|window, cx| {
                    window.dispatch_event(
                        gpui::PlatformInput::MouseUp(gpui::MouseUpEvent {
                            button: MouseButton::Left,
                            position: inside,
                            modifiers: gpui::Modifiers::none(),
                            click_count: 1,
                        }),
                        cx,
                    );
                });
            }),
            None
        );
    }

    #[test]
    fn keyboard_focus_and_enter_open_exact_url_after_redraw() {
        let href = "https://example.invalid/文档?q=hello%20world#标题";
        assert_eq!(
            click_link(Some(href), false, |window, _| {
                window.update(|window, cx| {
                    window.focus_next(cx);
                    window.draw(cx).clear(cx);
                });
                window.simulate_keystrokes("enter");
                assert_eq!(window.cx.opened_url(), None, "keydown must not open a URL");
                window.update(|window, cx| {
                    window.refresh();
                    window.draw(cx).clear(cx);
                });
                release_key(window, "enter");
            })
            .as_deref(),
            Some(href)
        );
    }

    #[test]
    fn outside_pointer_gestures_and_nonprimary_release_do_not_open_url() {
        for (press_outside, release_outside, release_button) in [
            (true, false, MouseButton::Left),
            (false, true, MouseButton::Left),
            (false, false, MouseButton::Right),
        ] {
            assert_eq!(
                click_link(Some("https://example.invalid/"), false, |window, inside| {
                    let outside = inside + gpui::point(px(300.0), px(80.0));
                    window.simulate_mouse_down(
                        if press_outside { outside } else { inside },
                        MouseButton::Left,
                        gpui::Modifiers::none(),
                    );
                    window.update(|window, cx| {
                        window.refresh();
                        window.draw(cx).clear(cx);
                    });
                    window.simulate_mouse_up(
                        if release_outside { outside } else { inside },
                        release_button,
                        gpui::Modifiers::none(),
                    );
                }),
                None
            );
        }
    }

    #[test]
    fn space_modified_enter_and_unpaired_release_do_not_open_url() {
        for (down, up) in [
            (Some("space"), "space"),
            (Some("space"), "enter"),
            (Some("ctrl-enter"), "ctrl-enter"),
            (Some("alt-enter"), "alt-enter"),
            (Some("shift-enter"), "shift-enter"),
            (None, "enter"),
        ] {
            assert_eq!(
                click_link(Some("https://example.invalid/"), false, |window, _| {
                    window.update(|window, cx| {
                        window.focus_next(cx);
                        window.draw(cx).clear(cx);
                    });
                    if let Some(down) = down {
                        window.simulate_keystrokes(down);
                    }
                    release_key(window, up);
                }),
                None,
                "unexpected activation from {down:?} -> {up}"
            );
        }
    }

    #[test]
    fn focus_departure_disabling_and_destination_removal_cancel_enter() {
        for change in ["blur", "disable", "remove destination"] {
            assert_eq!(
                click_link(Some("https://example.invalid/"), false, |window, _| {
                    window.update(|window, cx| {
                        window.focus_next(cx);
                        window.draw(cx).clear(cx);
                    });
                    window.simulate_keystrokes("enter");
                    window.update(|window, cx| {
                        if change == "blur" {
                            window.blur(cx);
                        } else {
                            window.root::<LinkUrlTest>().flatten().unwrap().update(
                                cx,
                                |view, cx| {
                                    if change == "disable" {
                                        view.disabled = true;
                                    } else {
                                        view.href = None;
                                    }
                                    cx.notify();
                                },
                            );
                        }
                        window.refresh();
                        window.draw(cx).clear(cx);
                    });
                    release_key(window, "enter");
                }),
                None,
                "unexpected activation after {change}"
            );
        }
    }

    #[test]
    fn enter_activation_does_not_reach_parent_key_up_handler() {
        let href = "https://example.invalid/only-link";
        assert_eq!(
            click_link(Some(href), false, |window, _| {
                window.update(|window, cx| {
                    window.focus_next(cx);
                    window.draw(cx).clear(cx);
                });
                window.simulate_keystrokes("enter");
                release_key(window, "enter");
                let parent_enters = window.update(|window, cx| {
                    window
                        .root::<LinkUrlTest>()
                        .flatten()
                        .unwrap()
                        .read(cx)
                        .parent_enters
                        .get()
                });
                assert_eq!(
                    parent_enters, 0,
                    "Enter release must not also trigger the parent"
                );
            })
            .as_deref(),
            Some(href)
        );
    }

    #[test]
    fn inactive_round_trip_cancels_pointer_press_and_allows_fresh_click() {
        for remove_destination in [false, true] {
            let href = "https://example.invalid/restored";
            assert_eq!(
                click_link(Some(href), false, |window, inside| {
                    window.simulate_mouse_down(inside, MouseButton::Left, gpui::Modifiers::none());
                    window.update(|window, cx| {
                        window
                            .root::<LinkUrlTest>()
                            .flatten()
                            .unwrap()
                            .update(cx, |view, cx| {
                                if remove_destination {
                                    view.href = None;
                                } else {
                                    view.disabled = true;
                                }
                                cx.notify();
                            });
                        window.refresh();
                        window.draw(cx).clear(cx);
                    });
                    window.simulate_mouse_up(inside, MouseButton::Left, gpui::Modifiers::none());
                    window.update(|window, cx| {
                        window
                            .root::<LinkUrlTest>()
                            .flatten()
                            .unwrap()
                            .update(cx, |view, cx| {
                                view.href = Some(href);
                                view.disabled = false;
                                cx.notify();
                            });
                        window.refresh();
                        window.draw(cx).clear(cx);
                    });
                    let outside = inside + gpui::point(px(300.0), px(80.0));
                    window.simulate_mouse_down(outside, MouseButton::Left, gpui::Modifiers::none());
                    window.simulate_mouse_up(inside, MouseButton::Left, gpui::Modifiers::none());
                    assert_eq!(
                        window.cx.opened_url(),
                        None,
                        "stale press survived an inactive frame"
                    );
                    window.simulate_click(inside, gpui::Modifiers::none());
                })
                .as_deref(),
                Some(href)
            );
        }
    }
}
