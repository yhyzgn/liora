pub mod affix_demo;
pub mod alert_demo;
pub mod anchor_demo;
pub mod autocomplete_demo;
pub mod avatar_demo;
pub mod backtop_demo;
pub mod badge_demo;
pub mod breadcrumb_demo;
pub mod button_demo;
pub mod card_demo;
pub mod cascader_demo;
pub mod collapse_demo;
pub mod color_picker_demo;
pub mod container_demo;
pub mod date_picker_demo;
pub mod date_time_picker_demo;
pub mod descriptions_demo;
pub mod dialog_demo;
pub mod drawer_demo;
pub mod dropdown_demo;
pub mod empty_demo;
pub mod form_controls_demo;
pub mod form_demo;
pub mod icon_demo;
pub mod image_demo;
pub mod layout_demo;
pub mod link_demo;
pub mod loading_demo;
pub mod menu_demo;
pub mod message_box_demo;
pub mod message_demo;
pub mod notification_demo;
pub mod page_header_demo;
pub mod pagination_demo;
pub mod popconfirm_demo;
pub mod popover_demo;
pub mod preview_demo;
pub mod progress_demo;
pub mod result_demo;
pub mod scrollbar_demo;
pub mod segmented_demo;
pub mod skeleton_demo;
pub mod splitter_demo;
pub mod statistic_demo;
pub mod steps_demo;
pub mod table_demo;
pub mod tabs_demo;
pub mod tag_demo;
pub mod time_picker_demo;
pub mod timeline_demo;
pub mod tooltip_demo;
pub mod transfer_demo;
pub mod tree_demo;
pub mod typography_demo;
pub mod upload_demo;

use gpui::{AnyView, App};

#[derive(Clone, Copy)]
pub struct DemoEntry {
    pub name: &'static str,
    pub description: &'static str,
    pub render: fn(&mut App) -> AnyView,
}

