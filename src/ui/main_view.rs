// 现代化主视图 - IRC客户端风格布局

use iced::widget::{column, container, row, stack, text};
use iced::{Element, Length};

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
            // 美观的空白界面
            let theme_colors = self.theme_colors();
            let current_font = self.current_font();
            let base_font_size = self.current_font_size();
            let scale = self.current_scale();
            
            container(
                column![
                    // 大型欢迎图标
                    text("📚")
                        .size(base_font_size * 4.0)
                        .color(theme_colors.accent_border),
                    
                    // 主标题
                    text("ArXiv Manager")
                        .size(base_font_size * 1.8)
                        .font(iced::Font {
                            weight: iced::font::Weight::Bold,
                            ..current_font
                        })
                        .color(theme_colors.text_primary),
                    
                    // 副标题
                    text("Modern Research Paper Management")
                        .size(base_font_size * 1.1)
                        .color(theme_colors.text_secondary),
                    
                    // 分隔空间
                    iced::widget::vertical_space().height(32.0 * scale),
                    
                    // 快速操作提示
                    column![
                        text("Get Started:")
                            .size(base_font_size * 1.2)
                            .font(iced::Font {
                                weight: iced::font::Weight::Medium,
                                ..current_font
                            })
                            .color(theme_colors.text_primary),
                        
                        iced::widget::vertical_space().height(16.0 * scale),
                        
                        // 操作提示列表
                        column![
                            text("• Click 'SEARCH' to find papers")
                                .size(base_font_size)
                                .color(theme_colors.text_secondary),
                            text("• Browse your 'LIBRARY' for saved papers")
                                .size(base_font_size)
                                .color(theme_colors.text_secondary),
                            text("• Check 'DOWNLOADS' for active transfers")
                                .size(base_font_size)
                                .color(theme_colors.text_secondary),
                            text("• Customize in 'SETTINGS'")
                                .size(base_font_size)
                                .color(theme_colors.text_secondary),
                        ]
                        .spacing(8.0 * scale)
                    ]
                    .spacing(8.0 * scale)
                ]
                .spacing(16.0 * scale)
                .align_x(iced::Alignment::Center)
                .padding(48.0 * scale)
            )
            .style(chat_container_dynamic_style(&self.settings.theme))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
        };

        // 创建带标签栏的内容区域 (类似IRC的消息区域)
        let content_area = container(
            column![
                // 顶部标签栏 (类似IRC的频道标签) - 无内边距，直接贴边
                TabBar::view(self),
                
                // 主内容区域 - 无内边距，直接贴边
                container(current_content)
                    .height(Length::Fill)
                    .width(Length::Fill)
            ]
        )
        .style(main_container_dynamic_style(&self.settings.theme))
        .height(Length::Fill)
        .width(Length::Fill);

        // 组合侧边栏和内容区域 - 无间距，直接贴合
        let base_layout: Element<Message> = if let Some(sidebar) = sidebar {
            row![
                sidebar,
                content_area
            ]
            .spacing(0) // 完全无间距，直接贴合
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        } else {
            content_area.into()
        };

        // 如果命令面板或右键菜单可见，添加覆盖层 (类似IRC的快速搜索)
        if self.command_palette_visible || self.context_menu.visible {
            let mut overlays = vec![base_layout];
            
            if self.command_palette_visible {
                overlays.push(CommandPalette::view(self));
            }
            
            if self.context_menu.visible {
                overlays.push(crate::ui::components::ContextMenu::view(&self.context_menu, self));
            }
            
            container(stack(overlays))
                .style(main_container_dynamic_style(&self.settings.theme))
                .into()
        } else {
            container(base_layout)
                .style(main_container_dynamic_style(&self.settings.theme))
                .into()
        }
    }
}
