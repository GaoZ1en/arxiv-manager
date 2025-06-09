use super::{IndexSchema, SearchDocument, SearchQuery, SearchResult};
use anyhow::{Context, Result};
use std::path::Path;
use std::sync::Arc;
use tantivy::{
    collector::TopDocs,
    directory::MmapDirectory,
    doc,
    query::QueryParser,
    Index, IndexReader, IndexWriter, TantivyError,
};
use tokio::sync::RwLock;

/// Main search engine implementation using tantivy
pub struct SearchEngine {
    index: Index,
    schema: IndexSchema,
    writer: Arc<RwLock<IndexWriter>>,
    reader: IndexReader,
}

impl SearchEngine {
    /// Create a new search engine with the given index directory
    pub async fn new<P: AsRef<Path>>(index_path: P) -> Result<Self> {
        let schema = IndexSchema::new();
        
        // Create directory if it doesn't exist
        std::fs::create_dir_all(&index_path)
            .context("Failed to create index directory")?;
        
        // Open or create the index
        let index = if index_path.as_ref().join("meta.json").exists() {
            // Open existing index
            let directory = MmapDirectory::open(&index_path)
                .context("Failed to open index directory")?;
            Index::open(directory)
                .context("Failed to open existing index")?
        } else {
            // Create new index
            let directory = MmapDirectory::open(&index_path)
                .context("Failed to open index directory")?;
            Index::create(directory, schema.schema.clone(), tantivy::IndexSettings::default())
                .context("Failed to create new index")?
        };
        
        // Create writer and reader
        let writer = index.writer(50_000_000)
            .context("Failed to create index writer")?;
        let reader = index.reader()
            .context("Failed to create index reader")?;
        
        Ok(Self {
            index,
            schema,
            writer: Arc::new(RwLock::new(writer)),
            reader,
        })
    }
    
    /// Index a document for searching
    pub async fn index_document(&mut self, document: SearchDocument) -> Result<()> {
        let doc = doc!(
            self.schema.id_field => document.id,
            self.schema.title_field => document.title,
            self.schema.authors_field => document.authors,
            self.schema.abstract_field => document.abstract_text,
            self.schema.categories_field => document.categories,
            self.schema.full_text_field => document.full_text,
            self.schema.file_path_field => document.file_path
        );
        
        let mut writer = self.writer.write().await;
        writer.add_document(doc)
            .context("Failed to add document to index")?;
        
        Ok(())
    }
    
    /// Index multiple documents in batch
    pub async fn index_documents(&mut self, documents: Vec<SearchDocument>) -> Result<()> {
        let mut writer = self.writer.write().await;
        
        for document in documents {
            let doc = doc!(
                self.schema.id_field => document.id,
                self.schema.title_field => document.title,
                self.schema.authors_field => document.authors,
                self.schema.abstract_field => document.abstract_text,
                self.schema.categories_field => document.categories,
                self.schema.full_text_field => document.full_text,
                self.schema.file_path_field => document.file_path
            );
            
            writer.add_document(doc)
                .context("Failed to add document to index")?;
        }
        
        Ok(())
    }
    
    /// Remove a document from the index by ID
    pub async fn remove_document(&mut self, id: &str) -> Result<()> {
        let term = tantivy::Term::from_field_text(self.schema.id_field, id);
        
        let mut writer = self.writer.write().await;
        writer.delete_term(term);
        
        Ok(())
    }
    
    /// Search for documents matching the query
    pub async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        let searcher = self.reader.searcher();
        
        // Build query parser for multiple fields
        let mut query_parser = QueryParser::for_index(
            &self.index,
            vec![
                self.schema.title_field,
                self.schema.authors_field,
                self.schema.abstract_field,
                self.schema.full_text_field,
                self.schema.categories_field,
            ],
        );
        
        // Parse the query
        let parsed_query = query_parser
            .parse_query(&query.query)
            .context("Failed to parse search query")?;
        
        // Execute search
        let top_docs = searcher
            .search(&parsed_query, &TopDocs::with_limit(query.limit))
            .context("Failed to execute search")?;
        
