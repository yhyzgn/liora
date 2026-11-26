//! Card module.
//!
//! This public module implements the Liora card container for grouped content with optional header and footer regions. It keeps the reusable
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
    AnyElement, App, ElementId, IntoElement, Pixels, RenderOnce, SharedString,
    ViewElement as Component, Window, div, prelude::*, px,
};
use liora_core::Config;

/// Fluent native GPUI component for rendering Liora card.
pub struct Card {
    id: ElementId,
    title: Option<SharedString>,
    header: Option<AnyElement>,
    footer: Option<AnyElement>,
    body: AnyElement,
    hoverable: bool,
    shadow: bool,
    width: Option<Pixels>,
    full_width: bool,
    shrink: bool,
}

impl Card {
    /// Creates `Card` initialized from the supplied body.
    #[track_caller]
    pub fn new(body: impl IntoElement) -> Self {
        Self {
            id: std::panic::Location::caller().into(),
            title: None,
            header: None,
            footer: None,
            body: body.into_any_element(),
            hoverable: false,
            shadow: true,
            width: None,
            full_width: false,
            shrink: true,
        }
    }

    /// Sets a stable identity for cards built by the same factory or loop.
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    /// Sets the primary title text displayed by the component.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the header value used by the component.
    pub fn header(mut self, header: impl IntoElement) -> Self {
        self.header = Some(header.into_any_element());
        self
    }

    /// Sets the footer value used by the component.
    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }

    /// Enables hover styling for the component frame.
    pub fn hoverable(mut self) -> Self {
        self.hoverable = true;
        self
    }

    /// Disables the default card shadow.
    pub fn no_shadow(mut self) -> Self {
        self.shadow = false;
        self
    }

    /// Sets the component width token used during GPUI layout.
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self.full_width = false;
        self
    }

    /// Expands the card to fill the available parent width.
    pub fn full_width(mut self) -> Self {
        self.width = None;
        self.full_width = true;
        self
    }

    /// Applies the predefined width md sizing preset.
    pub fn width_md(self) -> Self {
        self.width(px(300.0))
    }

    /// Applies the predefined width lg sizing preset.
    pub fn width_lg(self) -> Self {
        self.width(px(400.0))
    }

    /// Prevents the component from shrinking in flex layouts.
    pub fn no_shrink(mut self) -> Self {
        self.shrink = false;
        self
    }
}

impl RenderOnce for Card {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        let (background, body_color, border, radius, accent, title_color, footer_background) = (
            theme.neutral.card,
            theme.neutral.text_2,
            theme.neutral.border,
            theme.radius.md,
            theme.primary.base,
            theme.neutral.text_1,
            theme.neutral.hover,
        );

        let mut el = div()
            .id(self.id)
            .bg(background)
            .text_color(body_color)
            .border_1()
            .border_color(border)
            .rounded(px(radius))
            .overflow_hidden()
            .when(!self.shrink, |s| s.flex_none())
            .when(self.full_width, |s| s.w_full().min_w(px(0.0)))
            .when_some(self.width, |s, width| s.w(width));

        if self.shadow {
            el = el.shadow_md();
        }

        if self.hoverable {
            el = el.hover(|s| s.shadow_xl().border_color(accent));
        }

        // Header
        if let Some(title) = self.title {
            el = el.child(
                div()
                    .when(self.full_width, |s| s.w_full())
                    .p_4()
                    .border_b_1()
                    .border_color(border)
                    .child(
                        div()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(title_color)
                            .child(title),
                    ),
            );
        } else if let Some(header) = self.header {
            el = el.child(
                div()
                    .when(self.full_width, |s| s.w_full())
                    .p_4()
                    .border_b_1()
                    .border_color(border)
                    .child(header),
            );
        }

        // Body
        el = el.child(
            div()
                .when(self.full_width, |s| {
                    s.flex().flex_col().w_full().min_w(px(0.0))
                })
                .p_4()
                .text_color(body_color)
                .child(self.body),
        );

        // Footer
        if let Some(footer) = self.footer {
            el = el.child(
                div()
                    .when(self.full_width, |s| s.w_full())
                    .p_4()
                    .border_t_1()
                    .border_color(border)
                    .bg(footer_background)
                    .text_color(body_color)
                    .child(footer),
            );
        }

        el
    }
}

impl IntoElement for Card {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_width_helpers_set_demo_widths() {
        assert_eq!(Card::new("body").width_md().width, Some(px(300.0)));
        assert_eq!(Card::new("body").width_lg().width, Some(px(400.0)));
    }

