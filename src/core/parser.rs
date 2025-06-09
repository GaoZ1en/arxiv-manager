use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;

use super::models::*;

/// arXiv XML 解析器
pub struct ArxivParser;

impl ArxivParser {
    /// 解析搜索结果 XML
    pub fn parse_search_results(xml_content: &str) -> Result<ArxivSearchResult> {
        let mut reader = Reader::from_str(xml_content);
        reader.trim_text(true);

        let mut buf = Vec::new();
        let mut entries = Vec::new();
        let mut total_results = 0;
        let mut start_index = 0;
        let mut items_per_page = 0;

        let mut in_entry = false;
        let mut current_entry: Option<EntryBuilder> = None;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name().as_ref() {
                        b"entry" => {
                            in_entry = true;
                            current_entry = Some(EntryBuilder::new());
                        }
                        b"totalResults" => {
                            if let Ok(text) = reader.read_text(e.name()) {
                                total_results = text.parse().unwrap_or(0);
                            }
                        }
                        b"startIndex" => {
                            if let Ok(text) = reader.read_text(e.name()) {
                                start_index = text.parse().unwrap_or(0);
                            }
                        }
                        b"itemsPerPage" => {
                            if let Ok(text) = reader.read_text(e.name()) {
                                items_per_page = text.parse().unwrap_or(0);
                            }
                        }
                        _ => {}
                    }

                    if in_entry {
                        if let Some(ref mut entry) = current_entry {
                            Self::parse_entry_element(&mut reader, e, entry)?;
                        }
                    }
                }
                Ok(Event::End(ref e)) => {
                    if e.name().as_ref() == b"entry" {
                        in_entry = false;
                        if let Some(entry_builder) = current_entry.take() {
                            if let Ok(entry) = entry_builder.build() {
                                entries.push(entry);
                            }
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(anyhow!("XML parsing error: {}", e)),
                _ => {}
            }

            buf.clear();
        }

        Ok(ArxivSearchResult {
            total_results,
            start_index,
            items_per_page,
            entries,
        })
    }

    /// 解析条目元素
    fn parse_entry_element(
        reader: &mut Reader<&[u8]>,
        start_event: &quick_xml::events::BytesStart,
        entry: &mut EntryBuilder,
    ) -> Result<()> {
        let mut buf = Vec::new();

        match start_event.name().as_ref() {
            b"id" => {
                if let Ok(text) = reader.read_text(start_event.name()) {
                    entry.id = Some(text.to_string());
                }
            }
            b"title" => {
                if let Ok(text) = reader.read_text(start_event.name()) {
                    entry.title = Some(text.trim().to_string());
                }
            }
            b"summary" => {
                if let Ok(text) = reader.read_text(start_event.name()) {
                    entry.summary = Some(text.trim().to_string());
                }
            }
            b"published" => {
                if let Ok(text) = reader.read_text(start_event.name()) {
                    if let Ok(dt) = DateTime::parse_from_rfc3339(&text) {
                        entry.published = Some(dt.with_timezone(&Utc));
                    }
                }
            }
            b"updated" => {
                if let Ok(text) = reader.read_text(start_event.name()) {
                    if let Ok(dt) = DateTime::parse_from_rfc3339(&text) {
                        entry.updated = Some(dt.with_timezone(&Utc));
                    }
                }
            }
            b"author" => {
                let author = Self::parse_author(reader, start_event)?;
                entry.authors.push(author);
            }
            b"category" => {
                let category = Self::parse_category(start_event)?;
                if entry.primary_category.is_none() {
                    entry.primary_category = Some(category.clone());
                }
                entry.categories.push(category);
            }
            b"link" => {
                let link_info = Self::parse_link(start_event)?;
                match link_info.rel.as_str() {
                    "alternate" => entry.abs_url = Some(link_info.href),
                    "related" if link_info.title.as_deref() == Some("pdf") => {
                        entry.pdf_url = Some(link_info.href);
                    }
                    "related" if link_info.title.as_deref() == Some("doi") => {
                        entry.doi = Some(link_info.href.trim_start_matches("http://dx.doi.org/").to_string());
                    }
                    _ => {}
                }
            }
            b"arxiv:comment" => {
                if let Ok(text) = reader.read_text(start_event.name()) {
                    entry.comment = Some(text.to_string());
                }
            }
            b"arxiv:journal_ref" => {
                if let Ok(text) = reader.read_text(start_event.name()) {
                    entry.journal_ref = Some(text.to_string());
                }
            }
            _ => {
                // 跳过未知元素
                reader.read_to_end_into(start_event.name(), &mut buf)?;
            }
        }

        Ok(())
    }

