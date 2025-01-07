// utils/date_utils.rs
use chrono::{DateTime, Utc};
use log::warn;

#[derive(Debug)]
pub struct DateRange {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
}

pub fn parse_date_range(date_range: &str) -> Option<DateRange> {
    let dates: Vec<&str> = date_range.split(',').collect();
    let start_date_str = dates[0];
    let end_date_str = dates.get(1).unwrap_or(&start_date_str);

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

    Some(DateRange {
        start: parse_date(start_date_str, false),
        end: parse_date(end_date_str, true),
    })
}
