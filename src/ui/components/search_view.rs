use iced::{
    widget::{column, row, text, text_input, button, scrollable, Space, container},
    Element, Length, Alignment,
};

use crate::app::{ArxivManager, Message};
use crate::ui::{style, view};
use super::paper_card;

pub struct SearchView;

impl SearchView {
    /// 创建搜索视图
    pub fn view(app: &ArxivManager) -> Element<'static, Message> {
        Self::create(app)
    }

    /// 创建搜索视图
    fn create(app: &ArxivManager) -> Element<Message> {
        let search_section = column![
        // 搜索标题
        text("搜索 arXiv 论文")
            .size(28)
            .style(style::text::title()),
        
        Space::with_height(20),
        
        // 搜索输入框
        row![
            text_input("输入关键词、作者或 arXiv ID...", app.search_query())
                .on_input(Message::SearchInputChanged)
                .on_submit(Message::SearchSubmitted)
                .padding(12)
                .size(16)
                .width(Length::Fill)
                .style(style::text_input::default()),
            
            button(
                text("搜索")
                    .style(style::text::button())
            )
            .padding([12, 20])
            .style(style::button::primary())
            .on_press(Message::SearchSubmitted),
        ]
        .spacing(10)
        .align_items(Alignment::Center),
        
        Space::with_height(30),
    ]
    .spacing(10)
    .width(Length::Fill)
    .align_items(Alignment::Start);

    let content = if app.is_loading() {
        column![
            search_section,
            view::loading_indicator(),
        ]
    } else if app.papers().is_empty() && app.search_query().is_empty() {
        column![
            search_section,
            view::empty_state("开始搜索 arXiv 论文", "🔍"),
        ]
    } else if app.papers().is_empty() {
        column![
            search_section,
            view::empty_state("未找到相关论文", "📄"),
        ]
    } else {
        column![
            search_section,
            
            // 搜索结果
            text(format!("找到 {} 篇论文", app.papers().len()))
                .size(16)
                .style(style::text::subtitle()),
            
            Space::with_height(20),
            
            // 论文列表
            results_list(app),
        ]
    };

    content
        .spacing(10)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

/// 搜索结果列表
fn results_list(app: &ArxivManager) -> Element<Message> {
    let papers = app.papers();
    
    let mut items = column![];
    
    for (index, paper) in papers.iter().enumerate() {
        let is_selected = app.selected_paper()
            .map_or(false, |selected| selected.arxiv_id == paper.arxiv_id);
        
        items = items.push(
            paper_card::create(paper, index, is_selected, true)
        );
        
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
    .style(style::scrollable::default())
    .into()
}
