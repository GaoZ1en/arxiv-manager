-- 添加下载状态索引
-- 版本: 2
-- 描述: 为下载状态添加索引以提高查询性能

CREATE INDEX IF NOT EXISTS idx_papers_download_status ON papers(download_status);
CREATE INDEX IF NOT EXISTS idx_papers_local_path ON papers(local_path);

-- 添加分类索引
CREATE INDEX IF NOT EXISTS idx_papers_categories ON papers(categories);
