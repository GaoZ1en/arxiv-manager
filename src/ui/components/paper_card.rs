use iced::{
    widget::{button, column, container, row, text, Column, Container, Row},
    Alignment, Element, Length,
};

use crate::{
    app::Message,
    core::models::ArxivPaper,
    database::models::Paper,
    ui::style,
};

pub struct PaperCard;

impl PaperCard {
    /// Create paper card for ArXiv paper (used by search view)
    pub fn create(paper: &ArxivPaper, index: usize, is_selected: bool, show_download: bool) -> Element<Message> {
        Self::view_arxiv(paper, index)
    }

    pub fn view_arxiv(paper: &ArxivPaper, index: usize) -> Element<Message> {
        Self::create_card_content(
            &paper.title,
            &paper.authors.iter().map(|a| a.name.clone()).collect::<Vec<_>>(),
            &paper.summary,
            &paper.published.format("%Y-%m-%d").to_string(),
            &paper.categories.iter().map(|c| c.term.clone()).collect::<Vec<_>>(),
            Some(Message::DownloadPaper(index)),
            Some(Message::ViewPaperDetails(paper.id.clone())),
            None,
        )
    }

    pub fn view_local(paper: &Paper) -> Element<Message> {
        let download_action = if paper.downloaded {
            Some(Message::OpenLocalPaper(paper.id.unwrap_or(0).to_string()))
        } else {
            Some(Message::DownloadStoredPaper(paper.id.unwrap_or(0).to_string()))
        };

        Self::create_card_content(
            &paper.title,
            &paper.authors,
            &paper.abstract_text,
            &paper.published.format("%Y-%m-%d").to_string(),
            &paper.categories,
            download_action,
            Some(Message::ViewPaperDetails(paper.arxiv_id.clone())),
            Some(Message::RemoveFromLibrary(paper.id.unwrap_or(0).to_string())),
        )
    }

    fn create_card_content<'a>(
        title: &'a str,
        authors: &'a [String],
        summary: &'a str,
        published: &'a str,
        categories: &'a [String],
        primary_action: Option<Message>,
        details_action: Option<Message>,
        remove_action: Option<Message>,
    ) -> Element<'a, Message> {
        let mut content = Column::new()
            .spacing(10)
            .width(Length::Fill);

        // Title
        let title_text = text(title)
            .size(16)
            .style(GruvboxColors::Light0);
        content = content.push(title_text);

        // Authors
        let authors_str = if authors.len() > 3 {
            format!("{} 等", authors[..3].join(", "))
        } else {
            authors.join(", ")
        };
        let authors_text = text(format!("作者: {}", authors_str))
            .size(12)
            .style(GruvboxColors::Light2);
        content = content.push(authors_text);

        // Published date
        let published_text = text(format!("发布日期: {}", published))
            .size(11)
            .style(GruvboxColors::Light4);
        content = content.push(published_text);

        // Categories
        if !categories.is_empty() {
            let categories_row = Self::create_categories_row(categories);
            content = content.push(categories_row);
        }

        // Summary (truncated)
        let truncated_summary = Self::truncate_text(summary, 200);
        let summary_text = text(truncated_summary)
            .size(12)
            .style(GruvboxColors::Light3);
        content = content.push(summary_text);

        // Action buttons
        let actions_row = Self::create_actions_row(
            primary_action,
            details_action,
            remove_action,
        );
        content = content.push(actions_row);

        Container::new(content)
            .padding(15)
            .width(Length::Fill)
            .style(GruvboxStyle::card())
            .into()
    }

    fn create_categories_row(categories: &[String]) -> Row<Message> {
        let mut categories_row = Row::new().spacing(5);

        let visible_categories = if categories.len() > 3 {
            &categories[..3]
        } else {
            categories
        };

        for category in visible_categories {
            let category_chip = container(
                text(category)
                    .size(10)
                    .style(GruvboxColors::Dark0)
            )
            .padding([2, 8])
            .style(GruvboxStyle::chip());

            categories_row = categories_row.push(category_chip);
        }

        if categories.len() > 3 {
            let more_chip = container(
                text(format!("+{}", categories.len() - 3))
                    .size(10)
                    .style(GruvboxColors::Dark0)
            )
            .padding([2, 8])
            .style(GruvboxStyle::chip());

            categories_row = categories_row.push(more_chip);
        }

        categories_row
    }

    fn create_actions_row(
        primary_action: Option<Message>,
        details_action: Option<Message>,
        remove_action: Option<Message>,
    ) -> Row<'static, Message> {
        let mut actions_row = Row::new()
            .spacing(10)
            .align_items(Alignment::Center);

        // Primary action button
        if let Some(action) = primary_action {
            let button_text = match &action {
                Message::DownloadPaper(_) => "下载",
                Message::OpenLocalPaper(_) => "打开",
                Message::DownloadStoredPaper(_) => "下载",
                _ => "操作",
            };

            actions_row = actions_row.push(
                button(button_text)
                    .on_press(action)
                    .style(GruvboxStyle::button())
            );
        }

        // Details button
        if let Some(action) = details_action {
            actions_row = actions_row.push(
                button("详情")
                    .on_press(action)
                    .style(GruvboxStyle::secondary_button())
            );
        }

        // Remove button
        if let Some(action) = remove_action {
            actions_row = actions_row.push(
                button("移除")
                    .on_press(action)
                    .style(GruvboxStyle::danger_button())
            );
        }

        actions_row
    }

    fn truncate_text(text: &str, max_length: usize) -> String {
        if text.len() <= max_length {
            text.to_string()
        } else {
            let truncated = &text[..max_length.min(text.len())];
            // Try to break at a word boundary
            if let Some(last_space) = truncated.rfind(' ') {
                format!("{}...", &truncated[..last_space])
            } else {
                format!("{}...", truncated)
            }
        }
    }
}

/// Standalone create function for backward compatibility
pub fn create(paper: &ArxivPaper, index: usize, is_selected: bool, show_download: bool) -> Element<Message> {
    PaperCard::create(paper, index, is_selected, show_download)
}
