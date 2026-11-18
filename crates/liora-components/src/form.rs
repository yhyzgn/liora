//! Form module.
//!
//! This public module implements the Liora form and form-item layout primitives. It keeps the reusable
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
    AnyElement, App, Pixels, SharedString, ViewElement as Component, Window, div, prelude::*, px,
};
use liora_core::Config;

/// Fluent native GPUI component for rendering Liora form.
pub struct Form {
    label_width: Option<Pixels>,
    inline: bool,
    children: Vec<FormChild>,
}

enum FormChild {
    Item(FormItem),
    Element(AnyElement),
}

impl Form {
    /// Creates `Form` with default theme-driven styling and no optional callbacks attached.
    pub fn new() -> Self {
        Self {
            label_width: None,
            inline: false,
            children: Vec::new(),
        }
    }

    /// Sets the default label width for fields added with [`Self::item`].
    ///
    /// An explicit [`FormItem::label_width`] takes precedence. Children added
    /// with [`Self::child`] do not inherit this width.
    pub fn label_width(mut self, width: impl Into<Pixels>) -> Self {
        self.label_width = Some(width.into());
        self
    }
    /// Renders the code block with inline metrics instead of a block frame.
    pub fn inline(mut self, inline: bool) -> Self {
        self.inline = inline;
        self
    }
    /// Adds any child element without applying the form's default label width.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(FormChild::Element(child.into_any_element()));
        self
    }

    /// Adds a field that inherits the form's label width unless it sets its own.
    pub fn item(mut self, item: FormItem) -> Self {
        self.children.push(FormChild::Item(item));
        self
    }
}

impl IntoElement for Form {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl RenderOnce for Form {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .when(self.inline, |s| s.flex_row().gap_4().flex_wrap())
            .when(!self.inline, |s| s.flex_col().gap_4())
            .children(self.children.into_iter().map(|child| match child {
                FormChild::Item(mut item) => {
                    item.label_width = item.label_width.or(self.label_width);
                    item.into_any_element()
                }
                FormChild::Element(element) => element,
            }))
    }
}

/// Data model used by form item rendering.
pub struct FormItem {
    label: Option<SharedString>,
    label_width: Option<Pixels>,
    required: bool,
    error: Option<SharedString>,
    content: Option<AnyElement>,
}

impl FormItem {
    /// Creates `FormItem` with default theme-driven styling and no optional callbacks attached.
    pub fn new() -> Self {
        Self {
            label: None,
            label_width: None,
            required: false,
            error: None,
            content: None,
        }
    }

    /// Returns the stable user-facing label for this value.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
    /// Sets this item's label width, overriding a form default.
    pub fn label_width(mut self, width: impl Into<Pixels>) -> Self {
        self.label_width = Some(width.into());
        self
    }
    /// Sets the required value used by the component.
    pub fn required(mut self, r: bool) -> Self {
        self.required = r;
        self
    }
    /// Sets the error value used by the component.
    pub fn error(mut self, e: impl Into<SharedString>) -> Self {
        self.error = Some(e.into());
        self
    }

    /// Adds a child element to the component body.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.content = Some(child.into_any_element());
        self
    }
}

impl IntoElement for FormItem {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl RenderOnce for FormItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

        div()
            .flex()
            .flex_col()
            .gap_1()
            .child(div().flex().flex_row().items_center().gap_1().when_some(
                self.label,
                |this, label| {
                    this.child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_1()
                            .when_some(self.label_width, |s, w| s.w(w))
                            .child(
                                div()
                                    .text_size(px(theme.font_size.md))
                                    .text_color(theme.neutral.text_1)
                                    .child(label),
                            )
                            .when(self.required, |this| {
                                this.child(div().text_color(theme.danger.base).child("*"))
                            }),
                    )
                },
            ))
            .when_some(self.content, |this, content| this.child(content))
            .when_some(self.error, |this, error| {
                this.child(
                    div()
                        .text_size(px(theme.font_size.sm))
                        .text_color(theme.danger.base)
                        .child(error),
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Point, TestAppContext, canvas, size};
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn form_label_width_defaults_only_typed_items() {
        let mut app = TestAppContext::single();
        app.update(crate::init_liora);
        let window = app.add_empty_window();
        let widths = Rc::new(RefCell::new(Vec::new()));

        window.draw(Point::default(), size(px(320.0), px(96.0)), |_, _| {
            let field = || {
                let widths = widths.clone();
                FormItem::new().label("A").child(
                    canvas(
                        move |bounds, _, _| widths.borrow_mut().push(bounds.size.width),
                        |_, _, _, _| {},
                    )
                    .min_w(px(24.0))
                    .h(px(8.0)),
                )
            };

            Form::new()
                .inline(true)
                .item(field())
                .child(field())
                .item(field().label_width(px(48.0)))
                .label_width(px(120.0))
                .into_element()
        });

        assert_eq!(*widths.borrow(), [px(120.0), px(24.0), px(48.0)]);
        app.quit();
    }
}