pub fn registry() -> Vec<DemoEntry> {
    let mut entries = vec![
        DemoEntry {
            name: "Button 按钮",
            description: "常用的操作按钮",
            render: |cx| button_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Tag 标签",
            description: "用于标记和选择",
            render: |cx| tag_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Autocomplete 自动补全",
            description: "输入建议与快捷选择",
            render: |cx| autocomplete_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Avatar 头像",
            description: "展示用户或事物",
            render: |cx| avatar_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Badge 徽章",
            description: "右上角的提示信息",
            render: |cx| badge_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Input 输入框",
            description: "独立输入框用法",
            render: |cx| form_controls_demo::render_input(cx),
        },
        DemoEntry {
            name: "InputNumber 数字输入",
            description: "独立数字输入用法",
            render: |cx| form_controls_demo::render_input_number(cx),
        },
        DemoEntry {
            name: "Textarea 文本域",
            description: "独立多行输入用法",
            render: |cx| form_controls_demo::render_textarea(cx),
        },
        DemoEntry {
            name: "Checkbox 多选",
            description: "独立多选与多选组用法",
            render: |cx| form_controls_demo::render_checkbox(cx),
        },
        DemoEntry {
            name: "Radio 单选",
            description: "独立单选与单选组用法",
            render: |cx| form_controls_demo::render_radio(cx),
        },
        DemoEntry {
            name: "Switch 开关",
            description: "独立开关用法",
            render: |cx| form_controls_demo::render_switch(cx),
        },
        DemoEntry {
            name: "Select 选择器",
            description: "独立下拉选择用法",
            render: |cx| form_controls_demo::render_select(cx),
        },
        DemoEntry {
            name: "Slider 滑块",
            description: "独立滑块用法",
            render: |cx| form_controls_demo::render_slider(cx),
        },
        DemoEntry {
            name: "Rate 评分",
            description: "独立评分用法",
            render: |cx| form_controls_demo::render_rate(cx),
        },
        DemoEntry {
            name: "Form 表单",
            description: "输入框、单选、多选、开关",
            render: |cx| form_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Tooltip 文字提示",
            description: "简单的文字提示",
            render: |cx| tooltip_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Popover 气泡卡片",
            description: "点击弹出的卡片容器",
            render: |cx| popover_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Popconfirm 气泡确认框",
            description: "简单的确认操作",
            render: |cx| popconfirm_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Dialog 对话框",
            description: "模态对话框",
            render: |cx| dialog_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Drawer 抽屉",
            description: "屏幕边缘滑出的浮层面板",
            render: |cx| drawer_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Message 全局提示",
            description: "常用于操作后的反馈提示",
            render: |cx| message_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Notification 通知",
            description: "悬浮在页面角落的通知",
            render: |cx| notification_demo::render(cx).into(),
        },
        DemoEntry {
            name: "PageHeader 页头",
            description: "标识页面内容并提供操作",
            render: |cx| page_header_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Progress 进度条",
            description: "展示操作进度",
            render: |cx| progress_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Skeleton 骨架屏",
            description: "数据加载时的占位展示",
            render: |cx| skeleton_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Result 结果页",
            description: "反馈操作结果",
            render: |cx| result_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Descriptions 描述列表",
            description: "展示多个字段",
            render: |cx| descriptions_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Table 表格",
            description: "展示多条结构化数据",
            render: |cx| table_demo::render(cx).into(),
        },
        DemoEntry {
            name: "DatePicker 日期选择器",
            description: "选择单个日期",
            render: |cx| date_picker_demo::render(cx).into(),
        },
        DemoEntry {
            name: "TimePicker 时间选择器",
            description: "选择固定步进时间",
            render: |cx| time_picker_demo::render(cx).into(),
        },
        DemoEntry {
            name: "DateTimePicker 日期时间选择器",
            description: "选择日期时间和日期时间范围",
            render: |cx| date_time_picker_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Upload 上传",
            description: "上传入口与文件列表",
            render: |cx| upload_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Cascader 级联选择器",
            description: "多级联动选择",
            render: |cx| cascader_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Transfer 穿梭框",
            description: "在两个列表之间移动条目",
            render: |cx| transfer_demo::render(cx).into(),
        },
        DemoEntry {
            name: "ColorPicker 颜色选择器",
            description: "从预设色板中选择颜色",
            render: |cx| color_picker_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Image 图片",
            description: "图片容器与加载状态",
            render: |cx| image_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Preview 预览",
            description: "图片预览弹层",
            render: |cx| preview_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Timeline 时间线",
            description: "垂直展示一系列信息",
            render: |cx| timeline_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Tree 树形控件",
            description: "分层展示数据",
            render: |cx| tree_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Empty 空状态",
            description: "页面无数据时的占位提示",
            render: |cx| empty_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Alert 警告",
            description: "页面展示的重要提示信息",
            render: |cx| alert_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Affix 固钉",
            description: "将内容固定在特定可视区域",
            render: |cx| affix_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Backtop 回到顶部",
            description: "返回页面顶部的快捷按钮",
            render: |cx| backtop_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Anchor 锚点",
            description: "长页面快速跳转与滚动同步",
            render: |cx| anchor_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Breadcrumb 面包屑",
            description: "显示当前页面的路径",
            render: |cx| breadcrumb_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Steps 步骤条",
            description: "引导用户完成任务",
            render: |cx| steps_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Menu 导航菜单",
            description: "为网站提供导航轮廓",
            render: |cx| menu_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Tabs 标签页",
            description: "在同一区域展示多个面板",
            render: |cx| tabs_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Pagination 分页",
            description: "分页控制",
            render: |cx| pagination_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Statistic 统计数值",
            description: "展示强调的数值",
            render: |cx| statistic_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Segmented 分段控制器",
            description: "展示多个选项并进行单选",
            render: |cx| segmented_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Loading 加载",
            description: "加载数据时显示",
            render: |cx| loading_demo::render(cx).into(),
        },
        DemoEntry {
            name: "MessageBox 弹窗消息",
            description: "简单的消息对话框",
            render: |cx| message_box_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Card 卡片",
            description: "内容聚合容器",
            render: |cx| card_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Collapse 折叠面板",
            description: "内容收纳容器",
            render: |cx| collapse_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Dropdown 下拉菜单",
            description: "操作列表容器",
            render: |cx| dropdown_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Link 链接",
            description: "文字超链接",
            render: |cx| link_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Typography 排版",
            description: "标题、段落、文本",
            render: |cx| typography_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Layout 布局",
            description: "分割线、间距、栅格",
            render: |cx| layout_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Container 容器",
            description: "页面框架布局",
            render: |cx| container_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Scrollbar 滚动条",
            description: "原生滚动容器",
            render: |cx| scrollbar_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Splitter 分隔面板",
            description: "左右面板分割",
            render: |cx| splitter_demo::render(cx).into(),
        },
        DemoEntry {
            name: "Icon 图标",
            description: "基于 Lucide 的图标系统",
            render: |cx| icon_demo::render(cx).into(),
        },
    ];

    entries.sort_by(|a, b| a.name.cmp(b.name));
    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_entries_are_sorted_by_component_name() {
        let entries = registry();
        let names = entries.iter().map(|entry| entry.name).collect::<Vec<_>>();
        let mut sorted = names.clone();
        sorted.sort();

        assert_eq!(names, sorted);
    }

    #[test]
    fn layout_helpers_live_in_component_crate() {
        let registry_source = include_str!("mod.rs");
        let component_lib_source = include_str!("../../../../crates/aura-components/src/lib.rs");

        assert!(
            component_lib_source.contains("pub mod layout_helpers;"),
            "shared page/section layout helpers should be exported by aura-components::layout_helpers"
        );
        assert!(
            !registry_source
                .lines()
                .any(|line| line.trim() == concat!("pub mod ", "common;"))
                && !component_lib_source.contains("pub mod demo;"),
            "helpers should not remain in gallery-local common or crate-level demo modules once they are crate-owned"
        );
    }

    fn assert_demo_uses_aura_layout_primitives(file_name: &str, source: &str) {
        for forbidden in ["div(", "px(", ".flex()", ".flex_col()", ".flex_row()"] {
            assert!(
                !source.contains(forbidden),
                "{file_name} still contains forbidden GPUI primitive `{forbidden}`"
            );
        }
    }

    #[test]
    fn button_demo_uses_aura_layout_primitives() {
        assert_demo_uses_aura_layout_primitives("button_demo.rs", include_str!("button_demo.rs"));
    }

    #[test]
    fn link_demo_uses_aura_layout_primitives() {
        assert_demo_uses_aura_layout_primitives("link_demo.rs", include_str!("link_demo.rs"));
    }

    #[test]
    fn feedback_demos_use_aura_layout_primitives() {
        for (file_name, source) in [
            ("dropdown_demo.rs", include_str!("dropdown_demo.rs")),
            ("loading_demo.rs", include_str!("loading_demo.rs")),
            ("message_box_demo.rs", include_str!("message_box_demo.rs")),
            ("message_demo.rs", include_str!("message_demo.rs")),
            ("notification_demo.rs", include_str!("notification_demo.rs")),
        ] {
            assert_demo_uses_aura_layout_primitives(file_name, source);
        }
    }

    #[test]
    fn display_demos_use_aura_layout_primitives() {
        for (file_name, source) in [
            ("alert_demo.rs", include_str!("alert_demo.rs")),
            ("empty_demo.rs", include_str!("empty_demo.rs")),
            ("result_demo.rs", include_str!("result_demo.rs")),
            ("segmented_demo.rs", include_str!("segmented_demo.rs")),
            ("statistic_demo.rs", include_str!("statistic_demo.rs")),
            ("tree_demo.rs", include_str!("tree_demo.rs")),
        ] {
            assert_demo_uses_aura_layout_primitives(file_name, source);
        }
    }

    #[test]
    fn interaction_demos_use_aura_layout_primitives() {
        for (file_name, source) in [
            ("pagination_demo.rs", include_str!("pagination_demo.rs")),
            ("popconfirm_demo.rs", include_str!("popconfirm_demo.rs")),
            ("tooltip_demo.rs", include_str!("tooltip_demo.rs")),
        ] {
            assert_demo_uses_aura_layout_primitives(file_name, source);
        }
    }

    #[test]
    fn typography_and_progress_demos_use_aura_layout_primitives() {
        for (file_name, source) in [
            ("progress_demo.rs", include_str!("progress_demo.rs")),
            ("typography_demo.rs", include_str!("typography_demo.rs")),
        ] {
            assert_demo_uses_aura_layout_primitives(file_name, source);
        }
    }

    #[test]
    fn navigation_demos_use_aura_layout_primitives() {
        for (file_name, source) in [
            ("breadcrumb_demo.rs", include_str!("breadcrumb_demo.rs")),
            ("collapse_demo.rs", include_str!("collapse_demo.rs")),
            ("menu_demo.rs", include_str!("menu_demo.rs")),
            ("steps_demo.rs", include_str!("steps_demo.rs")),
        ] {
            assert_demo_uses_aura_layout_primitives(file_name, source);
        }
    }

    #[test]
    fn menu_demo_keeps_vertical_menu_compact() {
        let source = include_str!("menu_demo.rs");

        assert!(
            source.contains(
                r#"Col::new(4).child(
                                        Space::new()
                                            .vertical()
                                            .gap_md()
                                            .child(Text::new("垂直模式").bold())"#
            ),
            "vertical menu demo should use a narrower 4/24 grid column instead of the previous 6/24 layout"
        );
        assert!(
            source.contains(
                r#"Col::new(2).child(
                                        Space::new()
                                            .vertical()
                                            .gap_md()
                                            .child(Text::new("折叠").bold())"#
            ),
            "collapsed menu demo should use a compact 2/24 grid column instead of the regular menu width"
        );
    }

    #[test]
    fn input_picker_demos_use_aura_layout_primitives() {
        for (file_name, source) in [
            ("color_picker_demo.rs", include_str!("color_picker_demo.rs")),
            ("date_picker_demo.rs", include_str!("date_picker_demo.rs")),
            (
                "date_time_picker_demo.rs",
                include_str!("date_time_picker_demo.rs"),
            ),
            ("time_picker_demo.rs", include_str!("time_picker_demo.rs")),
            ("upload_demo.rs", include_str!("upload_demo.rs")),
        ] {
            assert_demo_uses_aura_layout_primitives(file_name, source);
        }
    }

    #[test]
    fn data_display_demos_use_aura_layout_primitives() {
        for (file_name, source) in [
            ("avatar_demo.rs", include_str!("avatar_demo.rs")),
            ("badge_demo.rs", include_str!("badge_demo.rs")),
            ("descriptions_demo.rs", include_str!("descriptions_demo.rs")),
            ("timeline_demo.rs", include_str!("timeline_demo.rs")),
        ] {
            assert_demo_uses_aura_layout_primitives(file_name, source);
        }
    }

    #[test]
    fn layout_container_demos_use_aura_layout_primitives() {
        for (file_name, source) in [
            ("card_demo.rs", include_str!("card_demo.rs")),
            ("container_demo.rs", include_str!("container_demo.rs")),
            ("layout_demo.rs", include_str!("layout_demo.rs")),
            ("scrollbar_demo.rs", include_str!("scrollbar_demo.rs")),
            ("splitter_demo.rs", include_str!("splitter_demo.rs")),
        ] {
            assert_demo_uses_aura_layout_primitives(file_name, source);
        }
    }

    #[test]
    fn selection_demos_use_aura_layout_primitives() {
        for (file_name, source) in [
            ("autocomplete_demo.rs", include_str!("autocomplete_demo.rs")),
            ("cascader_demo.rs", include_str!("cascader_demo.rs")),
            ("transfer_demo.rs", include_str!("transfer_demo.rs")),
        ] {
            assert_demo_uses_aura_layout_primitives(file_name, source);
        }
    }

    #[test]
    fn overlay_demos_use_aura_layout_primitives() {
        for (file_name, source) in [
            ("dialog_demo.rs", include_str!("dialog_demo.rs")),
            ("drawer_demo.rs", include_str!("drawer_demo.rs")),
            ("popover_demo.rs", include_str!("popover_demo.rs")),
            ("page_header_demo.rs", include_str!("page_header_demo.rs")),
        ] {
            assert_demo_uses_aura_layout_primitives(file_name, source);
        }
    }

    #[test]
    fn tag_and_tabs_demos_use_aura_layout_primitives() {
        for (file_name, source) in [
            ("tag_demo.rs", include_str!("tag_demo.rs")),
            ("tabs_demo.rs", include_str!("tabs_demo.rs")),
        ] {
            assert_demo_uses_aura_layout_primitives(file_name, source);
        }
    }

    #[test]
    fn icon_demo_uses_aura_layout_primitives() {
        assert_demo_uses_aura_layout_primitives("icon_demo.rs", include_str!("icon_demo.rs"));
    }

    #[test]
    fn icon_demo_labels_are_center_aligned_under_icons() {
        let source = include_str!("icon_demo.rs");

        assert!(
            source.contains(".align_center()"),
            "icon labels should use Space::align_center so text is centered under each icon"
        );
    }

    #[test]
    fn skeleton_demo_uses_aura_layout_primitives() {
        assert_demo_uses_aura_layout_primitives(
            "skeleton_demo.rs",
            include_str!("skeleton_demo.rs"),
        );
    }

    #[test]
    fn preview_demo_uses_aura_layout_primitives() {
        assert_demo_uses_aura_layout_primitives("preview_demo.rs", include_str!("preview_demo.rs"));
    }

    #[test]
    fn image_demo_uses_aura_layout_primitives() {
        assert_demo_uses_aura_layout_primitives("image_demo.rs", include_str!("image_demo.rs"));
    }

    #[test]
    fn image_and_preview_demos_keep_remote_loading_coverage_bounded() {
        for (file_name, source) in [
            ("image_demo.rs", include_str!("image_demo.rs")),
            ("preview_demo.rs", include_str!("preview_demo.rs")),
        ] {
            let remote_count = source.matches("https://").count();
            assert_eq!(
                remote_count, 1,
                "{file_name} should keep exactly one remote image for remote-loading coverage without triggering many network loads"
            );
        }
    }

    #[test]
    fn form_demos_use_aura_layout_primitives() {
        for (file_name, source) in [
            (
                "form_controls_demo.rs",
                include_str!("form_controls_demo.rs"),
            ),
            ("form_demo.rs", include_str!("form_demo.rs")),
        ] {
            assert_demo_uses_aura_layout_primitives(file_name, source);
        }
    }

    #[test]
    fn complex_scroll_demos_use_aura_layout_primitives() {
        for (file_name, source) in [
            ("affix_demo.rs", include_str!("affix_demo.rs")),
            ("anchor_demo.rs", include_str!("anchor_demo.rs")),
            ("backtop_demo.rs", include_str!("backtop_demo.rs")),
        ] {
            assert_demo_uses_aura_layout_primitives(file_name, source);
        }
    }

    #[test]
    fn table_demo_uses_aura_layout_primitives() {
        assert_demo_uses_aura_layout_primitives("table_demo.rs", include_str!("table_demo.rs"));
    }

    #[test]
    fn tag_dynamic_input_uses_compact_input_width() {
        let source = include_str!("tag_demo.rs");

        assert!(
            source.contains(r#"Input::new("", cx).width_sm()"#),
            "dynamic tag input should use compact input width instead of a medium card wrapper"
        );
        assert!(
            !source.contains("Card::new(self.input.clone())"),
            "dynamic tag input should not be wrapped in a Card just to set width"
        );
    }

    #[test]
    fn tabs_demo_scrolls_with_natural_tab_height() {
        let source = include_str!("../../../../crates/aura-components/src/tabs.rs");
        let production = source.split("#[cfg(test)]").next().unwrap();

        assert!(
            !production
                .contains(".w_full()\n            .h_full()\n            .when(!is_vertical"),
            "Tabs root should not force full height inside scrollable demo pages"
        );
    }
}
