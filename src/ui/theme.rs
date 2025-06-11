// 主题系统 - 支持多种经典主题

use iced::Color;
use crate::core::models::Theme as ThemeVariant;

// 主题颜色定义结构
#[derive(Debug, Clone, Copy)]
pub struct ThemeColors {
    pub dark_bg: Color,
    pub dark_bg_secondary: Color,
    pub sidebar_bg: Color,
    pub sidebar_hover: Color,
    pub border_color: Color,
    pub accent_border: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_muted: Color,
    pub success_color: Color,
    pub error_color: Color,
    pub info_color: Color,
    pub button_primary: Color,
    pub button_hover: Color,
    pub button_active: Color,
}

// 获取当前主题的颜色
pub fn get_theme_colors(theme: &ThemeVariant) -> ThemeColors {
    match theme {
        ThemeVariant::ModernDark => MODERN_DARK,
        ThemeVariant::ModernLight => MODERN_LIGHT,
        ThemeVariant::GruvboxDark => GRUVBOX_DARK,
        ThemeVariant::GruvboxLight => GRUVBOX_LIGHT,
        ThemeVariant::GruvboxMaterial => GRUVBOX_MATERIAL,
        ThemeVariant::CatppuccinMocha => CATPPUCCIN_MOCHA,
        ThemeVariant::CatppuccinMacchiato => CATPPUCCIN_MACCHIATO,
        ThemeVariant::CatppuccinFrappe => CATPPUCCIN_FRAPPE,
        ThemeVariant::CatppuccinLatte => CATPPUCCIN_LATTE,
        ThemeVariant::SolarizedDark => SOLARIZED_DARK,
        ThemeVariant::SolarizedLight => SOLARIZED_LIGHT,
        ThemeVariant::Dracula => DRACULA,
        ThemeVariant::Nord => NORD,
        ThemeVariant::NordLight => NORD_LIGHT,
        ThemeVariant::OneDark => ONE_DARK,
        ThemeVariant::OneLight => ONE_LIGHT,
        ThemeVariant::GitHubDark => GITHUB_DARK,
        ThemeVariant::GitHubLight => GITHUB_LIGHT,
        ThemeVariant::Dark => CLASSIC_DARK,
        ThemeVariant::Light => CLASSIC_LIGHT,
        ThemeVariant::TokyoNight => TOKYO_NIGHT,
        ThemeVariant::TokyoNightLight => TOKYO_NIGHT_LIGHT,
        ThemeVariant::AyuDark => AYU_DARK,
        ThemeVariant::AyuMirage => AYU_MIRAGE,
        ThemeVariant::AyuLight => AYU_LIGHT,
    }
}

// Modern Dark 主题 (默认IRC风格)
const MODERN_DARK: ThemeColors = ThemeColors {
    dark_bg: Color::from_rgb(0.08, 0.10, 0.12),         // #141a1f
    dark_bg_secondary: Color::from_rgb(0.11, 0.13, 0.15), // #1c2126
    sidebar_bg: Color::from_rgb(0.06, 0.08, 0.10),      // #0f1419
    sidebar_hover: Color::from_rgb(0.13, 0.15, 0.17),   // #212529
    border_color: Color::from_rgb(0.18, 0.20, 0.22),    // #2e3338
    accent_border: Color::from_rgb(0.25, 0.47, 0.75),   // #4078c0
    text_primary: Color::from_rgb(0.95, 0.96, 0.97),    // #f3f4f6
    text_secondary: Color::from_rgb(0.70, 0.73, 0.76),  // #b3bac1
    text_muted: Color::from_rgb(0.50, 0.54, 0.58),      // #808a94
    success_color: Color::from_rgb(0.26, 0.70, 0.29),   // #42b349
    error_color: Color::from_rgb(0.84, 0.24, 0.29),     // #d73a49
    info_color: Color::from_rgb(0.25, 0.47, 0.75),      // #4078c0
    button_primary: Color::from_rgb(0.25, 0.47, 0.75),  // #4078c0
    button_hover: Color::from_rgb(0.30, 0.52, 0.80),    // #4d85cc
    button_active: Color::from_rgb(0.20, 0.42, 0.70),   // #336bb3
};

// Modern Light 主题
const MODERN_LIGHT: ThemeColors = ThemeColors {
    dark_bg: Color::from_rgb(0.98, 0.98, 0.99),         // #fafbfc
    dark_bg_secondary: Color::from_rgb(0.96, 0.97, 0.98), // #f6f8fa
    sidebar_bg: Color::from_rgb(0.94, 0.95, 0.96),      // #f0f3f6
    sidebar_hover: Color::from_rgb(0.91, 0.93, 0.95),   // #e8edf2
    border_color: Color::from_rgb(0.84, 0.87, 0.90),    // #d7dee5
    accent_border: Color::from_rgb(0.25, 0.47, 0.75),   // #4078c0
    text_primary: Color::from_rgb(0.14, 0.16, 0.18),    // #24292f
    text_secondary: Color::from_rgb(0.40, 0.44, 0.48),  // #656d76
    text_muted: Color::from_rgb(0.55, 0.60, 0.65),      // #8c959f
    success_color: Color::from_rgb(0.13, 0.52, 0.16),   // #218518
    error_color: Color::from_rgb(0.82, 0.19, 0.25),     // #d1303f
    info_color: Color::from_rgb(0.25, 0.47, 0.75),      // #4078c0
    button_primary: Color::from_rgb(0.25, 0.47, 0.75),  // #4078c0
    button_hover: Color::from_rgb(0.20, 0.42, 0.70),    // #336bb3
    button_active: Color::from_rgb(0.15, 0.37, 0.65),   // #2660a4
};

