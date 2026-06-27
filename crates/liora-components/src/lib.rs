//! Liora's public GPUI component prelude.
//!
//! `liora-components` exports the visual and interactive controls used by the
//! native Gallery and Docs applications: form controls, overlays, navigation,
//! data display, charts, code blocks/editors, virtualized data views, and small
//! utility widgets.
//!
//! ## Application setup
//!
//! A GPUI app should initialize Liora once during application startup:
//!
//! ```no_run
//! use gpui::App;
//! use liora_components::init_liora;
//!
//! fn setup(cx: &mut App) {
//!     init_liora(cx);
//! }
//! ```
//!
//! The high-level `liora_components::init_liora(cx)` entry point initializes
//! Liora core/theme state, global component services, and the app-level key
//! bindings needed by inputs, text/code selection, overlays, Preview, Tour, and
//! picker popups. Use `liora_components::init_liora_with_mode(...)` for an
//! explicit Light or Dark startup mode.
//!
//! ## Stateful controls
//!
//! Controls with focus, selection, open state, or text value should normally be
//! stored as `gpui::Entity<T>` fields in a parent view. This preserves state
//! across GPUI renders. Gallery and Docs are the maintained examples for this pattern.
//!
//! ## Architecture boundary
//!
//! Liora components render native GPUI element trees. This crate does not require
//! Tauri, WebView, HTML/CSS, DOM, or a browser runtime.

pub mod accordion;
pub mod affix;
pub mod alert;
pub mod anchor;
pub mod area_chart;
pub mod autocomplete;
pub mod avatar;
pub mod backtop;
pub mod badge;
pub mod bar_chart;
pub mod breadcrumb;
pub mod button;
pub mod button_group;
pub mod calendar;
pub mod card;
pub mod carousel;
pub mod cascader;
pub mod chart;
mod chart_frame;
pub mod chart_scale;
pub mod chart_shape;
pub mod checkbox;
pub mod checkbox_group;
pub mod code_block;
pub mod code_editor;
pub mod col;
pub mod collapse;
pub mod color_picker;
pub mod container;
pub mod date_picker;
pub mod date_time_picker;
pub mod descriptions;
pub mod dialog;
pub mod divider;
pub mod draggable;
pub mod drawer;
pub mod dropdown;
pub mod dropdown_button;
pub mod empty;
pub mod flex;
pub mod form;
pub(crate) mod gpui_compat;
pub mod heat_bar;
pub mod horizontal_list;
pub mod image;
pub mod input;
pub mod input_number;
pub mod input_tag;
pub mod kbd;
pub mod label;
pub mod layout_helpers;
pub mod line_chart;
pub mod link;
pub mod loading;
pub mod mention;
pub mod menu;
pub mod message;
pub mod message_box;
pub mod motion;
pub mod notification;
pub mod operation;
pub mod otp_input;
pub mod page_header;
pub mod pagination;
pub mod paragraph;
pub mod pie_chart;
pub mod popconfirm;
pub mod popover;
pub mod preview;
pub mod progress;
pub mod qr_code;
pub mod radio;
pub mod radio_group;
pub mod rate;
pub mod result;
pub mod row;
pub mod scrollbar;
pub mod segment_ratio_bar;
pub mod segmented;
pub mod select;
pub mod selectable_text;
pub mod shell;
pub mod sidebar;
pub mod signal_meter;
pub mod skeleton;
pub mod slider;
pub mod space;
pub mod sparkline;
pub mod spinner;
pub mod splitter;
pub mod statistic;
pub mod steps;
pub mod switch;
pub mod table;
pub mod tabs;
pub mod tag;
pub mod text;
pub mod textarea;
pub mod time_picker;
pub mod timeline;
pub mod timer;
pub mod title;
pub mod titlebar;
pub mod tooltip;
pub mod tour;
pub mod transfer;
pub mod tree;
pub mod tree_select;
pub mod upload;
pub mod virtualized_list;
pub mod virtualized_table;
pub mod virtualized_tree;
pub mod watermark;
pub mod window_frame;

