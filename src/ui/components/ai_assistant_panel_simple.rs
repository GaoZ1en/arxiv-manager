// Simplified AI Assistant Panel UI Component

use iced::widget::{
    button, column, container, row, scrollable, text, text_input
};
use iced::{Element, Length, Theme};

use crate::ai::{AiChatMessage, AiSuggestion, AiMessage, MessageRole};
use crate::core::messages::Message;

#[derive(Debug, Clone, Default)]
pub struct AiAssistantPanel {
    current_input: String,
}

impl AiAssistantPanel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn view(
        chat_history: &[AiChatMessage],
        suggestions: &[AiSuggestion],
    ) -> Element<'static, Message, Theme> {
        let panel_content = column![
            // Header
            row![
                text("🤖 AI Assistant").size(16),
                button("×").on_press(Message::ToggleAiAssistant)
            ]
            .spacing(10),
            
            // Chat area
            container(
                scrollable(
                    column(
                        chat_history.iter().map(|msg| message_view(msg))
                            .collect::<Vec<_>>()
                    )
                    .spacing(8)
                )
                .height(Length::Fixed(200.0))
            )
            .padding(8),
            
            // Suggestions  
            if suggestions.is_empty() {
                Element::from(text("No suggestions").size(12))
            } else {
                column![
                    text("💡 Suggestions").size(14),
                    column(
                        suggestions.iter().take(3).map(|s| {
                            let title = s.title.clone();
                            let id = s.id.clone();
                            button(text(title).size(12))
                                .on_press(Message::Ai(AiMessage::ApplySuggestion(id)))
                                .width(Length::Fill)
                                .into()
                        }).collect::<Vec<_>>()
                    )
                    .spacing(4)
                ]
                .spacing(8)
                .into()
            },
            
            // Input
            row![
                text_input("Ask me anything...", "")
                    .on_input(|input| Message::Ai(AiMessage::SendChatMessage(input)))
                    .width(Length::Fill),
                button("Send")
                    .on_press(Message::Ai(AiMessage::SendChatMessage("".to_string())))
            ]
            .spacing(8)
        ]
        .spacing(12)
        .padding(16);

        container(panel_content)
            .width(Length::Fixed(300.0))
            .height(Length::Fill)
            .into()
    }
}

fn message_view(message: &AiChatMessage) -> Element<'static, Message, Theme> {
    let icon = match message.role {
        MessageRole::User => "👤",
        MessageRole::Assistant => "🤖",
        MessageRole::System => "ℹ️",
    };

    let content = message.content.clone();

    container(
        row![
            text(icon).size(14),
            text(content).size(12)
        ]
        .spacing(8)
    )
    .padding(8)
    .width(Length::Fill)
    .into()
}