// Gruvbox Dark 主题
const GRUVBOX_DARK: ThemeColors = ThemeColors {
    dark_bg: Color::from_rgb(0.16, 0.14, 0.12),         // #282828
    dark_bg_secondary: Color::from_rgb(0.20, 0.17, 0.15), // #3c3836
    sidebar_bg: Color::from_rgb(0.12, 0.11, 0.09),      // #1d2021
    sidebar_hover: Color::from_rgb(0.24, 0.21, 0.18),   // #504945
    border_color: Color::from_rgb(0.29, 0.26, 0.22),    // #665c54
    accent_border: Color::from_rgb(0.53, 0.75, 0.42),   // #98d982
    text_primary: Color::from_rgb(0.92, 0.86, 0.70),    // #ebdbb2
    text_secondary: Color::from_rgb(0.78, 0.72, 0.55),  // #d5c4a1
    text_muted: Color::from_rgb(0.60, 0.55, 0.42),      // #bdae93
    success_color: Color::from_rgb(0.60, 0.74, 0.42),   // #b8bb26
    error_color: Color::from_rgb(0.98, 0.34, 0.35),     // #fb4934
    info_color: Color::from_rgb(0.33, 0.71, 0.83),      // #83a598
    button_primary: Color::from_rgb(0.53, 0.75, 0.42),  // #98d982
    button_hover: Color::from_rgb(0.60, 0.74, 0.42),    // #b8bb26
    button_active: Color::from_rgb(0.45, 0.67, 0.34),   // #8ec07c
};

// Gruvbox Light 主题
const GRUVBOX_LIGHT: ThemeColors = ThemeColors {
    dark_bg: Color::from_rgb(0.98, 0.95, 0.91),         // #fbf1c7
    dark_bg_secondary: Color::from_rgb(0.96, 0.93, 0.89), // #f2e5bc
    sidebar_bg: Color::from_rgb(0.99, 0.96, 0.92),      // #f9f5d7
    sidebar_hover: Color::from_rgb(0.93, 0.89, 0.84),   // #ebdbb2
    border_color: Color::from_rgb(0.85, 0.81, 0.75),    // #d5c4a1
    accent_border: Color::from_rgb(0.07, 0.45, 0.02),   // #427b58
    text_primary: Color::from_rgb(0.25, 0.22, 0.17),    // #3c3836
    text_secondary: Color::from_rgb(0.40, 0.35, 0.29),  // #665c54
    text_muted: Color::from_rgb(0.55, 0.49, 0.42),      // #7c6f64
    success_color: Color::from_rgb(0.48, 0.62, 0.00),   // #79740e
    error_color: Color::from_rgb(0.80, 0.14, 0.11),     // #cc241d
    info_color: Color::from_rgb(0.07, 0.45, 0.02),      // #427b58
    button_primary: Color::from_rgb(0.07, 0.45, 0.02),  // #427b58
    button_hover: Color::from_rgb(0.48, 0.62, 0.00),    // #79740e
    button_active: Color::from_rgb(0.00, 0.35, 0.00),   // #689d6a
};

// Catppuccin Mocha 主题
const CATPPUCCIN_MOCHA: ThemeColors = ThemeColors {
    dark_bg: Color::from_rgb(0.11, 0.11, 0.16),         // #1e1e2e
    dark_bg_secondary: Color::from_rgb(0.12, 0.13, 0.19), // #313244
    sidebar_bg: Color::from_rgb(0.09, 0.09, 0.13),      // #181825
    sidebar_hover: Color::from_rgb(0.15, 0.16, 0.22),   // #45475a
    border_color: Color::from_rgb(0.31, 0.32, 0.40),    // #585b70
    accent_border: Color::from_rgb(0.53, 0.70, 0.97),   // #89b4fa
    text_primary: Color::from_rgb(0.80, 0.84, 0.91),    // #cdd6f4
    text_secondary: Color::from_rgb(0.73, 0.76, 0.84),  // #bac2de
    text_muted: Color::from_rgb(0.66, 0.69, 0.78),      // #a6adc8
    success_color: Color::from_rgb(0.65, 0.89, 0.63),   // #a6e3a1
    error_color: Color::from_rgb(0.95, 0.55, 0.64),     // #f38ba8
    info_color: Color::from_rgb(0.53, 0.70, 0.97),      // #89b4fa
    button_primary: Color::from_rgb(0.53, 0.70, 0.97),  // #89b4fa
    button_hover: Color::from_rgb(0.46, 0.63, 0.95),    // #74c7ec
    button_active: Color::from_rgb(0.40, 0.56, 0.88),   // #89dceb
};

