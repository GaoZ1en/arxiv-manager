// 设置视图模块 - 重构后的模块化架构

pub mod appearance;
pub mod downloads;
pub mod shortcuts;
pub mod components;

use iced::widget::{column, container, scrollable, text, vertical_space};
use iced::{Element, Background, Border, Shadow};

use crate::core::app_state::ArxivManager;
use crate::core::messages::Message;
use crate::ui::theme::*;

pub struct SettingsView;

impl SettingsView {
    /// 创建设置视图的主界面
    pub fn view(app: &ArxivManager) -> Element<'_, Message> {
        let theme_colors = app.theme_colors();
        let current_font = app.current_font();
        let base_font_size = app.current_font_size();
        let scale = app.current_scale();
        
        let title = text("Settings")
            .color(theme_colors.text_primary)
            .size(base_font_size * 2.0)
            .font(current_font);

        // 外观设置区域
        let appearance_section = appearance::create_appearance_section(app);
        
        // 下载设置区域
        let downloads_section = downloads::create_downloads_section(app);
        
        // 快捷键设置区域
        let shortcuts_section = shortcuts::create_shortcuts_section(app);

        // 组装完整的设置视图
        container(
            scrollable(
                column![
                    title,
                    vertical_space().height(20.0 * scale),
                    appearance_section,
                    vertical_space().height(15.0 * scale),
                    downloads_section,
                    vertical_space().height(15.0 * scale),
                    shortcuts_section,
                ].spacing(10.0 * scale)
            )
        )
        .padding(20.0 * scale)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(theme_colors.dark_bg)),
            border: Border::default(),
            text_color: Some(theme_colors.text_primary),
            shadow: Shadow::default(),
        })
        .into()
    }
}
