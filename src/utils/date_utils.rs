use super::{UtilError, UtilResult};
use chrono::{DateTime, NaiveDateTime, Utc, Local, TimeZone, Datelike, ParseError};
use serde::{Deserialize, Serialize};

/// Date and time utility functions
pub struct DateUtils;

impl DateUtils {
    /// Parse arXiv date format (YYYY-MM-DD or YYYY-MM-DDTHH:MM:SSZ)
    pub fn parse_arxiv_date(date_str: &str) -> UtilResult<DateTime<Utc>> {
        // Try different date formats used by arXiv
        let formats = vec![
            "%Y-%m-%d",
            "%Y-%m-%dT%H:%M:%SZ",
            "%Y-%m-%dT%H:%M:%S%.fZ",
            "%Y-%m-%d %H:%M:%S",
            "%a, %d %b %Y %H:%M:%S %Z",
            "%a, %d %b %Y %H:%M:%S %z",
        ];
        
        for format in formats {
            if let Ok(dt) = DateTime::parse_from_str(date_str, format) {
                return Ok(dt.with_timezone(&Utc));
            }
            
            if let Ok(naive_dt) = NaiveDateTime::parse_from_str(date_str, format) {
                return Ok(Utc.from_utc_datetime(&naive_dt));
            }
        }
        
        Err(UtilError::DateError(format!("Unable to parse date: {}", date_str)))
    }
    
    /// Format date for display
    pub fn format_display_date(date: &DateTime<Utc>) -> String {
        let local_date = date.with_timezone(&Local);
        local_date.format("%Y-%m-%d %H:%M").to_string()
    }
    
    /// Format date for file names (safe characters only)
    pub fn format_filename_date(date: &DateTime<Utc>) -> String {
        date.format("%Y%m%d_%H%M%S").to_string()
    }
    
    /// Get relative time string (e.g., "2 hours ago", "3 days ago")
    pub fn format_relative_time(date: &DateTime<Utc>) -> String {
        let now = Utc::now();
        let duration = now.signed_duration_since(*date);
        
        if duration.num_seconds() < 60 {
            "just now".to_string()
        } else if duration.num_minutes() < 60 {
            let mins = duration.num_minutes();
            format!("{} minute{} ago", mins, if mins == 1 { "" } else { "s" })
        } else if duration.num_hours() < 24 {
            let hours = duration.num_hours();
            format!("{} hour{} ago", hours, if hours == 1 { "" } else { "s" })
        } else if duration.num_days() < 30 {
            let days = duration.num_days();
            format!("{} day{} ago", days, if days == 1 { "" } else { "s" })
        } else if duration.num_days() < 365 {
            let months = duration.num_days() / 30;
            format!("{} month{} ago", months, if months == 1 { "" } else { "s" })
        } else {
            let years = duration.num_days() / 365;
            format!("{} year{} ago", years, if years == 1 { "" } else { "s" })
        }
    }
    
    /// Check if date is within the last N days
    pub fn is_within_days(date: &DateTime<Utc>, days: i64) -> bool {
        let now = Utc::now();
        let duration = now.signed_duration_since(*date);
        duration.num_days() <= days
    }
    
    /// Get start of day in UTC
    pub fn start_of_day(date: &DateTime<Utc>) -> DateTime<Utc> {
        date.date_naive().and_hms_opt(0, 0, 0)
            .map(|naive| Utc.from_utc_datetime(&naive))
            .unwrap_or(*date)
    }
    
    /// Get end of day in UTC
    pub fn end_of_day(date: &DateTime<Utc>) -> DateTime<Utc> {
        date.date_naive().and_hms_opt(23, 59, 59)
            .map(|naive| Utc.from_utc_datetime(&naive))
            .unwrap_or(*date)
    }
    
    /// Parse date range string (e.g., "2023-01-01:2023-12-31")
    pub fn parse_date_range(range_str: &str) -> UtilResult<(DateTime<Utc>, DateTime<Utc>)> {
        let parts: Vec<&str> = range_str.split(':').collect();
        if parts.len() != 2 {
            return Err(UtilError::DateError("Invalid date range format".to_string()));
        }
        
        let start_date = Self::parse_arxiv_date(parts[0])?;
        let end_date = Self::parse_arxiv_date(parts[1])?;
        
        if start_date > end_date {
            return Err(UtilError::DateError("Start date must be before end date".to_string()));
        }
        
        Ok((start_date, end_date))
    }
    
    /// Get current timestamp as string
    pub fn current_timestamp() -> String {
        Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
    }
    
    /// Convert to ISO 8601 format
    pub fn to_iso8601(date: &DateTime<Utc>) -> String {
        date.format("%Y-%m-%dT%H:%M:%SZ").to_string()
    }
    
