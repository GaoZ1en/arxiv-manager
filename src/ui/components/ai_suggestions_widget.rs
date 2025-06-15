// AI Suggestions Widget
// Displays AI suggestions in a floating panel

use iced::{
    widget::{button, column, container, row, text, Column, Space},
    Background, Color, Element, Length, Theme,
};

use crate::ai::{AiMessage, AiSuggestion, SuggestionType};
use crate::core::messages::Message;

#[derive(Debug, Clone, Default)]
pub struct AiSuggestionsWidget {
    pub is_visible: bool,
}

impl AiSuggestionsWidget {
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

    pub fn view(&self, suggestions: &[AiSuggestion]) -> Element<'_, Message> {
        if !self.is_visible || suggestions.is_empty() {
            return Space::new(Length::Fixed(0.0), Length::Fixed(0.0)).into();
        }

        let header = row![
            text("🔮 AI Suggestions")
                .size(16)
                .style(|theme: &Theme| text::Appearance {
                    color: Some(theme.palette().text),
                }),
            Space::with_width(Length::Fill),
            button("×")
                .style(button::text)
                .on_press(Message::Ai(AiMessage::HideSuggestions))
        ]
        .align_items(iced::Alignment::Center);

        let mut suggestions_column = Column::new()
            .spacing(8)
            .width(Length::Fill)
            .push(header);

        // Group suggestions by type
        let mut search_suggestions = Vec::new();
        let mut paper_suggestions = Vec::new();
        let mut code_suggestions = Vec::new();
        let mut trend_suggestions = Vec::new();

        for suggestion in suggestions {
            match suggestion.suggestion_type {
                SuggestionType::SearchQuery => search_suggestions.push(suggestion),
                SuggestionType::RelatedPapers => paper_suggestions.push(suggestion),
                SuggestionType::CodeExample => code_suggestions.push(suggestion),
                SuggestionType::ResearchTrend => trend_suggestions.push(suggestion),
                _ => {}
            }
        }

        // Add sections
        if !search_suggestions.is_empty() {
            suggestions_column = suggestions_column.push(
                self.section_view("🔍 Search Suggestions", &search_suggestions)
            );
        }

        if !paper_suggestions.is_empty() {
            suggestions_column = suggestions_column.push(
                self.section_view("📄 Related Papers", &paper_suggestions)
            );
        }

        if !code_suggestions.is_empty() {
            suggestions_column = suggestions_column.push(
                self.section_view("💻 Code Examples", &code_suggestions)
            );
        }

        if !trend_suggestions.is_empty() {
            suggestions_column = suggestions_column.push(
                self.section_view("📈 Research Trends", &trend_suggestions)
            );
        }

        container(suggestions_column)
            .width(Length::Fixed(350.0))
            .max_height(500.0)
            .style(|theme: &Theme| container::Appearance {
                background: Some(Background::Color(theme.palette().background)),
                border: iced::Border {
                    color: Color::from_rgb(0.8, 0.8, 0.8),
                    width: 1.0,
                    radius: 8.0.into(),
                },
                shadow: iced::Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                    offset: iced::Vector::new(0.0, 4.0),
                    blur_radius: 8.0,
                },
                ..Default::default()
            })
            .padding(16)
            .into()
    }

    fn section_view(&self, title: &str, suggestions: &[&AiSuggestion]) -> Element<'_, Message> {
        let mut section_column = Column::new()
            .spacing(4)
            .width(Length::Fill)
            .push(
                text(title)
                    .size(14)
                    .style(|theme: &Theme| text::Appearance {
                        color: Some(Color::from_rgb(0.4, 0.4, 0.4)),
                    })
            );

        for suggestion in suggestions.iter().take(3) {
            section_column = section_column.push(self.suggestion_item_view(suggestion));
        }

        container(section_column)
            .style(|_: &Theme| container::Appearance {
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

    fn suggestion_item_view(&self, suggestion: &AiSuggestion) -> Element<'_, Message> {
        let confidence_badge = self.confidence_badge(suggestion.confidence);
        let type_icon = self.get_type_icon(&suggestion.suggestion_type);

        button(
            row![
                text(type_icon).size(16),
                column![
                    text(&suggestion.title)
                        .size(13)
                        .style(|theme: &Theme| text::Appearance {
                            color: Some(theme.palette().text),
                        }),
                    text(&suggestion.description)
                        .size(11)
                        .style(|theme: &Theme| text::Appearance {
                            color: Some(Color::from_rgb(0.6, 0.6, 0.6)),
                        })
                ]
                .spacing(2),
                Space::with_width(Length::Fill),
                confidence_badge
            ]
            .spacing(8)
            .align_items(iced::Alignment::Center)
        )
        .style(|theme: &Theme| button::Appearance {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: theme.palette().text,
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        })
        .width(Length::Fill)
        .on_press(Message::Ai(AiMessage::ApplySuggestion(suggestion.clone())))
        .into()
    }

    fn confidence_badge(&self, confidence: f32) -> Element<'_, Message> {
        let (color, text_color) = if confidence > 0.8 {
            (Color::from_rgb(0.2, 0.8, 0.2), Color::WHITE)
        } else if confidence > 0.6 {
            (Color::from_rgb(1.0, 0.8, 0.2), Color::BLACK)
        } else {
            (Color::from_rgb(0.8, 0.8, 0.8), Color::BLACK)
        };

        container(
            text(format!("{:.0}%", confidence * 100.0))
                .size(10)
                .style(move |_: &Theme| text::Appearance {
                    color: Some(text_color),
                })
        )
        .style(move |_: &Theme| container::Appearance {
            background: Some(Background::Color(color)),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 10.0.into(),
            },
            ..Default::default()
        })
        .padding([2, 6])
        .into()
    }

    fn get_type_icon(&self, suggestion_type: &SuggestionType) -> &'static str {
        match suggestion_type {
            SuggestionType::SearchQuery => "🔍",
            SuggestionType::RelatedPapers => "📄",
            SuggestionType::CodeExample => "💻",
            SuggestionType::ResearchTrend => "📈",
            SuggestionType::Collaboration => "🤝",
            SuggestionType::DatasetSuggestion => "📊",
        }
    }
}
