// 基于iced框架的PDF阅读器界面

use std::sync::Arc;
use iced::{
    Element, Length, Alignment, keyboard,
    widget::{Button, Column, Container, Row, TextInput, Scrollable, button, text, svg},
    Background, Color,
};

use crate::core::messages::Message;
use crate::ui::style::{button_primary_style_dynamic, button_secondary_style_dynamic};
use crate::core::models::Theme as ThemeVariant;
use super::{PdfRenderer, PdfSearchEngine, PdfViewerState, PdfConfig, PdfPage};

/// PDF 查看器组件
pub struct PdfViewer {
    pub id: String,
    pub file_path: String,
    pub state: PdfViewerState,
    pub renderer: Option<Arc<PdfRenderer>>,
    pub search_engine: Option<Arc<PdfSearchEngine>>,
    pub config: PdfConfig,
    pub current_page_data: Option<PdfPage>,
    pub search_input: String,
    pub error_message: Option<String>,
}

impl PdfViewer {
    /// 创建新的PDF查看器
    pub fn new(id: String, file_path: String) -> Self {
        Self {
            id,
            file_path,
            state: PdfViewerState::default(),
            renderer: None,
            search_engine: None,
            config: PdfConfig::default(),
            current_page_data: None,
            search_input: String::new(),
            error_message: None,
        }
    }

    /// 初始化PDF查看器
    pub fn initialize(&mut self) -> Result<(), String> {
        println!("Initializing PDF viewer for file: {}", self.file_path);
        
        // 创建渲染器
        let mut renderer = PdfRenderer::new(self.config.clone());
        println!("Created PDF renderer");
        
        let page_count = renderer.load_document(&self.file_path)
            .map_err(|e| {
                println!("Failed to load PDF document: {}", e);
                format!("无法加载PDF文档: {}", e)
            })?;
        
        println!("Loaded PDF document with {} pages", page_count);

        self.state.total_pages = page_count;
        self.state.current_page = 1;
        
        let renderer_arc = Arc::new(renderer);
        
        // 创建搜索引擎
        let search_engine = PdfSearchEngine::new(renderer_arc.clone());
        
        self.renderer = Some(renderer_arc);
        self.search_engine = Some(Arc::new(search_engine));
        
        // 渲染第一页
        self.render_current_page()?;
        
        Ok(())
    }

    /// 渲染当前页面
    pub fn render_current_page(&mut self) -> Result<(), String> {
        if let Some(renderer) = &self.renderer {
            self.state.is_loading = true;
            
            match renderer.render_page(
                self.state.current_page,
                self.state.zoom_level,
                (1200, 900), // 更大的视口尺寸以获得更清晰的渲染
                self.state.search_term.as_deref(),
            ) {
                Ok(page) => {
                    self.current_page_data = Some(page);
                    self.error_message = None;
                }
                Err(e) => {
                    self.error_message = Some(format!("渲染页面失败: {}", e));
                }
            }
            
            self.state.is_loading = false;
        }
        
        Ok(())
    }

