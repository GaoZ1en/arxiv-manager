use iced::{
    widget::{column, button, text, Space},
    Element, Length, Alignment,
};

use crate::app::{ArxivManager, View, Message};
use crate::ui::style;

pub struct Sidebar;

impl Sidebar {
    /// 创建侧边栏视图
    pub fn view(app: &ArxivManager) -> Element<'static, Message> {
        Self::create(app)
    }

    /// 创建侧边栏
    fn create(app: &ArxivManager) -> Element<Message> {
        let current_view = *app.current_view();

        column![
            // 应用标题
            text("arXiv 管理器")
                .size(20)
                .style(style::text::title()),
            
            Space::with_height(30),
            
            // 导航按钮
            nav_button("🔍 搜索", View::Search, current_view),
            nav_button("📚 文献库", View::Library, current_view),
            nav_button("⬇️ 下载", View::Downloads, current_view),
            nav_button("⚙️ 设置", View::Settings, current_view),
            
            Space::with_height(Length::Fill),
            
            // 底部信息
            text("v0.1.0")
                .size(12)
                .style(style::text::muted()),
        ]
        .spacing(10)
        .padding(20)
        .width(250)
        .height(Length::Fill)
        .align_items(Alignment::Start)
        .into()
    }
}

/// 导航按钮
fn nav_button(label: &str, view: View, current_view: View) -> Element<Message> {
    let is_active = view == current_view;
    
    button(
        text(label)
            .size(14)
            .style(if is_active {
                style::text::title()
            } else {
                style::text::body()
            })
    )
    .width(Length::Fill)
    .padding([12, 16])
    .style(if is_active {
        style::button::sidebar_item_active()
    } else {
        style::button::sidebar_item()
    })
    .on_press(Message::ChangeView(view))
    .into()
}
