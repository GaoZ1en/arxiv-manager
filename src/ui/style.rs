// 按钮样式定义

use iced::{Color, Background, Border, Shadow};
use iced::widget::button;

use crate::ui::theme::*;

pub fn button_primary_style(_theme: &iced::Theme, status: button::Status) -> button::Style {
    let (background, text_color) = match status {
        button::Status::Hovered => (GRUVBOX_GREEN, Color::BLACK),
        _ => (GRUVBOX_GREEN, Color::BLACK),
    };
    
    button::Style {
        background: Some(Background::Color(background)),
        text_color,
        border: Border {
            color: GRUVBOX_GREEN,
            width: 1.0,
            radius: 4.0.into(),
        },
        shadow: Shadow::default(),
    }
}

pub fn button_secondary_style(_theme: &iced::Theme, status: button::Status) -> button::Style {
    let background = match status {
        button::Status::Hovered => Color::from_rgb(0.45, 0.41, 0.35),
        _ => GRUVBOX_SURFACE,
    };
    
    button::Style {
        background: Some(Background::Color(background)),
        text_color: GRUVBOX_TEXT,
        border: Border {
            color: GRUVBOX_BORDER,
            width: 1.0,
            radius: 4.0.into(),
        },
        shadow: Shadow::default(),
    }
}

pub fn button_danger_style(_theme: &iced::Theme, status: button::Status) -> button::Style {
    let background = match status {
        button::Status::Hovered => Color::from_rgb(0.85, 0.3, 0.3),
        _ => GRUVBOX_RED,
    };
    
    button::Style {
        background: Some(Background::Color(background)),
        text_color: Color::WHITE,
        border: Border {
            color: GRUVBOX_RED,
            width: 1.0,
            radius: 4.0.into(),
        },
        shadow: Shadow::default(),
    }
}
