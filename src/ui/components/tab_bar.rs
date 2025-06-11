// 现代化标签栏组件 - IRC客户端风格

use iced::widget::{button, row, text, horizontal_space, container};
use iced::{Element, Length, Alignment};

use crate::core::app_state::ArxivManager;
use crate::core::models::TabContent;
use crate::core::messages::Message;
use crate::ui::style::{button_secondary_style_dynamic, tab_active_dynamic_style, tab_inactive_dynamic_style, tab_close_dynamic_style, tab_bar_container_dynamic_style};

pub struct TabBar;

impl TabBar {
    pub fn view(app: &ArxivManager) -> Element<'_, Message> {
        let theme_colors = app.theme_colors();
        let mut tabs_row = row![].spacing(4);

        for (index, tab) in app.tabs.iter().enumerate() {
            let is_active = index == app.active_tab;
            
            let tab_content = row![
                text(&tab.title)
                    .size(13)
                    .color(if is_active { theme_colors.text_primary } else { theme_colors.text_secondary }),
                if tab.closable {
                    button(text("×").size(14))
                        .on_press(Message::TabClose(index))
                        .style(tab_close_dynamic_style(&app.settings.theme))
                        .padding([2, 4])
                        .into()
                } else {
                    Element::<Message>::from(horizontal_space().width(0))
                }
            ]
            .spacing(8)
            .align_y(Alignment::Center);
            
            let tab_button = if is_active {
                button(tab_content)
                    .on_press(Message::TabClicked(index))
                    .padding([10, 16])
                    .style(tab_active_dynamic_style(&app.settings.theme))
            } else {
                button(tab_content)
                    .on_press(Message::TabClicked(index))
                    .padding([10, 16])
                    .style(tab_inactive_dynamic_style(&app.settings.theme))
            };

            tabs_row = tabs_row.push(tab_button);
        }

        // 添加新标签页按钮
        let new_tab_button = button(
            text("+")
                .size(16)
                .color(theme_colors.text_secondary)
        )
        .on_press(Message::NewTab(TabContent::Search))
        .padding([10, 12])
        .style(button_secondary_style_dynamic(&app.settings.theme));

        tabs_row = tabs_row.push(horizontal_space()).push(new_tab_button);

        container(tabs_row)
            .padding([8, 16])
            .width(Length::Fill)
            .style(tab_bar_container_dynamic_style(&app.settings.theme))
            .into()
    }
}
