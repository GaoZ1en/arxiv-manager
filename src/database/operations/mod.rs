// 数据库操作模块

pub mod schema;
pub mod paper_ops;
pub mod search_ops;

// 新的CRUD操作模块
pub mod create;
pub mod read;
pub mod update;
pub mod delete;

// 重新导出模式相关函数
pub use schema::*;

// 重新导出新的CRUD操作（这些将替代旧的paper_ops）
pub use create::*;
pub use read::*;
pub use update::*;
pub use delete::*;

// 重新导出搜索操作
pub use search_ops::*;
