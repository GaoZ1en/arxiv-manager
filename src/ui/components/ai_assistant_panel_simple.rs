// AI Assistant Panel UI Component - Modern IRC-style design with Copilot integration

use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Element, Length, Padding};

use crate::ai::{AiChatMessage, AiSuggestion, AiMessage, MessageRole};
use crate::core::messages::Message;
use crate::core::app_state::ArxivManager;
use crate::ui::style::{
    button_primary_style_dynamic, button_secondary_style_dynamic, 
    chat_container_dynamic_style, sidebar_container_dynamic_style,
    text_input_dynamic_style, scrollable_style_dynamic
};
use crate::ui::components::CopilotPanel;

#[derive(Debug, Clone, Default)]
pub struct AiAssistantPanel {
    current_input: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AiPanelTab {
    Assistant,
    Copilot,
}

impl Default for AiPanelTab {
    fn default() -> Self {
        Self::Assistant
    }
}

impl AiAssistantPanel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn view<'a>(
        app: &'a ArxivManager,
        chat_history: &'a [AiChatMessage],
        suggestions: &'a [AiSuggestion],
        is_generating: bool,
    ) -> Element<'a, Message> {
        let theme_colors = app.theme_colors();
        let current_font = app.current_font();
        let base_font_size = app.current_font_size();
        let scale = app.current_scale();
        
        // Header with tabs
        let header = container(
            column![
                // Title row
                row![
                    text("🤖 AI Research Assistant")
                        .size(base_font_size * 1.1)
                        .font(iced::Font {
                            weight: iced::font::Weight::Medium,
                            ..current_font
                        })
                        .color(theme_colors.text_primary),
                    
                    Space::with_width(Length::Fill),
                    
                    button(text("×").size(base_font_size * 1.2))
                        .style(button_secondary_style_dynamic(&app.settings.theme))
                        .on_press(Message::ToggleAiAssistant)
                ]
                .spacing(8.0 * scale),
                
                // Tab bar
                Self::tab_bar(app, theme_colors, base_font_size, scale)
            ]
            .spacing(8.0 * scale)
        )
        .padding(Padding::new(16.0 * scale))
        .width(Length::Fill);

        // Content based on active tab
        let tab_content = match &app.ai_panel_tab {
            AiPanelTab::Assistant => Self::assistant_content(
                app, chat_history, suggestions, is_generating, 
                theme_colors, current_font, base_font_size, scale
            ),
            AiPanelTab::Copilot => CopilotPanel::view(app),
        };

        // Main panel content
        let panel_content = column![
            header,
            tab_content,
        ]
        .spacing(8.0 * scale)
        .width(Length::Fill);

        // Container with IRC-style design
        container(panel_content)
            .style(sidebar_container_dynamic_style(&app.settings.theme))
            .width(Length::Fixed(350.0 * scale))
            .height(Length::Fill)
            .into()
    }
    
    /// 标签页栏
    fn tab_bar(
        app: &ArxivManager,
        _theme_colors: crate::ui::theme::ThemeColors,
        base_font_size: f32,
        scale: f32,
    ) -> Element<Message> {
        let copilot_indicator = if app.ai_state.copilot_enabled {
            "● " // Green dot for connected
        } else {
            "○ " // Empty circle for disconnected
        };
        
        let assistant_button = if app.ai_panel_tab == AiPanelTab::Assistant {
            button(text("💬 Assistant").size(base_font_size * 0.85))
                .style(button_primary_style_dynamic(&app.settings.theme))
        } else {
            button(text("💬 Assistant").size(base_font_size * 0.85))
                .style(button_secondary_style_dynamic(&app.settings.theme))
        }
        .on_press(Message::ChangeAiPanelTab(AiPanelTab::Assistant))
        .width(Length::FillPortion(1));

        let copilot_button = if app.ai_panel_tab == AiPanelTab::Copilot {
            button(text(format!("{}🤖 Copilot", copilot_indicator)).size(base_font_size * 0.85))
                .style(button_primary_style_dynamic(&app.settings.theme))
        } else {
            button(text(format!("{}🤖 Copilot", copilot_indicator)).size(base_font_size * 0.85))
                .style(button_secondary_style_dynamic(&app.settings.theme))
        }
        .on_press(Message::ChangeAiPanelTab(AiPanelTab::Copilot))
        .width(Length::FillPortion(1));
        
        row![
            assistant_button,
            Space::with_width(4.0 * scale),
            copilot_button,
        ]
        .spacing(4.0 * scale)
        .into()
    }
    
    /// AI 助手内容
    fn assistant_content<'a>(
        app: &'a ArxivManager,
        chat_history: &'a [AiChatMessage],
        suggestions: &'a [AiSuggestion],
        is_generating: bool,
        theme_colors: crate::ui::theme::ThemeColors,
        current_font: iced::Font,
        base_font_size: f32,
        scale: f32,
    ) -> Element<'a, Message> {
        // Chat area
        let chat_area = if chat_history.is_empty() {
            container(
                column![
                    text("👋 Welcome to AI Research Assistant!")
                        .size(base_font_size)
                        .color(theme_colors.text_primary),
                    text("I can help you analyze papers and generate insights.")
                        .size(base_font_size * 0.9)
                        .color(theme_colors.text_secondary),
                ]
                .spacing(8.0 * scale)
            )
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fixed(200.0 * scale))
        } else {
            let mut chat_column = column![]
                .spacing(8.0 * scale)
                .width(Length::Fill);

            for message in chat_history {
                chat_column = chat_column.push(
                    Self::message_view(message, theme_colors, base_font_size, scale)
                );
            }

            if is_generating {
                chat_column = chat_column.push(
                    text("🤖 Thinking...")
                        .size(base_font_size * 0.9)
                        .color(theme_colors.text_muted)
                        .font(iced::Font {
                            style: iced::font::Style::Italic,
                            ..current_font
                        })
                );
            }

            container(
                scrollable(chat_column)
                    .style(scrollable_style_dynamic(&app.settings.theme))
                    .width(Length::Fill)
                    .height(Length::Fixed(200.0 * scale))
            )
            .padding(Padding::new(8.0 * scale))
            .width(Length::Fill)
        };

        let styled_chat_area = container(chat_area)
            .style(chat_container_dynamic_style(&app.settings.theme))
            .width(Length::Fill);

        // Suggestions area
        let suggestions_area = if !suggestions.is_empty() {
            container(
                column![
                    text("💡 Suggestions")
                        .size(base_font_size * 0.9)
                        .color(theme_colors.text_primary),
                    
                    column(
                        suggestions.iter().take(3).map(|s| {
                            button(
                                text(&s.title)
                                    .size(base_font_size * 0.85)
                            )
                            .style(button_secondary_style_dynamic(&app.settings.theme))
                            .on_press(Message::Ai(AiMessage::ApplySuggestion(s.id.clone())))
                            .width(Length::Fill)
                            .into()
                        }).collect::<Vec<_>>()
                    )
                    .spacing(4.0 * scale)
                    .width(Length::Fill)
                ]
                .spacing(8.0 * scale)
            )
            .padding(Padding::new(12.0 * scale))
            .style(chat_container_dynamic_style(&app.settings.theme))
            .width(Length::Fill)
        } else {
            container(Space::new(Length::Fixed(0.0), Length::Fixed(0.0)))
        };

        // Input area
        let input_area = container(
            row![
                text_input("Ask me anything about your research...", &app.ai_state.current_input)
                    .style(text_input_dynamic_style(&app.settings.theme))
                    .on_input(|input| Message::Ai(AiMessage::UpdateChatInput(input)))
                    .on_submit(Message::Ai(AiMessage::SendChatMessage("".to_string())))
                    .size(base_font_size * 0.9),
                
                button(text("Send").size(base_font_size * 0.9))
                    .style(button_primary_style_dynamic(&app.settings.theme))
                    .on_press(Message::Ai(AiMessage::SendChatMessage("".to_string())))
            ]
            .spacing(8.0 * scale)
        )
        .padding(Padding::new(16.0 * scale))
        .width(Length::Fill);

        // Assistant content
        column![
            styled_chat_area,
            suggestions_area,
            input_area,
        ]
        .spacing(8.0 * scale)
        .width(Length::Fill)
        .into()
    }

    fn message_view(
        message: &AiChatMessage,
        theme_colors: crate::ui::theme::ThemeColors,
        base_font_size: f32,
        scale: f32,
    ) -> Element<'static, Message> {
        let icon = match message.role {
            MessageRole::User => "👤",
            MessageRole::Assistant => "🤖",
            MessageRole::System => "ℹ️",
        };

        container(
            column![
                row![
                    text(icon).size(base_font_size * 0.9),
                    text(message.timestamp.format("%H:%M").to_string())
                        .size(base_font_size * 0.75)
                        .color(theme_colors.text_muted),
                ]
                .spacing(6.0 * scale),

                text(message.content.clone())
                    .size(base_font_size * 0.85)
                    .color(theme_colors.text_primary)
                    .wrapping(text::Wrapping::Word)
            ]
            .spacing(4.0 * scale)
        )
        .padding(Padding::new(8.0 * scale))
        .width(Length::Fill)
        .into()
    }
}
