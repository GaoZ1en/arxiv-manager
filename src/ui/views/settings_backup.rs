// 设置视图

use iced::widget::{
    button, column, container, row, text, text_input, scrollable, 
    vertical_space, checkbox, pick_list
};
use iced::{Element, Length, Color, Background, Border, Shadow};

use crate::core::app_state::ArxivManager;
use crate::core::models::{Theme, Language};
use crate::core::messages::Message;
use crate::ui::style::{button_primary_style, button_secondary_style, button_danger_style};
use crate::ui::theme::*;

pub struct SettingsView;

impl SettingsView {
    pub fn view(app: &ArxivManager) -> Element<Message> {
        let title = text("Settings")
            .color(GRUVBOX_TEXT)
            .size(28);

        // 外观设置
        let appearance_section = Self::create_settings_section(
            "Appearance",
            GRUVBOX_BLUE,
            vec![
                Self::create_setting_row(
                    "Theme:",
                    pick_list(
                        Theme::all_variants(),
                        Some(app.settings.theme.clone()),
                        Message::ThemeChanged,
                    )
                    .placeholder("Select theme...")
                    .style(Self::pick_list_style())
                    .into()
                ),
                Self::create_setting_row(
                    "Language:",
                    pick_list(
                        Language::all_variants(),
                        Some(app.settings.language.clone()),
                        Message::LanguageChanged,
                    )
                    .placeholder("Select language...")
                    .style(Self::pick_list_style())
                    .into()
                ),
            ]
        );

        // 下载设置
        let download_section = Self::create_settings_section(
            "Downloads",
            GRUVBOX_GREEN,
            vec![
                Self::create_setting_row(
                    "Download Directory:",
                    text_input("Path to download directory", &app.settings.download_directory)
                        .on_input(Message::DownloadDirectoryChanged)
                        .style(Self::text_input_style())
                        .into()
                ),
                Self::create_setting_row(
                    "Auto Download:",
                    checkbox("Automatically download papers when saved", app.settings.auto_download)
                        .on_toggle(|_| Message::AutoDownloadToggled)
                        .style(Self::checkbox_style())
                        .into()
                ),
                Self::create_setting_row(
                    "Max Concurrent Downloads:",
                    text_input("1-10", &app.settings.max_concurrent_downloads.to_string())
                        .on_input(Message::MaxConcurrentDownloadsChanged)
                        .style(Self::text_input_style())
                        .into()
                ),
            ]
        );

        // 快捷键设置
        let shortcuts_section = Self::create_shortcuts_section(app);

        container(
            scrollable(
                column![
                    title,
                    vertical_space().height(20),
                    appearance_section,
                    vertical_space().height(15),
                    download_section,
                    vertical_space().height(15),
                    shortcuts_section,
                ].spacing(10)
            )
        )
        .padding(20)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_BG)),
            border: Border::default(),
            text_color: Some(GRUVBOX_TEXT),
            shadow: Shadow::default(),
        })
        .into()
    }

    fn create_settings_section<'a>(title: &'a str, color: Color, items: Vec<Element<'a, Message>>) -> Element<'a, Message> {
        container(
            column![
                text(title).color(color).size(20),
                vertical_space().height(10),
                column(items).spacing(15)
            ].spacing(5)
        )
        .padding(15)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_SURFACE)),
            border: Border {
                color,
                width: 1.0,
                radius: 8.0.into(),
            },
            text_color: Some(GRUVBOX_TEXT),
            shadow: Shadow::default(),
        })
        .into()
    }

    fn create_setting_row<'a>(label: &'a str, control: Element<'a, Message>) -> Element<'a, Message> {
        row![
            text(label)
                .color(GRUVBOX_TEXT)
                .size(14)
                .width(Length::FillPortion(2)),
            container(control).width(Length::FillPortion(3))
        ]
        .spacing(15)
        .align_y(iced::Alignment::Center)
        .into()
    }

    fn create_shortcuts_section(app: &ArxivManager) -> Element<Message> {
        let shortcuts = app.settings.shortcuts.get_all_actions();
        let mut shortcut_items = Vec::new();

        for (action, description, shortcut_key) in shortcuts {
            let shortcut_display = if let Some(editing) = &app.editing_shortcut {
                if editing == action {
                    // 如果正在编辑这个快捷键，显示输入框和控制按钮
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
                            .style(Self::text_input_style()),
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
                                .style(|_theme, _status| iced::widget::button::Style {
                                    background: Some(Background::Color(GRUVBOX_BORDER)),
                                    text_color: GRUVBOX_TEXT_MUTED,
                                    border: Border {
                                        color: GRUVBOX_BORDER,
                                        width: 1.0,
                                        radius: 4.0.into(),
                                    },
                                    shadow: Shadow::default(),
                                })
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
                } else {
                    // 显示当前快捷键和编辑按钮（编辑其他快捷键时禁用）
                    row![
                        text(&shortcut_key.display)
                            .color(GRUVBOX_TEXT_MUTED)
                            .size(14),
                        button(text("编辑").size(12))
                            .style(|_theme, _status| iced::widget::button::Style {
                                background: Some(Background::Color(GRUVBOX_BORDER)),
                                text_color: GRUVBOX_TEXT_MUTED,
                                border: Border {
                                    color: GRUVBOX_BORDER,
                                    width: 1.0,
                                    radius: 4.0.into(),
                                },
                                shadow: Shadow::default(),
                            })
                            .padding([4, 8])
                    ]
                    .spacing(8)
                    .align_y(iced::Alignment::Center)
                    .into()
                }
            } else {
                // 显示当前快捷键和编辑按钮
                row![
                    text(&shortcut_key.display)
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
            };

            let shortcut_row = Self::create_setting_row(description, shortcut_display);
            shortcut_items.push(shortcut_row);
        }

        // 添加重置按钮
        let reset_button = button(
            text("重置所有快捷键")
                .color(GRUVBOX_TEXT)
                .size(14)
        )
        .on_press(Message::ResetShortcuts)
        .style(button_danger_style)
        .padding([8, 16]);

        let mut items = shortcut_items;
        items.push(
            container(reset_button)
                .width(Length::Fill)
                .align_x(iced::alignment::Horizontal::Center)
                .into()
        );

        Self::create_settings_section("Keyboard Shortcuts", GRUVBOX_PURPLE, items)
    }

    fn pick_list_style() -> impl Fn(&iced::Theme, iced::widget::pick_list::Status) -> iced::widget::pick_list::Style {
        |_theme, status| iced::widget::pick_list::Style {
            text_color: GRUVBOX_TEXT,
            background: Background::Color(GRUVBOX_BG),
            border: Border {
                color: match status {
                    iced::widget::pick_list::Status::Active => GRUVBOX_BORDER,
                    iced::widget::pick_list::Status::Hovered => GRUVBOX_GREEN,
                    iced::widget::pick_list::Status::Opened => GRUVBOX_GREEN,
                },
                width: 1.0,
                radius: 4.0.into(),
            },
            handle_color: GRUVBOX_TEXT,
            placeholder_color: GRUVBOX_TEXT_MUTED,
        }
    }

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

    fn checkbox_style() -> impl Fn(&iced::Theme, iced::widget::checkbox::Status) -> iced::widget::checkbox::Style {
        |_theme, _status| iced::widget::checkbox::Style {
            background: Background::Color(GRUVBOX_BG),
            icon_color: GRUVBOX_GREEN,
            border: Border {
                color: GRUVBOX_BORDER,
                width: 1.0,
                radius: 2.0.into(),
            },
            text_color: Some(GRUVBOX_TEXT),
        }
    }
}
