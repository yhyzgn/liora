//! Upload module.
//!
//! This public module implements the Liora upload list and validation component. It keeps the reusable
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

use crate::gpui_compat::element_id;
use gpui::{
    Context, IntoElement, MouseButton, PathPromptOptions, Pixels, Render, SharedString, Window,
    div, prelude::*, px,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Options that control upload status behavior.
pub enum UploadStatus {
    /// Marks an upload file as ready but not yet uploading.
    Ready,
    /// Marks an upload file as currently uploading.
    Uploading,
    /// Uses success semantic color tokens.
    Success,
    /// Reports a error failure.
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control upload list type behavior.
pub enum UploadListType {
    #[default]
    /// Accepts plain text input.
    Text,
    /// Renders upload items as picture-card tiles.
    PictureCard,
}

#[derive(Debug, Clone, PartialEq)]
/// Fluent native GPUI component for rendering Liora upload file.
pub struct UploadFile {
    /// Stable identifier used for GPUI state, callbacks, and automation.
    pub id: SharedString,
    /// Display name shown to users for this item.
    pub name: SharedString,
    /// File size in bytes reported by GitHub release metadata.
    pub size: Option<u64>,
    /// Current lifecycle status shown by this item.
    pub status: UploadStatus,
    /// Upload progress as a normalized percentage.
    pub progress: u8,
    /// Supporting descriptive text shown near the primary label.
    pub description: Option<SharedString>,
    /// Filesystem path associated with this item.
    pub path: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Options that control upload reject reason behavior.
pub enum UploadRejectReason {
    /// Uses the `TypeMismatch` option for `UploadRejectReason`.
    TypeMismatch,
    /// Uses the `TooLarge` option for `UploadRejectReason`.
    TooLarge,
    /// Uses the `MetadataUnavailable` option for `UploadRejectReason`.
    MetadataUnavailable,
}

/// Fluent native GPUI component for rendering Liora upload.
pub struct Upload {
    id: SharedString,
    files: Vec<UploadFile>,
    list_type: UploadListType,
    drag: bool,
    disabled: bool,
    multiple: bool,
    limit: Option<usize>,
    accept: Option<SharedString>,
    max_size: Option<u64>,
    selecting: bool,
    last_error: Option<SharedString>,
    button_text: SharedString,
    tip: Option<SharedString>,
    width: Option<Pixels>,
    on_select: Option<Arc<dyn Fn(&mut Upload, &mut Context<Upload>) + 'static>>,
    on_remove:
        Option<Arc<dyn Fn(&mut Upload, UploadFile, &mut Window, &mut Context<Upload>) + 'static>>,
}

impl UploadFile {
    /// Creates `UploadFile` initialized from the supplied id, and name.
    pub fn new(id: impl Into<SharedString>, name: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            size: None,
            status: UploadStatus::Ready,
            progress: 0,
            description: None,
            path: None,
        }
    }

    /// Sets an explicit icon size while preserving the default color behavior.
    pub fn size(mut self, size: u64) -> Self {
        self.size = Some(size);
        self
    }

    /// Sets the status value used by the component.
    pub fn status(mut self, status: UploadStatus) -> Self {
        self.status = status;
        self
    }

    /// Sets the progress value used by the component.
    pub fn progress(mut self, progress: u8) -> Self {
        self.progress = progress.min(100);
        self
    }

    /// Sets secondary descriptive text shown below the primary label.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Uses a local path as the component source.
    pub fn path(mut self, path: impl Into<PathBuf>) -> Self {
        self.path = Some(path.into());
        self
    }
}

impl Upload {
    /// Creates `Upload` with default theme-driven styling and no optional callbacks attached.
    pub fn new() -> Self {
        Self {
            id: liora_core::unique_id("upload"),
            files: Vec::new(),
            list_type: UploadListType::Text,
            drag: false,
            disabled: false,
            multiple: false,
            limit: None,
            accept: None,
            max_size: None,
            selecting: false,
            last_error: None,
            button_text: "点击上传".into(),
            tip: None,
            width: None,
            on_select: None,
            on_remove: None,
        }
    }

    /// Assigns a stable element id used by GPUI state, hit testing, and automated interaction tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Sets the files value used by the component.
    pub fn files(mut self, files: impl IntoIterator<Item = UploadFile>) -> Self {
        self.files = files.into_iter().collect();
        self
    }

    /// Adds file to the component state.
    pub fn add_file(mut self, file: UploadFile) -> Self {
        self.files.push(file);
        self
    }

    /// Sets the list type value used by the component.
    pub fn list_type(mut self, list_type: UploadListType) -> Self {
        self.list_type = list_type;
        self
    }

    /// Sets the picture card value used by the component.
    pub fn picture_card(self) -> Self {
        self.list_type(UploadListType::PictureCard)
    }

    /// Sets the drag value used by the component.
    pub fn drag(mut self, drag: bool) -> Self {
        self.drag = drag;
        self
    }

    /// Toggles the disabled state and suppresses user interaction when enabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Enables multi-selection behavior.
    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    /// Sets the limit value used by the component.
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the accept value used by the component.
    pub fn accept(mut self, accept: impl Into<SharedString>) -> Self {
        self.accept = Some(accept.into());
        self
    }

    /// Sets the maximum size limit.
    pub fn max_size(mut self, bytes: u64) -> Self {
        self.max_size = Some(bytes);
        self
    }

    /// Sets the button text value used by the component.
    pub fn button_text(mut self, text: impl Into<SharedString>) -> Self {
        self.button_text = text.into();
        self
    }

    /// Sets the tip value used by the component.
    pub fn tip(mut self, tip: impl Into<SharedString>) -> Self {
        self.tip = Some(tip.into());
        self
    }

    /// Sets the component width token used during GPUI layout.
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Applies the predefined width lg sizing preset.
    pub fn width_lg(self) -> Self {
        self.width(px(420.0))
    }

    /// Registers a callback that runs when select occurs.
    pub fn on_select(mut self, f: impl Fn(&mut Upload, &mut Context<Upload>) + 'static) -> Self {
        self.on_select = Some(Arc::new(f));
        self
    }

    /// Registers a callback that runs when remove occurs.
    pub fn on_remove(
        mut self,
        f: impl Fn(&mut Upload, UploadFile, &mut Window, &mut Context<Upload>) + 'static,
    ) -> Self {
        self.on_remove = Some(Arc::new(f));
        self
    }

    /// Updates the stored files value and keeps the existing component identity.
    pub fn set_files(&mut self, files: Vec<UploadFile>, cx: &mut Context<Self>) {
        self.files = files;
        cx.notify();
    }

    /// Performs the push file operation used by this component.
    pub fn push_file(&mut self, file: UploadFile, cx: &mut Context<Self>) {
        if !Self::can_accept_more_len(self.files.len(), self.limit, self.disabled) {
            return;
        }
        self.files.push(file);
        cx.notify();
    }

    /// Performs the file count operation used by this component.
    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    /// Performs the files ref operation used by this component.
    pub fn files_ref(&self) -> &[UploadFile] {
        &self.files
    }

    /// Returns filesystem paths for the selected resources.
    pub fn selected_paths(&self) -> Vec<PathBuf> {
        self.files
            .iter()
            .filter_map(|file| file.path.clone())
            .collect()
    }

    /// Returns whether the requested accept more len operation is currently allowed.
    pub fn can_accept_more_len(current_len: usize, limit: Option<usize>, disabled: bool) -> bool {
        !disabled && !limit.is_some_and(|limit| current_len >= limit)
    }

    /// Returns whether matches accept name is currently enabled or available.
    pub fn matches_accept_name(name: &str, accept: Option<&str>) -> bool {
        let Some(accept) = accept else {
            return true;
        };
        let accept = accept.trim();
        if accept.is_empty() {
            return true;
        }

        let lower_name = name.to_lowercase();
        let ext = Path::new(name)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase());

        accept
            .split(',')
            .map(str::trim)
            .filter(|token| !token.is_empty())
            .any(|token| {
                let token = token.to_lowercase();
                if token == "*" || token == "*/*" {
                    return true;
                }
                if let Some(expected_ext) = token.strip_prefix('.') {
                    return ext.as_deref() == Some(expected_ext);
                }
                if token.ends_with("/*") {
                    return matches_mime_group(ext.as_deref(), token.trim_end_matches("/*"));
                }
                lower_name.ends_with(&token)
            })
    }

    /// Performs the validate file name size operation used by this component.
    pub fn validate_file_name_size(
        name: &str,
        size: Option<u64>,
        accept: Option<&str>,
        max_size: Option<u64>,
    ) -> Result<(), UploadRejectReason> {
        if !Self::matches_accept_name(name, accept) {
            return Err(UploadRejectReason::TypeMismatch);
        }
        if let Some(max_size) = max_size {
            let Some(size) = size else {
                return Err(UploadRejectReason::MetadataUnavailable);
            };
            if size > max_size {
                return Err(UploadRejectReason::TooLarge);
            }
        }
        Ok(())
    }

    /// Returns the filesystem path for the validate resource.
    pub fn validate_path(
        path: &Path,
        accept: Option<&str>,
        max_size: Option<u64>,
    ) -> Result<UploadFile, UploadRejectReason> {
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("file")
            .to_string();
        let size = std::fs::metadata(path).ok().map(|metadata| metadata.len());
        Self::validate_file_name_size(&name, size, accept, max_size)?;
        Ok(UploadFile::new(path.to_string_lossy().into_owned(), name)
            .size(size.unwrap_or(0))
            .path(path.to_path_buf()))
    }

    /// Removes the matching file by id from the component state.
    pub fn remove_file_by_id(&mut self, id: &str, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(index) = self.files.iter().position(|file| file.id.as_ref() == id) {
            let file = self.files.remove(index);
            if let Some(on_remove) = self.on_remove.clone() {
                on_remove(self, file, window, cx);
            } else {
                cx.notify();
            }
        }
    }

    fn trigger_select(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !Self::can_accept_more_len(self.files.len(), self.limit, self.disabled) || self.selecting
        {
            return;
        }

        self.selecting = true;
        self.last_error = None;
        let receiver = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: self.multiple,
            prompt: Some(self.button_text.clone()),
        });
        cx.notify();

        cx.spawn(async move |this, cx| {
            let result = receiver.await;
            this.update(cx, |upload, cx| {
                upload.selecting = false;
                match result {
                    Ok(Ok(Some(paths))) => {
                        upload.accept_selected_paths(paths, cx);
                    }
                    Ok(Ok(None)) => {
                        upload.last_error = None;
                        cx.notify();
                    }
                    Ok(Err(err)) => {
                        upload.last_error = Some(format!("文件选择器打开失败：{err}").into());
                        cx.notify();
                    }
                    Err(_) => {
                        upload.last_error = Some("文件选择器已取消".into());
                        cx.notify();
                    }
                }
            })
            .ok();
        })
        .detach();

        let _ = window;
    }

    fn accept_selected_paths(&mut self, paths: Vec<PathBuf>, cx: &mut Context<Self>) {
        let accept = self.accept.as_ref().map(SharedString::as_str);
        let mut rejected = 0usize;
        for path in paths {
            if !Self::can_accept_more_len(self.files.len(), self.limit, self.disabled) {
                break;
            }
            match Self::validate_path(&path, accept, self.max_size) {
                Ok(file) => self.files.push(file),
                Err(_) => rejected += 1,
            }
            if !self.multiple {
                break;
            }
        }
        if rejected > 0 {
            self.last_error =
                Some(format!("已忽略 {rejected} 个不符合类型或大小限制的文件").into());
        } else {
            self.last_error = None;
        }
        if let Some(on_select) = self.on_select.clone() {
            on_select(self, cx);
        }
        cx.notify();
    }
}