    struct CardSizingTest {
        with_sibling: bool,
        prevent_shrink: bool,
    }

    impl gpui::Render for CardSizingTest {
        fn render(&mut self, _: &mut Window, _: &mut gpui::Context<Self>) -> impl IntoElement {
            let body = div()
                .w_full()
                .h(px(40.0))
                .debug_selector(|| "card-sizing-body".into());
            let card = Card::new(body).full_width();
            let card = if self.prevent_shrink {
                card.no_shrink()
            } else {
                card
            };
            div()
                .flex()
                .w(px(400.0))
                .child(card)
                .when(self.with_sibling, |row| {
                    row.child(div().w(px(200.0)).h(px(40.0)).flex_none())
                })
        }
    }

    #[test]
    fn full_width_respects_no_shrink_in_flex_layouts() -> Result<(), String> {
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let handle = app.add_window(|_, _| CardSizingTest {
            with_sibling: false,
            prevent_shrink: false,
        });
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        window.update(|window, cx| window.draw(cx).clear(cx));
        let baseline = window
            .debug_bounds("card-sizing-body")
            .ok_or("missing baseline body")?
            .size
            .width;
        handle
            .update(&mut app, |view, _, cx| {
                view.with_sibling = true;
                cx.notify();
            })
            .map_err(|error| error.to_string())?;
        window.update(|window, cx| window.draw(cx).clear(cx));
        let shrunk = window
            .debug_bounds("card-sizing-body")
            .ok_or("missing shrinking body")?
            .size
            .width;
        assert_eq!(shrunk, baseline - px(200.0));
        handle
            .update(&mut app, |view, _, cx| {
                view.prevent_shrink = true;
                cx.notify();
            })
            .map_err(|error| error.to_string())?;
        window.update(|window, cx| window.draw(cx).clear(cx));
        let preserved = window
            .debug_bounds("card-sizing-body")
            .ok_or("missing nonshrinking body")?
            .size
            .width;
        assert_eq!(preserved, baseline);
        app.quit();
        Ok(())
    }

    struct SiblingCardsTest {
        activations: std::rc::Rc<std::cell::RefCell<Vec<usize>>>,
    }

    impl gpui::Render for SiblingCardsTest {
        fn render(&mut self, _: &mut Window, _: &mut gpui::Context<Self>) -> impl IntoElement {
            let activations = self.activations.clone();
            let first = div()
                .id("card-action")
                .debug_selector(|| "card-first-action".into())
                .w_full()
                .h(px(40.0))
                .on_click(move |_, _, _| activations.borrow_mut().push(0));
            let activations = self.activations.clone();
            let second = div()
                .id("card-action")
                .debug_selector(|| "card-second-action".into())
                .w_full()
                .h(px(40.0))
                .on_click(move |_, _, _| activations.borrow_mut().push(1));
            div()
                .flex()
                .w(px(800.0))
                .child(Card::new(first).width(px(400.0)))
                .child(Card::new(second).width(px(400.0)))
        }
    }

    fn click_across_redraw(window: &mut gpui::VisualTestContext, position: gpui::Point<Pixels>) {
        window.update(|window, cx| {
            window.dispatch_event(
                gpui::PlatformInput::MouseDown(gpui::MouseDownEvent {
                    position,
                    button: gpui::MouseButton::Left,
                    modifiers: gpui::Modifiers::none(),
                    click_count: 1,
                    first_mouse: false,
                }),
                cx,
            );
            window.draw(cx).clear(cx);
            window.dispatch_event(
                gpui::PlatformInput::MouseUp(gpui::MouseUpEvent {
                    position,
                    button: gpui::MouseButton::Left,
                    modifiers: gpui::Modifiers::none(),
                    click_count: 1,
                }),
                cx,
            );
        });
    }

    #[test]
    fn sibling_cards_keep_native_child_actions_independent() -> Result<(), String> {
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let activations = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let handle = app.add_window({
            let activations = activations.clone();
            move |_, _| SiblingCardsTest { activations }
        });
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        window.update(|window, cx| window.draw(cx).clear(cx));
        let first = window
            .debug_bounds("card-first-action")
            .ok_or("missing first Card action")?;
        let second = window
            .debug_bounds("card-second-action")
            .ok_or("missing second Card action")?;
        click_across_redraw(&mut window, first.center());
        assert_eq!(*activations.borrow(), [0]);
        window.update(|window, cx| window.draw(cx).clear(cx));
        click_across_redraw(&mut window, second.center());
        assert_eq!(*activations.borrow(), [0, 1]);
        app.quit();
        Ok(())
    }
}