        // Convert results
        let mut results = Vec::new();
        for (score, doc_address) in top_docs {
            let retrieved_doc = searcher
                .doc(doc_address)
                .context("Failed to retrieve document")?;
            
            let id = retrieved_doc
                .get_first(self.schema.id_field)
                .and_then(|f| f.as_text())
                .unwrap_or("")
                .to_string();
            
            let title = retrieved_doc
                .get_first(self.schema.title_field)
                .and_then(|f| f.as_text())
                .unwrap_or("")
                .to_string();
            
            let authors = retrieved_doc
                .get_first(self.schema.authors_field)
                .and_then(|f| f.as_text())
                .unwrap_or("")
                .to_string();
            
            let abstract_text = retrieved_doc
                .get_first(self.schema.abstract_field)
                .and_then(|f| f.as_text())
                .unwrap_or("")
                .to_string();
            
            let categories = retrieved_doc
                .get_first(self.schema.categories_field)
                .and_then(|f| f.as_text())
                .unwrap_or("")
                .to_string();
            
            let file_path = retrieved_doc
                .get_first(self.schema.file_path_field)
                .and_then(|f| f.as_text())
                .unwrap_or("")
                .to_string();
            
            // Apply filters if specified
            if !query.categories.is_empty() {
                let doc_categories: Vec<&str> = categories.split(',').map(|s| s.trim()).collect();
                if !query.categories.iter().any(|cat| doc_categories.contains(&cat.as_str())) {
                    continue;
                }
            }
            
            if !query.authors.is_empty() {
                let doc_authors = authors.to_lowercase();
                if !query.authors.iter().any(|author| doc_authors.contains(&author.to_lowercase())) {
                    continue;
                }
            }
            
            results.push(SearchResult {
                id,
                title,
                authors,
                abstract_text,
                categories,
                score,
                file_path,
            });
        }
        
        Ok(results)
    }
    
    /// Commit all pending changes to the index
    pub async fn commit(&mut self) -> Result<()> {
        let mut writer = self.writer.write().await;
        writer.commit()
            .context("Failed to commit index changes")?;
        
        // Reload the reader to see new documents
        self.reader.reload()
            .context("Failed to reload index reader")?;
        
        Ok(())
    }
    
    /// Get the total number of documents in the index
    pub fn num_docs(&self) -> u64 {
        let searcher = self.reader.searcher();
        searcher.num_docs()
    }
    
    /// Clear all documents from the index
    pub async fn clear(&mut self) -> Result<()> {
        let mut writer = self.writer.write().await;
        writer.delete_all_documents()
            .context("Failed to clear all documents")?;
        writer.commit()
            .context("Failed to commit after clearing")?;
        
        self.reader.reload()
            .context("Failed to reload reader after clearing")?;
        
        Ok(())
    }
    
    /// Optimize the index for better search performance
    pub async fn optimize(&mut self) -> Result<()> {
        let mut writer = self.writer.write().await;
        // Just commit to merge segments, merge policy configuration is typically done at index creation
        writer.commit()
            .context("Failed to optimize index")?;
        
        Ok(())
    }
    
    /// Get suggestions for autocomplete based on partial query
    pub async fn suggest(&self, partial_query: &str, field: &str, limit: usize) -> Result<Vec<String>> {
        // This is a simplified implementation
        // In a real-world scenario, you might want to use a more sophisticated approach
        let query = SearchQuery {
            query: format!("{}*", partial_query),
            limit,
            ..Default::default()
        };
        
        let results = self.search(&query).await?;
        let mut suggestions = Vec::new();
        
        for result in results {
            match field {
                "title" => suggestions.push(result.title),
                "authors" => suggestions.push(result.authors),
                "categories" => suggestions.push(result.categories),
                _ => suggestions.push(result.title),
            }
        }
        
        // Remove duplicates and limit
        suggestions.sort();
        suggestions.dedup();
        suggestions.truncate(limit);
        
        Ok(suggestions)
    }
}
