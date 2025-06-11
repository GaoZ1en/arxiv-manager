// 下载状态枚举

#[derive(Debug, Clone, PartialEq)]
pub enum DownloadStatus {
    Pending,
    Downloading,
    Completed,
    Failed(String),
}

impl DownloadStatus {
    /// 将下载状态转换为数据库中的整数值
    pub fn to_db_value(&self) -> i32 {
        match self {
            DownloadStatus::Pending => 0,
            DownloadStatus::Downloading => 1,
            DownloadStatus::Completed => 2,
            DownloadStatus::Failed(_) => 3,
        }
    }
    
    /// 从数据库中的整数值创建下载状态
    pub fn from_db_value(value: i32) -> Self {
        match value {
            0 => DownloadStatus::Pending,
            1 => DownloadStatus::Downloading,
            2 => DownloadStatus::Completed,
            3 => DownloadStatus::Failed("Unknown error".to_string()),
            _ => DownloadStatus::Pending,
        }
    }
}