// Catppuccin Latte 主题 (浅色)
const CATPPUCCIN_LATTE: ThemeColors = ThemeColors {
    dark_bg: Color::from_rgb(0.94, 0.95, 0.97),         // #eff1f5
    dark_bg_secondary: Color::from_rgb(0.91, 0.93, 0.95), // #e6e9ef
    sidebar_bg: Color::from_rgb(0.96, 0.97, 0.98),      // #f4f6f8
    sidebar_hover: Color::from_rgb(0.87, 0.89, 0.92),   // #dce0e8
    border_color: Color::from_rgb(0.76, 0.80, 0.86),    // #acb0be
    accent_border: Color::from_rgb(0.12, 0.45, 0.96),   // #1e66f5
    text_primary: Color::from_rgb(0.30, 0.34, 0.42),    // #4c4f69
    text_secondary: Color::from_rgb(0.42, 0.46, 0.54),  // #6c6f85
    text_muted: Color::from_rgb(0.54, 0.58, 0.66),      // #8c8fa1
    success_color: Color::from_rgb(0.25, 0.68, 0.29),   // #40a02b
    error_color: Color::from_rgb(0.82, 0.11, 0.26),     // #d20f39
    info_color: Color::from_rgb(0.12, 0.45, 0.96),      // #1e66f5
    button_primary: Color::from_rgb(0.12, 0.45, 0.96),  // #1e66f5
    button_hover: Color::from_rgb(0.00, 0.35, 0.88),    // #04a5e5
    button_active: Color::from_rgb(0.00, 0.28, 0.78),   // #209fb5
};

// Solarized Dark 主题
const SOLARIZED_DARK: ThemeColors = ThemeColors {
    dark_bg: Color::from_rgb(0.00, 0.17, 0.21),         // #002b36
    dark_bg_secondary: Color::from_rgb(0.03, 0.21, 0.26), // #073642
    sidebar_bg: Color::from_rgb(0.00, 0.14, 0.18),      // #002a35
    sidebar_hover: Color::from_rgb(0.05, 0.24, 0.29),   // #586e75
    border_color: Color::from_rgb(0.35, 0.43, 0.46),    // #586e75
    accent_border: Color::from_rgb(0.15, 0.55, 0.82),   // #268bd2
    text_primary: Color::from_rgb(0.51, 0.58, 0.59),    // #839496
    text_secondary: Color::from_rgb(0.42, 0.48, 0.51),  // #657b83
    text_muted: Color::from_rgb(0.35, 0.43, 0.46),      // #586e75
    success_color: Color::from_rgb(0.52, 0.60, 0.00),   // #859900
    error_color: Color::from_rgb(0.86, 0.20, 0.18),     // #dc322f
    info_color: Color::from_rgb(0.15, 0.55, 0.82),      // #268bd2
    button_primary: Color::from_rgb(0.15, 0.55, 0.82),  // #268bd2
    button_hover: Color::from_rgb(0.20, 0.60, 0.85),    // #2aa198
    button_active: Color::from_rgb(0.10, 0.50, 0.78),   // #b58900
};

// Solarized Light 主题
const SOLARIZED_LIGHT: ThemeColors = ThemeColors {
    dark_bg: Color::from_rgb(0.99, 0.96, 0.89),         // #fdf6e3
    dark_bg_secondary: Color::from_rgb(0.93, 0.91, 0.84), // #eee8d5
    sidebar_bg: Color::from_rgb(1.00, 0.97, 0.91),      // #fff9e7
    sidebar_hover: Color::from_rgb(0.89, 0.86, 0.79),   // #e3dcc6
    border_color: Color::from_rgb(0.83, 0.81, 0.74),    // #d3cbb0
    accent_border: Color::from_rgb(0.15, 0.55, 0.82),   // #268bd2
    text_primary: Color::from_rgb(0.40, 0.48, 0.51),    // #657b83
    text_secondary: Color::from_rgb(0.51, 0.58, 0.59),  // #839496
    text_muted: Color::from_rgb(0.58, 0.63, 0.63),      // #93a1a1
    success_color: Color::from_rgb(0.52, 0.60, 0.00),   // #859900
    error_color: Color::from_rgb(0.86, 0.20, 0.18),     // #dc322f
    info_color: Color::from_rgb(0.15, 0.55, 0.82),      // #268bd2
    button_primary: Color::from_rgb(0.15, 0.55, 0.82),  // #268bd2
    button_hover: Color::from_rgb(0.20, 0.60, 0.85),    // #2aa198
    button_active: Color::from_rgb(0.10, 0.50, 0.78),   // #b58900
};

// Gruvbox Material 主题 (Material变体)
const GRUVBOX_MATERIAL: ThemeColors = ThemeColors {
    dark_bg: Color::from_rgb(0.15, 0.13, 0.11),         // #282828 -> #1d2021
    dark_bg_secondary: Color::from_rgb(0.19, 0.16, 0.14), // #32302f
    sidebar_bg: Color::from_rgb(0.11, 0.10, 0.08),      // #1d1f21
    sidebar_hover: Color::from_rgb(0.23, 0.20, 0.17),   // #504945
    border_color: Color::from_rgb(0.27, 0.24, 0.20),    // #665c54
    accent_border: Color::from_rgb(0.56, 0.76, 0.45),   // #a9b665
    text_primary: Color::from_rgb(0.85, 0.80, 0.64),    // #d4be98
    text_secondary: Color::from_rgb(0.70, 0.65, 0.50),  // #a89984
    text_muted: Color::from_rgb(0.55, 0.50, 0.37),      // #928374
    success_color: Color::from_rgb(0.66, 0.78, 0.27),   // #a9b665
    error_color: Color::from_rgb(0.92, 0.44, 0.42),     // #ea6962
    info_color: Color::from_rgb(0.33, 0.78, 0.84),      // #89b4fa
    button_primary: Color::from_rgb(0.56, 0.76, 0.45),  // #a9b665
    button_hover: Color::from_rgb(0.66, 0.78, 0.27),    // #a9b665
    button_active: Color::from_rgb(0.45, 0.66, 0.35),   // #7daea3
};

