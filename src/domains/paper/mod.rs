// 论文管理域 - Paper Domain
// 负责论文实体的生命周期管理、元数据处理和业务规则

pub mod models;       // 领域模型：Paper实体、元数据值对象
pub mod services;     // 领域服务：论文管理服务、元数据提取
pub mod repositories; // 存储库接口：论文持久化抽象
pub mod events;       // 领域事件：论文状态变化事件

// 重新导出公共接口
pub use models::*;
pub use services::*;
pub use repositories::*;
pub use events::*;