    /// 更新查看器状态
    pub fn update(&mut self, message: PdfViewerMessage) -> Option<Message> {
        println!("PDF Viewer received message: {:?}", message);
        match message {
            PdfViewerMessage::OpenFile(path) => {
                println!("OpenFile message received for: {:?}", path);
                self.file_path = path.to_string_lossy().to_string();
                println!("Setting file_path to: {}", self.file_path);
                if let Err(e) = self.initialize() {
                    println!("Initialization failed: {}", e);
                    self.error_message = Some(e);
                } else {
                    println!("Initialization successful!");
                }
            }
            
            PdfViewerMessage::NextPage => {
                if self.state.current_page < self.state.total_pages {
                    self.state.current_page += 1;
                    let _ = self.render_current_page();
                }
            }
            
            PdfViewerMessage::PreviousPage => {
                if self.state.current_page > 1 {
                    self.state.current_page -= 1;
                    let _ = self.render_current_page();
                }
            }
            
            PdfViewerMessage::GoToPage(page) => {
                if page >= 1 && page <= self.state.total_pages {
                    self.state.current_page = page;
                    let _ = self.render_current_page();
                }
            }
            
            PdfViewerMessage::ZoomIn => {
                self.state.zoom_level = (self.state.zoom_level * 1.2).min(self.config.max_zoom);
                let _ = self.render_current_page();
            }
            
            PdfViewerMessage::ZoomOut => {
                self.state.zoom_level = (self.state.zoom_level / 1.2).max(self.config.min_zoom);
                let _ = self.render_current_page();
            }
            
            PdfViewerMessage::ResetZoom => {
                self.state.zoom_level = self.config.default_zoom;
                self.state.scroll_x = 0.0;
                self.state.scroll_y = 0.0;
                let _ = self.render_current_page();
            }
            
            PdfViewerMessage::SearchInputChanged(input) => {
                self.search_input = input;
            }
            
            PdfViewerMessage::Search => {
                if !self.search_input.is_empty() {
                    self.perform_search();
                } else {
                    self.clear_search();
                }
            }
            
            PdfViewerMessage::NextSearchResult => {
                self.navigate_search_results(true);
            }
            
            PdfViewerMessage::PreviousSearchResult => {
                self.navigate_search_results(false);
            }
            
            PdfViewerMessage::ClearSearch => {
                self.clear_search();
            }
            
            PdfViewerMessage::Close => {
                return Some(Message::CloseTab(self.id.clone()));
            }
        }
        
        None
    }

    /// 执行搜索
    fn perform_search(&mut self) {
        if let Some(_search_engine) = &self.search_engine {
            self.state.search_term = Some(self.search_input.clone());
            
            // 简化的搜索实现，使用同步方法
            if let Some(renderer) = &self.renderer {
                let mut results = Vec::new();
                
                // 在所有页面中搜索
                for page_num in 1..=self.state.total_pages {
                    if let Ok(text) = renderer.extract_text(page_num) {
                        for (index, _) in text.match_indices(&self.search_input) {
                            results.push(super::SearchResult {
                                page_number: page_num,
                                text: self.search_input.clone(),
                                context: text.clone(),
                                position: (50.0 + (index % 20) as f32 * 10.0, 100.0 + (index / 20) as f32 * 25.0),
                                highlighted_text: self.search_input.clone(),
                            });
                        }
                    }
                }
                
                self.state.search_results = results;
                self.state.current_search_index = if self.state.search_results.is_empty() {
                    None
                } else {
                    Some(0)
                };
                
                // 跳转到第一个搜索结果
                if let Some(first_result) = self.state.search_results.first() {
                    self.state.current_page = first_result.page_number;
                    let _ = self.render_current_page();
                }
            }
        }
    }

    /// 导航搜索结果
    fn navigate_search_results(&mut self, next: bool) {
        if self.state.search_results.is_empty() {
            return;
        }
        
        let current_index = self.state.current_search_index.unwrap_or(0);
        let new_index = if next {
            (current_index + 1) % self.state.search_results.len()
        } else {
            if current_index == 0 {
                self.state.search_results.len() - 1
            } else {
                current_index - 1
            }
        };
        
        self.state.current_search_index = Some(new_index);
        
        if let Some(result) = self.state.search_results.get(new_index) {
            if result.page_number != self.state.current_page {
                self.state.current_page = result.page_number;
                // 这里应该触发页面重新渲染
            }
        }
    }

    /// 清除搜索
    fn clear_search(&mut self) {
        self.state.search_term = None;
        self.state.search_results.clear();
        self.state.current_search_index = None;
        self.search_input.clear();
    }

