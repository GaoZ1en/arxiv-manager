// 现代化按钮样式定义 - IRC客户端风格，支持动态主题

use iced::{Color, Background, Border, Shadow};
use iced::widget::{button, container, text_input};

use crate::ui::theme::{*, SIDEBAR_BG, BORDER_COLOR, TEXT_PRIMARY};
use crate::core::models::Theme as ThemeVariant;

// 动态主题样式 - 根据当前主题生成按钮样式
pub fn button_primary_dynamic_style(theme: &ThemeVariant, status: button::Status) -> button::Style {
    let colors = get_theme_colors(theme);
    let (background, text_color) = match status {
        button::Status::Hovered => (colors.button_hover, colors.text_primary),
        button::Status::Pressed => (colors.button_active, colors.text_primary),
        _ => (colors.button_primary, colors.text_primary),
    };
    
    button::Style {
        background: Some(Background::Color(background)),
        text_color,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 6.0.into(),
        },
        shadow: Shadow {
            color: Color::BLACK,
            offset: iced::Vector::new(0.0, 1.0),
            blur_radius: 2.0,
        },
    }
}

// 动态主题样式 - 次要按钮
pub fn button_secondary_dynamic_style(theme: &ThemeVariant, status: button::Status) -> button::Style {
    let colors = get_theme_colors(theme);
    let (background, text_color) = match status {
        button::Status::Hovered => (colors.sidebar_hover, colors.text_primary),
        button::Status::Pressed => (colors.border_color, colors.text_primary),
        _ => (Color::TRANSPARENT, colors.text_secondary),
    };
    
    button::Style {
        background: Some(Background::Color(background)),
        text_color,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 4.0.into(),
        },
        shadow: Shadow::default(),
    }
}

// 向后兼容的样式函数 - 使用静态主题
pub fn button_primary_style(_theme: &iced::Theme, status: button::Status) -> button::Style {
    let (background, text_color) = match status {
        button::Status::Hovered => (BUTTON_HOVER, TEXT_PRIMARY),
        button::Status::Pressed => (BUTTON_ACTIVE, TEXT_PRIMARY),
        _ => (BUTTON_PRIMARY, TEXT_PRIMARY),
    };
    
    button::Style {
        background: Some(Background::Color(background)),
        text_color,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 6.0.into(),
        },
        shadow: Shadow {
            color: Color::BLACK,
            offset: iced::Vector::new(0.0, 1.0),
            blur_radius: 2.0,
        },
    }
}

pub fn button_secondary_style(_theme: &iced::Theme, status: button::Status) -> button::Style {
    let (background, text_color) = match status {
        button::Status::Hovered => (SIDEBAR_HOVER, TEXT_PRIMARY),
        button::Status::Pressed => (BORDER_COLOR, TEXT_PRIMARY),
        _ => (Color::TRANSPARENT, TEXT_SECONDARY),
    };
    
    button::Style {
        background: Some(Background::Color(background)),
        text_color,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 4.0.into(),
        },
        shadow: Shadow::default(),
    }
}

// 侧边栏项目样式
pub fn sidebar_item_style(_theme: &iced::Theme, status: button::Status) -> button::Style {
    let (background, text_color) = match status {
        button::Status::Hovered => (SIDEBAR_HOVER, TEXT_PRIMARY),
        button::Status::Pressed => (ACCENT_BORDER, TEXT_PRIMARY),
        _ => (Color::TRANSPARENT, TEXT_SECONDARY),
    };
    
    button::Style {
        background: Some(Background::Color(background)),
        text_color,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 8.0.into(),
        },
        shadow: Shadow::default(),
    }
}

// 动态主题的侧边栏项目样式
pub fn sidebar_item_style_dynamic(theme: &ThemeVariant) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    let colors = get_theme_colors(theme);
    move |_theme, status| {
        let (background, text_color) = match status {
            button::Status::Hovered => (colors.sidebar_hover, colors.text_primary),
            button::Status::Pressed => (colors.accent_border, colors.text_primary),
            _ => (Color::TRANSPARENT, colors.text_secondary),
        };
        
        button::Style {
            background: Some(Background::Color(background)),
            text_color,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 8.0.into(),
            },
            shadow: Shadow::default(),
        }
    }
}

