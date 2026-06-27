//! Compile-check harness for all authored Rust snippets used by Liora Docs.
//!
//! This binary is not shown in the docs UI. It imports each snippet as a module
//! so `cargo check -p liora-docs --bin check_snippets` catches syntax errors,
//! missing imports, and stale public APIs in documentation examples.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[path = "../../content/snippets/about/doc_rule.rs"]
mod about_doc_rule;
#[path = "../../content/snippets/accordion/basic.rs"]
mod accordion_basic;
#[path = "../../content/snippets/accordion/multiple.rs"]
mod accordion_multiple;
#[path = "../../content/snippets/accordion/states.rs"]
mod accordion_states;
#[path = "../../content/snippets/affix/bottom.rs"]
mod affix_bottom;
#[path = "../../content/snippets/affix/container.rs"]
mod affix_container;
#[path = "../../content/snippets/affix/top.rs"]
mod affix_top;
#[path = "../../content/snippets/alert/description.rs"]
mod alert_description;
#[path = "../../content/snippets/alert/types.rs"]
mod alert_types;
#[path = "../../content/snippets/anchor/basic.rs"]
mod anchor_basic;
#[path = "../../content/snippets/anchor/nested.rs"]
mod anchor_nested;
#[path = "../../content/snippets/anchor/targets.rs"]
mod anchor_targets;
#[path = "../../content/snippets/architecture/render_pipeline.rs"]
mod architecture_render_pipeline;
#[path = "../../content/snippets/area_chart/basic.rs"]
mod area_chart_basic;
#[path = "../../content/snippets/area_chart/custom.rs"]
mod area_chart_custom;
#[path = "../../content/snippets/area_chart/downsample.rs"]
mod area_chart_downsample;
#[path = "../../content/snippets/area_chart/overlay.rs"]
mod area_chart_overlay;
#[path = "../../content/snippets/area_chart/stacked.rs"]
mod area_chart_stacked;
#[path = "../../content/snippets/authoring/code_block.rs"]
mod authoring_code_block;
#[path = "../../content/snippets/autocomplete/basic.rs"]
mod autocomplete_basic;
#[path = "../../content/snippets/autocomplete/custom.rs"]
mod autocomplete_custom;
#[path = "../../content/snippets/autocomplete/disabled.rs"]
mod autocomplete_disabled;
#[path = "../../content/snippets/autocomplete/no_suffix.rs"]
mod autocomplete_no_suffix;
#[path = "../../content/snippets/avatar/content.rs"]
mod avatar_content;
#[path = "../../content/snippets/avatar/shapes.rs"]
mod avatar_shapes;
#[path = "../../content/snippets/avatar/sizes.rs"]
mod avatar_sizes;
#[path = "../../content/snippets/backtop/basic.rs"]
mod backtop_basic;
#[path = "../../content/snippets/backtop/container.rs"]
mod backtop_container;
#[path = "../../content/snippets/backtop/custom.rs"]
mod backtop_custom;
#[path = "../../content/snippets/badge/basic.rs"]
mod badge_basic;
#[path = "../../content/snippets/badge/dot.rs"]
mod badge_dot;
#[path = "../../content/snippets/badge/max.rs"]
mod badge_max;
#[path = "../../content/snippets/bar_chart/basic.rs"]
mod bar_chart_basic;
#[path = "../../content/snippets/bar_chart/custom.rs"]
mod bar_chart_custom;
#[path = "../../content/snippets/bar_chart/gradient.rs"]
mod bar_chart_gradient;
#[path = "../../content/snippets/bar_chart/grouped.rs"]
mod bar_chart_grouped;
#[path = "../../content/snippets/bar_chart/per_bar_gradient.rs"]
mod bar_chart_per_bar_gradient;
#[path = "../../content/snippets/bar_chart/stacked.rs"]
mod bar_chart_stacked;
#[path = "../../content/snippets/bar_chart/standalone.rs"]
mod bar_chart_standalone;
#[path = "../../content/snippets/bar_chart/standalone_styles.rs"]
mod bar_chart_standalone_styles;
#[path = "../../content/snippets/breadcrumb/basic.rs"]
mod breadcrumb_basic;
#[path = "../../content/snippets/breadcrumb/clickable.rs"]
mod breadcrumb_clickable;
#[path = "../../content/snippets/breadcrumb/icon.rs"]
mod breadcrumb_icon;
#[path = "../../content/snippets/breadcrumb/separator.rs"]
mod breadcrumb_separator;
#[path = "../../content/snippets/breadcrumb/separator_icon.rs"]
mod breadcrumb_separator_icon;
#[path = "../../content/snippets/button/custom_colors.rs"]
mod button_custom_colors;
#[path = "../../content/snippets/button/gradient.rs"]
mod button_gradient;
#[path = "../../content/snippets/button/rounded.rs"]
mod button_rounded;
#[path = "../../content/snippets/button/secondary.rs"]
mod button_secondary;
#[path = "../../content/snippets/button/sizes.rs"]
mod button_sizes;
#[path = "../../content/snippets/button/states.rs"]
mod button_states;
#[path = "../../content/snippets/button/text.rs"]
mod button_text;
#[path = "../../content/snippets/button/types.rs"]
mod button_types;
#[path = "../../content/snippets/calendar/events.rs"]
mod calendar_events;
#[path = "../../content/snippets/calendar/range.rs"]
mod calendar_range;
#[path = "../../content/snippets/card/basic.rs"]
mod card_basic;
#[path = "../../content/snippets/card/footer.rs"]
mod card_footer;
#[path = "../../content/snippets/carousel/autoplay.rs"]
mod carousel_autoplay;
#[path = "../../content/snippets/carousel/basic.rs"]
mod carousel_basic;
#[path = "../../content/snippets/carousel/custom.rs"]
mod carousel_custom;
#[path = "../../content/snippets/cascader/basic.rs"]
mod cascader_basic;
#[path = "../../content/snippets/cascader/disabled.rs"]
mod cascader_disabled;
#[path = "../../content/snippets/cascader/filterable.rs"]
mod cascader_filterable;
#[path = "../../content/snippets/cascader/lazy.rs"]
mod cascader_lazy;
#[path = "../../content/snippets/cascader/selected.rs"]
mod cascader_selected;
#[path = "../../content/snippets/checkbox/basic.rs"]
mod checkbox_basic;
#[path = "../../content/snippets/checkbox/buttons.rs"]
mod checkbox_buttons;
#[path = "../../content/snippets/checkbox/custom.rs"]
mod checkbox_custom;
#[path = "../../content/snippets/checkbox/group.rs"]
mod checkbox_group;
#[path = "../../content/snippets/code_block/basic.rs"]
mod code_block_basic;
#[path = "../../content/snippets/code_block/inline.rs"]
mod code_block_inline;
#[path = "../../content/snippets/code_block/language.rs"]
mod code_block_language;
#[path = "../../content/snippets/code_block/theme.rs"]
mod code_block_theme;
#[path = "../../content/snippets/code_editor/basic.rs"]
mod code_editor_basic;
#[path = "../../content/snippets/code_editor/diagnostics.rs"]
mod code_editor_diagnostics;
#[path = "../../content/snippets/collapse/accordion.rs"]
mod collapse_accordion;
#[path = "../../content/snippets/collapse/basic.rs"]
mod collapse_basic;
#[path = "../../content/snippets/color_picker/basic.rs"]
mod color_picker_basic;
#[path = "../../content/snippets/color_picker/compact.rs"]
mod color_picker_compact;
#[path = "../../content/snippets/color_picker/disabled.rs"]
mod color_picker_disabled;
#[path = "../../content/snippets/color_picker/presets.rs"]
mod color_picker_presets;
#[path = "../../content/snippets/combobox/basic.rs"]
mod combobox_basic;
#[path = "../../content/snippets/combobox/footer.rs"]
mod combobox_footer;
#[path = "../../content/snippets/combobox/grouped.rs"]
mod combobox_grouped;
#[path = "../../content/snippets/combobox/multiple.rs"]
mod combobox_multiple;
#[path = "../../content/snippets/container/divider.rs"]
mod container_divider;
#[path = "../../content/snippets/container/layout.rs"]
mod container_layout;
#[path = "../../content/snippets/container/space.rs"]
mod container_space;
#[path = "../../content/snippets/date_picker/basic.rs"]
mod date_picker_basic;
#[path = "../../content/snippets/date_picker/disabled.rs"]
mod date_picker_disabled;
#[path = "../../content/snippets/date_picker/formatted.rs"]
mod date_picker_formatted;
#[path = "../../content/snippets/date_picker/month.rs"]
mod date_picker_month;
#[path = "../../content/snippets/date_picker/month_range.rs"]
mod date_picker_month_range;
#[path = "../../content/snippets/date_picker/range.rs"]
mod date_picker_range;
#[path = "../../content/snippets/date_picker/year.rs"]
mod date_picker_year;
#[path = "../../content/snippets/date_picker/year_range.rs"]
mod date_picker_year_range;
#[path = "../../content/snippets/date_time_picker/basic.rs"]
mod date_time_picker_basic;
#[path = "../../content/snippets/date_time_picker/disabled.rs"]
mod date_time_picker_disabled;
#[path = "../../content/snippets/date_time_picker/formatted.rs"]
mod date_time_picker_formatted;
#[path = "../../content/snippets/date_time_picker/no_seconds.rs"]
mod date_time_picker_no_seconds;
#[path = "../../content/snippets/date_time_picker/range.rs"]
mod date_time_picker_range;
#[path = "../../content/snippets/date_time_picker/stepped.rs"]
mod date_time_picker_stepped;
#[path = "../../content/snippets/descriptions/basic.rs"]
mod descriptions_basic;
#[path = "../../content/snippets/descriptions/border.rs"]
mod descriptions_border;
#[path = "../../content/snippets/descriptions/vertical.rs"]
mod descriptions_vertical;
#[path = "../../content/snippets/dialog/basic.rs"]
mod dialog_basic;
#[path = "../../content/snippets/dialog/custom_content.rs"]
mod dialog_custom_content;
#[path = "../../content/snippets/dialog/manual_close.rs"]
mod dialog_manual_close;
#[path = "../../content/snippets/drawer/manual_close.rs"]
mod drawer_manual_close;
#[path = "../../content/snippets/drawer/placements.rs"]
mod drawer_placements;
#[path = "../../content/snippets/drawer/sizes.rs"]
mod drawer_sizes;
#[path = "../../content/snippets/dropdown/basic.rs"]
mod dropdown_basic;
#[path = "../../content/snippets/dropdown_button/basic.rs"]
mod dropdown_button_basic;
#[path = "../../content/snippets/dropdown_button/item_states.rs"]
mod dropdown_button_item_states;
#[path = "../../content/snippets/dropdown_button/sizes.rs"]
mod dropdown_button_sizes;
#[path = "../../content/snippets/dropdown_button/split.rs"]
mod dropdown_button_split;
#[path = "../../content/snippets/dropdown/close_strategy.rs"]
mod dropdown_close_strategy;
#[path = "../../content/snippets/dropdown/placements.rs"]
mod dropdown_placements;
#[path = "../../content/snippets/empty/basic.rs"]
mod empty_basic;
#[path = "../../content/snippets/empty/description.rs"]
mod empty_description;
#[path = "../../content/snippets/empty/extra.rs"]
mod empty_extra;
#[path = "../../content/snippets/empty/image.rs"]
mod empty_image;
#[path = "../../content/snippets/form/basic.rs"]
mod form_basic;
#[path = "../../content/snippets/form/inline.rs"]
mod form_inline;
#[path = "../../content/snippets/form/validation.rs"]
mod form_validation;
#[path = "../../content/snippets/heat_bar/events.rs"]
mod heat_bar_events;
#[path = "../../content/snippets/horizontal_list/basic.rs"]
mod horizontal_list_basic;
#[path = "../../content/snippets/horizontal_list/divider.rs"]
mod horizontal_list_divider;
#[path = "../../content/snippets/horizontal_list/draggable.rs"]
mod horizontal_list_draggable;
#[path = "../../content/snippets/icon/colors.rs"]
mod icon_colors;
#[path = "../../content/snippets/icon/lucide.rs"]
mod icon_lucide;
#[path = "../../content/snippets/icon/sizes.rs"]
mod icon_sizes;
#[path = "../../content/snippets/image/basic.rs"]
mod image_basic;
#[path = "../../content/snippets/image/fit.rs"]
mod image_fit;
#[path = "../../content/snippets/image/preview.rs"]
mod image_preview;
#[path = "../../content/snippets/image/states.rs"]
mod image_states;
#[path = "../../content/snippets/input/affix.rs"]
mod input_affix;
#[path = "../../content/snippets/input/basic.rs"]
mod input_basic;
#[path = "../../content/snippets/input_number/basic.rs"]
mod input_number_basic;
#[path = "../../content/snippets/input_number/precision.rs"]
mod input_number_precision;
#[path = "../../content/snippets/input_number/vertical.rs"]
mod input_number_vertical;
#[path = "../../content/snippets/input/password.rs"]
mod input_password;
#[path = "../../content/snippets/input/states.rs"]
mod input_states;
#[path = "../../content/snippets/input_tag/basic.rs"]
mod input_tag_basic;
#[path = "../../content/snippets/input_tag/duplicates.rs"]
mod input_tag_duplicates;
#[path = "../../content/snippets/input_tag/limited.rs"]
mod input_tag_limited;
#[path = "../../content/snippets/kbd/basic.rs"]
mod kbd_basic;
#[path = "../../content/snippets/kbd/composition.rs"]
mod kbd_composition;
#[path = "../../content/snippets/kbd/sizes.rs"]
mod kbd_sizes;
#[path = "../../content/snippets/label/basic.rs"]
mod label_basic;
#[path = "../../content/snippets/layout/divider.rs"]
mod layout_divider;
#[path = "../../content/snippets/layout/grid.rs"]
mod layout_grid;
#[path = "../../content/snippets/layout/space.rs"]
mod layout_space;
#[path = "../../content/snippets/line_chart/basic.rs"]
mod line_chart_basic;
#[path = "../../content/snippets/line_chart/custom.rs"]
mod line_chart_custom;
#[path = "../../content/snippets/line_chart/downsample.rs"]
mod line_chart_downsample;
#[path = "../../content/snippets/line_chart/empty.rs"]
mod line_chart_empty;
#[path = "../../content/snippets/line_chart/line_styles.rs"]
mod line_chart_line_styles;
#[path = "../../content/snippets/line_chart/multi.rs"]
mod line_chart_multi;
#[path = "../../content/snippets/link/icons.rs"]
mod link_icons;
#[path = "../../content/snippets/link/states.rs"]
mod link_states;
#[path = "../../content/snippets/link/underline.rs"]
mod link_underline;
#[path = "../../content/snippets/link/variants.rs"]
mod link_variants;
#[path = "../../content/snippets/live_demo/button.rs"]
mod live_demo_button;
#[path = "../../content/snippets/loading/basic.rs"]
mod loading_basic;
#[path = "../../content/snippets/loading/fullscreen.rs"]
mod loading_fullscreen;
#[path = "../../content/snippets/markdown/state_machine.rs"]
mod markdown_state_machine;
#[path = "../../content/snippets/mention/disabled.rs"]
mod mention_disabled;
#[path = "../../content/snippets/mention/issues.rs"]
mod mention_issues;
#[path = "../../content/snippets/mention/people.rs"]
mod mention_people;
#[path = "../../content/snippets/menu/collapsed.rs"]
mod menu_collapsed;
#[path = "../../content/snippets/menu/horizontal.rs"]
mod menu_horizontal;
#[path = "../../content/snippets/menu/vertical.rs"]
mod menu_vertical;
#[path = "../../content/snippets/message_box/basic.rs"]
mod message_box_basic;
#[path = "../../content/snippets/message_box/manual_close.rs"]
mod message_box_manual_close;
#[path = "../../content/snippets/message/formatting.rs"]
mod message_formatting;
#[path = "../../content/snippets/message/types.rs"]
mod message_types;
#[path = "../../content/snippets/notification/types.rs"]
mod notification_types;
#[path = "../../content/snippets/operation/basic.rs"]
mod operation_basic;
#[path = "../../content/snippets/otp_input/basic.rs"]
mod otp_input_basic;
#[path = "../../content/snippets/otp_input/interactive.rs"]
mod otp_input_interactive;
#[path = "../../content/snippets/otp_input/masked.rs"]
mod otp_input_masked;
#[path = "../../content/snippets/otp_input/states.rs"]
mod otp_input_states;
#[path = "../../content/snippets/page_header/basic.rs"]
mod page_header_basic;
#[path = "../../content/snippets/page_header/extra.rs"]
mod page_header_extra;
#[path = "../../content/snippets/page_header/full.rs"]
mod page_header_full;
#[path = "../../content/snippets/pagination/advanced.rs"]
mod pagination_advanced;
#[path = "../../content/snippets/pagination/background.rs"]
mod pagination_background;
#[path = "../../content/snippets/pagination/basic.rs"]
mod pagination_basic;
#[path = "../../content/snippets/pie_chart/basic.rs"]
mod pie_chart_basic;
#[path = "../../content/snippets/pie_chart/custom.rs"]
mod pie_chart_custom;
#[path = "../../content/snippets/popconfirm/basic.rs"]
mod popconfirm_basic;
#[path = "../../content/snippets/popconfirm/custom_text.rs"]
mod popconfirm_custom_text;
#[path = "../../content/snippets/popconfirm/placements.rs"]
mod popconfirm_placements;
#[path = "../../content/snippets/popover/basic.rs"]
mod popover_basic;
#[path = "../../content/snippets/popover/close_strategy.rs"]
mod popover_close_strategy;
#[path = "../../content/snippets/popover/placements.rs"]
mod popover_placements;
#[path = "../../content/snippets/preview/custom_trigger.rs"]
mod preview_custom_trigger;
#[path = "../../content/snippets/preview/escape.rs"]
mod preview_escape;
#[path = "../../content/snippets/preview/image_trigger.rs"]
mod preview_image_trigger;
#[path = "../../content/snippets/progress/basic.rs"]
mod progress_basic;
#[path = "../../content/snippets/progress/circle.rs"]
mod progress_circle;
#[path = "../../content/snippets/progress/circle_gradient.rs"]
mod progress_circle_gradient;
#[path = "../../content/snippets/progress/color.rs"]
mod progress_color;
#[path = "../../content/snippets/progress/custom.rs"]
mod progress_custom;
#[path = "../../content/snippets/progress/gradient_complete.rs"]
mod progress_gradient_complete;
#[path = "../../content/snippets/progress/inside.rs"]
mod progress_inside;
#[path = "../../content/snippets/progress/status.rs"]
mod progress_status;
#[path = "../../content/snippets/qr_code/basic.rs"]
mod qr_code_basic;
#[path = "../../content/snippets/qr_code/decode.rs"]
mod qr_code_decode;
#[path = "../../content/snippets/qr_code/style.rs"]
mod qr_code_style;
#[path = "../../content/snippets/quick_start/component_view.rs"]
mod quick_start_component_view;
#[path = "../../content/snippets/quick_start/components.rs"]
mod quick_start_components;
#[path = "../../content/snippets/quick_start/fonts.rs"]
mod quick_start_fonts;
#[path = "../../content/snippets/quick_start/init.rs"]
mod quick_start_init;
#[path = "../../content/snippets/quick_start/main_window.rs"]
mod quick_start_main_window;
#[path = "../../content/snippets/radio/basic.rs"]
mod radio_basic;
#[path = "../../content/snippets/radio/buttons.rs"]
mod radio_buttons;
#[path = "../../content/snippets/radio/custom.rs"]
mod radio_custom;
#[path = "../../content/snippets/radio/group.rs"]
mod radio_group;
#[path = "../../content/snippets/rate/basic.rs"]
mod rate_basic;
#[path = "../../content/snippets/rate/custom.rs"]
mod rate_custom;
#[path = "../../content/snippets/result/statuses.rs"]
mod result_statuses;
#[path = "../../content/snippets/result/success.rs"]
mod result_success;
#[path = "../../content/snippets/ring_chart/basic.rs"]
mod ring_chart_basic;
#[path = "../../content/snippets/ring_chart/custom.rs"]
mod ring_chart_custom;
#[path = "../../content/snippets/ring_chart/external.rs"]
mod ring_chart_external;
#[path = "../../content/snippets/scrollbar/basic.rs"]
mod scrollbar_basic;
#[path = "../../content/snippets/searchable_list/basic.rs"]
mod searchable_list_basic;
#[path = "../../content/snippets/searchable_list/empty.rs"]
mod searchable_list_empty;
#[path = "../../content/snippets/searchable_list/filtered.rs"]
mod searchable_list_filtered;
#[path = "../../content/snippets/segment_ratio_bar/both.rs"]
mod segment_ratio_bar_both;
#[path = "../../content/snippets/segment_ratio_bar/bottom.rs"]
mod segment_ratio_bar_bottom;
#[path = "../../content/snippets/segment_ratio_bar/compact.rs"]
mod segment_ratio_bar_compact;
#[path = "../../content/snippets/segment_ratio_bar/hidden.rs"]
mod segment_ratio_bar_hidden;
#[path = "../../content/snippets/segment_ratio_bar/pattern.rs"]
mod segment_ratio_bar_pattern;
#[path = "../../content/snippets/segment_ratio_bar/top.rs"]
mod segment_ratio_bar_top;
#[path = "../../content/snippets/segmented/basic.rs"]
mod segmented_basic;
#[path = "../../content/snippets/segmented/block.rs"]
mod segmented_block;
#[path = "../../content/snippets/segmented/disabled.rs"]
mod segmented_disabled;
#[path = "../../content/snippets/select/basic.rs"]
mod select_basic;
#[path = "../../content/snippets/sheet/controlled.rs"]
mod sheet_controlled;
#[path = "../../content/snippets/sheet/placements.rs"]
mod sheet_placements;
#[path = "../../content/snippets/shell/basic.rs"]
mod shell_basic;
#[path = "../../content/snippets/shell/content_first.rs"]
mod shell_content_first;
#[path = "../../content/snippets/shell/full_product.rs"]
mod shell_full_product;
#[path = "../../content/snippets/shell/minimal.rs"]
mod shell_minimal;
#[path = "../../content/snippets/sidebar/basic.rs"]
mod sidebar_basic;
#[path = "../../content/snippets/sidebar/brand.rs"]
mod sidebar_brand;
#[path = "../../content/snippets/sidebar/custom_slots.rs"]
mod sidebar_custom_slots;
#[path = "../../content/snippets/sidebar/icon_rail.rs"]
mod sidebar_icon_rail;
#[path = "../../content/snippets/sidebar/inspector.rs"]
mod sidebar_inspector;
#[path = "../../content/snippets/sidebar/scrollable.rs"]
mod sidebar_scrollable;
#[path = "../../content/snippets/signal_meter/levels.rs"]
mod signal_meter_levels;
#[path = "../../content/snippets/signal_meter/mobile.rs"]
mod signal_meter_mobile;
#[path = "../../content/snippets/signal_meter/threshold_colors.rs"]
mod signal_meter_threshold_colors;
#[path = "../../content/snippets/signal_meter/wifi.rs"]
mod signal_meter_wifi;
#[path = "../../content/snippets/skeleton/basic.rs"]
mod skeleton_basic;
#[path = "../../content/snippets/skeleton/template.rs"]
mod skeleton_template;
#[path = "../../content/snippets/skeleton/variants.rs"]
mod skeleton_variants;
#[path = "../../content/snippets/slider/basic.rs"]
mod slider_basic;
#[path = "../../content/snippets/slider/step.rs"]
mod slider_step;
#[path = "../../content/snippets/sparkline/area.rs"]
mod sparkline_area;
#[path = "../../content/snippets/sparkline/basic.rs"]
mod sparkline_basic;
#[path = "../../content/snippets/sparkline/cards.rs"]
mod sparkline_cards;
#[path = "../../content/snippets/sparkline/downsample.rs"]
mod sparkline_downsample;
#[path = "../../content/snippets/sparkline/styles.rs"]
mod sparkline_styles;
#[path = "../../content/snippets/spinner/basic.rs"]
mod spinner_basic;
#[path = "../../content/snippets/spinner/colors.rs"]
mod spinner_colors;
#[path = "../../content/snippets/spinner/composition.rs"]
mod spinner_composition;
#[path = "../../content/snippets/spinner/sizes.rs"]
mod spinner_sizes;
#[path = "../../content/snippets/splitter/basic.rs"]
mod splitter_basic;
#[path = "../../content/snippets/statistic/affix.rs"]
mod statistic_affix;
#[path = "../../content/snippets/statistic/basic.rs"]
mod statistic_basic;
#[path = "../../content/snippets/statistic/icons.rs"]
mod statistic_icons;
#[path = "../../content/snippets/statistic/layout.rs"]
mod statistic_layout;
#[path = "../../content/snippets/status_bar/custom.rs"]
mod status_bar_custom;
#[path = "../../content/snippets/status_bar/shell.rs"]
mod status_bar_shell;
#[path = "../../content/snippets/status_bar/tones.rs"]
mod status_bar_tones;
#[path = "../../content/snippets/steps/basic.rs"]
mod steps_basic;
#[path = "../../content/snippets/steps/description.rs"]
mod steps_description;
#[path = "../../content/snippets/steps/status.rs"]
mod steps_status;
#[path = "../../content/snippets/steps/vertical.rs"]
mod steps_vertical;
#[path = "../../content/snippets/switch/basic.rs"]
mod switch_basic;
#[path = "../../content/snippets/switch/callback.rs"]
mod switch_callback;
#[path = "../../content/snippets/switch/disabled.rs"]
mod switch_disabled;
#[path = "../../content/snippets/table/basic.rs"]
mod table_basic;
#[path = "../../content/snippets/table/empty.rs"]
mod table_empty;
#[path = "../../content/snippets/table/fixed_header.rs"]
mod table_fixed_header;
#[path = "../../content/snippets/table/loading.rs"]
mod table_loading;
#[path = "../../content/snippets/table/sortable.rs"]
mod table_sortable;
#[path = "../../content/snippets/table/stripe_border.rs"]
mod table_stripe_border;
#[path = "../../content/snippets/tabs/basic.rs"]
mod tabs_basic;
#[path = "../../content/snippets/tabs/border_card.rs"]
mod tabs_border_card;
#[path = "../../content/snippets/tabs/card.rs"]
mod tabs_card;
#[path = "../../content/snippets/tabs/editable.rs"]
mod tabs_editable;
#[path = "../../content/snippets/tabs/position.rs"]
mod tabs_position;
#[path = "../../content/snippets/tabs/stretch.rs"]
mod tabs_stretch;
#[path = "../../content/snippets/tag/closable.rs"]
mod tag_closable;
#[path = "../../content/snippets/tag/flow.rs"]
mod tag_flow;
#[path = "../../content/snippets/tag/round.rs"]
mod tag_round;
#[path = "../../content/snippets/tag/sizes.rs"]
mod tag_sizes;
#[path = "../../content/snippets/tag/themes.rs"]
mod tag_themes;
#[path = "../../content/snippets/tag/types.rs"]
mod tag_types;
#[path = "../../content/snippets/textarea/basic.rs"]
mod textarea_basic;
#[path = "../../content/snippets/textarea/limit.rs"]
mod textarea_limit;
#[path = "../../content/snippets/theme/system_mode.rs"]
mod theme_system_mode;
#[path = "../../content/snippets/time_picker/basic.rs"]
mod time_picker_basic;
#[path = "../../content/snippets/time_picker/disabled.rs"]
mod time_picker_disabled;
#[path = "../../content/snippets/time_picker/formatted.rs"]
mod time_picker_formatted;
#[path = "../../content/snippets/time_picker/no_seconds.rs"]
mod time_picker_no_seconds;
#[path = "../../content/snippets/time_picker/stepped.rs"]
mod time_picker_stepped;
#[path = "../../content/snippets/timeline/basic.rs"]
mod timeline_basic;
#[path = "../../content/snippets/timeline/custom.rs"]
mod timeline_custom;
#[path = "../../content/snippets/timeline/placement.rs"]
mod timeline_placement;
#[path = "../../content/snippets/timeline/reverse.rs"]
mod timeline_reverse;
#[path = "../../content/snippets/timer/clock.rs"]
mod timer_clock;
#[path = "../../content/snippets/timer/count_down.rs"]
mod timer_count_down;
#[path = "../../content/snippets/timer/count_up.rs"]
mod timer_count_up;
#[path = "../../content/snippets/timer/result.rs"]
mod timer_result;
#[path = "../../content/snippets/timer/units.rs"]
mod timer_units;
#[path = "../../content/snippets/titlebar/basic.rs"]
mod titlebar_basic;
#[path = "../../content/snippets/titlebar/borderless.rs"]
mod titlebar_borderless;
#[path = "../../content/snippets/titlebar/command_center.rs"]
mod titlebar_command_center;
#[path = "../../content/snippets/titlebar/window_controls.rs"]
mod titlebar_window_controls;
#[path = "../../content/snippets/titlebar/window_controls_left.rs"]
mod titlebar_window_controls_left;
#[path = "../../content/snippets/titlebar/window_controls_right.rs"]
mod titlebar_window_controls_right;
#[path = "../../content/snippets/tooltip/basic.rs"]
mod tooltip_basic;
#[path = "../../content/snippets/tooltip/more.rs"]
mod tooltip_more;
#[path = "../../content/snippets/tour/basic.rs"]
mod tour_basic;
#[path = "../../content/snippets/tour/close_policy.rs"]
mod tour_close_policy;
#[path = "../../content/snippets/tour/middle.rs"]
mod tour_middle;
#[path = "../../content/snippets/tour/no_mask.rs"]
mod tour_no_mask;
#[path = "../../content/snippets/transfer/basic.rs"]
mod transfer_basic;
#[path = "../../content/snippets/transfer/disabled.rs"]
mod transfer_disabled;
#[path = "../../content/snippets/transfer/filterable.rs"]
mod transfer_filterable;
#[path = "../../content/snippets/tray/basic.rs"]
mod tray_basic;
#[path = "../../content/snippets/tray/checkbox.rs"]
mod tray_checkbox;
#[path = "../../content/snippets/tray/close_confirm.rs"]
mod tray_close_confirm;
#[path = "../../content/snippets/tray/dynamic_icon.rs"]
mod tray_dynamic_icon;
#[path = "../../content/snippets/tray/nested_menu.rs"]
mod tray_nested_menu;
#[path = "../../content/snippets/tray/residency.rs"]
mod tray_residency;
#[path = "../../content/snippets/tree/basic.rs"]
mod tree_basic;
#[path = "../../content/snippets/tree/checkable.rs"]
mod tree_checkable;
#[path = "../../content/snippets/tree_select/filterable.rs"]
mod tree_select_filterable;
#[path = "../../content/snippets/tree_select/multiple.rs"]
mod tree_select_multiple;
#[path = "../../content/snippets/tree_select/single.rs"]
mod tree_select_single;
#[path = "../../content/snippets/typography/paragraph.rs"]
mod typography_paragraph;
#[path = "../../content/snippets/upload/basic.rs"]
mod upload_basic;
#[path = "../../content/snippets/upload/drag.rs"]
mod upload_drag;
#[path = "../../content/snippets/upload/limits.rs"]
mod upload_limits;
#[path = "../../content/snippets/upload/picture_card.rs"]
mod upload_picture_card;
#[path = "../../content/snippets/virtualized_list/basic.rs"]
mod virtualized_list_basic;
#[path = "../../content/snippets/virtualized_list/draggable.rs"]
mod virtualized_list_draggable;
#[path = "../../content/snippets/virtualized_table/basic.rs"]
mod virtualized_table_basic;
#[path = "../../content/snippets/virtualized_table/sortable.rs"]
mod virtualized_table_sortable;
#[path = "../../content/snippets/virtualized_tree/basic.rs"]
mod virtualized_tree_basic;
#[path = "../../content/snippets/virtualized_tree/checkable.rs"]
mod virtualized_tree_checkable;
#[path = "../../content/snippets/watermark/cover.rs"]
mod watermark_cover;
#[path = "../../content/snippets/watermark/custom.rs"]
mod watermark_custom;
#[path = "../../content/snippets/watermark/header.rs"]
mod watermark_header;

fn main() {}
