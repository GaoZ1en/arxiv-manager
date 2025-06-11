-- 初始化论文表
-- 版本: 1
-- 描述: 创建基础的论文存储表

CREATE TABLE IF NOT EXISTS papers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    arxiv_id TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    authors TEXT NOT NULL, -- JSON数组
    abstract TEXT NOT NULL,
    categories TEXT NOT NULL, -- JSON数组
    published TEXT NOT NULL,
    updated TEXT NOT NULL,
    pdf_url TEXT NOT NULL,
    abstract_url TEXT NOT NULL,
    doi TEXT,
    journal_ref TEXT,
    comments TEXT,
    download_status INTEGER NOT NULL DEFAULT 0, -- 0=Pending, 1=Downloading, 2=Completed, 3=Failed
    local_path TEXT,
    tags TEXT DEFAULT '[]', -- JSON数组，用户标签
    read_progress REAL DEFAULT 0.0, -- 阅读进度 0.0-1.0
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- 创建索引以提高查询性能
CREATE INDEX IF NOT EXISTS idx_papers_arxiv_id ON papers(arxiv_id);
CREATE INDEX IF NOT EXISTS idx_papers_published ON papers(published);
CREATE INDEX IF NOT EXISTS idx_papers_created_at ON papers(created_at);
CREATE INDEX IF NOT EXISTS idx_papers_title ON papers(title);
