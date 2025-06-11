// 瀑布流布局组件
// 用于论文卡片的瀑布流显示

use iced::widget::{container, column, row};
use iced::{Element, Length};

use crate::core::messages::Message;
use crate::core::models::ArxivPaper;
use crate::core::app_state::ArxivManager;
use crate::ui::components::PaperCard;

pub struct WaterfallLayout;

impl WaterfallLayout {
    /// 创建瀑布流布局
    pub fn view<'a>(app: &'a ArxivManager, papers: &'a [ArxivPaper], columns: usize) -> Element<'a, Message> {
        if papers.is_empty() {
            return container(
                iced::widget::text("No papers to display")
                    .size(app.current_font_size())
                    .font(app.current_font())
            ).into();
        }

        let scale = app.current_scale();
        
        // 将论文分配到不同的列中
        let mut column_papers: Vec<Vec<&ArxivPaper>> = vec![Vec::new(); columns];
        
        // 简单的轮询分配（更复杂的实现可以考虑卡片高度）
        for (index, paper) in papers.iter().enumerate() {
            let column_index = index % columns;
            column_papers[column_index].push(paper);
        }
        
        // 创建列，确保每个卡片等宽填充
        let columns_elements: Vec<Element<Message>> = column_papers
            .into_iter()
            .map(|papers_in_column| {
                let cards: Vec<Element<Message>> = papers_in_column
                    .into_iter()
                    .map(|paper| {
                        // 确保每个卡片都填充列的宽度
                        container(PaperCard::view(app, paper, false))
                            .width(Length::Fill)
                            .into()
                    })
                    .collect();
                
                container(
                    column(cards).spacing(12.0 * scale)
                )
                .width(Length::Fill)
                .into()
            })
            .collect();
        
        // 使用固定宽度比例确保等宽分布
        container(
            row(columns_elements)
                .spacing(16.0 * scale)
                .width(Length::Fill)
        )
        .width(Length::Fill)
        .into()
    }
    
    /// 根据屏幕宽度自动计算合适的列数
    pub fn calculate_columns(container_width: f32, min_card_width: f32) -> usize {
        let columns = (container_width / min_card_width).floor() as usize;
        // 至少1列，最多4列
        columns.max(1).min(4)
    }
}