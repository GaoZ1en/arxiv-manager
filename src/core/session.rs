// 会话状态管理模块
// 负责保存和恢复应用程序的标签页状态

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::core::models::{TabContent, ui::{Tab, TabGroup}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub tabs: Vec<SerializableTab>,
    pub active_tab: usize,
    pub next_tab_id: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableTab {
    pub id: usize,
    pub title: String,
    pub content: TabContent,
    pub closable: bool,
    pub pinned: bool,
    pub group: TabGroup,
}

impl From<&Tab> for SerializableTab {
    fn from(tab: &Tab) -> Self {
        Self {
            id: tab.id,
            title: tab.title.clone(),
            content: tab.content.clone(),
            closable: tab.closable,
            pinned: tab.pinned,
            group: tab.group.clone(),
        }
    }
}

impl From<SerializableTab> for Tab {
    fn from(serializable: SerializableTab) -> Self {
        Self {
            id: serializable.id,
            title: serializable.title,
            content: serializable.content,
            closable: serializable.closable,
            pinned: serializable.pinned,
            group: serializable.group,
        }
    }
}

pub struct SessionManager;

impl SessionManager {
    const SESSION_FILE: &'static str = "session.json";
    
    pub fn get_session_path() -> PathBuf {
        // 获取用户配置目录
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("arxiv_manager")
            .join(Self::SESSION_FILE)
    }
    
    pub fn save_session(tabs: &[Tab], active_tab: usize, next_tab_id: usize) -> Result<(), Box<dyn std::error::Error>> {
        let session_data = SessionData {
            tabs: tabs.iter().map(SerializableTab::from).collect(),
            active_tab,
            next_tab_id,
        };
        
        let session_path = Self::get_session_path();
        
        // 确保目录存在
        if let Some(parent) = session_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let json = serde_json::to_string_pretty(&session_data)?;
        fs::write(session_path, json)?;
        
        Ok(())
    }
    
    pub fn load_session() -> Result<SessionData, Box<dyn std::error::Error>> {
        let session_path = Self::get_session_path();
        
        if !session_path.exists() {
            return Err("Session file does not exist".into());
        }
        
        let json = fs::read_to_string(session_path)?;
        let session_data: SessionData = serde_json::from_str(&json)?;
        
        Ok(session_data)
    }
    
    pub fn session_exists() -> bool {
        Self::get_session_path().exists()
    }
    
    pub fn delete_session() -> Result<(), Box<dyn std::error::Error>> {
        let session_path = Self::get_session_path();
        if session_path.exists() {
            fs::remove_file(session_path)?;
        }
        Ok(())
    }
}