// Dracula 主题
const DRACULA: ThemeColors = ThemeColors {
    dark_bg: Color::from_rgb(0.16, 0.16, 0.21),         // #282a36
    dark_bg_secondary: Color::from_rgb(0.20, 0.20, 0.25), // #343746
    sidebar_bg: Color::from_rgb(0.14, 0.14, 0.18),      // #21222c
    sidebar_hover: Color::from_rgb(0.25, 0.25, 0.30),   // #44475a
    border_color: Color::from_rgb(0.27, 0.28, 0.35),    // #44475a
    accent_border: Color::from_rgb(0.74, 0.58, 1.00),   // #bd93f9
    text_primary: Color::from_rgb(0.95, 0.95, 0.95),    // #f8f8f2
    text_secondary: Color::from_rgb(0.73, 0.73, 0.73),  // #bfbfbf
    text_muted: Color::from_rgb(0.42, 0.44, 0.54),      // #6272a4
    success_color: Color::from_rgb(0.31, 0.98, 0.48),   // #50fa7b
    error_color: Color::from_rgb(1.00, 0.34, 0.34),     // #ff5555
    info_color: Color::from_rgb(0.56, 0.85, 1.00),      // #8be9fd
    button_primary: Color::from_rgb(0.74, 0.58, 1.00),  // #bd93f9
    button_hover: Color::from_rgb(0.84, 0.68, 1.00),    // #d6acff
    button_active: Color::from_rgb(0.64, 0.48, 0.90),   // #a485e6
};

// Nord 主题
const NORD: ThemeColors = ThemeColors {
    dark_bg: Color::from_rgb(0.18, 0.20, 0.25),         // #2e3440
    dark_bg_secondary: Color::from_rgb(0.21, 0.24, 0.29), // #3b4252
    sidebar_bg: Color::from_rgb(0.15, 0.17, 0.21),      // #2e3440
    sidebar_hover: Color::from_rgb(0.26, 0.30, 0.37),   // #434c5e
    border_color: Color::from_rgb(0.30, 0.34, 0.42),    // #4c566a
    accent_border: Color::from_rgb(0.53, 0.75, 0.82),   // #88c0d0
    text_primary: Color::from_rgb(0.93, 0.94, 0.96),    // #eceff4
    text_secondary: Color::from_rgb(0.85, 0.87, 0.91),  // #d8dee9
    text_muted: Color::from_rgb(0.68, 0.73, 0.80),      // #a3be8c
    success_color: Color::from_rgb(0.64, 0.75, 0.55),   // #a3be8c
    error_color: Color::from_rgb(0.75, 0.38, 0.42),     // #bf616a
    info_color: Color::from_rgb(0.53, 0.75, 0.82),      // #88c0d0
    button_primary: Color::from_rgb(0.53, 0.75, 0.82),  // #88c0d0
    button_hover: Color::from_rgb(0.63, 0.85, 0.92),    // #a3e4f0
    button_active: Color::from_rgb(0.43, 0.65, 0.72),   // #6eb5c0
};

// Nord Light 主题
const NORD_LIGHT: ThemeColors = ThemeColors {
    dark_bg: Color::from_rgb(0.93, 0.94, 0.96),         // #eceff4
    dark_bg_secondary: Color::from_rgb(0.90, 0.92, 0.94), // #e5e9f0
    sidebar_bg: Color::from_rgb(0.96, 0.97, 0.98),      // #f5f6f8
    sidebar_hover: Color::from_rgb(0.85, 0.87, 0.91),   // #d8dee9
    border_color: Color::from_rgb(0.76, 0.81, 0.88),    // #c2d0e7
    accent_border: Color::from_rgb(0.53, 0.75, 0.82),   // #88c0d0
    text_primary: Color::from_rgb(0.18, 0.20, 0.25),    // #2e3440
    text_secondary: Color::from_rgb(0.26, 0.30, 0.37),  // #434c5e
    text_muted: Color::from_rgb(0.30, 0.34, 0.42),      // #4c566a
    success_color: Color::from_rgb(0.54, 0.65, 0.45),   // #8aa67c
    error_color: Color::from_rgb(0.65, 0.28, 0.32),     // #a5464a
    info_color: Color::from_rgb(0.43, 0.65, 0.72),      // #6eb5c0
    button_primary: Color::from_rgb(0.53, 0.75, 0.82),  // #88c0d0
    button_hover: Color::from_rgb(0.43, 0.65, 0.72),    // #6eb5c0
    button_active: Color::from_rgb(0.33, 0.55, 0.62),   // #5699ad
};

// One Dark 主题
const ONE_DARK: ThemeColors = ThemeColors {
    dark_bg: Color::from_rgb(0.16, 0.17, 0.20),         // #282c34
    dark_bg_secondary: Color::from_rgb(0.19, 0.20, 0.24), // #31353f
    sidebar_bg: Color::from_rgb(0.13, 0.14, 0.17),      // #21252b
    sidebar_hover: Color::from_rgb(0.22, 0.24, 0.28),   // #383e49
    border_color: Color::from_rgb(0.32, 0.35, 0.42),    // #528bff
    accent_border: Color::from_rgb(0.32, 0.55, 1.00),   // #528bff
    text_primary: Color::from_rgb(0.67, 0.71, 0.78),    // #abb2bf
    text_secondary: Color::from_rgb(0.55, 0.59, 0.66),  // #8c92a3
    text_muted: Color::from_rgb(0.42, 0.46, 0.52),      // #6b7280
    success_color: Color::from_rgb(0.60, 0.76, 0.38),   // #98c379
    error_color: Color::from_rgb(0.88, 0.33, 0.33),     // #e06c75
    info_color: Color::from_rgb(0.38, 0.68, 0.84),      // #61afef
    button_primary: Color::from_rgb(0.32, 0.55, 1.00),  // #528bff
    button_hover: Color::from_rgb(0.42, 0.65, 1.00),    // #6b9bff
    button_active: Color::from_rgb(0.22, 0.45, 0.90),   // #387bef
};

