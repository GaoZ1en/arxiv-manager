use crate::app::{AppMessage, AppState};
use crate::core::ArxivPaper;
use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Element, Length};

pub fn search_view(state: &AppState) -> Element<AppMessage> {
    let search_input = text_input("Enter search keywords...", &state.search_query)
        .on_input(AppMessage::SearchQueryChanged)
        .on_submit(AppMessage::SearchSubmitted)
        .padding(10)
        .size(16);

    let search_button = button(text("Search").size(16))
        .on_press(AppMessage::SearchSubmitted)
        .padding([10, 20]);

    let search_row = row![search_input, search_button]
        .spacing(10)
        .align_y(Alignment::Center);

    let results_list = if state.is_searching {
        column![text("Searching...").size(16)]
    } else if state.search_results.is_empty() {
        column![text("No search results").size(16)]
    } else {
        let mut results = column![].spacing(10);

        for paper in &state.search_results {
            results = results.push(paper_card(paper));
        }

        results
    };

    let content = column![
        container(search_row).padding(20),
        container(scrollable(results_list).height(Length::Fill))
            .padding(20)
            .height(Length::Fill)
    ]
    .spacing(10);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn paper_card(paper: &ArxivPaper) -> Element<AppMessage> {
    let title = text(&paper.title).size(18);

    let authors = text(format!("Authors: {}", paper.authors.join(", ")))
        .size(14);

    let categories = text(format!("分类: {}", paper.categories.join(", ")))
        .size(12);

    let published = text(format!("发布时间: {}", paper.published.format("%Y-%m-%d")))
        .size(12);

    let abstract_preview = {
        let preview = if paper.abstract_text.len() > 200 {
            format!("{}...", &paper.abstract_text[..200])
        } else {
            paper.abstract_text.clone()
        };
        text(preview).size(14)
    };

    let download_button = button(text("下载 PDF").size(14))
        .on_press(AppMessage::DownloadPaper(paper.clone()));

    let card_content = column![
        title,
        authors,
        row![categories, published].spacing(20),
        abstract_preview,
        row![download_button].spacing(10)
    ]
    .spacing(8)
    .padding(15);

    container(card_content)
        .width(Length::Fill)
        .into()
}