    /// Parse ISO 8601 format
    pub fn from_iso8601(date_str: &str) -> UtilResult<DateTime<Utc>> {
        DateTime::parse_from_rfc3339(date_str)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| UtilError::DateError(format!("Failed to parse ISO 8601 date: {}", e)))
    }
    
    /// Get date filters for common time ranges
    pub fn get_common_date_filters() -> Vec<(&'static str, DateTime<Utc>)> {
        let now = Utc::now();
        vec![
            ("Today", Self::start_of_day(&now)),
            ("Last 7 days", now - chrono::Duration::days(7)),
            ("Last 30 days", now - chrono::Duration::days(30)),
            ("Last 3 months", now - chrono::Duration::days(90)),
            ("Last 6 months", now - chrono::Duration::days(180)),
            ("Last year", now - chrono::Duration::days(365)),
        ]
    }
    
    /// Check if two dates are on the same day
    pub fn same_day(date1: &DateTime<Utc>, date2: &DateTime<Utc>) -> bool {
        date1.date_naive() == date2.date_naive()
    }
    
    /// Get business days between two dates (excluding weekends)
    pub fn business_days_between(start: &DateTime<Utc>, end: &DateTime<Utc>) -> i64 {
        let mut current = *start;
        let mut count = 0i64;
        
        while current.date_naive() <= end.date_naive() {
            let weekday = current.weekday();
            if weekday != chrono::Weekday::Sat && weekday != chrono::Weekday::Sun {
                count += 1;
            }
            current = current + chrono::Duration::days(1);
        }
        
        count
    }
}

/// Date range for filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl DateRange {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> UtilResult<Self> {
        if start > end {
            return Err(UtilError::DateError("Start date must be before end date".to_string()));
        }
        Ok(Self { start, end })
    }
    
    pub fn contains(&self, date: &DateTime<Utc>) -> bool {
        *date >= self.start && *date <= self.end
    }
    
    pub fn duration_days(&self) -> i64 {
        self.end.signed_duration_since(self.start).num_days()
    }
    
    pub fn from_days_ago(days: i64) -> Self {
        let now = Utc::now();
        let start = now - chrono::Duration::days(days);
        Self { start, end: now }
    }
    
    pub fn last_week() -> Self {
        Self::from_days_ago(7)
    }
    
    pub fn last_month() -> Self {
        Self::from_days_ago(30)
    }
    
    pub fn last_year() -> Self {
        Self::from_days_ago(365)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_arxiv_date() {
        // Test basic date format
        let date = DateUtils::parse_arxiv_date("2023-12-01").unwrap();
        assert_eq!(date.year(), 2023);
        assert_eq!(date.month(), 12);
        assert_eq!(date.day(), 1);
        
        // Test ISO format
        let date = DateUtils::parse_arxiv_date("2023-12-01T14:30:00Z").unwrap();
        assert_eq!(date.hour(), 14);
        assert_eq!(date.minute(), 30);
    }

    #[test]
    fn test_format_relative_time() {
        let now = Utc::now();
        
        // Test recent time
        let recent = now - chrono::Duration::minutes(5);
        assert_eq!(DateUtils::format_relative_time(&recent), "5 minutes ago");
        
        // Test hours ago
        let hours_ago = now - chrono::Duration::hours(2);
        assert_eq!(DateUtils::format_relative_time(&hours_ago), "2 hours ago");
        
        // Test days ago
        let days_ago = now - chrono::Duration::days(3);
        assert_eq!(DateUtils::format_relative_time(&days_ago), "3 days ago");
    }

    #[test]
    fn test_is_within_days() {
        let now = Utc::now();
        let recent = now - chrono::Duration::days(2);
        let old = now - chrono::Duration::days(10);
        
        assert!(DateUtils::is_within_days(&recent, 7));
        assert!(!DateUtils::is_within_days(&old, 7));
    }

    #[test]
    fn test_parse_date_range() {
        let range = DateUtils::parse_date_range("2023-01-01:2023-12-31").unwrap();
        assert_eq!(range.0.year(), 2023);
        assert_eq!(range.0.month(), 1);
        assert_eq!(range.1.month(), 12);
    }

    #[test]
    fn test_date_range() {
        let start = Utc::now() - chrono::Duration::days(10);
        let end = Utc::now();
        let range = DateRange::new(start, end).unwrap();
        
        let middle = start + chrono::Duration::days(5);
        assert!(range.contains(&middle));
        
        let before = start - chrono::Duration::days(1);
        assert!(!range.contains(&before));
        
        assert_eq!(range.duration_days(), 10);
    }

    #[test]
    fn test_same_day() {
        let date1 = DateUtils::parse_arxiv_date("2023-12-01T10:00:00Z").unwrap();
        let date2 = DateUtils::parse_arxiv_date("2023-12-01T20:00:00Z").unwrap();
        let date3 = DateUtils::parse_arxiv_date("2023-12-02T10:00:00Z").unwrap();
        
        assert!(DateUtils::same_day(&date1, &date2));
        assert!(!DateUtils::same_day(&date1, &date3));
    }

    #[test]
    fn test_iso8601_conversion() {
        let now = Utc::now();
        let iso_string = DateUtils::to_iso8601(&now);
        let parsed = DateUtils::from_iso8601(&iso_string).unwrap();
        
        // Should be equal within a second (due to potential precision differences)
        let diff = (now.timestamp() - parsed.timestamp()).abs();
        assert!(diff <= 1);
    }

    #[test]
    fn test_common_date_filters() {
        let filters = DateUtils::get_common_date_filters();
        assert_eq!(filters.len(), 6);
        assert_eq!(filters[0].0, "Today");
        assert_eq!(filters[1].0, "Last 7 days");
    }

    #[test]
    fn test_business_days() {
        // Monday to Friday (5 business days)
        let monday = DateUtils::parse_arxiv_date("2023-12-04").unwrap(); // Monday
        let friday = DateUtils::parse_arxiv_date("2023-12-08").unwrap(); // Friday
        
        let business_days = DateUtils::business_days_between(&monday, &friday);
        assert_eq!(business_days, 5);
    }
}