pub use accordion::*;
pub use affix::*;
pub use alert::*;
pub use anchor::*;
pub use area_chart::*;
pub use autocomplete::*;
pub use avatar::*;
pub use backtop::*;
pub use badge::*;
pub use bar_chart::*;
pub use breadcrumb::*;
pub use button::*;
pub use button_group::*;
pub use calendar::*;
pub use card::*;
pub use carousel::*;
pub use cascader::*;
pub use chart::*;
pub use chart_scale::*;
pub use chart_shape::*;
pub use checkbox::*;
pub use checkbox_group::*;
pub use code_block::*;
pub use code_editor::*;
pub use col::*;
pub use collapse::*;
pub use color_picker::*;
pub use container::*;
pub use date_picker::*;
pub use date_time_picker::*;
pub use descriptions::*;
pub use dialog::*;
pub use divider::*;
pub use draggable::*;
pub use drawer::*;
pub use dropdown::*;
pub use dropdown_button::*;
pub use empty::*;
pub use flex::*;
pub use form::*;
pub use heat_bar::*;
pub use horizontal_list::*;
pub use image::*;
pub use input::*;
pub use input_number::*;
pub use input_tag::*;
pub use kbd::*;
pub use label::*;
pub use line_chart::*;
pub use link::*;
pub use liora_core::{
    EmbeddedFont, FontConfig, FontDiscoveryReport, FontLoadFailure, FontLoadMode, FontLoadOptions,
    FontLoadReport, LinuxDesktopIdentity, LinuxDesktopPngIcon, LioraOptions, ThemeMode,
    apply_theme_mode, attach_system_theme_observer, discover_font_files,
    ensure_linux_desktop_identity, is_font_family_available, is_supported_font_path,
    linux_desktop_entry, linux_desktop_png_icon_path, load_app_fonts, load_custom_fonts,
    load_embedded_fonts, load_font_assets, load_font_files, load_fonts_from_dir, set_font_config,
    startup_maximized_window_bounds, sync_system_theme,
};
pub use liora_theme::{ButtonSize, ButtonVariant};
pub use loading::*;
pub use mention::*;
pub use menu::*;
pub use message::*;
pub use message_box::*;
pub use motion::*;
pub use notification::*;
pub use operation::*;
pub use otp_input::*;
pub use page_header::*;
pub use pagination::*;
pub use paragraph::*;
pub use pie_chart::*;
pub use popconfirm::*;
pub use popover::*;
pub use preview::*;
pub use progress::*;
pub use qr_code::*;
pub use radio::*;
pub use radio_group::*;
pub use rate::*;
pub use result::*;
pub use row::*;
pub use scrollbar::*;
pub use segment_ratio_bar::*;
pub use segmented::*;
pub use select::*;
pub use selectable_text::*;
pub use shell::*;
pub use sidebar::*;
pub use signal_meter::*;
pub use skeleton::*;
pub use slider::*;
pub use space::*;
pub use sparkline::*;
pub use spinner::*;
pub use splitter::*;
pub use statistic::*;
pub use steps::*;
pub use switch::*;
pub use table::*;
pub use tabs::*;
pub use tag::*;
pub use text::*;
pub use textarea::*;
pub use time_picker::*;
pub use timeline::*;
pub use timer::*;
pub use title::*;
pub use titlebar::*;
pub use tooltip::*;
pub use tour::*;
pub use transfer::*;
pub use tree::*;
pub use tree_select::*;
pub use upload::*;
pub use virtualized_list::*;
pub use virtualized_table::*;
pub use virtualized_tree::*;
pub use watermark::*;
pub use window_frame::*;

/// Initialize Liora's recommended application runtime in one call.
///
/// This is the high-level setup entry point application authors should use for
/// normal Liora + GPUI apps. It initializes the core theme/portal state with
/// [`liora_core::ThemeMode::System`], installs global component services such as
/// [`MessageManager`], and registers key bindings for interactive components.
///
/// Use [`init_liora_with_mode`] when a product needs an explicit Light or Dark
/// startup mode, or [`init_liora_with_options`] when it needs custom font
/// families after registering app-provided font bytes. The lower-level
/// `liora_core::init_liora(...)` functions remain
/// available for advanced crate-local setup, but they intentionally initialize
/// only core/theme state and not component services.
pub fn init_liora(cx: &mut gpui::App) {
    init_liora_with_mode(cx, ThemeMode::System);
}

/// Initialize Liora's recommended application runtime with an explicit theme mode.
pub fn init_liora_with_mode(cx: &mut gpui::App, mode: ThemeMode) {
    init_liora_with_options(cx, LioraOptions::default().with_theme_mode(mode));
}

/// Initialize Liora's recommended application runtime with full startup options.
pub fn init_liora_with_options(cx: &mut gpui::App, options: LioraOptions) {
    liora_core::init_liora_with_options(cx, options);
    MessageManager::init(cx);
    register_liora_key_bindings(cx);
}

