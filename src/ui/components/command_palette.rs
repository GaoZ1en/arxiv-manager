// 命令面板组件

use iced::widget::{button, column, container, row, text, text_input, scrollable, horizontal_space, vertical_space};
use iced::{Element, Length, Color, Background, Border, Shadow};

use crate::core::app_state::ArxivManager;
use crate::core::messages::Message;
use crate::ui::theme::*;

pub struct CommandPalette;

impl CommandPalette {
    pub fn view(app: &ArxivManager) -> Element<Message> {
        // 命令栏主体
        let command_input = text_input("Type a command...", &app.command_palette_input)
            .on_input(Message::CommandPaletteInputChanged)
            .on_submit(if let Some(index) = app.selected_command_index {
                if let Some(command) = app.command_suggestions.get(index) {
                    Message::ExecuteCommand(command.clone())
                } else {
                    Message::ClearCommandPalette
                }
            } else {
                Message::ClearCommandPalette
            })
            .size(18)
            .style(|_theme, status| iced::widget::text_input::Style {
                background: Background::Color(GRUVBOX_SURFACE),
                border: Border {
                    color: match status {
                        iced::widget::text_input::Status::Focused => GRUVBOX_GREEN,
                        _ => GRUVBOX_BORDER,
                    },
                    width: 2.0,
                    radius: 6.0.into(),
                },
                icon: Color::TRANSPARENT,
                placeholder: GRUVBOX_TEXT_MUTED,
                value: GRUVBOX_TEXT,
                selection: GRUVBOX_GREEN,
            });

        // 命令建议列表
        let suggestions_list = if !app.command_suggestions.is_empty() {
            let suggestions = column(
                app.command_suggestions.iter().enumerate().map(|(index, command)| {
                    let is_selected = app.selected_command_index == Some(index);
                    
                    let command_button = button(
                        row![
                            text(command.display_name())
                                .color(if is_selected { Color::BLACK } else { GRUVBOX_TEXT })
                                .size(14),
                            horizontal_space(),
                            text(command.keywords().join(" "))
                                .color(if is_selected { Color::from_rgb(0.3, 0.3, 0.3) } else { GRUVBOX_TEXT_MUTED })
                                .size(12),
                        ]
                        .padding([8, 12])
                        .width(Length::Fill)
                    )
                    .on_press(Message::ExecuteCommand(command.clone()))
                    .width(Length::Fill)
                    .style(move |_theme, status| {
                        let base_color = if is_selected {
                            GRUVBOX_GREEN
                        } else {
                            GRUVBOX_SURFACE
                        };
                        
                        iced::widget::button::Style {
                            background: Some(Background::Color(match status {
                                iced::widget::button::Status::Hovered => {
                                    if is_selected {
                                        Color::from_rgb(
                                            GRUVBOX_GREEN.r * 0.9,
                                            GRUVBOX_GREEN.g * 0.9,
                                            GRUVBOX_GREEN.b * 0.9,
                                        )
                                    } else {
                                        Color::from_rgb(
                                            GRUVBOX_SURFACE.r * 1.1,
                                            GRUVBOX_SURFACE.g * 1.1,
                                            GRUVBOX_SURFACE.b * 1.1,
                                        )
                                    }
                                }
                                _ => base_color,
                            })),
                            text_color: if is_selected { Color::BLACK } else { GRUVBOX_TEXT },
                            border: Border {
                                color: if is_selected { GRUVBOX_GREEN } else { Color::TRANSPARENT },
                                width: if is_selected { 1.0 } else { 0.0 },
                                radius: 4.0.into(),
                            },
                            shadow: Shadow::default(),
                        }
                    });

                    command_button.into()
                }).collect::<Vec<Element<Message>>>()
            )
            .spacing(2);

            scrollable(suggestions)
                .height(Length::Fixed(300.0))
        } else {
            scrollable(
                container(
                    text("No commands found")
                        .color(GRUVBOX_TEXT_MUTED)
                        .size(14)
                )
                .padding(20)
                .center_x(Length::Fill)
            )
            .height(Length::Fixed(60.0))
        };

        let command_palette = container(
            column![
                command_input,
                vertical_space().height(8),
                suggestions_list
            ]
            .spacing(0)
        )
        .padding(20)
        .width(Length::Fixed(600.0))
        .max_height(400.0)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_BG)),
            border: Border {
                color: GRUVBOX_BORDER,
                width: 2.0,
                radius: 12.0.into(),
            },
            text_color: Some(GRUVBOX_TEXT),
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                offset: iced::Vector::new(0.0, 4.0),
                blur_radius: 20.0,
            },
        });

        // 将命令栏居中显示，添加半透明背景
        container(
            container(command_palette)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .width(Length::Fill)
                .height(Length::Fill)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.5))),
            border: Border::default(),
            text_color: None,
            shadow: Shadow::default(),
        })
        .into()
    }
}
