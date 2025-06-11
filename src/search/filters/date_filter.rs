// 日期过滤器实现

use crate::core::models::{ArxivPaper, DateRange};
use chrono::{DateTime, Utc, Duration, Datelike};

/// 根据日期范围过滤论文
pub fn filter_papers_by_date(papers: Vec<ArxivPaper>, date_range: &DateRange) -> Vec<ArxivPaper> {
    let cutoff_date = match date_range {
        DateRange::LastWeek => Utc::now() - Duration::weeks(1),
        DateRange::LastMonth => Utc::now() - Duration::weeks(4),
        DateRange::LastYear => Utc::now() - Duration::weeks(52),
        DateRange::Any => return papers, // 不过滤
        DateRange::Custom { from: _, to: _ } => {
            // 自定义日期范围的过滤逻辑可以在这里实现
            // 目前先返回所有论文
            return papers;
        }
    };
    
    papers.into_iter().filter(|paper| {
        if let Ok(published_date) = DateTime::parse_from_rfc3339(&paper.published) {
            published_date.with_timezone(&Utc) > cutoff_date
        } else {
            true // 如果无法解析日期，保留论文
        }
    }).collect()
}

/// 根据自定义日期范围过滤论文
#[allow(dead_code)]
pub fn filter_papers_by_custom_date_range(
    papers: Vec<ArxivPaper>, 
    start_date: &str, 
    end_date: &str
) -> Result<Vec<ArxivPaper>, String> {
    let start = DateTime::parse_from_rfc3339(start_date)
        .map_err(|_| "无效的开始日期格式")?
        .with_timezone(&Utc);
    
    let end = DateTime::parse_from_rfc3339(end_date)
        .map_err(|_| "无效的结束日期格式")?
        .with_timezone(&Utc);
    
    let filtered_papers = papers.into_iter().filter(|paper| {
        if let Ok(published_date) = DateTime::parse_from_rfc3339(&paper.published) {
            let pub_date = published_date.with_timezone(&Utc);
            pub_date >= start && pub_date <= end
        } else {
            false // 如果无法解析日期，排除论文
        }
    }).collect();
    
    Ok(filtered_papers)
}

/// 获取论文的发布年份
#[allow(dead_code)]
pub fn get_paper_year(paper: &ArxivPaper) -> Option<i32> {
    DateTime::parse_from_rfc3339(&paper.published)
        .ok()
        .map(|dt| dt.year())
}

/// 根据年份过滤论文
#[allow(dead_code)]
pub fn filter_papers_by_year(papers: Vec<ArxivPaper>, year: i32) -> Vec<ArxivPaper> {
    papers.into_iter().filter(|paper| {
        get_paper_year(paper).map_or(false, |paper_year| paper_year == year)
    }).collect()
}