// One Light 主题
const ONE_LIGHT: ThemeColors = ThemeColors {
    dark_bg: Color::from_rgb(0.98, 0.98, 0.99),         // #fafafa
    dark_bg_secondary: Color::from_rgb(0.96, 0.96, 0.97), // #f5f5f6
    sidebar_bg: Color::from_rgb(0.99, 0.99, 1.00),      // #fcfcfc
    sidebar_hover: Color::from_rgb(0.92, 0.93, 0.94),   // #ebebec
    border_color: Color::from_rgb(0.87, 0.88, 0.90),    // #dfe1e5
    accent_border: Color::from_rgb(0.25, 0.47, 0.82),   // #4078f2
    text_primary: Color::from_rgb(0.24, 0.29, 0.34),    // #383a42
    text_secondary: Color::from_rgb(0.40, 0.45, 0.52),  // #696c77
    text_muted: Color::from_rgb(0.55, 0.60, 0.67),      // #8e939b
    success_color: Color::from_rgb(0.33, 0.63, 0.20),   // #50a14f
    error_color: Color::from_rgb(0.82, 0.18, 0.25),     // #ca1243
    info_color: Color::from_rgb(0.16, 0.50, 0.78),      // #0184bc
    button_primary: Color::from_rgb(0.25, 0.47, 0.82),  // #4078f2
    button_hover: Color::from_rgb(0.16, 0.37, 0.72),    // #2a5cc6
    button_active: Color::from_rgb(0.06, 0.27, 0.62),   // #1046a3
};

// GitHub Dark 主题
const GITHUB_DARK: ThemeColors = ThemeColors {
    dark_bg: Color::from_rgb(0.05, 0.07, 0.09),         // #0d1117
    dark_bg_secondary: Color::from_rgb(0.08, 0.10, 0.12), // #161b22
    sidebar_bg: Color::from_rgb(0.03, 0.05, 0.06),      // #010409
    sidebar_hover: Color::from_rgb(0.13, 0.16, 0.20),   // #21262d
    border_color: Color::from_rgb(0.19, 0.23, 0.28),    // #30363d
    accent_border: Color::from_rgb(0.36, 0.68, 1.00),   // #58a6ff
    text_primary: Color::from_rgb(0.95, 0.96, 0.98),    // #f0f6fc
    text_secondary: Color::from_rgb(0.81, 0.85, 0.89),  // #c9d1d9
    text_muted: Color::from_rgb(0.58, 0.64, 0.70),      // #8b949e
    success_color: Color::from_rgb(0.17, 0.81, 0.35),   // #2da44e
    error_color: Color::from_rgb(0.98, 0.27, 0.27),     // #f85149
    info_color: Color::from_rgb(0.36, 0.68, 1.00),      // #58a6ff
    button_primary: Color::from_rgb(0.14, 0.63, 0.18),  // #238636
    button_hover: Color::from_rgb(0.17, 0.71, 0.21),    // #2ea043
    button_active: Color::from_rgb(0.11, 0.55, 0.15),   // #1c7e29
};

// GitHub Light 主题
const GITHUB_LIGHT: ThemeColors = ThemeColors {
    dark_bg: Color::from_rgb(1.00, 1.00, 1.00),         // #ffffff
    dark_bg_secondary: Color::from_rgb(0.98, 0.98, 0.99), // #f6f8fa
    sidebar_bg: Color::from_rgb(1.00, 1.00, 1.00),      // #ffffff
    sidebar_hover: Color::from_rgb(0.95, 0.96, 0.98),   // #f3f4f6
    border_color: Color::from_rgb(0.84, 0.87, 0.90),    // #d0d7de
    accent_border: Color::from_rgb(0.02, 0.39, 0.78),   // #0969da
    text_primary: Color::from_rgb(0.09, 0.11, 0.15),    // #1f2328
    text_secondary: Color::from_rgb(0.40, 0.44, 0.49),  // #656d76
    text_muted: Color::from_rgb(0.55, 0.60, 0.67),      // #8c959f
    success_color: Color::from_rgb(0.10, 0.64, 0.18),   // #1a7f37
    error_color: Color::from_rgb(0.82, 0.10, 0.14),     // #d1242f
    info_color: Color::from_rgb(0.02, 0.39, 0.78),      // #0969da
    button_primary: Color::from_rgb(0.10, 0.64, 0.18),  // #1a7f37
    button_hover: Color::from_rgb(0.14, 0.68, 0.22),    // #2da44e
    button_active: Color::from_rgb(0.06, 0.56, 0.14),   // #1c7e29
};