fn register_liora_key_bindings(cx: &mut gpui::App) {
    Input::register_key_bindings(cx);
    CodeBlock::register_key_bindings(cx);
    CodeEditor::register_key_bindings(cx);
    Checkbox::register_key_bindings(cx);
    CheckboxGroup::register_key_bindings(cx);
    Radio::register_key_bindings(cx);
    RadioGroup::register_key_bindings(cx);
    Switch::register_key_bindings(cx);
    Dialog::register_key_bindings(cx);
    Drawer::register_key_bindings(cx);
    Preview::register_key_bindings(cx);
    Autocomplete::register_key_bindings(cx);
    Cascader::register_key_bindings(cx);
    ColorPicker::register_key_bindings(cx);
    DatePicker::register_key_bindings(cx);
    DateTimePicker::register_key_bindings(cx);
    Popover::register_key_bindings(cx);
    Select::register_key_bindings(cx);
    TimePicker::register_key_bindings(cx);
    Text::register_key_bindings(cx);
    Paragraph::register_key_bindings(cx);
    Title::register_key_bindings(cx);
    Tour::register_key_bindings(cx);
}

#[cfg(test)]
mod application_init_api_tests {
    #[test]
    fn component_modules_have_english_module_documentation() {
        let modules = [
            ("accordion.rs", include_str!("accordion.rs")),
            ("affix.rs", include_str!("affix.rs")),
            ("alert.rs", include_str!("alert.rs")),
            ("anchor.rs", include_str!("anchor.rs")),
            ("area_chart.rs", include_str!("area_chart.rs")),
            ("autocomplete.rs", include_str!("autocomplete.rs")),
            ("avatar.rs", include_str!("avatar.rs")),
            ("backtop.rs", include_str!("backtop.rs")),
            ("badge.rs", include_str!("badge.rs")),
            ("bar_chart.rs", include_str!("bar_chart.rs")),
            ("breadcrumb.rs", include_str!("breadcrumb.rs")),
            ("button.rs", include_str!("button.rs")),
            ("button_group.rs", include_str!("button_group.rs")),
            ("calendar.rs", include_str!("calendar.rs")),
            ("card.rs", include_str!("card.rs")),
            ("carousel.rs", include_str!("carousel.rs")),
            ("cascader.rs", include_str!("cascader.rs")),
            ("chart.rs", include_str!("chart.rs")),
            ("chart_frame.rs", include_str!("chart_frame.rs")),
            ("chart_scale.rs", include_str!("chart_scale.rs")),
            ("chart_shape.rs", include_str!("chart_shape.rs")),
            ("checkbox.rs", include_str!("checkbox.rs")),
            ("checkbox_group.rs", include_str!("checkbox_group.rs")),
            ("code_block.rs", include_str!("code_block.rs")),
            ("code_editor.rs", include_str!("code_editor.rs")),
            ("col.rs", include_str!("col.rs")),
            ("collapse.rs", include_str!("collapse.rs")),
            ("color_picker.rs", include_str!("color_picker.rs")),
            ("container.rs", include_str!("container.rs")),
            ("date_picker.rs", include_str!("date_picker.rs")),
            ("date_time_picker.rs", include_str!("date_time_picker.rs")),
            ("descriptions.rs", include_str!("descriptions.rs")),
            ("dialog.rs", include_str!("dialog.rs")),
            ("divider.rs", include_str!("divider.rs")),
            ("draggable.rs", include_str!("draggable.rs")),
            ("drawer.rs", include_str!("drawer.rs")),
            ("dropdown.rs", include_str!("dropdown.rs")),
            ("empty.rs", include_str!("empty.rs")),
            ("flex.rs", include_str!("flex.rs")),
            ("form.rs", include_str!("form.rs")),
            ("gpui_compat.rs", include_str!("gpui_compat.rs")),
            ("heat_bar.rs", include_str!("heat_bar.rs")),
            ("horizontal_list.rs", include_str!("horizontal_list.rs")),
            ("image.rs", include_str!("image.rs")),
            ("input.rs", include_str!("input.rs")),
            ("input_number.rs", include_str!("input_number.rs")),
            ("input_tag.rs", include_str!("input_tag.rs")),
            ("label.rs", include_str!("label.rs")),
            ("kbd.rs", include_str!("kbd.rs")),
            ("layout_helpers.rs", include_str!("layout_helpers.rs")),
            ("line_chart.rs", include_str!("line_chart.rs")),
            ("link.rs", include_str!("link.rs")),
            ("loading.rs", include_str!("loading.rs")),
            ("mention.rs", include_str!("mention.rs")),
            ("menu.rs", include_str!("menu.rs")),
            ("message.rs", include_str!("message.rs")),
            ("message_box.rs", include_str!("message_box.rs")),
            ("motion.rs", include_str!("motion.rs")),
            ("notification.rs", include_str!("notification.rs")),
            ("operation.rs", include_str!("operation.rs")),
            ("otp_input.rs", include_str!("otp_input.rs")),
            ("page_header.rs", include_str!("page_header.rs")),
            ("pagination.rs", include_str!("pagination.rs")),
            ("paragraph.rs", include_str!("paragraph.rs")),
            ("pie_chart.rs", include_str!("pie_chart.rs")),
            ("popconfirm.rs", include_str!("popconfirm.rs")),
            ("popover.rs", include_str!("popover.rs")),
            ("preview.rs", include_str!("preview.rs")),
            ("progress.rs", include_str!("progress.rs")),
            ("qr_code.rs", include_str!("qr_code.rs")),
            ("radio.rs", include_str!("radio.rs")),
            ("radio_group.rs", include_str!("radio_group.rs")),
            ("rate.rs", include_str!("rate.rs")),
            ("result.rs", include_str!("result.rs")),
            ("row.rs", include_str!("row.rs")),
            ("scrollbar.rs", include_str!("scrollbar.rs")),
            ("segment_ratio_bar.rs", include_str!("segment_ratio_bar.rs")),
            ("segmented.rs", include_str!("segmented.rs")),
            ("select.rs", include_str!("select.rs")),
            ("selectable_text.rs", include_str!("selectable_text.rs")),
            ("signal_meter.rs", include_str!("signal_meter.rs")),
            ("skeleton.rs", include_str!("skeleton.rs")),
            ("slider.rs", include_str!("slider.rs")),
            ("space.rs", include_str!("space.rs")),
            ("spinner.rs", include_str!("spinner.rs")),
            ("sparkline.rs", include_str!("sparkline.rs")),
            ("splitter.rs", include_str!("splitter.rs")),
            ("statistic.rs", include_str!("statistic.rs")),
            ("steps.rs", include_str!("steps.rs")),
            ("switch.rs", include_str!("switch.rs")),
            ("table.rs", include_str!("table.rs")),
            ("tabs.rs", include_str!("tabs.rs")),
            ("tag.rs", include_str!("tag.rs")),
            ("text.rs", include_str!("text.rs")),
            ("textarea.rs", include_str!("textarea.rs")),
            ("time_picker.rs", include_str!("time_picker.rs")),
            ("timeline.rs", include_str!("timeline.rs")),
            ("timer.rs", include_str!("timer.rs")),
            ("title.rs", include_str!("title.rs")),
            ("tooltip.rs", include_str!("tooltip.rs")),
            ("tour.rs", include_str!("tour.rs")),
            ("transfer.rs", include_str!("transfer.rs")),
            ("tree.rs", include_str!("tree.rs")),
            ("tree_select.rs", include_str!("tree_select.rs")),
            ("upload.rs", include_str!("upload.rs")),
            ("virtualized_list.rs", include_str!("virtualized_list.rs")),
            ("virtualized_table.rs", include_str!("virtualized_table.rs")),
            ("virtualized_tree.rs", include_str!("virtualized_tree.rs")),
            ("watermark.rs", include_str!("watermark.rs")),
            ("window_frame.rs", include_str!("window_frame.rs")),
        ];

        for (path, source) in modules {
            assert!(
                source.starts_with("//!"),
                "{path} must start with module docs"
            );
            assert!(
                source.contains("## Usage model"),
                "{path} must document usage model"
            );
            assert!(
                source.contains("## Design contract"),
                "{path} must document design contract"
            );
            assert!(
                !source.contains("代目"),
                "{path} docs must be English, not draft text"
            );
            for forbidden in [
                "Configuration and state type for",
                "value used by this public",
                "Sets or returns",
                "Selects the value variant for this API",
                "variant for this API",
                "mode for this API",
                "Represents the ",
                "option for the ",
                "Configures or computes",
                "associated with this public",
                "Creates a new value with the required baseline configuration",
                "Creates a default",
                "initial public state",
                "documented default layout",
                "for this data model",
                "for fluent component construction",
                "Returns a copy configured with the supplied",
                "Applies the value setting",
                "setting and returns the updated builder",
                "Current value represented by this option or component state",
                "Human-readable label shown in the component UI",
                "Configured width used during layout",
                "Configured height used during layout",
                "Stable identifier used to connect rendered UI",
            ] {
                assert!(
                    !source.contains(forbidden),
                    "{path} rustdoc should be specific, not template phrase: {forbidden}"
                );
            }
        }
    }

