// 现代化论文库视图 - 支持文件夹结构管理，类似Zotero

use iced::widget::{column, container, scrollable, text, row, button, text_input, horizontal_space};
use iced::{Element, Length, Alignment, Color};

use crate::core::app_state::ArxivManager;
use crate::core::messages::Message;
use crate::core::models::{Collection, CollectionTreeNode, SystemCollections, LibrarySortBy, LibraryGroupBy, ArxivPaper};
use crate::core::handlers::CollectionHandler;
use crate::ui::style::{chat_container_dynamic_style, scrollable_tab_style_dynamic_with_fade, button_primary_style_dynamic, button_secondary_style_dynamic, pick_list_dynamic_style, ultra_thin_vertical_scrollbar};

pub struct LibraryView;

impl LibraryView {
    pub fn view(app: &ArxivManager) -> Element<'_, Message> {
        let theme_colors = app.theme_colors();
        
        // 左侧：集合树视图
        let collections_panel = Self::create_collections_panel(app);
        
        // 右侧：论文列表
        let papers_panel = Self::create_papers_panel(app);
        
        // 主布局：左右分栏
        let content = row![
            // 集合面板 (30% 宽度)
            container(collections_panel)
                .width(Length::FillPortion(3))
                .height(Length::Fill),
            
            // 分隔线
            container("")
                .width(1)
                .height(Length::Fill)
                .style(move |_: &iced::Theme| {
                    iced::widget::container::Style {
                        background: Some(iced::Background::Color(theme_colors.border_color)),
                        ..Default::default()
                    }
                }),
            
            // 论文面板 (70% 宽度)
            container(papers_panel)
                .width(Length::FillPortion(7))
                .height(Length::Fill),
        ];

        container(content)
            .style(chat_container_dynamic_style(&app.settings.theme))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
    
    /// 创建集合面板
    fn create_collections_panel(app: &ArxivManager) -> Element<'_, Message> {
        let theme_colors = app.theme_colors();
        let current_font = app.current_font();
        let base_font_size = app.current_font_size();
        let scale = app.current_scale();
        
        // 顶部标题和创建按钮
        let header = row![
            text("Collections")
                .color(theme_colors.text_primary)
                .size(base_font_size * 1.1)
                .font(current_font),
            horizontal_space(),
            button(text("+").size(base_font_size).font(current_font))
                .on_press(Message::CreateCollection { 
                    name: "New Collection".to_string(), 
                    parent_id: None 
                })
                .style(button_secondary_style_dynamic(&app.settings.theme))
                .padding(4.0 * scale),
        ]
        .spacing(8.0 * scale)
        .align_y(Alignment::Center);
        
        // 集合创建输入框（如果正在创建）
        let creation_input = if app.is_creating_collection {
            Some(
                row![
                    text_input("Collection name", &app.collection_name_input)
                        .on_input(|input| Message::CreateCollection { 
                            name: input, 
                            parent_id: None 
                        })
                        .size(base_font_size)
                        .font(current_font),
                    button(text("✓").size(base_font_size).font(current_font))
                        .on_press(Message::CreateCollection { 
                            name: app.collection_name_input.clone(), 
                            parent_id: app.collection_parent_id 
                        })
                        .style(button_primary_style_dynamic(&app.settings.theme))
                        .padding(2.0 * scale),
                ]
                .spacing(4.0 * scale)
            )
        } else {
            None
        };
        
        // 系统集合（始终显示）
        let system_collections = Self::create_system_collections_list(app);
        
        // 用户集合
        let user_collections = Self::create_user_collections_tree(app);
        
        let mut content_column = column![header].spacing(8.0 * scale);
        
        if let Some(input) = creation_input {
            content_column = content_column.push(input);
        }
        
        content_column = content_column
            .push(system_collections)
            .push(user_collections);
        