// 危险操作按钮
pub fn button_danger_style(_theme: &iced::Theme, status: button::Status) -> button::Style {
    let (background, text_color) = match status {
        button::Status::Hovered => (Color::from_rgb(0.90, 0.30, 0.35), TEXT_PRIMARY),
        button::Status::Pressed => (Color::from_rgb(0.80, 0.25, 0.30), TEXT_PRIMARY),
        _ => (ERROR_COLOR, TEXT_PRIMARY),
    };
    
    button::Style {
        background: Some(Background::Color(background)),
        text_color,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 6.0.into(),
        },
        shadow: Shadow {
            color: Color::BLACK,
            offset: iced::Vector::new(0.0, 1.0),
            blur_radius: 2.0,
        },
    }
}

// 容器样式 - 动态主题支持
pub fn sidebar_container_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(SIDEBAR_BG)),
        border: Border {
            color: BORDER_COLOR,
            width: 1.0,
            radius: 0.0.into(),
        },
        text_color: Some(TEXT_PRIMARY),
        shadow: Shadow::default(),
    }
}

// 动态主题的侧边栏容器样式
pub fn sidebar_container_dynamic_style(theme: &ThemeVariant) -> impl Fn(&iced::Theme) -> container::Style {
    let colors = get_theme_colors(theme);
    move |_theme| {
        container::Style {
            background: Some(Background::Color(colors.sidebar_bg)),
            border: Border {
                color: colors.border_color,
                width: 1.0,
                radius: 0.0.into(),
            },
            text_color: Some(colors.text_primary),
            shadow: Shadow::default(),
        }
    }
}

pub fn main_container_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(DARK_BG)),
        border: Border::default(),
        text_color: Some(TEXT_PRIMARY),
        shadow: Shadow::default(),
    }
}

// 动态主题的主容器样式
pub fn main_container_dynamic_style(theme: &ThemeVariant) -> impl Fn(&iced::Theme) -> container::Style {
    let colors = get_theme_colors(theme);
    move |_theme| {
        container::Style {
            background: Some(Background::Color(colors.dark_bg)),
            border: Border::default(),
            text_color: Some(colors.text_primary),
            shadow: Shadow::default(),
        }
    }
}

pub fn chat_container_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(DARK_BG_SECONDARY)),
        border: Border {
            color: BORDER_COLOR,
            width: 1.0,
            radius: 8.0.into(),
        },
        text_color: Some(TEXT_PRIMARY),
        shadow: Shadow::default(),
    }
}

// 动态主题的聊天容器样式
pub fn chat_container_dynamic_style(theme: &ThemeVariant) -> impl Fn(&iced::Theme) -> container::Style {
    let colors = get_theme_colors(theme);
    move |_theme| {
        container::Style {
            background: Some(Background::Color(colors.dark_bg_secondary)),
            border: Border {
                color: colors.border_color,
                width: 1.0,
                radius: 8.0.into(),
            },
            text_color: Some(colors.text_primary),
            shadow: Shadow::default(),
        }
    }
}

// 输入框样式
pub fn text_input_style(_theme: &iced::Theme, status: text_input::Status) -> text_input::Style {
    let background = match status {
        text_input::Status::Focused => DARK_BG_SECONDARY,
        text_input::Status::Hovered => DARK_BG_SECONDARY,
        _ => SIDEBAR_BG,
    };
    
    text_input::Style {
        background: Background::Color(background),
        border: Border {
            color: match status {
                text_input::Status::Focused => ACCENT_BORDER,
                _ => BORDER_COLOR,
            },
            width: 1.0,
            radius: 6.0.into(),
        },
        icon: TEXT_SECONDARY,
        placeholder: TEXT_MUTED,
        value: TEXT_PRIMARY,
        selection: BUTTON_PRIMARY,
    }
}

