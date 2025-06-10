// 用户界面视图组件

use iced::widget::{
    button, column, container, row, text, text_input, scrollable, progress_bar, 
    pane_grid, horizontal_rule, vertical_space
};
use iced::{Element, Length, Color, Background, Border, Shadow};

use crate::app_state::ArxivManager;
use crate::models::{ArxivPaper, DownloadItem, DownloadStatus, PaneType};
use crate::messages::Message;
use crate::style::{button_primary_style, button_secondary_style, button_danger_style};
use crate::theme::*;

impl ArxivManager {
    pub fn view(&self) -> Element<Message> {
        let sidebar = if self.sidebar_visible {
            Some(self.sidebar())
        } else {
            None
        };

        let pane_grid = pane_grid::PaneGrid::new(&self.panes, |_pane, pane_data, _is_maximized| {
            let title_bar = pane_grid::TitleBar::new(text(&pane_data.title).color(GRUVBOX_TEXT))
                .controls(pane_grid::Controls::new(text("×")));
            
            let content = match pane_data.pane_type {
                PaneType::Search => self.search_view(),
                PaneType::Library => self.library_view(),
                PaneType::Downloads => self.downloads_view(),
                PaneType::Settings => self.settings_view(),
                PaneType::PaperView(index) => {
                    if let Some(paper) = self.saved_papers.get(index) {
                        self.paper_view(paper)
                    } else {
                        container(text("Paper not found").color(GRUVBOX_TEXT)).into()
                    }
                }
            };

            pane_grid::Content::new(content)
                .title_bar(title_bar)
        })
        .on_click(Message::PaneClicked)
        .on_resize(10, Message::PaneResized)
        .on_drag(Message::PaneDragged)
        .spacing(4);

        let main_content = container(pane_grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme| iced::widget::container::Style {
                background: Some(Background::Color(GRUVBOX_BG)),
                border: Border::default(),
                text_color: Some(GRUVBOX_TEXT),
                shadow: Shadow::default(),
            });

        if let Some(sidebar) = sidebar {
            row![sidebar, main_content]
                .spacing(0)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        } else {
            main_content.into()
        }
    }

