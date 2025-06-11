// 应用层 - Application Layer
// 协调领域服务和基础设施层，实现用例和命令查询处理

pub mod use_cases;  // 用例实现
pub mod commands;   // 命令处理
pub mod queries;    // 查询处理
pub mod dto;        // 数据传输对象
pub mod handlers;   // 命令和查询处理器

// 重新导出核心类型（避免循环依赖）
pub use commands::*;
pub use queries::*;
pub use dto::{ApplicationError, ApplicationResult};