    #[test]
    fn components_crate_exposes_one_line_application_init() {
        let source = include_str!("lib.rs");
        assert!(source.contains("pub fn init_liora(cx: &mut gpui::App)"));
        assert!(
            source.contains("pub fn init_liora_with_mode(cx: &mut gpui::App, mode: ThemeMode)")
        );
        assert!(source.contains("pub use liora_core::ThemeMode"));
        assert!(source.contains("fn register_liora_key_bindings(cx: &mut gpui::App)"));
        assert!(source.contains("MessageManager::init(cx)"));

        for component in [
            "Input",
            "CodeBlock",
            "CodeEditor",
            "Checkbox",
            "Radio",
            "RadioGroup",
            "Switch",
            "Dialog",
            "Drawer",
            "Preview",
            "Autocomplete",
            "Cascader",
            "ColorPicker",
            "DatePicker",
            "DateTimePicker",
            "Popover",
            "Select",
            "TimePicker",
            "Text",
            "Paragraph",
            "Title",
            "Tour",
        ] {
            let registration = format!("{component}::register_key_bindings(cx)");
            assert!(
                source.contains(&registration),
                "unified app init should include {registration}"
            );
        }
    }
}

#[cfg(test)]
mod motion_coverage_tests {
    #[test]
    fn interactive_surfaces_use_liora_motion() {
        let popup_sources = [
            include_str!("select.rs"),
            include_str!("cascader.rs"),
            include_str!("date_picker.rs"),
            include_str!("time_picker.rs"),
            include_str!("date_time_picker.rs"),
        ];

        for source in popup_sources {
            assert!(source.contains("panel-motion"));
            assert!(source.contains("pop_in("));
        }
    }