// 动态主题的输入框样式
pub fn text_input_dynamic_style(theme: &ThemeVariant) -> impl Fn(&iced::Theme, text_input::Status) -> text_input::Style {
    let colors = get_theme_colors(theme);
    move |_theme, status| {
        let background = match status {
            text_input::Status::Focused => colors.dark_bg_secondary,
            text_input::Status::Hovered => colors.dark_bg_secondary,
            _ => colors.sidebar_bg,
        };
        
        text_input::Style {
            background: Background::Color(background),
            border: Border {
                color: match status {
                    text_input::Status::Focused => colors.accent_border,
                    _ => colors.border_color,
                },
                width: 1.0,
                radius: 6.0.into(),
            },
            icon: colors.text_secondary,
            placeholder: colors.text_muted,
            value: colors.text_primary,
            selection: colors.button_primary,
        }
    }
}

// 标签栏样式函数
pub fn tab_active_style(_theme: &iced::Theme, _status: button::Status) -> button::Style {
    button::Style {
        background: Some(Background::Color(ACCENT_BORDER)),
        text_color: TEXT_PRIMARY,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius::new(8.0).top_left(8.0).top_right(8.0),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
            offset: iced::Vector::new(0.0, -1.0),
            blur_radius: 4.0,
        },
    }
}

// 动态主题的活动标签栏样式
pub fn tab_active_dynamic_style(theme: &ThemeVariant) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    let colors = get_theme_colors(theme);
    move |_theme, _status| {
        button::Style {
            background: Some(Background::Color(colors.accent_border)),
            text_color: colors.text_primary,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: iced::border::Radius::new(8.0).top_left(8.0).top_right(8.0),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                offset: iced::Vector::new(0.0, -1.0),
                blur_radius: 4.0,
            },
        }
    }
}

pub fn tab_inactive_style(_theme: &iced::Theme, status: button::Status) -> button::Style {
    let (background, text_color) = match status {
        button::Status::Hovered => (SIDEBAR_HOVER, TEXT_PRIMARY),
        button::Status::Pressed => (ACCENT_BORDER, TEXT_PRIMARY),
        _ => (Color::TRANSPARENT, TEXT_SECONDARY),
    };
    
    button::Style {
        background: Some(Background::Color(background)),
        text_color,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius::new(6.0).top_left(6.0).top_right(6.0),
        },
        shadow: Shadow::default(),
    }
}

// 动态主题的非活动标签栏样式
pub fn tab_inactive_dynamic_style(theme: &ThemeVariant) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    let colors = get_theme_colors(theme);
    move |_theme, status| {
        let (background, text_color) = match status {
            button::Status::Hovered => (colors.sidebar_hover, colors.text_primary),
            button::Status::Pressed => (colors.accent_border, colors.text_primary),
            _ => (Color::TRANSPARENT, colors.text_secondary),
        };
        
        button::Style {
            background: Some(Background::Color(background)),
            text_color,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: iced::border::Radius::new(6.0).top_left(6.0).top_right(6.0),
            },
            shadow: Shadow::default(),
        }
    }
}

// 动态主题的标签关闭按钮样式
pub fn tab_close_dynamic_style(theme: &ThemeVariant) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    let colors = get_theme_colors(theme);
    move |_theme, status| {
        let (background, text_color) = match status {
            button::Status::Hovered => (colors.error_color, colors.text_primary),
            button::Status::Pressed => (Color::from_rgb(0.80, 0.25, 0.30), colors.text_primary),
            _ => (Color::TRANSPARENT, colors.text_muted),
        };
        
        button::Style {
            background: Some(Background::Color(background)),
            text_color,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 12.0.into(),
            },
            shadow: Shadow::default(),
        }
    }
}

pub fn tab_bar_container_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(DARK_BG_SECONDARY)),
        border: Border {
            color: BORDER_COLOR,
            width: 1.0,
            radius: 0.0.into(),
        },
        text_color: Some(TEXT_PRIMARY),
        shadow: Shadow::default(),
    }
}

