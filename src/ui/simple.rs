use iced::widget::{container, text, column};
use iced::{Element, Length};
use crate::app::{AppMessage, AppState};

pub fn simple_view(state: &AppState) -> Element<AppMessage> {
    let content = column![
        text("arXiv Paper Manager").size(24),
        text("Project under development...").size(16),
        text(format!("Current tab: {:?}", state.active_tab)).size(14),
    ]
    .spacing(20)
    .padding(40);
    
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
