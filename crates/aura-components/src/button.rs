use aura_core::{Config, stable_unique_id};
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use aura_theme::{ButtonSize, ButtonVariant, ButtonVariantColors, Theme};
use gpui::{
    AbsoluteLength, AnyElement, App, Component, ElementId, Hsla, IntoElement, RenderOnce, Rgba,
    SharedString, Window, prelude::*, px,
};

fn rgba(r: u8, g: u8, b: u8, a: f32) -> Hsla {
    Rgba {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a,
    }
    .into()
}

pub enum ButtonIcon {
    IconName(IconName),
    Icon(Icon),
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
    on_click: Option<Box<dyn Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl Button {
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
            on_click: None,
        }
    }
    pub fn variant(mut self, v: ButtonVariant) -> Self {
        self.variant = v;
        self
    }
    pub fn primary(mut self) -> Self {
        self.variant = ButtonVariant::Primary;
        self
    }
    pub fn tertiary(mut self) -> Self {
        self.variant = ButtonVariant::Tertiary;
        self
    }
    pub fn text(mut self) -> Self {
        self.variant = ButtonVariant::Text;
        self
    }
    pub fn info(mut self) -> Self {
        self.variant = ButtonVariant::Info;
        self
    }
    pub fn success(mut self) -> Self {
        self.variant = ButtonVariant::Success;
        self
    }
    pub fn warning(mut self) -> Self {
        self.variant = ButtonVariant::Warning;
        self
    }
    pub fn danger(mut self) -> Self {
        self.variant = ButtonVariant::Danger;
        self
    }
    pub fn size(mut self, s: ButtonSize) -> Self {
        self.size = s;
        self
    }
    pub fn small(mut self) -> Self {
        self.size = ButtonSize::Small;
        self
    }
    pub fn large(mut self) -> Self {
        self.size = ButtonSize::Large;
        self
    }
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }
    pub fn loading(mut self, l: bool) -> Self {
        self.loading = l;
        self
    }
    pub fn secondary(mut self) -> Self {
        self.secondary = true;
        self
    }
    pub fn background(mut self, show: bool) -> Self {
        self.background = show;
        self
    }
    pub fn border(mut self, show: bool) -> Self {
        self.border = show;
        self
    }
    pub fn rounded(mut self, r: impl Into<AbsoluteLength>) -> Self {
        self.rounded = Some(r.into());
        self
    }
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn icon_start(mut self, icon: impl Into<ButtonIcon>) -> Self {
        self.icon_start = Some(icon.into());
        self
    }
    pub fn icon_end(mut self, icon: impl Into<ButtonIcon>) -> Self {
        self.icon_end = Some(icon.into());
        self
    }
    pub fn icon_top(mut self, icon: IconName) -> Self {
        self.icon_top = Some(icon);
        self
    }
    pub fn icon_bottom(mut self, icon: IconName) -> Self {
        self.icon_bottom = Some(icon);
        self
    }
    pub fn icon_only(mut self, icon: IconName) -> Self {
        self.icon_only = Some(icon);
        self
    }
    pub fn on_click(
        mut self,
        cb: impl Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(cb));
        self
    }

    fn colors(&self, theme: &Theme) -> ButtonVariantColors {
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
        let c = self.colors(&theme);
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
                    "aura-button:{}:{:?}:{:?}:secondary={}:background={}:border={}:rounded={:?}",
                    self.label,
                    self.variant,
                    self.size,
                    self.secondary,
                    self.background,
                    self.border,
                    self.rounded
                ),
                "aura-button",
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

        let mut div = gpui::div()
            .flex()
            .justify_center()
            .items_center()
            .gap_1()
            .h(px(if vertical { h + icon_sz + 6.0 } else { h }))
            .rounded(r)
            .bg(c.bg)
            .text_color(c.text)
            .text_size(px(fs));

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

        if icon_only {
            let icon = self.icon_only.unwrap();
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
                Icon::new(IconName::LoaderCircle)
                    .size(px(sz))
                    .color(c.text)
                    .group_hover_color(group, c.text_hover)
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

        div.id(id)
            .group(hover_group)
            .hover(move |style| {
                let mut s = style.bg(c.hover_bg).text_color(c.text_hover);
                if !c.border_hover.is_transparent() {
                    s = s.border_color(c.border_hover);
                }
                s
            })
            .active(move |style| style.bg(c.active_bg))
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
