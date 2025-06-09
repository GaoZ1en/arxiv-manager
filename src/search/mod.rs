use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tantivy::{
    collector::TopDocs,
    directory::MmapDirectory,
    doc,
    query::QueryParser,
    schema::{Field, Schema, STORED, TEXT},
    Index, IndexReader, IndexWriter,
};
use uuid::Uuid;

pub mod search_engine;

pub use search_engine::SearchEngine;

/// Represents a searchable document in the index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchDocument {
    pub id: String,
    pub title: String,
    pub authors: String,
    pub abstract_text: String,
    pub categories: String,
    pub full_text: String,
    pub file_path: String,
}

/// Search query parameters
#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub query: String,
    pub limit: usize,
    pub categories: Vec<String>,
    pub authors: Vec<String>,
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            query: String::new(),
            limit: 50,
            categories: Vec::new(),
            authors: Vec::new(),
        }
    }
}

/// Search result item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub authors: String,
    pub abstract_text: String,
    pub categories: String,
    pub score: f32,
    pub file_path: String,
}

/// Schema fields for tantivy index
#[derive(Debug)]
pub struct IndexSchema {
    pub schema: Schema,
    pub id_field: Field,
    pub title_field: Field,
    pub authors_field: Field,
    pub abstract_field: Field,
    pub categories_field: Field,
    pub full_text_field: Field,
    pub file_path_field: Field,
}

impl IndexSchema {
    pub fn new() -> Self {
        let mut schema_builder = Schema::builder();
        
        let id_field = schema_builder.add_text_field("id", STORED);
        let title_field = schema_builder.add_text_field("title", TEXT | STORED);
        let authors_field = schema_builder.add_text_field("authors", TEXT | STORED);
        let abstract_field = schema_builder.add_text_field("abstract", TEXT | STORED);
        let categories_field = schema_builder.add_text_field("categories", TEXT | STORED);
        let full_text_field = schema_builder.add_text_field("full_text", TEXT);
        let file_path_field = schema_builder.add_text_field("file_path", STORED);
        
        let schema = schema_builder.build();
        
        Self {
            schema,
            id_field,
            title_field,
            authors_field,
            abstract_field,
            categories_field,
            full_text_field,
            file_path_field,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_search_engine_creation() {
        let temp_dir = TempDir::new().unwrap();
        let index_path = temp_dir.path().join("test_index");
        
        let engine = SearchEngine::new(&index_path).await.unwrap();
        assert!(index_path.exists());
    }

    #[tokio::test]
    async fn test_document_indexing() {
        let temp_dir = TempDir::new().unwrap();
        let index_path = temp_dir.path().join("test_index");
        
        let mut engine = SearchEngine::new(&index_path).await.unwrap();
        
        let doc = SearchDocument {
            id: Uuid::new_v4().to_string(),
            title: "Test Paper".to_string(),
            authors: "Test Author".to_string(),
            abstract_text: "This is a test abstract".to_string(),
            categories: "cs.AI".to_string(),
            full_text: "This is the full text of the paper".to_string(),
            file_path: "/path/to/paper.pdf".to_string(),
        };
        
        engine.index_document(doc).await.unwrap();
        engine.commit().await.unwrap();
        
        let query = SearchQuery {
            query: "test".to_string(),
            ..Default::default()
        };
        
        let results = engine.search(&query).await.unwrap();
        assert!(!results.is_empty());
    }
}
