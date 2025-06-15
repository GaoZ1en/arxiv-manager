// AI Assistant Panel UI Component
// Chat-style interface for AI interactions

use iced::{
    widget::{button, column, container, row, scrollable, text, text_input, Column, Space},
    Background, Color, Element, Length, Padding, Theme,
};

use crate::ai::{AiMessage, AiChatMessage, AiSuggestion, MessageRole};
use crate::core::messages::Message;

#[derive(Debug, Clone, Default)]
pub struct AiAssistantPanel {
    pub is_visible: bool,
    pub current_input: String,
    pub session_active: bool,
}

impl AiAssistantPanel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show(&mut self) {
        self.is_visible = true;
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
    }

    pub fn toggle(&mut self) {
        self.is_visible = !self.is_visible;
    }

    pub fn update_input(&mut self, input: String) {
        self.current_input = input;
    }

    pub fn clear_input(&mut self) {
        self.current_input.clear();
    }

    pub fn view(
        &self,
        chat_history: &[AiChatMessage],
        suggestions: &[AiSuggestion],
    ) -> Element<'_, Message> {
        if !self.is_visible {
            return Space::new(Length::Fixed(0.0), Length::Fixed(0.0)).into();
        }

        let panel_content = column![
            self.header_view(),
            self.chat_view(chat_history),
            self.suggestions_view(suggestions),
            self.input_view(),
        ]
        .spacing(8)
        .padding(16);

        container(panel_content)
            .width(Length::Fixed(400.0))
            .height(Length::Fill)
            .style(|theme: &Theme| container::Appearance {
                background: Some(Background::Color(theme.palette().background)),
                border: iced::Border {
                    color: Color::from_rgb(0.7, 0.7, 0.7),
                    width: 1.0,
                    radius: 8.0.into(),
                },
                ..Default::default()
            })
            .into()
    }

    fn header_view(&self) -> Element<'_, Message> {
        row![
            text("AI Assistant")
                .size(18)
                .style(|theme: &Theme| text::Appearance {
                    color: Some(theme.palette().text),
                }),
            Space::with_width(Length::Fill),
            button("×")
                .style(button::danger)
                .on_press(Message::Ai(AiMessage::ToggleAssistant))
        ]
        .align_items(iced::Alignment::Center)
        .into()
    }

    fn chat_view(&self, chat_history: &[AiChatMessage]) -> Element<'_, Message> {
        let mut chat_column = Column::new().spacing(8).width(Length::Fill);

        if chat_history.is_empty() {
            chat_column = chat_column.push(
                text("👋 Hi! I'm your AI research assistant. Ask me anything about your papers!")
                    .size(14)
                    .style(|theme: &Theme| text::Appearance {
                        color: Some(Color::from_rgb(0.6, 0.6, 0.6)),
                    })
            );
        } else {
            for message in chat_history {
                chat_column = chat_column.push(self.message_view(message));
            }
        }

        container(
            scrollable(chat_column)
                .width(Length::Fill)
                .height(Length::Fixed(300.0))
        )
        .style(|theme: &Theme| container::Appearance {
            background: Some(Background::Color(Color::from_rgb(0.98, 0.98, 0.98))),
            border: iced::Border {
                color: Color::from_rgb(0.9, 0.9, 0.9),
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        })
        .padding(8)
        .into()
    }

    fn message_view(&self, message: &AiChatMessage) -> Element<'_, Message> {
        let (icon, color, alignment) = match message.role {
            MessageRole::User => ("👤", Color::from_rgb(0.2, 0.6, 1.0), iced::Alignment::End),
            MessageRole::Assistant => ("🤖", Color::from_rgb(0.2, 0.8, 0.2), iced::Alignment::Start),
            MessageRole::System => ("ℹ️", Color::from_rgb(0.6, 0.6, 0.6), iced::Alignment::Center),
        };

        let message_content = container(
            row![
                text(icon).size(16),
                text(&message.content)
                    .size(14)
                    .style(move |_: &Theme| text::Appearance {
                        color: Some(Color::BLACK),
                    })
            ]
            .spacing(8)
            .align_items(iced::Alignment::Center)
        )
        .style(move |_: &Theme| container::Appearance {
            background: Some(Background::Color(Color::from_rgba(color.r, color.g, color.b, 0.1))),
            border: iced::Border {
                color,
                width: 1.0,
                radius: 8.0.into(),
            },
            ..Default::default()
        })
        .padding(8)
        .width(Length::FillPortion(3));

        match alignment {
            iced::Alignment::End => row![Space::with_width(Length::FillPortion(1)), message_content].into(),
            iced::Alignment::Start => row![message_content, Space::with_width(Length::FillPortion(1))].into(),
            _ => container(message_content).center_x().into(),
        }
    }

    fn suggestions_view(&self, suggestions: &[AiSuggestion]) -> Element<'_, Message> {
        if suggestions.is_empty() {
            return Space::new(Length::Fixed(0.0), Length::Fixed(0.0)).into();
        }

        let mut suggestions_column = Column::new()
            .spacing(4)
            .width(Length::Fill)
            .push(
                text("💡 Suggestions")
                    .size(14)
                    .style(|theme: &Theme| text::Appearance {
                        color: Some(theme.palette().text),
                    })
            );

        for suggestion in suggestions.iter().take(3) {
            suggestions_column = suggestions_column.push(self.suggestion_view(suggestion));
        }

        container(suggestions_column)
            .style(|theme: &Theme| container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.95, 0.98, 1.0))),
                border: iced::Border {
                    color: Color::from_rgb(0.8, 0.9, 1.0),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            })
            .padding(8)
            .into()
    }

    fn suggestion_view(&self, suggestion: &AiSuggestion) -> Element<'_, Message> {
        let confidence_color = if suggestion.confidence > 0.8 {
            Color::from_rgb(0.2, 0.8, 0.2)
        } else if suggestion.confidence > 0.6 {
            Color::from_rgb(1.0, 0.8, 0.2)
        } else {
            Color::from_rgb(0.8, 0.8, 0.8)
        };

        button(
            row![
                text("💭").size(12),
                column![
                    text(&suggestion.title)
                        .size(12)
                        .style(|theme: &Theme| text::Appearance {
                            color: Some(theme.palette().text),
                        }),
                    text(format!("Confidence: {:.0}%", suggestion.confidence * 100.0))
                        .size(10)
                        .style(move |_: &Theme| text::Appearance {
                            color: Some(confidence_color),
                        })
                ]
                .spacing(2)
            ]
            .spacing(6)
            .align_items(iced::Alignment::Center)
        )
        .style(|theme: &Theme| button::Appearance {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: theme.palette().text,
            border: iced::Border {
                color: Color::from_rgb(0.9, 0.9, 0.9),
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        })
        .width(Length::Fill)
        .on_press(Message::Ai(AiMessage::ApplySuggestion(suggestion.clone())))
        .into()
    }

    fn input_view(&self) -> Element<'_, Message> {
        row![
            text_input("Ask me anything...", &self.current_input)
                .on_input(|input| Message::Ai(AiMessage::UpdateChatInput(input)))
                .on_submit(Message::Ai(AiMessage::SendChatMessage(self.current_input.clone())))
                .size(14)
                .padding(8),
            button("Send")
                .style(button::primary)
                .on_press(Message::Ai(AiMessage::SendChatMessage(self.current_input.clone())))
                .padding([8, 16]),
        ]
        .spacing(8)
        .align_items(iced::Alignment::Center)
        .into()
    }
}
