use gpui::{App, Context, Global, Hsla, prelude::*};

pub use aura_theme::AuraTheme;

pub struct AuraConfig {
    pub theme: AuraTheme,
    pub z_index_base: u32,
}

impl Global for AuraConfig {}

pub fn init_aura(cx: &mut App, theme: AuraTheme) {
    cx.set_global(AuraConfig {
        theme,
        z_index_base: 1000,
    });
}

pub fn aura_theme<'a, V>(cx: &'a Context<'a, V>) -> &'a AuraTheme {
    &cx.global::<AuraConfig>().theme
}

pub trait AuraContextExt {
    fn aura(&self) -> &AuraTheme;
}

impl<V> AuraContextExt for Context<'_, V> {
    fn aura(&self) -> &AuraTheme {
        &self.global::<AuraConfig>().theme
    }
}

pub trait AuraElement: IntoElement + Sized {
    fn size(self, _size: aura_theme::ButtonSize) -> Self {
        self
    }

    fn variant(self, _variant: aura_theme::ButtonVariant) -> Self {
        self
    }

    fn disabled(self, _disabled: bool) -> Self {
        self
    }

    fn loading(self, _loading: bool) -> Self {
        self
    }
}

impl<E: IntoElement> AuraElement for E {}

pub fn z_index_popup<V>(cx: &Context<'_, V>) -> u32 {
    cx.global::<AuraConfig>().z_index_base + 100
}

pub fn z_index_modal<V>(cx: &Context<'_, V>) -> u32 {
    cx.global::<AuraConfig>().z_index_base + 200
}

pub fn z_index_notification<V>(cx: &Context<'_, V>) -> u32 {
    cx.global::<AuraConfig>().z_index_base + 300
}

pub fn z_index_tooltip<V>(cx: &Context<'_, V>) -> u32 {
    cx.global::<AuraConfig>().z_index_base + 400
}

pub fn hex_color(hex: u32) -> Hsla {
    gpui::rgb(hex).into()
}
