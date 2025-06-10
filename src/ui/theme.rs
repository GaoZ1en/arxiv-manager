use iced::{Color, Theme};

pub fn gruvbox_dark() -> Theme {
    Theme::custom("Gruvbox Dark".to_string(), iced::theme::Palette {
        background: Color::from_rgb(0.157, 0.157, 0.157), // #282828
        text: Color::from_rgb(0.922, 0.859, 0.698),        // #ebdbb2
        primary: Color::from_rgb(0.522, 0.600, 0.259),     // #83a142
        success: Color::from_rgb(0.722, 0.733, 0.149),     // #b8bb26
        danger: Color::from_rgb(0.984, 0.286, 0.204),      // #fb4934
    })
}

pub fn gruvbox_light() -> Theme {
    Theme::custom("Gruvbox Light".to_string(), iced::theme::Palette {
        background: Color::from_rgb(0.984, 0.937, 0.827), // #fbf1d3
        text: Color::from_rgb(0.251, 0.200, 0.114),        // #3c3836
        primary: Color::from_rgb(0.522, 0.600, 0.259),     // #83a142
        success: Color::from_rgb(0.596, 0.643, 0.000),     // #98971a
        danger: Color::from_rgb(0.800, 0.141, 0.114),      // #cc241d
    })
}