// 动态主题的标签栏容器样式
pub fn tab_bar_container_dynamic_style(theme: &ThemeVariant) -> impl Fn(&iced::Theme) -> container::Style {
    let colors = get_theme_colors(theme);
    move |_theme| {
        container::Style {
            background: Some(Background::Color(colors.dark_bg_secondary)),
            border: Border {
                color: colors.border_color,
                width: 1.0,
                radius: 0.0.into(),
            },
            text_color: Some(colors.text_primary),
            shadow: Shadow::default(),
        }
    }
}

// Pick list 样式
pub fn pick_list_style(_theme: &iced::Theme, status: iced::widget::pick_list::Status) -> iced::widget::pick_list::Style {
    iced::widget::pick_list::Style {
        text_color: TEXT_PRIMARY,
        background: Background::Color(SIDEBAR_BG),
        border: Border {
            color: match status {
                iced::widget::pick_list::Status::Active => BORDER_COLOR,
                iced::widget::pick_list::Status::Hovered => ACCENT_BORDER,
                iced::widget::pick_list::Status::Opened => ACCENT_BORDER,
            },
            width: 1.0,
            radius: 6.0.into(),
        },
        handle_color: TEXT_SECONDARY,
        placeholder_color: TEXT_MUTED,
    }
}

// 动态主题的pick_list样式
pub fn pick_list_dynamic_style(theme: &ThemeVariant) -> impl Fn(&iced::Theme, iced::widget::pick_list::Status) -> iced::widget::pick_list::Style {
    let colors = get_theme_colors(theme);
    move |_theme, status| {
        iced::widget::pick_list::Style {
            text_color: colors.text_primary,
            background: Background::Color(colors.sidebar_bg),
            border: Border {
                color: match status {
                    iced::widget::pick_list::Status::Active => colors.border_color,
                    iced::widget::pick_list::Status::Hovered => colors.accent_border,
                    iced::widget::pick_list::Status::Opened => colors.accent_border,
                },
                width: 1.0,
                radius: 6.0.into(),
            },
            handle_color: colors.text_secondary,
            placeholder_color: colors.text_muted,
        }
    }
}

// 返回闭包的动态按钮样式函数
pub fn button_primary_style_dynamic(theme: &ThemeVariant) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    let colors = get_theme_colors(theme);
    move |_theme, status| {
        let (background, text_color) = match status {
            button::Status::Hovered => (colors.button_hover, colors.text_primary),
            button::Status::Pressed => (colors.button_active, colors.text_primary),
            _ => (colors.button_primary, colors.text_primary),
        };
        
        button::Style {
            background: Some(Background::Color(background)),
            text_color,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 6.0.into(),
            },
            shadow: Shadow {
                color: Color::BLACK,
                offset: iced::Vector::new(0.0, 1.0),
                blur_radius: 2.0,
            },
        }
    }
}

pub fn button_secondary_style_dynamic(theme: &ThemeVariant) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    let colors = get_theme_colors(theme);
    move |_theme, status| {
        let (background, text_color) = match status {
            button::Status::Hovered => (colors.sidebar_hover, colors.text_primary),
            button::Status::Pressed => (colors.dark_bg_secondary, colors.text_primary),
            _ => (Color::TRANSPARENT, colors.text_secondary),
        };
        
        button::Style {
            background: Some(Background::Color(background)),
            text_color,
            border: Border {
                color: colors.border_color,
                width: 1.0,
                radius: 4.0.into(),
            },
            shadow: Shadow::default(),
        }
    }
}

pub fn button_danger_style_dynamic(theme: &ThemeVariant) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    let colors = get_theme_colors(theme);
    move |_theme, status| {
        let (background, text_color) = match status {
            button::Status::Hovered => (Color::from_rgb(0.85, 0.3, 0.3), colors.text_primary),
            button::Status::Pressed => (Color::from_rgb(0.75, 0.2, 0.2), colors.text_primary),
            _ => (colors.error_color, colors.text_primary),
        };
        
        button::Style {
            background: Some(Background::Color(background)),
            text_color,
            border: Border {
                color: colors.error_color,
                width: 1.0,
                radius: 4.0.into(),
            },
            shadow: Shadow::default(),
        }
    }
}