        container(
            scrollable(content_column.padding(16.0 * scale)) // 将padding移到scrollable内部
                .direction(ultra_thin_vertical_scrollbar())
                .style(scrollable_tab_style_dynamic_with_fade(
                    &app.settings.theme, 
                    app.get_scrollbar_alpha("library_collections")
                ))
                .on_scroll(|_| Message::ScrollbarActivity("library_collections".to_string()))
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
    
    /// 创建系统集合列表
    fn create_system_collections_list(app: &ArxivManager) -> Element<'_, Message> {
        let theme_colors = app.theme_colors();
        let current_font = app.current_font();
        let base_font_size = app.current_font_size();
        let scale = app.current_scale();
        
        let system_collections = SystemCollections::get_all_system_collections();
        let mut collections_column = column![]
            .spacing(2.0 * scale);
        
        for collection in system_collections {
            let is_selected = app.selected_collection_id == Some(collection.id);
            let paper_count = match collection.id {
                -1 => app.saved_papers.len(),
                -2 => app.saved_papers.len().min(10),
                -3 => app.saved_papers.iter().filter(|p| p.is_favorite).count(), // 收藏数量
                -4 => app.saved_papers.len(),
                _ => 0,
            };
            
            let collection_item = Self::create_collection_item_by_value(
                collection, 
                paper_count, 
                is_selected, 
                0, // 系统集合没有缩进
                theme_colors,
                current_font,
                base_font_size,
                scale
            );
            
            collections_column = collections_column.push(collection_item);
        }
        
        column![
            text("System")
                .color(theme_colors.text_muted)
                .size(base_font_size * 0.8)
                .font(iced::Font {
                    weight: iced::font::Weight::Bold,
                    ..current_font
                }),
            collections_column,
            // 分隔线
            container("")
                .height(1)
                .width(Length::Fill)
                .style(move |_: &iced::Theme| {
                    iced::widget::container::Style {
                        background: Some(iced::Background::Color(theme_colors.border_color)),
                        ..Default::default()
                    }
                })
        ]
        .spacing(4.0 * scale)
        .into()
    }
    
    /// 创建用户集合树
    fn create_user_collections_tree(app: &ArxivManager) -> Element<'_, Message> {
        let theme_colors = app.theme_colors();
        let current_font = app.current_font();
        let base_font_size = app.current_font_size();
        let scale = app.current_scale();
        
        let user_collections: Vec<&Collection> = app.collections.iter()
            .filter(|c| c.id > 0) // 排除系统集合
            .collect();
        
        if user_collections.is_empty() {
            return column![
                text("My Collections")
                    .color(theme_colors.text_muted)
                    .size(base_font_size * 0.8)
                    .font(iced::Font {
                        weight: iced::font::Weight::Bold,
                        ..current_font
                    }),
                container(
                    text("No collections yet")
                        .color(theme_colors.text_muted)
                        .size(base_font_size * 0.9)
                        .font(current_font)
                )
                .padding(8.0 * scale)
            ]
            .spacing(4.0 * scale)
            .into();
        }
        
        let tree_nodes = app.build_collection_tree();
        let mut collections_column = column![]
            .spacing(2.0 * scale);
        
        for node in tree_nodes {
            if node.collection.id > 0 { // 只显示用户集合
                let tree_view = Self::create_tree_node_view_owned(node, 0, app);
                collections_column = collections_column.push(tree_view);
            }
        }
        
