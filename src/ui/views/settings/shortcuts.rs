// 快捷键设置页面 - 键盘快捷键配置

use iced::widget::{button, row, text, text_input};
use iced::{Element, Background, Border, Color, Shadow};

use crate::core::app_state::ArxivManager;
use crate::core::messages::Message;
use crate::ui::style::{button_primary_style, button_secondary_style, button_danger_style};
use crate::ui::theme::*;
use super::components::settings_section::create_settings_section;
use super::components::setting_row::create_setting_row;

/// 创建快捷键设置区域
pub fn create_shortcuts_section(app: &ArxivManager) -> Element<Message> {
    let shortcuts = app.settings.shortcuts.get_all_actions();
    let mut shortcut_items = Vec::new();

    for (action, description, shortcut_key) in shortcuts {
        let shortcut_display = if let Some(editing) = &app.editing_shortcut {
            if editing == action {
                // 如果正在编辑这个快捷键，显示输入框和控制按钮
                create_shortcut_edit_row(app, action)
            } else {
                // 显示当前快捷键和编辑按钮（编辑其他快捷键时禁用）
                create_shortcut_disabled_row(&shortcut_key.display)
            }
        } else {
            // 显示当前快捷键和编辑按钮
            create_shortcut_normal_row(action, &shortcut_key.display)
        };

        let shortcut_row = create_setting_row(description, shortcut_display);
        shortcut_items.push(shortcut_row);
    }

    // 添加重置按钮
    let reset_button = create_reset_shortcuts_button();
    shortcut_items.push(reset_button);

    create_settings_section("Keyboard Shortcuts", GRUVBOX_PURPLE, shortcut_items)
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
            .style(text_input_style()),
        if input_valid {
            button(text("确认").size(12))
                .on_press(Message::ShortcutChanged {
                    action: action.to_string(),
                    shortcut: app.shortcut_input.clone(),
                })
                .style(button_primary_style)
                .padding([4, 8])
        } else {
            button(text("确认").size(12))
                .style(disabled_button_style)
                .padding([4, 8])
        },
        button(text("取消").size(12))
            .on_press(Message::ShortcutEditCancelled)
            .style(button_secondary_style)
            .padding([4, 8])
    ]
    .spacing(8)
    .align_y(iced::Alignment::Center)
    .into()
}

/// 创建禁用状态的快捷键行（当编辑其他快捷键时）
fn create_shortcut_disabled_row<'a>(shortcut_display: &'a str) -> Element<'a, Message> {
    row![
        text(shortcut_display)
            .color(GRUVBOX_TEXT_MUTED)
            .size(14),
        button(text("编辑").size(12))
            .style(disabled_button_style)
            .padding([4, 8])
    ]
    .spacing(8)
    .align_y(iced::Alignment::Center)
    .into()
}

/// 创建正常状态的快捷键行
fn create_shortcut_normal_row<'a>(action: &'a str, shortcut_display: &'a str) -> Element<'a, Message> {
    row![
        text(shortcut_display)
            .color(GRUVBOX_TEXT)
            .size(14),
        button(text("编辑").size(12))
            .on_press(Message::ShortcutEditStarted(action.to_string()))
            .style(button_secondary_style)
            .padding([4, 8])
    ]
    .spacing(8)
    .align_y(iced::Alignment::Center)
    .into()
}

/// 创建重置快捷键按钮
fn create_reset_shortcuts_button() -> Element<'static, Message> {
    use iced::widget::container;
    use iced::{Length, alignment};
    
    let reset_button = button(
        text("重置所有快捷键")
            .color(GRUVBOX_TEXT)
            .size(14)
    )
    .on_press(Message::ResetShortcuts)
    .style(button_danger_style)
    .padding([8, 16]);

    container(reset_button)
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Center)
        .into()
}

/// TextInput组件的样式
fn text_input_style() -> impl Fn(&iced::Theme, iced::widget::text_input::Status) -> iced::widget::text_input::Style {
    |_theme, status| iced::widget::text_input::Style {
        background: Background::Color(GRUVBOX_BG),
        border: Border {
            color: match status {
                iced::widget::text_input::Status::Focused => GRUVBOX_GREEN,
                _ => GRUVBOX_BORDER,
            },
            width: 1.0,
            radius: 4.0.into(),
        },
        icon: Color::TRANSPARENT,
        placeholder: GRUVBOX_TEXT_MUTED,
        value: GRUVBOX_TEXT,
        selection: GRUVBOX_GREEN,
    }
}

/// 禁用按钮的样式
fn disabled_button_style(_theme: &iced::Theme, _status: iced::widget::button::Status) -> iced::widget::button::Style {
    iced::widget::button::Style {
        background: Some(Background::Color(GRUVBOX_BORDER)),
        text_color: GRUVBOX_TEXT_MUTED,
        border: Border {
            color: GRUVBOX_BORDER,
            width: 1.0,
            radius: 4.0.into(),
        },
        shadow: Shadow::default(),
    }
}
