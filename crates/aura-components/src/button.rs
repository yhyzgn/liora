use gpui::{prelude::*, px, SharedString, Hsla, Rgba, MouseButton, MouseDownEvent, MouseUpEvent};
use aura_theme::{ButtonVariant, ButtonSize, ButtonVariantColors, AuraTheme};
use aura_core::AuraContextExt;

fn rgba(r: u8, g: u8, b: u8, a: f32) -> Hsla {
    Rgba { r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a }.into()
}

/// Button configuration (stateless builder).
pub struct AuraButton {
    label: SharedString,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
    loading: bool,
    secondary: bool,
    background: bool,
    border: bool,
    rounded: Option<f32>,
    on_click: Option<Box<dyn Fn(&gpui::ClickEvent, &mut gpui::Window, &mut gpui::App) + 'static>>,
}

impl AuraButton {
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
            on_click: None,
        }
    }

    pub fn variant(mut self, v: ButtonVariant) -> Self { self.variant = v; self }
    pub fn primary(mut self) -> Self   { self.variant = ButtonVariant::Primary; self }
    pub fn tertiary(mut self) -> Self  { self.variant = ButtonVariant::Tertiary; self }
    pub fn info(mut self) -> Self      { self.variant = ButtonVariant::Info; self }
    pub fn success(mut self) -> Self   { self.variant = ButtonVariant::Success; self }
    pub fn warning(mut self) -> Self   { self.variant = ButtonVariant::Warning; self }
    pub fn danger(mut self) -> Self    { self.variant = ButtonVariant::Danger; self }
    pub fn size(mut self, s: ButtonSize) -> Self { self.size = s; self }
    pub fn small(mut self) -> Self  { self.size = ButtonSize::Small; self }
    pub fn large(mut self) -> Self  { self.size = ButtonSize::Large; self }
    pub fn disabled(mut self, d: bool) -> Self { self.disabled = d; self }
    pub fn loading(mut self, l: bool) -> Self   { self.loading = l; self }
    pub fn secondary(mut self) -> Self { self.secondary = true; self }
    pub fn background(mut self, show: bool) -> Self { self.background = show; self }
    pub fn border(mut self, show: bool) -> Self { self.border = show; self }
    pub fn rounded(mut self, r: f32) -> Self { self.rounded = Some(r); self }

    pub fn on_click(
        mut self,
        handler: impl Fn(&gpui::ClickEvent, &mut gpui::Window, &mut gpui::App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    /// Returns the button config — used by AuraButtonView to read styling.
    fn colors(&self, theme: &AuraTheme) -> ButtonVariantColors {
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

    /// Stateless element builder — no press effect, for simple display.
    pub fn build(self, theme: &AuraTheme) -> impl IntoElement {
        let colors = self.colors(theme);
        let height = self.size.height();
        let padding_x = self.size.padding_x();
        let font_size = match self.size {
            ButtonSize::Small => theme.font_size.xs,
            ButtonSize::Default => theme.font_size.md,
            ButtonSize::Large => theme.font_size.lg,
        };
        let radius = self.rounded.unwrap_or(theme.radius.md);

        let label_text = if self.loading {
            SharedString::from(format!("⟳ {}", self.label))
        } else {
            self.label.clone()
        };

        let mut el = gpui::div()
            .flex()
            .flex_row()
            .justify_center()
            .items_center()
            .gap_1()
            .h(px(height))
            .px(px(padding_x))
            .rounded(px(radius))
            .bg(colors.bg)
            .text_color(colors.text)
            .text_size(px(font_size));

        if !self.disabled {
            el = el.cursor_pointer();
        } else {
            el = el.cursor_not_allowed();
        }

        if !colors.border.is_transparent() {
            el = el.border_1().border_color(colors.border);
        }

        if !self.disabled {
            el = el.hover(|style| {
                let mut s = style.bg(colors.hover_bg);
                if !colors.border_hover.is_transparent() {
                    s = s.border_color(colors.border_hover);
                }
                s
            });
        }

        el.child(label_text).into_any_element()
    }
}

// ── View-based button with full press/hover state ─────────────────────────

pub struct AuraButtonView {
    config: AuraButton,
    is_pressed: bool,
}

impl AuraButtonView {
    pub fn new(config: AuraButton) -> Self {
        Self { config, is_pressed: false }
    }

    fn on_mouse_down(
        &mut self,
        _: &MouseDownEvent,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) {
        self.is_pressed = true;
        cx.notify();
    }

    fn on_mouse_up(
        &mut self,
        _: &MouseUpEvent,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) {
        self.is_pressed = false;
        cx.notify();
    }

    fn on_click_handler(
        &mut self,
        event: &gpui::ClickEvent,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) {
        self.is_pressed = false;
        cx.notify();
        if let Some(ref handler) = self.config.on_click {
            handler(event, window, cx);
        }
    }
}

impl gpui::Render for AuraButtonView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.aura();
        let config = &self.config;
        let colors = config.colors(&theme);

        let height = config.size.height();
        let padding_x = config.size.padding_x();
        let font_size = match config.size {
            ButtonSize::Small => theme.font_size.xs,
            ButtonSize::Default => theme.font_size.md,
            ButtonSize::Large => theme.font_size.lg,
        };
        let radius = config.rounded.unwrap_or(theme.radius.md);

        let label_text = if config.loading {
            SharedString::from(format!("⟳ {}", config.label))
        } else {
            config.label.clone()
        };

        // Active bg when pressed, normal otherwise
        let current_bg = if self.is_pressed { colors.active_bg } else { colors.bg };

        let btn_id = SharedString::from(format!("btn-{}", 
            std::sync::atomic::AtomicU64::new(0).fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        // Use .id() to unlock StatefulInteractiveElement for .on_click()
        let mut el = gpui::div()
            .flex()
            .flex_row()
            .justify_center()
            .items_center()
            .gap_1()
            .h(px(height))
            .px(px(padding_x))
            .rounded(px(radius))
            .bg(current_bg)
            .text_color(colors.text)
            .text_size(px(font_size))
            .id(btn_id);

        if !config.disabled {
            el = el.cursor_pointer();
        } else {
            el = el.cursor_not_allowed();
        }

        if !colors.border.is_transparent() {
            el = el.border_1().border_color(colors.border);
        }

        if !config.disabled {
            el = el
                .hover(|style| {
                    let mut s = style.bg(colors.hover_bg);
                    if !colors.border_hover.is_transparent() {
                        s = s.border_color(colors.border_hover);
                    }
                    s
                })
                .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
                .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
                .on_click(cx.listener(Self::on_click_handler));
        }

        el.child(label_text)
    }
}
