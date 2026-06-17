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
pub mod empty;
pub mod flex;
pub mod form;
pub mod heat_bar;
pub mod horizontal_list;
pub mod image;
pub mod input;
pub mod input_number;
pub mod input_tag;
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
pub mod signal_meter;
pub mod skeleton;
pub mod slider;
pub mod space;
pub mod sparkline;
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

pub use affix::*;
pub use alert::*;
pub use anchor::*;
pub use area_chart::*;
pub use aura_theme::{ButtonSize, ButtonVariant};
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
pub use empty::*;
pub use flex::*;
pub use form::*;
pub use heat_bar::*;
pub use horizontal_list::*;
pub use image::*;
pub use input::*;
pub use input_number::*;
pub use input_tag::*;
pub use label::*;
pub use line_chart::*;
pub use link::*;
pub use loading::*;
pub use mention::*;
pub use menu::*;
pub use message::*;
pub use message_box::*;
pub use motion::*;
pub use notification::*;
pub use operation::*;
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
pub use signal_meter::*;
pub use skeleton::*;
pub use slider::*;
pub use space::*;
pub use sparkline::*;
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

#[cfg(test)]
mod motion_coverage_tests {
    #[test]
    fn interactive_surfaces_use_aura_motion() {
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
    fn interactive_state_indicators_use_aura_motion() {
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
    fn popup_key_bindings_are_registered_by_apps() {
        let docs = include_str!("../../../apps/aura-docs/src/main.rs");
        let gallery = include_str!("../../../apps/aura-gallery/src/main.rs");
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
            assert!(docs.contains(&registration), "docs missing {registration}");
            assert!(
                gallery.contains(&registration),
                "gallery missing {registration}"
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
            ("tour", include_str!("tour.rs")),
            ("select", include_str!("select.rs")),
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
    }
}

#[cfg(test)]
mod visual_theme_consistency_tests {
    #[test]
    fn hardened_colored_surfaces_use_theme_inverted_text_token() {
        for (name, source) in [
            ("tag", include_str!("tag.rs")),
            ("progress", include_str!("progress.rs")),
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
}
