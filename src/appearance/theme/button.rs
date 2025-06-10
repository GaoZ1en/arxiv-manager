use iced::widget::button::{Catalog, Status, Style, StyleFn};
use iced::{Background, Border, Color};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> StyleFn<'a, Self> {
        Box::new(primary)
    }

    fn style(&self, class: &StyleFn<'_, Self>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn primary(theme: &Theme, status: Status) -> Style {
    let button = &theme.colors.buttons.primary;
    
    let background = match status {
        Status::Hovered | Status::Pressed => button.background_hover,
        _ => button.background,
    };

    Style {
        background: Some(Background::Color(background)),
        text_color: button.text,
        border: Border {
            width: 1.0,
            color: button.border,
            radius: 4.0.into(),
        },
        ..Default::default()
    }
}

pub fn secondary(theme: &Theme, status: Status) -> Style {
    let button = &theme.colors.buttons.secondary;
    
    let background = match status {
        Status::Hovered | Status::Pressed => button.background_hover,
        _ => button.background,
    };

    Style {
        background: Some(Background::Color(background)),
        text_color: button.text,
        border: Border {
            width: 1.0,
            color: button.border,
            radius: 4.0.into(),
        },
        ..Default::default()
    }
}

pub fn danger(theme: &Theme, status: Status) -> Style {
    let background = match status {
        Status::Hovered | Status::Pressed => Color::from_rgb(0.9, 0.3, 0.3),
        _ => theme.colors.text.error,
    };

    Style {
        background: Some(Background::Color(background)),
        text_color: Color::WHITE,
        border: Border {
            width: 1.0,
            color: theme.colors.text.error,
            radius: 4.0.into(),
        },
        ..Default::default()
    }
}
