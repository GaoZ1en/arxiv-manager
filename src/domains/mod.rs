// 领域驱动设计 - 功能域根模块
// Domain-Driven Design - Functional Domains Root Module

pub mod paper;      // 📄 论文管理域
// pub mod search;     // 🔍 搜索域 - 待实现
// pub mod download;   // ⬇️ 下载域 - 待实现
// pub mod library;    // 📚 库管理域 - 待实现
// pub mod user;       // 👤 用户域 - 待实现

// 重新导出核心域类型供应用层使用
pub use paper::*;