impl Render for Upload {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let can_add = Self::can_accept_more_len(self.files.len(), self.limit, self.disabled)
            && !self.selecting;
        let entity = cx.entity().clone();
        let trigger_id = format!("{}-trigger", self.id);

        div()
            .flex()
            .flex_col()
            .gap_3()
            .when_some(self.width, |s, width| s.w(width))
            .child(if self.drag {
                render_drag_trigger(
                    trigger_id,
                    self.button_text.clone(),
                    self.accept.clone(),
                    self.multiple,
                    self.max_size,
                    can_add,
                    &theme,
                    entity.clone(),
                )
                .into_any_element()
            } else {
                render_button_trigger(
                    trigger_id,
                    if self.selecting {
                        "选择中...".into()
                    } else {
                        self.button_text.clone()
                    },
                    can_add,
                    &theme,
                    entity.clone(),
                )
                .into_any_element()
            })
            .when_some(self.tip.clone(), |s, tip| {
                s.child(div().text_xs().text_color(theme.neutral.text_3).child(tip))
            })
            .when_some(self.last_error.clone(), |s, error| {
                s.child(div().text_xs().text_color(theme.danger.base).child(error))
            })
            .child(match self.list_type {
                UploadListType::Text => {
                    render_text_file_list(self.id.clone(), &self.files, &theme, entity)
                        .into_any_element()
                }
                UploadListType::PictureCard => {
                    render_picture_file_list(self.id.clone(), &self.files, &theme, entity)
                        .into_any_element()
                }
            })
    }
}

