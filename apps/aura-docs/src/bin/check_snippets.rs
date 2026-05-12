//! Compile-check harness for all authored Rust snippets used by Aura Docs.
//!
//! This binary is not shown in the docs UI. It imports each snippet as a module
//! so `cargo check -p aura-docs --bin check_snippets` catches syntax errors,
//! missing imports, and stale public APIs in documentation examples.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[path = "../../content/snippets/about/doc_rule.rs"]
mod about_doc_rule;
#[path = "../../content/snippets/alert/description.rs"]
mod alert_description;
#[path = "../../content/snippets/alert/types.rs"]
mod alert_types;
#[path = "../../content/snippets/architecture/render_pipeline.rs"]
mod architecture_render_pipeline;
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
#[path = "../../content/snippets/badge/basic.rs"]
mod badge_basic;
#[path = "../../content/snippets/badge/dot.rs"]
mod badge_dot;
#[path = "../../content/snippets/badge/max.rs"]
mod badge_max;
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
#[path = "../../content/snippets/checkbox/basic.rs"]
mod checkbox_basic;
#[path = "../../content/snippets/checkbox/buttons.rs"]
mod checkbox_buttons;
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
#[path = "../../content/snippets/live_demo/button.rs"]
mod live_demo_button;
#[path = "../../content/snippets/markdown/state_machine.rs"]
mod markdown_state_machine;
#[path = "../../content/snippets/message/formatting.rs"]
mod message_formatting;
#[path = "../../content/snippets/message/types.rs"]
mod message_types;
#[path = "../../content/snippets/quick_start/component_view.rs"]
mod quick_start_component_view;
#[path = "../../content/snippets/quick_start/components.rs"]
mod quick_start_components;
#[path = "../../content/snippets/quick_start/init.rs"]
mod quick_start_init;
#[path = "../../content/snippets/quick_start/main_window.rs"]
mod quick_start_main_window;
#[path = "../../content/snippets/radio/basic.rs"]
mod radio_basic;
#[path = "../../content/snippets/radio/buttons.rs"]
mod radio_buttons;
#[path = "../../content/snippets/radio/group.rs"]
mod radio_group;
#[path = "../../content/snippets/rate/basic.rs"]
mod rate_basic;
#[path = "../../content/snippets/rate/custom.rs"]
mod rate_custom;
#[path = "../../content/snippets/select/basic.rs"]
mod select_basic;
#[path = "../../content/snippets/slider/basic.rs"]
mod slider_basic;
#[path = "../../content/snippets/slider/step.rs"]
mod slider_step;
#[path = "../../content/snippets/switch/basic.rs"]
mod switch_basic;
#[path = "../../content/snippets/switch/callback.rs"]
mod switch_callback;
#[path = "../../content/snippets/switch/disabled.rs"]
mod switch_disabled;
#[path = "../../content/snippets/tag/closable.rs"]
mod tag_closable;
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
#[path = "../../content/snippets/typography/paragraph.rs"]
mod typography_paragraph;

fn main() {}
