// 命令面板组件

use iced::widget::{button, column, container, row, text, text_input, scrollable, horizontal_space, vertical_space};
use iced::{Element, Length, Color, Background, Border, Shadow};

use crate::core::app_state::ArxivManager;
use crate::core::messages::Message;


pub struct CommandPalette;

impl CommandPalette {
    pub fn view(app: &ArxivManager) -> Element<'_, Message> {
        let theme_colors = app.theme_colors();
        let current_font = app.current_font();
        let base_font_size = app.current_font_size();
        let scale = app.current_scale();
        
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
            .size(base_font_size * 1.29)
            .font(current_font)
            .style(move |_theme, status| iced::widget::text_input::Style {
                background: Background::Color(theme_colors.dark_bg_secondary),
                border: Border {
                    color: match status {
                        iced::widget::text_input::Status::Focused => theme_colors.accent_border,
                        _ => theme_colors.border_color,
                    },
                    width: 2.0,
                    radius: 6.0.into(),
                },
                icon: Color::TRANSPARENT,
                placeholder: theme_colors.text_muted,
                value: theme_colors.text_primary,
                selection: theme_colors.accent_border,
            });

        // 命令建议列表
        let suggestions_list = if !app.command_suggestions.is_empty() {
            let suggestions = column(
                app.command_suggestions.iter().enumerate().map(|(index, command)| {
                    let is_selected = app.selected_command_index == Some(index);
                    
                    let command_button = button(
                        row![
                            text(command.display_name())
                                .color(if is_selected { Color::BLACK } else { theme_colors.text_primary })
                                .size(base_font_size)
                                .font(current_font),
                            horizontal_space(),
                            text(command.keywords().join(" "))
                                .color(if is_selected { Color::from_rgb(0.3, 0.3, 0.3) } else { theme_colors.text_muted })
                                .size(base_font_size * 0.86)
                                .font(current_font),
                        ]
                        .padding([8.0 * scale, 12.0 * scale])
                        .width(Length::Fill)
                    )
                    .on_press(Message::ExecuteCommand(command.clone()))
                    .width(Length::Fill)
                    .style(move |_theme, status| {
                        let base_color = if is_selected {
                            theme_colors.accent_border
                        } else {
                            theme_colors.dark_bg_secondary
                        };
                        
                        iced::widget::button::Style {
                            background: Some(Background::Color(match status {
                                iced::widget::button::Status::Hovered => {
                                    if is_selected {
                                        Color::from_rgb(
                                            theme_colors.accent_border.r * 0.9,
                                            theme_colors.accent_border.g * 0.9,
                                            theme_colors.accent_border.b * 0.9,
                                        )
                                    } else {
                                        Color::from_rgb(
                                            theme_colors.dark_bg_secondary.r * 1.1,
                                            theme_colors.dark_bg_secondary.g * 1.1,
                                            theme_colors.dark_bg_secondary.b * 1.1,
                                        )
                                    }
                                }
                                _ => base_color,
                            })),
                            text_color: if is_selected { Color::BLACK } else { theme_colors.text_primary },
                            border: Border {
                                color: if is_selected { theme_colors.accent_border } else { Color::TRANSPARENT },
                                width: if is_selected { 1.0 } else { 0.0 },
                                radius: 4.0.into(),
                            },
                            shadow: Shadow::default(),
                        }
                    });

                    command_button.into()
                }).collect::<Vec<Element<Message>>>()
            )
            .spacing(2.0 * scale);

            scrollable(suggestions)
                .height(Length::Fixed(300.0 * scale))
        } else {
            scrollable(
                container(
                    text("No commands found")
                        .color(theme_colors.text_muted)
                        .size(base_font_size)
                        .font(current_font)
                )
                .padding(20.0 * scale)
                .center_x(Length::Fill)
            )
            .height(Length::Fixed(60.0 * scale))
        };

        let command_palette = container(
            column![
                command_input,
                vertical_space().height(8.0 * scale),
                suggestions_list
            ]
            .spacing(0.0)
        )
        .padding(20.0 * scale)
        .width(Length::Fixed(600.0 * scale))
        .max_height(400.0 * scale)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(theme_colors.dark_bg)),
            border: Border {
                color: theme_colors.border_color,
                width: 2.0,
                radius: 12.0.into(),
            },
            text_color: Some(theme_colors.text_primary),
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