// Tokyo Night 主题
const TOKYO_NIGHT: ThemeColors = ThemeColors {
    dark_bg: Color::from_rgb(0.09, 0.10, 0.15),         // #1a1b26
    dark_bg_secondary: Color::from_rgb(0.13, 0.14, 0.20), // #24283b
    sidebar_bg: Color::from_rgb(0.07, 0.08, 0.12),      // #16161e
    sidebar_hover: Color::from_rgb(0.18, 0.20, 0.27),   // #2f3549
    border_color: Color::from_rgb(0.28, 0.31, 0.40),    // #414868
    accent_border: Color::from_rgb(0.45, 0.69, 1.00),   // #7aa2f7
    text_primary: Color::from_rgb(0.77, 0.82, 0.96),    // #c0caf5
    text_secondary: Color::from_rgb(0.65, 0.70, 0.84),  // #a9b1d6
    text_muted: Color::from_rgb(0.54, 0.59, 0.74),      // #9aa5ce
    success_color: Color::from_rgb(0.60, 0.87, 0.65),   // #9ece6a
    error_color: Color::from_rgb(0.96, 0.46, 0.52),     // #f7768e
    info_color: Color::from_rgb(0.45, 0.69, 1.00),      // #7aa2f7
    button_primary: Color::from_rgb(0.45, 0.69, 1.00),  // #7aa2f7
    button_hover: Color::from_rgb(0.55, 0.79, 1.00),    // #8cb2ff
    button_active: Color::from_rgb(0.35, 0.59, 0.90),   // #5992e4
};

// Tokyo Night Light 主题
const TOKYO_NIGHT_LIGHT: ThemeColors = ThemeColors {
    dark_bg: Color::from_rgb(0.85, 0.89, 0.95),         // #d5d6db
    dark_bg_secondary: Color::from_rgb(0.89, 0.93, 0.98), // #e1e2e7
    sidebar_bg: Color::from_rgb(0.92, 0.95, 1.00),      // #ececec
    sidebar_hover: Color::from_rgb(0.80, 0.84, 0.90),   // #cbccd1
    border_color: Color::from_rgb(0.70, 0.74, 0.80),    // #b2b5bd
    accent_border: Color::from_rgb(0.20, 0.43, 0.78),   // #34548a
    text_primary: Color::from_rgb(0.23, 0.28, 0.36),    // #3b4261
    text_secondary: Color::from_rgb(0.35, 0.40, 0.50),  // #565a6e
    text_muted: Color::from_rgb(0.48, 0.53, 0.63),      // #7a7ca8
    success_color: Color::from_rgb(0.20, 0.65, 0.25),   // #33635c
    error_color: Color::from_rgb(0.78, 0.20, 0.26),     // #c64343
    info_color: Color::from_rgb(0.20, 0.43, 0.78),      // #34548a
    button_primary: Color::from_rgb(0.20, 0.43, 0.78),  // #34548a
    button_hover: Color::from_rgb(0.10, 0.33, 0.68),    // #1a447a
    button_active: Color::from_rgb(0.00, 0.23, 0.58),   // #003a6b
};

// Ayu Dark 主题
const AYU_DARK: ThemeColors = ThemeColors {
    dark_bg: Color::from_rgb(0.05, 0.07, 0.09),         // #0a0e14
    dark_bg_secondary: Color::from_rgb(0.08, 0.10, 0.13), // #131721
    sidebar_bg: Color::from_rgb(0.03, 0.05, 0.07),      // #07090f
    sidebar_hover: Color::from_rgb(0.12, 0.15, 0.19),   // #1f2430
    border_color: Color::from_rgb(0.19, 0.23, 0.29),    // #2d3640
    accent_border: Color::from_rgb(1.00, 0.67, 0.27),   // #ffb454
    text_primary: Color::from_rgb(0.73, 0.74, 0.78),    // #b3b1ad
    text_secondary: Color::from_rgb(0.60, 0.62, 0.66),  // #999693
    text_muted: Color::from_rgb(0.47, 0.49, 0.53),      // #787b86
    success_color: Color::from_rgb(0.74, 0.88, 0.40),   // #bae67e
    error_color: Color::from_rgb(0.94, 0.35, 0.35),     // #f07178
    info_color: Color::from_rgb(0.25, 0.82, 0.96),      // #39bae6
    button_primary: Color::from_rgb(1.00, 0.67, 0.27),  // #ffb454
    button_hover: Color::from_rgb(1.00, 0.77, 0.37),    // #ffc464
    button_active: Color::from_rgb(0.90, 0.57, 0.17),   // #e59244
};

// Ayu Mirage 主题
const AYU_MIRAGE: ThemeColors = ThemeColors {
    dark_bg: Color::from_rgb(0.10, 0.12, 0.16),         // #1f2430
    dark_bg_secondary: Color::from_rgb(0.13, 0.16, 0.21), // #232834
    sidebar_bg: Color::from_rgb(0.08, 0.10, 0.14),      // #151a1e
    sidebar_hover: Color::from_rgb(0.17, 0.20, 0.26),   // #2d3640
    border_color: Color::from_rgb(0.25, 0.29, 0.36),    // #3e4b59
    accent_border: Color::from_rgb(1.00, 0.67, 0.27),   // #ffb454
    text_primary: Color::from_rgb(0.80, 0.81, 0.84),    // #cbccc6
    text_secondary: Color::from_rgb(0.67, 0.68, 0.72),  // #a6acb9
    text_muted: Color::from_rgb(0.54, 0.56, 0.61),      // #8a8f98
    success_color: Color::from_rgb(0.74, 0.88, 0.40),   // #bae67e
    error_color: Color::from_rgb(0.94, 0.35, 0.35),     // #f07178
    info_color: Color::from_rgb(0.25, 0.82, 0.96),      // #39bae6
    button_primary: Color::from_rgb(1.00, 0.67, 0.27),  // #ffb454
    button_hover: Color::from_rgb(1.00, 0.77, 0.37),    // #ffc464
    button_active: Color::from_rgb(0.90, 0.57, 0.17),   // #e59244
};

