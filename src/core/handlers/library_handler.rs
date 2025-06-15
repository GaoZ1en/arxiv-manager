// Library视图处理器 - 处理排序、分组和显示模式

use iced::Task;
use chrono::Datelike;

use crate::core::ArxivManager;
use crate::core::messages::Message;
use crate::core::models::{ArxivPaper, LibrarySortBy, LibraryGroupBy, LibraryViewMode};

pub trait LibraryHandler {
    fn handle_library_sort_changed(&mut self, sort_by: LibrarySortBy) -> Task<Message>;
    fn handle_library_group_changed(&mut self, group_by: LibraryGroupBy) -> Task<Message>;
    fn handle_library_view_mode_changed(&mut self, view_mode: LibraryViewMode) -> Task<Message>;
    fn apply_library_filters_and_sorting(&mut self);
    fn sort_papers(&self, papers: &mut Vec<ArxivPaper>);
    fn group_papers(&self, papers: &[ArxivPaper]) -> Vec<(String, Vec<ArxivPaper>)>;
    
    // 新增的论文管理方法
    fn handle_toggle_paper_favorite(&mut self, paper_id: &str) -> Task<Message>;
    fn handle_set_paper_rating(&mut self, paper_id: String, rating: Option<u8>) -> Task<Message>;
    fn handle_set_paper_read_status(&mut self, paper_id: String, status: crate::core::models::ReadingStatus) -> Task<Message>;
    fn handle_add_paper_tag(&mut self, paper_id: String, tag: String) -> Task<Message>;
    fn handle_remove_paper_tag(&mut self, paper_id: String, tag: String) -> Task<Message>;
    fn handle_set_paper_notes(&mut self, paper_id: String, notes: Option<String>) -> Task<Message>;
}

impl LibraryHandler for ArxivManager {
    fn handle_library_sort_changed(&mut self, sort_by: LibrarySortBy) -> Task<Message> {
        self.library_sort_by = sort_by;
        self.apply_library_filters_and_sorting();
        Task::none()
    }

    fn handle_library_group_changed(&mut self, group_by: LibraryGroupBy) -> Task<Message> {
        self.library_group_by = group_by;
        self.apply_library_filters_and_sorting();
        Task::none()
    }

    fn handle_library_view_mode_changed(&mut self, view_mode: LibraryViewMode) -> Task<Message> {
        self.library_view_mode = view_mode;
        Task::none()
    }

    fn apply_library_filters_and_sorting(&mut self) {
        // 1. 获取要显示的论文列表（根据选中的集合）
        let mut papers_to_show = if let Some(collection_id) = self.selected_collection_id {
            match collection_id {
                -1 => self.saved_papers.clone(), // All Papers
                -2 => {
                    // Recently Added: 最近添加的论文（最近10个）
                    let mut recent_papers = self.saved_papers.clone();
                    // 按添加时间排序，有添加时间的在前
                    recent_papers.sort_by(|a, b| {
                        match (a.added_at, b.added_at) {
                            (Some(a_time), Some(b_time)) => b_time.cmp(&a_time),
                            (Some(_), None) => std::cmp::Ordering::Less,
                            (None, Some(_)) => std::cmp::Ordering::Greater,
                            (None, None) => std::cmp::Ordering::Equal,
                        }
                    });
                    recent_papers.truncate(10);
                    recent_papers
                },
                -3 => {
                    // Favorites: 收藏的论文
                    self.saved_papers.iter()
                        .filter(|paper| paper.is_favorite)
                        .cloned()
                        .collect()
                },
                -4 => {
                    // Uncategorized: 未分类的论文
                    self.saved_papers.iter()
                        .filter(|paper| paper.is_uncategorized())
                        .cloned()
                        .collect()
                },
                _ => {
                    // 用户自定义集合: 根据collection_ids过滤
                    self.saved_papers.iter()
                        .filter(|paper| paper.belongs_to_collection(collection_id))
                        .cloned()
                        .collect()
                }
            }
        } else {
            self.saved_papers.clone()
        };

        // 2. 应用排序
        self.sort_papers(&mut papers_to_show);

        // 3. 更新filtered_papers
        self.filtered_papers = papers_to_show;
    }

    fn sort_papers(&self, papers: &mut Vec<ArxivPaper>) {
        match self.library_sort_by {
            LibrarySortBy::Title => {
                papers.sort_by(|a, b| a.title.cmp(&b.title));
            },
            LibrarySortBy::Author => {
                papers.sort_by(|a, b| {
                    let a_author = a.authors.first().map(|s| s.as_str()).unwrap_or("");
                    let b_author = b.authors.first().map(|s| s.as_str()).unwrap_or("");
                    a_author.cmp(b_author)
                });
            },
            LibrarySortBy::PublishDate => {
                papers.sort_by(|a, b| b.published.cmp(&a.published)); // 最新的在前
            },
            LibrarySortBy::AddedDate => {
                // 按添加时间排序，最新添加的在前
                papers.sort_by(|a, b| {
                    match (a.added_at, b.added_at) {
                        (Some(a_time), Some(b_time)) => b_time.cmp(&a_time),
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        (None, None) => b.published.cmp(&a.published), // 回退到发布时间
                    }
                });
            },
            LibrarySortBy::Category => {
                papers.sort_by(|a, b| {
                    let a_category = a.categories.first().map(|s| s.as_str()).unwrap_or("");
                    let b_category = b.categories.first().map(|s| s.as_str()).unwrap_or("");
                    a_category.cmp(b_category)
                });
            },
            LibrarySortBy::Relevance => {
                // TODO: 实现相关性排序算法
                // 暂时按发布时间排序
                papers.sort_by(|a, b| b.published.cmp(&a.published));
            }
        }
    }