    pub fn sidebar(&self) -> Element<Message> {
        let toggle_button = button(text("☰").color(GRUVBOX_TEXT))
            .on_press(Message::SidebarToggled)
            .style(button_secondary_style);

        let pane_controls = column![
            button(text("Split Horizontal").color(GRUVBOX_TEXT))
                .on_press(Message::SplitHorizontal)
                .style(button_secondary_style),
            button(text("Split Vertical").color(GRUVBOX_TEXT))
                .on_press(Message::SplitVertical)
                .style(button_secondary_style),
            button(text("Close Pane").color(Color::WHITE))
                .on_press(Message::CloseFocusedPane)
                .style(button_danger_style),
        ]
        .spacing(8);

        let saved_papers_list = scrollable(
            column(
                self.saved_papers.iter().map(|paper| {
                    button(text(&paper.title).color(GRUVBOX_TEXT))
                        .on_press(Message::OpenPaperPane(paper.clone()))
                        .width(Length::Fill)
                        .style(button_secondary_style)
                        .into()
                }).collect::<Vec<Element<Message>>>()
            ).spacing(4)
        );

        container(
            column![
                toggle_button,
                horizontal_rule(2),
                text("Pane Controls").color(GRUVBOX_TEXT).size(16),
                pane_controls,
                horizontal_rule(2),
                text("Saved Papers").color(GRUVBOX_TEXT).size(16),
                saved_papers_list,
            ]
            .spacing(16)
            .padding(16)
        )
        .width(280)
        .height(Length::Fill)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_SURFACE)),
            border: Border {
                color: GRUVBOX_BORDER,
                width: 1.0,
                radius: 0.0.into(),
            },
            text_color: Some(GRUVBOX_TEXT),
            shadow: Shadow::default(),
        })
        .into()
    }

    pub fn search_view(&self) -> Element<Message> {
        let search_input = text_input("Search arXiv papers...", &self.search_query)
            .on_input(Message::SearchQueryChanged)
            .on_submit(Message::SearchSubmitted)
            .style(|_theme, status| iced::widget::text_input::Style {
                background: Background::Color(GRUVBOX_SURFACE),
                border: Border {
                    color: match status {
                        iced::widget::text_input::Status::Focused => GRUVBOX_GREEN,
                        _ => GRUVBOX_BORDER,
                    },
                    width: 1.0,
                    radius: 4.0.into(),
                },
                icon: Color::TRANSPARENT,
                placeholder: GRUVBOX_TEXT_MUTED,
                value: GRUVBOX_TEXT,
                selection: GRUVBOX_GREEN,
            });

        let search_button = button(text("Search").color(Color::BLACK))
            .on_press(Message::SearchSubmitted)
            .style(button_primary_style);

        let search_bar = row![search_input, search_button]
            .spacing(10)
            .padding(10);

        let content = if self.is_searching {
            column![text("Searching...").color(GRUVBOX_TEXT)]
        } else if let Some(error) = &self.search_error {
            column![
                text("Error:").color(GRUVBOX_RED),
                text(error).color(GRUVBOX_TEXT)
            ]
        } else if self.search_results.is_empty() {
            column![text("No results").color(GRUVBOX_TEXT_MUTED)]
        } else {
            column(
                self.search_results.iter().map(|paper| {
                    self.paper_card(paper, false)
                }).collect::<Vec<Element<Message>>>()
            ).spacing(10)
        };

        container(
            column![
                search_bar,
                horizontal_rule(1),
                scrollable(content).height(Length::Fill)
            ]
        )
        .padding(20)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_BG)),
            border: Border::default(),
            text_color: Some(GRUVBOX_TEXT),
            shadow: Shadow::default(),
        })
        .into()
    }

    pub fn library_view(&self) -> Element<Message> {
        let content = if self.saved_papers.is_empty() {
            column![text("No saved papers").color(GRUVBOX_TEXT_MUTED)]
        } else {
            column(
                self.saved_papers.iter().map(|paper| {
                    self.paper_card(paper, true)
                }).collect::<Vec<Element<Message>>>()
            ).spacing(10)
        };

        container(
            scrollable(content).height(Length::Fill)
        )
        .padding(20)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_BG)),
            border: Border::default(),
            text_color: Some(GRUVBOX_TEXT),
            shadow: Shadow::default(),
        })
        .into()
    }

    pub fn downloads_view(&self) -> Element<Message> {
        let content = if self.downloads.is_empty() {
            column![text("No downloads").color(GRUVBOX_TEXT_MUTED)]
        } else {
            column(
                self.downloads.iter().map(|download| {
                    self.download_card(download)
                }).collect::<Vec<Element<Message>>>()
            ).spacing(10)
        };

        container(
            scrollable(content).height(Length::Fill)
        )
        .padding(20)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_BG)),
            border: Border::default(),
            text_color: Some(GRUVBOX_TEXT),
            shadow: Shadow::default(),
        })
        .into()
    }

    pub fn settings_view(&self) -> Element<Message> {
        container(
            column![
                text("Settings").color(GRUVBOX_TEXT).size(24),
                vertical_space().height(20),
                text("Theme: Gruvbox Dark").color(GRUVBOX_TEXT),
            ]
        )
        .padding(20)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_BG)),
            border: Border::default(),
            text_color: Some(GRUVBOX_TEXT),
            shadow: Shadow::default(),
        })
        .into()
    }

    pub fn paper_view<'a>(&'a self, paper: &'a ArxivPaper) -> Element<'a, Message> {
        let title = text(&paper.title)
            .color(GRUVBOX_TEXT)
            .size(24);

        let authors = text(paper.authors.join(", "))
            .color(GRUVBOX_TEXT_MUTED)
            .size(14);

        let published = text(format!("Published: {}", paper.published))
            .color(GRUVBOX_TEXT_MUTED)
            .size(12);

        let categories = text(format!("Categories: {}", paper.categories.join(", ")))
            .color(GRUVBOX_TEXT_MUTED)
            .size(12);

        let abstract_text = text(&paper.abstract_text)
            .color(GRUVBOX_TEXT)
            .size(14);

        container(
            scrollable(
                column![
                    title,
                    authors,
                    published,
                    categories,
                    horizontal_rule(1),
                    text("Abstract").color(GRUVBOX_TEXT).size(18),
                    abstract_text,
                ]
                .spacing(10)
            )
        )
        .padding(20)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_BG)),
            border: Border::default(),
            text_color: Some(GRUVBOX_TEXT),
            shadow: Shadow::default(),
        })
        .into()
    }

    pub fn paper_card<'a>(&'a self, paper: &'a ArxivPaper, is_saved: bool) -> Element<'a, Message> {
        let title = text(&paper.title)
            .color(GRUVBOX_TEXT)
            .size(16);

        let authors = text(paper.authors.join(", "))
            .color(GRUVBOX_TEXT_MUTED)
            .size(12);

        let buttons = if is_saved {
            row![
                button(text("Remove").color(Color::WHITE))
                    .on_press(Message::RemovePaper(paper.id.clone()))
                    .style(button_danger_style),
                button(text("Download").color(Color::BLACK))
                    .on_press(Message::DownloadPaper(paper.clone()))
                    .style(button_primary_style),
                button(text("View").color(GRUVBOX_TEXT))
                    .on_press(Message::OpenPaperPane(paper.clone()))
                    .style(button_secondary_style),
            ]
        } else {
            row![
                button(text("Save").color(Color::BLACK))
                    .on_press(Message::SavePaper(paper.clone()))
                    .style(button_primary_style),
                button(text("Download").color(GRUVBOX_TEXT))
                    .on_press(Message::DownloadPaper(paper.clone()))
                    .style(button_secondary_style),
            ]
        }
        .spacing(8);

        container(
            column![
                title,
                authors,
                vertical_space().height(8),
                buttons,
            ]
            .spacing(4)
        )
        .padding(12)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_SURFACE)),
            border: Border {
                color: GRUVBOX_BORDER,
                width: 1.0,
                radius: 8.0.into(),
            },
            text_color: Some(GRUVBOX_TEXT),
            shadow: Shadow::default(),
        })
        .into()
    }

    pub fn download_card<'a>(&'a self, download: &'a DownloadItem) -> Element<'a, Message> {
        let title = text(&download.title)
            .color(GRUVBOX_TEXT)
            .size(14);

        let status_text = match &download.status {
            DownloadStatus::Pending => "Pending",
            DownloadStatus::Downloading => "Downloading",
            DownloadStatus::Completed => "Completed",
            DownloadStatus::Failed(_) => "Failed",
        };

        let status = text(status_text)
            .color(match download.status {
                DownloadStatus::Failed(_) => GRUVBOX_RED,
                DownloadStatus::Completed => GRUVBOX_GREEN,
                _ => GRUVBOX_TEXT_MUTED,
            })
            .size(12);

        let progress = if matches!(download.status, DownloadStatus::Downloading) {
            Some(progress_bar(0.0..=100.0, download.progress))
        } else {
            None
        };

        let mut content = column![title, status].spacing(4);
        
        if let Some(progress_bar) = progress {
            content = content.push(progress_bar);
        }

        container(content)
            .padding(12)
            .style(|_theme| iced::widget::container::Style {
                background: Some(Background::Color(GRUVBOX_SURFACE)),
                border: Border {
                    color: GRUVBOX_BORDER,
                    width: 1.0,
                    radius: 8.0.into(),
                },
                text_color: Some(GRUVBOX_TEXT),
                shadow: Shadow::default(),
            })
            .into()
    }
}
