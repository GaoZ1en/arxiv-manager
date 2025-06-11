// 快捷键设置页面 - 键盘快捷键配置

use iced::widget::{button, row, text, text_input};
use iced::{Element, Background, Border, Color, Shadow};

use crate::core::app_state::ArxivManager;
use crate::core::messages::Message;
use crate::ui::style::{button_primary_style_dynamic, button_secondary_style_dynamic, button_danger_style_dynamic};
use super::components::settings_section::create_settings_section_with_colors;
use super::components::setting_row::create_setting_row;

/// 创建快捷键设置区域
pub fn create_shortcuts_section(app: &ArxivManager) -> Element<'_, Message> {
    let theme_colors = app.theme_colors();
    let shortcuts = app.settings.shortcuts.get_all_actions();
    let mut shortcut_items = Vec::new();

    for (action, description, shortcut_key) in shortcuts {
        let shortcut_display = if let Some(editing) = &app.editing_shortcut {
            if editing == action {
                // 如果正在编辑这个快捷键，显示输入框和控制按钮
                create_shortcut_edit_row(app, action)
            } else {
                // 显示当前快捷键和编辑按钮（编辑其他快捷键时禁用）
                create_shortcut_disabled_row(&shortcut_key.display, &theme_colors)
            }
        } else {
            // 显示当前快捷键和编辑按钮
            create_shortcut_normal_row(action, &shortcut_key.display, app)
        };

        let shortcut_row = create_setting_row(description, shortcut_display);
        shortcut_items.push(shortcut_row);
    }

    // 直接在这里创建重置按钮，避免生命周期问题
    use iced::widget::container;
    use iced::{Length, alignment};
    
    let reset_button_elem = button(
        text("重置所有快捷键")
            .color(theme_colors.text_primary)
            .size(14)
    )
    .on_press(Message::ResetShortcuts)
    .style(button_danger_style_dynamic(&app.settings.theme))
    .padding([8, 16]);

    let reset_container = container(reset_button_elem)
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Center);
    
    shortcut_items.push(reset_container.into());

    create_settings_section_with_colors(
        "⌨️ Keyboard Shortcuts", 
        theme_colors.button_primary, 
        shortcut_items, 
        theme_colors
    )
}

/// 创建正在编辑状态的快捷键行
fn create_shortcut_edit_row<'a>(app: &'a ArxivManager, action: &'a str) -> Element<'a, Message> {
    let input_valid = crate::core::models::ShortcutKey::is_valid_shortcut(&app.shortcut_input);
    
    row![
        text_input("例: Ctrl+K, Shift+F1, Alt+Tab", &app.shortcut_input)
            .on_input(Message::ShortcutInputChanged)
            .on_submit(if input_valid {
                Message::ShortcutChanged {
                    action: action.to_string(),
                    shortcut: app.shortcut_input.clone(),
                }
            } else {
                Message::NoOp
            })
            .style(text_input_dynamic_style(&app.settings.theme)),
        if input_valid {
            button(text("确认").size(12))
                .on_press(Message::ShortcutChanged {
                    action: action.to_string(),
                    shortcut: app.shortcut_input.clone(),
                })
                .style(button_primary_style_dynamic(&app.settings.theme))
                .padding([4, 8])
        } else {
            button(text("确认").size(12))
                .style(disabled_button_dynamic_style(&app.theme_colors()))
                .padding([4, 8])
        },
        button(text("取消").size(12))
            .on_press(Message::ShortcutEditCancelled)
            .style(button_secondary_style_dynamic(&app.settings.theme))
            .padding([4, 8])
    ]
    .spacing(8)
    .align_y(iced::Alignment::Center)
    .into()
}

/// 创建禁用状态的快捷键行（当编辑其他快捷键时）
fn create_shortcut_disabled_row<'a>(shortcut_display: &'a str, theme_colors: &crate::ui::theme::ThemeColors) -> Element<'a, Message> {
    row![
        text(shortcut_display)
            .color(theme_colors.text_muted)
            .size(14),
        button(text("编辑").size(12))
            .style(disabled_button_dynamic_style(theme_colors))
            .padding([4, 8])
    ]
    .spacing(8)
    .align_y(iced::Alignment::Center)
    .into()
}

/// 创建正常状态的快捷键行
fn create_shortcut_normal_row<'a>(action: &'a str, shortcut_display: &'a str, app: &'a ArxivManager) -> Element<'a, Message> {
    let theme_colors = app.theme_colors();
    row![
        text(shortcut_display)
            .color(theme_colors.text_primary)
            .size(14),
        button(text("编辑").size(12))
            .on_press(Message::ShortcutEditStarted(action.to_string()))
            .style(button_secondary_style_dynamic(&app.settings.theme))
            .padding([4, 8])
    ]
    .spacing(8)
    .align_y(iced::Alignment::Center)
    .into()
}

/// TextInput组件的动态样式
fn text_input_dynamic_style(theme: &crate::core::models::Theme) -> impl Fn(&iced::Theme, iced::widget::text_input::Status) -> iced::widget::text_input::Style {
    use crate::ui::theme::get_theme_colors;
    let colors = get_theme_colors(theme);
    move |_theme, status| iced::widget::text_input::Style {
        background: Background::Color(colors.dark_bg),
        border: Border {
            color: match status {
                iced::widget::text_input::Status::Focused => colors.success_color,
                _ => colors.border_color,
            },
            width: 1.0,
            radius: 4.0.into(),
        },
        icon: Color::TRANSPARENT,
        placeholder: colors.text_muted,
        value: colors.text_primary,
        selection: colors.success_color,
    }
}

/// 禁用按钮的动态样式
fn disabled_button_dynamic_style(theme_colors: &crate::ui::theme::ThemeColors) -> impl Fn(&iced::Theme, iced::widget::button::Status) -> iced::widget::button::Style {
    let colors = *theme_colors;
    move |_theme, _status| iced::widget::button::Style {
        background: Some(Background::Color(colors.border_color)),
        text_color: colors.text_muted,
        border: Border {
            color: colors.border_color,
            width: 1.0,
            radius: 4.0.into(),
        },
        shadow: Shadow::default(),
    }
}