        column![
            text("My Collections")
                .color(theme_colors.text_muted)
                .size(base_font_size * 0.8)
                .font(iced::Font {
                    weight: iced::font::Weight::Bold,
                    ..current_font
                }),
            collections_column
        ]
        .spacing(4.0 * scale)
        .into()
    }
    
    /// 创建树节点视图（递归）
    fn create_tree_node_view<'a>(node: &'a CollectionTreeNode, indent_level: usize, app: &'a ArxivManager) -> Element<'a, Message> {
        let scale = app.current_scale();
        let _is_selected = app.selected_collection_id == Some(node.collection.id);
        
        let mut content = column![]
            .spacing(1.0 * scale);
        
        // 当前节点
        let collection_item = Self::create_collection_item(
            &node.collection,
            node.paper_count,
            _is_selected,
            indent_level,
            app
        );
        content = content.push(collection_item);
        
        // 子节点（如果展开）
        if node.collection.is_expanded {
            for child in &node.children {
                let child_view = Self::create_tree_node_view(child, indent_level + 1, app);
                content = content.push(child_view);
            }
        }
        
        content.into()
    }
    
    /// 创建树节点视图（递归，拥有所有权版本）
    fn create_tree_node_view_owned<'a>(node: CollectionTreeNode, indent_level: usize, app: &'a ArxivManager) -> Element<'a, Message> {
        let scale = app.current_scale();
        let _is_selected = app.selected_collection_id == Some(node.collection.id);
        
        let mut content = column![]
            .spacing(1.0 * scale);
        
        // 当前节点 - 使用 create_collection_item_by_value 来避免生命周期问题
        let collection_item = Self::create_collection_item_by_value(
            node.collection.clone(),
            node.paper_count,
            _is_selected,
            indent_level,
            app.theme_colors(),
            app.current_font(),
            app.current_font_size(),
            app.current_scale()
        );
        content = content.push(collection_item);
        
        // 子节点（如果展开）
        if node.collection.is_expanded {
            for child in node.children {
                let child_view = Self::create_tree_node_view_owned(child, indent_level + 1, app);
                content = content.push(child_view);
            }
        }
        
        content.into()
    }
    
    
    /// 创建单个集合项（支持编辑模式）
    fn create_collection_item<'a>(
        collection: &'a Collection,
        paper_count: usize,
        is_selected: bool,
        indent_level: usize,
        app: &'a ArxivManager,
    ) -> Element<'a, Message> {
        // 检查是否正在编辑这个集合
        let is_editing = app.editing_collection_id == Some(collection.id);
        
        if is_editing {
            // 编辑模式：显示输入框和确认/取消按钮
            Self::create_collection_edit_item(collection, indent_level, app)
        } else {
            // 普通模式：显示普通的集合项
            Self::create_collection_normal_item(collection, paper_count, is_selected, indent_level, app)
        }
    }
    
    /// 创建编辑模式的集合项
    fn create_collection_edit_item<'a>(
        collection: &'a Collection,
        indent_level: usize,
        app: &'a ArxivManager,
    ) -> Element<'a, Message> {
        let current_font = app.current_font();
        let base_font_size = app.current_font_size();
        let scale = app.current_scale();
        let indent = (indent_level as f32) * 16.0 * scale;
        
        let edit_row = row![
            container("").width(indent as u16), // 缩进
            text("▶")
                .size(base_font_size)
                .font(current_font),
            text_input("Collection name", &app.collection_rename_input)
                .on_input(Message::CollectionRenameInputChanged)
                .size(base_font_size)
                .font(current_font)
                .width(Length::Fill),
            button(text("✓").size(base_font_size).font(current_font))
                .on_press(Message::RenameCollection { 
                    id: collection.id, 
                    new_name: app.collection_rename_input.clone() 
                })
                .style(button_primary_style_dynamic(&app.settings.theme))
                .padding(2.0 * scale),
            button(text("✕").size(base_font_size).font(current_font))
                .on_press(Message::CancelRenameCollection)
                .style(button_secondary_style_dynamic(&app.settings.theme))
                .padding(2.0 * scale),
        ]
        .spacing(4.0 * scale)
        .align_y(Alignment::Center);
        
        container(edit_row)
            .width(Length::Fill)
            .padding(iced::Padding::new(3.0 * scale))
            .into()
    }
    
    /// 创建普通模式的集合项
    fn create_collection_normal_item<'a>(
        collection: &'a Collection,
        paper_count: usize,
        is_selected: bool,
        indent_level: usize,
        app: &'a ArxivManager,
    ) -> Element<'a, Message> {
        let theme_colors = app.theme_colors();
        let current_font = app.current_font();
        let base_font_size = app.current_font_size();
        let scale = app.current_scale();
        let indent = (indent_level as f32) * 16.0 * scale;
        
        // 颜色
        let text_color = if is_selected {
            theme_colors.accent_border
        } else {
            theme_colors.text_secondary
        };
        
        // 背景色
        let bg_color = if is_selected {
            Some(Color::from_rgba(
                theme_colors.accent_border.r,
                theme_colors.accent_border.g,
                theme_colors.accent_border.b,
                0.1
            ))
        } else {
            None
        };
        
        let content = row![
            container("").width(indent as u16), // 缩进
            text("▶")
                .size(base_font_size)
                .font(current_font),
            text(collection.name.clone())
                .color(text_color)
                .size(base_font_size)
                .font(current_font),
            horizontal_space(),
            if paper_count > 0 {
                Element::from(text(paper_count.to_string())
                    .color(theme_colors.text_muted)
                    .size(base_font_size * 0.8)
                    .font(current_font))
            } else {
                Element::from(text(""))
            }
        ]
        .spacing(6.0 * scale)
        .align_y(Alignment::Center);
        
        // 只有用户集合（ID > 0）才支持重命名
        if collection.id > 0 {
            // 用户集合：支持重命名按钮
            row![
                button(content)
                    .on_press(Message::SelectCollection(Some(collection.id)))
                    .style(move |theme: &iced::Theme, status| {
                        let mut style = button_secondary_style_dynamic(&crate::core::models::Theme::ModernDark)(theme, status);
                        if let Some(bg) = bg_color {
                            style.background = Some(iced::Background::Color(bg));
                        }
                        style
                    })
                    .width(Length::Fill)
                    .padding(iced::Padding::new(3.0 * scale)),
                button(text("✏").size(base_font_size * 0.8).font(current_font))
                    .on_press(Message::StartRenameCollection(collection.id))
                    .style(button_secondary_style_dynamic(&app.settings.theme))
                    .padding(2.0 * scale),
            ]
            .spacing(2.0 * scale)
            .align_y(Alignment::Center)
            .into()
        } else {
            // 系统集合：仅支持选择
            button(content)
                .on_press(Message::SelectCollection(Some(collection.id)))
                .style(move |theme: &iced::Theme, status| {
                    let mut style = button_secondary_style_dynamic(&crate::core::models::Theme::ModernDark)(theme, status);
                    if let Some(bg) = bg_color {
                        style.background = Some(iced::Background::Color(bg));
                    }
                    style
                })
                .width(Length::Fill)
                .padding(iced::Padding::new(3.0 * scale))
                .into()
        }
    }

    /// 创建单个集合项（使用值而非引用，避免生命周期问题）
    fn create_collection_item_by_value(
        collection: Collection, 
        paper_count: usize, 
        is_selected: bool,
        indent_level: usize,
        theme_colors: crate::ui::theme::ThemeColors,
        current_font: iced::Font,
        base_font_size: f32,
        scale: f32
    ) -> Element<'static, Message> {
        let indent = (indent_level as f32) * 16.0 * scale;
        
        // 图标 - 使用简单的符号
        let icon = "▶";
        
        // 颜色
        let text_color = if is_selected {
            theme_colors.accent_border
        } else {
            theme_colors.text_secondary
        };
        
        // 背景色
        let bg_color = if is_selected {
            Some(Color::from_rgba(
                theme_colors.accent_border.r,
                theme_colors.accent_border.g,
                theme_colors.accent_border.b,
                0.1
            ))
        } else {
            None
        };
        
        let content = row![
            container("").width(indent as u16), // 缩进
            text(icon.clone())
                .size(base_font_size)
                .font(current_font),
            text(collection.name.clone())
                .color(text_color)
                .size(base_font_size)
                .font(current_font),
            horizontal_space(),
            if paper_count > 0 {
                Element::from(text(paper_count.to_string())
                    .color(theme_colors.text_muted)
                    .size(base_font_size * 0.8)
                    .font(current_font))
            } else {
                Element::from(text(""))
            }
        ]
        .spacing(6.0 * scale)
        .align_y(Alignment::Center);
        
        button(content)
            .on_press(Message::SelectCollection(Some(collection.id)))
            .style(move |theme: &iced::Theme, status| {
                let mut style = button_secondary_style_dynamic(&crate::core::models::Theme::ModernDark)(theme, status);
                if let Some(bg) = bg_color {
                    style.background = Some(iced::Background::Color(bg));
                }
                style
            })
            .width(Length::Fill)
            .padding(iced::Padding::new(3.0 * scale))
            .into()
    }
    
    /// 创建论文面板
    fn create_papers_panel(app: &ArxivManager) -> Element<'_, Message> {
        let theme_colors = app.theme_colors();
        let current_font = app.current_font();
        let base_font_size = app.current_font_size();
        let scale = app.current_scale();
        
        // 获取当前选中集合的信息
        let (collection_name, paper_count) = if let Some(collection_id) = app.selected_collection_id {
            let collection = if collection_id < 0 {
                // 系统集合
                SystemCollections::get_all_system_collections()
                    .into_iter()
                    .find(|c| c.id == collection_id)
            } else {
                // 用户集合
                app.collections.iter().find(|c| c.id == collection_id).cloned()
            };
            
            if let Some(c) = collection {
                (c.name, app.filtered_papers.len())
            } else {
                ("Unknown Collection".to_string(), 0)
            }
        } else {
            ("All Papers".to_string(), app.saved_papers.len())
        };
        
        // 顶部工具栏：标题 + 排序/显示控件
        let title_section = row![
            text(format!("{} ({} papers)", collection_name, paper_count))
                .color(theme_colors.text_primary)
                .size(base_font_size * 1.1)
                .font(current_font),
            horizontal_space(),
        ]
        .align_y(Alignment::Center);

        // 排序和显示控件
        let controls_section = Self::create_library_controls(app);
        
        // 完整的标题栏
        let header = container(
            column![
                title_section,
                controls_section
            ].spacing(8.0 * scale)
        )
        .padding(iced::Padding {
            top: 16.0 * scale,
            right: 16.0 * scale,
            bottom: 12.0 * scale,
            left: 16.0 * scale,
        });
        
        // 论文内容区域
        let papers_content = Self::create_papers_content(app);
        
        column![
            header,
            // 添加分隔线，与Search视图保持一致
            iced::widget::horizontal_rule(1)
                .style(move |_theme| iced::widget::rule::Style {
                    color: theme_colors.border_color,
                    width: 1,
                    radius: 0.0.into(),
                    fill_mode: iced::widget::rule::FillMode::Full,
                }),
            container(papers_content)
                .padding(16.0 * scale)
                .width(Length::Fill)
                .height(Length::Fill)
        ]
        .into()
    }
    
    /// 创建Library控件（排序、分组）
    fn create_library_controls(app: &ArxivManager) -> Element<'_, Message> {
        let theme_colors = app.theme_colors();
        let current_font = app.current_font();
        let base_font_size = app.current_font_size();
        let scale = app.current_scale();
        
        use iced::widget::pick_list;
        
        // 排序选择器
        let sort_picker = pick_list(
            LibrarySortBy::all_variants(),
            Some(app.library_sort_by),
            Message::LibrarySortChanged,
        )
        .placeholder("Sort by...")
        .style(pick_list_dynamic_style(&app.settings.theme))
        .text_size(base_font_size * 0.9)
        .font(current_font)
        .width(Length::Fixed(120.0 * scale));
        
        // 分组选择器
        let group_picker = pick_list(
            LibraryGroupBy::all_variants(),
            Some(app.library_group_by),
            Message::LibraryGroupChanged,
        )
        .placeholder("Group by...")
        .style(pick_list_dynamic_style(&app.settings.theme))
        .text_size(base_font_size * 0.9)
        .font(current_font)
        .width(Length::Fixed(120.0 * scale));
        
        row![
            text("Sort:")
                .color(theme_colors.text_secondary)
                .size(base_font_size * 0.9)
                .font(current_font),
            sort_picker,
            text("Group:")
                .color(theme_colors.text_secondary)
                .size(base_font_size * 0.9)
                .font(current_font),
            group_picker,
            horizontal_space(),
        ]
        .spacing(8.0 * scale)
        .align_y(Alignment::Center)
        .into()
    }
    
    /// 创建论文内容区域（固定为瀑布流视图）
    fn create_papers_content(app: &ArxivManager) -> Element<'_, Message> {
        let theme_colors = app.theme_colors();
        let current_font = app.current_font();
        let base_font_size = app.current_font_size();
        let scale = app.current_scale();
        
        // 获取要显示的论文
        let papers_to_show = if app.filtered_papers.is_empty() {
            &app.saved_papers
        } else {
            &app.filtered_papers
        };
        
        if papers_to_show.is_empty() {
            return Element::from(column![
                text("No papers in this collection")
                    .color(theme_colors.text_muted)
                    .size(base_font_size * 1.1)
                    .font(current_font),
                text("Papers you save will appear here")
                    .color(theme_colors.text_secondary)
                    .size(base_font_size)
                    .font(current_font)
            ]
            .spacing(12.0 * scale)
            .padding(32.0 * scale)
            .align_x(iced::Alignment::Center));
        }
        
        // 使用新的智能瀑布流视图
        Element::from(scrollable(
            Self::create_intelligent_waterfall_view(papers_to_show, app)
        )
        .direction(ultra_thin_vertical_scrollbar())
        .style(scrollable_tab_style_dynamic_with_fade(
            &app.settings.theme, 
            app.get_scrollbar_alpha("library_papers")
        ))
        .on_scroll(|_| Message::ScrollbarActivity("library_papers".to_string()))
        .height(Length::Fill))
    }
    
    /// 创建智能瀑布流视图（使用新的WaterfallLayout）
    fn create_intelligent_waterfall_view<'a>(papers: &'a [ArxivPaper], app: &'a ArxivManager) -> Element<'a, Message> {
        use crate::ui::components::WaterfallLayout;
        WaterfallLayout::library_view(app, papers)
    }
}
