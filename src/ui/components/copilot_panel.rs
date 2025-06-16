// GitHub Copilot 面板组件
// 显示 Copilot 状态、建议和控制选项

use iced::widget::{button, column, container, row, text, scrollable, Space};
use iced::{Element, Length, Alignment, Color, Background, Border};

use crate::core::messages::Message;
use crate::core::app_state::ArxivManager;
use crate::ai::AiMessage;

pub struct CopilotPanel;

impl CopilotPanel {
    /// 创建 GitHub Copilot 面板视图
    pub fn view(app: &ArxivManager) -> Element<Message> {
        let theme_colors = app.theme_colors();
        
        let content = column![
            // 面板标题
            container(
                row![
                    text("GitHub Copilot")
                        .size(18)
                        .color(Color::from_rgb(0.9, 0.9, 0.9)),
                    Space::with_width(Length::Fill),
                    // Copilot 状态指示器
                    Self::status_indicator(app)
                ]
                .align_y(Alignment::Center)
            )
            .padding(16)
            .style(move |_theme| container::Style {
                background: Some(Background::Color(theme_colors.surface())),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 8.0.into(),
                },
                ..Default::default()
            }),
            
            // 认证状态和控制
            Self::auth_section(app),
            
            // Copilot 建议显示
            Self::suggestions_section(app),
            
            // 控制按钮
            Self::controls_section(app),
        ]
        .spacing(12)
        .padding(16);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_theme| container::Style {
                background: Some(Background::Color(theme_colors.background())),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 12.0.into(),
                },
                ..Default::default()
            })
            .into()
    }
    
    /// 状态指示器
    fn status_indicator(app: &ArxivManager) -> Element<Message> {
        let (status_text, status_color) = if app.ai_state.copilot_enabled {
            ("●", Color::from_rgb(0.0, 0.8, 0.0)) // 绿色表示已连接
        } else {
            ("●", Color::from_rgb(0.8, 0.0, 0.0)) // 红色表示未连接
        };
        
        text(status_text)
            .size(16)
            .color(status_color)
            .into()
    }
    
    /// 认证部分
    fn auth_section(app: &ArxivManager) -> Element<Message> {
        let theme_colors = app.theme_colors();
        
        let auth_content = if let Some(auth) = &app.ai_state.copilot_auth_status {
            column![
                text(format!("Status: {}", auth.status))
                    .size(14)
                    .color(Color::from_rgb(0.8, 0.8, 0.8)),
                
                if let Some(user) = &auth.user {
                    Element::from(text(format!("User: {}", user))
                        .size(12)
                        .color(Color::from_rgb(0.6, 0.6, 0.6)))
                } else {
                    Space::with_height(Length::Fixed(0.0)).into()
                },
                
                row![
                    Space::with_width(Length::Fill),
                    button("Sign Out")
                        .on_press(Message::Ai(AiMessage::CopilotSignOut))
                        .padding([8, 16])
                        .style(iced::widget::button::primary)
                ]
            ]
            .spacing(8)
        } else {
            column![
                text("GitHub Copilot is not authenticated")
                    .size(14)
                    .color(Color::from_rgb(0.8, 0.8, 0.8)),
                
                row![
                    Space::with_width(Length::Fill),
                    button("Sign In")
                        .on_press(Message::Ai(AiMessage::CopilotSignIn))
                        .padding([8, 16])
                        .style(iced::widget::button::primary)
                ]
            ]
            .spacing(8)
        };
        
        container(auth_content)
            .width(Length::Fill)
            .padding(12)
            .style(move |_theme| container::Style {
                background: Some(Background::Color(Color::from_rgba(
                    theme_colors.surface().r,
                    theme_colors.surface().g,
                    theme_colors.surface().b,
                    0.5,
                ))),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 6.0.into(),
                },
                ..Default::default()
            })
            .into()
    }
    
    /// 建议显示部分
    fn suggestions_section(app: &ArxivManager) -> Element<Message> {
        let theme_colors = app.theme_colors();
        
        if app.ai_state.copilot_suggestions.is_empty() {
            container(
                text("No Copilot suggestions available")
                    .size(14)
                    .color(Color::from_rgb(0.6, 0.6, 0.6))
            )
            .width(Length::Fill)
            .padding(20)
            .style(move |_theme| container::Style {
                background: Some(Background::Color(Color::from_rgba(
                    theme_colors.surface().r,
                    theme_colors.surface().g,
                    theme_colors.surface().b,
                    0.3,
                ))),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 6.0.into(),
                },
                ..Default::default()
            })
            .into()
        } else {
            let suggestions = app.ai_state.copilot_suggestions.iter()
                .map(|suggestion| {
                    container(
                        column![
                            text(&suggestion.display_text)
                                .size(14)
                                .color(Color::from_rgb(0.9, 0.9, 0.9)),
                            
                            text(&suggestion.text)
                                .size(12)
                                .color(Color::from_rgb(0.7, 0.7, 0.7)),
                            
                            row![
                                Space::with_width(Length::Fill),
                                button("Apply")
                                    .on_press(Message::Ai(AiMessage::ApplyCopilotSuggestion(suggestion.uuid.clone())))
                                    .padding([4, 12])
                                    .style(iced::widget::button::primary)
                            ]
                        ]
                        .spacing(4)
                    )
                    .width(Length::Fill)
                    .padding(8)
                    .style(move |_theme| container::Style {
                        background: Some(Background::Color(theme_colors.surface())),
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: 4.0.into(),
                        },
                        ..Default::default()
                    })
                    .into()
                })
                .collect::<Vec<_>>();
            
            scrollable(
                column(suggestions)
                    .spacing(8)
                    .padding(8)
            )
            .height(Length::Fixed(200.0))
            .into()
        }
    }
    
    /// 控制按钮部分
    fn controls_section(app: &ArxivManager) -> Element<Message> {
        let theme_colors = app.theme_colors();
        
        let controls = row![
            button("Initialize Copilot")
                .on_press(Message::Ai(AiMessage::InitializeCopilot))
                .padding([8, 16])
                .style(iced::widget::button::primary),
            
            Space::with_width(16),
            
            button("Get Suggestions")
                .on_press(Message::Ai(AiMessage::GetCopilotCompletions(
                    lsp_types::Position::new(0, 0)
                )))
                .padding([8, 16])
                .style(iced::widget::button::secondary),
            
            Space::with_width(Length::Fill),
            
            if app.ai_state.copilot_enabled {
                Element::from(button("Test Document")
                    .on_press(Message::Ai(AiMessage::OpenDocumentInCopilot {
                        uri: "file:///test.py".to_string(),
                        content: "# Test Python file\nimport numpy as np\n".to_string(),
                        language: "python".to_string(),
                    }))
                    .padding([8, 16])
                    .style(iced::widget::button::success))
            } else {
                Element::from(Space::with_width(Length::Fixed(0.0)))
            }
        ]
        .align_y(iced::Alignment::Center);
        
        container(controls)
            .width(Length::Fill)
            .padding(12)
            .style(move |_theme| container::Style {
                background: Some(Background::Color(Color::from_rgba(
                    theme_colors.surface().r,
                    theme_colors.surface().g,
                    theme_colors.surface().b,
                    0.5,
                ))),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 6.0.into(),
                },
                ..Default::default()
            })
            .into()
    }
}
