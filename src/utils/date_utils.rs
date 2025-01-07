// utils/date_utils.rs
use chrono::{DateTime, Utc};

pub fn parse_date_range(date_range: &str) -> Option<DateRange> {
    let dates: Vec<&str> = date_range.split(',').collect();
    if dates.len() != 2 {
        log::warn!("Invalid date range format: {}", date_range);
        return None;
    }

    let parse_date = |date: &str| {
        if date.trim().is_empty() {
            None
        } else {
            DateTime::parse_from_str(&format!("{} 00:00:00 +0000", date), "%Y-%m-%d %H:%M:%S %z")
                .ok()
                .map(|dt| dt.with_timezone(&Utc))
        }
    };

    Some(DateRange {
        start: parse_date(dates[0]),
        end: parse_date(dates[1]),
    })
}

pub fn validate_interval(interval: &str) -> bool {
    matches!(interval, "hour" | "day" | "week" | "month")
}

#[derive(Debug)]
pub struct DateRange {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
}
