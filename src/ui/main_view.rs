// 主视图集成文件

use iced::widget::{column, container, row, stack};
use iced::{Element, Length};

use crate::core::app_state::ArxivManager;
use crate::core::models::TabContent;
use crate::core::messages::Message;
use crate::ui::components::{TabBar, Sidebar, CommandPalette};
use crate::ui::views::{SearchView, LibraryView, DownloadsView, SettingsView, PaperView};

impl ArxivManager {
    pub fn view(&self) -> Element<Message> {
        let sidebar = if self.sidebar_visible {
            Some(Sidebar::view(self))
        } else {
            None
        };

        // 创建标签栏
        let tab_bar = TabBar::view(self);

        // 获取当前活动标签页的内容
        let current_content = if let Some(current_tab) = self.tabs.get(self.active_tab) {
            match &current_tab.content {
                TabContent::Search => SearchView::view(self),
                TabContent::Library => LibraryView::view(self),
                TabContent::Downloads => DownloadsView::view(self),
                TabContent::Settings => SettingsView::view(self),
                TabContent::PaperView(index) => {
                    if let Some(paper) = self.saved_papers.get(*index) {
                        PaperView::view(paper)
                    } else {
                        container(iced::widget::text("Paper not found")).into()
                    }
                }
            }
        } else {
            container(iced::widget::text("No active tab")).into()
        };

        // 创建主内容区域
        let main_content = container(
            column![
                tab_bar,
                current_content
            ]
            .spacing(0)
        )
        .width(Length::Fill)
        .height(Length::Fill);

        let base_layout = if let Some(sidebar) = sidebar {
            row![sidebar, main_content]
                .spacing(0)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        } else {
            main_content.into()
        };

        // 如果命令栏可见，添加覆盖层
        if self.command_palette_visible {
            let overlay = CommandPalette::view(self);
            container(stack![base_layout, overlay]).into()
        } else {
            base_layout
        }
    }
}
