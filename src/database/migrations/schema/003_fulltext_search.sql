-- 添加全文搜索支持
-- 版本: 3  
-- 描述: 创建FTS表以支持全文搜索功能

-- 创建FTS虚拟表
CREATE VIRTUAL TABLE IF NOT EXISTS papers_fts USING fts5(
    arxiv_id UNINDEXED,
    title,
    abstract,
    authors,
    content='papers',
    content_rowid='id'
);

-- 创建触发器以保持FTS表同步
CREATE TRIGGER IF NOT EXISTS papers_fts_insert AFTER INSERT ON papers BEGIN
    INSERT INTO papers_fts(rowid, arxiv_id, title, abstract, authors) 
    VALUES (new.id, new.arxiv_id, new.title, new.abstract, new.authors);
END;

CREATE TRIGGER IF NOT EXISTS papers_fts_delete AFTER DELETE ON papers BEGIN
    INSERT INTO papers_fts(papers_fts, rowid, arxiv_id, title, abstract, authors) 
    VALUES ('delete', old.id, old.arxiv_id, old.title, old.abstract, old.authors);
END;

CREATE TRIGGER IF NOT EXISTS papers_fts_update AFTER UPDATE ON papers BEGIN
    INSERT INTO papers_fts(papers_fts, rowid, arxiv_id, title, abstract, authors) 
    VALUES ('delete', old.id, old.arxiv_id, old.title, old.abstract, old.authors);
    INSERT INTO papers_fts(rowid, arxiv_id, title, abstract, authors) 
    VALUES (new.id, new.arxiv_id, new.title, new.abstract, new.authors);
END;
