// UI组件模块

pub mod tab_bar;
pub mod sidebar;
pub mod command_palette;
pub mod paper_card;
pub mod waterfall;
pub mod context_menu;
pub mod ai_assistant_panel_simple;

// 重新导出主要组件
pub use tab_bar::TabBar;
pub use sidebar::Sidebar;
pub use command_palette::CommandPalette;
pub use paper_card::PaperCard;
pub use waterfall::WaterfallLayout;
pub use context_menu::{ContextMenu, ContextMenuState};
pub use ai_assistant_panel_simple::AiAssistantPanel;
