// 现代化标签栏组件 - IRC客户端风格

use iced::widget::{button, row, text, container, scrollable, mouse_area};
use iced::{Element, Length, Alignment, Point};

use crate::core::app_state::ArxivManager;
use crate::core::messages::Message;
use crate::ui::style::{tab_active_dynamic_style, tab_inactive_dynamic_style, tab_close_dynamic_style, tab_bar_container_dynamic_style, scrollable_tab_style_dynamic_with_fade, ultra_thin_horizontal_scrollbar};

pub struct TabBar;

impl TabBar {
    pub fn view(app: &ArxivManager) -> Element<'_, Message> {
        let theme_colors = app.theme_colors();
        let current_font = app.current_font();
        let base_font_size = app.current_font_size();
        let scale = app.current_scale();
        let mut tabs_row = row![].spacing(4.0 * scale);

        for (index, tab) in app.tabs.iter().enumerate() {
            let is_active = index == app.active_tab;
            
            // 简化的标签页布局：只有标题和关闭按钮，保持一致性
            let mut tab_elements = vec![];
            
            // 主标题区域 - 统一高度布局
            let display_title = tab.title.clone();
            
            tab_elements.push(
                container(
                    text(display_title)
                        .size(base_font_size * 0.93)
                        .font(current_font)
                        .color(if is_active { theme_colors.text_primary } else { theme_colors.text_secondary })
                )
                .width(Length::Fill)
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Center)  // 垂直居中
                .padding([0.0, 4.0 * scale])
                .height(Length::Fixed(18.0 * scale))  // 进一步减小高度
                .into()
            );
            
            // 关闭按钮区域（固定宽度）
            if tab.closable {
                tab_elements.push(
                    container(
                        button(text("×").size(base_font_size * 0.9).font(current_font))
                            .on_press(Message::TabClose(index))
                            .style(tab_close_dynamic_style(&app.settings.theme))
                            .padding([1.0 * scale, 3.0 * scale])
                    )
                    .width(Length::Fixed(24.0 * scale)) // 较小的固定宽度
                    .align_x(iced::alignment::Horizontal::Center)
                    .into()
                );
            } else {
                // 为不可关闭的标签页保留相同的空间
                tab_elements.push(
                    container(text(""))
                        .width(Length::Fixed(24.0 * scale))
                        .into()
                );
            }
            
            let tab_content = row(tab_elements)
                .spacing(2.0 * scale)
                .align_y(Alignment::Center);
            
            // 统一的标签页宽度和样式 - 更紧凑的设计
            let tab_width = 100.0 * scale; // 进一步减小宽度
            
            let tab_button = if is_active {
                button(tab_content)
                    .on_press(Message::TabClicked(index))
                    .width(Length::Fixed(tab_width))
                    .padding([3.0 * scale, 4.0 * scale])  // 进一步减小内边距
                    .style(tab_active_dynamic_style(&app.settings.theme))
            } else {
                button(tab_content)
                    .on_press(Message::TabClicked(index))
                    .width(Length::Fixed(tab_width))
                    .padding([3.0 * scale, 4.0 * scale])  // 进一步减小内边距
                    .style(tab_inactive_dynamic_style(&app.settings.theme))
            };

            // 右键菜单支持
            let tab_with_context = mouse_area(tab_button)
                .on_right_press(Message::TabRightClicked { 
                    tab_index: index, 
                    position: Point::new(200.0, 100.0)
                });

            tabs_row = tabs_row.push(tab_with_context);
        }

        // 不添加任何右侧按钮，保持标签页栏简洁

        // 使用可滚动容器承载标签页，应用现代化滚动条样式
        let scrollable_tabs = scrollable(tabs_row)
            .direction(ultra_thin_horizontal_scrollbar())
            .style(scrollable_tab_style_dynamic_with_fade(
                &app.settings.theme, 
                app.get_scrollbar_alpha("tab_bar")
            ))
            .on_scroll(|_| Message::ScrollbarActivity("tab_bar".to_string()))
            .width(Length::Fill);

        container(scrollable_tabs)
            .padding(0) // 完全去掉padding，贴边显示
            .width(Length::Fill)
            .style(tab_bar_container_dynamic_style(&app.settings.theme))
            .into()
    }
}
