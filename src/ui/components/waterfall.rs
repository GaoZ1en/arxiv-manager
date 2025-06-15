// 瀑布流布局组件
// 用于论文卡片的瀑布流显示，支持响应式列数调整

use iced::widget::{container, column, row};
use iced::{Element, Length};

use crate::core::messages::Message;
use crate::core::models::ArxivPaper;
use crate::core::app_state::ArxivManager;
use crate::ui::components::PaperCard;

pub struct WaterfallLayout;

impl WaterfallLayout {
    /// 创建智能瀑布流布局用于搜索视图
    pub fn search_view<'a>(app: &'a ArxivManager, papers: &'a [ArxivPaper]) -> Element<'a, Message> {
        Self::create_layout(app, papers, false)
    }
    
    /// 创建智能瀑布流布局用于图书馆视图
    pub fn library_view<'a>(app: &'a ArxivManager, papers: &'a [ArxivPaper]) -> Element<'a, Message> {
        Self::create_layout(app, papers, true)
    }
    
    /// 兼容旧接口的方法（逐步废弃）
    pub fn view<'a>(app: &'a ArxivManager, papers: &'a [ArxivPaper]) -> Element<'a, Message> {
        // 默认按照图书馆视图处理（因为大多数现有调用都是图书馆视图）
        Self::library_view(app, papers)
    }
    
    /// 创建智能瀑布流布局，自动计算列数和平衡分配
    fn create_layout<'a>(app: &'a ArxivManager, papers: &'a [ArxivPaper], is_library_view: bool) -> Element<'a, Message> {
        if papers.is_empty() {
            return container(
                iced::widget::text("No papers to display")
                    .size(app.current_font_size())
                    .font(app.current_font())
                    .color(app.theme_colors().text_muted)
            )
            .padding(32.0 * app.current_scale())
            .width(Length::Fill)
            .into();
        }

        let scale = app.current_scale();
        
        // 智能计算列数：基于真实的可用宽度
        // 设置每列最小宽度为300px，确保能看到多列效果
        let min_column_width = 300.0 * scale;
        let estimated_available_width = Self::estimate_available_width(app);
        let columns = Self::calculate_optimal_columns(estimated_available_width, min_column_width);
        
        log::info!("Waterfall layout debug: window_width={:.0}px, sidebar_visible={}, available_width={:.0}px, min_column_width={:.0}px, columns={}", 
                   app.window_width, app.sidebar_visible, estimated_available_width, min_column_width, columns);
        
        // 使用高度平衡算法分配论文到列
        let column_papers = Self::balance_papers_by_height(papers, columns);
        
        // 创建列，确保每个卡片等宽填充
        let columns_elements: Vec<Element<Message>> = column_papers
            .into_iter()
            .map(|papers_in_column| {
                let cards: Vec<Element<Message>> = papers_in_column
                    .into_iter()
                    .map(|paper| {
                        // 根据视图类型使用正确的论文卡片
                        let card = if is_library_view {
                            PaperCard::library_view(app, paper)
                        } else {
                            PaperCard::search_view(app, paper)
                        };
                        
                        // 为每个卡片添加适当的间距
                        container(card)
                            .width(Length::Fill)
                            .padding([6.0 * scale, 8.0 * scale]) // 上下6px，左右8px
                            .into()
                    })
                    .collect();
                
                container(
                    column(cards).spacing(12.0 * scale)
                )
                .width(Length::FillPortion(1)) // 均等分配宽度
                .into()
            })
            .collect();
        
        // 创建响应式瀑布流容器
        container(
            row(columns_elements)
                .spacing(16.0 * scale)
                .width(Length::Fill)
        )
        .width(Length::Fill)
        .into()
    }
    
    /// 估算可用宽度（基于真实的窗口尺寸）
    fn estimate_available_width(app: &ArxivManager) -> f32 {
        let scale = app.current_scale();
        // 使用真实的窗口宽度
        let window_width = app.window_width;
        
        // 减去侧边栏宽度（如果可见的话）
        let sidebar_width = if app.sidebar_visible { 
            window_width * 0.3 // 侧边栏占30%
        } else { 
            0.0 
        };
        
        // 减去padding、边距和其他UI元素占用的空间
        // 包括：主容器padding、分隔线、滚动条、论文面板padding等
        let ui_overhead = 80.0 * scale; // scale应用在UI开销上
        
        // 计算论文面板的可用宽度
        let available_width = window_width - sidebar_width - ui_overhead;
        
        // 确保有最小可用宽度，至少能容纳一列
        let min_available = 300.0 * scale;
        available_width.max(min_available)
    }
    
    /// 计算最佳列数
    fn calculate_optimal_columns(available_width: f32, min_column_width: f32) -> usize {
        // 计算理论上可以容纳的列数
        let theoretical_columns = (available_width / min_column_width).floor() as usize;
        
        // 确保至少有1列，最多不超过4列（600px最小宽度下，4列已经很宽了）
        theoretical_columns.max(1).min(4)
    }
    
    /// 使用高度估算算法平衡分配论文到各列
    fn balance_papers_by_height(papers: &[ArxivPaper], columns: usize) -> Vec<Vec<&ArxivPaper>> {
        let mut column_papers: Vec<Vec<&ArxivPaper>> = vec![Vec::new(); columns];
        let mut column_heights: Vec<usize> = vec![0; columns];
        
        // 根据估算高度将论文分配到高度最小的列
        for paper in papers {
            // 找到当前高度最小的列
            let min_height_index = column_heights
                .iter()
                .enumerate()
                .min_by_key(|(_, &height)| height)
                .map(|(index, _)| index)
                .unwrap_or(0);
            
            column_papers[min_height_index].push(paper);
            
            // 估算论文卡片高度：基于标题长度、摘要长度和作者数量
            let estimated_height = Self::estimate_paper_card_height(paper);
            column_heights[min_height_index] += estimated_height;
        }
        
        column_papers
    }
    
    /// 估算单个论文卡片的高度
    fn estimate_paper_card_height(paper: &ArxivPaper) -> usize {
        // 基础高度（包含标题、作者、按钮等固定元素）
        let base_height = 200;
        
        // 基于标题长度的额外高度（每40字符约增加20px）
        let title_height = (paper.title.len() / 40) * 20;
        
        // 基于摘要长度的额外高度（每100字符约增加15px，最多显示200字符）
        let summary_display_length = paper.abstract_text.len().min(200);
        let summary_height = (summary_display_length / 100) * 15;
        
        // 基于作者数量的额外高度（每个作者约5px）
        let authors_height = paper.authors.len() * 5;
        
        base_height + title_height + summary_height + authors_height
    }
    
    /// 兼容旧接口的方法（用于其他地方可能的调用）
    pub fn calculate_columns(container_width: f32, min_card_width: f32) -> usize {
        Self::calculate_optimal_columns(container_width, min_card_width)
    }
}