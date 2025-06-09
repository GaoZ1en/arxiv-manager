use iced::{
    widget::{column, row, text, button, scrollable, Space, container},
    Element, Length, Alignment,
};

use crate::app::{ArxivManager, Message};
use crate::ui::{style, view};
use super::paper_card;

pub struct LibraryView;

impl LibraryView {
    /// 创建文献库视图
    pub fn view(app: &ArxivManager) -> Element<Message> {
        let header = row![
            // 标题
            text("我的文献库")
                .size(28),
            
            Space::with_width(Length::Fill),
            
            // 刷新按钮
            button("🔄 刷新")
                .on_press(Message::LoadData)
                .style(style::button::secondary),
        ]
        .align_items(Alignment::Center)
        .width(Length::Fill);

        let content = if app.papers().is_empty() {
            column![
                header,
                Space::with_height(50),
                empty_library_state(),
            ]
        } else {
            column![
                header,
                Space::with_height(20),
                
                // 统计信息
                stats_section(app),
                
                Space::with_height(20),
                
                // 论文列表
                library_list(app),
            ]
        };

        content
            .spacing(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

/// 空状态
fn empty_library_state() -> Element<'static, Message> {
    container(
        column![
            text("📚")
                .size(48),
            Space::with_height(20),
            text("还没有保存的论文")
                .size(16),
            Space::with_height(10),
            text("从搜索页面添加您感兴趣的论文到文献库")
                .size(14),
            Space::with_height(20),
            button("前往搜索")
                .on_press(Message::ChangeView(crate::app::View::Search))
                .style(style::button::primary),
        ]
        .align_items(Alignment::Center)
        .spacing(10)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y()
    .into()
}

/// 统计信息区域
fn stats_section(app: &ArxivManager) -> Element<Message> {
    let papers = app.papers();
    let total = papers.len();
    let downloaded = papers.iter().filter(|p| p.downloaded).count();
    let favorites = papers.iter().filter(|p| p.favorite).count();
    let unread = papers.iter().filter(|p| !p.read).count();

    row![
        stat_card("总论文", &total.to_string(), "📄"),
        stat_card("已下载", &downloaded.to_string(), "⬇️"),
        stat_card("收藏", &favorites.to_string(), "⭐"),
        stat_card("未读", &unread.to_string(), "📖"),
    ]
    .spacing(20)
    .width(Length::Fill)
    .into()
}

/// 统计卡片
fn stat_card(label: &str, value: &str, icon: &str) -> Element<'static, Message> {
    container(
        column![
            row![
                text(icon).size(24),
                Space::with_width(10),
                text(value).size(24),
            ]
            .align_items(Alignment::Center),
            
            Space::with_height(5),
            
            text(label).size(14),
        ]
        .align_items(Alignment::Center)
        .spacing(5)
    )
    .width(Length::Fill)
    .padding(20)
    .style(style::container::card)
    .into()
}

/// 文献库论文列表
fn library_list(app: &ArxivManager) -> Element<Message> {
    let papers = app.papers();
    
    let mut items = column![];
    
    for (index, paper) in papers.iter().enumerate() {
        let paper_element = container(
            column![
                row![
                    column![
                        text(&paper.title).size(16),
                        Space::with_height(5),
                        text(format!("arXiv:{}", &paper.arxiv_id)).size(12),
                        text(format!("作者: {}", paper.authors.join(", "))).size(12),
                    ]
                    .width(Length::Fill),
                    
                    column![
                        if paper.downloaded {
                            button("📖 阅读")
                                .on_press(Message::OpenLocalPaper(paper.arxiv_id.clone()))
                                .style(style::button::primary)
                        } else {
                            button("⬇️ 下载")
                                .on_press(Message::DownloadStoredPaper(paper.arxiv_id.clone()))
                                .style(style::button::secondary)
                        },
                        Space::with_height(5),
                        button(if paper.favorite { "⭐" } else { "☆" })
                            .on_press(Message::ToggleFavorite(paper.arxiv_id.clone()))
                            .style(style::button::text),
                    ]
                    .align_items(Alignment::End)
                    .spacing(5),
                ]
                .align_items(Alignment::Start)
                .spacing(10),
            ]
            .padding(15)
        )
        .width(Length::Fill)
        .style(style::container::card);
        
        items = items.push(paper_element);
        
        if index < papers.len() - 1 {
            items = items.push(Space::with_height(10));
        }
    }

    scrollable(
        container(items)
            .width(Length::Fill)
            .padding(10)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
