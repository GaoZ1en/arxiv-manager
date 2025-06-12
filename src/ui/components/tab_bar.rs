// 现代化标签栏组件 - IRC客户端风格

use iced::widget::{button, row, text, container, scrollable, mouse_area};
use iced::{Element, Length, Alignment, Point};

use crate::core::app_state::ArxivManager;
use crate::core::models::TabContent;
use crate::core::messages::Message;
use crate::ui::style::{tab_active_dynamic_style, tab_inactive_dynamic_style, tab_close_dynamic_style, tab_bar_container_dynamic_style, scrollable_style_dynamic};

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
            
            // 创建标签页内容，包含标题和可选的固定/关闭按钮
            let mut tab_elements = vec![];
            
            // 添加固定指示器（如果标签页被固定）
            if tab.pinned {
                tab_elements.push(
                    text("*")
                        .size(base_font_size * 0.8)
                        .font(current_font)
                        .color(theme_colors.accent_border)
                        .into()
                );
            }
            
            // 添加分组指示器（如果不是默认分组）
            if tab.group != crate::core::models::ui::TabGroup::Default {
                let group_indicator = match &tab.group {
                    crate::core::models::ui::TabGroup::Research => "R",
                    crate::core::models::ui::TabGroup::Library => "L",
                    crate::core::models::ui::TabGroup::Downloads => "D",
                    crate::core::models::ui::TabGroup::Custom(_) => "C",
                    _ => "",
                };
                if !group_indicator.is_empty() {
                    tab_elements.push(
                        text(group_indicator)
                            .size(base_font_size * 0.8)
                            .font(current_font)
                            .color(theme_colors.text_muted)
                            .into()
                    );
                }
            }
            
            // Add title
            tab_elements.push(
                text(&tab.title)
                    .size(base_font_size * 0.93)
                    .font(current_font)
                    .color(if is_active { theme_colors.text_primary } else { theme_colors.text_secondary })
                    .into()
            );
            
            // Add close button only
            if tab.closable {
                tab_elements.push(
                    button(text("x").size(base_font_size).font(current_font))
                        .on_press(Message::TabClose(index))
                        .style(tab_close_dynamic_style(&app.settings.theme))
                        .padding([2.0 * scale, 4.0 * scale])
                        .into()
                );
            }
            
            let tab_content = row(tab_elements)
                .spacing(4.0 * scale)
                .align_y(Alignment::Center);
            
            let tab_button = if is_active {
                button(tab_content)
                    .on_press(Message::TabClicked(index))
                    .padding([2.0 * scale, 5.0 * scale])
                    .style(tab_active_dynamic_style(&app.settings.theme))
            } else {
                button(tab_content)
                    .on_press(Message::TabClicked(index))
                    .padding([2.0 * scale, 5.0 * scale])
                    .style(tab_inactive_dynamic_style(&app.settings.theme))
            };

            // Simple right-click support using mouse_area
            let tab_with_context = mouse_area(tab_button)
                .on_right_press(Message::TabRightClicked { 
                    tab_index: index, 
                    position: Point::new(200.0, 100.0) // 固定位置，简单但有效
                });

            tabs_row = tabs_row.push(tab_with_context);
        }

        // 不添加任何右侧按钮，保持标签页栏简洁

        // 使用可滚动容器承载标签页，应用现代化滚动条样式
        let scrollable_tabs = scrollable(tabs_row)
            .direction(scrollable::Direction::Horizontal(
                scrollable::Scrollbar::default()
                    .width(1) // 极细的滚动条
                    .margin(0)
                    .scroller_width(1)
            ))
            .style(scrollable_style_dynamic(&app.settings.theme))
            .width(Length::Fill);

        container(scrollable_tabs)
            .padding([1.0 * scale, 5.0 * scale])
            .width(Length::Fill)
            .style(tab_bar_container_dynamic_style(&app.settings.theme))
            .into()
    }
}
