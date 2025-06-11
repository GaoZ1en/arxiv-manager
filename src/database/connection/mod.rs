// 数据库连接管理模块

use rusqlite::Connection;
use std::path::Path;
use crate::utils::Result;

/// 数据库连接配置
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    /// 数据库文件路径
    pub db_path: String,
    /// 连接超时时间（秒）
    pub timeout_seconds: u64,
    /// 是否启用WAL模式
    pub enable_wal: bool,
    /// 最大连接重试次数
    pub max_retries: u32,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            db_path: "arxiv_manager.db".to_string(),
            timeout_seconds: 30,
            enable_wal: true,
            max_retries: 3,
        }
    }
}

/// 数据库连接管理器
#[derive(Debug)]
pub struct ConnectionManager {
    config: DatabaseConfig,
}

impl ConnectionManager {
    /// 创建新的连接管理器
    pub fn new(config: DatabaseConfig) -> Self {
        Self { config }
    }
    
    /// 创建数据库连接
    pub fn create_connection(&self) -> Result<Connection> {
        let mut last_error = None;
        
        for attempt in 0..=self.config.max_retries {
            match self.try_connect() {
                Ok(conn) => {
                    if attempt > 0 {
                        log::info!("数据库连接成功，重试次数: {}", attempt);
                    }
                    return Ok(conn);
                }
                Err(e) => {
                    last_error = Some(e);
                    if attempt < self.config.max_retries {
                        log::warn!("数据库连接失败，将重试... ({})", attempt + 1);
                        std::thread::sleep(std::time::Duration::from_millis(100 * (1 << attempt)));
                    }
                }
            }
        }
        
        Err(last_error.unwrap_or_else(|| {
            crate::utils::ArxivError::Config("未知连接错误".to_string())
        }))
    }
    
    /// 尝试连接数据库
    fn try_connect(&self) -> Result<Connection> {
        let conn = Connection::open(&self.config.db_path)?;
        
        // 设置连接超时
        conn.busy_timeout(std::time::Duration::from_secs(self.config.timeout_seconds))?;
        
        // 启用WAL模式（如果配置了）
        if self.config.enable_wal {
            conn.execute("PRAGMA journal_mode=WAL", [])?;
        }
        
        // 启用外键约束
        conn.execute("PRAGMA foreign_keys=ON", [])?;
        
        // 设置性能优化参数
        conn.execute("PRAGMA synchronous=NORMAL", [])?;
        conn.execute("PRAGMA cache_size=10000", [])?;
        conn.execute("PRAGMA temp_store=memory", [])?;
        
        Ok(conn)
    }
    
    /// 检查数据库连接是否有效
    pub fn check_connection(&self, conn: &Connection) -> Result<bool> {
        match conn.execute("SELECT 1", []) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

/// 创建默认的数据库连接
pub fn create_default_connection() -> Result<Connection> {
    let manager = ConnectionManager::new(DatabaseConfig::default());
    manager.create_connection()
}

/// 创建指定路径的数据库连接
pub fn create_connection<P: AsRef<Path>>(db_path: P) -> Result<Connection> {
    let config = DatabaseConfig {
        db_path: db_path.as_ref().to_string_lossy().to_string(),
        ..DatabaseConfig::default()
    };
    let manager = ConnectionManager::new(config);
    manager.create_connection()
}