    #[test]
    fn interactive_state_indicators_use_liora_motion() {
        let state_sources = [
            include_str!("backtop.rs"),
            include_str!("checkbox.rs"),
            include_str!("radio.rs"),
            include_str!("collapse.rs"),
            include_str!("tree.rs"),
            include_str!("menu.rs"),
            include_str!("segmented.rs"),
            include_str!("tabs.rs"),
            include_str!("rate.rs"),
        ];

        for source in state_sources {
            assert!(source.contains("pop_in("));
        }
    }
}

#[cfg(test)]
mod overlay_escape_coverage_tests {
    #[test]
    fn overlay_like_components_expose_configurable_escape_close() {
        let components = [
            ("dialog", include_str!("dialog.rs")),
            ("drawer", include_str!("drawer.rs")),
            ("message_box", include_str!("message_box.rs")),
            ("preview", include_str!("preview.rs")),
            ("popover", include_str!("popover.rs")),
            ("dropdown", include_str!("dropdown.rs")),
            ("popconfirm", include_str!("popconfirm.rs")),
            ("menu", include_str!("menu.rs")),
            ("select", include_str!("select.rs")),
            ("cascader", include_str!("cascader.rs")),
            ("date_picker", include_str!("date_picker.rs")),
            ("date_time_picker", include_str!("date_time_picker.rs")),
            ("time_picker", include_str!("time_picker.rs")),
            ("color_picker", include_str!("color_picker.rs")),
            ("autocomplete", include_str!("autocomplete.rs")),
            ("tour", include_str!("tour.rs")),
        ];

        for (name, source) in components {
            assert!(
                source.contains("close_on_escape"),
                "{name} should expose/forward close_on_escape"
            );
            assert!(
                source.contains("close_on_escape: true")
                    || source.contains("close_on_escape = true")
                    || source.contains(".close_on_escape(")
                    || source.contains("close_on_escape: true,")
                    || name == "message_box",
                "{name} should default or forward Escape close behavior"
            );
        }
    }

    #[test]
    fn popover_wrappers_forward_click_outside_close_policy() {
        for (name, source) in [
            ("dropdown", include_str!("dropdown.rs")),
            ("popconfirm", include_str!("popconfirm.rs")),
        ] {
            assert!(
                source.contains("close_on_click_outside: true"),
                "{name} should default to click-outside close"
            );
            assert!(
                source.contains("pub fn close_on_click_outside("),
                "{name} should expose close_on_click_outside(...)"
            );
            assert!(
                source.contains(".close_on_click_outside(close_on_click_outside)"),
                "{name} should forward click-outside policy to Popover"
            );
        }
    }

