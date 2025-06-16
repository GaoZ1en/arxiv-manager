// 论文卡片组件

use iced::widget::{button, column, row, text, vertical_space, progress_bar, container};
use iced::{Element, Color, Background, Border, Shadow};

use crate::core::app_state::ArxivManager;
use crate::core::models::{ArxivPaper, DownloadItem, DownloadStatus, TabContent};
use crate::core::messages::Message;
use crate::ui::style::{button_primary_style_dynamic, button_secondary_style_dynamic, button_danger_style_dynamic};

pub struct PaperCard;

impl PaperCard {
    /// 用于Search视图的论文卡片 - 包含Save和Download按钮
    pub fn search_view<'a>(app: &'a ArxivManager, paper: &'a ArxivPaper) -> Element<'a, Message> {
        Self::create_card_base(app, paper, Self::create_search_buttons(app, paper))
    }
    
    /// 用于Library视图的论文卡片 - 包含Remove、Download和View按钮
    pub fn library_view<'a>(app: &'a ArxivManager, paper: &'a ArxivPaper) -> Element<'a, Message> {
        Self::create_card_base(app, paper, Self::create_library_buttons(app, paper))
    }
    
    /// 兼容旧接口的方法（逐步废弃）
    pub fn view<'a>(app: &'a ArxivManager, paper: &'a ArxivPaper, is_saved: bool) -> Element<'a, Message> {
        if is_saved {
            Self::library_view(app, paper)
        } else {
            Self::search_view(app, paper)
        }
    }
    
