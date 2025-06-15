// 现代化下载视图 - IRC客户端风格

use iced::widget::{column, container, scrollable, text};
use iced::{Element, Length};

use crate::core::app_state::ArxivManager;
use crate::core::messages::Message;
use crate::ui::components::PaperCard;
use crate::ui::style::{chat_container_dynamic_style, scrollable_tab_style_dynamic_with_fade, ultra_thin_vertical_scrollbar};

pub struct DownloadsView;

impl DownloadsView {
    pub fn view(app: &ArxivManager) -> Element<'_, Message> {
        let theme_colors = app.theme_colors();
        let current_font = app.current_font();
        let base_font_size = app.current_font_size();
        let scale = app.current_scale();
        
        let content = if app.downloads.is_empty() {
            column![
                text("Downloads")
                    .color(theme_colors.text_primary)
                    .size(base_font_size * 1.5)  // 接近Settings的比例
                    .font(current_font),
                text("No active downloads")
                    .color(theme_colors.text_muted)
                    .size(base_font_size * 1.1)  // 稍微放大副标题
                    .font(current_font),
                text("PDF downloads will appear here")
                    .color(theme_colors.text_secondary)
                    .size(base_font_size)  // 描述文字使用标准大小
                    .font(current_font)
            ]
            .spacing(12.0 * scale)
            .padding(32.0 * scale)
            .align_x(iced::Alignment::Center)
        } else {
            let downloads_count = app.downloads.len();
            column![
                container(
                    text(format!("Downloads ({} active)", downloads_count))
                        .color(theme_colors.text_primary)
                        .size(base_font_size * 1.1)  // 减小标题字体，与Settings保持一致
                        .font(current_font)
                )
                .padding(iced::Padding {
                    top: 16.0 * scale,
                    right: 16.0 * scale,
                    bottom: 8.0 * scale,
                    left: 16.0 * scale,
                }),
                
                container(
                    scrollable(
                        column(
                            app.downloads.iter().map(|download| {
                                PaperCard::download_card(download, app)
                            }).collect::<Vec<Element<Message>>>()
                        ).spacing(12)
                    )
                    .direction(ultra_thin_vertical_scrollbar())
                    .style(scrollable_tab_style_dynamic_with_fade(
                        &app.settings.theme, 
                        app.get_scrollbar_alpha("downloads_view")
                    ))
                    .on_scroll(|_| Message::ScrollbarActivity("downloads_view".to_string()))
                    .height(Length::Fill)
                )
                .padding(16)
            ]
        };

        container(content)
            .style(chat_container_dynamic_style(&app.settings.theme))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
