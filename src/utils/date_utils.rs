use chrono::{DateTime, Local, TimeZone, Utc};
use log::warn;

/// Parses a date range in the format "YYYY-MM-DD,YYYY-MM-DD".
/// Returns `None` if the format is invalid or parsing fails.

pub fn parse_date_range(date_range: &str) -> Option<DateRange> {
    let end_date_str = if date_range.contains(',') {
        date_range.split(',').nth(1).unwrap_or(date_range)
    } else {
        date_range
    };

    // Create the end date string with proper lifetime
    let end_date_with_time = format!("{} 23:59:59", end_date_str);

    let dates: Vec<&str> = if date_range.contains(',') {
        date_range.split(',').collect()
    } else {
        vec![date_range, &end_date_with_time]
    };

    let parse_date = |date: &str, is_end: bool| {
        if date.trim().is_empty() {
            None
        } else {
            let time_part = if is_end { "23:59:59" } else { "00:00:00" };
            match DateTime::parse_from_str(
                &format!("{} {} +0000", date, time_part),
                "%Y-%m-%d %H:%M:%S %z",
            ) {
                Ok(dt) => Some(dt.with_timezone(&Utc)),
                Err(e) => {
                    warn!("Failed to parse date '{}': {}", date, e);
                    None
                }
            }
        }
    };

    let start_date = parse_date(dates[0], false);
    let end_date = parse_date(dates[1], true);

    Some(DateRange {
        start: start_date,
        end: end_date,
    })
}

/// Validates that the interval is either "hour" or "day".
pub fn validate_interval(interval: &str) -> bool {
    matches!(interval, "hour" | "day")
}

#[derive(Debug)]
pub struct DateRange {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
}