    /// 创建卡片的基础结构
    fn create_card_base<'a>(app: &'a ArxivManager, paper: &'a ArxivPaper, buttons: Element<'a, Message>) -> Element<'a, Message> {
        let theme_colors = app.theme_colors();
        let current_font = app.current_font();
        let base_font_size = app.current_font_size();
        let scale = app.current_scale();
        
        let title = text(&paper.title)
            .color(theme_colors.text_primary)
            .size(base_font_size * 1.14)
            .font(current_font);

        // 显示 arXiv ID
        let arxiv_id = text(format!("arXiv:{}", &paper.id))
            .color(theme_colors.info_color)
            .size(base_font_size * 0.8)
            .font(current_font);

        let authors = Self::create_clickable_authors(app, &paper.authors);

        container(
            column![
                title,
                arxiv_id,
                authors,
                vertical_space().height(8.0 * scale),
                buttons,
            ]
            .spacing(4.0 * scale)
        )
        .width(iced::Length::Fill)  // 确保卡片填充可用宽度
        .padding(12.0 * scale)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(theme_colors.dark_bg_secondary)),
            border: Border {
                color: theme_colors.border_color,
                width: 1.0,
                radius: 8.0.into(),
            },
            text_color: Some(theme_colors.text_primary),
            shadow: Shadow::default(),
        })
        .into()
    }
     /// 创建Search视图的按钮 - Save和Open PDF（常驻）
    fn create_search_buttons<'a>(app: &'a ArxivManager, paper: &'a ArxivPaper) -> Element<'a, Message> {
        let theme_colors = app.theme_colors();
        let current_font = app.current_font();
        let base_font_size = app.current_font_size();
        let scale = app.current_scale();
        
        // 确定PDF按钮的文本
        let pdf_button_text = if paper.local_file_path.is_some() {
            "Open PDF"
        } else {
            "Download & Open PDF"
        };
        
        let pdf_button = if paper.local_file_path.is_some() {
            button(text(pdf_button_text).color(theme_colors.text_primary).size(base_font_size).font(current_font))
                .on_press(Message::OpenOrDownloadPdf(paper.clone()))
                .style(button_secondary_style_dynamic(&app.settings.theme))
                .padding([8.0 * scale, 8.0 * scale])
        } else {
            button(text(pdf_button_text).color(theme_colors.text_primary).size(base_font_size).font(current_font))
                .on_press(Message::OpenOrDownloadPdf(paper.clone()))
                .style(button_primary_style_dynamic(&app.settings.theme))
                .padding([8.0 * scale, 8.0 * scale])
        };
        
        let buttons = vec![
            button(text("Save").color(Color::BLACK).size(base_font_size).font(current_font))
                .on_press(Message::SavePaper(paper.clone()))
                .style(button_primary_style_dynamic(&app.settings.theme))
                .padding([8.0 * scale, 8.0 * scale])
                .into(),
            pdf_button.into(),
        ];
        
        row(buttons)
        .spacing(8.0 * scale)
        .into()
    }
    
    /// 创建Library视图的按钮 - Favorite、Remove、Open PDF（常驻）和View
    fn create_library_buttons<'a>(app: &'a ArxivManager, paper: &'a ArxivPaper) -> Element<'a, Message> {
        let theme_colors = app.theme_colors();
        let current_font = app.current_font();
        let base_font_size = app.current_font_size();
        let scale = app.current_scale();
        
        let favorite_text = if paper.is_favorite { "Unfavorite" } else { "Favorite" };
        let favorite_style = if paper.is_favorite {
            button_primary_style_dynamic(&app.settings.theme)
        } else {
            button_primary_style_dynamic(&app.settings.theme) // 使用相同的样式函数
        };
        
        // 确定PDF按钮的文本
        let pdf_button_text = if paper.local_file_path.is_some() {
            "Open PDF"
        } else {
            "Download & Open PDF"
        };
        
        let pdf_button = if paper.local_file_path.is_some() {
            button(text(pdf_button_text).color(theme_colors.text_primary).size(base_font_size).font(current_font))
                .on_press(Message::OpenOrDownloadPdf(paper.clone()))
                .style(button_secondary_style_dynamic(&app.settings.theme))
                .padding([8.0 * scale, 8.0 * scale])
        } else {
            button(text(pdf_button_text).color(theme_colors.text_primary).size(base_font_size).font(current_font))
                .on_press(Message::OpenOrDownloadPdf(paper.clone()))
                .style(button_primary_style_dynamic(&app.settings.theme))
                .padding([8.0 * scale, 8.0 * scale])
        };
        
        let buttons = vec![
            button(text(favorite_text).color(theme_colors.text_primary).size(base_font_size).font(current_font))
                .on_press(Message::TogglePaperFavorite(paper.id.clone()))
                .style(favorite_style)
                .padding([8.0 * scale, 8.0 * scale])
                .into(),
            button(text("Remove").color(Color::WHITE).size(base_font_size).font(current_font))
                .on_press(Message::RemovePaper(paper.id.clone()))
                .style(button_danger_style_dynamic(&app.settings.theme))
                .padding([8.0 * scale, 8.0 * scale])
                .into(),
            pdf_button.into(),
            button(text("View").color(theme_colors.text_primary).size(base_font_size).font(current_font))
                .on_press(if let Some(index) = app.saved_papers.iter().position(|p| p.id == paper.id) {
                    Message::NewTab(TabContent::PaperView(index))
                } else {
                    Message::NoOp
                })
                .style(button_secondary_style_dynamic(&app.settings.theme))
                .padding([8.0 * scale, 8.0 * scale])
                .into()
        ];
        
        row(buttons)
        .spacing(8.0 * scale)
        .into()
    }

    pub fn download_card<'a>(download: &'a DownloadItem, app: &'a ArxivManager) -> Element<'a, Message> {
        let theme_colors = app.theme_colors();
        let current_font = app.current_font();
        let base_font_size = app.current_font_size();
        let scale = app.current_scale();
        
        let title = text(&download.title)
            .color(theme_colors.text_primary)
            .size(base_font_size)
            .font(current_font);

        let status_text = match &download.status {
            DownloadStatus::Pending => "Pending",
            DownloadStatus::Downloading => "Downloading",
            DownloadStatus::Completed => "Completed",
            DownloadStatus::Failed(_) => "Failed",
        };

        let status = text(status_text)
            .color(match download.status {
                DownloadStatus::Failed(_) => theme_colors.error_color,
                DownloadStatus::Completed => theme_colors.success_color,
                _ => theme_colors.text_muted,
            })
            .size(base_font_size * 0.86)
            .font(current_font);

        let progress = if matches!(download.status, DownloadStatus::Downloading) {
            Some(progress_bar(0.0..=100.0, download.progress))
        } else {
            None
        };

        let mut content = column![title, status].spacing(4.0 * scale);
        
        if let Some(progress_bar) = progress {
            content = content.push(progress_bar);
        }

        container(content)
            .padding(12.0 * scale)
            .style(move |_theme| iced::widget::container::Style {
                background: Some(Background::Color(theme_colors.dark_bg_secondary)),
                border: Border {
                    color: theme_colors.border_color,
                    width: 1.0,
                    radius: 8.0.into(),
                },
                text_color: Some(theme_colors.text_primary),
                shadow: Shadow::default(),
            })
            .into()
    }

    /// 创建可点击的作者列表
    fn create_clickable_authors<'a>(app: &'a ArxivManager, authors: &'a [String]) -> Element<'a, Message> {
        Self::create_clickable_authors_with_limit(app, authors, Some(2))
    }

    /// 创建可点击的作者列表（带限制）
    fn create_clickable_authors_with_limit<'a>(
        app: &'a ArxivManager, 
        authors: &'a [String], 
        limit: Option<usize>
    ) -> Element<'a, Message> {
        let theme_colors = app.theme_colors();
        let current_font = app.current_font();
        let base_font_size = app.current_font_size();
        let _scale = app.current_scale(); // 可能在将来会用到

        if authors.is_empty() {
            return text("Unknown authors")
                .color(theme_colors.text_muted)
                .size(base_font_size * 0.86)
                .font(current_font)
                .into();
        }

        // 创建作者按钮的行，确保垂直对齐
        let mut author_row = row![].spacing(0).align_y(iced::Alignment::Center);
        
        // 决定要显示的作者数量
        let (display_authors, show_et_al) = match limit {
            Some(max_authors) if authors.len() > max_authors => {
                (&authors[..max_authors], true)
            },
            _ => (authors, false)
        };
        
        for (i, author) in display_authors.iter().enumerate() {
            // 添加逗号分隔符（除了第一个作者）
            if i > 0 {
                author_row = author_row.push(
                    container(
                        text(", ")
                            .color(theme_colors.text_muted)
                            .size(base_font_size * 0.86)
                            .font(current_font)
                    )
                    .align_y(iced::alignment::Vertical::Center)
                );
            }
            
            // 创建可点击的作者按钮，移除默认padding以确保对齐
            let author_button = button(
                text(author)
                    .color(theme_colors.accent_border)
                    .size(base_font_size * 0.86)
                    .font(current_font)
            )
            .padding(0) // 移除按钮的默认padding
            .on_press(Message::SearchByAuthor(author.clone()))
            .style(move |_theme, status| {
                let (background, text_color, border_color) = match status {
                    button::Status::Hovered => (
                        Some(Background::Color(Color::from_rgba(
                            theme_colors.accent_border.r,
                            theme_colors.accent_border.g,
                            theme_colors.accent_border.b,
                            0.1,
                        ))),
                        theme_colors.accent_border,
                        theme_colors.accent_border,
                    ),
                    button::Status::Pressed => (
                        Some(Background::Color(Color::from_rgba(
                            theme_colors.accent_border.r,
                            theme_colors.accent_border.g,
                            theme_colors.accent_border.b,
                            0.2,
                        ))),
                        theme_colors.text_primary,
                        theme_colors.accent_border,
                    ),
                    _ => (
                        None,
                        theme_colors.accent_border,
                        Color::TRANSPARENT,
                    ),
                };
                
                button::Style {
                    background,
                    text_color,
                    border: Border {
                        color: border_color,
                        width: if border_color == Color::TRANSPARENT { 0.0 } else { 1.0 },
                        radius: 2.0.into(), // 减小圆角半径
                    },
                    shadow: Shadow::default(),
                }
            });
            
            author_row = author_row.push(author_button);
        }

        // 如果需要，添加 "et al."
        if show_et_al {
            author_row = author_row.push(
                container(
                    text(", et al.")
                        .color(theme_colors.text_muted)
                        .size(base_font_size * 0.86)
                        .font(current_font)
                )
                .align_y(iced::alignment::Vertical::Center)
            );
        }

        // 将作者行包装在容器中以便于对齐
        container(author_row)
            .width(iced::Length::Fill)
            .into()
    }
}
