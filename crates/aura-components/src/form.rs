use aura_core::Config;
use gpui::{prelude::*, px, App, Render, Window, Context, SharedString, AnyElement, div, Entity, Pixels};

pub struct Form {
    _label_width: Option<Pixels>,
    inline: bool,
    items: Vec<Entity<FormItem>>,
}

impl Form {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            _label_width: None,
            inline: false,
            items: Vec::new(),
        }
    }

    pub fn label_width(mut self, width: impl Into<Pixels>) -> Self { self._label_width = Some(width.into()); self }
    pub fn inline(mut self, inline: bool) -> Self { self.inline = inline; self }
    pub fn add_item(&mut self, item: Entity<FormItem>, cx: &mut Context<Self>) {
        self.items.push(item);
        cx.notify();
    }
}

impl Render for Form {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .when(self.inline, |s| s.flex_row().gap_4().flex_wrap())
            .when(!self.inline, |s| s.flex_col().gap_4())
            .children(self.items.clone())
    }
}

pub struct FormItem {
    label: Option<SharedString>,
    label_width: Option<Pixels>,
    required: bool,
    error: Option<SharedString>,
    content: Option<AnyElement>,
}

impl FormItem {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            label: None,
            label_width: None,
            required: false,
            error: None,
            content: None,
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self { self.label = Some(label.into()); self }
    pub fn label_width(mut self, width: impl Into<Pixels>) -> Self { self.label_width = Some(width.into()); self }
    pub fn required(mut self, r: bool) -> Self { self.required = r; self }
    pub fn error(mut self, e: impl Into<SharedString>) -> Self { self.error = Some(e.into()); self }
    pub fn set_error(&mut self, e: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.error = Some(e.into());
        cx.notify();
    }
    
    pub fn clear_error(&mut self, cx: &mut Context<Self>) {
        self.error = None;
        cx.notify();
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.content = Some(child.into_any_element());
        self
    }
}

impl Render for FormItem {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        
        div()
            .flex().flex_col().gap_1()
            .child(
                div().flex().flex_row().items_center().gap_1()
                    .when_some(self.label.clone(), |this, label| {
                        this.child(
                            div()
                                .flex().flex_row().items_center().gap_1()
                                .when_some(self.label_width, |s, w| s.w(w))
                                .child(
                                    div()
                                        .text_size(px(theme.font_size.md))
                                        .text_color(theme.neutral.text_1)
                                        .child(label)
                                )
                                .when(self.required, |this| {
                                    this.child(div().text_color(theme.danger.base).child("*"))
                                })
                        )
                    })
            )
            .when_some(self.content.take(), |this, content| {
                this.child(content)
            })
            .when_some(self.error.clone(), |this, error| {
                this.child(
                    div()
                        .text_size(px(theme.font_size.sm))
                        .text_color(theme.danger.base)
                        .child(error)
                )
            })
    }
}
