// 现代化主视图 - IRC客户端风格布局

use iced::widget::{column, container, row, stack};
use iced::{Element, Length, Padding};

use crate::core::app_state::ArxivManager;
use crate::core::models::TabContent;
use crate::core::messages::Message;
use crate::ui::components::{TabBar, Sidebar, CommandPalette};
use crate::ui::views::{SearchView, LibraryView, DownloadsView, SettingsView, PaperView};
use crate::ui::style::{main_container_dynamic_style, chat_container_dynamic_style};

impl ArxivManager {
    pub fn view(&self) -> Element<'_, Message> {
        // 创建侧边栏 (类似IRC的频道列表)
        let sidebar = if self.sidebar_visible {
            Some(Sidebar::view(self))
        } else {
            None
        };

        // 获取当前活动标签页的内容
        let current_content = if let Some(current_tab) = self.tabs.get(self.active_tab) {
            match &current_tab.content {
                TabContent::Search => SearchView::view(self),
                TabContent::Library => LibraryView::view(self),
                TabContent::Downloads => DownloadsView::view(self),
                TabContent::Settings => SettingsView::view(self),
                TabContent::PaperView(index) => {
                    if let Some(paper) = self.saved_papers.get(*index) {
                        PaperView::view(paper, self)
                    } else {
                        let theme_colors = self.theme_colors();
                        let current_font = self.current_font();
                        let base_font_size = self.current_font_size();
                        container(
                            iced::widget::text("Paper not found")
                                .color(theme_colors.text_muted)
                                .size(base_font_size)
                                .font(current_font)
                        )
                        .style(chat_container_dynamic_style(&self.settings.theme))
                        .into()
                    }
                }
            }
        } else {
            container(
                iced::widget::text("No active tab")
                    .color(crate::ui::theme::TEXT_MUTED)
            )
            .style(chat_container_dynamic_style(&self.settings.theme))
            .into()
        };

        // 创建带标签栏的内容区域 (类似IRC的消息区域)
        let content_area = container(
            column![
                // 顶部标签栏 (类似IRC的频道标签)
                container(TabBar::view(self))
                    .padding(Padding::new(8.0)),
                
                // 主内容区域
                container(current_content)
                    .padding(Padding::new(12.0))
                    .height(Length::Fill)
                    .width(Length::Fill)
            ]
        )
        .style(main_container_dynamic_style(&self.settings.theme))
        .height(Length::Fill)
        .width(Length::Fill);

        // 组合侧边栏和内容区域
        let base_layout: Element<Message> = if let Some(sidebar) = sidebar {
            row![
                sidebar,
                content_area
            ]
            .spacing(0)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        } else {
            content_area.into()
        };

        // 如果命令面板可见，添加覆盖层 (类似IRC的快速搜索)
        if self.command_palette_visible {
            let overlay = CommandPalette::view(self);
            container(stack![base_layout, overlay])
                .style(main_container_dynamic_style(&self.settings.theme))
                .into()
        } else {
            container(base_layout)
                .style(main_container_dynamic_style(&self.settings.theme))
                .into()
        }
    }
}
