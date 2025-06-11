// 数据库迁移管理模块

use rusqlite::Connection;
use crate::utils::Result;

/// 数据库版本信息
#[derive(Debug, Clone)]
pub struct MigrationInfo {
    pub version: u32,
    pub description: String,
    pub sql: &'static str,
}

/// 数据库迁移管理器
#[derive(Debug)]
pub struct MigrationManager {
    migrations: Vec<MigrationInfo>,
}

impl MigrationManager {
    /// 创建新的迁移管理器
    pub fn new() -> Self {
        Self {
            migrations: get_all_migrations(),
        }
    }
    
    /// 执行数据库迁移
    pub fn migrate(&self, conn: &Connection) -> Result<()> {
        // 确保迁移表存在
        self.ensure_migration_table(conn)?;
        
        // 获取当前数据库版本
        let current_version = self.get_current_version(conn)?;
        
        // 执行需要的迁移
        for migration in &self.migrations {
            if migration.version > current_version {
                log::info!("执行迁移 v{}: {}", migration.version, migration.description);
                self.execute_migration(conn, migration)?;
            }
        }
        
        Ok(())
    }
    
    /// 确保迁移表存在
    fn ensure_migration_table(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS schema_migrations (
                version INTEGER PRIMARY KEY,
                applied_at TEXT NOT NULL
            )
            "#,
            [],
        )?;
        Ok(())
    }
    
    /// 获取当前数据库版本
    fn get_current_version(&self, conn: &Connection) -> Result<u32> {
        let version = conn.query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
            [],
            |row| row.get::<_, u32>(0),
        )?;
        Ok(version)
    }
    
    /// 执行单个迁移
    fn execute_migration(&self, conn: &Connection, migration: &MigrationInfo) -> Result<()> {
        let tx = conn.unchecked_transaction()?;
        
        // 执行迁移SQL
        tx.execute_batch(migration.sql)?;
        
        // 记录迁移版本
        let now = chrono::Utc::now().to_rfc3339();
        tx.execute(
            "INSERT INTO schema_migrations (version, applied_at) VALUES (?1, ?2)",
            [migration.version.to_string(), now],
        )?;
        
        tx.commit()?;
        Ok(())
    }
}

/// 获取所有迁移
fn get_all_migrations() -> Vec<MigrationInfo> {
    vec![
        MigrationInfo {
            version: 1,
            description: "创建论文表".to_string(),
            sql: include_str!("schema/001_initial_papers.sql"),
        },
        MigrationInfo {
            version: 2,
            description: "添加下载状态索引".to_string(),
            sql: include_str!("schema/002_download_status_index.sql"),
        },
        MigrationInfo {
            version: 3,
            description: "添加全文搜索支持".to_string(),
            sql: include_str!("schema/003_fulltext_search.sql"),
        },
    ]
}