fn render_button_trigger(
    id: String,
    text: SharedString,
    enabled: bool,
    theme: &liora_theme::Theme,
    upload: gpui::Entity<Upload>,
) -> impl IntoElement {
    let trigger_color = if enabled {
        theme.primary.base
    } else {
        theme.neutral.text_3
    };

    div()
        .id(element_id(id))
        .flex()
        .items_center()
        .gap_2()
        .px_4()
        .py_2()
        .rounded(px(theme.radius.md))
        .border_1()
        .border_color(if enabled {
            theme.primary.base
        } else {
            theme.neutral.border
        })
        .bg(if enabled {
            theme.neutral.card
        } else {
            theme.neutral.hover
        })
        .text_color(trigger_color)
        .cursor_pointer()
        .hover(|s| {
            if enabled {
                s.cursor_pointer().bg(theme.primary.light_9)
            } else {
                s
            }
        })
        .on_mouse_down(MouseButton::Left, move |_, window, cx| {
            if enabled {
                upload.update(cx, |upload, cx| upload.trigger_select(window, cx));
            }
        })
        .child(
            Icon::new(IconName::Upload)
                .size(px(16.0))
                .color(trigger_color),
        )
        .child(div().text_sm().child(text))
}

fn render_drag_trigger(
    id: String,
    text: SharedString,
    accept: Option<SharedString>,
    multiple: bool,
    max_size: Option<u64>,
    enabled: bool,
    theme: &liora_theme::Theme,
    upload: gpui::Entity<Upload>,
) -> impl IntoElement {
    let hint = match (multiple, accept) {
        (true, Some(accept)) => format!("支持多选，类型：{}", accept),
        (true, None) => "支持多选".to_string(),
        (false, Some(accept)) => format!("文件类型：{}", accept),
        (false, None) => "将文件拖到此处，或点击选择".to_string(),
    };
    let hint = match max_size {
        Some(max_size) => format!("{}，单文件 ≤ {}", hint, format_size(max_size)),
        None => hint,
    };
    let text_color = if enabled {
        theme.neutral.text_1
    } else {
        theme.neutral.text_3
    };

    div()
        .id(element_id(id))
        .h(px(150.0))
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap_3()
        .rounded(px(theme.radius.lg))
        .border_1()
        .border_color(if enabled {
            theme.primary.light_9
        } else {
            theme.neutral.border
        })
        .bg(if enabled {
            theme.primary.light_9
        } else {
            theme.neutral.hover
        })
        .cursor_pointer()
        .hover(|s| {
            if enabled {
                s.cursor_pointer().border_color(theme.primary.base)
            } else {
                s
            }
        })
        .on_mouse_down(MouseButton::Left, move |_, window, cx| {
            if enabled {
                upload.update(cx, |upload, cx| upload.trigger_select(window, cx));
            }
        })
        .child(Icon::new(IconName::Upload).size(px(32.0)).color(text_color))
        .child(div().text_sm().text_color(text_color).child(text))
        .child(div().text_xs().text_color(theme.neutral.text_3).child(hint))
}