    /// 解析作者信息
    fn parse_author(
        reader: &mut Reader<&[u8]>,
        _start_event: &quick_xml::events::BytesStart,
    ) -> Result<ArxivAuthor> {
        let mut buf = Vec::new();
        let mut name = String::new();
        let mut affiliation = None;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().as_ref() {
                    b"name" => {
                        if let Ok(text) = reader.read_text(e.name()) {
                            name = text.to_string();
                        }
                    }
                    b"arxiv:affiliation" => {
                        if let Ok(text) = reader.read_text(e.name()) {
                            affiliation = Some(text.to_string());
                        }
                    }
                    _ => {
                        reader.read_to_end_into(e.name(), &mut Vec::new())?;
                    }
                },
                Ok(Event::End(ref e)) if e.name().as_ref() == b"author" => break,
                Ok(Event::Eof) => break,
                Err(e) => return Err(anyhow!("XML parsing error: {}", e)),
                _ => {}
            }
            buf.clear();
        }

        Ok(ArxivAuthor { name, affiliation })
    }

    /// 解析类别信息
    fn parse_category(start_event: &quick_xml::events::BytesStart) -> Result<ArxivCategory> {
        let mut term = String::new();
        let mut scheme = String::new();
        let mut label = None;

        for attr in start_event.attributes() {
            let attr = attr?;
            match attr.key.as_ref() {
                b"term" => term = String::from_utf8_lossy(&attr.value).to_string(),
                b"scheme" => scheme = String::from_utf8_lossy(&attr.value).to_string(),
                b"label" => label = Some(String::from_utf8_lossy(&attr.value).to_string()),
                _ => {}
            }
        }

        Ok(ArxivCategory { term, scheme, label })
    }

    /// 解析链接信息
    fn parse_link(start_event: &quick_xml::events::BytesStart) -> Result<LinkInfo> {
        let mut href = String::new();
        let mut rel = String::new();
        let mut link_type = None;
        let mut title = None;

        for attr in start_event.attributes() {
            let attr = attr?;
            match attr.key.as_ref() {
                b"href" => href = String::from_utf8_lossy(&attr.value).to_string(),
                b"rel" => rel = String::from_utf8_lossy(&attr.value).to_string(),
                b"type" => link_type = Some(String::from_utf8_lossy(&attr.value).to_string()),
                b"title" => title = Some(String::from_utf8_lossy(&attr.value).to_string()),
                _ => {}
            }
        }

        Ok(LinkInfo {
            href,
            rel,
            link_type,
            title,
        })
    }
}

/// 条目构建器
#[derive(Debug, Default)]
struct EntryBuilder {
    pub id: Option<String>,
    pub title: Option<String>,
    pub authors: Vec<ArxivAuthor>,
    pub summary: Option<String>,
    pub categories: Vec<ArxivCategory>,
    pub primary_category: Option<ArxivCategory>,
    pub published: Option<DateTime<Utc>>,
    pub updated: Option<DateTime<Utc>>,
    pub doi: Option<String>,
    pub journal_ref: Option<String>,
    pub pdf_url: Option<String>,
    pub abs_url: Option<String>,
    pub comment: Option<String>,
}

impl EntryBuilder {
    fn new() -> Self {
        Self::default()
    }

    fn build(self) -> Result<ArxivPaper> {
        let id = self.id.ok_or_else(|| anyhow!("Missing id"))?;
        let published = self.published.ok_or_else(|| anyhow!("Missing published date"))?;
        
        Ok(ArxivPaper {
            id: id.clone(),
            title: self.title.ok_or_else(|| anyhow!("Missing title"))?,
            authors: self.authors,
            summary: self.summary.ok_or_else(|| anyhow!("Missing summary"))?,
            categories: self.categories.clone(),
            primary_category: self.primary_category.unwrap_or_else(|| {
                self.categories.first().cloned().unwrap_or_else(|| ArxivCategory::new("unknown".to_string()))
            }),
            published,
            updated: self.updated.unwrap_or(published),
            doi: self.doi,
            journal_ref: self.journal_ref,
            pdf_url: self.pdf_url.unwrap_or_else(|| {
                let arxiv_id = id
                    .trim_start_matches("http://arxiv.org/abs/");
                format!("https://arxiv.org/pdf/{}.pdf", arxiv_id)
            }),
            abs_url: self.abs_url.unwrap_or_else(|| {
                id.clone()
            }),
            comment: self.comment,
        })
    }
}

/// 链接信息
#[derive(Debug)]
struct LinkInfo {
    pub href: String,
    pub rel: String,
    pub link_type: Option<String>,
    pub title: Option<String>,
}