    fn group_papers(&self, papers: &[ArxivPaper]) -> Vec<(String, Vec<ArxivPaper>)> {
        use std::collections::HashMap;

        match self.library_group_by {
            LibraryGroupBy::None => {
                vec![("All Papers".to_string(), papers.to_vec())]
            },
            LibraryGroupBy::Author => {
                let mut groups: HashMap<String, Vec<ArxivPaper>> = HashMap::new();
                
                for paper in papers {
                    let author = paper.authors.first()
                        .cloned()
                        .unwrap_or_else(|| "Unknown Author".to_string());
                    groups.entry(author).or_insert_with(Vec::new).push(paper.clone());
                }

                let mut result: Vec<(String, Vec<ArxivPaper>)> = groups.into_iter().collect();
                result.sort_by(|a, b| a.0.cmp(&b.0));
                result
            },
            LibraryGroupBy::Category => {
                let mut groups: HashMap<String, Vec<ArxivPaper>> = HashMap::new();
                
                for paper in papers {
                    let category = paper.categories.first()
                        .cloned()
                        .unwrap_or_else(|| "Unknown Category".to_string());
                    groups.entry(category).or_insert_with(Vec::new).push(paper.clone());
                }

                let mut result: Vec<(String, Vec<ArxivPaper>)> = groups.into_iter().collect();
                result.sort_by(|a, b| a.0.cmp(&b.0));
                result
            },
            LibraryGroupBy::PublishYear => {
                let mut groups: HashMap<String, Vec<ArxivPaper>> = HashMap::new();
                
                for paper in papers {
                    // 尝试从published字符串中提取年份
                    let year = paper.published.chars().take(4).collect::<String>();
                    let year = if year.len() == 4 && year.chars().all(char::is_numeric) {
                        year
                    } else {
                        "Unknown".to_string()
                    };
                    groups.entry(year).or_insert_with(Vec::new).push(paper.clone());
                }

                let mut result: Vec<(String, Vec<ArxivPaper>)> = groups.into_iter().collect();
                result.sort_by(|a, b| b.0.cmp(&a.0)); // 年份倒序
                result
            },
            LibraryGroupBy::AddedDate => {
                // 按添加日期分组（年月）
                let mut groups: HashMap<String, Vec<ArxivPaper>> = HashMap::new();
                
                for paper in papers {
                    let date_group = if let Some(added_at) = paper.added_at {
                        format!("{}-{:02}", added_at.year(), added_at.month())
                    } else {
                        "Not Added".to_string()
                    };
                    groups.entry(date_group).or_insert_with(Vec::new).push(paper.clone());
                }

                let mut result: Vec<(String, Vec<ArxivPaper>)> = groups.into_iter().collect();
                result.sort_by(|a, b| b.0.cmp(&a.0)); // 最新的在前
                result
            },
            LibraryGroupBy::Tag => {
                // 按标签分组
                let mut groups: HashMap<String, Vec<ArxivPaper>> = HashMap::new();
                
                for paper in papers {
                    if paper.tags.is_empty() {
                        groups.entry("Untagged".to_string())
                            .or_insert_with(Vec::new)
                            .push(paper.clone());
                    } else {
                        for tag in &paper.tags {
                            groups.entry(tag.clone())
                                .or_insert_with(Vec::new)
                                .push(paper.clone());
                        }
                    }
                }

                let mut result: Vec<(String, Vec<ArxivPaper>)> = groups.into_iter().collect();
                result.sort_by(|a, b| a.0.cmp(&b.0));
                result
            }
        }
    }

    // 新增的论文管理方法
    fn handle_toggle_paper_favorite(&mut self, paper_id: &str) -> Task<Message> {
        if let Some(paper) = self.saved_papers.iter_mut().find(|p| p.id == paper_id) {
            paper.toggle_favorite();
        }
        self.apply_library_filters_and_sorting();
        Task::none()
    }

    fn handle_set_paper_rating(&mut self, paper_id: String, rating: Option<u8>) -> Task<Message> {
        if let Some(paper) = self.saved_papers.iter_mut().find(|p| p.id == paper_id) {
            paper.rating = rating;
        }
        Task::none()
    }

    fn handle_set_paper_read_status(&mut self, paper_id: String, status: crate::core::models::ReadingStatus) -> Task<Message> {
        if let Some(paper) = self.saved_papers.iter_mut().find(|p| p.id == paper_id) {
            paper.read_status = status;
        }
        Task::none()
    }

    fn handle_add_paper_tag(&mut self, paper_id: String, tag: String) -> Task<Message> {
        if let Some(paper) = self.saved_papers.iter_mut().find(|p| p.id == paper_id) {
            if !paper.tags.contains(&tag) {
                paper.tags.push(tag);
            }
        }
        self.apply_library_filters_and_sorting();
        Task::none()
    }

    fn handle_remove_paper_tag(&mut self, paper_id: String, tag: String) -> Task<Message> {
        if let Some(paper) = self.saved_papers.iter_mut().find(|p| p.id == paper_id) {
            paper.tags.retain(|t| t != &tag);
        }
        self.apply_library_filters_and_sorting();
        Task::none()
    }

    fn handle_set_paper_notes(&mut self, paper_id: String, notes: Option<String>) -> Task<Message> {
        if let Some(paper) = self.saved_papers.iter_mut().find(|p| p.id == paper_id) {
            paper.notes = notes;
        }
        Task::none()
    }
}
