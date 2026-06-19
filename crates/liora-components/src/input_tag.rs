//! Input Tag module.
//!
//! This public module implements the Liora tag entry component for tokenized text values. It keeps the reusable
//! component logic inside `liora-components` rather than Gallery or Docs so
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
//! the component, and avoid app-specific Gallery/Docs resources in this SDK
//! crate.

use crate::{Input, Tag, TagFlow};
use gpui::{App, Context, Entity, Render, SharedString, Window, div, prelude::*, px};
use liora_core::Config;
use std::collections::HashSet;

pub struct InputTag {
    tags: Vec<SharedString>,
    input: Entity<Input>,
    placeholder: SharedString,
    max_tags: Option<usize>,
    allow_duplicates: bool,
    disabled: bool,
    on_change: Option<Box<dyn Fn(Vec<SharedString>, &mut Window, &mut App) + 'static>>,
}

impl InputTag {
    pub fn new(tags: Vec<impl Into<SharedString>>, cx: &mut Context<Self>) -> Self {
        Self {
            tags: tags.into_iter().map(Into::into).collect(),
            input: cx.new(|cx| Input::new("", cx).width_sm()),
            placeholder: "Add tag".into(),
            max_tags: None,
            allow_duplicates: false,
            disabled: false,
            on_change: None,
        }
    }

    pub fn entity(tags: Vec<impl Into<SharedString>>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(tags, cx))
    }
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }
    pub fn max_tags(mut self, max: usize) -> Self {
        self.max_tags = Some(max);
        self
    }
    pub fn allow_duplicates(mut self, allow: bool) -> Self {
        self.allow_duplicates = allow;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn on_change(
        mut self,
        cb: impl Fn(Vec<SharedString>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(cb));
        self
    }
    pub fn tags(&self) -> &[SharedString] {
        &self.tags
    }

    pub fn add_tag(&mut self, tag: impl Into<SharedString>, cx: &mut Context<Self>) -> bool {
        let tag = tag.into();
        let trimmed = tag.trim();
        if trimmed.is_empty() || self.disabled {
            return false;
        }
        if let Some(max) = self.max_tags
            && self.tags.len() >= max
        {
            return false;
        }
        if !self.allow_duplicates
            && self
                .tags
                .iter()
                .any(|existing| existing.as_ref() == trimmed)
        {
            return false;
        }
        self.tags.push(SharedString::from(trimmed.to_string()));
        cx.notify();
        true
    }

    pub fn remove_tag(&mut self, index: usize, cx: &mut Context<Self>) -> Option<SharedString> {
        if self.disabled || index >= self.tags.len() {
            return None;
        }
        let removed = self.tags.remove(index);
        cx.notify();
        Some(removed)
    }

    fn emit_change(&self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ref cb) = self.on_change {
            cb(self.tags.clone(), window, cx);
        }
    }
}

impl Render for InputTag {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let input = self.input.clone();
        let placeholder = self.placeholder.clone();
        let max_reached = self.max_tags.is_some_and(|max| self.tags.len() >= max);
        let view_for_enter = cx.entity().clone();
        cx.update_entity(&input, |input, cx| {
            input.set_placeholder(placeholder, cx);
            input.set_width(px(116.0), cx);
            input.set_disabled(max_reached || self.disabled, cx);
            input.set_on_enter(
                {
                    let input_tag = view_for_enter.clone();
                    move |input, value, window, cx| {
                        let submitted = value.trim().to_string();
                        if submitted.is_empty() {
                            return;
                        }
                        input.set_value("", cx);
                        input_tag.update(cx, |view: &mut InputTag, cx| {
                            if view.add_tag(submitted, cx) {
                                view.emit_change(window, cx);
                            }
                        });
                    }
                },
                cx,
            );
        });

        let tags = self
            .tags
            .iter()
            .enumerate()
            .map(|(idx, label)| {
                let view = cx.entity().clone();
                Tag::new(label.clone())
                    .round(true)
                    .closable(!self.disabled)
                    .on_close(move |window, cx| {
                        view.update(cx, |view, cx| {
                            if view.remove_tag(idx, cx).is_some() {
                                view.emit_change(window, cx);
                            }
                        });
                    })
            })
            .collect::<Vec<_>>();

        div()
            .id(liora_core::unique_id("input-tag"))
            .rounded_md()
            .border_1()
            .border_color(if self.disabled {
                theme.neutral.border.opacity(0.5)
            } else {
                theme.neutral.border
            })
            .bg(if self.disabled {
                theme.neutral.hover
            } else {
                theme.neutral.card
            })
            .p_2()
            .flex()
            .flex_col()
            .gap_2()
            .child(TagFlow::new(tags).gap(px(8.0)))
            .child(input)
            .when_some(self.max_tags, |s, max| {
                s.child(
                    div()
                        .text_xs()
                        .text_color(theme.neutral.text_3)
                        .child(format!("{}/{} tags", self.tags.len(), max)),
                )
            })
    }
}

pub fn normalize_tags(
    tags: impl IntoIterator<Item = impl Into<SharedString>>,
    allow_duplicates: bool,
    max_tags: Option<usize>,
) -> Vec<SharedString> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for tag in tags {
        let tag = tag.into();
        let trimmed = tag.trim();
        if trimmed.is_empty() {
            continue;
        }
        if !allow_duplicates && !seen.insert(trimmed.to_string()) {
            continue;
        }
        out.push(SharedString::from(trimmed.to_string()));
        if max_tags.is_some_and(|max| out.len() >= max) {
            break;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn normalize_tags_trims_deduplicates_and_caps() {
        let tags = normalize_tags([" rust ", "", "gpui", "rust", "liora"], false, Some(2));
        assert_eq!(
            tags.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            vec!["rust", "gpui"]
        );
    }
}
