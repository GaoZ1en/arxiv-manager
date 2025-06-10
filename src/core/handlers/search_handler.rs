// 搜索消息处理器
// 处理所有与搜索相关的消息

use iced::Task;

use crate::core::{ArxivManager, Message, ArxivPaper, SearchField, DateRange, SortBy, SortOrder};
use crate::search::services::search_arxiv_papers_advanced;

pub trait SearchHandler {
    fn handle_search_query_changed(&mut self, query: String) -> Task<Message>;
    fn handle_search_submitted(&mut self) -> Task<Message>;
    fn handle_search_completed(&mut self, result: Result<Vec<ArxivPaper>, String>) -> Task<Message>;
    fn handle_advanced_search_toggled(&mut self) -> Task<Message>;
    fn handle_search_field_changed(&mut self, field: SearchField) -> Task<Message>;
    fn handle_category_toggled(&mut self, category: String) -> Task<Message>;
    fn handle_date_range_changed(&mut self, range: DateRange) -> Task<Message>;
    fn handle_sort_by_changed(&mut self, sort_by: SortBy) -> Task<Message>;
    fn handle_sort_order_changed(&mut self, order: SortOrder) -> Task<Message>;
    fn handle_max_results_changed(&mut self, value: String) -> Task<Message>;
    fn handle_author_added(&mut self, author: String) -> Task<Message>;
    fn handle_author_removed(&mut self, index: usize) -> Task<Message>;
}

impl SearchHandler for ArxivManager {
    fn handle_search_query_changed(&mut self, query: String) -> Task<Message> {
        self.search_query = query.clone();
        self.search_config.query = query;
        Task::none()
    }

    fn handle_search_submitted(&mut self) -> Task<Message> {
        if !self.search_config.query.trim().is_empty() {
            self.is_searching = true;
            self.search_error = None;
            
            let config = self.search_config.clone();
            Task::perform(
                search_arxiv_papers_advanced(config),
                Message::SearchCompleted
            )
        } else {
            Task::none()
        }
    }

    fn handle_search_completed(&mut self, result: Result<Vec<ArxivPaper>, String>) -> Task<Message> {
        self.is_searching = false;
        match result {
            Ok(papers) => {
                self.search_results = papers;
                self.search_error = None;
            }
            Err(error) => {
                self.search_error = Some(error);
                self.search_results.clear();
            }
        }
        Task::none()
    }

    fn handle_advanced_search_toggled(&mut self) -> Task<Message> {
        self.advanced_search_visible = !self.advanced_search_visible;
        Task::none()
    }

    fn handle_search_field_changed(&mut self, field: SearchField) -> Task<Message> {
        self.search_config.search_in = field;
        Task::none()
    }

    fn handle_category_toggled(&mut self, category: String) -> Task<Message> {
        if let Some(pos) = self.search_config.categories.iter().position(|x| x == &category) {
            self.search_config.categories.remove(pos);
        } else {
            self.search_config.categories.push(category);
        }
        Task::none()
    }

    fn handle_date_range_changed(&mut self, range: DateRange) -> Task<Message> {
        self.search_config.date_range = range;
        Task::none()
    }

    fn handle_sort_by_changed(&mut self, sort_by: SortBy) -> Task<Message> {
        self.search_config.sort_by = sort_by;
        Task::none()
    }

    fn handle_sort_order_changed(&mut self, order: SortOrder) -> Task<Message> {
        self.search_config.sort_order = order;
        Task::none()
    }

    fn handle_max_results_changed(&mut self, value: String) -> Task<Message> {
        if let Ok(num) = value.parse::<u32>() {
            self.search_config.max_results = num.min(100).max(1);
        }
        Task::none()
    }

    fn handle_author_added(&mut self, author: String) -> Task<Message> {
        if !author.trim().is_empty() && !self.search_config.authors.contains(&author) {
            self.search_config.authors.push(author);
        }
        Task::none()
    }

    fn handle_author_removed(&mut self, index: usize) -> Task<Message> {
        if index < self.search_config.authors.len() {
            self.search_config.authors.remove(index);
        }
        Task::none()
    }
}
