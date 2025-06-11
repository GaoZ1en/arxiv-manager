// 数据库连接池实现

use super::ConnectionManager;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use crate::utils::Result;

/// 简单的数据库连接池
#[derive(Debug)]
pub struct ConnectionPool {
    manager: Arc<ConnectionManager>,
    pool: Arc<Mutex<VecDeque<Connection>>>,
    max_size: usize,
    current_size: Arc<Mutex<usize>>,
}

impl ConnectionPool {
    /// 创建新的连接池
    pub fn new(manager: ConnectionManager, max_size: usize) -> Self {
        Self {
            manager: Arc::new(manager),
            pool: Arc::new(Mutex::new(VecDeque::new())),
            max_size,
            current_size: Arc::new(Mutex::new(0)),
        }
    }
    
    /// 从池中获取连接
    pub fn get_connection(&self) -> Result<PooledConnection> {
        // 首先尝试从池中获取现有连接
        if let Ok(mut pool) = self.pool.lock() {
            if let Some(conn) = pool.pop_front() {
                // 检查连接是否仍然有效
                if self.manager.check_connection(&conn)? {
                    return Ok(PooledConnection::new(conn, self.pool.clone()));
                }
                // 连接无效，减少计数
                if let Ok(mut size) = self.current_size.lock() {
                    *size = size.saturating_sub(1);
                }
            }
        }
        
        // 池中没有可用连接，创建新连接
        let current_size = {
            let size_guard = self.current_size.lock().map_err(|_| {
                crate::utils::ArxivError::Database("获取连接池大小失败".to_string())
            })?;
            *size_guard
        };
        
        if current_size < self.max_size {
            let conn = self.manager.create_connection()?;
            
            // 增加连接计数
            if let Ok(mut size) = self.current_size.lock() {
                *size += 1;
            }
            
            Ok(PooledConnection::new(conn, self.pool.clone()))
        } else {
            Err(crate::utils::ArxivError::Database("连接池已满".to_string()))
        }
    }
    
    /// 获取连接池统计信息
    pub fn stats(&self) -> PoolStats {
        let pool_size = self.pool.lock().map(|p| p.len()).unwrap_or(0);
        let total_size = self.current_size.lock().map(|s| *s).unwrap_or(0);
        
        PoolStats {
            active_connections: total_size.saturating_sub(pool_size),
            idle_connections: pool_size,
            total_connections: total_size,
            max_connections: self.max_size,
        }
    }
}

/// 池化连接包装器
pub struct PooledConnection {
    connection: Option<Connection>,
    pool: Arc<Mutex<VecDeque<Connection>>>,
}

impl PooledConnection {
    fn new(connection: Connection, pool: Arc<Mutex<VecDeque<Connection>>>) -> Self {
        Self {
            connection: Some(connection),
            pool,
        }
    }
    
    /// 获取内部连接的引用
    pub fn as_ref(&self) -> &Connection {
        self.connection.as_ref().expect("连接已被移动")
    }
}

impl Drop for PooledConnection {
    fn drop(&mut self) {
        if let Some(conn) = self.connection.take() {
            // 将连接返回到池中
            if let Ok(mut pool) = self.pool.lock() {
                pool.push_back(conn);
            }
        }
    }
}

/// 连接池统计信息
#[derive(Debug, Clone)]
pub struct PoolStats {
    /// 正在使用的连接数
    pub active_connections: usize,
    /// 空闲连接数
    pub idle_connections: usize,
    /// 总连接数
    pub total_connections: usize,
    /// 最大连接数
    pub max_connections: usize,
}

impl std::fmt::Display for PoolStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "连接池状态: {}/{} 活跃, {} 空闲",
            self.active_connections, self.max_connections, self.idle_connections
        )
    }
}
