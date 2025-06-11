// 数据库模型模块

pub mod paper_record;
pub mod download_status;

// 重新导出所有公共类型
pub use paper_record::*;
pub use download_status::*;
