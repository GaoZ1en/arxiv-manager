// UI组件模块

pub mod tab_bar;
pub mod sidebar;
pub mod command_palette;
pub mod paper_card;

// 重新导出主要组件
pub use tab_bar::TabBar;
pub use sidebar::Sidebar;
pub use command_palette::CommandPalette;
pub use paper_card::PaperCard;