// Ayu Light 主题
const AYU_LIGHT: ThemeColors = ThemeColors {
    dark_bg: Color::from_rgb(0.98, 0.98, 0.98),         // #fafafa
    dark_bg_secondary: Color::from_rgb(0.96, 0.96, 0.96), // #f3f4f5
    sidebar_bg: Color::from_rgb(1.00, 1.00, 1.00),      // #ffffff
    sidebar_hover: Color::from_rgb(0.93, 0.93, 0.93),   // #ededed
    border_color: Color::from_rgb(0.85, 0.85, 0.85),    // #d9d7ce
    accent_border: Color::from_rgb(1.00, 0.60, 0.00),   // #ff9940
    text_primary: Color::from_rgb(0.24, 0.29, 0.34),    // #5c6773
    text_secondary: Color::from_rgb(0.40, 0.45, 0.52),  // #828c99
    text_muted: Color::from_rgb(0.56, 0.61, 0.68),      // #8f9aae
    success_color: Color::from_rgb(0.47, 0.75, 0.18),   // #86b300
    error_color: Color::from_rgb(0.94, 0.20, 0.20),     // #f51818
    info_color: Color::from_rgb(0.00, 0.60, 0.80),      // #399ee6
    button_primary: Color::from_rgb(1.00, 0.60, 0.00),  // #ff9940
    button_hover: Color::from_rgb(1.00, 0.70, 0.10),    // #ffb350
    button_active: Color::from_rgb(0.90, 0.50, 0.00),   // #e58930
};

// Classic Dark/Light 主题 (原始默认主题)
const CLASSIC_DARK: ThemeColors = ThemeColors {
    dark_bg: Color::from_rgb(0.12, 0.12, 0.12),         // #1e1e1e
    dark_bg_secondary: Color::from_rgb(0.16, 0.16, 0.16), // #2d2d2d
    sidebar_bg: Color::from_rgb(0.09, 0.09, 0.09),      // #171717
    sidebar_hover: Color::from_rgb(0.20, 0.20, 0.20),   // #333333
    border_color: Color::from_rgb(0.25, 0.25, 0.25),    // #404040
    accent_border: Color::from_rgb(0.00, 0.47, 0.84),   // #007acc
    text_primary: Color::from_rgb(0.85, 0.85, 0.85),    // #d4d4d4
    text_secondary: Color::from_rgb(0.70, 0.70, 0.70),  // #b3b3b3
    text_muted: Color::from_rgb(0.55, 0.55, 0.55),      // #8c8c8c
    success_color: Color::from_rgb(0.27, 0.69, 0.31),   // #4caf50
    error_color: Color::from_rgb(0.96, 0.26, 0.21),     // #f44336
    info_color: Color::from_rgb(0.13, 0.59, 0.95),      // #2196f3
    button_primary: Color::from_rgb(0.00, 0.47, 0.84),  // #007acc
    button_hover: Color::from_rgb(0.10, 0.57, 0.94),    // #1976d2
    button_active: Color::from_rgb(0.00, 0.37, 0.74),   // #0d47a1
};

const CLASSIC_LIGHT: ThemeColors = ThemeColors {
    dark_bg: Color::from_rgb(1.00, 1.00, 1.00),         // #ffffff
    dark_bg_secondary: Color::from_rgb(0.97, 0.97, 0.97), // #f5f5f5
    sidebar_bg: Color::from_rgb(0.98, 0.98, 0.98),      // #fafafa
    sidebar_hover: Color::from_rgb(0.93, 0.93, 0.93),   // #eeeeee
    border_color: Color::from_rgb(0.87, 0.87, 0.87),    // #e0e0e0
    accent_border: Color::from_rgb(0.00, 0.47, 0.84),   // #007acc
    text_primary: Color::from_rgb(0.13, 0.13, 0.13),    // #212121
    text_secondary: Color::from_rgb(0.38, 0.38, 0.38),  // #616161
    text_muted: Color::from_rgb(0.62, 0.62, 0.62),      // #9e9e9e
    success_color: Color::from_rgb(0.30, 0.69, 0.31),   // #4caf50
    error_color: Color::from_rgb(0.96, 0.26, 0.21),     // #f44336
    info_color: Color::from_rgb(0.13, 0.59, 0.95),      // #2196f3
    button_primary: Color::from_rgb(0.00, 0.47, 0.84),  // #007acc
    button_hover: Color::from_rgb(0.10, 0.57, 0.94),    // #1976d2
    button_active: Color::from_rgb(0.00, 0.37, 0.74),   // #0d47a1
};