    /// 构建查看器UI - 支持主题
    pub fn view(&self, theme: &ThemeVariant) -> Element<Message> {
        let content = if self.renderer.is_some() {
            self.build_pdf_view(theme)
        } else {
            self.build_loading_view(theme)
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    /// 构建PDF查看界面
    fn build_pdf_view(&self, theme: &ThemeVariant) -> Element<Message> {
        let toolbar = self.build_toolbar(theme);
        let content = self.build_content_area(theme);

        Column::new()
            .push(toolbar)
            .push(content)
            .spacing(0)
            .into()
    }

    /// 构建工具栏 - 使用应用程序一致的主题风格
    fn build_toolbar(&self, theme: &ThemeVariant) -> Element<Message> {
        use crate::ui::theme::get_theme_colors;
        let theme_colors = get_theme_colors(theme);
        
        // 导航控件 - 使用更好的图标
        let navigation = Row::new()
            .push(
                Button::new(text("◀").size(16))
                    .on_press_maybe(if self.state.current_page > 1 {
                        Some(Message::PdfViewer(self.id.clone(), PdfViewerMessage::PreviousPage))
                    } else {
                        None
                    })
                    .style(button_secondary_style_dynamic(theme))
                    .padding([6, 12])
            )
            .push(
                Container::new(
                    text(format!("第 {} 页，共 {} 页", self.state.current_page, self.state.total_pages))
                        .size(14)
                        .color(theme_colors.text_primary)
                )
                .padding([8, 16])
            )
            .push(
                Button::new(text("▶").size(16))
                    .on_press_maybe(if self.state.current_page < self.state.total_pages {
                        Some(Message::PdfViewer(self.id.clone(), PdfViewerMessage::NextPage))
                    } else {
                        None
                    })
                    .style(button_secondary_style_dynamic(theme))
                    .padding([6, 12])
            )
            .spacing(8)
            .align_y(Alignment::Center);

        // 缩放控件
        let zoom_controls = Row::new()
            .push(
                Button::new(text("−").size(16))
                    .on_press(Message::PdfViewer(self.id.clone(), PdfViewerMessage::ZoomOut))
                    .style(button_secondary_style_dynamic(theme))
                    .padding([6, 12])
            )
            .push(
                Container::new(
                    text(format!("{:.0}%", self.state.zoom_level * 100.0))
                        .size(14)
                        .color(theme_colors.text_primary)
                )
                .padding([8, 16])
            )
            .push(
                Button::new(text("+").size(16))
                    .on_press(Message::PdfViewer(self.id.clone(), PdfViewerMessage::ZoomIn))
                    .style(button_secondary_style_dynamic(theme))
                    .padding([6, 12])
            )
            .push(
                Button::new(text("重置").size(12))
                    .on_press(Message::PdfViewer(self.id.clone(), PdfViewerMessage::ResetZoom))
                    .style(button_secondary_style_dynamic(theme))
                    .padding([6, 12])
            )
            .spacing(8)
            .align_y(Alignment::Center);

        // 搜索栏
        let search_bar = Row::new()
            .push(
                TextInput::new("搜索文档内容...", &self.search_input)
                    .on_input(|input| Message::PdfViewer(
                        self.id.clone(), 
                        PdfViewerMessage::SearchInputChanged(input)
                    ))
                    .on_submit(Message::PdfViewer(self.id.clone(), PdfViewerMessage::Search))
                    .width(Length::Fixed(250.0))
                    .style(move |_theme, status| {
                        use iced::widget::text_input::{Status, Style};
                        use iced::{Border, Background};
                        Style {
                            background: Background::Color(theme_colors.dark_bg_secondary),
                            border: Border {
                                color: match status {
                                    Status::Focused => theme_colors.accent_border,
                                    _ => theme_colors.border_color,
                                },
                                width: 1.0,
                                radius: 6.0.into(),
                            },
                            icon: Color::TRANSPARENT,
                            placeholder: theme_colors.text_muted,
                            value: theme_colors.text_primary,
                            selection: theme_colors.accent_border,
                        }
                    })
            )
            .push(
                Button::new(text("搜索").size(12))
                    .on_press(Message::PdfViewer(self.id.clone(), PdfViewerMessage::Search))
                    .style(button_primary_style_dynamic(theme))
                    .padding([6, 12])
            )
            .spacing(8)
            .align_y(Alignment::Center);

        // 快捷键提示
        let shortcuts_info = Container::new(
            text("快捷键: ←/→ 翻页 | +/- 缩放 | 0 重置")
                .size(11)
                .color(theme_colors.text_muted)
        ).padding([4, 8]);

        // 关闭按钮
        let close_button = Button::new(text("✕").size(14))
            .on_press(Message::PdfViewer(self.id.clone(), PdfViewerMessage::Close))
            .style(move |_theme, status| {
                use iced::widget::button::{Status, Style};
                use iced::{Border, Background, Shadow};
                let (background, text_color) = match status {
                    Status::Hovered => (theme_colors.error_color, theme_colors.text_primary),
                    Status::Pressed => (Color::from_rgb(0.80, 0.25, 0.30), theme_colors.text_primary),
                    _ => (Color::TRANSPARENT, theme_colors.text_muted),
                };
                
                Style {
                    background: Some(Background::Color(background)),
                    text_color,
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: 4.0.into(),
                    },
                    shadow: Shadow::default(),
                }
            })
            .padding([6, 12]);

        let toolbar_content = Row::new()
            .push(navigation)
            .push(zoom_controls)
            .push(search_bar)
            .push(shortcuts_info)
            .push(close_button)
            .spacing(24)
            .align_y(Alignment::Center)
            .padding(12);

        Container::new(toolbar_content)
            .width(Length::Fill)
            .style(move |_theme: &iced::Theme| {
                iced::widget::container::Style {
                    background: Some(Background::Color(theme_colors.dark_bg_secondary)),
                    border: iced::Border {
                        color: theme_colors.border_color,
                        width: 1.0,
                        radius: 0.0.into(),
                    },
                    text_color: Some(theme_colors.text_primary),
                    shadow: iced::Shadow {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                        offset: iced::Vector::new(0.0, 2.0),
                        blur_radius: 4.0,
                    },
                }
            })
            .into()
    }

    /// 构建内容区域
    fn build_content_area(&self, theme: &ThemeVariant) -> Element<Message> {
        use crate::ui::theme::get_theme_colors;
        let theme_colors = get_theme_colors(theme);
        
        if let Some(error) = &self.error_message {
            // 显示错误信息
            Container::new(
                Column::new()
                    .push(text("⚠️").size(48).color(theme_colors.error_color))
                    .push(text("PDF加载失败").size(20).color(theme_colors.text_primary))
                    .push(text(error).size(14).color(theme_colors.text_secondary))
                    .spacing(10)
                    .align_x(Alignment::Center)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(move |_theme: &iced::Theme| {
                iced::widget::container::Style {
                    background: Some(Background::Color(theme_colors.dark_bg)),
                    border: iced::Border::default(),
                    text_color: Some(theme_colors.text_primary),
                    shadow: iced::Shadow::default(),
                }
            })
            .into()
        } else if self.state.is_loading {
            // 显示加载状态
            Container::new(
                Column::new()
                    .push(text("📄").size(48).color(theme_colors.info_color))
                    .push(text("正在加载...").size(18).color(theme_colors.text_primary))
                    .spacing(10)
                    .align_x(Alignment::Center)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(move |_theme: &iced::Theme| {
                iced::widget::container::Style {
                    background: Some(Background::Color(theme_colors.dark_bg)),
                    border: iced::Border::default(),
                    text_color: Some(theme_colors.text_primary),
                    shadow: iced::Shadow::default(),
                }
            })
            .into()
        } else {
            // 显示PDF内容
            self.build_pdf_content(theme)
        }
    }

    /// 构建PDF内容显示
    fn build_pdf_content(&self, theme: &ThemeVariant) -> Element<Message> {
        use crate::ui::theme::get_theme_colors;
        let theme_colors = get_theme_colors(theme);
        
        // 检查是否有渲染的页面数据
        if let Some(page_data) = &self.current_page_data {
            // 使用SVG矢量渲染
            let svg_handle = iced::widget::svg::Handle::from_memory(page_data.svg_data.clone().into_bytes());
            let pdf_display: Element<Message> = iced::widget::Svg::<iced::Theme>::new(svg_handle)
                .width(Length::Fixed(page_data.width as f32))
                .height(Length::Fixed(page_data.height as f32))
                .into();

            // 页面信息栏
            let info_bar = Row::new()
                .push(text(format!("第 {} 页，共 {} 页", self.state.current_page, self.state.total_pages)).size(12).color(theme_colors.text_secondary))
                .push(text(format!("缩放: {:.0}%", self.state.zoom_level * 100.0)).size(12).color(theme_colors.text_secondary))
                .push(text(format!("尺寸: {}×{}", page_data.width, page_data.height)).size(12).color(theme_colors.text_secondary))
                .push(text("类型: SVG矢量").size(12).color(theme_colors.success_color))
                .spacing(20)
                .align_y(Alignment::Center);

            let content = Column::new()
                .push(
                    Container::new(info_bar)
                        .width(Length::Shrink)
                        .padding(8)
                        .style(move |_theme: &iced::Theme| {
                            iced::widget::container::Style {
                                background: Some(Background::Color(theme_colors.dark_bg_secondary)),
                                border: iced::Border {
                                    color: theme_colors.border_color,
                                    width: 1.0,
                                    radius: 6.0.into(),
                                },
                                text_color: Some(theme_colors.text_primary),
                                shadow: iced::Shadow::default(),
                            }
                        })
                )
                .push(
                    Container::new(pdf_display)
                        .width(Length::Shrink)
                        .padding(16)
                        .center_x(Length::Shrink)
                        .style(move |_theme: &iced::Theme| {
                            iced::widget::container::Style {
                                background: Some(Background::Color(Color::WHITE)),
                                border: iced::Border {
                                    color: theme_colors.border_color,
                                    width: 2.0,
                                    radius: 8.0.into(),
                                },
                                text_color: None,
                                shadow: iced::Shadow {
                                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                                    offset: iced::Vector::new(0.0, 4.0),
                                    blur_radius: 12.0,
                                },
                            }
                        })
                )
                .spacing(8);

            // 使用可滚动的容器包装内容
            Scrollable::new(content)
                .direction(iced::widget::scrollable::Direction::Both {
                    vertical: iced::widget::scrollable::Scrollbar::default(),
                    horizontal: iced::widget::scrollable::Scrollbar::default(),
                })
                .style(move |_theme, _status| {
                    iced::widget::scrollable::Style {
                        container: iced::widget::container::Style {
                            background: Some(Background::Color(theme_colors.dark_bg)),
                            border: iced::Border::default(),
                            text_color: Some(theme_colors.text_primary),
                            shadow: iced::Shadow::default(),
                        },
                        vertical_rail: iced::widget::scrollable::Rail {
                            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.1))),
                            border: iced::Border {
                                radius: 4.0.into(),
                                ..Default::default()
                            },
                            scroller: iced::widget::scrollable::Scroller {
                                color: theme_colors.accent_border,
                                border: iced::Border {
                                    radius: 4.0.into(),
                                    ..Default::default()
                                },
                            },
                        },
                        horizontal_rail: iced::widget::scrollable::Rail {
                            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.1))),
                            border: iced::Border {
                                radius: 4.0.into(),
                                ..Default::default()
                            },
                            scroller: iced::widget::scrollable::Scroller {
                                color: theme_colors.accent_border,
                                border: iced::Border {
                                    radius: 4.0.into(),
                                    ..Default::default()
                                },
                            },
                        },
                        gap: None,
                    }
                })
                .into()
        } else {
            // 如果没有页面数据，显示提示信息
            let content = Column::new()
                .push(text("📄").size(64).color(theme_colors.text_muted))
                .push(text("PDF页面未加载").size(20).color(theme_colors.text_primary))
                .push(text("请稍候...").size(14).color(theme_colors.text_secondary))
                .spacing(10)
                .align_x(Alignment::Center);

            Container::new(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .style(move |_theme: &iced::Theme| {
                    iced::widget::container::Style {
                        background: Some(Background::Color(theme_colors.dark_bg)),
                        border: iced::Border {
                            color: theme_colors.border_color,
                            width: 1.0,
                            radius: 8.0.into(),
                        },
                        text_color: Some(theme_colors.text_primary),
                        shadow: iced::Shadow::default(),
                    }
                })
                .into()
        }
    }

    /// 构建加载视图
    fn build_loading_view(&self, theme: &ThemeVariant) -> Element<Message> {
        use crate::ui::theme::get_theme_colors;
        let theme_colors = get_theme_colors(theme);
        
        Container::new(
            Column::new()
                .push(text("📄").size(64).color(theme_colors.info_color))
                .push(text("正在初始化PDF查看器...").size(18).color(theme_colors.text_primary))
                .push(text("请稍候...").size(14).color(theme_colors.text_secondary))
                .spacing(20)
                .align_x(Alignment::Center)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(move |_theme: &iced::Theme| {
            iced::widget::container::Style {
                background: Some(Background::Color(theme_colors.dark_bg)),
                border: iced::Border::default(),
                text_color: Some(theme_colors.text_primary),
                shadow: iced::Shadow::default(),
            }
        })
        .into()
    }

    /// 处理键盘事件 - 支持横向翻页
    pub fn handle_keyboard_event(&mut self, event: keyboard::Event) -> Option<Message> {
        match event {
            keyboard::Event::KeyPressed { key, .. } => {
                match key {
                    // 左箭头键 - 上一页
                    keyboard::Key::Named(keyboard::key::Named::ArrowLeft) => {
                        if self.state.current_page > 1 {
                            return Some(Message::PdfViewer(self.id.clone(), PdfViewerMessage::PreviousPage));
                        }
                    }
                    // 右箭头键 - 下一页  
                    keyboard::Key::Named(keyboard::key::Named::ArrowRight) => {
                        if self.state.current_page < self.state.total_pages {
                            return Some(Message::PdfViewer(self.id.clone(), PdfViewerMessage::NextPage));
                        }
                    }
                    // Home键 - 第一页
                    keyboard::Key::Named(keyboard::key::Named::Home) => {
                        if self.state.current_page != 1 {
                            return Some(Message::PdfViewer(self.id.clone(), PdfViewerMessage::GoToPage(1)));
                        }
                    }
                    // End键 - 最后一页
                    keyboard::Key::Named(keyboard::key::Named::End) => {
                        if self.state.current_page != self.state.total_pages {
                            return Some(Message::PdfViewer(self.id.clone(), PdfViewerMessage::GoToPage(self.state.total_pages)));
                        }
                    }
                    // 字符键处理
                    keyboard::Key::Character(c) => {
                        match c.as_str() {
                            // + 或 = 放大
                            "+" | "=" => {
                                return Some(Message::PdfViewer(self.id.clone(), PdfViewerMessage::ZoomIn));
                            }
                            // - 缩小
                            "-" => {
                                return Some(Message::PdfViewer(self.id.clone(), PdfViewerMessage::ZoomOut));
                            }
                            // 0 重置缩放
                            "0" => {
                                return Some(Message::PdfViewer(self.id.clone(), PdfViewerMessage::ResetZoom));
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        None
    }
}

/// PDF 查看器消息类型
#[derive(Debug, Clone)]
pub enum PdfViewerMessage {
    OpenFile(std::path::PathBuf),
    NextPage,
    PreviousPage,
    GoToPage(u32),
    ZoomIn,
    ZoomOut,
    ResetZoom,
    SearchInputChanged(String),
    Search,
    NextSearchResult,
    PreviousSearchResult,
    ClearSearch,
    Close,
}