fn render_text_file_list(
    id: SharedString,
    files: &[UploadFile],
    theme: &liora_theme::Theme,
    upload: gpui::Entity<Upload>,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .children(files.iter().cloned().map(move |file| {
            render_text_file_item(
                format!("{}-file-{}", id, file.id),
                file,
                theme.clone(),
                upload.clone(),
            )
        }))
}

fn render_text_file_item(
    id: String,
    file: UploadFile,
    theme: liora_theme::Theme,
    upload: gpui::Entity<Upload>,
) -> impl IntoElement {
    let file_id = file.id.clone();
    div()
        .id(element_id(id))
        .flex()
        .flex_col()
        .gap_1()
        .rounded(px(theme.radius.md))
        .px_3()
        .py_2()
        .bg(theme.neutral.card)
        .border_1()
        .border_color(theme.neutral.border)
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(status_icon(file.status, &theme))
                .child(
                    div()
                        .flex_1()
                        .min_w(px(0.0))
                        .child(
                            div()
                                .text_sm()
                                .text_color(theme.neutral.text_1)
                                .child(file.name.clone()),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.neutral.text_3)
                                .child(file_meta(&file)),
                        ),
                )
                .child(
                    div()
                        .id(element_id(format!("{}-remove", file_id)))
                        .p_1()
                        .rounded(px(theme.radius.sm))
                        .cursor_pointer()
                        .hover(|s| s.cursor_pointer().bg(theme.neutral.hover))
                        .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                            let file_id = file_id.clone();
                            upload.update(cx, |upload, cx| {
                                upload.remove_file_by_id(file_id.as_ref(), window, cx)
                            });
                        })
                        .child(
                            Icon::new(IconName::Trash2)
                                .size(px(14.0))
                                .color(theme.neutral.icon),
                        ),
                ),
        )
        .when(file.status == UploadStatus::Uploading, |s| {
            s.child(progress_bar(file.progress, &theme))
        })
}