// Catppuccin Macchiato 主题
const CATPPUCCIN_MACCHIATO: ThemeColors = ThemeColors {
    dark_bg: Color::from_rgb(0.14, 0.15, 0.20),         // #24273a
    dark_bg_secondary: Color::from_rgb(0.17, 0.19, 0.24), // #363a4f
    sidebar_bg: Color::from_rgb(0.11, 0.12, 0.17),      // #1e2030
    sidebar_hover: Color::from_rgb(0.21, 0.23, 0.28),   // #494d64
    border_color: Color::from_rgb(0.36, 0.39, 0.45),    // #5b6078
    accent_border: Color::from_rgb(0.55, 0.71, 0.99),   // #8aadf4
    text_primary: Color::from_rgb(0.79, 0.84, 0.92),    // #cad3f5
    text_secondary: Color::from_rgb(0.72, 0.76, 0.85),  // #b8c0e0
    text_muted: Color::from_rgb(0.65, 0.69, 0.79),      // #a5adcb
    success_color: Color::from_rgb(0.64, 0.89, 0.64),   // #a6da95
    error_color: Color::from_rgb(0.93, 0.54, 0.63),     // #ed8796
    info_color: Color::from_rgb(0.55, 0.71, 0.99),      // #8aadf4
    button_primary: Color::from_rgb(0.55, 0.71, 0.99),  // #8aadf4
    button_hover: Color::from_rgb(0.45, 0.61, 0.89),    // #739df2
    button_active: Color::from_rgb(0.35, 0.51, 0.79),   // #5985d3
};

// Catppuccin Frappe 主题
const CATPPUCCIN_FRAPPE: ThemeColors = ThemeColors {
    dark_bg: Color::from_rgb(0.19, 0.20, 0.25),         // #303446
    dark_bg_secondary: Color::from_rgb(0.22, 0.24, 0.29), // #414559
    sidebar_bg: Color::from_rgb(0.16, 0.17, 0.22),      // #292c3c
    sidebar_hover: Color::from_rgb(0.26, 0.28, 0.33),   // #51576d
    border_color: Color::from_rgb(0.37, 0.40, 0.47),    // #626880
    accent_border: Color::from_rgb(0.53, 0.70, 0.97),   // #8caaee
    text_primary: Color::from_rgb(0.78, 0.83, 0.91),    // #c6d0f5
    text_secondary: Color::from_rgb(0.71, 0.75, 0.84),  // #b5bfe2
    text_muted: Color::from_rgb(0.64, 0.68, 0.78),      // #a5adce
    success_color: Color::from_rgb(0.65, 0.89, 0.63),   // #a6d189
    error_color: Color::from_rgb(0.90, 0.54, 0.64),     // #e78284
    info_color: Color::from_rgb(0.53, 0.70, 0.97),      // #8caaee
    button_primary: Color::from_rgb(0.53, 0.70, 0.97),  // #8caaee
    button_hover: Color::from_rgb(0.43, 0.60, 0.87),    // #6e9bd8
    button_active: Color::from_rgb(0.33, 0.50, 0.77),   // #548cc2
};

// 向后兼容的常量别名 (基于Modern Dark主题)
pub const DARK_BG: Color = MODERN_DARK.dark_bg;
pub const DARK_BG_SECONDARY: Color = MODERN_DARK.dark_bg_secondary;
pub const SIDEBAR_BG: Color = MODERN_DARK.sidebar_bg;
pub const SIDEBAR_HOVER: Color = MODERN_DARK.sidebar_hover;
pub const BORDER_COLOR: Color = MODERN_DARK.border_color;
pub const ACCENT_BORDER: Color = MODERN_DARK.accent_border;
pub const TEXT_PRIMARY: Color = MODERN_DARK.text_primary;
pub const TEXT_SECONDARY: Color = MODERN_DARK.text_secondary;
pub const TEXT_MUTED: Color = MODERN_DARK.text_muted;
pub const SUCCESS_COLOR: Color = MODERN_DARK.success_color;
pub const ERROR_COLOR: Color = MODERN_DARK.error_color;
pub const INFO_COLOR: Color = MODERN_DARK.info_color;
pub const BUTTON_PRIMARY: Color = MODERN_DARK.button_primary;
pub const BUTTON_HOVER: Color = MODERN_DARK.button_hover;
pub const BUTTON_ACTIVE: Color = MODERN_DARK.button_active;

// Gruvbox 兼容性别名
pub const GRUVBOX_BG: Color = DARK_BG;
pub const GRUVBOX_SURFACE: Color = SIDEBAR_BG;
pub const GRUVBOX_BORDER: Color = BORDER_COLOR;
pub const GRUVBOX_TEXT: Color = TEXT_PRIMARY;
pub const GRUVBOX_TEXT_MUTED: Color = TEXT_SECONDARY;
pub const GRUVBOX_GREEN: Color = SUCCESS_COLOR;
pub const GRUVBOX_RED: Color = ERROR_COLOR;
pub const GRUVBOX_BLUE: Color = INFO_COLOR;
pub const GRUVBOX_PURPLE: Color = Color::from_rgb(0.69, 0.38, 0.78);  // #b16286

// 主题切换辅助函数
impl ThemeColors {
    /// 获取当前主题的主背景色
    pub fn background(&self) -> Color {
        self.dark_bg
    }
    
    /// 获取当前主题的次要背景色
    pub fn surface(&self) -> Color {
        self.dark_bg_secondary
    }
    
    /// 获取当前主题的边框颜色
    pub fn border(&self) -> Color {
        self.border_color
    }
    
    /// 获取当前主题的主文本颜色
    pub fn text(&self) -> Color {
        self.text_primary
    }
}

// 全局主题切换函数
pub fn apply_theme(theme: &ThemeVariant) -> ThemeColors {
    get_theme_colors(theme)
}

// 主题预览函数
pub fn get_theme_preview(theme: &ThemeVariant) -> (Color, Color, Color) {
    let colors = get_theme_colors(theme);
    (colors.dark_bg, colors.text_primary, colors.button_primary)
}