    #[test]
    fn preview_exposes_click_outside_close_policy() {
        let source = include_str!("preview.rs");
        assert!(source.contains("close_on_click_outside: true"));
        assert!(source.contains("pub fn close_on_click_outside("));
        assert!(source.contains(".when(close_on_click_outside"));
        assert!(source.contains("preview.close_on_click_outside = self.close_on_click_outside"));
    }

    #[test]
    fn input_popups_expose_click_outside_close_policy() {
        for (name, source) in [
            ("select", include_str!("select.rs")),
            ("autocomplete", include_str!("autocomplete.rs")),
            ("cascader", include_str!("cascader.rs")),
            ("date_picker", include_str!("date_picker.rs")),
            ("date_time_picker", include_str!("date_time_picker.rs")),
            ("time_picker", include_str!("time_picker.rs")),
            ("color_picker", include_str!("color_picker.rs")),
        ] {
            assert!(
                source.contains("close_on_click_outside: true"),
                "{name} should default to click-outside close"
            );
            assert!(
                source.contains("pub fn close_on_click_outside("),
                "{name} should expose close_on_click_outside(...)"
            );
            assert!(
                source.contains(".when(close_on_click_outside"),
                "{name} should bind outside-click close conditionally"
            );
        }
    }

    #[test]
    fn popup_key_bindings_are_registered_by_unified_component_init() {
        let source = include_str!("lib.rs");
        let docs_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../apps/liora-docs/src/main.rs");
        let gallery_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../apps/liora-gallery/src/main.rs");

        if docs_path.exists() {
            let docs = std::fs::read_to_string(&docs_path).expect("read docs main.rs");
            assert!(docs.contains("init_liora(cx)"));
        }
        if gallery_path.exists() {
            let gallery = std::fs::read_to_string(&gallery_path).expect("read gallery main.rs");
            assert!(gallery.contains("init_liora(cx)"));
        }

        for component in [
            "Autocomplete",
            "Cascader",
            "ColorPicker",
            "DatePicker",
            "DateTimePicker",
            "Dialog",
            "Drawer",
            "Popover",
            "Preview",
            "Select",
            "TimePicker",
            "Tour",
        ] {
            let registration = format!("{component}::register_key_bindings(cx)");
            assert!(
                source.contains(&registration),
                "unified component init missing {registration}"
            );
        }
    }
}

#[cfg(test)]
mod api_consistency_audit_tests {
    #[test]
    fn public_callbacks_keep_value_window_app_shape_except_entity_local_controls() {
        let value_callbacks = [
            (
                "affix",
                include_str!("affix.rs"),
                "Fn(bool, &mut Window, &mut App)",
            ),
            (
                "autocomplete",
                include_str!("autocomplete.rs"),
                "Fn(AutocompleteItem, &mut Window, &mut App)",
            ),
            (
                "calendar",
                include_str!("calendar.rs"),
                "Fn(CalendarDate, &mut Window, &mut App)",
            ),
            (
                "checkbox",
                include_str!("checkbox.rs"),
                "Fn(bool, &mut Window, &mut App)",
            ),
            (
                "checkbox_group",
                include_str!("checkbox_group.rs"),
                "Fn(Vec<usize>, &mut Window, &mut App)",
            ),
            (
                "color_picker",
                include_str!("color_picker.rs"),
                "Fn(SharedString, &mut Window, &mut App)",
            ),
            (
                "input_number",
                include_str!("input_number.rs"),
                "Fn(f64, &mut Window, &mut App)",
            ),
            (
                "input_tag",
                include_str!("input_tag.rs"),
                "Fn(Vec<SharedString>, &mut Window, &mut App)",
            ),
            (
                "pagination",
                include_str!("pagination.rs"),
                "Fn(usize, &mut Window, &mut App)",
            ),
            (
                "radio_group",
                include_str!("radio_group.rs"),
                "Fn(usize, &mut Window, &mut App)",
            ),
            (
                "switch",
                include_str!("switch.rs"),
                "Fn(bool, &mut Window, &mut App)",
            ),
            (
                "tour",
                include_str!("tour.rs"),
                "Fn(usize, &mut Window, &mut App)",
            ),
        ];

        for (name, source, signature) in value_callbacks {
            assert!(
                source.contains(signature),
                "{name} should keep callback signature `{signature}`"
            );
        }

        let entity_local_callbacks = [
            (
                "input",
                include_str!("input.rs"),
                "Fn(&str, &mut Context<Self>)",
            ),
            (
                "code_editor",
                include_str!("code_editor.rs"),
                "Fn(&str, &mut Context<CodeEditor>)",
            ),
            (
                "horizontal_list",
                include_str!("horizontal_list.rs"),
                "Fn(usize, usize, &mut Window, &mut Context<HorizontalList>)",
            ),
        ];

        for (name, source, signature) in entity_local_callbacks {
            assert!(
                source.contains(signature),
                "{name} should document its entity-local callback context with `{signature}`"
            );
        }
    }