fn render_picture_file_list(
    id: SharedString,
    files: &[UploadFile],
    theme: &liora_theme::Theme,
    upload: gpui::Entity<Upload>,
) -> impl IntoElement {
    div()
        .flex()
        .flex_wrap()
        .gap_3()
        .children(files.iter().cloned().map(move |file| {
            let file_id = file.id.clone();
            let remove_id = file.id.clone();
            div()
                .id(element_id(format!("{}-picture-{}", id, file.id)))
                .relative()
                .w(px(112.0))
                .h(px(112.0))
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .gap_2()
                .rounded(px(theme.radius.lg))
                .border_1()
                .border_color(theme.neutral.border)
                .bg(theme.neutral.hover)
                .child(status_icon(file.status, &theme).size(px(24.0)))
                .child(
                    div()
                        .px_2()
                        .text_xs()
                        .text_color(theme.neutral.text_1)
                        .child(file.name.clone()),
                )
                .when(file.status == UploadStatus::Uploading, |s| {
                    s.child(
                        div()
                            .absolute()
                            .bottom(px(8.0))
                            .left(px(8.0))
                            .right(px(8.0))
                            .child(progress_bar(file.progress, &theme)),
                    )
                })
                .child({
                    let upload = upload.clone();
                    div()
                        .id(element_id(format!("{}-picture-remove", file_id)))
                        .absolute()
                        .top(px(6.0))
                        .right(px(6.0))
                        .p_1()
                        .rounded(px(theme.radius.sm))
                        .bg(theme.neutral.card.opacity(0.9))
                        .cursor_pointer()
                        .hover(|s| s.cursor_pointer().bg(theme.neutral.card))
                        .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                            let remove_id = remove_id.clone();
                            upload.update(cx, |upload, cx| {
                                upload.remove_file_by_id(remove_id.as_ref(), window, cx)
                            });
                        })
                        .child(
                            Icon::new(IconName::X)
                                .size(px(14.0))
                                .color(theme.neutral.icon),
                        )
                })
        }))
}

fn matches_mime_group(ext: Option<&str>, group: &str) -> bool {
    match group {
        "image" => matches!(
            ext,
            Some("png" | "jpg" | "jpeg" | "gif" | "webp" | "bmp" | "svg")
        ),
        "text" => matches!(
            ext,
            Some("txt" | "md" | "csv" | "json" | "toml" | "yaml" | "yml" | "rs")
        ),
        "audio" => matches!(ext, Some("mp3" | "wav" | "ogg" | "flac" | "m4a")),
        "video" => matches!(ext, Some("mp4" | "mov" | "webm" | "mkv" | "avi")),
        _ => false,
    }
}

fn status_icon(status: UploadStatus, theme: &liora_theme::Theme) -> Icon {
    match status {
        UploadStatus::Ready => Icon::new(IconName::File)
            .size(px(16.0))
            .color(theme.neutral.icon),
        UploadStatus::Uploading => Icon::new(IconName::Upload)
            .size(px(16.0))
            .color(theme.primary.base),
        UploadStatus::Success => Icon::new(IconName::CircleCheck)
            .size(px(16.0))
            .color(theme.success.base),
        UploadStatus::Error => Icon::new(IconName::CircleX)
            .size(px(16.0))
            .color(theme.danger.base),
    }
}

fn progress_bar(progress: u8, theme: &liora_theme::Theme) -> impl IntoElement {
    div()
        .h(px(4.0))
        .rounded(px(999.0))
        .bg(theme.neutral.hover)
        .child(
            div()
                .h_full()
                .w(gpui::relative(progress as f32 / 100.0))
                .rounded(px(999.0))
                .bg(theme.primary.base),
        )
}

fn file_meta(file: &UploadFile) -> String {
    let status = match file.status {
        UploadStatus::Ready => "等待上传",
        UploadStatus::Uploading => "上传中",
        UploadStatus::Success => "上传成功",
        UploadStatus::Error => "上传失败",
    };
    let size = file
        .size
        .map(format_size)
        .unwrap_or_else(|| "未知大小".to_string());
    match &file.description {
        Some(description) => format!("{} · {} · {}", status, size, description),
        None => format!("{} · {}", status, size),
    }
}

fn format_size(size: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    let size = size as f64;
    if size >= MB {
        format!("{:.1} MB", size / MB)
    } else if size >= KB {
        format!("{:.1} KB", size / KB)
    } else {
        format!("{} B", size as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upload_width_lg_sets_demo_width() {
        assert_eq!(Upload::new().width_lg().width, Some(px(420.0)));
    }
}