    #[test]
    fn state_builders_keep_consistent_boolean_builder_names() {
        let disabled_sources = [
            ("button", include_str!("button.rs")),
            ("checkbox", include_str!("checkbox.rs")),
            ("radio", include_str!("radio.rs")),
            ("switch", include_str!("switch.rs")),
            ("segmented", include_str!("segmented.rs")),
            ("upload", include_str!("upload.rs")),
            ("transfer", include_str!("transfer.rs")),
            ("horizontal_list", include_str!("horizontal_list.rs")),
        ];
        for (name, source) in disabled_sources {
            assert!(
                source.contains("pub fn disabled("),
                "{name} should expose disabled(...) as its public boolean state builder"
            );
        }

        for (name, source) in [
            ("dialog", include_str!("dialog.rs")),
            ("drawer", include_str!("drawer.rs")),
            ("popover", include_str!("popover.rs")),
            ("dropdown", include_str!("dropdown.rs")),
            ("popconfirm", include_str!("popconfirm.rs")),
            ("tour", include_str!("tour.rs")),
            ("select", include_str!("select.rs")),
            ("autocomplete", include_str!("autocomplete.rs")),
            ("date_picker", include_str!("date_picker.rs")),
            ("time_picker", include_str!("time_picker.rs")),
        ] {
            assert!(
                source.contains("pub fn close_on_escape("),
                "{name} should expose close_on_escape(...) for overlay keyboard behavior"
            );
        }
    }

    #[test]
    fn avoidable_runtime_panics_stay_out_of_hardened_paths() {
        let hardened_sources = [
            ("button", include_str!("button.rs")),
            ("chart", include_str!("chart.rs")),
            ("date_time_picker", include_str!("date_time_picker.rs")),
            ("input", include_str!("input.rs")),
            ("input_number", include_str!("input_number.rs")),
            ("sparkline", include_str!("sparkline.rs")),
        ];

        for (name, source) in hardened_sources {
            let production = source.split("#[cfg(test)]").next().unwrap_or(source);
            assert!(
                !production.contains(".unwrap()"),
                "{name} production path should not use avoidable bare unwrap()"
            );
            assert!(
                !production.contains("expect(\"valid default"),
                "{name} production path should not panic on constant default values"
            );
        }

        let code_block_production = include_str!("code_block.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap_or(include_str!("code_block.rs"));
        assert!(
            !code_block_production.contains(".paint(\n")
                || !code_block_production.contains(".unwrap();"),
            "CodeBlock paint paths should not panic on shaped text paint results"
        );
        assert!(
            !code_block_production.contains("lock poisoned")
                && code_block_production.contains("lock_highlight_cache")
                && code_block_production.contains("lock_selectable_state_map"),
            "CodeBlock synchronized caches should recover poisoned locks instead of panicking"
        );

        for (name, source, helper) in [
            (
                "selectable_text",
                include_str!("selectable_text.rs"),
                "lock_selection_state_map",
            ),
            ("timer", include_str!("timer.rs"), "lock_timer_windows"),
        ] {
            let production = source.split("#[cfg(test)]").next().unwrap_or(source);
            assert!(
                !production.contains("lock poisoned") && production.contains(helper),
                "{name} synchronized runtime state should recover poisoned locks instead of panicking"
            );
        }
    }
}

#[cfg(test)]
mod visual_theme_consistency_tests {
    #[test]
    fn hardened_colored_surfaces_use_theme_inverted_text_token() {
        for (name, source) in [
            ("tag", include_str!("tag.rs")),
            ("progress", include_str!("progress.rs")),
            ("badge", include_str!("badge.rs")),
            ("pagination", include_str!("pagination.rs")),
            ("bar_chart", include_str!("bar_chart.rs")),
            ("pie_chart", include_str!("pie_chart.rs")),
        ] {
            let production = source.split("#[cfg(test)]").next().unwrap_or(source);
            assert!(
                production.contains("theme.neutral.inverted"),
                "{name} should use theme.neutral.inverted for text on colored/dark surfaces"
            );
            assert!(
                !production.contains("gpui::white()"),
                "{name} production rendering should not hard-code white text"
            );
        }
    }

    #[test]
    fn gradient_buttons_use_theme_inverted_text_token() {
        let production = include_str!("button.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap_or_default();

        assert!(
            production.contains("let text = theme.neutral.inverted"),
            "gradient button text should use the semantic inverted text token"
        );
    }

    #[test]
    fn virtualized_components_use_theme_surface_border_and_radius_tokens() {
        for (name, source) in [
            ("virtualized_table", include_str!("virtualized_table.rs")),
            ("virtualized_tree", include_str!("virtualized_tree.rs")),
        ] {
            assert!(
                source.contains("theme.neutral.card"),
                "{name} should use the themed card surface"
            );
            assert!(
                source.contains("theme.neutral.border"),
                "{name} should use themed borders"
            );
            assert!(
                source.contains("theme.radius.md"),
                "{name} should use themed radius tokens"
            );
        }
    }

    #[test]
    fn modal_masks_and_loading_masks_use_theme_tokens() {
        for (name, source, token) in [
            ("dialog", include_str!("dialog.rs"), "theme.neutral.overlay"),
            ("drawer", include_str!("drawer.rs"), "theme.neutral.overlay"),
            ("tour", include_str!("tour.rs"), "theme.neutral.overlay"),
            ("loading", include_str!("loading.rs"), "theme.neutral.mask"),
        ] {
            let production = source.split("#[cfg(test)]").next().unwrap_or(source);
            assert!(production.contains(token), "{name} should use {token}");
            assert!(
                !production.contains("0x00000066") && !production.contains("0xFFFFFF99"),
                "{name} should not hard-code light/dark mask colors"
            );
        }
    }

    #[test]
    fn code_editor_and_window_frame_use_theme_interaction_tokens() {
        let code_editor = include_str!("code_editor.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap_or_default();
        assert!(code_editor.contains("theme.neutral.border"));
        assert!(!code_editor.contains("rgb(0xe2e8f0)"));

        let window_frame = include_str!("window_frame.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap_or_default();
        let titlebar = include_str!("titlebar.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap_or_default();
        assert!(window_frame.contains("TitleBar"));
        assert!(titlebar.contains("theme.danger.base"));
        assert!(titlebar.contains("theme.neutral.inverted"));
        assert!(!window_frame.contains("gpui::red()"));
        assert!(!window_frame.contains("gpui::white()"));
        assert!(!titlebar.contains("gpui::red()"));
        assert!(!titlebar.contains("gpui::white()"));
    }
}

#[cfg(test)]
mod shell_component_api_tests {
    use super::*;

    #[test]
    fn titlebar_and_sidebar_public_builders_are_available() {
        let _titlebar = TitleBar::new()
            .title("Liora")
            .subtitle("Native GPUI shell")
            .padding_x(gpui::px(16.0))
            .gap(gpui::px(8.0))
            .actions_gap(gpui::px(6.0))
            .background(gpui::transparent_black())
            .border_color(gpui::transparent_black())
            .border(true)
            .title_color(gpui::transparent_black())
            .subtitle_color(gpui::transparent_black())
            .content_align(TitleBarContentAlign::Start)
            .window_controls_position(WindowControlsPosition::Right)
            .window_controls(true)
            .draggable(true);

        let _sidebar = Sidebar::new()
            .id("app-sidebar")
            .brand("Liora")
            .brand_subtitle("Native GPUI")
            .logo("logo")
            .brand_action("action")
            .expanded_width(gpui::px(280.0))
            .collapsed_width(gpui::px(64.0))
            .header_padding(gpui::px(12.0))
            .content_padding(gpui::px(8.0))
            .footer_padding(gpui::px(12.0))
            .gap(gpui::px(8.0))
            .background(gpui::transparent_black())
            .border_color(gpui::transparent_black())
            .border(true)
            .rounded(gpui::px(12.0))
            .scrollable()
            .collapse_mode(SidebarCollapseMode::Full);

        let _shell = Shell::new("body")
            .id("app-shell")
            .mode(WindowFrameMode::Custom)
            .titlebar(TitleBar::new().title("Liora"))
            .header("header")
            .sidebar(Sidebar::new())
            .right_sidebar(Sidebar::new().right())
            .main("main")
            .footer("footer")
            .overlay("overlay")
            .main_scroll()
            .main_padding_units(24.0)
            .background(gpui::transparent_black());
    }
}
